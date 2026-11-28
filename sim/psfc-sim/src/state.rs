//! Canonical deterministic simulation state and platform-independent 64-bit state hashing.

use crate::config::{MatchFormat, RuleSet, TeamSide};
use crate::events::{MatchEvent, PlayerMatchStats};
use crate::input::{charge_permille_for, ControlSettings};
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
    /// Aerial landing recovery (`0..=42` ticks, `0.7s`): no re-jump while counting down (anti bunny-hop).
    #[serde(default)]
    pub jump_recover_ticks: u8,
    /// Consecutive ticks this player's body has overlapped another (push-out stuck-pair detector;
    /// above `avoidance::STUCK_TICKS` the pair gets a lateral nudge).
    #[serde(default)]
    pub overlap_ticks: u8,
    pub yellow_cards: u8,
    pub sent_off: bool,
    /// Steering diagnostics for `psfc-replay analyze` (side-flip cause split). DIAGNOSTICS ONLY:
    /// never serialized, never hashed, never read by the simulation, ignored by `PartialEq`.
    #[serde(skip)]
    pub steer_debug: SteerDebug,
}

/// Per-tick steering pipeline values of one player (diagnostics only, see
/// [`PlayerState::steer_debug`]): the desired velocity before local avoidance, the velocity the
/// avoidance stage selected, and the command after the steering filter, all on tick `tick`.
#[derive(Clone, Copy, Debug, Default)]
pub struct SteerDebug {
    /// Tick these values were written on (`0` = never); stale values must be ignored.
    pub tick: u32,
    pub desired: Vec2,
    pub selected: Vec2,
    pub filtered: Vec2,
}

impl PartialEq for SteerDebug {
    /// Diagnostics never take part in state equality.
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl Eq for SteerDebug {}

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
            PlayerActionState::Charging { button, ticks, .. } => charge_permille_for(button, ticks),
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

/// Goalkeeper decision scenario (see `docs/goalkeeper-ai.md` for the full entry / exit table).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GkScenario {
    /// Hold the angle-bisector arc (depth scaled by ball distance and `positioning`), feet set.
    #[default]
    Set,
    /// Incoming shot: reflex-latency gated save execution (lateral shuffle / dive).
    Shot,
    /// Committed claim of a high ball the keeper reaches first inside his command zone.
    CrossClaim,
    /// High ball the keeper cannot safely claim: hold a near-post / central stance and react.
    CrossHold,
    /// Committed sweep of a loose ball behind the defensive line.
    Sweep,
    /// Attacker through on goal with no covering defender: narrow the angle and stand big.
    OneVOne,
    /// Committed collection of a loose ball inside the box.
    Collect,
    /// Keeper holds / controls the ball: step to an outlet angle (distribution is in `sim.rs`).
    Possession,
    /// Own outfielder has the ball: central outlet stance at the goal-area edge.
    Support,
    /// Out of position: sprint back to the set position by the straight line.
    Recover,
    /// Live set-piece delivery (corner / penalty) or a dead-ball stance.
    SetPiece,
    /// Human-held `BTN_GK_RUSH`.
    HumanRush,
}

impl GkScenario {
    /// Stable wire / hash code.
    pub const fn code(self) -> u8 {
        match self {
            Self::Set => 0,
            Self::Shot => 1,
            Self::CrossClaim => 2,
            Self::CrossHold => 3,
            Self::Sweep => 4,
            Self::OneVOne => 5,
            Self::Collect => 6,
            Self::Possession => 7,
            Self::Support => 8,
            Self::Recover => 9,
            Self::SetPiece => 10,
            Self::HumanRush => 11,
        }
    }

    /// Scenarios that take the keeper off his set position toward the ball.
    pub const fn is_outward(self) -> bool {
        matches!(
            self,
            Self::CrossClaim | Self::Sweep | Self::OneVOne | Self::Collect | Self::HumanRush
        )
    }

    /// Scenarios that hold a commitment window once entered.
    pub const fn is_committed(self) -> bool {
        matches!(self, Self::CrossClaim | Self::Sweep | Self::OneVOne | Self::Collect)
    }
}

/// Small deterministic per-side goalkeeper brain (scenario state machine memory).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoalkeeperBrain {
    pub scenario: GkScenario,
    /// Remaining ticks the current committed scenario is held unless invalidated.
    pub commit_ticks: u16,
    /// Tick the current scenario was entered.
    pub entered_tick: u32,
    /// Tick of the last decision.
    pub last_decision_tick: u32,
    /// Committed target (claim / sweep / collect point) for the current scenario.
    pub target: Vec2,
    /// After an aborted outward commitment, outward re-entry needs a larger margin for this many ticks.
    pub reentry_block_ticks: u16,
    /// Diagnostic: total scenario switches this match.
    pub switches: u32,
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
    /// Accumulated charge ticks (`1..=max_charge_ticks_for(button)`) recorded while holding the kick button pre-contact.
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
        charge_permille_for(self.button, self.charge_ticks)
    }
}

/// Team-shape phase driving which formation shape the AI holds (see `docs/team-shape-ai.md`).
pub const AI_PHASE_ATTACK: u8 = 0;
pub const AI_PHASE_DEFEND: u8 = 1;
pub const AI_PHASE_COUNTER_PRESS: u8 = 2;
pub const AI_PHASE_RETREAT: u8 = 3;
pub const AI_PHASE_COUNTER_ATTACK: u8 = 4;
/// Sentinel for "no player / no opponent" in [`TeamAiMemory`] index fields.
pub const AI_NONE: u8 = u8::MAX;

/// Per-team outfield AI commitments carried tick to tick so assignments have hysteresis instead of
/// being re-derived (and flipping) every tick. Serialized and hashed: it changes the simulation.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamAiMemory {
    /// `tick + 1` of the last team plan (`0` = never planned).
    pub plan_tick: u32,
    pub phase: u8,
    pub phase_ticks: u16,
    /// Ball position when the current transition phase began.
    pub phase_origin: Vec2,
    /// Reaction-delayed ball reference the shape follows (low-passed by difficulty & positioning).
    pub ref_ball: Vec2,
    pub presser: u8,
    pub presser_ticks: u16,
    pub second_presser: u8,
    pub cover: u8,
    pub cover_ticks: u16,
    /// Formation slot index per player index (`AI_NONE` for the goalkeeper / unassigned).
    pub slot_of: Vec<u8>,
    pub slot_ticks: u16,
    /// Opponent index each player is marking (`AI_NONE` = zonal), and how long it has been held.
    pub mark_of: Vec<u8>,
    pub mark_ticks: Vec<u16>,
    /// Committed (low-passed) movement target per player and the tick it was last refreshed.
    pub target: Vec<Vec2>,
    pub target_tick: Vec<u32>,
    /// Bit `0` = sprinting, bit `1` = jockeying (decision hysteresis).
    pub flags: Vec<u8>,
    /// Low-passed velocity command per player (movement-model jerk filter) and its last tick.
    #[serde(default)]
    pub cmd_vel: Vec<Vec2>,
    #[serde(default)]
    pub cmd_tick: Vec<u32>,
    /// Rate state of the second-order steering filter (`cmd_vel` holds its direction), per player.
    #[serde(default)]
    pub steer_cmd_rate: Vec<Vec2>,
    /// Chaser intercept anti-dither: accepted intercept point per player, the tick it was accepted
    /// and whether it came from the attack-the-ball (near) predictor.
    #[serde(default)]
    pub steer_chase_pt: Vec<Vec2>,
    #[serde(default)]
    pub steer_chase_tick: Vec<u32>,
    #[serde(default)]
    pub steer_chase_near: Vec<u8>,
    /// CPU carrier evade-side hysteresis: side (`1` / `-1`, `0` none), carrier index, last tick
    /// used and the tick the side was chosen.
    #[serde(default)]
    pub steer_evade_sign: i8,
    #[serde(default)]
    pub steer_evade_idx: u8,
    #[serde(default)]
    pub steer_evade_tick: u32,
    #[serde(default)]
    pub steer_evade_since: u32,
    /// Low-passed speed of the steering command, per player.
    #[serde(default)]
    pub steer_cmd_speed: Vec<Fp>,
    /// Exclusivity layer (`docs/exclusivity-layer.md`): the one outfielder elected to chase a loose
    /// ball. `0` = none (the keeper index is never an elected outfield chaser).
    #[serde(default)]
    pub chaser: u8,
    /// Ticks the current chaser has held the role.
    #[serde(default)]
    pub chaser_ticks: u16,
    /// `tick + 1` of the last challenger evaluation (challengers are checked every few ticks).
    #[serde(default)]
    pub chaser_eval_tick: u32,
    /// The chaser's continuous intercept point (his reserved path end for the teammate yield rule).
    #[serde(default)]
    pub chaser_pt: Vec2,
    /// The keeper is committed to this loose ball (collect / claim / sweep / rush): no outfield chaser.
    #[serde(default)]
    pub keeper_claim: bool,
    /// Movement priority class per player for this tick (`0` carrier/chaser .. `3` idle/cruise/yield).
    #[serde(default)]
    pub move_class: Vec<u8>,

    // --- Attacking support and runs (round 4 work area 3, `docs/attacking-support-and-runs.md`) ---
    /// Committed support candidate kind per player (`AI_NONE` = none; see `ai::SUP_*`).
    #[serde(default)]
    pub sup_kind: Vec<u8>,
    /// `tick + 1` when the current support candidate was adopted (dwell / incumbent bonus).
    #[serde(default)]
    pub sup_since: Vec<u32>,
    /// `tick + 1` of the player's last support re-score (15-30 tick cadence).
    #[serde(default)]
    pub sup_eval: Vec<u32>,
    /// The committed support point (absolute; refreshed each tick from the candidate's geometry).
    #[serde(default)]
    pub sup_pt: Vec<Vec2>,
    /// Carrier index the support choice was made for (`AI_NONE` = none): a new carrier re-scores.
    #[serde(default)]
    pub sup_carrier: Vec<u8>,
    /// Sticky lateral nudge of the final target to reopen its lane (units of 2 m, signed).
    #[serde(default)]
    pub sup_nudge: Vec<i8>,
    /// Sticky side sign of the teammate yield (`yield_target`) for the current episode (0 = none).
    #[serde(default)]
    pub yield_sign: Vec<i8>,
    /// Sticky side sign of the keeper-corridor yield for the current episode (0 = none).
    #[serde(default)]
    pub gk_yield_sign: Vec<i8>,
    /// Sticky bend sign of the curved run around a blocker, and the tick it was last applied.
    #[serde(default)]
    pub bend_sign: Vec<i8>,
    #[serde(default)]
    pub bend_tick: Vec<u32>,
    /// Teammate this player yielded his destination to last tick (`AI_NONE` = none).
    #[serde(default)]
    pub res_yield: Vec<u8>,
    /// Alpha-beta target filter: estimated target velocity (m per tick) per player.
    #[serde(default)]
    pub tgt_vel: Vec<Vec2>,
    /// Last raw target accepted through the 0.4 m re-target deadband.
    #[serde(default)]
    pub tgt_raw: Vec<Vec2>,
    /// Committed run role per player (`0` none; see `ai::RUN_*`), when it began (`tick + 1`) and
    /// its end-point beyond the offside line.
    #[serde(default)]
    pub run_role: Vec<u8>,
    #[serde(default)]
    pub run_since: Vec<u32>,
    #[serde(default)]
    pub run_end: Vec<Vec2>,
    /// Support-triangle incumbent (`0` = none), the tick (`+ 1`) he took the role, and `tick + 1`
    /// of the last pick.
    #[serde(default)]
    pub tri_idx: u8,
    #[serde(default)]
    pub tri_since: u32,
    #[serde(default)]
    pub tri_eval: u32,
    /// `passer_ready` Schmitt state, the tick (`+ 1`) of its last toggle, `tick + 1` of the last
    /// evaluation and the carrier it was evaluated for.
    #[serde(default)]
    pub pr_on: bool,
    #[serde(default)]
    pub pr_since: u32,
    #[serde(default)]
    pub pr_eval: u32,
    #[serde(default)]
    pub pr_carrier: u8,
    /// Run arbiter: `tick + 1` of its last pass, and the number of re-score rounds so far (work-rate
    /// cadence).
    #[serde(default)]
    pub run_eval: u32,
    #[serde(default)]
    pub run_round: u32,
    /// Runner released by the CPU carrier's pass decision (`0` = none) and the tick (`+ 1`).
    #[serde(default)]
    pub run_release: u8,
    #[serde(default)]
    pub run_release_tick: u32,
    /// Near support ("players closest to the ball come to support"): `tick + 1` of the last
    /// evaluation, of the assignment, the carrier it was made for, the (up to two) assigned
    /// supporters (`AI_NONE` = none) and their support points as offsets from the carrier.
    #[serde(default)]
    pub near_eval: u32,
    #[serde(default)]
    pub near_since: u32,
    #[serde(default)]
    pub near_carrier: u8,
    #[serde(default)]
    pub near_idx: Vec<u8>,
    #[serde(default)]
    pub near_off: Vec<Vec2>,
    // ---- Round 4, area T (transition urgency and off-ball defending, `docs/transition-and-defending.md`).
    /// The press-or-fall-back decision for the current counter-press has been taken (it is made
    /// after a 6-12 tick reaction; until then the team reacts as if counter-pressing).
    #[serde(default)]
    pub cp_decided: bool,
    /// Second counter-press lane-closer (`second_presser` is the first); `AI_NONE` = none.
    #[serde(default = "ai_none_u8")]
    pub third_presser: u8,
    /// Opponent receiver whose lane each closer shuts (`[second_presser, third_presser]`).
    #[serde(default = "ai_none_pair")]
    pub closer_lane: [u8; 2],
    /// The human's teammate-contain mate registered as the second engager (`AI_NONE` = none).
    #[serde(default = "ai_none_u8")]
    pub contain_mate: u8,
    /// The one Low-defensive-work-rate forward kept high as the rest offence (`AI_NONE` = none).
    #[serde(default = "ai_none_u8")]
    pub rest_offence: u8,
    /// Shared back-line height (mm of progress from our goal line) the back-liners lag-track;
    /// `0` = not yet set.
    #[serde(default)]
    pub line_prog: i64,
    /// Ticks left of an active press trigger (heavy touch, carrier facing his own goal, carrier
    /// pinned on the touchline): the presser sprints and a second man shadows him.
    #[serde(default)]
    pub press_trigger_ticks: u16,
    /// Per player: bit 0 = recovery sprint latched (ahead of the ball, running goal-side).
    #[serde(default)]
    pub rec_flags: Vec<u8>,
    /// Per player: ticks of the current recovery burst (sprint capped at 180-240 ticks).
    #[serde(default)]
    pub burst_ticks: Vec<u16>,
    /// Per player: recent high-intensity distance (mm), a leaky integrator with a 60 s time
    /// constant over ticks above 5.5 m/s (the fatigue multiplier's input).
    #[serde(default)]
    pub hi_dist_mm: Vec<u32>,
}

const fn ai_none_u8() -> u8 {
    AI_NONE
}

const fn ai_none_pair() -> [u8; 2] {
    [AI_NONE; 2]
}

impl TeamAiMemory {
    pub(crate) fn ensure_len(&mut self, n: usize) {
        if self.slot_of.len() != n {
            self.slot_of = vec![AI_NONE; n];
            self.slot_ticks = 0;
        }
        if self.mark_of.len() != n {
            self.mark_of = vec![AI_NONE; n];
            self.mark_ticks = vec![0; n];
        }
        if self.mark_ticks.len() != n {
            self.mark_ticks = vec![0; n];
        }
        if self.target.len() != n {
            self.target = vec![Vec2::ZERO; n];
            self.target_tick = vec![0; n];
        }
        if self.target_tick.len() != n {
            self.target_tick = vec![0; n];
        }
        if self.flags.len() != n {
            self.flags = vec![0; n];
        }
        if self.cmd_vel.len() != n
            || self.cmd_tick.len() != n
            || self.steer_cmd_rate.len() != n
            || self.steer_cmd_speed.len() != n
        {
            self.cmd_vel = vec![Vec2::ZERO; n];
            self.cmd_tick = vec![0; n];
            self.steer_cmd_rate = vec![Vec2::ZERO; n];
            self.steer_cmd_speed = vec![Fp::ZERO; n];
        }
        if self.steer_chase_pt.len() != n
            || self.steer_chase_tick.len() != n
            || self.steer_chase_near.len() != n
        {
            self.steer_chase_pt = vec![Vec2::ZERO; n];
            self.steer_chase_tick = vec![0; n];
            self.steer_chase_near = vec![0; n];
        }
        if self.move_class.len() != n {
            self.move_class = vec![2; n];
        }
        // Attacking support and runs (round 4 work area 3).
        if self.sup_kind.len() != n {
            self.sup_kind = vec![AI_NONE; n];
            self.sup_since = vec![0; n];
            self.sup_eval = vec![0; n];
            self.sup_pt = vec![Vec2::ZERO; n];
            self.sup_carrier = vec![AI_NONE; n];
            self.sup_nudge = vec![0; n];
        }
        if self.sup_since.len() != n || self.sup_eval.len() != n || self.sup_pt.len() != n || self.sup_carrier.len() != n || self.sup_nudge.len() != n {
            self.sup_since = vec![0; n];
            self.sup_eval = vec![0; n];
            self.sup_pt = vec![Vec2::ZERO; n];
            self.sup_carrier = vec![AI_NONE; n];
            self.sup_nudge = vec![0; n];
        }
        if self.yield_sign.len() != n || self.gk_yield_sign.len() != n || self.bend_sign.len() != n || self.bend_tick.len() != n {
            self.yield_sign = vec![0; n];
            self.gk_yield_sign = vec![0; n];
            self.bend_sign = vec![0; n];
            self.bend_tick = vec![0; n];
        }
        if self.res_yield.len() != n {
            self.res_yield = vec![AI_NONE; n];
        }
        if self.tgt_vel.len() != n || self.tgt_raw.len() != n {
            self.tgt_vel = vec![Vec2::ZERO; n];
            self.tgt_raw = vec![Vec2::ZERO; n];
        }
        if self.near_idx.len() != 2 || self.near_off.len() != 2 {
            self.near_idx = vec![AI_NONE; 2];
            self.near_off = vec![Vec2::ZERO; 2];
        }
        if self.run_role.len() != n || self.run_since.len() != n || self.run_end.len() != n {
            self.run_role = vec![0; n];
            self.run_since = vec![0; n];
            self.run_end = vec![Vec2::ZERO; n];
        }
        if self.rec_flags.len() != n {
            self.rec_flags = vec![0; n];
        }
        if self.burst_ticks.len() != n {
            self.burst_ticks = vec![0; n];
        }
        if self.hi_dist_mm.len() != n {
            self.hi_dist_mm = vec![0; n];
        }
    }

    fn hash_into(&self, h: &mut CanonicalHasher) {
        h.u32(self.plan_tick);
        h.u8(self.phase);
        h.u16(self.phase_ticks);
        h.i64(self.phase_origin.x.raw());
        h.i64(self.phase_origin.y.raw());
        h.i64(self.ref_ball.x.raw());
        h.i64(self.ref_ball.y.raw());
        h.u8(self.presser);
        h.u16(self.presser_ticks);
        h.u8(self.second_presser);
        h.u8(self.cover);
        h.u16(self.cover_ticks);
        h.u32(self.slot_of.len() as u32);
        h.bytes(&self.slot_of);
        h.u16(self.slot_ticks);
        h.u32(self.mark_of.len() as u32);
        h.bytes(&self.mark_of);
        for t in &self.mark_ticks {
            h.u16(*t);
        }
        h.u32(self.target.len() as u32);
        for (t, tt) in self.target.iter().zip(self.target_tick.iter()) {
            h.i64(t.x.raw());
            h.i64(t.y.raw());
            h.u32(*tt);
        }
        h.bytes(&self.flags);
        h.u32(self.cmd_vel.len() as u32);
        for (v, t) in self.cmd_vel.iter().zip(self.cmd_tick.iter()) {
            h.i64(v.x.raw());
            h.i64(v.y.raw());
            h.u32(*t);
        }
        h.u32(self.steer_cmd_rate.len() as u32);
        for (r, sp) in self.steer_cmd_rate.iter().zip(self.steer_cmd_speed.iter()) {
            h.i64(r.x.raw());
            h.i64(r.y.raw());
            h.i64(sp.raw());
        }
        h.u32(self.steer_chase_pt.len() as u32);
        for ((pt, t), nr) in self
            .steer_chase_pt
            .iter()
            .zip(self.steer_chase_tick.iter())
            .zip(self.steer_chase_near.iter())
        {
            h.i64(pt.x.raw());
            h.i64(pt.y.raw());
            h.u32(*t);
            h.u8(*nr);
        }
        h.u8(self.steer_evade_sign as u8);
        h.u8(self.steer_evade_idx);
        h.u32(self.steer_evade_tick);
        h.u32(self.steer_evade_since);
        h.u8(self.chaser);
        h.u16(self.chaser_ticks);
        h.u32(self.chaser_eval_tick);
        h.i64(self.chaser_pt.x.raw());
        h.i64(self.chaser_pt.y.raw());
        h.u8(u8::from(self.keeper_claim));
        h.u32(self.move_class.len() as u32);
        h.bytes(&self.move_class);
        // Attacking support and runs (round 4 work area 3).
        h.u32(self.sup_kind.len() as u32);
        h.bytes(&self.sup_kind);
        for (((s, e), pt), c) in self.sup_since.iter().zip(&self.sup_eval).zip(&self.sup_pt).zip(&self.sup_carrier) {
            h.u32(*s);
            h.u32(*e);
            h.i64(pt.x.raw());
            h.i64(pt.y.raw());
            h.u8(*c);
        }
        let signs = |h: &mut CanonicalHasher, v: &[i8]| {
            h.u32(v.len() as u32);
            for x in v {
                h.u8(*x as u8);
            }
        };
        signs(h, &self.sup_nudge);
        signs(h, &self.yield_sign);
        signs(h, &self.gk_yield_sign);
        signs(h, &self.bend_sign);
        for t in &self.bend_tick {
            h.u32(*t);
        }
        h.u32(self.res_yield.len() as u32);
        h.bytes(&self.res_yield);
        h.u32(self.tgt_vel.len() as u32);
        for (v, r) in self.tgt_vel.iter().zip(&self.tgt_raw) {
            h.i64(v.x.raw());
            h.i64(v.y.raw());
            h.i64(r.x.raw());
            h.i64(r.y.raw());
        }
        h.u32(self.run_role.len() as u32);
        h.bytes(&self.run_role);
        for (s, e) in self.run_since.iter().zip(&self.run_end) {
            h.u32(*s);
            h.i64(e.x.raw());
            h.i64(e.y.raw());
        }
        h.u8(self.tri_idx);
        h.u32(self.tri_since);
        h.u32(self.tri_eval);
        h.u8(u8::from(self.pr_on));
        h.u32(self.pr_since);
        h.u32(self.pr_eval);
        h.u8(self.pr_carrier);
        h.u32(self.run_eval);
        h.u32(self.run_round);
        h.u8(self.run_release);
        h.u32(self.run_release_tick);
        h.u32(self.near_eval);
        h.u32(self.near_since);
        h.u8(self.near_carrier);
        h.u32(self.near_idx.len() as u32);
        h.bytes(&self.near_idx);
        for o in &self.near_off {
            h.i64(o.x.raw());
            h.i64(o.y.raw());
        }
        h.u8(u8::from(self.cp_decided));
        h.u8(self.third_presser);
        h.bytes(&self.closer_lane);
        h.u8(self.contain_mate);
        h.u8(self.rest_offence);
        h.i64(self.line_prog);
        h.u16(self.press_trigger_ticks);
        h.u32(self.rec_flags.len() as u32);
        h.bytes(&self.rec_flags);
        for t in &self.burst_ticks {
            h.u16(*t);
        }
        for d in &self.hi_dist_mm {
            h.u32(*d);
        }
    }
}

/// Velocity-stage memory of the local avoidance (`avoidance::select_velocity_mem`), indexed by the
/// avoidance frame id `side * stride + idx` (`n` = `2 * stride` agents). Sized by
/// [`SteerMemory::ensure_len`] at the start of every tick; a size change clears it.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SteerMemory {
    /// Agent count the tables are sized for.
    #[serde(default)]
    pub n: u16,
    /// `tick` of the agent's last velocity selection (`0` = none; continuity = last tick).
    #[serde(default)]
    pub sel_tick: Vec<u32>,
    /// Last selected (avoidance output) velocity per agent: tie-break reference.
    #[serde(default)]
    pub prev_sel: Vec<Vec2>,
    /// Lateral sign (`-1`, `0`, `1`) of the last selected velocity versus the agent's heading.
    #[serde(default)]
    pub lat_sign: Vec<i8>,
    /// Committed lateral sign and the ticks it is still held (dwell).
    #[serde(default)]
    pub dwell_sign: Vec<i8>,
    #[serde(default)]
    pub dwell_left: Vec<u8>,
    /// Per ordered pair `me * n + other`: bits `0..2` remembered passing side (`0` none, `1` the
    /// HRVO determinant was positive at first contact, `2` negative), bits `2..8` ticks since the
    /// pair last interacted (cleared at 60).
    #[serde(default)]
    pub pair: Vec<u8>,
}

impl SteerMemory {
    /// Size the tables for `n` agents (a size change resets everything).
    pub fn ensure_len(&mut self, n: usize) {
        if self.n as usize != n
            || self.sel_tick.len() != n
            || self.prev_sel.len() != n
            || self.lat_sign.len() != n
            || self.dwell_sign.len() != n
            || self.dwell_left.len() != n
            || self.pair.len() != n * n
        {
            *self = Self {
                n: n as u16,
                sel_tick: vec![0; n],
                prev_sel: vec![Vec2::ZERO; n],
                lat_sign: vec![0; n],
                dwell_sign: vec![0; n],
                dwell_left: vec![0; n],
                pair: vec![0; n * n],
            };
        }
    }

    fn hash_into(&self, h: &mut CanonicalHasher) {
        h.u16(self.n);
        h.u32(self.sel_tick.len() as u32);
        for (((t, v), l), (ds, dl)) in self
            .sel_tick
            .iter()
            .zip(self.prev_sel.iter())
            .zip(self.lat_sign.iter())
            .zip(self.dwell_sign.iter().zip(self.dwell_left.iter()))
        {
            h.u32(*t);
            h.i64(v.x.raw());
            h.i64(v.y.raw());
            h.u8(*l as u8);
            h.u8(*ds as u8);
            h.u8(*dl);
        }
        h.u32(self.pair.len() as u32);
        h.bytes(&self.pair);
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
    /// Desired pass/shot charge in legacy 22-unit scale (converted with `charge_units_to_ticks` at use).
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
    /// Per-side goalkeeper scenario brain (`[home, away]`).
    #[serde(default)]
    pub gk_brain: [GoalkeeperBrain; 2],
    /// Outfield AI team-shape commitments (`[home, away]`). Interior-mutable because the AI plans
    /// from `&MatchState` mid-tick; it is serialized and included in [`Self::compute_hash`].
    #[serde(default)]
    pub ai_memory: core::cell::RefCell<[TeamAiMemory; 2]>,
    /// Local-avoidance steering memory (WA1: committed lateral sign, dwell, per-pair passing side).
    /// Serialized and hashed: it changes the simulation.
    #[serde(default)]
    pub steer_memory: SteerMemory,
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
        for b in &self.gk_brain {
            h.u8(b.scenario.code());
            h.u16(b.commit_ticks);
            h.u32(b.entered_tick);
            h.u32(b.last_decision_tick);
            h.i64(b.target.x.raw());
            h.i64(b.target.y.raw());
            h.u16(b.reentry_block_ticks);
            h.u32(b.switches);
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

        for mem in self.ai_memory.borrow().iter() {
            mem.hash_into(&mut h);
        }
        self.steer_memory.hash_into(&mut h);

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
                h.u8(p.jump_recover_ticks);
                h.u8(p.overlap_ticks);
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
