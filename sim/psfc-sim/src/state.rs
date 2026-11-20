//! Canonical deterministic simulation state and platform-independent 64-bit state hashing.

use crate::config::{MatchFormat, RuleSet, TeamSide};
use crate::events::{MatchEvent, PlayerMatchStats};
use crate::math::{Fp, Vec2, Vec3};
use crate::rng::DeterministicRng;
use crate::snapshot::{PlayerAttributes, PlayerRole, TeamSnapshot};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerActionState {
    Normal,
    Charging {
        button: u16,
        ticks: u16,
        finesse: bool,
        aim: Vec2,
    },
    Tackling {
        ticks_left: u8,
    },
    Sliding {
        ticks_left: u8,
        dir: Vec2,
    },
    Diving {
        ticks_left: u8,
        dir: Vec2,
    },
    Stumbled {
        ticks_left: u8,
    },
    Cooldown {
        ticks_left: u8,
    },
    SkillMove {
        kind: u8,
        ticks_left: u8,
        dir: Vec2,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerState {
    pub team: TeamSide,
    pub index: usize,
    pub footballer_id: String,
    pub name: String,
    pub shirt_number: u8,
    pub role: PlayerRole,
    pub attributes: PlayerAttributes,
    /// Base tactical anchor in `[-1000, 1000]` permille when attacking `+X`.
    pub base_anchor_permille: (i64, i64),
    pub pos: Vec2,
    pub vel: Vec2,
    pub z: Fp,
    pub vz: Fp,
    pub facing_bam: u16,
    /// `0..=1000` permille stamina.
    pub stamina_permille: i32,
    pub action: PlayerActionState,
    /// Triggered forward burst timer (`BTN_CALL_RUN`).
    pub run_boost_ticks: u16,
    pub yellow_cards: u8,
    pub sent_off: bool,
}

impl PlayerState {
    #[inline]
    pub const fn is_goalkeeper(&self) -> bool {
        matches!(self.role, PlayerRole::Goalkeeper)
    }

    #[inline]
    pub const fn can_control_ball(&self) -> bool {
        if self.sent_off {
            return false;
        }
        !matches!(
            self.action,
            PlayerActionState::Stumbled { .. } | PlayerActionState::Cooldown { .. }
        )
    }

    #[inline]
    pub fn charge_ratio_permille(&self) -> u16 {
        match self.action {
            PlayerActionState::Charging { ticks, .. } => ((ticks as u32 * 1000) / 28).min(1000) as u16,
            _ => 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PassInFlight {
    pub team: TeamSide,
    pub passer_idx: usize,
    #[serde(default)]
    pub passer_id: String,
    #[serde(default)]
    pub passer_name: String,
    pub intended_receiver_idx: usize,
    pub target_pos: Vec2,
    pub launched_tick: u32,
    /// Receiver `X` in attacking coordinates vs second-last defender at pass instant (for 11v11 offside).
    pub receiver_was_offside: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistCandidate {
    pub team: TeamSide,
    pub passer_idx: usize,
    #[serde(default)]
    pub passer_id: String,
    #[serde(default)]
    pub passer_name: String,
    pub receiver_idx: usize,
    #[serde(default)]
    pub receiver_id: String,
    pub completed_tick: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BallState {
    pub pos: Vec3,
    pub vel: Vec3,
    /// Horizontal Magnus curve spin (`rad/s` approx in Q32.32).
    pub spin_side: Fp,
    /// Topspin / backspin.
    pub spin_top: Fp,
    /// `(TeamSide, player_index)` when dribbling or held by GK.
    pub owner: Option<(TeamSide, usize)>,
    /// Non-zero while Goalkeeper holds a caught shot inside the box.
    pub gk_hold_ticks: u16,
    /// Countdown ticks between discrete stride touches while dribbling.
    #[serde(default)]
    pub dribble_touch_cd: u8,
    pub last_touch_team: Option<TeamSide>,
    pub last_touch_player: Option<(TeamSide, usize)>,
    /// Substitution-safe `(TeamSide, footballer_id, name)` of the last player who touched the ball.
    #[serde(default)]
    pub last_touch_meta: Option<(TeamSide, String, String)>,
    pub pass_in_flight: Option<PassInFlight>,
    pub assist_candidate: Option<AssistCandidate>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RestartKind {
    ThrowIn,
    CornerKick,
    GoalKick,
    FreeKick,
    PenaltyKick,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchPhase {
    KickOff {
        team: TeamSide,
        wait_ticks: u16,
    },
    OpenPlay,
    SetPiece {
        kind: RestartKind,
        team: TeamSide,
        spot: Vec2,
        taker_idx: usize,
        wait_ticks: u16,
    },
    GoalScored {
        scoring_team: TeamSide,
        scorer_idx: usize,
        celebration_ticks: u16,
    },
    HalfTime {
        wait_ticks: u16,
    },
    FullTime,
}

impl MatchPhase {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::KickOff { .. } => "kick_off",
            Self::OpenPlay => "open_play",
            Self::SetPiece {
                kind: RestartKind::ThrowIn,
                ..
            } => "throw_in",
            Self::SetPiece {
                kind: RestartKind::CornerKick,
                ..
            } => "corner_kick",
            Self::SetPiece {
                kind: RestartKind::GoalKick,
                ..
            } => "goal_kick",
            Self::SetPiece {
                kind: RestartKind::FreeKick,
                ..
            } => "free_kick",
            Self::SetPiece {
                kind: RestartKind::PenaltyKick,
                ..
            } => "penalty_kick",
            Self::GoalScored { .. } => "goal_scored",
            Self::HalfTime { .. } => "half_time",
            Self::FullTime => "full_time",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BufferedAction {
    pub button: u16,
    pub aim: Vec2,
    pub finesse: bool,
    pub ttl_ticks: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatchState {
    pub fixture_id: String,
    pub format: MatchFormat,
    pub rules: RuleSet,
    pub teams: [TeamSnapshot; 2],
    pub tick: u32,
    /// Match clock ticks elapsed across open play (`0 .. 2 * half_duration_ticks`).
    pub clock_ticks: u32,
    /// `1` or `2`.
    pub half: u8,
    pub score: [u16; 2],
    pub possession_ticks: [u32; 2],
    /// Smoothed ("laggy") team possession balance in `[-1000, +1000]` (`+1000` = Home, `-1000` = Away)
    /// preventing 1-tick offensive/defensive jitter on 50/50 loose balls.
    #[serde(default)]
    pub possession_balance_permille: i32,
    pub phase: MatchPhase,
    /// `[home_controlled_idx, away_controlled_idx]`.
    pub controlled_idx: [usize; 2],
    pub prev_buttons: [u16; 2],
    pub switch_cooldown: [u8; 2],
    /// Pre-contact `~233ms` (`14`-tick) input lookahead buffer per team for crisp one-touch play.
    #[serde(default)]
    pub input_buffer: [Option<BufferedAction>; 2],
    #[serde(default)]
    pub subs_used: [u8; 2],
    /// `[home_players, away_players]`.
    pub players: [Vec<PlayerState>; 2],
    pub player_stats: [Vec<PlayerMatchStats>; 2],
    #[serde(default)]
    pub archived_player_stats: [Vec<PlayerMatchStats>; 2],
    pub ball: BallState,
    pub rng: DeterministicRng,
    #[serde(default)]
    pub went_to_extra_time: bool,
    #[serde(default)]
    pub shootout_score: Option<[u8; 2]>,
    #[serde(skip)]
    pub pending_events: Vec<MatchEvent>,
    pub recorded_events: Vec<MatchEvent>,
}

impl MatchState {
    #[inline]
    pub const fn second_half_swapped(&self) -> bool {
        self.half.is_multiple_of(2)
    }

    /// Returns stats for all starters and substitutes who appeared for `side`.
    pub fn combined_player_stats(&self, side: TeamSide) -> Vec<PlayerMatchStats> {
        let s = side.index();
        let mut out = Vec::with_capacity(self.player_stats[s].len() + self.archived_player_stats[s].len());
        out.extend(self.archived_player_stats[s].iter().cloned());
        out.extend(self.player_stats[s].iter().cloned());
        out
    }

    /// Mutable lookup by `footballer_id` across active pitch slots and archived bench stats,
    /// ensuring goals, assists, and completed passes credit the exact athlete even after a live substitution.
    pub fn player_stats_mut_by_id(
        &mut self,
        side: TeamSide,
        footballer_id: &str,
    ) -> Option<&mut PlayerMatchStats> {
        let s = side.index();
        if let Some(st) = self.player_stats[s]
            .iter_mut()
            .find(|st| st.footballer_id == footballer_id)
        {
            return Some(st);
        }
        self.archived_player_stats[s]
            .iter_mut()
            .find(|st| st.footballer_id == footballer_id)
    }

    /// Approximate football match minute (`1..=90` in regulation, `91..=120` in extra time) scaled to configured match length.
    pub fn display_minute(&self) -> u16 {
        let reg_ticks = (self.rules.half_duration_ticks.saturating_mul(2)).max(1);
        if self.half <= 2 {
            let scaled = ((self.clock_ticks as u64) * 90) / (reg_ticks as u64);
            (scaled as u16 + 1).min(90)
        } else {
            let et_total = ((self.rules.half_duration_ticks / 3).max(180).saturating_mul(2)).max(1);
            let et_elapsed = self.clock_ticks.saturating_sub(reg_ticks);
            let scaled = ((et_elapsed as u64) * 30) / (et_total as u64);
            (91 + scaled as u16).min(120)
        }
    }

    /// Canonical 64-bit deterministic hash over simulation state.
    /// Uses explicit fixed-width little-endian integer encoding so 32-bit `wasm32`
    /// and 64-bit `x86_64` / `aarch64` produce bit-for-bit identical hashes.
    pub fn compute_hash(&self) -> u64 {
        let mut h = CanonicalHasher::new();
        h.u32(self.tick);
        h.u32(self.clock_ticks);
        h.u8(self.half);
        h.u16(self.score[0]);
        h.u16(self.score[1]);
        h.u8(u8::from(self.went_to_extra_time));
        match self.shootout_score {
            Some([hp, ap]) => {
                h.u8(1);
                h.u8(hp);
                h.u8(ap);
            }
            None => h.u8(0),
        }
        h.u32(self.possession_ticks[0]);
        h.u32(self.possession_ticks[1]);
        h.i32(self.possession_balance_permille);
        h.u32(self.controlled_idx[0] as u32);
        h.u32(self.controlled_idx[1] as u32);
        h.u16(self.prev_buttons[0]);
        h.u16(self.prev_buttons[1]);
        h.u8(self.switch_cooldown[0]);
        h.u8(self.switch_cooldown[1]);
        h.u8(self.subs_used[0]);
        h.u8(self.subs_used[1]);
        for slot in &self.input_buffer {
            match slot {
                Some(buf) => {
                    h.u8(1);
                    h.u16(buf.button);
                    h.i64(buf.aim.x.raw());
                    h.i64(buf.aim.y.raw());
                    h.u8(u8::from(buf.finesse));
                    h.u8(buf.ttl_ticks);
                }
                None => h.u8(0),
            }
        }

        for w in self.rng.state_words() {
            h.u64(w);
        }

        // Phase
        match self.phase {
            MatchPhase::KickOff { team, wait_ticks } => {
                h.u8(1);
                h.u8(team.index() as u8);
                h.u16(wait_ticks);
            }
            MatchPhase::OpenPlay => h.u8(2),
            MatchPhase::SetPiece {
                kind,
                team,
                spot,
                taker_idx,
                wait_ticks,
            } => {
                h.u8(3);
                h.u8(kind as u8);
                h.u8(team.index() as u8);
                h.i64(spot.x.raw());
                h.i64(spot.y.raw());
                h.u32(taker_idx as u32);
                h.u16(wait_ticks);
            }
            MatchPhase::GoalScored {
                scoring_team,
                scorer_idx,
                celebration_ticks,
            } => {
                h.u8(4);
                h.u8(scoring_team.index() as u8);
                h.u32(scorer_idx as u32);
                h.u16(celebration_ticks);
            }
            MatchPhase::HalfTime { wait_ticks } => {
                h.u8(5);
                h.u16(wait_ticks);
            }
            MatchPhase::FullTime => h.u8(6),
        }

        // Ball
        h.i64(self.ball.pos.x.raw());
        h.i64(self.ball.pos.y.raw());
        h.i64(self.ball.pos.z.raw());
        h.i64(self.ball.vel.x.raw());
        h.i64(self.ball.vel.y.raw());
        h.i64(self.ball.vel.z.raw());
        h.i64(self.ball.spin_side.raw());
        h.i64(self.ball.spin_top.raw());
        match self.ball.owner {
            Some((side, idx)) => {
                h.u8(1);
                h.u8(side.index() as u8);
                h.u32(idx as u32);
            }
            None => h.u8(0),
        }
        h.u16(self.ball.gk_hold_ticks);
        h.u8(self.ball.dribble_touch_cd);

        // Players
        for side_players in &self.players {
            h.u32(side_players.len() as u32);
            for p in side_players {
                h.u32(p.footballer_id.len() as u32);
                h.bytes(p.footballer_id.as_bytes());
                h.u8(p.shirt_number);
                h.i64(p.pos.x.raw());
                h.i64(p.pos.y.raw());
                h.i64(p.vel.x.raw());
                h.i64(p.vel.y.raw());
                h.i64(p.z.raw());
                h.i64(p.vz.raw());
                h.u16(p.facing_bam);
                h.i32(p.stamina_permille);
                h.u16(p.run_boost_ticks);
                h.u8(p.yellow_cards);
                h.u8(u8::from(p.sent_off));
                match p.action {
                    PlayerActionState::Normal => h.u8(0),
                    PlayerActionState::Charging {
                        button,
                        ticks,
                        finesse,
                        aim,
                    } => {
                        h.u8(1);
                        h.u16(button);
                        h.u16(ticks);
                        h.u8(u8::from(finesse));
                        h.i64(aim.x.raw());
                        h.i64(aim.y.raw());
                    }
                    PlayerActionState::Tackling { ticks_left } => {
                        h.u8(2);
                        h.u8(ticks_left);
                    }
                    PlayerActionState::Sliding { ticks_left, dir } => {
                        h.u8(3);
                        h.u8(ticks_left);
                        h.i64(dir.x.raw());
                        h.i64(dir.y.raw());
                    }
                    PlayerActionState::Diving { ticks_left, dir } => {
                        h.u8(4);
                        h.u8(ticks_left);
                        h.i64(dir.x.raw());
                        h.i64(dir.y.raw());
                    }
                    PlayerActionState::Stumbled { ticks_left } => {
                        h.u8(5);
                        h.u8(ticks_left);
                    }
                    PlayerActionState::Cooldown { ticks_left } => {
                        h.u8(6);
                        h.u8(ticks_left);
                    }
                    PlayerActionState::SkillMove {
                        kind,
                        ticks_left,
                        dir,
                    } => {
                        h.u8(7);
                        h.u8(kind);
                        h.u8(ticks_left);
                        h.i64(dir.x.raw());
                        h.i64(dir.y.raw());
                    }
                }
            }
        }

        h.finish()
    }
}

struct CanonicalHasher(u64);

impl CanonicalHasher {
    const OFFSET: u64 = 0xCBF2_9CE4_8422_2325;
    const PRIME: u64 = 0x0100_0000_01B3;

    #[inline]
    const fn new() -> Self {
        Self(Self::OFFSET)
    }

    #[inline]
    fn bytes(&mut self, data: &[u8]) {
        for &b in data {
            self.0 ^= b as u64;
            self.0 = self.0.wrapping_mul(Self::PRIME);
        }
    }

    #[inline]
    fn u8(&mut self, v: u8) {
        self.bytes(&[v]);
    }

    #[inline]
    fn u16(&mut self, v: u16) {
        self.bytes(&v.to_le_bytes());
    }

    #[inline]
    fn u32(&mut self, v: u32) {
        self.bytes(&v.to_le_bytes());
    }

    #[inline]
    fn i32(&mut self, v: i32) {
        self.bytes(&v.to_le_bytes());
    }

    #[inline]
    fn u64(&mut self, v: u64) {
        self.bytes(&v.to_le_bytes());
    }

    #[inline]
    fn i64(&mut self, v: i64) {
        self.bytes(&v.to_le_bytes());
    }

    #[inline]
    const fn finish(self) -> u64 {
        // SplitMix64 avalanche finalizer
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}
