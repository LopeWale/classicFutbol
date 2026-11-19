#![forbid(unsafe_code)]

//! Deterministic GGPO-style rollback netcode over `psfc-sim` (`MAX_ROLLBACK_FRAMES = 8`).

use psfc_sim::{FixtureSnapshot, InputFrame, MatchSimulation, SimError, TeamSide};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub const MAX_ROLLBACK_FRAMES: u32 = 8;
pub const HASH_CHECK_INTERVAL_TICKS: u32 = 60;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetMessage {
    InputBatch {
        side: TeamSide,
        first_tick: u32,
        frames: Vec<InputFrame>,
    },
    StateHash {
        side: TeamSide,
        tick: u32,
        hash: u64,
    },
}

pub trait RollbackTransport {
    fn send_peer(&mut self, msg: NetMessage);
    fn poll_peer(&mut self) -> Option<NetMessage>;
    fn mirror_to_host(&mut self, msg: NetMessage);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdvanceOutcome {
    Advanced {
        tick: u32,
        rolled_back_frames: u32,
    },
    StalledWaitingForOpponent {
        current_tick: u32,
        confirmed_remote_tick: u32,
    },
    DesyncDetected {
        tick: u32,
        local_hash: u64,
        remote_hash: u64,
    },
}

#[derive(Clone)]
struct FrameRecord {
    tick: u32,
    saved_state_after_tick: Vec<u8>,
    hash_after_tick: u64,
}

pub struct RollbackSession {
    local_side: TeamSide,
    sim: MatchSimulation,
    initial_state: Vec<u8>,
    local_inputs: Vec<InputFrame>,
    remote_inputs: Vec<Option<InputFrame>>,
    predicted_inputs: Vec<InputFrame>,
    confirmed_remote_tick: u32,
    snapshots: VecDeque<FrameRecord>,
    pending_remote_hashes: Vec<(u32, u64)>,
    desync: Option<(u32, u64, u64)>,
}

impl RollbackSession {
    pub fn new(snapshot: &FixtureSnapshot, local_side: TeamSide) -> Result<Self, SimError> {
        let sim = MatchSimulation::new(snapshot)?;
        let initial_state = sim.save_state()?;
        Ok(Self {
            local_side,
            sim,
            initial_state,
            local_inputs: Vec::new(),
            remote_inputs: Vec::new(),
            predicted_inputs: Vec::new(),
            confirmed_remote_tick: 0,
            snapshots: VecDeque::with_capacity((MAX_ROLLBACK_FRAMES + 4) as usize),
            pending_remote_hashes: Vec::new(),
            desync: None,
        })
    }

    pub fn sim(&self) -> &MatchSimulation {
        &self.sim
    }

    pub fn confirmed_remote_tick(&self) -> u32 {
        self.confirmed_remote_tick
    }

    /// Ingests arrived peer packets and rolls back/re-simulates up to `current_tick`
    /// without advancing a new local frame.
    pub fn poll_and_reconcile(
        &mut self,
        transport: &mut dyn RollbackTransport,
    ) -> Result<u32, SimError> {
        let mut earliest_misprediction: Option<u32> = None;

        while let Some(msg) = transport.poll_peer() {
            match msg {
                NetMessage::InputBatch {
                    side,
                    first_tick,
                    frames,
                } if side == self.local_side.opponent() => {
                    for (offset, frame) in frames.into_iter().enumerate() {
                        let tick_1based = first_tick + (offset as u32);
                        if tick_1based == 0 {
                            continue;
                        }
                        let idx = (tick_1based - 1) as usize;
                        if self.remote_inputs.len() <= idx {
                            self.remote_inputs.resize(idx + 1, None);
                        }
                        if self.remote_inputs[idx].is_none() {
                            self.remote_inputs[idx] = Some(frame);
                            if idx < self.predicted_inputs.len()
                                && self.predicted_inputs[idx] != frame
                            {
                                earliest_misprediction = Some(
                                    earliest_misprediction
                                        .map_or(tick_1based, |prev| prev.min(tick_1based)),
                                );
                            }
                        }
                    }
                    self.refresh_confirmed_remote_tick();
                }
                NetMessage::StateHash { side, tick, hash }
                    if side == self.local_side.opponent() =>
                {
                    self.pending_remote_hashes.push((tick, hash));
                }
                _ => {}
            }
        }

        let mut rolled_back_frames = 0;
        if let Some(dirty_tick) = earliest_misprediction {
            let current_tick = self.sim.state().tick;
            if dirty_tick <= current_tick {
                rolled_back_frames = current_tick - dirty_tick + 1;
                self.resimulate_from(dirty_tick, current_tick)?;
            }
        }

        self.verify_confirmed_hashes();
        Ok(rolled_back_frames)
    }

    fn send_recent_inputs(&self, transport: &mut dyn RollbackTransport) {
        if self.local_inputs.is_empty() {
            return;
        }
        let window_len = MAX_ROLLBACK_FRAMES as usize;
        let window_start_idx = self.local_inputs.len().saturating_sub(window_len);
        let first_tick = (window_start_idx as u32) + 1;
        let batch = NetMessage::InputBatch {
            side: self.local_side,
            first_tick,
            frames: self.local_inputs[window_start_idx..].to_vec(),
        };
        transport.send_peer(batch.clone());
        transport.mirror_to_host(batch);
    }

    /// Poll incoming network packets, reconcile predictions via rollback if needed,
    /// and advance one local tick if within the 8-frame rollback window.
    pub fn step_frame(
        &mut self,
        local_input: InputFrame,
        transport: &mut dyn RollbackTransport,
    ) -> Result<AdvanceOutcome, SimError> {
        let rolled_back_frames = self.poll_and_reconcile(transport)?;

        if let Some((tick, local_hash, remote_hash)) = self.desync {
            return Ok(AdvanceOutcome::DesyncDetected {
                tick,
                local_hash,
                remote_hash,
            });
        }

        // Stall if peer is more than MAX_ROLLBACK_FRAMES behind (retransmit recent window to heal drops)
        let current_tick = self.sim.state().tick;
        if current_tick.saturating_sub(self.confirmed_remote_tick) >= MAX_ROLLBACK_FRAMES {
            self.send_recent_inputs(transport);
            return Ok(AdvanceOutcome::StalledWaitingForOpponent {
                current_tick,
                confirmed_remote_tick: self.confirmed_remote_tick,
            });
        }

        // Advance next tick (`next_tick = current_tick + 1`)
        let next_tick = current_tick + 1;
        let idx = (next_tick - 1) as usize;
        self.local_inputs.push(local_input);

        let remote_in = self
            .remote_inputs
            .get(idx)
            .and_then(|x| *x)
            .unwrap_or_else(|| self.last_confirmed_remote_input());
        self.predicted_inputs.push(remote_in);

        let pair = self.pair_inputs(local_input, remote_in);
        self.sim.tick(&pair);

        let saved = self.sim.save_state()?;
        let hash = self.sim.state_hash();
        self.snapshots.push_back(FrameRecord {
            tick: next_tick,
            saved_state_after_tick: saved,
            hash_after_tick: hash,
        });
        while self.snapshots.len() > (MAX_ROLLBACK_FRAMES as usize + 6) {
            self.snapshots.pop_front();
        }

        self.send_recent_inputs(transport);

        if next_tick.is_multiple_of(HASH_CHECK_INTERVAL_TICKS)
            && self.confirmed_remote_tick >= next_tick
        {
            let hash_msg = NetMessage::StateHash {
                side: self.local_side,
                tick: next_tick,
                hash,
            };
            transport.send_peer(hash_msg.clone());
            transport.mirror_to_host(hash_msg);
        }

        self.verify_confirmed_hashes();
        if let Some((tick, local_hash, remote_hash)) = self.desync {
            return Ok(AdvanceOutcome::DesyncDetected {
                tick,
                local_hash,
                remote_hash,
            });
        }

        Ok(AdvanceOutcome::Advanced {
            tick: next_tick,
            rolled_back_frames,
        })
    }

    fn resimulate_from(&mut self, from_tick: u32, to_tick: u32) -> Result<(), SimError> {
        let base_tick = from_tick - 1;
        if base_tick == 0 {
            self.sim.load_state(&self.initial_state)?;
        } else {
            let base_bytes = self
                .snapshots
                .iter()
                .find(|r| r.tick == base_tick)
                .map(|r| r.saved_state_after_tick.clone())
                .ok_or_else(|| {
                    SimError(format!(
                        "missing rollback snapshot for base_tick {base_tick}"
                    ))
                })?;
            self.sim.load_state(&base_bytes)?;
        }

        // Drop snapshots from `from_tick` onward and rebuild them cleanly
        while self
            .snapshots
            .back()
            .is_some_and(|rec| rec.tick >= from_tick)
        {
            self.snapshots.pop_back();
        }

        for t in from_tick..=to_tick {
            let i = (t - 1) as usize;
            let local_in = self.local_inputs[i];
            let remote_in = self
                .remote_inputs
                .get(i)
                .and_then(|x| *x)
                .unwrap_or_else(|| self.confirmed_remote_at_or_before(i));
            self.predicted_inputs[i] = remote_in;
            let pair = self.pair_inputs(local_in, remote_in);
            self.sim.tick(&pair);
            self.snapshots.push_back(FrameRecord {
                tick: t,
                saved_state_after_tick: self.sim.save_state()?,
                hash_after_tick: self.sim.state_hash(),
            });
        }
        Ok(())
    }

    fn verify_confirmed_hashes(&mut self) {
        if self.desync.is_some() {
            return;
        }
        let confirmed = self.confirmed_remote_tick;
        let mut remaining = Vec::new();
        for (tick, remote_hash) in self.pending_remote_hashes.drain(..) {
            if tick > confirmed {
                remaining.push((tick, remote_hash));
                continue;
            }
            if let Some(rec) = self.snapshots.iter().find(|r| r.tick == tick) {
                if rec.hash_after_tick != remote_hash {
                    self.desync = Some((tick, rec.hash_after_tick, remote_hash));
                }
            }
        }
        self.pending_remote_hashes = remaining;
    }

    fn refresh_confirmed_remote_tick(&mut self) {
        while (self.confirmed_remote_tick as usize) < self.remote_inputs.len()
            && self.remote_inputs[self.confirmed_remote_tick as usize].is_some()
        {
            self.confirmed_remote_tick += 1;
        }
    }

    fn last_confirmed_remote_input(&self) -> InputFrame {
        if self.confirmed_remote_tick == 0 {
            InputFrame::IDLE
        } else {
            self.remote_inputs[(self.confirmed_remote_tick - 1) as usize]
                .unwrap_or(InputFrame::IDLE)
        }
    }

    fn confirmed_remote_at_or_before(&self, idx: usize) -> InputFrame {
        for back in (0..=idx).rev() {
            if let Some(Some(frame)) = self.remote_inputs.get(back) {
                return *frame;
            }
        }
        InputFrame::IDLE
    }

    fn pair_inputs(&self, local: InputFrame, remote: InputFrame) -> [InputFrame; 2] {
        match self.local_side {
            TeamSide::Home => [local, remote],
            TeamSide::Away => [remote, local],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use psfc_sim::input::{BTN_PASS, BTN_SHOOT, BTN_SPRINT, BTN_TACKLE};
    use psfc_sim::SideController;

    struct DelayedQueueEndpoint {
        now_tick: u32,
        latency_ticks: u32,
        outbox: Vec<(u32, NetMessage)>,
        inbox: VecDeque<NetMessage>,
    }

    impl DelayedQueueEndpoint {
        fn new(latency_ticks: u32) -> Self {
            Self {
                now_tick: 0,
                latency_ticks,
                outbox: Vec::new(),
                inbox: VecDeque::new(),
            }
        }
    }

    impl RollbackTransport for DelayedQueueEndpoint {
        fn send_peer(&mut self, msg: NetMessage) {
            self.outbox.push((self.now_tick + self.latency_ticks, msg));
        }

        fn poll_peer(&mut self) -> Option<NetMessage> {
            self.inbox.pop_front()
        }

        fn mirror_to_host(&mut self, _msg: NetMessage) {}
    }

    #[test]
    fn two_rollback_peers_under_latency_and_packet_loss_converge_bit_for_bit() {
        let mut snap = FixtureSnapshot::sample_five_a_side(9001);
        snap.rules.home_controller = SideController::Human;
        snap.rules.away_controller = SideController::Human;

        let target_ticks = 300_usize;
        let mut home_script = Vec::with_capacity(target_ticks);
        let mut away_script = Vec::with_capacity(target_ticks);
        for t in 1..=(target_ticks as u32) {
            home_script.push(InputFrame {
                move_x: (((t * 13) % 200) as i32 - 100) as i8,
                move_y: (((t * 7) % 180) as i32 - 90) as i8,
                buttons: if t % 23 == 0 {
                    BTN_PASS
                } else if t % 41 == 0 {
                    BTN_SHOOT
                } else {
                    BTN_SPRINT
                },
                ..InputFrame::IDLE
            });
            away_script.push(InputFrame {
                move_x: -(((t * 11) % 200) as i32 - 100) as i8,
                move_y: (((t * 17) % 180) as i32 - 90) as i8,
                buttons: if t % 19 == 0 {
                    BTN_TACKLE
                } else if t % 29 == 0 {
                    BTN_PASS
                } else {
                    0
                },
                ..InputFrame::IDLE
            });
        }

        let mut home_sess = RollbackSession::new(&snap, TeamSide::Home).unwrap();
        let mut away_sess = RollbackSession::new(&snap, TeamSide::Away).unwrap();
        let mut ref_sim = MatchSimulation::new(&snap).unwrap();
        for i in 0..target_ticks {
            ref_sim.tick(&[home_script[i], away_script[i]]);
        }

        let mut home_net = DelayedQueueEndpoint::new(4); // ~67 ms one-way (~133 ms RTT)
        let mut away_net = DelayedQueueEndpoint::new(4);
        let mut total_rollbacks = 0_u32;

        for wall_step in 1..=600_u32 {
            home_net.now_tick = wall_step;
            away_net.now_tick = wall_step;

            let mut rem_home = Vec::new();
            for (due, msg) in home_net.outbox.drain(..) {
                if due <= wall_step {
                    if wall_step % 7 != 0 {
                        away_net.inbox.push_back(msg);
                    }
                } else {
                    rem_home.push((due, msg));
                }
            }
            home_net.outbox = rem_home;

            let mut rem_away = Vec::new();
            for (due, msg) in away_net.outbox.drain(..) {
                if due <= wall_step {
                    if wall_step % 11 != 0 {
                        home_net.inbox.push_back(msg);
                    }
                } else {
                    rem_away.push((due, msg));
                }
            }
            away_net.outbox = rem_away;

            let h_tick = home_sess.sim().state().tick as usize;
            if h_tick < target_ticks {
                let out_h = home_sess.step_frame(home_script[h_tick], &mut home_net).unwrap();
                if let AdvanceOutcome::Advanced {
                    rolled_back_frames, ..
                } = out_h
                {
                    total_rollbacks += rolled_back_frames;
                }
                assert!(!matches!(out_h, AdvanceOutcome::DesyncDetected { .. }));
            } else {
                total_rollbacks += home_sess.poll_and_reconcile(&mut home_net).unwrap();
            }

            let a_tick = away_sess.sim().state().tick as usize;
            if a_tick < target_ticks {
                let out_a = away_sess.step_frame(away_script[a_tick], &mut away_net).unwrap();
                if let AdvanceOutcome::Advanced {
                    rolled_back_frames, ..
                } = out_a
                {
                    total_rollbacks += rolled_back_frames;
                }
                assert!(!matches!(out_a, AdvanceOutcome::DesyncDetected { .. }));
            } else {
                total_rollbacks += away_sess.poll_and_reconcile(&mut away_net).unwrap();
            }

            if home_sess.sim().state().tick as usize == target_ticks
                && away_sess.sim().state().tick as usize == target_ticks
                && home_sess.confirmed_remote_tick() as usize == target_ticks
                && away_sess.confirmed_remote_tick() as usize == target_ticks
            {
                break;
            }
        }

        // Deliver any trailing packets if loop ended right before delivery
        for (_, msg) in home_net.outbox.drain(..) {
            away_net.inbox.push_back(msg);
        }
        for (_, msg) in away_net.outbox.drain(..) {
            home_net.inbox.push_back(msg);
        }
        total_rollbacks += home_sess.poll_and_reconcile(&mut home_net).unwrap();
        total_rollbacks += away_sess.poll_and_reconcile(&mut away_net).unwrap();

        assert_eq!(home_sess.sim().state().tick as usize, target_ticks);
        assert_eq!(away_sess.sim().state().tick as usize, target_ticks);
        assert!(total_rollbacks > 0, "expected rollbacks to have occurred under 4-tick delay");
        assert_eq!(home_sess.sim().state_hash(), ref_sim.state_hash());
        assert_eq!(away_sess.sim().state_hash(), ref_sim.state_hash());
    }
}
