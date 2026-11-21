//! Deterministic 60 Hz football simulation engine (`MatchSimulation`).

use crate::ai::{
    best_ball_intercept_for_player, compute_goalkeeper_intent, compute_outfield_ai_intent,
    generate_cpu_input, world_anchor_for_player,
};
use crate::config::{formation_anchors, SideController, TeamSide, TICKS_PER_SECOND};
use crate::events::{MatchEvent, MatchEventType, PlayerMatchStats};
use crate::input::{
    InputFrame, BTN_CALL_RUN, BTN_JOCKEY, BTN_LOB, BTN_PASS, BTN_SHOOT, BTN_SKILL, BTN_SLIDE,
    BTN_SPRINT, BTN_SWITCH, BTN_TACKLE, BTN_THROUGH,
};
use crate::math::{angle_diff_bam, atan2_bam, turn_toward_bam, Fp, Vec2, Vec3};
use crate::rng::DeterministicRng;
use crate::snapshot::{FixtureSnapshot, PlayerRole};
use crate::state::{
    AssistCandidate, BallState, BufferedAction, MatchPhase, MatchState, PassInFlight,
    PlayerActionState, PlayerState, RestartKind,
};

const CHARGE_MASK: u16 = BTN_PASS | BTN_THROUGH | BTN_LOB | BTN_SHOOT;
const MAX_CHARGE_TICKS: u16 = 28;
const INPUT_BUFFER_TTL_TICKS: u8 = 16;

#[derive(Debug, Clone)]
pub struct SimError(pub String);

impl core::fmt::Display for SimError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct MatchSimulation {
    state: MatchState,
}

impl MatchSimulation {
    pub fn new(snapshot: &FixtureSnapshot) -> Result<Self, SimError> {
        snapshot.validate().map_err(SimError)?;

        let build_side = |side: TeamSide, snap: &crate::snapshot::TeamSnapshot| {
            let anchors = formation_anchors(&snap.formation, snapshot.format.players_per_side);
            let mut players = Vec::with_capacity(snap.starters.len());
            let mut stats = Vec::with_capacity(snap.starters.len());
            for (idx, p) in snap.starters.iter().enumerate() {
                let anchor = anchors.get(idx).copied().unwrap_or((0, 0));
                players.push(PlayerState {
                    team: side,
                    index: idx,
                    footballer_id: p.footballer_id.clone(),
                    name: p.name.clone(),
                    shirt_number: p.shirt_number,
                    role: p.role,
                    attributes: p.attributes,
                    base_anchor_permille: anchor,
                    pos: Vec2::ZERO,
                    vel: Vec2::ZERO,
                    z: Fp::ZERO,
                    vz: Fp::ZERO,
                    facing_bam: if side == TeamSide::Home { 0 } else { 32768 },
                    stamina_permille: 1_000,
                    action: PlayerActionState::Normal,
                    run_boost_ticks: 0,
                    yellow_cards: 0,
                    sent_off: false,
                });
                stats.push(PlayerMatchStats {
                    footballer_id: p.footballer_id.clone(),
                    name: p.name.clone(),
                    team: Some(side),
                    ..PlayerMatchStats::default()
                });
            }
            (players, stats)
        };

        let (home_players, home_stats) = build_side(TeamSide::Home, &snapshot.home);
        let (away_players, away_stats) = build_side(TeamSide::Away, &snapshot.away);
        let default_ctrl_home = snapshot.format.players_per_side.saturating_sub(1).max(1);
        let default_ctrl_away = snapshot.format.players_per_side.saturating_sub(1).max(1);

        let mut sim = Self {
            state: MatchState {
                fixture_id: snapshot.fixture_id.clone(),
                format: snapshot.format.clone(),
                rules: snapshot.rules.clone(),
                teams: [snapshot.home.clone(), snapshot.away.clone()],
                tick: 0,
                clock_ticks: 0,
                half: 1,
                score: [0, 0],
                possession_ticks: [0, 0],
                possession_balance_permille: 0,
                phase: MatchPhase::KickOff {
                    team: TeamSide::Home,
                    wait_ticks: 0,
                },
                controlled_idx: [default_ctrl_home, default_ctrl_away],
                prev_buttons: [0, 0],
                switch_cooldown: [0, 0],
                input_buffer: [None, None],
                subs_used: [0, 0],
                players: [home_players, away_players],
                player_stats: [home_stats, away_stats],
                archived_player_stats: [Vec::new(), Vec::new()],
                ball: BallState {
                    pos: Vec3::ZERO,
                    vel: Vec3::ZERO,
                    spin_side: Fp::ZERO,
                    spin_top: Fp::ZERO,
                    owner: None,
                    gk_hold_ticks: 0,
                    dribble_touch_cd: 0,
                    last_touch_team: None,
                    last_touch_player: None,
                    last_touch_meta: None,
                    last_team_touch_meta: [None, None],
                    pass_in_flight: None,
                    assist_candidate: None,
                },
                rng: DeterministicRng::new(snapshot.seed),
                went_to_extra_time: false,
                shootout_score: None,
                pending_events: Vec::new(),
                recorded_events: Vec::new(),
            },
        };
        sim.arrange_kickoff(TeamSide::Home);
        sim.emit_event(
            MatchEventType::KickOff,
            TeamSide::Home,
            None,
            None,
            "First half ready for kick-off".to_string(),
        );
        Ok(sim)
    }

    #[inline]
    fn set_last_touch(&mut self, side: TeamSide, idx: usize) {
        let p = &self.state.players[side.index()][idx];
        let fid = p.footballer_id.clone();
        let pname = p.name.clone();
        self.state.ball.last_team_touch_meta[side.index()] = Some((fid.clone(), pname.clone()));
        self.state.ball.last_touch_team = Some(side);
        self.state.ball.last_touch_player = Some((side, idx));
        self.state.ball.last_touch_meta = Some((side, fid, pname));
    }

    #[inline]
    pub fn state(&self) -> &MatchState {
        &self.state
    }

    #[inline]
    pub fn is_finished(&self) -> bool {
        matches!(self.state.phase, MatchPhase::FullTime)
    }

    #[inline]
    pub fn state_hash(&self) -> u64 {
        self.state.compute_hash()
    }

    pub fn drain_events(&mut self) -> Vec<MatchEvent> {
        core::mem::take(&mut self.state.pending_events)
    }

    pub fn save_state(&self) -> Result<Vec<u8>, SimError> {
        serde_json::to_vec(&self.state).map_err(|e| SimError(e.to_string()))
    }

    pub fn load_state(&mut self, bytes: &[u8]) -> Result<(), SimError> {
        let restored: MatchState =
            serde_json::from_slice(bytes).map_err(|e| SimError(e.to_string()))?;
        self.state = restored;
        Ok(())
    }

    /// Advance the simulation by one 60 Hz tick (`16.667 ms`).
    /// For any side configured with `SideController::Cpu`, the CPU controller deterministically
    /// produces that side's `InputFrame` inside the simulation from `state` + `seed`.
    pub fn tick(&mut self, external_inputs: &[InputFrame; 2]) {
        if matches!(self.state.phase, MatchPhase::FullTime) {
            return;
        }

        self.state.tick = self.state.tick.saturating_add(1);

        // Execute any deterministic substitution encoded in `external_inputs[s].reserved`
        for side in [TeamSide::Home, TeamSide::Away] {
            if let Some((pitch_slot, bench_idx)) = external_inputs[side.index()].decode_substitution() {
                self.apply_substitution(side, pitch_slot, bench_idx);
            }
        }

        let home_in = match self.state.rules.home_controller {
            SideController::Human => external_inputs[0],
            SideController::Cpu(diff) => generate_cpu_input(&self.state, TeamSide::Home, diff),
        };
        let away_in = match self.state.rules.away_controller {
            SideController::Human => external_inputs[1],
            SideController::Cpu(diff) => generate_cpu_input(&self.state, TeamSide::Away, diff),
        };
        let inputs = [home_in, away_in];

        for c in &mut self.state.switch_cooldown {
            *c = c.saturating_sub(1);
        }
        for slot in &mut self.state.input_buffer {
            if let Some(mut buf) = *slot {
                if buf.ttl_ticks <= 1 {
                    *slot = None;
                } else {
                    buf.ttl_ticks -= 1;
                    *slot = Some(buf);
                }
            }
        }

        // Phase state machine before movement/ball
        match self.state.phase {
            MatchPhase::GoalScored {
                scoring_team,
                celebration_ticks,
                ..
            } => {
                if celebration_ticks <= 1 {
                    let restart_team = scoring_team.opponent();
                    self.arrange_kickoff(restart_team);
                } else {
                    self.state.phase = MatchPhase::GoalScored {
                        scoring_team,
                        scorer_idx: 0,
                        celebration_ticks: celebration_ticks - 1,
                    };
                    self.step_celebration_drift();
                }
                self.state.prev_buttons = [inputs[0].buttons, inputs[1].buttons];
                return;
            }
            MatchPhase::HalfTime { wait_ticks } => {
                if wait_ticks <= 1 {
                    match self.state.half {
                        1 => {
                            self.state.half = 2;
                            self.arrange_kickoff(TeamSide::Away);
                            self.emit_event(
                                MatchEventType::KickOff,
                                TeamSide::Away,
                                None,
                                None,
                                "Second half kick-off".to_string(),
                            );
                        }
                        2 => {
                            self.state.half = 3;
                            self.state.went_to_extra_time = true;
                            self.arrange_kickoff(TeamSide::Home);
                            self.emit_event(
                                MatchEventType::ExtraTimeStart,
                                TeamSide::Home,
                                None,
                                None,
                                "Extra time first half kick-off".to_string(),
                            );
                        }
                        _ => {
                            self.state.half = 4;
                            self.arrange_kickoff(TeamSide::Away);
                            self.emit_event(
                                MatchEventType::KickOff,
                                TeamSide::Away,
                                None,
                                None,
                                "Extra time second half kick-off".to_string(),
                            );
                        }
                    }
                } else {
                    self.state.phase = MatchPhase::HalfTime {
                        wait_ticks: wait_ticks - 1,
                    };
                }
                self.state.prev_buttons = [inputs[0].buttons, inputs[1].buttons];
                return;
            }
            MatchPhase::KickOff { team, wait_ticks } => {
                let side_in = inputs[team.index()];
                let prev = self.state.prev_buttons[team.index()];
                let kick_pressed = (side_in.buttons & CHARGE_MASK) != 0 && (prev & CHARGE_MASK) == 0;
                if kick_pressed || wait_ticks >= 150 {
                    let taker_idx = self.state.controlled_idx[team.index()];
                    let attack_sign = team.attack_sign(self.state.second_half_swapped());
                    let default_aim = Vec2::new(Fp::from_int(-attack_sign as i32), Fp::from_milli(400));
                    let aim = if side_in.move_vec().len() > Fp::from_milli(150) {
                        side_in.move_vec()
                    } else {
                        default_aim.normalize_or_zero()
                    };
                    self.state.phase = MatchPhase::OpenPlay;
                    self.execute_kick(team, taker_idx, BTN_PASS, 10, false, aim);
                } else {
                    self.state.phase = MatchPhase::KickOff {
                        team,
                        wait_ticks: wait_ticks + 1,
                    };
                }
                self.state.prev_buttons = [inputs[0].buttons, inputs[1].buttons];
                return;
            }
            MatchPhase::SetPiece {
                kind,
                team,
                spot,
                taker_idx,
                wait_ticks,
            } => {
                let side_in = inputs[team.index()];
                let prev = self.state.prev_buttons[team.index()];
                let action_btns = side_in.buttons & CHARGE_MASK;
                let just_pressed = action_btns != 0 && (prev & CHARGE_MASK) == 0;

                // Allow slight aiming / teammate drifting while maintaining FreeKick defensive wall
                self.step_set_piece_movement(kind, team, taker_idx, spot);

                if just_pressed || wait_ticks >= 180 {
                    let chosen_btn = if action_btns != 0 {
                        action_btns
                    } else {
                        match kind {
                            RestartKind::CornerKick => BTN_LOB,
                            RestartKind::PenaltyKick => BTN_SHOOT,
                            RestartKind::FreeKick => {
                                let attack_sign = team.attack_sign(self.state.second_half_swapped());
                                let opp_goal = Vec2::new(
                                    Fp::from_milli(attack_sign * self.state.format.half_length().to_milli()),
                                    Fp::ZERO,
                                );
                                let shoot_range = if self.state.format.players_per_side <= 6 {
                                    Fp::from_milli(16_500)
                                } else {
                                    Fp::from_milli(25_000)
                                };
                                if spot.dist(opp_goal) <= shoot_range {
                                    BTN_SHOOT
                                } else {
                                    BTN_PASS
                                }
                            }
                            _ => BTN_PASS,
                        }
                    };
                    let attack_sign = team.attack_sign(self.state.second_half_swapped());
                    let fallback_aim = match kind {
                        RestartKind::CornerKick => {
                            let y_inward = if spot.y > Fp::ZERO {
                                Fp::NEG_ONE
                            } else {
                                Fp::ONE
                            };
                            Vec2::new(Fp::from_milli(-attack_sign * 200), y_inward).normalize_or_zero()
                        }
                        RestartKind::FreeKick if chosen_btn == BTN_SHOOT => {
                            let goal_x = Fp::from_milli(attack_sign * self.state.format.half_length().to_milli());
                            let corner_y = if spot.y >= Fp::ZERO {
                                -self.state.format.goal_half_width().mul(Fp::from_milli(580))
                            } else {
                                self.state.format.goal_half_width().mul(Fp::from_milli(580))
                            };
                            (Vec2::new(goal_x, corner_y) - spot).normalize_or_zero()
                        }
                        _ => Vec2::new(Fp::from_int(attack_sign as i32), Fp::ZERO),
                    };
                    let aim = if side_in.move_vec().len() > Fp::from_milli(150) {
                        side_in.move_vec()
                    } else {
                        fallback_aim
                    };
                    self.state.phase = MatchPhase::OpenPlay;
                    let charge = match kind {
                        RestartKind::PenaltyKick => 14,
                        RestartKind::CornerKick | RestartKind::GoalKick => 18,
                        RestartKind::FreeKick if chosen_btn == BTN_SHOOT => 16,
                        _ => 11,
                    };
                    self.execute_kick(team, taker_idx, chosen_btn, charge, kind == RestartKind::FreeKick, aim);
                } else {
                    self.state.phase = MatchPhase::SetPiece {
                        kind,
                        team,
                        spot,
                        taker_idx,
                        wait_ticks: wait_ticks + 1,
                    };
                }
                self.state.prev_buttons = [inputs[0].buttons, inputs[1].buttons];
                return;
            }
            MatchPhase::OpenPlay => {}
            MatchPhase::FullTime => return,
        }

        // OpenPlay tick!
        self.state.clock_ticks = self.state.clock_ticks.saturating_add(1);
        if let Some((owner_side, _)) = self.state.ball.owner {
            self.state.possession_ticks[owner_side.index()] += 1;
            let target_bal = if owner_side == TeamSide::Home { 1_000 } else { -1_000 };
            let diff = target_bal - self.state.possession_balance_permille;
            self.state.possession_balance_permille += diff.clamp(-38, 38);
        } else if let Some(ref pass) = self.state.ball.pass_in_flight {
            let target_bal = if pass.team == TeamSide::Home { 850 } else { -850 };
            let diff = target_bal - self.state.possession_balance_permille;
            self.state.possession_balance_permille += diff.clamp(-24, 24);
        } else {
            // Loose ball: decay gently (~12 permille/tick) so teammates hold tactical shape during 50/50s!
            let cur = self.state.possession_balance_permille;
            self.state.possession_balance_permille -= cur.clamp(-12, 12);
        }

        // Update ticks_on_pitch stats
        for side in [TeamSide::Home, TeamSide::Away] {
            for (idx, p) in self.state.players[side.index()].iter().enumerate() {
                if !p.sent_off {
                    self.state.player_stats[side.index()][idx].ticks_on_pitch += 1;
                }
            }
        }

        // 1. Handle player switching & teammate run triggers per team
        for side in [TeamSide::Home, TeamSide::Away] {
            let s_idx = side.index();
            let inp = inputs[s_idx];
            let prev = self.state.prev_buttons[s_idx];

            // Manual player switch when not possessing ball on current player
            let switch_rising = inp.pressed(BTN_SWITCH) && (prev & BTN_SWITCH) == 0;
            if switch_rising && self.state.switch_cooldown[s_idx] == 0 {
                let owns_ball = self.state.ball.owner == Some((side, self.state.controlled_idx[s_idx]));
                if !owns_ball {
                    self.switch_player(side, inp.aim_vec());
                    self.state.switch_cooldown[s_idx] = 12;
                }
            }

            // Teammate run trigger when team owns ball (`BTN_CALL_RUN` rising edge)
            let run_rising = inp.pressed(BTN_CALL_RUN) && (prev & BTN_CALL_RUN) == 0;
            if run_rising && self.state.ball.owner.map(|(s, _)| s) == Some(side) {
                self.trigger_teammate_run(side);
            }
        }

        // 2. Step all players (movement, stamina, charging, tackles, GK dives)
        self.step_all_players(&inputs);

        // 3. Resolve active tackles against ball / ball carrier
        self.resolve_tackles();

        // 4. Step 3D ball physics & dribbler ball-carry
        if matches!(self.state.phase, MatchPhase::OpenPlay) {
            self.step_ball(&inputs);
        }

        // 5. Resolve loose ball pickups, pass completions, interceptions, offsides & GK saves
        if matches!(self.state.phase, MatchPhase::OpenPlay) {
            self.resolve_ball_contacts();
        }

        // 6. Player-player soft body separation so models never stack on identical coordinates
        self.resolve_player_separation();

        // 7. Check goal scored or out-of-bounds (throw-in, corner, goal kick)
        if matches!(self.state.phase, MatchPhase::OpenPlay) {
            self.check_boundaries_and_goals();
        }

        // 8. Check halftime / fulltime whistle (let an imminent shot finish if close to goal, else blow whistle)
        if matches!(self.state.phase, MatchPhase::OpenPlay) {
            self.check_clock_whistle();
        }

        self.state.prev_buttons = [inputs[0].buttons, inputs[1].buttons];
    }

    fn arrange_kickoff(&mut self, kicking_team: TeamSide) {
        let half_len = self.state.format.half_length();
        let half_wid = self.state.format.half_width();

        self.state.ball = BallState {
            pos: Vec3::ZERO,
            vel: Vec3::ZERO,
            spin_side: Fp::ZERO,
            spin_top: Fp::ZERO,
            owner: None,
            gk_hold_ticks: 0,
            dribble_touch_cd: 0,
            last_touch_team: Some(kicking_team),
            last_touch_player: None,
            last_touch_meta: None,
            last_team_touch_meta: [None, None],
            pass_in_flight: None,
            assist_candidate: None,
        };

        for side in [TeamSide::Home, TeamSide::Away] {
            let attack_sign = side.attack_sign(self.state.second_half_swapped());
            let count = self.state.players[side.index()].len();
            for (idx, p) in self.state.players[side.index()].iter_mut().enumerate() {
                let (ax_pm, ay_pm) = p.base_anchor_permille;
                // Map `ax_pm` (`-950..+450`) strictly into own half (`-920..-110`)
                let own_half_pm = if idx == 0 {
                    -920
                } else {
                    (-520 + (ax_pm + 500) * 320 / 1000).clamp(-820, -110)
                };
                let wx = Fp::from_milli(own_half_pm)
                    .mul(half_len)
                    .mul(Fp::from_int(attack_sign as i32));
                let wy = Fp::from_milli(ay_pm)
                    .mul(half_wid)
                    .mul(Fp::from_int(attack_sign as i32))
                    .mul(Fp::from_milli(800));

                p.pos = Vec2::new(wx, wy);
                p.vel = Vec2::ZERO;
                p.z = Fp::ZERO;
                p.vz = Fp::ZERO;
                p.facing_bam = if attack_sign > 0 { 0 } else { 32768 };
                p.action = PlayerActionState::Normal;
                p.run_boost_ticks = 0;
            }

            // Pick forward-most non-sent-off player as controlled
            let taker = (1..count)
                .rev()
                .find(|&i| !self.state.players[side.index()][i].sent_off)
                .unwrap_or(0);
            self.state.controlled_idx[side.index()] = taker;
        }

        // Place kicking team's taker at center spot
        let k_idx = self.state.controlled_idx[kicking_team.index()];
        let k_sign = kicking_team.attack_sign(self.state.second_half_swapped());
        self.state.players[kicking_team.index()][k_idx].pos =
            Vec2::from_milli(-k_sign * 380, 180);
        self.state.ball.owner = Some((kicking_team, k_idx));
        self.set_last_touch(kicking_team, k_idx);
        self.state.phase = MatchPhase::KickOff {
            team: kicking_team,
            wait_ticks: 0,
        };
    }

    fn setup_set_piece(&mut self, kind: RestartKind, team: TeamSide, spot: Vec2) {
        let half_len = self.state.format.half_length();
        let attack_sign = team.attack_sign(self.state.second_half_swapped());

        // Pick taker
        let count = self.state.players[team.index()].len();
        let taker_idx = match kind {
            RestartKind::GoalKick => 0,
            RestartKind::PenaltyKick => (1..count)
                .rev()
                .find(|&i| !self.state.players[team.index()][i].sent_off)
                .unwrap_or(0),
            _ => {
                let mut best = 1.min(count.saturating_sub(1));
                let mut best_d = Fp::from_int(9999);
                for (idx, p) in self.state.players[team.index()].iter().enumerate().skip(1) {
                    if p.sent_off {
                        continue;
                    }
                    let d = p.pos.dist(spot);
                    if d < best_d {
                        best_d = d;
                        best = idx;
                    }
                }
                best
            }
        };

        let taker_fid = self.state.players[team.index()][taker_idx]
            .footballer_id
            .clone();
        let taker_fname = self.state.players[team.index()][taker_idx].name.clone();
        let mut set_piece_team_meta: [Option<(String, String)>; 2] = [None, None];
        set_piece_team_meta[team.index()] = Some((taker_fid.clone(), taker_fname.clone()));
        let taker_meta = (team, taker_fid, taker_fname);
        self.state.ball = BallState {
            pos: Vec3::new(spot.x, spot.y, Fp::ZERO),
            vel: Vec3::ZERO,
            spin_side: Fp::ZERO,
            spin_top: Fp::ZERO,
            owner: Some((team, taker_idx)),
            gk_hold_ticks: 0,
            dribble_touch_cd: 0,
            last_touch_team: Some(team),
            last_touch_player: Some((team, taker_idx)),
            last_touch_meta: Some(taker_meta),
            last_team_touch_meta: set_piece_team_meta,
            pass_in_flight: None,
            assist_candidate: None,
        };

        // Reset action states and place taker on spot
        for s in [TeamSide::Home, TeamSide::Away] {
            for p in &mut self.state.players[s.index()] {
                p.vel = Vec2::ZERO;
                p.action = PlayerActionState::Normal;
            }
        }

        let taker = &mut self.state.players[team.index()][taker_idx];
        taker.pos = spot - Vec2::from_milli(attack_sign * 300, 0);
        taker.facing_bam = if attack_sign > 0 { 0 } else { 32768 };
        self.state.controlled_idx[team.index()] = taker_idx;

        // Enforce minimum opponent distance from restart spot & arrange FreeKick defensive wall!
        let opp = team.opponent();
        let is_5v5 = self.state.format.players_per_side <= 6;
        let min_dist = if is_5v5 {
            Fp::from_milli(4_500)
        } else {
            Fp::from_milli(9_150)
        };
        let opp_goal_x = Fp::from_milli(attack_sign * half_len.to_milli());
        let opp_goal = Vec2::new(opp_goal_x, Fp::ZERO);

        if kind == RestartKind::PenaltyKick {
            for (idx, p) in self.state.players[opp.index()].iter_mut().enumerate() {
                if idx == 0 {
                    p.pos = opp_goal;
                } else {
                    p.pos.x = spot.x - Fp::from_milli(attack_sign * 3_500);
                }
            }
        } else {
            for (idx, p) in self.state.players[opp.index()].iter_mut().enumerate() {
                if idx == 0 || p.sent_off {
                    continue;
                }
                let d = p.pos.dist(spot);
                if d < min_dist {
                    let push_dir = if d > Fp::from_milli(100) {
                        (p.pos - spot).normalize_or_zero()
                    } else {
                        Vec2::from_milli(attack_sign * 1_000, 0).normalize_or_zero()
                    };
                    p.pos = spot + push_dir.scale(min_dist);
                }
            }

            // Form a shoulder-to-shoulder Defensive Wall on dangerous Free Kicks!
            let wall_assignments = self.compute_free_kick_wall(kind, team, spot);
            for (w_idx, w_pos, w_facing) in wall_assignments {
                let p = &mut self.state.players[opp.index()][w_idx];
                p.pos = w_pos;
                p.facing_bam = w_facing;
                p.vel = Vec2::ZERO;
            }
        }

        self.state.phase = MatchPhase::SetPiece {
            kind,
            team,
            spot,
            taker_idx,
            wait_ticks: 0,
        };
    }

    fn compute_free_kick_wall(
        &self,
        kind: RestartKind,
        taking_team: TeamSide,
        spot: Vec2,
    ) -> Vec<(usize, Vec2, u16)> {
        if kind != RestartKind::FreeKick {
            return Vec::new();
        }
        let is_5v5 = self.state.format.players_per_side <= 6;
        let half_len = self.state.format.half_length();
        let half_wid = self.state.format.half_width();
        let attack_sign = taking_team.attack_sign(self.state.second_half_swapped());
        let opp_goal = Vec2::new(Fp::from_milli(attack_sign * half_len.to_milli()), Fp::ZERO);
        let dist_to_goal = spot.dist(opp_goal);
        let max_wall_range = if is_5v5 {
            Fp::from_milli(24_000)
        } else {
            Fp::from_milli(35_000)
        };
        if dist_to_goal > max_wall_range || dist_to_goal < Fp::from_milli(5_200) {
            return Vec::new();
        }

        let min_dist = if is_5v5 {
            Fp::from_milli(4_500)
        } else {
            Fp::from_milli(9_150)
        };
        let to_goal = (opp_goal - spot).normalize_or_zero();
        let perp = Vec2::new(-to_goal.y, to_goal.x);
        let wall_center = spot + to_goal.scale(min_dist);
        let face_ball_bam = (spot - wall_center).to_bam();

        let opp = taking_team.opponent();
        let mut eligible: Vec<(i64, usize)> = self.state.players[opp.index()]
            .iter()
            .enumerate()
            .skip(1)
            .filter(|(_, p)| !p.sent_off)
            .map(|(i, p)| (p.pos.dist(wall_center).to_milli(), i))
            .collect();
        eligible.sort_by_key(|&(d, i)| (d, i));

        let wall_count = if is_5v5 {
            2.min(eligible.len())
        } else if spot.y.abs() > Fp::from_milli(14_000) {
            3.min(eligible.len())
        } else {
            4.min(eligible.len())
        };
        if wall_count == 0 {
            return Vec::new();
        }

        let spacing_mm = 580_i64;
        let mut out = Vec::with_capacity(wall_count);
        for (slot, &(_, idx)) in eligible.iter().take(wall_count).enumerate() {
            let offset_mm = (slot as i64) * spacing_mm - ((wall_count as i64 - 1) * spacing_mm) / 2;
            let w_pos = (wall_center + perp.scale(Fp::from_milli(offset_mm)))
                .clamp_box(half_len - Fp::from_milli(600), half_wid - Fp::from_milli(600));
            out.push((idx, w_pos, face_ball_bam));
        }
        out
    }

    fn step_set_piece_movement(
        &mut self,
        kind: RestartKind,
        taking_team: TeamSide,
        taker_idx: usize,
        spot: Vec2,
    ) {
        let dt = Fp::from_ratio(1, TICKS_PER_SECOND as i64);
        let wall = self.compute_free_kick_wall(kind, taking_team, spot);
        let opp = taking_team.opponent();

        for side in [TeamSide::Home, TeamSide::Away] {
            let count = self.state.players[side.index()].len();
            for idx in 0..count {
                if side == taking_team && idx == taker_idx {
                    continue;
                }
                if idx == 0 || self.state.players[side.index()][idx].sent_off {
                    continue;
                }
                if side == opp {
                    if let Some(&(_, w_pos, w_facing)) = wall.iter().find(|&&(wi, _, _)| wi == idx) {
                        let p = &mut self.state.players[side.index()][idx];
                        p.pos = w_pos;
                        p.facing_bam = w_facing;
                        p.vel = Vec2::ZERO;
                        continue;
                    }
                }
                let anchor = world_anchor_for_player(&self.state, side, idx, Fp::ZERO);
                let p = &mut self.state.players[side.index()][idx];
                let delta = anchor - p.pos;
                if delta.len() > Fp::from_milli(500) {
                    let step = delta.normalize_or_zero().scale(Fp::from_milli(3_500)).scale(dt);
                    p.pos += step;
                }
            }
        }
    }

    pub fn apply_substitution(&mut self, side: TeamSide, pitch_slot: usize, bench_idx: usize) -> bool {
        let s_idx = side.index();
        if self.state.subs_used[s_idx] >= 5 {
            return false;
        }
        if pitch_slot >= self.state.players[s_idx].len()
            || bench_idx >= self.state.teams[s_idx].substitutes.len()
        {
            return false;
        }
        if self.state.players[s_idx][pitch_slot].sent_off {
            return false;
        }
        let incoming = self.state.teams[s_idx].substitutes[bench_idx].clone();
        let is_gk_slot = pitch_slot == 0;
        if is_gk_slot != (incoming.role == PlayerRole::Goalkeeper) {
            return false;
        }

        let outgoing_snap = self.state.teams[s_idx].starters[pitch_slot].clone();
        let outgoing_name = self.state.players[s_idx][pitch_slot].name.clone();
        let incoming_id = incoming.footballer_id.clone();
        let incoming_name = incoming.name.clone();

        // Archive outgoing player's stats (or restore if re-entering in rolling 5v5 exhibition!)
        let outgoing_stats = core::mem::replace(
            &mut self.state.player_stats[s_idx][pitch_slot],
            PlayerMatchStats {
                footballer_id: incoming.footballer_id.clone(),
                name: incoming.name.clone(),
                team: Some(side),
                ..PlayerMatchStats::default()
            },
        );
        if let Some(prev_pos) = self.state.archived_player_stats[s_idx]
            .iter()
            .position(|st| st.footballer_id == incoming.footballer_id)
        {
            self.state.player_stats[s_idx][pitch_slot] =
                self.state.archived_player_stats[s_idx].swap_remove(prev_pos);
        }
        self.state.archived_player_stats[s_idx].push(outgoing_stats);

        // Swap TeamSnapshot starters <-> substitutes so bench remains up to date
        self.state.teams[s_idx].starters[pitch_slot] = incoming.clone();
        self.state.teams[s_idx].substitutes[bench_idx] = outgoing_snap;

        // Refresh pitch PlayerState with fresh legs (`1000` permille stamina) while preserving position & anchor
        let p = &mut self.state.players[s_idx][pitch_slot];
        p.footballer_id = incoming.footballer_id;
        p.name = incoming.name;
        p.shirt_number = incoming.shirt_number;
        p.role = incoming.role;
        p.attributes = incoming.attributes;
        p.stamina_permille = 1_000;
        p.yellow_cards = 0;

        self.state.subs_used[s_idx] += 1;

        self.emit_event(
            MatchEventType::Substitution,
            side,
            Some(incoming_id),
            None,
            format!("SUB: {incoming_name} ON for {outgoing_name}"),
        );
        true
    }

    fn step_celebration_drift(&mut self) {
        let dt = Fp::from_ratio(1, TICKS_PER_SECOND as i64);
        for side in [TeamSide::Home, TeamSide::Away] {
            for p in &mut self.state.players[side.index()] {
                p.vel = p.vel.scale(Fp::from_milli(920));
                p.pos += p.vel.scale(dt);
            }
        }
    }

    fn switch_player(&mut self, side: TeamSide, aim_dir: Vec2) {
        let s_idx = side.index();
        let current = self.state.controlled_idx[s_idx];
        let cur_pos = self.state.players[s_idx][current].pos;
        let ball_xy = self.state.ball.pos.xy();
        let attack_sign = side.attack_sign(self.state.second_half_swapped());
        let own_goal_x = Fp::from_milli(-attack_sign * self.state.format.half_length().to_milli());
        let to_own_goal = (Vec2::new(own_goal_x, Fp::ZERO) - ball_xy).normalize_or_zero();
        let use_directional = aim_dir.len() > Fp::from_milli(300);
        let aim_norm = aim_dir.normalize_or_zero();

        let mut best_idx = current;
        let mut best_score = Fp::from_int(-999_999);

        for (idx, p) in self.state.players[s_idx].iter().enumerate() {
            if idx == 0 || idx == current || p.sent_off {
                continue;
            }
            let score = if use_directional {
                let rel = p.pos - cur_pos;
                let dist = rel.len().max(Fp::ONE);
                let align = rel.normalize_or_zero().dot(aim_norm);
                if align < Fp::from_milli(160) {
                    continue;
                }
                align.mul(Fp::from_int(22)) - dist.mul(Fp::from_milli(320))
            } else {
                let (_, intercept_ticks) = best_ball_intercept_for_player(&self.state, side, idx);
                let rel_ball = (p.pos - ball_xy).normalize_or_zero();
                let goal_side_bonus = if rel_ball.dot(to_own_goal) > Fp::from_milli(150) {
                    Fp::from_milli(1_950)
                } else {
                    Fp::ZERO
                };
                -Fp::from_milli((intercept_ticks as i64) * 145) + goal_side_bonus
            };

            if score > best_score {
                best_score = score;
                best_idx = idx;
            }
        }
        self.state.controlled_idx[s_idx] = best_idx;
    }

    fn trigger_teammate_run(&mut self, side: TeamSide) {
        let s_idx = side.index();
        let ctrl = self.state.controlled_idx[s_idx];
        let attack_sign = side.attack_sign(self.state.second_half_swapped());
        let mut best_idx: Option<usize> = None;
        let mut best_prog = i64::MIN;

        for (idx, p) in self.state.players[s_idx].iter().enumerate() {
            if idx == 0 || idx == ctrl || p.sent_off {
                continue;
            }
            let prog = p.pos.x.to_milli() * attack_sign
                + if p.role == PlayerRole::Forward { 6_000 } else { 0 };
            if prog > best_prog {
                best_prog = prog;
                best_idx = Some(idx);
            }
        }
        if let Some(target_idx) = best_idx {
            self.state.players[s_idx][target_idx].run_boost_ticks = 150;
        }
    }

    fn step_all_players(&mut self, inputs: &[InputFrame; 2]) {
        let dt = Fp::from_ratio(1, TICKS_PER_SECOND as i64);
        let half_len = self.state.format.half_length() + Fp::from_milli(1_500);
        let half_wid = self.state.format.half_width() + Fp::from_milli(1_500);

        for side in [TeamSide::Home, TeamSide::Away] {
            let s_idx = side.index();
            let ctrl_idx = self.state.controlled_idx[s_idx];
            let side_input = inputs[s_idx];
            let prev_btns = self.state.prev_buttons[s_idx];
            let gk_rush_held = side_input.pressed(BTN_CALL_RUN)
                && self.state.ball.owner.map(|(s, _)| s) != Some(side);

            let count = self.state.players[s_idx].len();
            for idx in 0..count {
                if self.state.players[s_idx][idx].sent_off {
                    continue;
                }

                if self.state.players[s_idx][idx].run_boost_ticks > 0 {
                    self.state.players[s_idx][idx].run_boost_ticks -= 1;
                }

                let is_gk = idx == 0;
                let is_controlled = idx == ctrl_idx && (!is_gk || self.state.ball.owner == Some((side, 0)));
                let has_ball = self.state.ball.owner == Some((side, idx));

                // Compute movement / dive intent first using immutable state borrow
                let ai_intent = if is_gk {
                    Some(compute_goalkeeper_intent(&self.state, side, gk_rush_held))
                } else if !is_controlled {
                    Some(compute_outfield_ai_intent(&self.state, side, idx))
                } else {
                    None
                };

                // Check charging / action state transitions for controlled player (or GK holding ball)
                let current_tick = self.state.tick;
                let side_attack_sign = side.attack_sign(self.state.second_half_swapped());
                let ball_xy_now = self.state.ball.pos.xy();
                let ball_z_now = self.state.ball.pos.z;
                let ball_is_loose = self.state.ball.owner.is_none();
                let own_team_receiving = ball_is_loose
                    && (self.state.ball.pass_in_flight.as_ref().is_some_and(|pf| pf.team == side)
                        || self.state.ball.last_touch_team == Some(side));
                let is_open_play = matches!(self.state.phase, MatchPhase::OpenPlay);
                let mut kick_to_execute: Option<(u16, u16, bool, Vec2)> = None;
                let mut skill_move_triggered: Option<(String, String, u8)> = None;
                let mut new_buffered: Option<BufferedAction> = None;
                {
                    let p = &mut self.state.players[s_idx][idx];

                    // Step vertical jump kinematics (`p.z`, `p.vz`) for aerial headers & volleys!
                    if p.z > Fp::ZERO || p.vz != Fp::ZERO {
                        p.z += p.vz.mul(dt);
                        p.vz -= Fp::from_milli(9_810).mul(dt);
                        if p.z <= Fp::ZERO {
                            p.z = Fp::ZERO;
                            p.vz = Fp::ZERO;
                        }
                    } else if is_open_play
                        && ball_is_loose
                        && !is_gk
                        && p.can_control_ball()
                        && ball_z_now >= Fp::from_milli(1_180)
                        && ball_z_now <= Fp::from_milli(2_450)
                        && p.pos.dist(ball_xy_now) <= Fp::from_milli(2_350)
                    {
                        // Athletic leap to contest aerial cross / corner header!
                        let jump_boost = 3_280 + (p.attributes.strength as i64) * 3;
                        p.vz = Fp::from_milli(jump_boost);
                    }

                    // Tick down temporary action states
                    match p.action {
                        PlayerActionState::Tackling { ticks_left } => {
                            if ticks_left <= 1 {
                                p.action = PlayerActionState::Cooldown { ticks_left: 8 };
                            } else {
                                p.action = PlayerActionState::Tackling {
                                    ticks_left: ticks_left - 1,
                                };
                            }
                        }
                        PlayerActionState::Sliding { ticks_left, dir } => {
                            if ticks_left <= 1 {
                                p.action = PlayerActionState::Cooldown { ticks_left: 14 };
                            } else {
                                p.action = PlayerActionState::Sliding {
                                    ticks_left: ticks_left - 1,
                                    dir,
                                };
                            }
                        }
                        PlayerActionState::Diving { ticks_left, dir } => {
                            if ticks_left <= 1 {
                                p.action = PlayerActionState::Cooldown { ticks_left: 10 };
                            } else {
                                p.action = PlayerActionState::Diving {
                                    ticks_left: ticks_left - 1,
                                    dir,
                                };
                            }
                        }
                        PlayerActionState::Stumbled { ticks_left } => {
                            if ticks_left <= 1 {
                                p.action = PlayerActionState::Normal;
                            } else {
                                p.action = PlayerActionState::Stumbled {
                                    ticks_left: ticks_left - 1,
                                };
                            }
                        }
                        PlayerActionState::Cooldown { ticks_left } => {
                            if ticks_left <= 1 {
                                p.action = PlayerActionState::Normal;
                            } else {
                                p.action = PlayerActionState::Cooldown {
                                    ticks_left: ticks_left - 1,
                                };
                            }
                        }
                        PlayerActionState::SkillMove {
                            kind,
                            ticks_left,
                            dir,
                        } => {
                            if !has_ball || ticks_left <= 1 {
                                p.action = PlayerActionState::Normal;
                            } else {
                                p.action = PlayerActionState::SkillMove {
                                    kind,
                                    ticks_left: ticks_left - 1,
                                    dir,
                                };
                            }
                        }
                        PlayerActionState::Charging {
                            button,
                            ticks,
                            finesse,
                            aim: prev_aim,
                        } => {
                            if !has_ball {
                                p.action = PlayerActionState::Normal;
                            } else {
                                let still_held = (side_input.buttons & button) != 0;
                                let next_ticks = ticks + 1;
                                let aim = if side_input.aim_vec().len() > Fp::from_milli(150) {
                                    side_input.aim_vec()
                                } else if side_input.move_vec().len() > Fp::from_milli(120) {
                                    side_input.move_vec()
                                } else if prev_aim.len() > Fp::from_milli(120) {
                                    prev_aim
                                } else {
                                    Vec2::from_bam(p.facing_bam, Fp::ONE)
                                };
                                if !still_held || next_ticks >= MAX_CHARGE_TICKS {
                                    p.action = PlayerActionState::Cooldown { ticks_left: 6 };
                                    kick_to_execute = Some((button, next_ticks, finesse, aim));
                                } else {
                                    p.action = PlayerActionState::Charging {
                                        button,
                                        ticks: next_ticks,
                                        finesse: finesse || side_input.pressed(BTN_JOCKEY),
                                        aim,
                                    };
                                }
                            }
                        }
                        PlayerActionState::Normal => {
                            // Trigger GK dive if AI requested it
                            if let Some(intent) = ai_intent {
                                if let Some(dive_dir) = intent.trigger_dive {
                                    p.action = PlayerActionState::Diving {
                                        ticks_left: 18,
                                        dir: dive_dir,
                                    };
                                }
                            }

                            // Controlled player (or human pressing pass/lob while GK holds ball!)
                            let gk_human_distribute =
                                is_gk && has_ball && (side_input.buttons & CHARGE_MASK) != 0;
                            if (is_controlled || gk_human_distribute)
                                && matches!(p.action, PlayerActionState::Normal)
                            {
                                let pressed_kick = side_input.buttons & CHARGE_MASK;
                                let kick_rising = pressed_kick != 0 && (prev_btns & CHARGE_MASK) == 0;
                                if has_ball {
                                    let skill_rising =
                                        side_input.pressed(BTN_SKILL) && (prev_btns & BTN_SKILL) == 0;
                                    let rstick_flick =
                                        !is_gk && side_input.aim_vec().len() > Fp::from_milli(680);
                                    if kick_rising {
                                        // Pick highest-priority single button bit
                                        let single_btn = if (pressed_kick & BTN_SHOOT) != 0 {
                                            BTN_SHOOT
                                        } else if (pressed_kick & BTN_LOB) != 0 {
                                            BTN_LOB
                                        } else if (pressed_kick & BTN_THROUGH) != 0 {
                                            BTN_THROUGH
                                        } else {
                                            BTN_PASS
                                        };
                                        let init_aim = if side_input.aim_vec().len() > Fp::from_milli(150) {
                                            side_input.aim_vec()
                                        } else if side_input.move_vec().len() > Fp::from_milli(120) {
                                            side_input.move_vec()
                                        } else {
                                            Vec2::from_bam(p.facing_bam, Fp::ONE)
                                        };
                                        p.action = PlayerActionState::Charging {
                                            button: single_btn,
                                            ticks: 1,
                                            finesse: side_input.pressed(BTN_JOCKEY),
                                            aim: init_aim,
                                        };
                                    } else if !is_gk && (skill_rising || rstick_flick) {
                                        let kind: u8 = if skill_rising { 1 } else { 2 };
                                        let burst_dir = if rstick_flick {
                                            side_input.aim_vec().normalize_or_zero()
                                        } else if side_input.move_vec().len() > Fp::from_milli(150) {
                                            side_input.move_vec().normalize_or_zero()
                                        } else {
                                            Vec2::from_bam(p.facing_bam, Fp::ONE)
                                        };
                                        p.facing_bam = burst_dir.to_bam();
                                        p.action = PlayerActionState::SkillMove {
                                            kind,
                                            ticks_left: 14,
                                            dir: burst_dir,
                                        };
                                        skill_move_triggered = Some((
                                            p.footballer_id.clone(),
                                            p.name.clone(),
                                            kind,
                                        ));
                                    }
                                } else if !is_gk {
                                    // Pre-contact Input Buffering (`~266ms` lookahead):
                                    // When a pass is arriving from a teammate (or player explicitly presses Shoot/Through
                                    // on a loose ball), buffer a crisp First-Time One-Touch Pass/Volley instead of lunging!
                                    let explicit_first_time =
                                        (pressed_kick & (BTN_SHOOT | BTN_THROUGH)) != 0;
                                    if kick_rising
                                        && ball_is_loose
                                        && (own_team_receiving || explicit_first_time)
                                        && p.pos.dist(ball_xy_now) <= Fp::from_milli(8_500)
                                    {
                                        let single_btn = if (pressed_kick & BTN_SHOOT) != 0 {
                                            BTN_SHOOT
                                        } else if (pressed_kick & BTN_LOB) != 0 {
                                            BTN_LOB
                                        } else if (pressed_kick & BTN_THROUGH) != 0 {
                                            BTN_THROUGH
                                        } else {
                                            BTN_PASS
                                        };
                                        let init_aim = if side_input.aim_vec().len() > Fp::from_milli(150) {
                                            side_input.aim_vec()
                                        } else if side_input.move_vec().len() > Fp::from_milli(120) {
                                            side_input.move_vec()
                                        } else {
                                            Vec2::new(Fp::from_int(side_attack_sign as i32), Fp::ZERO)
                                        };
                                        new_buffered = Some(BufferedAction {
                                            button: single_btn,
                                            aim: init_aim,
                                            finesse: side_input.pressed(BTN_JOCKEY),
                                            ttl_ticks: INPUT_BUFFER_TTL_TICKS,
                                        });
                                    } else if !own_team_receiving {
                                        // Defending tackle / slide triggers
                                        let tackle_rising =
                                            side_input.pressed(BTN_TACKLE) && (prev_btns & BTN_TACKLE) == 0;
                                        let slide_rising =
                                            side_input.pressed(BTN_SLIDE) && (prev_btns & BTN_SLIDE) == 0;
                                        if slide_rising {
                                            let dir = if side_input.move_vec().len() > Fp::from_milli(150) {
                                                side_input.move_vec().normalize_or_zero()
                                            } else {
                                                Vec2::from_bam(p.facing_bam, Fp::ONE)
                                            };
                                            p.facing_bam = dir.to_bam();
                                            p.action = PlayerActionState::Sliding {
                                                ticks_left: 20,
                                                dir,
                                            };
                                        } else if tackle_rising {
                                            p.action = PlayerActionState::Tackling { ticks_left: 12 };
                                        }
                                    }
                                }
                            }
                        }
                    }

                    let is_5v5 = self.state.format.players_per_side <= 6;

                    // Determine desired velocity for this tick
                    let (move_dir, sprint, jockey) = if is_controlled {
                        (
                            side_input.move_vec(),
                            side_input.pressed(BTN_SPRINT),
                            side_input.pressed(BTN_JOCKEY),
                        )
                    } else if let Some(intent) = ai_intent {
                        (intent.move_dir, intent.sprint, intent.jockey)
                    } else {
                        (Vec2::ZERO, false, false)
                    };

                    // Stamina update
                    let moving = move_dir.len() > Fp::from_milli(150);
                    if sprint && moving {
                        // Drain ~1 permille every 8-12 ticks depending on stamina attribute
                        let drain_mod = (125_u32.saturating_sub(p.attributes.stamina as u32)).max(25);
                        if (current_tick * drain_mod) % 420 < drain_mod {
                            p.stamina_permille = (p.stamina_permille - 1).max(150);
                        }
                    } else if !moving && current_tick.is_multiple_of(12) {
                        p.stamina_permille = (p.stamina_permille + 1).min(1_000);
                    }

                    // Compute target velocity based on action & attributes
                    let pace_mm_s = 5_450 + (p.attributes.pace as i64) * 26; // 5.45 - 8.05 m/s base
                    let stamina_scale = Fp::from_milli(760 + (p.stamina_permille as i64) * 240 / 1_000);
                    let mut top_speed = Fp::from_milli(pace_mm_s).mul(stamina_scale);
                    if sprint && !jockey {
                        top_speed = top_speed.mul(Fp::from_milli(if is_5v5 { 1_205 } else { 1_220 }));
                    } else if jockey {
                        top_speed = top_speed.mul(Fp::from_milli(if has_ball { 740 } else { 690 }));
                    }
                    if has_ball {
                        let drib_keep = 885 + (p.attributes.dribbling as i64);
                        top_speed = top_speed.mul(Fp::from_milli(drib_keep.min(975)));
                    }

                    // Anisotropic human speed envelope: 100% forward along heading, ~85% lateral strafe, ~75% backpedal
                    if moving && !matches!(p.action, PlayerActionState::SkillMove { .. }) {
                        let heading = Vec2::from_bam(p.facing_bam, Fp::ONE);
                        let move_norm = move_dir.normalize_or_zero();
                        let align_milli = move_norm.dot(heading).to_milli(); // [-1000, +1000]
                        let dir_scale_milli = (865 + (align_milli * 135) / 1_000).clamp(740, 1_000);
                        top_speed = top_speed.mul(Fp::from_milli(dir_scale_milli));
                    }

                    let desired_vel = match p.action {
                        PlayerActionState::Sliding { ticks_left, dir } => {
                            let burst = Fp::from_milli(4_200 + (ticks_left as i64) * 220);
                            dir.scale(burst)
                        }
                        PlayerActionState::Diving { ticks_left, dir } => {
                            let gk_burst = Fp::from_milli(
                                4_500 + (p.attributes.goalkeeping as i64) * 35 + (ticks_left as i64) * 90,
                            );
                            dir.scale(gk_burst)
                        }
                        PlayerActionState::Tackling { ticks_left } => {
                            let lunge = Fp::from_milli(1_400 + (ticks_left as i64) * 180);
                            Vec2::from_bam(p.facing_bam, lunge)
                        }
                        PlayerActionState::SkillMove { dir, .. } => {
                            let skill_burst =
                                Fp::from_milli(5_950 + (p.attributes.dribbling as i64) * 22);
                            dir.scale(skill_burst)
                        }
                        PlayerActionState::Stumbled { .. } => p.vel.scale(Fp::from_milli(820)),
                        _ => move_dir.scale(top_speed),
                    };

                    // Biomechanical Hill force-velocity launch boost + eccentric foot-plant braking cut
                    let base_accel = if is_5v5 {
                        30_500 + (p.attributes.acceleration as i64) * 220
                    } else {
                        24_000 + (p.attributes.acceleration as i64) * 180
                    };
                    let cur_speed = p.vel.len();
                    let dv = desired_vel - p.vel;
                    let cutting_or_braking = cur_speed > Fp::from_milli(900) && dv.dot(p.vel) < Fp::ZERO;
                    let explosive_start = moving && !cutting_or_braking && cur_speed < Fp::from_milli(3_200);
                    let accel_rate_mm_s2 = if cutting_or_braking {
                        (base_accel * 152) / 100
                    } else if explosive_start {
                        (base_accel * 118) / 100
                    } else {
                        base_accel
                    };
                    let max_dv = Fp::from_milli(accel_rate_mm_s2).mul(dt);
                    p.vel = if dv.len() <= max_dv {
                        desired_vel
                    } else {
                        p.vel + dv.normalize_or_zero().scale(max_dv)
                    };

                    p.pos += p.vel.scale(dt);
                    p.pos.x = p.pos.x.clamp(-half_len, half_len);
                    p.pos.y = p.pos.y.clamp(-half_wid, half_wid);

                    // Keep GK inside own half
                    if is_gk {
                        if side_attack_sign > 0 {
                            p.pos.x = p.pos.x.min(-Fp::from_milli(2_000));
                        } else {
                            p.pos.x = p.pos.x.max(Fp::from_milli(2_000));
                        }
                    }

                    // Centripetal traction turn bound: agile forefoot pivot at low speed vs tighter sprint radius
                    let face_target = if jockey && !has_ball {
                        let to_ball = ball_xy_now - p.pos;
                        if to_ball.len() > Fp::from_milli(100) {
                            to_ball.to_bam()
                        } else {
                            p.facing_bam
                        }
                    } else if move_dir.len() > Fp::from_milli(150) {
                        move_dir.to_bam()
                    } else if p.vel.len() > Fp::from_milli(300) {
                        p.vel.to_bam()
                    } else {
                        p.facing_bam
                    };
                    let spd_mm = p.vel.len().to_milli().clamp(0, 8_500);
                    let pivot_bam: i64 = if is_5v5 { 5_400 } else { 3_900 };
                    let sprint_min_bam: i64 = if is_5v5 { 2_850 } else { 2_100 };
                    let dyn_turn = (pivot_bam - (spd_mm * (pivot_bam - sprint_min_bam)) / 8_500)
                        .clamp(sprint_min_bam, pivot_bam) as u16;
                    p.facing_bam = turn_toward_bam(p.facing_bam, face_target, dyn_turn);
                }

                if let Some(buf) = new_buffered {
                    self.state.input_buffer[s_idx] = Some(buf);
                }

                if let Some((fid, pname, kind)) = skill_move_triggered {
                    self.state.player_stats[s_idx][idx].skill_moves_completed += 1;
                    let move_label = if kind == 1 { "roulette" } else { "step-over" };
                    self.emit_event(
                        MatchEventType::SkillMove,
                        side,
                        Some(fid),
                        None,
                        format!("{pname} executes {move_label}"),
                    );
                }

                if let Some((btn, charge_ticks, finesse, aim)) = kick_to_execute {
                    self.execute_kick(side, idx, btn, charge_ticks, finesse, aim);
                }
            }
        }
    }

    fn execute_kick(
        &mut self,
        side: TeamSide,
        kicker_idx: usize,
        button: u16,
        charge_ticks: u16,
        finesse: bool,
        aim_dir: Vec2,
    ) {
        let s_idx = side.index();
        let is_5v5 = self.state.format.players_per_side <= 6;
        let charge_norm = Fp::from_ratio(charge_ticks.clamp(1, MAX_CHARGE_TICKS) as i64, MAX_CHARGE_TICKS as i64);
        let attack_sign = side.attack_sign(self.state.second_half_swapped());
        let half_len = self.state.format.half_length();
        let goal_half_w = self.state.format.goal_half_width();
        let opp_goal_x = Fp::from_milli(attack_sign * half_len.to_milli());

        let kicker_pos = self.state.players[s_idx][kicker_idx].pos;
        let kicker_attrs = self.state.players[s_idx][kicker_idx].attributes;
        let kicker_id = self.state.players[s_idx][kicker_idx].footballer_id.clone();
        let kicker_name = self.state.players[s_idx][kicker_idx].name.clone();

        self.state.ball.owner = None;
        self.state.ball.gk_hold_ticks = 0;
        self.state.ball.dribble_touch_cd = 0;
        self.set_last_touch(side, kicker_idx);
        self.state.players[s_idx][kicker_idx].action = PlayerActionState::Cooldown {
            ticks_left: if is_5v5 { 7 } else { 10 },
        };

        let aim_norm = if aim_dir.len() > Fp::from_milli(100) {
            aim_dir.normalize_or_zero()
        } else {
            Vec2::new(Fp::from_int(attack_sign as i32), Fp::ZERO)
        };
        self.state.players[s_idx][kicker_idx].facing_bam = aim_norm.to_bam();

        if (button & BTN_SHOOT) != 0 {
            // --- SHOOTING ---
            self.state.ball.pass_in_flight = None;
            self.state.player_stats[s_idx][kicker_idx].shots += 1;

            // Aim inside goal mouth modulated by stick Y + shooting skill scatter
            let stick_y = aim_norm.y.clamp(Fp::NEG_ONE, Fp::ONE);
            let corner_reach = if finesse {
                Fp::from_milli(890)
            } else {
                Fp::from_milli(860)
            };
            let target_y_base = stick_y.mul(goal_half_w.mul(corner_reach));
            let max_err_mm = ((105_i64 - kicker_attrs.shooting as i64).max(8)) * if finesse { 9 } else { 17 };
            let err_y = self.state.rng.range_milli(-max_err_mm, max_err_mm);
            let target_y = target_y_base + err_y;
            let on_target = target_y.abs() <= goal_half_w;
            if on_target {
                self.state.player_stats[s_idx][kicker_idx].shots_on_target += 1;
            }

            let target_xy = Vec2::new(opp_goal_x, target_y);
            let dir_xy = (target_xy - kicker_pos).normalize_or_zero();

            let base_speed_mm = 16_200 + (kicker_attrs.shooting as i64) * 68;
            let charge_boost_mm = charge_norm.mul(Fp::from_milli(9_800)).to_milli();
            let speed = Fp::from_milli(if finesse {
                ((base_speed_mm + charge_boost_mm) * 88) / 100
            } else {
                base_speed_mm + charge_boost_mm
            });

            // Vertical lift: low driven on quick tap, rising dipping strike on higher charge
            let vz = Fp::from_milli(780) + charge_norm.mul(Fp::from_milli(4_200));
            let finesse_curl = if is_5v5 { 5_400 } else { 4_500 };
            let spin = if finesse {
                if target_y >= Fp::ZERO {
                    Fp::from_milli(-finesse_curl)
                } else {
                    Fp::from_milli(finesse_curl)
                }
            } else {
                self.state.rng.range_milli(-850, 850)
            };

            self.state.ball.pos.z = self.state.ball.pos.z.max(Fp::from_milli(110));
            self.state.ball.vel = Vec3::new(dir_xy.x.mul(speed), dir_xy.y.mul(speed), vz);
            self.state.ball.spin_side = spin;
            self.state.ball.spin_top = Fp::from_milli(1_650);

            self.emit_event(
                MatchEventType::Shot,
                side,
                Some(kicker_id),
                None,
                format!("{kicker_name} shoots ({})", if on_target { "on target" } else { "wide" }),
            );
            return;
        }

        // --- PASS / THROUGH BALL / LOB ---
        self.state.player_stats[s_idx][kicker_idx].passes_attempted += 1;
        let is_through = (button & BTN_THROUGH) != 0;
        let is_lob = (button & BTN_LOB) != 0;
        let opp_s_idx = side.opponent().index();

        // Receiver selection: score teammates by alignment with `aim_norm`, distance, and openness
        let mut best_receiver: Option<(usize, Vec2)> = None;
        let mut best_score = Fp::from_int(-999_999);

        for (idx, mate) in self.state.players[s_idx].iter().enumerate() {
            if idx == kicker_idx || mate.sent_off {
                continue;
            }
            // Don't auto-target own GK unless aiming strongly backward
            if idx == 0 && aim_norm.x.mul(Fp::from_int(attack_sign as i32)) > Fp::from_milli(-350) {
                continue;
            }
            let through_lead_mm = if is_5v5 { 3_900 } else { 5_200 };
            let lead_pos = if is_through {
                mate.pos
                    + Vec2::new(Fp::from_int(attack_sign as i32), Fp::ZERO)
                        .scale(Fp::from_milli(through_lead_mm))
                    + mate.vel.scale(Fp::from_milli(330))
            } else {
                mate.pos + mate.vel.scale(Fp::from_milli(260))
            };
            let to_mate = lead_pos - kicker_pos;
            let dist = to_mate.len();
            if dist < Fp::from_milli(1_100) {
                continue;
            }
            let align = to_mate.normalize_or_zero().dot(aim_norm);
            if align < Fp::from_milli(-120) {
                continue;
            }
            let pref_dist = if is_5v5 {
                Fp::from_milli(4_200) + charge_norm.mul(Fp::from_milli(13_500))
            } else {
                Fp::from_milli(5_000) + charge_norm.mul(Fp::from_milli(18_000))
            };
            let dist_diff = (dist - pref_dist).abs();

            // Openness bonus: slightly prefer unmarked teammates in 5v5 passing triangles
            let mut nearest_opp = Fp::from_milli(8_000);
            for opp in self.state.players[opp_s_idx].iter().skip(1) {
                if !opp.sent_off {
                    let od = opp.pos.dist(lead_pos);
                    if od < nearest_opp {
                        nearest_opp = od;
                    }
                }
            }
            let open_bonus = nearest_opp.min(Fp::from_milli(5_000)).mul(Fp::from_milli(520));
            let score = align.mul(Fp::from_int(28)) - dist_diff.mul(Fp::from_milli(360)) + open_bonus;
            if score > best_score {
                best_score = score;
                best_receiver = Some((idx, lead_pos));
            }
        }

        let (receiver_idx, mut target_pos) = best_receiver.unwrap_or_else(|| {
            let fallback_dist = if is_5v5 {
                Fp::from_milli(6_500) + charge_norm.mul(Fp::from_milli(11_000))
            } else {
                Fp::from_milli(8_000) + charge_norm.mul(Fp::from_milli(14_000))
            };
            let fallback_idx = (1..self.state.players[s_idx].len())
                .find(|&i| i != kicker_idx && !self.state.players[s_idx][i].sent_off)
                .unwrap_or(kicker_idx);
            (fallback_idx, kicker_pos + aim_norm.scale(fallback_dist))
        });

        // Clamp through-ball / lob target inside pitch bounds
        let max_x = half_len - Fp::from_milli(1_400);
        let max_y = self.state.format.half_width() - Fp::from_milli(1_200);
        target_pos.x = target_pos.x.clamp(-max_x, max_x);
        target_pos.y = target_pos.y.clamp(-max_y, max_y);

        // Check offside at pass instant if `offsides_enabled`
        let receiver_was_offside =
            self.is_player_offside_at_pass(side, kicker_idx, receiver_idx);

        let delta = target_pos - kicker_pos;
        let dist = delta.len().max(Fp::ONE);
        let pass_err_mm = ((102_i64 - kicker_attrs.passing as i64).max(4)) * 6;
        let lateral_err = self.state.rng.range_milli(-pass_err_mm, pass_err_mm);
        let dir = (delta + Vec2::new(-delta.y, delta.x).normalize_or_zero().scale(lateral_err))
            .normalize_or_zero();

        // Distance-weighted pass kinematics: silky short layoffs (~8.5 m/s) up to crisp driven cross-field balls (~22.5 m/s)
        let (horiz_speed, vz) = if is_lob {
            let spd = (Fp::from_milli(9_600)
                + dist.mul(Fp::from_milli(390))
                + charge_norm.mul(Fp::from_milli(5_600)))
            .clamp(Fp::from_milli(9_600), Fp::from_milli(21_200));
            let lift = (Fp::from_milli(5_100) + charge_norm.mul(Fp::from_milli(4_100)))
                .clamp(Fp::from_milli(4_900), Fp::from_milli(9_200));
            (spd, lift)
        } else {
            let spd = (Fp::from_milli(6_800)
                + dist.mul(Fp::from_milli(490))
                + charge_norm.mul(Fp::from_milli(5_200)))
            .clamp(Fp::from_milli(7_500), Fp::from_milli(22_800));
            let skim_vz = if dist > Fp::from_milli(9_500) || charge_norm > Fp::from_milli(480) {
                Fp::from_milli(180) + charge_norm.mul(Fp::from_milli(360))
            } else {
                Fp::ZERO
            };
            (spd, skim_vz)
        };

        self.state.ball.vel = Vec3::new(dir.x.mul(horiz_speed), dir.y.mul(horiz_speed), vz);
        self.state.ball.spin_side = Fp::ZERO;
        self.state.ball.spin_top = if is_lob {
            Fp::from_milli(-1_450)
        } else {
            Fp::from_milli(-620)
        };
        self.state.ball.pass_in_flight = Some(PassInFlight {
            team: side,
            passer_idx: kicker_idx,
            passer_id: kicker_id.clone(),
            passer_name: kicker_name.clone(),
            intended_receiver_idx: receiver_idx,
            target_pos,
            launched_tick: self.state.tick,
            receiver_was_offside,
        });

        // Auto-switch control to receiver (unless receiver is GK)
        if receiver_idx != 0 {
            self.state.controlled_idx[s_idx] = receiver_idx;
        }

        self.emit_event(
            MatchEventType::Pass,
            side,
            Some(kicker_id),
            None,
            format!("{kicker_name} pass"),
        );
    }

    fn is_player_offside_at_pass(
        &self,
        attacking_side: TeamSide,
        passer_idx: usize,
        receiver_idx: usize,
    ) -> bool {
        if !self.state.rules.offsides_enabled || receiver_idx == passer_idx || receiver_idx == 0 {
            return false;
        }
        let attack_sign = attacking_side.attack_sign(self.state.second_half_swapped());
        let recv_proj = self.state.players[attacking_side.index()][receiver_idx]
            .pos
            .x
            .to_milli()
            * attack_sign;
        // Must be in opponent's half
        if recv_proj <= 0 {
            return false;
        }
        let ball_proj = self.state.ball.pos.x.to_milli() * attack_sign;
        if recv_proj <= ball_proj + 150 {
            return false;
        }
        let opp_side = attacking_side.opponent();
        let mut opp_projs: Vec<i64> = self.state.players[opp_side.index()]
            .iter()
            .filter(|p| !p.sent_off)
            .map(|p| p.pos.x.to_milli() * attack_sign)
            .collect();
        opp_projs.sort_unstable();
        if opp_projs.len() < 2 {
            return false;
        }
        let second_last = opp_projs[opp_projs.len() - 2];
        recv_proj > second_last + 200
    }

    fn resolve_tackles(&mut self) {
        let Some((carrier_side, carrier_idx)) = self.state.ball.owner else {
            return;
        };
        // Cannot slide-tackle a goalkeeper holding the ball in hands
        if carrier_idx == 0 && self.state.ball.gk_hold_ticks > 0 {
            return;
        }

        let def_side = carrier_side.opponent();
        let carrier_pos = self.state.players[carrier_side.index()][carrier_idx].pos;
        let carrier_facing = self.state.players[carrier_side.index()][carrier_idx].facing_bam;
        let ball_xy = self.state.ball.pos.xy();

        let def_count = self.state.players[def_side.index()].len();
        for d_idx in 0..def_count {
            let (is_slide, reach_mm) = match self.state.players[def_side.index()][d_idx].action {
                PlayerActionState::Tackling { ticks_left } if ticks_left >= 5 => (false, 1_250_i64),
                PlayerActionState::Sliding { ticks_left, .. } if ticks_left >= 6 => (true, 1_550_i64),
                _ => continue,
            };

            let def_pos = self.state.players[def_side.index()][d_idx].pos;
            let dist_to_ball = def_pos.dist(ball_xy);
            let dist_to_carrier = def_pos.dist(carrier_pos);
            let reach = Fp::from_milli(reach_mm);

            if dist_to_ball > reach && dist_to_carrier > reach {
                continue;
            }

            // Active SkillMove carrier can evade a lunging tackle based on dribbling skill
            if matches!(
                self.state.players[carrier_side.index()][carrier_idx].action,
                PlayerActionState::SkillMove { .. }
            ) {
                let drib = self.state.players[carrier_side.index()][carrier_idx]
                    .attributes
                    .dribbling as u32;
                let evade_pct = (48 + drib / 2).min(92);
                if self.state.rng.chance_percent(evade_pct) {
                    self.state.players[def_side.index()][d_idx].action =
                        PlayerActionState::Cooldown { ticks_left: 16 };
                    continue;
                }
            }

            let tackle_skill = self.state.players[def_side.index()][d_idx]
                .attributes
                .tackling as u32;
            let def_id = self.state.players[def_side.index()][d_idx]
                .footballer_id
                .clone();
            let def_name = self.state.players[def_side.index()][d_idx].name.clone();

            // Check if challenge went through the back of carrier before reaching ball
            let carrier_forward = Vec2::from_bam(carrier_facing, Fp::ONE);
            let def_to_carrier = (carrier_pos - def_pos).normalize_or_zero();
            let from_behind = def_to_carrier.dot(carrier_forward) > Fp::from_milli(520)
                && dist_to_carrier + Fp::from_milli(120) < dist_to_ball;

            let foul_chance = if from_behind {
                if is_slide {
                    72_u32.saturating_sub(tackle_skill / 4)
                } else {
                    38_u32.saturating_sub(tackle_skill / 5)
                }
            } else if is_slide && dist_to_carrier < dist_to_ball {
                26_u32.saturating_sub(tackle_skill / 5)
            } else {
                0
            };

            if self.state.rules.fouls_enabled && self.state.rng.chance_percent(foul_chance) {
                self.handle_foul(def_side, d_idx, carrier_side, carrier_idx, is_slide, from_behind);
                return;
            }

            // Clean tackle won!
            self.state.player_stats[def_side.index()][d_idx].tackles_won += 1;
            self.state.players[carrier_side.index()][carrier_idx].action =
                PlayerActionState::Stumbled { ticks_left: 20 };
            self.state.players[def_side.index()][d_idx].action =
                PlayerActionState::Cooldown { ticks_left: 6 };

            if is_slide {
                // Slide pokes the ball loose in slide direction
                let poke_dir = Vec2::from_bam(
                    self.state.players[def_side.index()][d_idx].facing_bam,
                    Fp::from_milli(6_500),
                );
                self.state.ball.owner = None;
                self.state.ball.vel = Vec3::new(poke_dir.x, poke_dir.y, Fp::from_milli(400));
            } else {
                self.state.ball.owner = Some((def_side, d_idx));
                self.state.ball.dribble_touch_cd = 6;
                self.state.ball.vel = Vec3::ZERO;
                if d_idx != 0 {
                    self.state.controlled_idx[def_side.index()] = d_idx;
                }
            }
            self.set_last_touch(def_side, d_idx);
            self.state.ball.pass_in_flight = None;
            self.state.ball.assist_candidate = None;

            self.emit_event(
                MatchEventType::Tackle,
                def_side,
                Some(def_id),
                None,
                format!("{def_name} wins tackle"),
            );
            return;
        }
    }

    fn handle_foul(
        &mut self,
        fouling_side: TeamSide,
        fouler_idx: usize,
        victim_side: TeamSide,
        victim_idx: usize,
        is_slide: bool,
        from_behind: bool,
    ) {
        let foul_pos = self.state.players[victim_side.index()][victim_idx].pos;
        let fouler_id = self.state.players[fouling_side.index()][fouler_idx]
            .footballer_id
            .clone();
        let fouler_name = self.state.players[fouling_side.index()][fouler_idx]
            .name
            .clone();

        self.state.player_stats[fouling_side.index()][fouler_idx].fouls_committed += 1;
        self.emit_event(
            MatchEventType::Foul,
            fouling_side,
            Some(fouler_id.clone()),
            None,
            format!("Foul by {fouler_name}"),
        );

        // Card check (never send off GK in 5v5 so each team always keeps a goalkeeper!)
        if fouler_idx != 0 && (is_slide || from_behind) {
            let card_pct = if is_slide && from_behind { 55 } else { 25 };
            if self.state.rng.chance_percent(card_pct) {
                self.state.players[fouling_side.index()][fouler_idx].yellow_cards += 1;
                self.state.player_stats[fouling_side.index()][fouler_idx].yellow_cards += 1;
                self.emit_event(
                    MatchEventType::YellowCard,
                    fouling_side,
                    Some(fouler_id.clone()),
                    None,
                    format!("Yellow card for {fouler_name}"),
                );
                if self.state.players[fouling_side.index()][fouler_idx].yellow_cards >= 2 {
                    self.state.players[fouling_side.index()][fouler_idx].sent_off = true;
                    self.state.players[fouling_side.index()][fouler_idx].pos =
                        Vec2::from_milli(0, self.state.format.pitch_width_mm);
                    self.state.player_stats[fouling_side.index()][fouler_idx].red_cards += 1;
                    self.emit_event(
                        MatchEventType::RedCard,
                        fouling_side,
                        Some(fouler_id),
                        None,
                        format!("Second yellow — {fouler_name} sent off"),
                    );
                }
            }
        }

        // Check if foul occurred inside `fouling_side`'s own penalty box -> Penalty!
        let victim_attack_sign = victim_side.attack_sign(self.state.second_half_swapped());
        let half_len = self.state.format.half_length();
        let opp_goal_x = Fp::from_milli(victim_attack_sign * half_len.to_milli());
        let in_box = (foul_pos.x - opp_goal_x).abs() <= self.state.format.penalty_box_depth()
            && foul_pos.y.abs() <= self.state.format.penalty_box_half_width();

        if in_box {
            let pen_dist = (self.state.format.penalty_box_depth_mm * 2) / 3;
            let pen_spot = Vec2::new(
                opp_goal_x - Fp::from_milli(victim_attack_sign * pen_dist),
                Fp::ZERO,
            );
            self.emit_event(
                MatchEventType::PenaltyKick,
                victim_side,
                None,
                None,
                "Penalty kick awarded!".to_string(),
            );
            self.setup_set_piece(RestartKind::PenaltyKick, victim_side, pen_spot);
        } else {
            // Advantage rule: in attacking half, if fouler wasn't sent off and an open teammate
            // is within 9.5m in an advantageous position, play on!
            let fouler_sent_off = self.state.players[fouling_side.index()][fouler_idx].sent_off;
            let in_attacking_half = foul_pos.x.to_milli() * victim_attack_sign > 0;
            let mut advantage_mate: Option<usize> = None;
            if !fouler_sent_off && in_attacking_half {
                let mut best_d = Fp::from_milli(9_500);
                for (m_idx, mate) in self.state.players[victim_side.index()].iter().enumerate() {
                    if m_idx == 0 || m_idx == victim_idx || mate.sent_off {
                        continue;
                    }
                    let d = mate.pos.dist(foul_pos);
                    let prog_ok = mate.pos.x.to_milli() * victim_attack_sign
                        >= foul_pos.x.to_milli() * victim_attack_sign - 2_500;
                    if d <= best_d && prog_ok {
                        best_d = d;
                        advantage_mate = Some(m_idx);
                    }
                }
            }

            if let Some(mate_idx) = advantage_mate {
                let mate_id = self.state.players[victim_side.index()][mate_idx]
                    .footballer_id
                    .clone();
                let mate_name = self.state.players[victim_side.index()][mate_idx]
                    .name
                    .clone();
                let mate_pos = self.state.players[victim_side.index()][mate_idx].pos;
                self.state.players[fouling_side.index()][fouler_idx].action =
                    PlayerActionState::Cooldown { ticks_left: 22 };
                self.state.players[victim_side.index()][victim_idx].action =
                    PlayerActionState::Stumbled { ticks_left: 10 };
                self.state.ball.owner = Some((victim_side, mate_idx));
                self.state.ball.dribble_touch_cd = 6;
                self.state.ball.pos = Vec3::new(mate_pos.x, mate_pos.y, Fp::ZERO);
                self.state.ball.vel = Vec3::ZERO;
                self.set_last_touch(victim_side, mate_idx);
                self.state.controlled_idx[victim_side.index()] = mate_idx;
                self.emit_event(
                    MatchEventType::Advantage,
                    victim_side,
                    Some(mate_id),
                    None,
                    format!("Advantage played to {mate_name}!"),
                );
            } else {
                self.emit_event(
                    MatchEventType::FreeKick,
                    victim_side,
                    None,
                    None,
                    "Free kick awarded".to_string(),
                );
                self.setup_set_piece(RestartKind::FreeKick, victim_side, foul_pos);
            }
        }
    }

    fn step_ball(&mut self, inputs: &[InputFrame; 2]) {
        let dt = Fp::from_ratio(1, TICKS_PER_SECOND as i64);

        // Case A: Ball controlled by a player
        if let Some((side, idx)) = self.state.ball.owner {
            let p = &self.state.players[side.index()][idx];
            if p.sent_off {
                self.state.ball.owner = None;
                return;
            }
            // Goalkeeper holding caught ball
            if idx == 0 && self.state.ball.gk_hold_ticks > 0 {
                let carry_xy = p.pos + Vec2::from_bam(p.facing_bam, Fp::from_milli(420));
                self.state.ball.pos = Vec3::new(carry_xy.x, carry_xy.y, Fp::from_milli(1_150));
                self.state.ball.vel = Vec3::new(p.vel.x, p.vel.y, Fp::ZERO);
                self.state.ball.gk_hold_ticks -= 1;
                if self.state.ball.gk_hold_ticks == 0 {
                    // Auto-distribute upfield if human hasn't released yet
                    let attack_sign = side.attack_sign(self.state.second_half_swapped());
                    let aim = Vec2::new(Fp::from_int(attack_sign as i32), Fp::from_milli(200))
                        .normalize_or_zero();
                    self.execute_kick(side, 0, BTN_PASS, 15, false, aim);
                }
                return;
            }

            // 1. Vertical aerial trap settle: if trapped above grass (chest/thigh), settle under gravity + cushioned bounce!
            let mut next_z = self.state.ball.pos.z;
            let mut next_vz = self.state.ball.vel.z;
            if next_z > Fp::ZERO || next_vz != Fp::ZERO {
                next_vz -= Fp::from_milli(9_810).mul(dt);
                next_z += next_vz.mul(dt);
                if next_z <= Fp::ZERO {
                    next_z = Fp::ZERO;
                    if next_vz < Fp::from_milli(-720) {
                        next_vz = (-next_vz).mul(Fp::from_milli(360));
                    } else {
                        next_vz = Fp::ZERO;
                    }
                }
            }

            // 2. Impulse-Based 360° Dribbling: free rolling ball between discrete rhythmic boot touches!
            let inp = inputs[side.index()];
            let drib_skill = p.attributes.dribbling as i64;
            let p_speed = p.vel.len();
            let facing_dir = Vec2::from_bam(p.facing_bam, Fp::ONE);
            let carry_dir = if p_speed > Fp::from_milli(380) {
                facing_dir
                    .lerp(p.vel.normalize_or_zero(), Fp::from_milli(480))
                    .normalize_or_zero()
            } else {
                facing_dir
            };

            let (touch_cd_ticks, lead_mm, max_reach_mm, touch_gain) = if inp.pressed(BTN_JOCKEY) {
                // Close sole/laces control: frequent soft touches right at the boot
                (6_u8, 305_i64, 435_i64, Fp::from_milli(6_200))
            } else if inp.pressed(BTN_SPRINT) && p_speed > Fp::from_milli(3_600) {
                // Open-field sprint knock-on: stride push ahead into space, ball rolls freely between strides!
                (12_u8, 720_i64, 910_i64, Fp::from_milli(5_600))
            } else {
                // Balanced 360° stride carry (Personality+ dribbling tightness)
                let tight_lead = (445 - (drib_skill - 75) * 2).clamp(385, 480);
                (9_u8, tight_lead, 620_i64, Fp::from_milli(5_800))
            };

            let prev_ball_xy = self.state.ball.pos.xy();
            let mut vel_xy = self.state.ball.vel.xy();

            // Natural turf deceleration while ball rolls between stride touches
            let roll_decel = Fp::from_milli(1_850).mul(dt);
            let cur_spd = vel_xy.len();
            if cur_spd <= roll_decel {
                vel_xy = Vec2::ZERO;
            } else {
                vel_xy = (vel_xy - vel_xy.normalize_or_zero().scale(roll_decel)).scale(Fp::from_milli(996));
            }

            if self.state.ball.dribble_touch_cd > 0 {
                self.state.ball.dribble_touch_cd -= 1;
            }

            let rel_to_ball = prev_ball_xy - p.pos;
            let rel_dist = rel_to_ball.len();
            let align_with_carry = if rel_dist > Fp::from_milli(60) {
                rel_to_ball.normalize_or_zero().dot(carry_dir)
            } else {
                Fp::NEG_ONE
            };
            // Trigger discrete boot touch when stride cadence fires, or when cutting/turning or ball crowds the feet
            let cut_trigger = p_speed > Fp::from_milli(320) && align_with_carry < Fp::from_milli(700);
            let crowd_trigger = rel_dist < Fp::from_milli(235);
            let did_touch = self.state.ball.dribble_touch_cd == 0 || cut_trigger || crowd_trigger;
            if did_touch {
                let ideal_xy = p.pos + carry_dir.scale(Fp::from_milli(lead_mm));
                if p_speed <= Fp::from_milli(220) && rel_dist <= Fp::from_milli(440) {
                    // Standing shield / sole trap: settle ball gently in front of laces
                    vel_xy = (ideal_xy - prev_ball_xy).scale(Fp::from_milli(4_200));
                } else {
                    // Rhythmic stride tap: impart forward rolling velocity ahead of carrier!
                    let tap_impulse = (ideal_xy - prev_ball_xy).scale(touch_gain);
                    vel_xy = p.vel.scale(Fp::from_milli(1_040)) + tap_impulse;
                    let max_tap_spd = (p_speed + Fp::from_milli(3_400)).max(Fp::from_milli(2_800));
                    vel_xy = vel_xy.clamp_len(max_tap_spd);
                }
                self.state.ball.dribble_touch_cd = touch_cd_ticks;
                self.state.ball.spin_side = Fp::ZERO;
                self.state.ball.spin_top = Fp::from_milli(520);
            }

            let mut next_xy = prev_ball_xy + vel_xy.scale(dt);

            // Soft outer biomechanical leg-reach bound so ball never separates unnaturally or overlaps torso
            let offset = next_xy - p.pos;
            let off_len = offset.len();
            let max_reach = Fp::from_milli(max_reach_mm);
            let min_clear = Fp::from_milli(210);
            if off_len > max_reach {
                next_xy = p.pos + offset.normalize_or_zero().scale(max_reach);
                vel_xy = (next_xy - prev_ball_xy).scale(Fp::from_int(TICKS_PER_SECOND as i32));
            } else if off_len < min_clear {
                next_xy = p.pos + carry_dir.scale(min_clear);
                vel_xy = (next_xy - prev_ball_xy).scale(Fp::from_int(TICKS_PER_SECOND as i32));
            }

            self.state.ball.vel = Vec3::new(vel_xy.x, vel_xy.y, next_vz);
            self.state.ball.pos = Vec3::new(next_xy.x, next_xy.y, next_z);
            if did_touch {
                self.set_last_touch(side, idx);
            }
            return;
        }

        // Case B: Free 3D flight / bounce / roll
        let prev_pos = self.state.ball.pos;
        let is_5v5 = self.state.format.players_per_side <= 6;

        // Horizontal Magnus curve from side spin when moving through air
        let speed_xy = self.state.ball.vel.xy().len();
        if speed_xy > Fp::from_milli(1_800) && self.state.ball.spin_side.abs() > Fp::from_milli(45) {
            let perp = Vec2::new(-self.state.ball.vel.y, self.state.ball.vel.x).normalize_or_zero();
            let magnus_accel = self.state.ball.spin_side.mul(Fp::from_milli(1_050));
            self.state.ball.vel.x += perp.x.mul(magnus_accel).mul(dt);
            self.state.ball.vel.y += perp.y.mul(magnus_accel).mul(dt);
            self.state.ball.spin_side = self.state.ball.spin_side.mul(Fp::from_milli(991));
        }

        // Quadratic aerodynamic drag + vertical Magnus lift/dip in air vs natural grass rolling friction
        if self.state.ball.pos.z > Fp::from_milli(45) {
            let speed_3d = (self.state.ball.vel.xy().len_sq()
                + self.state.ball.vel.z.mul(self.state.ball.vel.z))
            .sqrt();
            // Quadratic drag: a_drag = -k_d * |v| * v (k_d = 0.0135 m^-1)
            let drag_loss = speed_3d.mul(Fp::from_milli(14)).mul(dt);
            let air_keep = (Fp::ONE - drag_loss).clamp(Fp::from_milli(978), Fp::from_milli(999));
            self.state.ball.vel.x = self.state.ball.vel.x.mul(air_keep);
            self.state.ball.vel.y = self.state.ball.vel.y.mul(air_keep);
            self.state.ball.vel.z = self.state.ball.vel.z.mul(air_keep);

            // Vertical Magnus force: negative spin_top (backspin) lifts lobs; positive spin_top (topspin) dips shots
            if speed_xy > Fp::from_milli(2_200) && self.state.ball.spin_top.abs() > Fp::from_milli(40) {
                let vert_magnus = (-self.state.ball.spin_top).mul(Fp::from_milli(720));
                self.state.ball.vel.z += vert_magnus.mul(dt);
                self.state.ball.spin_top = self.state.ball.spin_top.mul(Fp::from_milli(992));
            }
            self.state.ball.vel.z -= Fp::from_milli(9_810).mul(dt);
        } else {
            // Natural football turf rolling resistance: steady rolling decel + subtle grass-blade drag
            let decel = Fp::from_milli(if is_5v5 { 1_750 } else { 1_880 }).mul(dt);
            let v_xy = self.state.ball.vel.xy();
            let len = v_xy.len();
            if len <= decel {
                self.state.ball.vel.x = Fp::ZERO;
                self.state.ball.vel.y = Fp::ZERO;
            } else {
                let next_xy = v_xy - v_xy.normalize_or_zero().scale(decel);
                let grass_viscous = Fp::from_milli(997);
                self.state.ball.vel.x = next_xy.x.mul(grass_viscous);
                self.state.ball.vel.y = next_xy.y.mul(grass_viscous);
            }
            self.state.ball.spin_top = self.state.ball.spin_top.mul(Fp::from_milli(930));
            self.state.ball.spin_side = self.state.ball.spin_side.mul(Fp::from_milli(940));
            if self.state.ball.vel.z < Fp::ZERO {
                self.state.ball.vel.z = Fp::ZERO;
            }
            self.state.ball.pos.z = Fp::ZERO;
        }

        // Advance position
        self.state.ball.pos = self.state.ball.pos + self.state.ball.vel.scale(dt);

        // Ground bounce with spin-to-turf friction coupling (backspin checks up; topspin skids forward)
        if self.state.ball.pos.z < Fp::ZERO {
            self.state.ball.pos.z = Fp::ZERO;
            if self.state.ball.vel.z < Fp::from_milli(-780) {
                let restitution = if is_5v5 {
                    Fp::from_milli(615)
                } else {
                    Fp::from_milli(590)
                };
                let horiz_keep = if self.state.ball.spin_top < Fp::from_milli(-300) {
                    Fp::from_milli(800)
                } else if self.state.ball.spin_top > Fp::from_milli(400) {
                    Fp::from_milli(915)
                } else {
                    Fp::from_milli(880)
                };
                self.state.ball.vel.z = (-self.state.ball.vel.z).mul(restitution);
                self.state.ball.vel.x = self.state.ball.vel.x.mul(horiz_keep);
                self.state.ball.vel.y = self.state.ball.vel.y.mul(horiz_keep);
                self.state.ball.spin_top = self.state.ball.spin_top.mul(Fp::from_milli(650));
            } else {
                self.state.ball.vel.z = Fp::ZERO;
            }
        }

        // Goalpost & crossbar collisions
        self.check_goal_frame_collisions(prev_pos);
    }

    fn check_goal_frame_collisions(&mut self, prev_pos: Vec3) {
        let half_len = self.state.format.half_length();
        let goal_half_w = self.state.format.goal_half_width();
        let goal_h = self.state.format.goal_height();
        let post_radius = Fp::from_milli(170); // ball radius (110mm) + post radius (60mm)

        for sign in [-1_i64, 1_i64] {
            let goal_x = Fp::from_milli(sign * half_len.to_milli());
            if (self.state.ball.pos.x - goal_x).abs() > Fp::from_milli(650) {
                continue;
            }

            // Left & right vertical posts (tube center offset by 60mm so clear mouth = 2*goal_half_w)
            let post_center_y = goal_half_w + Fp::from_milli(60);
            let bar_center_z = goal_h + Fp::from_milli(60);
            let mut hit_woodwork = false;
            if self.state.ball.pos.z <= bar_center_z + post_radius {
                for post_y in [-post_center_y, post_center_y] {
                    let post_xy = Vec2::new(goal_x, post_y);
                    let delta = self.state.ball.pos.xy() - post_xy;
                    let dist = delta.len();
                    if dist < post_radius && dist > Fp::ZERO {
                        let normal = delta.normalize_or_zero();
                        let v_xy = self.state.ball.vel.xy();
                        let vn = v_xy.dot(normal);
                        if vn < Fp::ZERO {
                            let bounced = (v_xy - normal.scale(vn.mul(Fp::from_int(2))))
                                .scale(Fp::from_milli(720));
                            self.state.ball.vel.x = bounced.x;
                            self.state.ball.vel.y = bounced.y;
                            let pushed = post_xy + normal.scale(post_radius + Fp::from_milli(10));
                            self.state.ball.pos.x = pushed.x;
                            self.state.ball.pos.y = pushed.y;
                            self.state.ball.pass_in_flight = None;
                            hit_woodwork = true;
                        }
                    }
                }
            }

            // Horizontal crossbar
            let crossed_goal_plane = (prev_pos.x - goal_x).signum_int()
                != (self.state.ball.pos.x - goal_x).signum_int()
                || (self.state.ball.pos.x - goal_x).abs() < post_radius;
            if crossed_goal_plane
                && self.state.ball.pos.y.abs() <= post_center_y + post_radius
                && (self.state.ball.pos.z - bar_center_z).abs() <= post_radius
            {
                self.state.ball.vel.x = (-self.state.ball.vel.x).mul(Fp::from_milli(680));
                self.state.ball.vel.z = self.state.ball.vel.z.abs().mul(Fp::from_milli(550))
                    + Fp::from_milli(1_200);
                self.state.ball.pos.x = goal_x - Fp::from_milli(sign * 200);
                self.state.ball.pass_in_flight = None;
                hit_woodwork = true;
            }

            if hit_woodwork {
                let shoot_side = self.state.ball.last_touch_team.unwrap_or(TeamSide::Home);
                self.emit_event(
                    MatchEventType::Woodwork,
                    shoot_side,
                    None,
                    None,
                    "Rattles the woodwork!".to_string(),
                );
            }
        }
    }

    fn resolve_ball_contacts(&mut self) {
        if self.state.ball.owner.is_some() {
            return;
        }

        let ball_xy = self.state.ball.pos.xy();
        let ball_z = self.state.ball.pos.z;
        let ball_speed = self.state.ball.vel.xy().len();
        let half_len = self.state.format.half_length();

        // Collect eligible candidates sorted deterministically by distance to ball
        let mut candidates: Vec<(i64, TeamSide, usize)> = Vec::new();
        for side in [TeamSide::Home, TeamSide::Away] {
            for (idx, p) in self.state.players[side.index()].iter().enumerate() {
                if !p.can_control_ball() {
                    continue;
                }
                let attack_sign = side.attack_sign(self.state.second_half_swapped());
                let own_goal_x = Fp::from_milli(-attack_sign * half_len.to_milli());
                let in_own_box = (p.pos.x - own_goal_x).abs()
                    <= self.state.format.penalty_box_depth()
                    && p.pos.y.abs() <= self.state.format.penalty_box_half_width();

                let is_intended_recv = self
                    .state
                    .ball
                    .pass_in_flight
                    .as_ref()
                    .is_some_and(|pf| pf.team == side && pf.intended_receiver_idx == idx);

                let (horiz_r, max_z) = if idx == 0 && in_own_box {
                    let gk_bonus = (p.attributes.goalkeeping as i64) * 5;
                    let dive_bonus = if matches!(p.action, PlayerActionState::Diving { .. }) {
                        420
                    } else {
                        0
                    };
                    (
                        Fp::from_milli(820 + gk_bonus + dive_bonus),
                        Fp::from_milli(2_250),
                    )
                } else {
                    // Regulation 110mm ball + anatomical leg/chest reach (~520mm, +65mm anticipation for intended receiver)
                    let reach_mm = if is_intended_recv { 585 } else { 520 };
                    (Fp::from_milli(reach_mm), Fp::from_milli(1_480) + p.z)
                };

                let dist = p.pos.dist(ball_xy);
                if dist <= horiz_r && ball_z <= max_z {
                    candidates.push((dist.to_milli(), side, idx));
                }
            }
        }

        candidates.sort_by_key(|&(d, side, idx)| (d, side.index(), idx));

        let Some((_, side, idx)) = candidates.first().copied() else {
            return;
        };

        let s_idx = side.index();
        let player_id = self.state.players[s_idx][idx].footballer_id.clone();
        let player_name = self.state.players[s_idx][idx].name.clone();

        // 1. Check offside violation if attacker touches an offside pass first!
        if let Some(ref pass) = self.state.ball.pass_in_flight {
            if pass.team == side && pass.receiver_was_offside {
                let spot = self.state.players[s_idx][idx].pos;
                self.emit_event(
                    MatchEventType::Offside,
                    side,
                    Some(player_id),
                    None,
                    format!("{player_name} flagged offside"),
                );
                self.setup_set_piece(RestartKind::FreeKick, side.opponent(), spot);
                return;
            }
        }

        // 2. Goalkeeper save / claim inside own penalty area!
        let attack_sign = side.attack_sign(self.state.second_half_swapped());
        let own_goal_x = Fp::from_milli(-attack_sign * half_len.to_milli());
        let in_own_box = idx == 0
            && (ball_xy.x - own_goal_x).abs() <= self.state.format.penalty_box_depth()
            && ball_xy.y.abs() <= self.state.format.penalty_box_half_width();

        if in_own_box && self.state.ball.last_touch_team == Some(side.opponent()) {
            let gk_attr = self.state.players[s_idx][0].attributes.goalkeeping as i64;
            let vel_toward_own_goal = if attack_sign > 0 {
                -self.state.ball.vel.x
            } else {
                self.state.ball.vel.x
            };
            let was_shot_threat = vel_toward_own_goal > Fp::from_milli(6_500);
            let catch_limit = Fp::from_milli(14_500 + gk_attr * 60);

            if ball_speed <= catch_limit {
                // Clean catch!
                self.state.ball.owner = Some((side, 0));
                self.state.ball.gk_hold_ticks = 105;
                self.state.ball.dribble_touch_cd = 0;
                self.state.ball.vel = Vec3::ZERO;
                self.state.ball.pass_in_flight = None;
                self.state.ball.assist_candidate = None;
                self.set_last_touch(side, 0);
                if was_shot_threat {
                    self.state.player_stats[s_idx][0].saves += 1;
                    self.emit_event(
                        MatchEventType::Save,
                        side,
                        Some(player_id),
                        None,
                        format!("{player_name} catches the shot!"),
                    );
                }
                return;
            } else {
                // Strong shot -> GK parries wide! Corner-bound shots are tipped past the post for a Corner Kick!
                let parry_y_sign = if self.state.ball.pos.y >= Fp::ZERO {
                    Fp::ONE
                } else {
                    Fp::NEG_ONE
                };
                let tip_behind_for_corner = self.state.ball.pos.y.abs() > Fp::from_milli(920);
                let (rebound_x, rebound_y) = if tip_behind_for_corner {
                    (
                        Fp::from_int(-attack_sign as i32).mul(Fp::from_milli(4_800)),
                        parry_y_sign.mul(Fp::from_milli(16_200)),
                    )
                } else {
                    (
                        Fp::from_int(attack_sign as i32).mul(Fp::from_milli(5_200)),
                        parry_y_sign.mul(Fp::from_milli(9_500)),
                    )
                };
                self.state.ball.vel = Vec3::new(rebound_x, rebound_y, Fp::from_milli(1_800));
                self.set_last_touch(side, 0);
                self.state.ball.pass_in_flight = None;
                self.state.players[s_idx][0].action = PlayerActionState::Cooldown { ticks_left: 14 };
                if was_shot_threat {
                    self.state.player_stats[s_idx][0].saves += 1;
                    self.emit_event(
                        MatchEventType::Save,
                        side,
                        Some(player_id),
                        None,
                        format!("{player_name} parries wide!"),
                    );
                }
                return;
            }
        }

        // 3. Physical body block / ricochet deflection when a driven strike hits an opponent outfielder!
        let drib_attr = self.state.players[s_idx][idx].attributes.dribbling as i64;
        let opp_control_threshold = Fp::from_milli(10_200 + drib_attr * 52);
        if ball_speed > opp_control_threshold && self.state.ball.last_touch_team != Some(side) {
            let p_pos = self.state.players[s_idx][idx].pos;
            let p_vel = self.state.players[s_idx][idx].vel;
            let v_in = self.state.ball.vel.xy();
            let pre_ball_xy = ball_xy - v_in.scale(Fp::from_ratio(1, TICKS_PER_SECOND as i64));
            let normal = {
                let d = pre_ball_xy - p_pos;
                if d.len() > Fp::from_milli(25) {
                    d.normalize_or_zero()
                } else {
                    (-v_in).normalize_or_zero()
                }
            };
            let vn = v_in.dot(normal);
            let reflected = if vn < Fp::ZERO {
                v_in - normal.scale(vn.mul(Fp::from_int(2)))
            } else {
                normal.scale(ball_speed.mul(Fp::from_milli(700)))
            };
            let defl_xy = reflected
                .scale(Fp::from_milli(510))
                + p_vel.scale(Fp::from_milli(350));
            let defl_vz = self.state.ball.vel.z.mul(Fp::from_milli(420)) + Fp::from_milli(1_150);
            let pushed_xy = p_pos + normal.scale(Fp::from_milli(550));
            self.state.ball.pos.x = pushed_xy.x;
            self.state.ball.pos.y = pushed_xy.y;
            self.state.ball.vel = Vec3::new(defl_xy.x, defl_xy.y, defl_vz);
            self.state.ball.spin_top = Fp::from_milli(-450);
            self.set_last_touch(side, idx);
            self.state.ball.pass_in_flight = None;
            self.state.ball.assist_candidate = None;
            self.state.players[s_idx][idx].action = PlayerActionState::Cooldown { ticks_left: 11 };
            return;
        }

        // 4. Clean trap & possession (with substitution-safe pass & assist attribution)!
        if let Some(pass) = self.state.ball.pass_in_flight.take() {
            let same_athlete = if !pass.passer_id.is_empty() {
                pass.passer_id == player_id
            } else {
                pass.passer_idx == idx
            };
            if pass.team == side && !same_athlete {
                if !pass.passer_id.is_empty() {
                    if let Some(st) = self.state.player_stats_mut_by_id(side, &pass.passer_id) {
                        st.passes_completed += 1;
                    }
                } else if pass.passer_idx < self.state.player_stats[s_idx].len() {
                    self.state.player_stats[s_idx][pass.passer_idx].passes_completed += 1;
                }
                self.state.ball.assist_candidate = Some(AssistCandidate {
                    team: side,
                    passer_idx: pass.passer_idx,
                    passer_id: pass.passer_id,
                    passer_name: pass.passer_name,
                    receiver_idx: idx,
                    receiver_id: player_id.clone(),
                    completed_tick: self.state.tick,
                });
            } else if pass.team != side {
                self.state.player_stats[s_idx][idx].interceptions += 1;
                self.state.ball.assist_candidate = None;
                self.emit_event(
                    MatchEventType::Interception,
                    side,
                    Some(player_id),
                    None,
                    format!("{player_name} intercepts pass"),
                );
            }
        } else if self.state.ball.last_touch_team != Some(side) {
            self.state.ball.assist_candidate = None;
        }

        let incoming_xy = self.state.ball.vel.xy();
        let recv_vel = self.state.players[s_idx][idx].vel;
        let cushion_keep = Fp::from_milli((290 - drib_attr).clamp(150, 250));
        let cushioned_xy = if idx == 0 && in_own_box {
            Vec2::ZERO
        } else {
            incoming_xy.scale(cushion_keep) + recv_vel.scale(Fp::from_milli(760))
        };

        // Aerial chest/thigh cushioning: preserve elevated `ball_z` above 180mm so it drops smoothly under gravity!
        let (trapped_z, trapped_vz) = if idx == 0 && in_own_box {
            (Fp::from_milli(1_150), Fp::ZERO)
        } else if ball_z > Fp::from_milli(180) {
            let damped_vz = self
                .state
                .ball
                .vel
                .z
                .min(Fp::ZERO)
                .mul(Fp::from_milli(320))
                - Fp::from_milli(480);
            (ball_z, damped_vz)
        } else {
            (Fp::ZERO, Fp::ZERO)
        };

        self.state.ball.owner = Some((side, idx));
        self.state.ball.gk_hold_ticks = if idx == 0 && in_own_box { 75 } else { 0 };
        self.state.ball.dribble_touch_cd = 6;
        self.state.ball.pos.z = trapped_z;
        self.state.ball.vel = Vec3::new(cushioned_xy.x, cushioned_xy.y, trapped_vz);
        self.set_last_touch(side, idx);

        if idx != 0 {
            self.state.controlled_idx[s_idx] = idx;
        }

        // Consume any pre-buffered command (`input_buffer`) for an instant First-Time One-Touch Pass / Volley!
        if idx != 0 {
            if let Some(buffered) = self.state.input_buffer[s_idx].take() {
                self.state.players[s_idx][idx].action = PlayerActionState::Cooldown { ticks_left: 6 };
                let one_touch_charge: u16 = if (buffered.button & BTN_SHOOT) != 0 { 13 } else { 9 };
                self.execute_kick(
                    side,
                    idx,
                    buffered.button,
                    one_touch_charge,
                    buffered.finesse,
                    buffered.aim,
                );
            }
        }
    }

    fn resolve_player_separation(&mut self) {
        let min_sep = Fp::from_milli(760);
        // Flatten indices (side_idx, player_idx)
        let mut refs: Vec<(usize, usize)> = Vec::with_capacity(22);
        for (s, side_players) in self.state.players.iter().enumerate() {
            for (i, p) in side_players.iter().enumerate() {
                if !p.sent_off {
                    refs.push((s, i));
                }
            }
        }
        for a in 0..refs.len() {
            for b in (a + 1)..refs.len() {
                let (sa, ia) = refs[a];
                let (sb, ib) = refs[b];
                let pa = self.state.players[sa][ia].pos;
                let pb = self.state.players[sb][ib].pos;
                let delta = pb - pa;
                let dist = delta.len();
                if dist < min_sep {
                    let push_dir = if dist > Fp::from_milli(10) {
                        delta.normalize_or_zero()
                    } else {
                        Vec2::new(Fp::ONE, Fp::ZERO)
                    };
                    let overlap = min_sep - dist;
                    let str_a = (45 + self.state.players[sa][ia].attributes.strength as i64).max(50);
                    let str_b = (45 + self.state.players[sb][ib].attributes.strength as i64).max(50);
                    let total_str = str_a + str_b;
                    // Stronger player yields less ground in shoulder-to-shoulder jostling!
                    let share_a = Fp::from_ratio(str_b, total_str);
                    let share_b = Fp::from_ratio(str_a, total_str);
                    self.state.players[sa][ia].pos -= push_dir.scale(overlap.mul(share_a));
                    self.state.players[sb][ib].pos += push_dir.scale(overlap.mul(share_b));

                    // Damp inward closing normal velocity so bodies don't jitter through each other
                    let rel_v = self.state.players[sb][ib].vel - self.state.players[sa][ia].vel;
                    let closing = rel_v.dot(push_dir);
                    if closing < Fp::ZERO {
                        let impulse = (-closing).mul(Fp::from_milli(750));
                        self.state.players[sa][ia].vel -= push_dir.scale(impulse.mul(share_a));
                        self.state.players[sb][ib].vel += push_dir.scale(impulse.mul(share_b));
                    }
                }
            }
        }
    }

    fn check_boundaries_and_goals(&mut self) {
        let half_len = self.state.format.half_length();
        let half_wid = self.state.format.half_width();
        let goal_half_w = self.state.format.goal_half_width();
        let goal_h = self.state.format.goal_height();
        let ball_margin = Fp::from_milli(110);

        let bx = self.state.ball.pos.x;
        let by = self.state.ball.pos.y;
        let bz = self.state.ball.pos.z;

        // 1. Goal line (`x > +half_len` or `x < -half_len`)
        if bx.abs() > half_len + ball_margin {
            let crossed_pos_x = bx > Fp::ZERO;
            // Which team attacks the goal that was just crossed?
            // If Home attacks +X (`attack_sign == +1`), then `+half_len` is Home's target goal (Away's own goal).
            let home_attacks_pos_x = TeamSide::Home.attack_sign(self.state.second_half_swapped()) > 0;
            let attacking_team = if crossed_pos_x == home_attacks_pos_x {
                TeamSide::Home
            } else {
                TeamSide::Away
            };
            let defending_team = attacking_team.opponent();

            if by.abs() < goal_half_w && bz < goal_h {
                // GOAL SCORED!
                self.record_goal(attacking_team);
                return;
            }

            // Out over goal line -> CornerKick or GoalKick
            self.state.ball.owner = None;
            let goal_sign = if crossed_pos_x { 1_i64 } else { -1_i64 };
            if self.state.ball.last_touch_team == Some(defending_team) {
                let corner_y_sign = if by >= Fp::ZERO { 1_i64 } else { -1_i64 };
                let spot = Vec2::from_milli(
                    goal_sign * (half_len.to_milli() - 500),
                    corner_y_sign * (half_wid.to_milli() - 500),
                );
                self.emit_event(
                    MatchEventType::CornerKick,
                    attacking_team,
                    None,
                    None,
                    "Corner kick".to_string(),
                );
                self.setup_set_piece(RestartKind::CornerKick, attacking_team, spot);
            } else {
                let spot = Vec2::from_milli(goal_sign * (half_len.to_milli() - 4_200), 0);
                self.emit_event(
                    MatchEventType::GoalKick,
                    defending_team,
                    None,
                    None,
                    "Goal kick".to_string(),
                );
                self.setup_set_piece(RestartKind::GoalKick, defending_team, spot);
            }
            return;
        }

        // 2. Touchline (`y > +half_wid` or `y < -half_wid`)
        if by.abs() > half_wid + ball_margin {
            self.state.ball.owner = None;
            let throw_team = self
                .state
                .ball
                .last_touch_team
                .map(|t| t.opponent())
                .unwrap_or(TeamSide::Home);
            let y_sign = if by >= Fp::ZERO { 1_i64 } else { -1_i64 };
            let clamped_x = bx.clamp(
                -(half_len - Fp::from_milli(1_000)),
                half_len - Fp::from_milli(1_000),
            );
            let spot = Vec2::new(clamped_x, Fp::from_milli(y_sign * (half_wid.to_milli() - 250)));
            self.emit_event(
                MatchEventType::ThrowIn,
                throw_team,
                None,
                None,
                "Throw-in".to_string(),
            );
            self.setup_set_piece(RestartKind::ThrowIn, throw_team, spot);
        }
    }

    fn record_goal(&mut self, scoring_team: TeamSide) {
        let s_idx = scoring_team.index();
        self.state.score[s_idx] += 1;
        self.state.ball.owner = None;
        self.state.ball.vel = self.state.ball.vel.scale(Fp::from_milli(250));

        // Credit scorer using substitution-safe `last_touch_meta`, then per-team `last_team_touch_meta`
        // (so shots deflected off an opponent GK/defender still credit an outgoing substituted shooter!)
        let (scorer_id, scorer_name, scorer_idx) = match self.state.ball.last_touch_meta.clone() {
            Some((touch_side, fid, fname)) if touch_side == scoring_team && !fid.is_empty() => {
                let cur_slot = self.state.players[s_idx]
                    .iter()
                    .position(|p| p.footballer_id == fid)
                    .or_else(|| {
                        self.state
                            .ball
                            .last_touch_player
                            .and_then(|(ts, idx)| (ts == scoring_team).then_some(idx))
                    })
                    .unwrap_or((self.state.players[s_idx].len() - 1).max(1));
                (fid, fname, cur_slot)
            }
            _ => {
                if let Some((fid, fname)) = self.state.ball.last_team_touch_meta[s_idx].clone() {
                    if !fid.is_empty() {
                        let cur_slot = self.state.players[s_idx]
                            .iter()
                            .position(|p| p.footballer_id == fid)
                            .unwrap_or((self.state.players[s_idx].len() - 1).max(1));
                        (fid, fname, cur_slot)
                    } else {
                        let slot = (self.state.players[s_idx].len() - 1).max(1);
                        (
                            self.state.players[s_idx][slot].footballer_id.clone(),
                            self.state.players[s_idx][slot].name.clone(),
                            slot,
                        )
                    }
                } else {
                    let slot = match self.state.ball.last_touch_player {
                        Some((touch_side, idx)) if touch_side == scoring_team => idx,
                        _ => {
                            let ball_xy = self.state.ball.pos.xy();
                            let mut best = (self.state.players[s_idx].len() - 1).max(1);
                            let mut best_d = Fp::from_int(9999);
                            for (i, p) in self.state.players[s_idx].iter().enumerate().skip(1) {
                                let d = p.pos.dist(ball_xy);
                                if d < best_d {
                                    best_d = d;
                                    best = i;
                                }
                            }
                            best
                        }
                    };
                    (
                        self.state.players[s_idx][slot].footballer_id.clone(),
                        self.state.players[s_idx][slot].name.clone(),
                        slot,
                    )
                }
            }
        };

        if let Some(st) = self.state.player_stats_mut_by_id(scoring_team, &scorer_id) {
            st.goals += 1;
        } else if scorer_idx < self.state.player_stats[s_idx].len() {
            self.state.player_stats[s_idx][scorer_idx].goals += 1;
        }

        // Check assist within 420 ticks (7 seconds) using substitution-safe identity
        let mut assist_id: Option<String> = None;
        let mut detail = format!("GOAL! {scorer_name}");
        if let Some(cand) = self.state.ball.assist_candidate.clone() {
            let recv_matches = if !cand.receiver_id.is_empty() {
                cand.receiver_id == scorer_id
            } else {
                cand.receiver_idx == scorer_idx
            };
            let distinct_passer = if !cand.passer_id.is_empty() {
                cand.passer_id != scorer_id
            } else {
                cand.passer_idx != scorer_idx
            };
            if cand.team == scoring_team
                && recv_matches
                && distinct_passer
                && self.state.tick.saturating_sub(cand.completed_tick) <= 420
            {
                let (a_id, a_name) = if !cand.passer_id.is_empty() {
                    (cand.passer_id.clone(), cand.passer_name.clone())
                } else {
                    (
                        self.state.players[s_idx][cand.passer_idx]
                            .footballer_id
                            .clone(),
                        self.state.players[s_idx][cand.passer_idx].name.clone(),
                    )
                };
                if let Some(ast) = self.state.player_stats_mut_by_id(scoring_team, &a_id) {
                    ast.assists += 1;
                } else if cand.passer_idx < self.state.player_stats[s_idx].len() {
                    self.state.player_stats[s_idx][cand.passer_idx].assists += 1;
                }
                assist_id = Some(a_id);
                detail = format!("GOAL! {scorer_name} (assist: {a_name})");
            }
        }
        self.state.ball.assist_candidate = None;
        self.state.ball.pass_in_flight = None;

        self.emit_event(
            MatchEventType::Goal,
            scoring_team,
            Some(scorer_id),
            assist_id,
            detail,
        );

        self.state.phase = MatchPhase::GoalScored {
            scoring_team,
            scorer_idx,
            celebration_ticks: 90,
        };
    }

    fn check_clock_whistle(&mut self) {
        let half_limit = self.state.rules.half_duration_ticks;
        let full_limit = half_limit.saturating_mul(2);
        let et_half = (half_limit / 3).max(180);
        let et_first_limit = full_limit.saturating_add(et_half);
        let et_full_limit = full_limit.saturating_add(et_half.saturating_mul(2));

        // Allow up to 90 ticks (1.5s) injury grace if a shot is actively flying near a goal
        let ball_near_goal = self.state.ball.owner.is_none()
            && self.state.ball.pos.x.abs()
                > self.state.format.half_length() - Fp::from_milli(10_000)
            && self.state.ball.vel.xy().len() > Fp::from_milli(7_000);

        if self.state.half == 1 && self.state.clock_ticks >= half_limit {
            if ball_near_goal && self.state.clock_ticks < half_limit + 90 {
                return;
            }
            self.emit_event(
                MatchEventType::HalfTime,
                TeamSide::Home,
                None,
                None,
                format!(
                    "Half-time: {} {} - {} {}",
                    self.state.teams[0].short_name,
                    self.state.score[0],
                    self.state.score[1],
                    self.state.teams[1].short_name
                ),
            );
            self.state.phase = MatchPhase::HalfTime { wait_ticks: 60 };
        } else if self.state.half == 2 && self.state.clock_ticks >= full_limit {
            if ball_near_goal && self.state.clock_ticks < full_limit + 90 {
                return;
            }
            if self.state.rules.extra_time_enabled && self.state.score[0] == self.state.score[1] {
                self.emit_event(
                    MatchEventType::HalfTime,
                    TeamSide::Home,
                    None,
                    None,
                    format!(
                        "End of regulation ({} - {}) — Extra Time!",
                        self.state.score[0], self.state.score[1]
                    ),
                );
                self.state.phase = MatchPhase::HalfTime { wait_ticks: 60 };
            } else {
                self.finish_full_time();
            }
        } else if self.state.half == 3 && self.state.clock_ticks >= et_first_limit {
            if ball_near_goal && self.state.clock_ticks < et_first_limit + 90 {
                return;
            }
            self.emit_event(
                MatchEventType::HalfTime,
                TeamSide::Home,
                None,
                None,
                format!(
                    "Extra time half-time: {} {} - {} {}",
                    self.state.teams[0].short_name,
                    self.state.score[0],
                    self.state.score[1],
                    self.state.teams[1].short_name
                ),
            );
            self.state.phase = MatchPhase::HalfTime { wait_ticks: 45 };
        } else if self.state.half >= 4 && self.state.clock_ticks >= et_full_limit {
            if ball_near_goal && self.state.clock_ticks < et_full_limit + 90 {
                return;
            }
            if self.state.score[0] == self.state.score[1] {
                self.resolve_penalty_shootout();
            }
            self.finish_full_time();
        }
    }

    fn finish_full_time(&mut self) {
        let summary = match self.state.shootout_score {
            Some([hp, ap]) => format!(
                "Full-time (AET, Pens {hp}-{ap}): {} {} - {} {}",
                self.state.teams[0].short_name,
                self.state.score[0],
                self.state.score[1],
                self.state.teams[1].short_name
            ),
            None if self.state.went_to_extra_time => format!(
                "Full-time (AET): {} {} - {} {}",
                self.state.teams[0].short_name,
                self.state.score[0],
                self.state.score[1],
                self.state.teams[1].short_name
            ),
            None => format!(
                "Full-time: {} {} - {} {}",
                self.state.teams[0].short_name,
                self.state.score[0],
                self.state.score[1],
                self.state.teams[1].short_name
            ),
        };
        self.emit_event(MatchEventType::FullTime, TeamSide::Home, None, None, summary);
        self.state.phase = MatchPhase::FullTime;
    }

    fn resolve_penalty_shootout(&mut self) {
        self.emit_event(
            MatchEventType::PenaltyShootoutStart,
            TeamSide::Home,
            None,
            None,
            "Scores level after 120' — Penalty Shootout!".to_string(),
        );

        let takers_for = |players: &[PlayerState]| -> Vec<usize> {
            let mut order: Vec<usize> = (1..players.len())
                .rev()
                .filter(|&i| !players[i].sent_off)
                .collect();
            if !players[0].sent_off {
                order.push(0);
            }
            if order.is_empty() {
                order.push(0);
            }
            order
        };

        let home_order = takers_for(&self.state.players[0]);
        let away_order = takers_for(&self.state.players[1]);
        let mut h_pens: u8 = 0;
        let mut a_pens: u8 = 0;

        for round in 0..25_usize {
            // Home kick
            {
                let idx = home_order[round % home_order.len()];
                let sht = self.state.players[0][idx].attributes.shooting as i32;
                let gk = self.state.players[1][0].attributes.goalkeeping as i32;
                let fid = self.state.players[0][idx].footballer_id.clone();
                let name = self.state.players[0][idx].name.clone();
                let pct = (58 + sht / 2 - gk / 3).clamp(48, 88) as u32;
                if self.state.rng.chance_percent(pct) {
                    h_pens = h_pens.saturating_add(1);
                    self.emit_event(
                        MatchEventType::ShootoutGoal,
                        TeamSide::Home,
                        Some(fid),
                        None,
                        format!("Shootout R{}: {name} scores! ({h_pens}-{a_pens})", round + 1),
                    );
                } else {
                    self.emit_event(
                        MatchEventType::ShootoutMiss,
                        TeamSide::Home,
                        Some(fid),
                        None,
                        format!("Shootout R{}: {name} saved/missed ({h_pens}-{a_pens})", round + 1),
                    );
                }
            }

            if round < 5 {
                let rem_h = (4 - round) as u8;
                let rem_a = (5 - round) as u8;
                if h_pens > a_pens + rem_a || a_pens > h_pens + rem_h {
                    break;
                }
            }

            // Away kick
            {
                let idx = away_order[round % away_order.len()];
                let sht = self.state.players[1][idx].attributes.shooting as i32;
                let gk = self.state.players[0][0].attributes.goalkeeping as i32;
                let fid = self.state.players[1][idx].footballer_id.clone();
                let name = self.state.players[1][idx].name.clone();
                let pct = (58 + sht / 2 - gk / 3).clamp(48, 88) as u32;
                if self.state.rng.chance_percent(pct) {
                    a_pens = a_pens.saturating_add(1);
                    self.emit_event(
                        MatchEventType::ShootoutGoal,
                        TeamSide::Away,
                        Some(fid),
                        None,
                        format!("Shootout R{}: {name} scores! ({h_pens}-{a_pens})", round + 1),
                    );
                } else {
                    self.emit_event(
                        MatchEventType::ShootoutMiss,
                        TeamSide::Away,
                        Some(fid),
                        None,
                        format!("Shootout R{}: {name} saved/missed ({h_pens}-{a_pens})", round + 1),
                    );
                }
            }

            if round < 5 {
                let rem = (4 - round) as u8;
                if h_pens > a_pens + rem || a_pens > h_pens + rem {
                    break;
                }
            } else if h_pens != a_pens {
                break;
            }
        }

        if h_pens == a_pens {
            if self.state.rng.chance_percent(50) {
                h_pens = h_pens.saturating_add(1);
            } else {
                a_pens = a_pens.saturating_add(1);
            }
        }
        self.state.shootout_score = Some([h_pens, a_pens]);
    }

    fn emit_event(
        &mut self,
        event_type: MatchEventType,
        team: TeamSide,
        footballer_id: Option<String>,
        assist_footballer_id: Option<String>,
        detail: String,
    ) {
        let ev = MatchEvent {
            tick: self.state.tick,
            minute: self.state.display_minute(),
            event_type,
            team,
            footballer_id,
            assist_footballer_id,
            pos_mm: (
                self.state.ball.pos.x.to_milli(),
                self.state.ball.pos.y.to_milli(),
            ),
            detail,
        };
        self.state.pending_events.push(ev.clone());
        self.state.recorded_events.push(ev);
    }
}

// Re-export angle_diff_bam / atan2_bam silence check if needed by callers
const _: fn(u16, u16) -> i16 = angle_diff_bam;
const _: fn(Fp, Fp) -> u16 = atan2_bam;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{CpuDifficulty, MatchFormat, SideController};

    #[test]
    fn five_a_side_setup_has_four_outfield_plus_one_goalkeeper_per_side() {
        let snap = FixtureSnapshot::sample_five_a_side(42);
        let sim = MatchSimulation::new(&snap).expect("valid 5v5 snapshot");
        assert_eq!(sim.state().players[0].len(), 5);
        assert_eq!(sim.state().players[1].len(), 5);
        assert!(sim.state().players[0][0].is_goalkeeper());
        assert!(sim.state().players[1][0].is_goalkeeper());
        assert_eq!(
            sim.state().players[0]
                .iter()
                .filter(|p| !p.is_goalkeeper())
                .count(),
            4
        );
    }

    #[test]
    fn identical_inputs_and_seed_produce_bit_identical_hash_series() {
        let mut snap = FixtureSnapshot::sample_five_a_side(2026_11_18);
        snap.rules.home_controller = SideController::Cpu(CpuDifficulty::Normal);
        snap.rules.away_controller = SideController::Cpu(CpuDifficulty::Hard);
        snap.rules.half_duration_ticks = 900; // 15s halves

        let mut sim_a = MatchSimulation::new(&snap).unwrap();
        let mut sim_b = MatchSimulation::new(&snap).unwrap();

        for _ in 0..2_200 {
            sim_a.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
            sim_b.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
            assert_eq!(sim_a.state_hash(), sim_b.state_hash());
        }
        assert!(sim_a.is_finished());
        assert_eq!(sim_a.state().score, sim_b.state().score);
    }

    #[test]
    fn rollback_save_and_load_restores_exact_future_trajectory() {
        let mut snap = FixtureSnapshot::sample_five_a_side(777);
        snap.rules.home_controller = SideController::Cpu(CpuDifficulty::Hard);
        snap.rules.away_controller = SideController::Cpu(CpuDifficulty::Hard);

        let mut sim = MatchSimulation::new(&snap).unwrap();
        for _ in 0..240 {
            sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
        }
        let saved = sim.save_state().unwrap();
        let checkpoint_hash = sim.state_hash();

        for _ in 0..120 {
            sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
        }
        let target_hash_at_360 = sim.state_hash();

        // Mutate further, then rewind back to tick 240 and re-simulate 120 ticks!
        for _ in 0..50 {
            sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
        }
        sim.load_state(&saved).unwrap();
        assert_eq!(sim.state_hash(), checkpoint_hash);

        for _ in 0..120 {
            sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
        }
        assert_eq!(sim.state_hash(), target_hash_at_360);
    }

    #[test]
    fn parameterized_eleven_a_side_preset_runs_with_offsides_enabled() {
        let mut snap = FixtureSnapshot::sample_eleven_a_side(99);
        assert_eq!(snap.format, MatchFormat::eleven_a_side());
        assert!(snap.rules.offsides_enabled);
        snap.rules.home_controller = SideController::Cpu(CpuDifficulty::Normal);
        snap.rules.away_controller = SideController::Cpu(CpuDifficulty::Normal);
        snap.rules.half_duration_ticks = 300;

        let mut sim = MatchSimulation::new(&snap).unwrap();
        assert_eq!(sim.state().players[0].len(), 11);
        assert_eq!(sim.state().players[1].len(), 11);
        for _ in 0..800 {
            sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
        }
        assert!(sim.is_finished());
    }

    #[test]
    fn goal_and_goalkeeper_save_mechanics_work_in_5v5() {
        let mut snap = FixtureSnapshot::sample_five_a_side(12345);
        snap.rules.home_controller = SideController::Human;
        snap.rules.away_controller = SideController::Human;
        let mut sim = MatchSimulation::new(&snap).unwrap();

        // Kick off into OpenPlay
        sim.tick(&[
            InputFrame {
                move_x: -80,
                move_y: 40,
                buttons: BTN_PASS,
                ..InputFrame::IDLE
            },
            InputFrame::IDLE,
        ]);
        assert_eq!(sim.state().phase, MatchPhase::OpenPlay);

        // 1. Place striker in front of open net (move away GK aside) and shoot -> Goal!
        sim.state.players[1][0].pos = Vec2::from_milli(27_000, -9_000);
        sim.state.players[0][4].pos = Vec2::from_milli(22_000, 0);
        sim.state.players[0][4].action = PlayerActionState::Normal;
        sim.state.controlled_idx[0] = 4;
        sim.state.ball.owner = Some((TeamSide::Home, 4));
        sim.state.ball.pass_in_flight = None;
        sim.state.ball.pos = Vec3::from_milli(22_500, 0, 0);
        sim.state.prev_buttons = [0, 0];

        // Charge shot for 6 ticks and release
        for _ in 0..6 {
            sim.tick(&[
                InputFrame {
                    move_x: 127,
                    move_y: 0,
                    buttons: BTN_SHOOT,
                    ..InputFrame::IDLE
                },
                InputFrame::IDLE,
            ]);
        }
        for _ in 0..60 {
            sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
            if sim.state().score[0] == 1 {
                break;
            }
        }
        assert_eq!(sim.state().score[0], 1, "expected Home goal on open net");
        assert!(
            sim.state()
                .recorded_events
                .iter()
                .any(|e| e.event_type == MatchEventType::Goal)
        );

        // 2. Test Goalkeeper shot-stopping save!
        sim.state.phase = MatchPhase::OpenPlay;
        // Move outfielders away from shot lane so GK Raya makes the save cleanly
        for p in sim.state.players[1].iter_mut().skip(1) {
            p.pos = Vec2::from_milli(5_000, 12_000);
        }
        sim.state.players[1][0].pos = Vec2::from_milli(26_500, 0);
        sim.state.players[1][0].vel = Vec2::ZERO;
        sim.state.players[1][0].action = PlayerActionState::Normal;

        sim.state.players[0][4].pos = Vec2::from_milli(17_500, 0);
        sim.state.players[0][4].action = PlayerActionState::Normal;
        sim.state.controlled_idx[0] = 4;
        sim.state.ball.owner = Some((TeamSide::Home, 4));
        sim.state.ball.pos = Vec3::from_milli(18_000, 0, 0);
        sim.state.prev_buttons = [0, 0];

        // Low tap shot directly toward goal mouth -> GK catches / parries and records Save!
        sim.tick(&[
            InputFrame {
                move_x: 127,
                move_y: 0,
                buttons: BTN_SHOOT,
                ..InputFrame::IDLE
            },
            InputFrame::IDLE,
        ]);
        for _ in 0..60 {
            sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
            if sim.state().player_stats[1][0].saves >= 1 {
                break;
            }
        }
        assert!(
            sim.state().player_stats[1][0].saves >= 1,
            "expected Away Goalkeeper to record a Save"
        );
    }

    #[test]
    fn skill_move_execution_and_right_stick_flick_work() {
        let mut snap = FixtureSnapshot::sample_five_a_side(31415);
        snap.rules.home_controller = SideController::Human;
        snap.rules.away_controller = SideController::Human;
        let mut sim = MatchSimulation::new(&snap).unwrap();

        sim.state.phase = MatchPhase::OpenPlay;
        sim.state.controlled_idx[0] = 4;
        sim.state.players[0][4].pos = Vec2::from_milli(5_000, 0);
        sim.state.players[0][4].action = PlayerActionState::Normal;
        sim.state.ball.owner = Some((TeamSide::Home, 4));
        sim.state.ball.pos = Vec3::from_milli(5_500, 0, 0);
        sim.state.prev_buttons = [0, 0];

        // Trigger roulette via BTN_SKILL
        sim.tick(&[
            InputFrame {
                move_x: 127,
                move_y: 20,
                buttons: BTN_SKILL,
                ..InputFrame::IDLE
            },
            InputFrame::IDLE,
        ]);
        assert!(matches!(
            sim.state().players[0][4].action,
            PlayerActionState::SkillMove { kind: 1, .. }
        ));
        assert_eq!(sim.state().player_stats[0][4].skill_moves_completed, 1);
        assert!(
            sim.state()
                .recorded_events
                .iter()
                .any(|e| e.event_type == MatchEventType::SkillMove)
        );
    }

    #[test]
    fn knockout_extra_time_and_penalty_shootout_produce_deterministic_winner() {
        let mut snap = FixtureSnapshot::sample_five_a_side(8888);
        snap.rules.extra_time_enabled = true;
        snap.rules.half_duration_ticks = 180;
        snap.rules.home_controller = SideController::Human;
        snap.rules.away_controller = SideController::Human;

        let mut sim_a = MatchSimulation::new(&snap).unwrap();
        let mut sim_b = MatchSimulation::new(&snap).unwrap();

        // Both human sides stay idle after auto-kickoffs -> 0-0 draw through regulation & extra time -> shootout!
        for _ in 0..1_600 {
            sim_a.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
            sim_b.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
            if sim_a.is_finished() && sim_b.is_finished() {
                break;
            }
        }

        assert!(sim_a.is_finished());
        assert!(sim_a.state().went_to_extra_time);
        let pens = sim_a
            .state()
            .shootout_score
            .expect("expected shootout score after level extra time");
        assert_ne!(pens[0], pens[1], "shootout must produce a winner");
        assert_eq!(sim_a.state().shootout_score, sim_b.state().shootout_score);
        assert_eq!(sim_a.state_hash(), sim_b.state_hash());
    }

    #[test]
    fn custom_five_a_side_lineup_formations_and_physics_tuning_work() {
        let mut snap = FixtureSnapshot::sample_five_a_side(2026_55);
        snap.home.formation = "2-2".to_string();
        snap.away.formation = "1-1-2".to_string();
        // Swap an official-XI substitute into the 5v5 starting lineup if present
        if let Some(sub) = snap.home.substitutes.pop() {
            let old = snap.home.starters.remove(1);
            snap.home.starters.insert(1, sub);
            snap.home.substitutes.push(old);
        }
        snap.rules.home_controller = SideController::Human;
        snap.rules.away_controller = SideController::Human;

        let mut sim = MatchSimulation::new(&snap).expect("valid custom 5v5 lineup snapshot");
        assert_eq!(sim.state().teams[0].formation, "2-2");
        assert_eq!(sim.state().teams[1].formation, "1-1-2");

        // Enter OpenPlay and verify tight sole-roll vs sprint knock-on carry offset
        sim.state.phase = MatchPhase::OpenPlay;
        sim.state.controlled_idx[0] = 4;
        sim.state.players[0][4].pos = Vec2::from_milli(-4_000, 0);
        sim.state.players[0][4].facing_bam = 0;
        sim.state.ball.owner = Some((TeamSide::Home, 4));
        sim.state.ball.pos = Vec3::from_milli(-3_600, 0, 0);

        for _ in 0..10 {
            sim.tick(&[
                InputFrame {
                    move_x: 127,
                    move_y: 0,
                    buttons: BTN_JOCKEY,
                    ..InputFrame::IDLE
                },
                InputFrame::IDLE,
            ]);
        }
        let close_dist = sim.state().players[0][4]
            .pos
            .dist(sim.state().ball.pos.xy());

        for _ in 0..14 {
            sim.tick(&[
                InputFrame {
                    move_x: 127,
                    move_y: 0,
                    buttons: BTN_SPRINT,
                    ..InputFrame::IDLE
                },
                InputFrame::IDLE,
            ]);
        }
        let sprint_dist = sim.state().players[0][4]
            .pos
            .dist(sim.state().ball.pos.xy());
        assert!(
            close_dist < sprint_dist,
            "BTN_JOCKEY close-control carry ({close_dist:?}) must stay tighter than sprint knock-on ({sprint_dist:?})"
        );

        // Verify neutral BTN_SWITCH favors goal-side defender over attacker standing behind ball
        sim.state.ball.owner = Some((TeamSide::Home, 4));
        sim.state.ball.pos = Vec3::from_milli(4_000, 0, 0);
        // Away defends +X goal (`+28m`). Put Away[1] goal-side at +7.0m (`3.0m` away) and Away[4] behind ball at +1.5m (`2.5m` away).
        sim.state.players[1][1].pos = Vec2::from_milli(7_000, 0);
        sim.state.players[1][2].pos = Vec2::from_milli(16_000, 9_000);
        sim.state.players[1][3].pos = Vec2::from_milli(16_000, -9_000);
        sim.state.players[1][4].pos = Vec2::from_milli(1_500, 0);
        sim.state.controlled_idx[1] = 2;
        sim.state.prev_buttons = [0, 0];

        sim.tick(&[
            InputFrame::IDLE,
            InputFrame {
                buttons: BTN_SWITCH,
                ..InputFrame::IDLE
            },
        ]);
        assert_eq!(
            sim.state().controlled_idx[1],
            1,
            "neutral defensive switch should prioritize goal-side defender"
        );
    }

    #[test]
    fn input_buffering_free_kick_wall_and_live_substitutions_are_deterministic() {
        let mut snap = FixtureSnapshot::sample_five_a_side(2026_99);
        snap.rules.home_controller = SideController::Human;
        snap.rules.away_controller = SideController::Human;

        let mut sim_a = MatchSimulation::new(&snap).expect("valid 5v5 snapshot");
        let mut sim_b = MatchSimulation::new(&snap).expect("valid 5v5 snapshot");

        // 1. Test Free-Kick Defensive Wall setup in shooting range (`+16m` toward Away goal `+28m`)
        sim_a.setup_set_piece(RestartKind::FreeKick, TeamSide::Home, Vec2::from_milli(16_000, 1_500));
        sim_b.setup_set_piece(RestartKind::FreeKick, TeamSide::Home, Vec2::from_milli(16_000, 1_500));
        let wall_dists: Vec<i64> = sim_a.state().players[1]
            .iter()
            .skip(1)
            .map(|p| p.pos.dist(Vec2::from_milli(16_000, 1_500)).to_milli())
            .collect();
        let at_wall_dist = wall_dists.iter().filter(|&&d| (4_350..=4_650).contains(&d)).count();
        assert!(
            at_wall_dist >= 2,
            "5v5 direct free kick should form a 2-man defensive wall at 4.5m ({wall_dists:?})"
        );

        // 2. Test Deterministic Live Substitution via `InputFrame::encode_substitution`
        sim_a.state.players[0][4].stamina_permille = 420;
        sim_b.state.players[0][4].stamina_permille = 420;
        let bench_player_id = sim_a.state().teams[0].substitutes[4].footballer_id.clone();
        let sub_frame = InputFrame {
            reserved: InputFrame::encode_substitution(4, 4),
            ..InputFrame::IDLE
        };
        sim_a.tick(&[sub_frame, InputFrame::IDLE]);
        sim_b.tick(&[sub_frame, InputFrame::IDLE]);
        assert_eq!(sim_a.state().players[0][4].footballer_id, bench_player_id);
        assert_eq!(sim_a.state().players[0][4].stamina_permille, 1_000);
        assert_eq!(sim_a.state().subs_used[0], 1);
        assert_eq!(sim_a.state_hash(), sim_b.state_hash());

        // 3. Test Pre-Contact Input Buffering -> Crisp First-Time One-Touch Shot!
        sim_a.state.phase = MatchPhase::OpenPlay;
        sim_a.state.controlled_idx[0] = 4;
        sim_a.state.players[0][4].pos = Vec2::from_milli(12_000, 0);
        sim_a.state.players[0][4].vel = Vec2::ZERO;
        sim_a.state.ball.owner = None;
        sim_a.state.ball.last_touch_team = Some(TeamSide::Home);
        sim_a.state.ball.pos = Vec3::from_milli(10_800, 0, 0); // 1.2m away (outside 0.72m trap radius!)
        sim_a.state.ball.vel = Vec3::from_milli(14_000, 0, 0); // Rolling rapidly toward Home[4]

        // Press Shoot BEFORE ball reaches Home[4]!
        sim_a.tick(&[
            InputFrame {
                move_x: 127,
                aim_x: 127,
                buttons: BTN_SHOOT,
                ..InputFrame::IDLE
            },
            InputFrame::IDLE,
        ]);
        assert!(
            sim_a.state().input_buffer[0].is_some() || sim_a.state().player_stats[0][4].shots >= 1,
            "pressing Shoot right before ball arrives must buffer or fire a first-time shot"
        );
        // Step 4 more ticks with buttons released: ball contacts Home[4] and fires one-touch shot!
        for _ in 0..4 {
            sim_a.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
        }
        assert!(
            sim_a.state().player_stats[0][4].shots >= 1,
            "buffered pre-contact BTN_SHOOT must execute a crisp first-time shot upon contact!"
        );
    }

    #[test]
    fn in_flight_substitution_attribution_and_authentic_ball_physics_work() {
        let mut snap = FixtureSnapshot::sample_five_a_side(2026_77);
        snap.rules.home_controller = SideController::Human;
        snap.rules.away_controller = SideController::Human;
        let mut sim = MatchSimulation::new(&snap).expect("valid 5v5 snapshot");

        sim.state.phase = MatchPhase::OpenPlay;
        // Clear defenders out of the lane
        for p in sim.state.players[1].iter_mut() {
            p.pos = Vec2::from_milli(-18_000, -14_000);
        }

        // 1. Aerial chest trap cushioning: ball at z = 1.05m should NOT snap to z = 0 in 1 tick!
        sim.state.players[0][2].pos = Vec2::from_milli(0, 0);
        sim.state.players[0][2].vel = Vec2::ZERO;
        sim.state.ball.owner = None;
        sim.state.ball.pos = Vec3::from_milli(180, 0, 1_050);
        sim.state.ball.vel = Vec3::from_milli(2_000, 0, -1_200);
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
        assert_eq!(sim.state().ball.owner, Some((TeamSide::Home, 2)));
        assert!(
            sim.state().ball.pos.z > Fp::from_milli(700),
            "chest-high aerial trap must cushion downward under gravity instead of snapping to z=0 ({:?})",
            sim.state().ball.pos.z
        );

        // 2. Driven shot deflection off opponent defender (`21 m/s` > opponent trap limit -> ricochets free!)
        sim.state.ball.owner = None;
        sim.state.ball.last_touch_team = Some(TeamSide::Home);
        sim.state.players[1][2].pos = Vec2::from_milli(8_000, 0);
        sim.state.players[1][2].vel = Vec2::ZERO;
        sim.state.players[1][2].action = PlayerActionState::Normal;
        sim.state.ball.pos = Vec3::from_milli(7_650, 0, 150);
        sim.state.ball.vel = Vec3::from_milli(21_000, 0, 400);
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
        assert!(
            sim.state().ball.owner.is_none(),
            "21 m/s thunderbolt hitting defender must deflect off body rather than magnetically trapping"
        );
        assert!(sim.state().ball.vel.x < Fp::ZERO, "frontal block should ricochet backward");
        sim.state.players[1][2].pos = Vec2::from_milli(-18_000, -14_000);

        // 3. Codex PR #8 edge case: Passer (slot 3) passes to Striker (slot 4), Passer is subbed off mid-flight,
        // Striker traps & shoots toward open net, Striker is ALSO subbed off while shot is in flight -> Goal!
        // Both outgoing Passer and outgoing Striker MUST be credited in `combined_player_stats`!
        let passer_orig_id = sim.state().players[0][3].footballer_id.clone();
        let scorer_orig_id = sim.state().players[0][4].footballer_id.clone();
        let sub_passer_id = sim.state().teams[0].substitutes[2].footballer_id.clone();
        let sub_scorer_id = sim.state().teams[0].substitutes[3].footballer_id.clone();

        sim.state.players[0][3].pos = Vec2::from_milli(12_000, 0);
        sim.state.players[0][3].vel = Vec2::ZERO;
        sim.state.players[0][3].action = PlayerActionState::Normal;
        sim.state.players[0][4].pos = Vec2::from_milli(21_000, 0);
        sim.state.players[0][4].vel = Vec2::ZERO;
        sim.state.players[0][4].action = PlayerActionState::Normal;
        sim.state.controlled_idx[0] = 3;
        sim.state.ball.owner = Some((TeamSide::Home, 3));
        sim.state.ball.pos = Vec3::from_milli(12_350, 0, 0);
        sim.state.ball.vel = Vec3::ZERO;
        sim.state.prev_buttons = [0, 0];

        // Launch ground pass from slot 3 -> slot 4 (charge 4 ticks then release)
        for _ in 0..4 {
            sim.tick(&[
                InputFrame {
                    move_x: 127,
                    aim_x: 127,
                    buttons: BTN_PASS,
                    ..InputFrame::IDLE
                },
                InputFrame::IDLE,
            ]);
        }
        sim.tick(&[
            InputFrame {
                move_x: 127,
                aim_x: 127,
                buttons: 0,
                ..InputFrame::IDLE
            },
            InputFrame::IDLE,
        ]);
        assert!(sim.state().ball.pass_in_flight.is_some());

        // Substitute slot 3 OFF while pass is still in flight!
        assert!(sim.apply_substitution(TeamSide::Home, 3, 2));
        assert_eq!(sim.state().players[0][3].footballer_id, sub_passer_id);

        // Step until slot 4 traps the pass
        for _ in 0..90 {
            sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
            if sim.state().ball.owner == Some((TeamSide::Home, 4)) {
                break;
            }
        }
        assert_eq!(sim.state().ball.owner, Some((TeamSide::Home, 4)));

        // Slot 4 shoots at open Away goal (`+28m`) — charge 5 ticks then release!
        for _ in 0..5 {
            sim.tick(&[
                InputFrame {
                    move_x: 127,
                    aim_x: 127,
                    buttons: BTN_SHOOT,
                    ..InputFrame::IDLE
                },
                InputFrame::IDLE,
            ]);
        }
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
        assert!(sim.state().ball.owner.is_none());

        // Substitute slot 4 OFF while shot is flying toward the net!
        assert!(sim.apply_substitution(TeamSide::Home, 4, 3));
        assert_eq!(sim.state().players[0][4].footballer_id, sub_scorer_id);

        // Step until ball crosses goal line!
        for _ in 0..60 {
            sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
            if sim.state().score[0] >= 1 {
                break;
            }
        }
        assert_eq!(sim.state().score[0], 1, "expected in-flight shot to score");

        let combined = sim.state().combined_player_stats(TeamSide::Home);
        let orig_passer_st = combined
            .iter()
            .find(|s| s.footballer_id == passer_orig_id)
            .expect("archived passer stats present");
        let orig_scorer_st = combined
            .iter()
            .find(|s| s.footballer_id == scorer_orig_id)
            .expect("archived scorer stats present");
        let sub_scorer_st = combined
            .iter()
            .find(|s| s.footballer_id == sub_scorer_id)
            .expect("incoming sub scorer stats present");

        assert_eq!(orig_passer_st.passes_completed, 1, "outgoing passer must get completed pass");
        assert_eq!(orig_passer_st.assists, 1, "outgoing passer must get assist");
        assert_eq!(orig_scorer_st.goals, 1, "outgoing shooter must get goal");
        assert_eq!(sub_scorer_st.goals, 0, "incoming substitute must NOT steal in-flight goal");

        // 4. Deflected-shot edge case: Shooter (`slot 2`) shoots, is subbed off, Away GK touches/deflects
        // on the way into the net -> outgoing Shooter (`slot 2`) STILL gets the goal rather than incoming sub!
        sim.state.phase = MatchPhase::OpenPlay;
        let defl_shooter_id = sim.state().players[0][2].footballer_id.clone();
        sim.set_last_touch(TeamSide::Home, 2);
        sim.state.ball.owner = None;
        sim.state.ball.pos = Vec3::from_milli(27_600, 0, 200);
        sim.state.ball.vel = Vec3::from_milli(19_000, 0, 0);
        assert!(sim.apply_substitution(TeamSide::Home, 2, 1));
        // Simulate Away GK grazing the shot (`last_touch_meta` becomes Away, while `last_team_touch_meta[0]` stays `defl_shooter_id`)
        sim.set_last_touch(TeamSide::Away, 0);
        for _ in 0..10 {
            sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
            if sim.state().score[0] >= 2 {
                break;
            }
        }
        assert_eq!(sim.state().score[0], 2);
        let combined2 = sim.state().combined_player_stats(TeamSide::Home);
        let defl_shooter_st = combined2
            .iter()
            .find(|s| s.footballer_id == defl_shooter_id)
            .expect("archived deflected-shot shooter stats present");
        assert_eq!(
            defl_shooter_st.goals, 1,
            "outgoing shooter must still be credited when shot deflects off opponent GK into net"
        );
    }
}
