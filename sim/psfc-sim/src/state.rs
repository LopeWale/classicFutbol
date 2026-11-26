//! Canonical deterministic simulation state and platform-independent 64-bit state hashing.

use crate::config::{MatchFormat, RuleSet, TeamSide};
use crate::events::{MatchEvent, PlayerMatchStats};
use crate::input::ControlSettings;
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

const fn default_contain_stamina() -> u8 {
    100
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
    /// Post-strike visual cue timer (`0..=14`) so kick leg-swing plays on exact contact even for 1-tick taps.
    #[serde(default)]
    pub recent_kick_ticks: u8,
    /// Visual kick category (`1=Pass, 2=Through, 3=Lob, 4=Shot, 5=Chip`) during `recent_kick_ticks`.
    #[serde(default)]
    pub recent_kick_kind: u8,
    /// FIFA 22 Teammate Contain (`RB / R1`) stamina pool (`0..=100`).
    #[serde(default = "default_contain_stamina")]
    pub contain_stamina: u8,
    /// FIFA 22 Teammate Contain cooldown ticks (`0..=150`) after contain stamina depletes.
    #[serde(default)]
    pub contain_cooldown: u8,
    /// FIFA 21 chained skill-move counter (`> 3` chained moves within decay window triggers heavy-touch error).
    #[serde(default)]
    pub skill_chain_count: u8,
    /// Decay timer (`0..=180` ticks) resetting `skill_chain_count` once dribbler settles.
    #[serde(default)]
    pub skill_chain_decay_ticks: u8,
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
            PlayerActionState::Charging { ticks, .. } => ((ticks as u32 * 1000) / 22).min(1000) as u16,
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

/// Tracks an untouched restart delivery to enforce IFAB double-touch & direct-goal restrictions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestartTouchState {
    /// `None` represents a KickOff; `Some(kind)` represents the corresponding `SetPiece`.
    pub kind: Option<RestartKind>,
    pub taker_side: TeamSide,
    pub taker_idx: usize,
    pub indirect: bool,
}

/// Active IFAB Law 12 Referee Advantage window (`~2.5s`), calling play back only if advantage fails.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingAdvantage {
    pub victim_side: TeamSide,
    pub fouling_side: TeamSide,
    pub fouler_idx: usize,
    pub fouler_id: String,
    pub fouler_name: String,
    pub foul_pos: Vec2,
    pub start_ball_x: Fp,
    pub ticks_left: u8,
    pub pending_yellow: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BallState {
    pub pos: Vec3,
    #[serde(default)]
    pub prev_pos: Vec3,
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
    /// Per-team substitution-safe `(footballer_id, name)` so shots deflected off an opponent/GK still credit an outgoing shooter.
    #[serde(default)]
    pub last_team_touch_meta: [Option<(String, String)>; 2],
    /// IFAB Law 11 snapshot of all offside teammates (`1 << idx`) at the moment `offside_team` last played the ball.
    #[serde(default)]
    pub offside_team: Option<TeamSide>,
    #[serde(default)]
    pub offside_mask: u16,
    /// IFAB Law 12 back-pass tracking: `Some(side)` when `side` deliberately kicked or threw-in the ball (forbids own GK hand catch).
    #[serde(default)]
    pub deliberate_foot_backpass: Option<TeamSide>,
    /// Shooter credited with a tentative on-target shot so outfield blocks or woodwork misses roll it back accurately.
    #[serde(default)]
    pub pending_shot_on_target: Option<(TeamSide, String)>,
    /// Post-kick re-touch lockout `(TeamSide, player_index, ticks_left)` preventing self-pass traps and Goal-Kick double-touch fouls until another player or woodwork touches the ball.
    #[serde(default)]
    pub kicker_lockout: Option<(TeamSide, usize, u8)>,
    /// EA FC 26 anti-tackleback grace window (`0..=12` ticks) protecting a newly won tackle from immediate passive re-dispossession.
    #[serde(default)]
    pub possession_grace_ticks: u8,
    /// Exact 60 Hz tick when the active shot was struck (for EA SEED human-like GK reaction latency).
    #[serde(default)]
    pub shot_launched_tick: u32,
    /// Pitch coordinates `(x, y)` where the active shot was struck (for FIFA 23 GK Line-of-Sight screening checks).
    #[serde(default)]
    pub shot_origin: Vec2,
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

/// Context-sensitive gameplay state for a team's input state machine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PossessionContext {
    /// Team currently owns/carries the ball (attacking controls active).
    Attacking,
    /// Opponent currently owns/carries the ball (defending controls active).
    Defending,
    /// Ball is free/unowned in play (contested loose ball, aerial duel, or pass in flight).
    LooseBall,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BufferedAction {
    pub button: u16,
    pub aim: Vec2,
    pub finesse: bool,
    /// EA FC 26 Low Driven modifier (`double-tap BTN_SHOOT` before foot contact).
    #[serde(default)]
    pub low_driven: bool,
    /// Accumulated charge ticks (`1..=22`) recorded while holding the kick button pre-contact.
    #[serde(default)]
    pub charge_ticks: u16,
    /// True while the player is still holding the kick button down pre-contact.
    #[serde(default)]
    pub charging: bool,
    pub ttl_ticks: u8,
}

impl BufferedAction {
    #[inline]
    pub fn charge_ratio_permille(&self) -> u16 {
        ((self.charge_ticks as u32 * 1000) / 22).min(1000) as u16
    }
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
    /// Grace window ticks (`~1.2s`) preserving a manual `BTN_SWITCH` selection before auto-switching resumes.
    #[serde(default)]
    pub manual_switch_lock: [u8; 2],
    /// Per-side FIFA control assistance & defending mode settings (deterministically synced via `InputFrame.reserved`).
    #[serde(default)]
    pub control_settings: [ControlSettings; 2],
    /// True once a side emits dual-map (`ControlProfile`) inputs (`0x4000` settings sync or `0x0800` flag),
    /// disabling legacy single-map `BTN_JOCKEY` attacking modifier aliases so remapped defensive Jockey never leaks into Attack.
    #[serde(default)]
    pub dual_map_controls: [bool; 2],
    /// Remaining Auto-Switching Move Assistance ticks carrying prior run direction after a player switch.
    #[serde(default)]
    pub switch_move_assist_ticks: [u8; 2],
    /// Normalized previous movement direction preserved during `switch_move_assist_ticks`.
    #[serde(default)]
    pub switch_move_assist_dir: [Vec2; 2],
    /// Rising-edge latch for Right Stick directional player switching & knock-on flicks.
    #[serde(default)]
    pub prev_rstick_active: [bool; 2],
    /// Active AI teammate currently assigned to `BTN_TEAMMATE_CONTAIN` (Second-Man Press) per side.
    #[serde(default)]
    pub teammate_contain_idx: [Option<usize>; 2],
    /// Remaining ticks of FIFA 12 Push-Pull shoulder-jostle momentum slowdown per team's ball carrier.
    #[serde(default)]
    pub push_pull_slow_ticks: [u8; 2],
    /// Pre-contact `~250ms` (`15`-tick) input lookahead & power-bar buffer per team for crisp one-touch play.
    #[serde(default)]
    pub input_buffer: [Option<BufferedAction>; 2],
    /// Desired pass/shot charge ticks computed from exact target distance when CPU initiates `Charging`.
    #[serde(default)]
    pub cpu_target_charge: [u8; 2],
    /// Untouched set-piece / kickoff state for IFAB double-touch & direct-goal enforcement.
    #[serde(default)]
    pub restart_touch_state: Option<RestartTouchState>,
    /// Active non-teleporting IFAB Law 12 Referee Advantage window.
    #[serde(default)]
    pub pending_advantage: Option<PendingAdvantage>,
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

    /// Authoritative context-sensitive possession state for `side`.
    #[inline]
    pub fn possession_context(&self, side: TeamSide) -> PossessionContext {
        match self.ball.owner {
            Some((owner_side, _)) if owner_side == side => PossessionContext::Attacking,
            Some(_) => PossessionContext::Defending,
            None => PossessionContext::LooseBall,
        }
    }

    /// Earliest reachable ball-intercept tick across active outfielders (`1..len`) for `side`.
    pub fn fastest_outfield_intercept_ticks(&self, side: TeamSide) -> u32 {
        let is_5v5 = self.format.players_per_side <= 6;
        let mut best = 120_u32;
        if self.ball.owner.is_some() {
            let pt = crate::ai::predict_ball_xy_at_ticks(self, 8);
            for p in self.players[side.index()].iter().skip(1) {
                if !p.sent_off {
                    let (_, t) = crate::ai::best_ball_intercept_for_player(self, side, p.index);
                    let _ = pt;
                    if t < best {
                        best = t;
                    }
                }
            }
            return best;
        }
        let waypoints = crate::ai::sample_ball_3d_waypoints(self);
        for p in self.players[side.index()].iter().skip(1) {
            if !p.sent_off {
                let (_, t) = crate::ai::best_intercept_from_waypoints(p, &waypoints, is_5v5, false);
                if t < best {
                    best = t;
                }
            }
        }
        best
    }

    /// True when `side` is in the Attacking control context:
    /// taking a KickOff/SetPiece, owning the ball, holding an active pre-contact `input_buffer`,
    /// receiving its own pass in flight, or clearly beating the opponent to its own loose touch/knock-on.
    /// Crucially returns `false` on contested rebounds, deflections, or defender toe-pokes where an
    /// opponent is as close or faster to the ball so defensive Switch/Tackle/Contain/GK-Rush never lock out!
    pub fn is_attacking_context(&self, side: TeamSide) -> bool {
        match self.phase {
            MatchPhase::KickOff { team, .. } | MatchPhase::SetPiece { team, .. } => {
                return team == side;
            }
            _ => {}
        }
        match self.ball.owner {
            Some((owner_side, _)) => owner_side == side,
            None => {
                if self.input_buffer[side.index()].is_some() {
                    return true;
                }
                let own_pass_in_flight = self
                    .ball
                    .pass_in_flight
                    .as_ref()
                    .is_some_and(|pf| pf.team == side);
                if !own_pass_in_flight && self.ball.last_touch_team != Some(side) {
                    return false;
                }
                let us_ticks = self.fastest_outfield_intercept_ticks(side);
                let opp_ticks = self.fastest_outfield_intercept_ticks(side.opponent());
                if own_pass_in_flight {
                    return us_ticks <= opp_ticks + 2;
                }
                us_ticks + 2 <= opp_ticks
            }
        }
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
        h.u8(self.manual_switch_lock[0]);
        h.u8(self.manual_switch_lock[1]);
        h.u16(self.control_settings[0].encode_reserved());
        h.u16(self.control_settings[1].encode_reserved());
        h.u8(u8::from(self.dual_map_controls[0]));
        h.u8(u8::from(self.dual_map_controls[1]));
        h.u8(self.switch_move_assist_ticks[0]);
        h.u8(self.switch_move_assist_ticks[1]);
        h.i64(self.switch_move_assist_dir[0].x.raw());
        h.i64(self.switch_move_assist_dir[0].y.raw());
        h.i64(self.switch_move_assist_dir[1].x.raw());
        h.i64(self.switch_move_assist_dir[1].y.raw());
        h.u8(u8::from(self.prev_rstick_active[0]));
        h.u8(u8::from(self.prev_rstick_active[1]));
        for tc in &self.teammate_contain_idx {
            match tc {
                Some(idx) => {
                    h.u8(1);
                    h.u32(*idx as u32);
                }
                None => h.u8(0),
            }
        }
        h.u8(self.push_pull_slow_ticks[0]);
        h.u8(self.push_pull_slow_ticks[1]);
        h.u8(self.cpu_target_charge[0]);
        h.u8(self.cpu_target_charge[1]);
        h.u8(self.subs_used[0]);
        h.u8(self.subs_used[1]);
        match self.restart_touch_state {
            Some(rs) => {
                h.u8(1);
                match rs.kind {
                    Some(k) => {
                        h.u8(1);
                        h.u8(k as u8);
                    }
                    None => h.u8(0),
                }
                h.u8(rs.taker_side.index() as u8);
                h.u32(rs.taker_idx as u32);
                h.u8(u8::from(rs.indirect));
            }
            None => h.u8(0),
        }
        match &self.pending_advantage {
            Some(pa) => {
                h.u8(1);
                h.u8(pa.victim_side.index() as u8);
                h.u8(pa.fouling_side.index() as u8);
                h.u32(pa.fouler_idx as u32);
                h.i64(pa.foul_pos.x.raw());
                h.i64(pa.foul_pos.y.raw());
                h.i64(pa.start_ball_x.raw());
                h.u8(pa.ticks_left);
                h.u8(u8::from(pa.pending_yellow));
            }
            None => h.u8(0),
        }
        for slot in &self.input_buffer {
            match slot {
                Some(buf) => {
                    h.u8(1);
                    h.u16(buf.button);
                    h.i64(buf.aim.x.raw());
                    h.i64(buf.aim.y.raw());
                    h.u8(u8::from(buf.finesse));
                    h.u8(u8::from(buf.low_driven));
                    h.u16(buf.charge_ticks);
                    h.u8(u8::from(buf.charging));
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
        h.i64(self.ball.prev_pos.x.raw());
        h.i64(self.ball.prev_pos.y.raw());
        h.i64(self.ball.prev_pos.z.raw());
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
        match self.ball.kicker_lockout {
            Some((ts, idx, left)) => {
                h.u8(1);
                h.u8(ts.index() as u8);
                h.u32(idx as u32);
                h.u8(left);
            }
            None => h.u8(0),
        }
        h.u8(self.ball.possession_grace_ticks);
        h.u32(self.ball.shot_launched_tick);
        h.i64(self.ball.shot_origin.x.raw());
        h.i64(self.ball.shot_origin.y.raw());
        match self.ball.last_touch_team {
            Some(ts) => {
                h.u8(1);
                h.u8(ts.index() as u8);
            }
            None => h.u8(0),
        }
        match self.ball.last_touch_player {
            Some((ts, idx)) => {
                h.u8(1);
                h.u8(ts.index() as u8);
                h.u32(idx as u32);
            }
            None => h.u8(0),
        }
        match self.ball.offside_team {
            Some(ts) => {
                h.u8(1);
                h.u8(ts.index() as u8);
                h.u16(self.ball.offside_mask);
            }
            None => h.u8(0),
        }
        match self.ball.deliberate_foot_backpass {
            Some(ts) => {
                h.u8(1);
                h.u8(ts.index() as u8);
            }
            None => h.u8(0),
        }
        match &self.ball.last_touch_meta {
            Some((ts, fid, _)) => {
                h.u8(1);
                h.u8(ts.index() as u8);
                h.u32(fid.len() as u32);
                h.bytes(fid.as_bytes());
            }
            None => h.u8(0),
        }
        for team_meta in &self.ball.last_team_touch_meta {
            match team_meta {
                Some((fid, _)) => {
                    h.u8(1);
                    h.u32(fid.len() as u32);
                    h.bytes(fid.as_bytes());
                }
                None => h.u8(0),
            }
        }
        match &self.ball.pass_in_flight {
            Some(pf) => {
                h.u8(1);
                h.u8(pf.team.index() as u8);
                h.u32(pf.passer_idx as u32);
                h.u32(pf.passer_id.len() as u32);
                h.bytes(pf.passer_id.as_bytes());
                h.u32(pf.intended_receiver_idx as u32);
                h.i64(pf.target_pos.x.raw());
                h.i64(pf.target_pos.y.raw());
                h.u32(pf.launched_tick);
                h.u8(u8::from(pf.receiver_was_offside));
            }
            None => h.u8(0),
        }
        match &self.ball.assist_candidate {
            Some(ac) => {
                h.u8(1);
                h.u8(ac.team.index() as u8);
                h.u32(ac.passer_id.len() as u32);
                h.bytes(ac.passer_id.as_bytes());
                h.u32(ac.receiver_id.len() as u32);
                h.bytes(ac.receiver_id.as_bytes());
                h.u32(ac.completed_tick);
            }
            None => h.u8(0),
        }

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
                h.u8(p.recent_kick_ticks);
                h.u8(p.recent_kick_kind);
                h.u8(p.contain_stamina);
                h.u8(p.contain_cooldown);
                h.u8(p.skill_chain_count);
                h.u8(p.skill_chain_decay_ticks);
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
