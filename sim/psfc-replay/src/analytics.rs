//! `psfc-replay analyze`: deterministic CPU-vs-CPU match analytics.
//!
//! Strictly read-only with respect to `psfc-sim`: it builds a sample fixture, sets both controllers
//! to `SideController::Cpu`, ticks the simulation with idle external input (the sim generates CPU
//! input itself) and samples the public `MatchState` after every tick. Floats are used here for
//! reporting only; nothing is fed back into the simulation.
//!
//! Every metric key and its exact definition is listed in [`metric_definitions`], which is also
//! emitted into the JSON and Markdown reports.

use psfc_sim::{
    CpuDifficulty, FixtureSnapshot, Fp, InputFrame, MatchEventType, MatchPhase, MatchSimulation,
    MatchState, PlayerRole, SideController, TeamSide, Vec2, TICKS_PER_SECOND,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;

/// Metric map: dotted metric key -> value. Sorted, so JSON output is stable and diffable.
pub type Metrics = BTreeMap<String, f64>;

const TICKS_PER_MIN: f64 = (TICKS_PER_SECOND * 60) as f64;
/// FIFA/IFAB goal area (six-yard box) depth and its extension beyond each post.
const SIX_YARD_MM: f64 = 5_500.0;
const MOVING_SPEED: f64 = 1.0;
const SWARM_RADIUS: f64 = 1.6;
const GK_OFF_LINE_GOAL_M: f64 = 3.0;
const AIRBORNE_Z: f64 = 0.0;
const HEADER_MIN_Z: f64 = 0.14;
/// How far back (ticks) a goal may be from the scoring side's last `Shot` event to be attributed to it.
const GOAL_SHOT_LOOKBACK_TICKS: u32 = 10 * TICKS_PER_SECOND;

// Movement-coordination metrics (round 3, see docs/04 / the research report "What to measure").
/// Centre-to-centre distance below which two players overlap (the sim's `min_sep`, 2 x body radius 0.4 m).
const OVERLAP_DIST_M: f64 = 0.8;
/// An overlap episode longer than this many consecutive open-play ticks is a "stuck pair".
const STUCK_TICKS: u32 = 10;
/// Overlap depth above which a contact counts as real penetration rather than resting contact
/// (the separation pass leaves resolved pairs at 0.8 m minus fixed-point rounding).
const DEEP_OVERLAP_M: f64 = 0.01;
/// A position-correction sample above this (m/s) counts as "corrected" (fixed-point noise is ~1e-7).
const CORRECTED_MIN_MPS: f64 = 0.01;
/// Position-correction velocity threshold (research report target p99).
const CORRECTION_VEL_GATE: f64 = 0.3;
/// Dead band (m/s per tick) on the lateral velocity component for side-flip detection.
const SIDE_FLIP_EPS: f64 = 0.02;
const CHASE_RADIUS_M: f64 = 8.0;
const CHASE_CLOSING_MPS: f64 = 1.5;
const ENGAGE_RADIUS_M: f64 = 2.5;
const CRUISE_BALL_RADIUS_M: f64 = 8.0;
const CRUISE_FRACTION: f64 = 0.72;
const HEADING_P95_MIN_SPEED: f64 = 3.0;

// Round 4: defensive transition (`trans.*`), attacking support (`support.*`), flip detail.
/// Max loose-ball gap (ticks) between the losing side's last ownership and the opponent's first.
const TURNOVER_GAP_TICKS: u32 = 2 * TICKS_PER_SECOND;
/// Transition episode horizon / `time_to_shape` cap (ticks).
const TRANS_CAP_TICKS: u32 = 15 * TICKS_PER_SECOND;
const TRANS_SHAPE_RADIUS_M: f64 = 5.0;
const TRANS_RECOVERY_TICKS: u32 = 5 * TICKS_PER_SECOND;
const TRANS_SPRINT_MPS: f64 = 6.0;
const TRANS_COUNTERPRESS_TICKS: u32 = 6 * TICKS_PER_SECOND;
const SUPPORT_MIN_M: f64 = 6.0;
const SUPPORT_MAX_M: f64 = 30.0;
const LANE_CLEAR_M: f64 = 1.5;
const FORWARD_OPTION_M: f64 = 3.0;
const RUN_BEHIND_MPS: f64 = 5.0;
/// Round 4: a committed run ended before this many ticks counts as abandoned (work area 5).
const RUN_ABANDON_TICKS: u32 = 60;
/// A runner who crossed the line must drop this far behind it again before a new run can count.
const RUN_REARM_M: f64 = 1.0;
const CLUSTER_PAIR_M: f64 = 4.0;
const CLUSTER_CARRIER_M: f64 = 15.0;
const SPEED_BAND_MPS: f64 = 3.0;
const HEADING_FLIP_MIN_DEG: f64 = 1.0;
/// Schmitt-band side flips (WA1 instrumentation): band on the lateral component (m/s) and the
/// number of consecutive samples a new side must persist before the debounced side changes.
const SIDE_FLIP_SCHMITT_EPS: f64 = 0.05;
const SIDE_FLIP_SCHMITT_DWELL: u8 = 4;
/// Side-flip cause buckets (`move.side_flips_cause.<name>`), in pipeline order.
const FLIP_CAUSES: [&str; 4] = ["desired", "sampler", "filter", "limiter"];
/// A Pass event is attributed to the open-play carrier sampled at most this many ticks before it.
const PASS_CARRIER_LOOKBACK_TICKS: u32 = 2;

#[inline]
fn dist(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

/// Distance from `p` to the segment `a..b`.
fn dist_to_segment(p: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    let ab = (b.0 - a.0, b.1 - a.1);
    let len2 = ab.0 * ab.0 + ab.1 * ab.1;
    if len2 <= 1e-12 {
        return dist(p, a);
    }
    let t = (((p.0 - a.0) * ab.0 + (p.1 - a.1) * ab.1) / len2).clamp(0.0, 1.0);
    dist(p, (a.0 + t * ab.0, a.1 + t * ab.1))
}

/// Second-last on-pitch player of `defending` (GK included) projected on the attacking axis of
/// the opponent (`x * sign`, sign = attacking side's attack_sign); `None` with fewer than 2.
fn second_last_proj(st: &MatchState, defending: TeamSide, sign: f64) -> Option<f64> {
    let mut v: Vec<f64> = st.players[defending.index()]
        .iter()
        .filter(|p| !p.sent_off)
        .map(|p| f(p.pos.x) * sign)
        .collect();
    if v.len() < 2 {
        return None;
    }
    v.sort_by(f64::total_cmp);
    Some(v[v.len() - 2])
}

/// Offside position of `players[side][idx]`, mirroring the sim's `is_player_offside_at_pass`
/// (offsides enabled, past the offside zone line: halfway in 11v11, +half_len/3 in 5v5; more than
/// 0.15 m ahead of the ball and more than 0.2 m ahead of the second-last opponent). The GK is never
/// offside.
fn offside_position(st: &MatchState, side: TeamSide, idx: usize, sign: f64) -> bool {
    if !st.rules.offsides_enabled || idx == 0 {
        return false;
    }
    let recv = f(st.players[side.index()][idx].pos.x) * sign;
    let zone = if st.format.players_per_side <= 6 {
        f(st.format.half_length()) / 3.0
    } else {
        0.0
    };
    if recv <= zone || recv <= f(st.ball.pos.x) * sign + 0.15 {
        return false;
    }
    second_last_proj(st, side.opponent(), sign).is_some_and(|sl| recv > sl + 0.2)
}

/// Is the lane from `from` to `to` clear (no on-pitch player of `opp` within 1.5 m of the segment)?
fn lane_open(st: &MatchState, opp: TeamSide, from: (f64, f64), to: (f64, f64)) -> bool {
    st.players[opp.index()]
        .iter()
        .filter(|p| !p.sent_off)
        .all(|p| dist_to_segment(v2(p.pos), from, to) > LANE_CLEAR_M)
}

/// Sprint top speed (m/s) at full stamina, mirroring `step_all_players`:
/// `(5450 + 26 * pace) mm/s * (1 + max_sprint_bonus)`, bonus 0.205 (5v5) / 0.220 (11v11).
fn approx_top_speed(pace: u8, five_a_side: bool) -> f64 {
    let base = (5_450.0 + 26.0 * f64::from(pace)) / 1_000.0;
    base * if five_a_side { 1.205 } else { 1.220 }
}

/// Nearest-rank percentile (`q` in 0..=1) of an unsorted sample; 0 for an empty sample.
fn percentile(v: &mut [f32], q: f64) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    v.sort_by(f32::total_cmp);
    let rank = ((q * v.len() as f64).ceil() as usize).clamp(1, v.len());
    f64::from(v[rank - 1])
}

/// Overlap / stuck-pair accumulator for one pair class.
#[derive(Default, Clone)]
struct CollAcc {
    /// Open-play ticks with at least one overlapping pair of this class.
    ticks: u64,
    /// Open-play ticks with at least one pair of this class overlapping by more than 1 cm.
    deep_ticks: u64,
    /// Overlap episodes (pair goes from >= 0.8 m to < 0.8 m, or starts overlapped at open play).
    events: u64,
    depth: Mean,
    depth_max: f64,
    stuck: u64,
    stuck_dur: Mean,
}

impl CollAcc {
    fn close_run(&mut self, run: u32) {
        if run > STUCK_TICKS {
            self.stuck += 1;
            self.stuck_dur.add(f64::from(run));
        }
    }
}

/// Pair classes: 0 any pair, 1 home-home, 2 away-away, 3 home-away.
const COLL_ANY: usize = 0;
const COLL_OPP: usize = 3;

#[inline]
fn f(x: Fp) -> f64 {
    x.raw() as f64 / Fp::ONE_RAW as f64
}

#[inline]
fn v2(v: Vec2) -> (f64, f64) {
    (f(v.x), f(v.y))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormatKind {
    #[serde(rename = "5v5")]
    FiveASide,
    #[serde(rename = "11v11")]
    ElevenASide,
}

impl FormatKind {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "5v5" | "5" => Ok(Self::FiveASide),
            "11v11" | "11" => Ok(Self::ElevenASide),
            o => Err(format!("invalid --format '{o}' (expected 5v5 or 11v11)")),
        }
    }
    pub const fn label(self) -> &'static str {
        match self {
            Self::FiveASide => "5v5",
            Self::ElevenASide => "11v11",
        }
    }
}

pub fn parse_difficulty(s: &str) -> Result<CpuDifficulty, String> {
    match s.to_ascii_lowercase().as_str() {
        "easy" => Ok(CpuDifficulty::Easy),
        "normal" => Ok(CpuDifficulty::Normal),
        "hard" => Ok(CpuDifficulty::Hard),
        o => Err(format!(
            "invalid difficulty '{o}' (expected easy|normal|hard)"
        )),
    }
}

const fn difficulty_label(d: CpuDifficulty) -> &'static str {
    match d {
        CpuDifficulty::Easy => "easy",
        CpuDifficulty::Normal => "normal",
        CpuDifficulty::Hard => "hard",
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeConfig {
    pub format: FormatKind,
    pub seeds: u32,
    pub seed_base: u64,
    pub minutes: u32,
    pub home_difficulty: CpuDifficulty,
    pub away_difficulty: CpuDifficulty,
    /// Round 4: the home side is `SideController::Human`, fed `ai::suggest_cpu_input` every tick.
    #[serde(default)]
    pub home_autopilot: bool,
    #[serde(default)]
    pub away_autopilot: bool,
}

impl AnalyzeConfig {
    /// `hard` for a CPU side, `autopilot-hard` for an auto-piloted human side.
    fn side_label(&self, side: TeamSide) -> String {
        let (d, ap) = match side {
            TeamSide::Home => (self.home_difficulty, self.home_autopilot),
            TeamSide::Away => (self.away_difficulty, self.away_autopilot),
        };
        if ap {
            format!("autopilot-{}", difficulty_label(d))
        } else {
            difficulty_label(d).to_string()
        }
    }

    pub fn label(&self) -> String {
        format!(
            "{} {} vs {} ({} min x {} seeds from {})",
            self.format.label(),
            self.side_label(TeamSide::Home),
            self.side_label(TeamSide::Away),
            self.minutes,
            self.seeds,
            self.seed_base
        )
    }

    /// Stable id used by `--compare` to pair reports across runs (unchanged for CPU-vs-CPU).
    pub fn id(&self) -> String {
        format!(
            "{}_{}_vs_{}_{}min_{}x{}",
            self.format.label(),
            self.side_label(TeamSide::Home),
            self.side_label(TeamSide::Away),
            self.minutes,
            self.seeds,
            self.seed_base
        )
    }

    pub fn command_line(&self) -> String {
        let mut s = format!(
            "psfc-replay analyze --format {} --seeds {} --seed-base {} --minutes {} --home-difficulty {} --away-difficulty {}",
            self.format.label(),
            self.seeds,
            self.seed_base,
            self.minutes,
            difficulty_label(self.home_difficulty),
            difficulty_label(self.away_difficulty)
        );
        if self.home_autopilot {
            s.push_str(" --home-controller autopilot");
        }
        if self.away_autopilot {
            s.push_str(" --away-controller autopilot");
        }
        s
    }

    /// Half duration in ticks so that the full match clock is `minutes`.
    pub fn half_duration_ticks(&self) -> u32 {
        (self.minutes * 60 * TICKS_PER_SECOND / 2).max(1)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeedReport {
    pub seed: u64,
    pub final_hash_hex: String,
    pub ticks_simulated: u32,
    pub finished: bool,
    pub score: [u16; 2],
    pub match_metrics: Metrics,
    pub home: Metrics,
    pub away: Metrics,
    /// Name of a goalkeeper-scenario field found in the public state (`None` until the sim exposes one).
    pub gk_scenario_field: Option<String>,
    /// Ticks spent in each observed scenario value, per side.
    pub gk_scenario_ticks: [BTreeMap<String, u64>; 2],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigReport {
    pub id: String,
    pub label: String,
    pub command: String,
    pub config: AnalyzeConfig,
    pub gameplay_version: String,
    pub final_hashes: Vec<String>,
    pub mean_match: Metrics,
    pub mean_home: Metrics,
    pub mean_away: Metrics,
    /// Mean of home and away (both sides are CPU, so this is the headline per-team number).
    pub mean_teams: Metrics,
    #[serde(default)]
    pub per_seed: Vec<SeedReport>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub tool: String,
    pub reports: Vec<ConfigReport>,
    pub definitions: BTreeMap<String, String>,
}

// ---------------------------------------------------------------------------------------------
// Accumulators
// ---------------------------------------------------------------------------------------------

#[derive(Default, Clone, Copy)]
struct Mean {
    sum: f64,
    n: u64,
}

impl Mean {
    fn add(&mut self, v: f64) {
        self.sum += v;
        self.n += 1;
    }
    fn get(self) -> f64 {
        if self.n == 0 {
            0.0
        } else {
            self.sum / self.n as f64
        }
    }
}

const ROLES: [(&str, PlayerRole); 4] = [
    ("goalkeeper", PlayerRole::Goalkeeper),
    ("defender", PlayerRole::Defender),
    ("midfielder", PlayerRole::Midfielder),
    ("forward", PlayerRole::Forward),
];

fn role_index(r: PlayerRole) -> usize {
    match r {
        PlayerRole::Goalkeeper => 0,
        PlayerRole::Defender => 1,
        PlayerRole::Midfielder => 2,
        PlayerRole::Forward => 3,
    }
}

/// Index 0 = attacking, 1 = defending.
#[derive(Default, Clone)]
struct PhaseAcc {
    ticks: u64,
    anchor_dist: [Mean; 4],
    anchor_dist_shape: [Mean; 4],
    line_var: Mean,
    line_std: Mean,
    behind_ball: Mean,
    bbox_area: Mean,
    bbox_depth: Mean,
    bbox_width: Mean,
    carrier_ticks: u64,
    swarm2: u64,
    swarm3: u64,
    // Round-3 shape
    length: Mean,
    width: Mean,
    stretch: Mean,
    nearest_mate: Vec<f32>,
}

#[derive(Default, Clone)]
struct SideAcc {
    open_ticks: u64,
    // Goalkeeper
    gk_out6: u64,
    gk_outbox: u64,
    gk_line_dist: Mean,
    gk_line_dist_max: f64,
    gk_rush: u64,
    /// Length (ticks) of the current continuous open-play stretch outside the goal area.
    gk_excursion_len: u64,
    gk_excursions: Mean,
    gk_excursions_gt1s: u64,
    gk_prev_in6: Option<bool>,
    gk_gathers: u64,
    gk_catches: u64,
    gk_parries: u64,
    gk_tips: u64,
    goals_conceded: u64,
    goals_conceded_gk_off_line: u64,
    goals_conceded_no_shot: u64,
    gk_line_dist_at_conceded_shot: Mean,
    gk_scenario_prev: Option<String>,
    gk_scenario_switches: u64,
    gk_scenario_ticks: BTreeMap<String, u64>,
    // Shape
    phase: [PhaseAcc; 2],
    // Movement
    heading_hist: [u64; 4],
    turn_rate: Mean,
    lateral_accel: Mean,
    reversals: u64,
    accel: Mean,
    accel_gt10: u64,
    speed_osc: u64,
    speed_osc_n: u64,
    accel_pos: Mean,
    max_dspeed: f64,
    player_ticks: u64,
    // Aerial
    jumps_outfield: u64,
    jumps_gk: u64,
    max_airborne: u64,
    headers: u64,
    headers_shot: u64,
    headers_clear: u64,
    headers_pass: u64,
    // Events
    offsides: u64,
    // Coordination (round 3)
    corr_vel: Vec<f32>,
    side_flips: u64,
    heading_gt3: Vec<f32>,
    loose_ticks: u64,
    dual_chaser_ticks: u64,
    max_chasers: u64,
    engager3: u64,
    cruise_n: u64,
    cruise_over: u64,
    // Round 4: defensive transition (this side lost the ball).
    trans_count: u64,
    trans_ep: Option<TransEp>,
    trans_shape: Mean,
    trans_shape_capped: u64,
    trans_shape_excluded: u64,
    trans_shape_at_t0: u64,
    trans_shape_static: Mean,
    trans_shape_static_capped: u64,
    trans_shape_static_excluded: u64,
    trans_behind3: Mean,
    trans_behind6: Mean,
    trans_fwd_speed: Mean,
    trans_fwd_sprint: u64,
    trans_mid_speed: Mean,
    trans_mid_sprint: u64,
    trans_fwd_high: Mean,
    trans_counterpress: u64,
    // Round 4: attacking support (this side has an outfield carrier).
    sup_ticks: u64,
    sup_open: Vec<f32>,
    sup_le1: u64,
    sup_nearest: Mean,
    sup_fwd: Mean,
    sup_cluster: u64,
    sup_width: Mean,
    sup_depth: Mean,
    sup_runs: u64,
    /// Per slot: 0 = not armed, 1 = armed (at/behind the line), 2 = crossed (re-arm 1 m behind).
    run_state: Vec<u8>,
    /// Round 4 (work area 3): committed runs seen last open-play tick, per slot `(role, since)`
    /// from the sim's `TeamAiMemory::run_role` / `run_since`, and abandoned runs counted.
    run_prev: Vec<(u8, u32)>,
    sup_run_abandon: u64,
    /// Intended receiver of this side's pass in flight on the previous open-play tick.
    prev_pass_to: Option<usize>,
    /// Releases of committed runners (CPU pass decision or a pass launched toward the runner),
    /// the last CPU release stamp seen, and the release passes pending / launched / completed.
    sup_releases: u64,
    rel_last_stamp: u32,
    rel_last_cpu: Option<(usize, u32)>,
    rel_pending: Option<usize>,
    rel_passes: u64,
    rel_completed: u64,
    /// Last open-play outfield-carrier sample: (carrier idx, tick, open-lane mask of teammates).
    last_carrier: Option<(usize, u32, u32)>,
    /// Pass in progress: (passer idx, open-lane mask at the last carrier sample).
    pending_pass: Option<(usize, u32)>,
    passes_completed: u64,
    passes_open: u64,
    // Round 4: flip detail.
    flips_class: [u64; 4],
    ticks_class: [u64; 4],
    flips_band: [u64; 2],
    ticks_band: [u64; 2],
    heading_flips: u64,
    // WA1 instrumentation: Schmitt-band side flips and the raw side flips split by cause.
    flips_schmitt: u64,
    flips_cause: [u64; 4],
}

/// An active defensive-transition episode of the side that lost the ball.
#[derive(Clone, Copy)]
struct TransEp {
    t0: u32,
    shape_done: bool,
    static_done: bool,
}

#[derive(Clone)]
struct PrevPlayer {
    id: String,
    vel: (f64, f64),
    pos: (f64, f64),
    /// Velocity derived from the position delta to the tick before (m/s), if that tick was sampled.
    pos_vel: Option<(f64, f64)>,
    /// Speed change (m/s) from the tick before, if that tick was sampled.
    dspeed: Option<f64>,
    z: f64,
    recent_kick_ticks: u8,
    /// Signed lateral velocity component (m/s) of the last heading sample, if that tick had one.
    lat: Option<f64>,
    /// Signed heading change (deg) of the last heading sample, if that tick had one.
    theta: Option<f64>,
    /// Signed lateral component (m/s, versus the previous tick's velocity) of the steering stages
    /// `[desired, selected, filtered]` on the last heading sample (psfc_sim `steer_debug`).
    lat_stage: [Option<f64>; 3],
    /// Schmitt-band side detector: debounced side (`-1`, `0`, `1`), the side currently trying to
    /// take over and for how many consecutive samples.
    sch_side: i8,
    sch_cand: i8,
    sch_run: u8,
}

fn close_excursion(a: &mut SideAcc) {
    let secs = a.gk_excursion_len as f64 / f64::from(TICKS_PER_SECOND);
    a.gk_excursions.add(secs);
    if secs > 1.0 {
        a.gk_excursions_gt1s += 1;
    }
    a.gk_excursion_len = 0;
}

/// The six-yard box and penalty box membership for a point relative to `side`'s own goal.
fn box_flags(st: &MatchState, side: TeamSide, p: (f64, f64)) -> (bool, bool, f64) {
    let sign = side.attack_sign(st.second_half_swapped()) as f64;
    let half_len = st.format.pitch_length_mm as f64 / 2_000.0;
    let own_goal_x = -sign * half_len;
    let depth = (p.0 - own_goal_x).abs();
    let goal_hw = st.format.goal_width_mm as f64 / 2_000.0;
    let in6 = depth <= SIX_YARD_MM / 1_000.0 && p.1.abs() <= goal_hw + SIX_YARD_MM / 1_000.0;
    let inbox = depth <= st.format.penalty_box_depth_mm as f64 / 1_000.0
        && p.1.abs() <= st.format.penalty_box_width_mm as f64 / 2_000.0;
    (in6, inbox, depth)
}

/// Looks for a goalkeeper scenario/mode field in the serialized GK `PlayerState`, then in the
/// top-level `MatchState` (per-side arrays are indexed by side). Purely reflective so this tool
/// compiles unchanged against sims with or without such a field.
fn is_scenario_key(k: &str) -> bool {
    let k = k.to_ascii_lowercase();
    k.contains("scenario") || k.contains("gk_mode") || k.contains("keeper_mode") || k == "gk_state"
}

fn scenario_value(v: &serde_json::Value, side: TeamSide) -> String {
    match v {
        serde_json::Value::Array(a) if a.len() == 2 => a[side.index()].to_string(),
        other => other.to_string(),
    }
}

struct ScenarioProbe {
    /// `Some((in_player_state, key))` once discovered.
    key: Option<(bool, String)>,
}

impl ScenarioProbe {
    fn discover(st: &MatchState) -> Self {
        if let Ok(serde_json::Value::Object(m)) = serde_json::to_value(&st.players[0][0]) {
            if let Some(k) = m.keys().find(|k| is_scenario_key(k)) {
                return Self {
                    key: Some((true, k.clone())),
                };
            }
        }
        if let Ok(serde_json::Value::Object(m)) = serde_json::to_value(st) {
            if let Some(k) = m.keys().find(|k| is_scenario_key(k)) {
                return Self {
                    key: Some((false, k.clone())),
                };
            }
        }
        Self { key: None }
    }

    fn sample(&self, st: &MatchState) -> Option<[String; 2]> {
        let (in_player, key) = self.key.as_ref()?;
        if *in_player {
            let get = |side: TeamSide| -> String {
                serde_json::to_value(&st.players[side.index()][0])
                    .ok()
                    .and_then(|v| v.get(key).map(|x| x.to_string()))
                    .unwrap_or_default()
            };
            Some([get(TeamSide::Home), get(TeamSide::Away)])
        } else {
            let v = serde_json::to_value(st).ok()?;
            let x = v.get(key)?;
            Some([
                scenario_value(x, TeamSide::Home),
                scenario_value(x, TeamSide::Away),
            ])
        }
    }
}

/// Runs one CPU-vs-CPU match for `seed` and returns its metrics.
pub fn analyze_seed(cfg: &AnalyzeConfig, seed: u64) -> Result<SeedReport, String> {
    let mut snapshot = match cfg.format {
        FormatKind::FiveASide => FixtureSnapshot::sample_five_a_side(seed),
        FormatKind::ElevenASide => FixtureSnapshot::sample_eleven_a_side(seed),
    };
    snapshot.rules.half_duration_ticks = cfg.half_duration_ticks();
    let side_controller = |ap: bool, d: CpuDifficulty| {
        if ap {
            SideController::Human
        } else {
            SideController::Cpu(d)
        }
    };
    snapshot.rules.home_controller = side_controller(cfg.home_autopilot, cfg.home_difficulty);
    snapshot.rules.away_controller = side_controller(cfg.away_autopilot, cfg.away_difficulty);

    let mut sim = MatchSimulation::new(&snapshot).map_err(|e| e.to_string())?;
    let probe = ScenarioProbe::discover(sim.state());
    let top_level_probe = matches!(probe.key, Some((false, _)));

    let mut acc = [SideAcc::default(), SideAcc::default()];
    let mut prev: [Vec<Option<PrevPlayer>>; 2] = [Vec::new(), Vec::new()];
    // GK distance from own goal line per tick (index = tick), for "GK off line at the shot".
    let mut gk_line_hist: Vec<[f32; 2]> = vec![[0.0, 0.0]];
    let mut last_shot_tick: [Option<u32>; 2] = [None, None];
    let mut prev_score = sim.state().score;
    let mut prev_gk_owner = [false, false];
    let mut open_ticks_total: u64 = 0;
    let five = matches!(cfg.format, FormatKind::FiveASide);
    // Collisions: per-pair current overlap run length, pairs indexed over the flattened
    // (side * per_side + slot) list.
    let per_side = sim.state().players[0]
        .len()
        .max(sim.state().players[1].len());
    let n_flat = 2 * per_side;
    let mut coll: Vec<CollAcc> = vec![CollAcc::default(); 4];
    let mut pair_run: Vec<u32> = vec![0; n_flat * n_flat];
    let pair_class = |a: usize, b: usize| -> usize {
        match (a / per_side, b / per_side) {
            (0, 0) => 1,
            (1, 1) => 2,
            _ => COLL_OPP,
        }
    };

    let safety_cap = cfg
        .half_duration_ticks()
        .saturating_mul(5)
        .saturating_add(6_000);
    // Round 4: last open-play ball owner (side, tick), for turnover detection.
    let mut last_owner: Option<(TeamSide, u32)> = None;
    while !sim.is_finished() && sim.state().tick < safety_cap {
        // Auto-piloted human sides get the side-effect-free CPU suggestion for the current state
        // (computed outside `tick`, as `suggest_cpu_input` requires); CPU sides ignore the frame.
        let input_for = |side: TeamSide, ap: bool, d: CpuDifficulty| {
            if ap {
                psfc_sim::ai::suggest_cpu_input(sim.state(), side, d)
            } else {
                InputFrame::IDLE
            }
        };
        let inputs = [
            input_for(TeamSide::Home, cfg.home_autopilot, cfg.home_difficulty),
            input_for(TeamSide::Away, cfg.away_autopilot, cfg.away_difficulty),
        ];
        sim.tick(&inputs);
        let events = sim.drain_events();
        let st = sim.state();
        let open = matches!(st.phase, MatchPhase::OpenPlay);
        let swapped = st.second_half_swapped();

        // GK line distance (all ticks, so shots during any phase can be looked up).
        let mut gk_row = [0.0_f32; 2];
        for side in [TeamSide::Home, TeamSide::Away] {
            let gk = &st.players[side.index()][0];
            let (_, _, depth) = box_flags(st, side, v2(gk.pos));
            gk_row[side.index()] = depth as f32;
        }
        gk_line_hist.push(gk_row);

        for ev in &events {
            let s = ev.team.index();
            match ev.event_type {
                MatchEventType::Shot => last_shot_tick[s] = Some(ev.tick),
                MatchEventType::Offside => acc[s].offsides += 1,
                MatchEventType::Pass => {
                    let a = &mut acc[s];
                    a.pending_pass = a
                        .last_carrier
                        .filter(|&(_, t, _)| {
                            ev.tick.saturating_sub(t) <= PASS_CARRIER_LOOKBACK_TICKS
                        })
                        .map(|(ci, _, mask)| (ci, mask));
                }
                MatchEventType::Save => {
                    if ev.detail.contains("catches") {
                        acc[s].gk_catches += 1;
                    } else if ev.detail.contains("tips") {
                        acc[s].gk_tips += 1;
                    } else {
                        acc[s].gk_parries += 1;
                    }
                }
                _ => {}
            }
        }

        // Goals conceded with GK position at the shot.
        for side in [TeamSide::Home, TeamSide::Away] {
            let s = side.index();
            let scored = st.score[s].saturating_sub(prev_score[s]);
            for _ in 0..scored {
                let conceding = side.opponent().index();
                acc[conceding].goals_conceded += 1;
                let shot_tick = last_shot_tick[s]
                    .filter(|&t| st.tick.saturating_sub(t) <= GOAL_SHOT_LOOKBACK_TICKS);
                match shot_tick {
                    Some(t) => {
                        let d = gk_line_hist
                            .get(t as usize)
                            .map_or(0.0, |r| f64::from(r[conceding]));
                        acc[conceding].gk_line_dist_at_conceded_shot.add(d);
                        if d > GK_OFF_LINE_GOAL_M {
                            acc[conceding].goals_conceded_gk_off_line += 1;
                        }
                    }
                    None => acc[conceding].goals_conceded_no_shot += 1,
                }
            }
        }
        prev_score = st.score;

        // Scenario sampling (top-level probes are serialized only once per 30 ticks: that path
        // serializes the whole state).
        if probe.key.is_some() && (!top_level_probe || st.tick % 30 == 0) {
            if let Some(vals) = probe.sample(st) {
                for (s, val) in vals.into_iter().enumerate() {
                    let a = &mut acc[s];
                    *a.gk_scenario_ticks.entry(val.clone()).or_insert(0) +=
                        if top_level_probe { 30 } else { 1 };
                    if open && a.gk_scenario_prev.as_ref().is_some_and(|p| *p != val) {
                        a.gk_scenario_switches += 1;
                    }
                    a.gk_scenario_prev = Some(val);
                }
            }
        }

        if !open {
            for side in [TeamSide::Home, TeamSide::Away] {
                let a = &mut acc[side.index()];
                a.gk_prev_in6 = None;
                if a.gk_excursion_len > 0 {
                    close_excursion(a);
                }
                prev[side.index()].iter_mut().for_each(|p| *p = None);
                // Round 4: a stoppage ends (censors) a transition episode and any pass or run.
                if let Some(ep) = a.trans_ep.take() {
                    a.trans_shape_excluded += u64::from(!ep.shape_done);
                    a.trans_shape_static_excluded += u64::from(!ep.static_done);
                }
                a.run_state.iter_mut().for_each(|r| *r = 0);
                // A stoppage ends committed runs without counting them as abandoned.
                a.run_prev.iter_mut().for_each(|r| *r = (0, 0));
                // A release pass ended by a stoppage (offside, out of play) is not completed.
                if a.rel_pending.take().is_some() {
                    a.rel_passes += 1;
                }
                a.prev_pass_to = None;
                a.last_carrier = None;
                a.pending_pass = None;
            }
            last_owner = None;
            prev_gk_owner = [false, false];
            for a in 0..n_flat {
                for b in (a + 1)..n_flat {
                    let run = std::mem::take(&mut pair_run[a * n_flat + b]);
                    if run > 0 {
                        coll[COLL_ANY].close_run(run);
                        coll[pair_class(a, b)].close_run(run);
                    }
                }
            }
            continue;
        }
        open_ticks_total += 1;

        let ball = (f(st.ball.pos.x), f(st.ball.pos.y));
        let poss_team = st.ball.owner.map(|(s, _)| s).or(st.ball.last_touch_team);
        let carrier = st.ball.owner;

        // ---------------- Round 4: turnovers and defensive transition ----------------
        if let Some((os, oi)) = carrier {
            if let Some((ls, lt)) = last_owner {
                if ls != os && st.tick.saturating_sub(lt) <= TURNOVER_GAP_TICKS {
                    let a = &mut acc[ls.index()];
                    a.trans_count += 1;
                    // The losing side has no active episode: it would have ended when it regained.
                    a.trans_ep = Some(TransEp {
                        t0: st.tick,
                        shape_done: false,
                        static_done: false,
                    });
                }
            }
            last_owner = Some((os, st.tick));
            // Completed pass: a teammate of the passer becomes the owner; any other owner
            // (opponent, or the passer himself) ends the pass uncompleted.
            for side in [TeamSide::Home, TeamSide::Away] {
                let a = &mut acc[side.index()];
                if let Some((passer, mask)) = a.pending_pass.take() {
                    if os == side && oi != passer {
                        a.passes_completed += 1;
                        a.passes_open += u64::from(mask & (1 << oi) != 0);
                    }
                }
            }
        }
        for side in [TeamSide::Home, TeamSide::Away] {
            let s = side.index();
            let a = &mut acc[s];
            let Some(mut ep) = a.trans_ep else {
                continue;
            };
            let e = st.tick.saturating_sub(ep.t0);
            if carrier.is_some_and(|(cs, _)| cs == side) {
                // Regained before the episode horizon.
                if e <= TRANS_COUNTERPRESS_TICKS {
                    a.trans_counterpress += 1;
                }
                a.trans_shape_excluded += u64::from(!ep.shape_done);
                a.trans_shape_static_excluded += u64::from(!ep.static_done);
                a.trans_ep = None;
                continue;
            }
            let sign = side.attack_sign(swapped) as f64;
            let players = &st.players[s];
            let outfield: Vec<usize> = (1..players.len())
                .filter(|&i| !players[i].sent_off)
                .collect();
            let n = outfield.len();
            let ahead = |i: usize| (f(players[i].pos.x) - ball.0) * sign;
            let secs = f64::from(e) / f64::from(TICKS_PER_SECOND);
            if n > 0 && (!ep.shape_done || !ep.static_done) {
                let (mut ok_dyn, mut ok_static) = (0_usize, 0_usize);
                for &i in &outfield {
                    let pos = v2(players[i].pos);
                    let gs = ahead(i) < 0.0;
                    if !ep.shape_done
                        && (gs
                            || dist(pos, v2(psfc_sim::ai::defensive_shape_anchor(st, side, i)))
                                <= TRANS_SHAPE_RADIUS_M)
                    {
                        ok_dyn += 1;
                    }
                    if !ep.static_done
                        && (gs
                            || dist(
                                pos,
                                v2(psfc_sim::ai::world_anchor_for_player(st, side, i, Fp::ZERO)),
                            ) <= TRANS_SHAPE_RADIUS_M)
                    {
                        ok_static += 1;
                    }
                }
                if !ep.shape_done && ok_dyn * 5 >= n * 4 {
                    a.trans_shape.add(secs);
                    a.trans_shape_at_t0 += u64::from(e == 0);
                    ep.shape_done = true;
                }
                if !ep.static_done && ok_static * 5 >= n * 4 {
                    a.trans_shape_static.add(secs);
                    ep.static_done = true;
                }
            }
            if n > 0 && (e == 3 * TICKS_PER_SECOND || e == 6 * TICKS_PER_SECOND) {
                let behind = outfield.iter().filter(|&&i| ahead(i) < 0.0).count() as f64 / n as f64;
                if e == 3 * TICKS_PER_SECOND {
                    a.trans_behind3.add(behind);
                } else {
                    a.trans_behind6.add(behind);
                    let high = outfield
                        .iter()
                        .filter(|&&i| players[i].role == PlayerRole::Forward && ahead(i) > 0.0)
                        .count();
                    a.trans_fwd_high.add(high as f64);
                }
            }
            if e < TRANS_RECOVERY_TICKS {
                for &i in &outfield {
                    let v = v2(players[i].vel);
                    let sp = (v.0 * v.0 + v.1 * v.1).sqrt();
                    let sprint = u64::from(sp > TRANS_SPRINT_MPS);
                    match players[i].role {
                        PlayerRole::Forward => {
                            a.trans_fwd_speed.add(sp);
                            a.trans_fwd_sprint += sprint;
                        }
                        PlayerRole::Midfielder => {
                            a.trans_mid_speed.add(sp);
                            a.trans_mid_sprint += sprint;
                        }
                        _ => {}
                    }
                }
            }
            if e >= TRANS_CAP_TICKS {
                let cap = f64::from(TRANS_CAP_TICKS) / f64::from(TICKS_PER_SECOND);
                if !ep.shape_done {
                    a.trans_shape.add(cap);
                    a.trans_shape_capped += 1;
                }
                if !ep.static_done {
                    a.trans_shape_static.add(cap);
                    a.trans_shape_static_capped += 1;
                }
                a.trans_ep = None;
            } else {
                a.trans_ep = Some(ep);
            }
        }

        // ---------------- Round 4: attacking support ----------------
        for side in [TeamSide::Home, TeamSide::Away] {
            let s = side.index();
            let sign = side.attack_sign(swapped) as f64;
            let opp = side.opponent();
            let players = &st.players[s];
            let a = &mut acc[s];
            if a.run_state.len() != players.len() {
                a.run_state = vec![0; players.len()];
            }
            let own_carrier = carrier.filter(|&(cs, _)| cs == side).map(|(_, ci)| ci);
            // Run abandon (round 4): a committed run (sim run role) that ends, or is replaced,
            // within RUN_ABANDON_TICKS of its start while the opponent does not own the ball and
            // the runner neither owns the ball nor is the receiver of our pass in flight.
            {
                let mems = st.ai_memory.borrow();
                let m = &mems[s];
                if a.run_prev.len() != players.len() {
                    a.run_prev = vec![(0, 0); players.len()];
                }
                // A loose ball with no pass of ours in flight counts as possession lost.
                let opp_owns = carrier.is_some_and(|(cs, _)| cs != side)
                    || (carrier.is_none() && st.ball.pass_in_flight.as_ref().is_none_or(|pf| pf.team != side));
                let pass_to = st.ball.pass_in_flight.as_ref().filter(|pf| pf.team == side).map(|pf| pf.intended_receiver_idx);
                for j in 0..players.len() {
                    let now = (m.run_role.get(j).copied().unwrap_or(0), m.run_since.get(j).copied().unwrap_or(0));
                    let (prole, psince) = a.run_prev[j];
                    if prole != 0 && now != (prole, psince) {
                        let age = st.tick.wrapping_add(1).wrapping_sub(psince);
                        let received = own_carrier == Some(j) || pass_to == Some(j);
                        if age < RUN_ABANDON_TICKS && !opp_owns && !received {
                            a.sup_run_abandon += 1;
                        }
                    }
                    a.run_prev[j] = now;
                }
            }
            let pass_to = st.ball.pass_in_flight.as_ref().filter(|pf| pf.team == side).map(|pf| pf.intended_receiver_idx);
            // Releases (work area 3): a CPU pass decision selecting a committed runner, or a pass
            // launched toward a penetrate / called runner that no CPU release preceded.
            {
                let mems = st.ai_memory.borrow();
                let m = &mems[s];
                if m.run_release != 0 && m.run_release_tick != a.rel_last_stamp {
                    a.rel_last_stamp = m.run_release_tick;
                    a.rel_last_cpu = Some((m.run_release as usize, st.tick));
                    a.sup_releases += 1;
                }
                if let Some(pf) = st.ball.pass_in_flight.as_ref().filter(|pf| pf.team == side && a.prev_pass_to != Some(pf.intended_receiver_idx)) {
                    let r = pf.intended_receiver_idx;
                    let role = m.run_role.get(r).copied().unwrap_or(0);
                    let cpu_released = a.rel_last_cpu.is_some_and(|(ri, t)| ri == r && st.tick.saturating_sub(t) <= 60);
                    // A ground or through pass (recent_kick_kind 1 / 2): a lofted switch that
                    // happens to find a committed runner does not release a held run.
                    let kind = players.get(pf.passer_idx).map_or(0, |k| k.recent_kick_kind);
                    if cpu_released || ((role == 1 || role == 5) && (kind == 1 || kind == 2)) {
                        if !cpu_released {
                            a.sup_releases += 1;
                        }
                        a.rel_pending = Some(r);
                    }
                }
            }
            if let Some(r) = a.rel_pending {
                if let Some((cs, ci)) = carrier {
                    a.rel_passes += 1;
                    a.rel_completed += u64::from(cs == side && ci == r);
                    a.rel_pending = None;
                }
            }
            // Runs in behind: while this side owns the ball (any player, GK included), and while
            // our pass is in flight for its intended receiver, whose reception tick also counts.
            match (own_carrier.is_some() || pass_to.is_some(), second_last_proj(st, opp, sign)) {
                (true, Some(line)) => {
                    for (j, p) in players.iter().enumerate() {
                        if j == 0 || p.sent_off {
                            a.run_state[j] = 0;
                            continue;
                        }
                        if own_carrier.is_none() && pass_to != Some(j) {
                            continue;
                        }
                        let proj = f(p.pos.x) * sign;
                        let v = v2(p.vel);
                        let sp = (v.0 * v.0 + v.1 * v.1).sqrt();
                        if own_carrier == Some(j) {
                            // Reception tick of the pass he was running onto.
                            if a.prev_pass_to == Some(j) && a.run_state[j] == 1 && proj > line && sp > RUN_BEHIND_MPS {
                                a.sup_runs += 1;
                            }
                            a.run_state[j] = 0;
                            continue;
                        }
                        match a.run_state[j] {
                            0 if proj <= line => a.run_state[j] = 1,
                            1 if proj > line => {
                                if sp > RUN_BEHIND_MPS {
                                    a.sup_runs += 1;
                                }
                                a.run_state[j] = 2;
                            }
                            2 if proj < line - RUN_REARM_M => a.run_state[j] = 1,
                            _ => {}
                        }
                    }
                }
                _ => a.run_state.iter_mut().for_each(|r| *r = 0),
            }
            a.prev_pass_to = pass_to;
            let Some(ci) = own_carrier.filter(|&ci| ci != 0) else {
                continue;
            };
            let cpos = v2(players[ci].pos);
            let mut mask = 0_u32;
            let mut n_open = 0_u32;
            let mut n_fwd = 0_u32;
            let mut nearest = f64::INFINITY;
            for (j, p) in players.iter().enumerate() {
                if j == ci || p.sent_off {
                    continue;
                }
                let q = v2(p.pos);
                if !lane_open(st, opp, cpos, q) || offside_position(st, side, j, sign) {
                    continue;
                }
                mask |= 1 << j;
                let d = dist(cpos, q);
                if j == 0 || !(SUPPORT_MIN_M..=SUPPORT_MAX_M).contains(&d) {
                    continue;
                }
                n_open += 1;
                nearest = nearest.min(d);
                if (q.0 - cpos.0) * sign >= FORWARD_OPTION_M {
                    n_fwd += 1;
                }
            }
            a.sup_ticks += 1;
            a.sup_open.push(n_open as f32);
            a.sup_le1 += u64::from(n_open <= 1);
            a.sup_fwd.add(f64::from(n_fwd));
            if n_open > 0 {
                a.sup_nearest.add(nearest);
            }
            a.last_carrier = Some((ci, st.tick, mask));
            let near: Vec<(f64, f64)> = players
                .iter()
                .enumerate()
                .filter(|&(j, p)| j != 0 && j != ci && !p.sent_off)
                .map(|(_, p)| v2(p.pos))
                .filter(|&q| dist(q, cpos) <= CLUSTER_CARRIER_M)
                .collect();
            let clustered = (0..near.len())
                .any(|x| ((x + 1)..near.len()).any(|y| dist(near[x], near[y]) <= CLUSTER_PAIR_M));
            a.sup_cluster += u64::from(clustered);
            let (mut x0, mut x1, mut y0, mut y1) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
            for p in players.iter().skip(1).filter(|p| !p.sent_off) {
                let q = v2(p.pos);
                x0 = x0.min(q.0);
                x1 = x1.max(q.0);
                y0 = y0.min(q.1);
                y1 = y1.max(q.1);
            }
            a.sup_width.add(y1 - y0);
            a.sup_depth.add(x1 - x0);
        }

        // ---------------- Collisions (all on-pitch pairs) ----------------
        {
            let pos_of = |k: usize| -> Option<(f64, f64)> {
                let p = st.players[k / per_side].get(k % per_side)?;
                (!p.sent_off).then(|| v2(p.pos))
            };
            let mut tick_hit = [false; 4];
            let mut tick_deep = [false; 4];
            for a in 0..n_flat {
                let pa = pos_of(a);
                for b in (a + 1)..n_flat {
                    let idx = a * n_flat + b;
                    let cls = pair_class(a, b);
                    let d = match (pa, pos_of(b)) {
                        (Some(x), Some(y)) => ((x.0 - y.0).powi(2) + (x.1 - y.1).powi(2)).sqrt(),
                        _ => f64::INFINITY,
                    };
                    if d < OVERLAP_DIST_M {
                        let depth = OVERLAP_DIST_M - d;
                        for c in [COLL_ANY, cls] {
                            let ca = &mut coll[c];
                            if pair_run[idx] == 0 {
                                ca.events += 1;
                            }
                            ca.depth.add(depth);
                            ca.depth_max = ca.depth_max.max(depth);
                            tick_hit[c] = true;
                            tick_deep[c] |= depth > DEEP_OVERLAP_M;
                        }
                        pair_run[idx] += 1;
                    } else {
                        let run = std::mem::take(&mut pair_run[idx]);
                        if run > 0 {
                            coll[COLL_ANY].close_run(run);
                            coll[cls].close_run(run);
                        }
                    }
                }
            }
            for (c, hit) in tick_hit.iter().enumerate() {
                if *hit {
                    coll[c].ticks += 1;
                }
                if tick_deep[c] {
                    coll[c].deep_ticks += 1;
                }
            }
        }

        // ---------------- Coordination: chasers, engagers, cruise ----------------
        for side in [TeamSide::Home, TeamSide::Away] {
            let s = side.index();
            let players = &st.players[s];
            let a = &mut acc[s];
            let dist_ball = |p: &psfc_sim::PlayerState| {
                let q = v2(p.pos);
                ((q.0 - ball.0).powi(2) + (q.1 - ball.1).powi(2)).sqrt()
            };
            if carrier.is_none() {
                a.loose_ticks += 1;
                let chasers = players
                    .iter()
                    .filter(|p| !p.sent_off)
                    .filter(|p| {
                        let d = dist_ball(p);
                        if !(1e-6..=CHASE_RADIUS_M).contains(&d) {
                            return false;
                        }
                        let q = v2(p.pos);
                        let v = v2(p.vel);
                        let closing = (v.0 * (ball.0 - q.0) + v.1 * (ball.1 - q.1)) / d;
                        closing > CHASE_CLOSING_MPS
                    })
                    .count() as u64;
                if chasers >= 2 {
                    a.dual_chaser_ticks += 1;
                }
                a.max_chasers = a.max_chasers.max(chasers);
            }
            if let Some((cs, ci)) = carrier {
                if cs != side {
                    let cpos = v2(st.players[cs.index()][ci].pos);
                    let near = players
                        .iter()
                        .skip(1)
                        .filter(|p| !p.sent_off)
                        .filter(|p| {
                            let q = v2(p.pos);
                            ((q.0 - cpos.0).powi(2) + (q.1 - cpos.1).powi(2)).sqrt()
                                <= ENGAGE_RADIUS_M
                        })
                        .count();
                    if near >= 3 {
                        a.engager3 += 1;
                    }
                }
            }
            // Cruise compliance: outfielders not carrying, > 8 m from the ball and not among the
            // side's two outfielders closest to the ball.
            let mut ranked: Vec<(f64, usize)> = (1..players.len())
                .filter(|&i| !players[i].sent_off)
                .map(|i| (dist_ball(&players[i]), i))
                .collect();
            ranked.sort_by(|x, y| x.0.total_cmp(&y.0).then(x.1.cmp(&y.1)));
            for (rank, &(d, i)) in ranked.iter().enumerate() {
                if rank < 2 || d <= CRUISE_BALL_RADIUS_M || carrier == Some((side, i)) {
                    continue;
                }
                let p = &players[i];
                let v = v2(p.vel);
                a.cruise_n += 1;
                if (v.0 * v.0 + v.1 * v.1).sqrt()
                    > CRUISE_FRACTION * approx_top_speed(p.attributes.pace, five)
                {
                    a.cruise_over += 1;
                }
            }
        }

        for side in [TeamSide::Home, TeamSide::Away] {
            let s = side.index();
            let sign = side.attack_sign(swapped) as f64;
            let players = &st.players[s];
            let a = &mut acc[s];
            a.open_ticks += 1;

            // ---------------- Goalkeeper ----------------
            let gk = &players[0];
            let gpos = v2(gk.pos);
            let (in6, inbox, depth) = box_flags(st, side, gpos);
            if !in6 {
                a.gk_out6 += 1;
            }
            if !inbox {
                a.gk_outbox += 1;
            }
            a.gk_line_dist.add(depth);
            a.gk_line_dist_max = a.gk_line_dist_max.max(depth);
            if a.gk_prev_in6 == Some(true) && !in6 {
                let gv = v2(gk.vel);
                let to_ball = (ball.0 - gpos.0, ball.1 - gpos.1);
                if gv.0 * to_ball.0 + gv.1 * to_ball.1 > 0.0 {
                    a.gk_rush += 1;
                }
            }
            a.gk_prev_in6 = Some(in6);
            if in6 {
                if a.gk_excursion_len > 0 {
                    close_excursion(a);
                }
            } else {
                a.gk_excursion_len += 1;
            }
            let gk_owns = st.ball.owner == Some((side, 0));
            if gk_owns && !prev_gk_owner[s] {
                a.gk_gathers += 1;
            }
            prev_gk_owner[s] = gk_owns;

            // ---------------- Shape ----------------
            let Some(pt) = poss_team else { continue };
            let ph = usize::from(pt != side);
            let outfield: Vec<usize> = (1..players.len())
                .filter(|&i| !players[i].sent_off)
                .collect();
            let pa = &mut a.phase[ph];
            pa.ticks += 1;
            if !outfield.is_empty() {
                // Anchor distance (static formation slot, forward shift 0).
                let mut offs = Vec::with_capacity(players.len());
                for (i, p) in players.iter().enumerate() {
                    if p.sent_off {
                        continue;
                    }
                    let anc = v2(psfc_sim::ai::world_anchor_for_player(st, side, i, Fp::ZERO));
                    let pos = v2(p.pos);
                    let d = ((pos.0 - anc.0).powi(2) + (pos.1 - anc.1).powi(2)).sqrt();
                    pa.anchor_dist[role_index(p.role)].add(d);
                    offs.push((i, pos.0 - anc.0, pos.1 - anc.1));
                }
                // Shape-corrected: remove the outfield block's mean x offset (team pushing up or
                // dropping off as a unit is not counted as losing shape).
                let out_offs: Vec<_> = offs.iter().filter(|o| o.0 != 0).collect();
                let mean_dx =
                    out_offs.iter().map(|o| o.1).sum::<f64>() / out_offs.len().max(1) as f64;
                for o in &out_offs {
                    let d = ((o.1 - mean_dx).powi(2) + o.2.powi(2)).sqrt();
                    pa.anchor_dist_shape[role_index(players[o.0].role)].add(d);
                }

                // Bounding box of outfielders.
                let xs: Vec<f64> = outfield.iter().map(|&i| f(players[i].pos.x)).collect();
                let ys: Vec<f64> = outfield.iter().map(|&i| f(players[i].pos.y)).collect();
                let depth_m = xs.iter().cloned().fold(f64::MIN, f64::max)
                    - xs.iter().cloned().fold(f64::MAX, f64::min);
                let width_m = ys.iter().cloned().fold(f64::MIN, f64::max)
                    - ys.iter().cloned().fold(f64::MAX, f64::min);
                pa.bbox_area.add(depth_m * width_m);
                pa.bbox_depth.add(depth_m);
                pa.bbox_width.add(width_m);
                pa.length.add(depth_m);
                pa.width.add(width_m);
                let n_out = outfield.len() as f64;
                let cx = xs.iter().sum::<f64>() / n_out;
                let cy = ys.iter().sum::<f64>() / n_out;
                pa.stretch.add(
                    xs.iter()
                        .zip(&ys)
                        .map(|(x, y)| ((x - cx).powi(2) + (y - cy).powi(2)).sqrt())
                        .sum::<f64>()
                        / n_out,
                );
                if outfield.len() >= 2 {
                    for k in 0..outfield.len() {
                        let nearest = (0..outfield.len())
                            .filter(|&j| j != k)
                            .map(|j| ((xs[k] - xs[j]).powi(2) + (ys[k] - ys[j]).powi(2)).sqrt())
                            .fold(f64::INFINITY, f64::min);
                        pa.nearest_mate.push(nearest as f32);
                    }
                }

                if ph == 1 {
                    // Defensive line: role Defender; fall back to the two deepest base anchors.
                    let mut line: Vec<usize> = outfield
                        .iter()
                        .copied()
                        .filter(|&i| players[i].role == PlayerRole::Defender)
                        .collect();
                    if line.len() < 2 {
                        let mut by_depth = outfield.clone();
                        by_depth.sort_by_key(|&i| (players[i].base_anchor_permille.0, i));
                        line = by_depth.into_iter().take(2).collect();
                    }
                    if line.len() >= 2 {
                        let lx: Vec<f64> = line.iter().map(|&i| f(players[i].pos.x)).collect();
                        let m = lx.iter().sum::<f64>() / lx.len() as f64;
                        let var = lx.iter().map(|x| (x - m).powi(2)).sum::<f64>() / lx.len() as f64;
                        pa.line_var.add(var);
                        pa.line_std.add(var.sqrt());
                    }
                    // Behind the ball = strictly between ball and own goal along x.
                    let behind = outfield
                        .iter()
                        .filter(|&&i| (f(players[i].pos.x) - ball.0) * sign < 0.0)
                        .count();
                    pa.behind_ball.add(behind as f64 / outfield.len() as f64);
                    // Swarming around the opposing carrier.
                    if let Some((cs, ci)) = carrier {
                        if cs != side {
                            let cpos = v2(st.players[cs.index()][ci].pos);
                            let near = outfield
                                .iter()
                                .filter(|&&i| {
                                    let p = v2(players[i].pos);
                                    ((p.0 - cpos.0).powi(2) + (p.1 - cpos.1).powi(2)).sqrt()
                                        <= SWARM_RADIUS
                                })
                                .count();
                            pa.carrier_ticks += 1;
                            if near >= 2 {
                                pa.swarm2 += 1;
                            }
                            if near >= 3 {
                                pa.swarm3 += 1;
                            }
                        }
                    }
                }
            }
        }

        // ---------------- Movement + aerial (per player, consecutive open-play ticks) ----------------
        for side in [TeamSide::Home, TeamSide::Away] {
            let s = side.index();
            let players = &st.players[s];
            if prev[s].len() != players.len() {
                prev[s] = vec![None; players.len()];
            }
            let a = &mut acc[s];
            let mut airborne = 0_u64;
            for (i, p) in players.iter().enumerate() {
                if p.sent_off {
                    prev[s][i] = None;
                    continue;
                }
                let vel = v2(p.vel);
                let pos = v2(p.pos);
                let z = f(p.z);
                let mut pos_vel = None;
                let mut dspeed = None;
                let mut lat = None;
                let mut theta = None;
                let mut lat_stage = [None; 3];
                let (mut sch_side, mut sch_cand, mut sch_run) = (0_i8, 0_i8, 0_u8);
                if i != 0 && z > AIRBORNE_Z {
                    airborne += 1;
                }
                if let Some(pp) = prev[s][i].as_ref().filter(|pp| pp.id == p.footballer_id) {
                    a.player_ticks += 1;
                    let sp0 = (pp.vel.0.powi(2) + pp.vel.1.powi(2)).sqrt();
                    let sp1 = (vel.0.powi(2) + vel.1.powi(2)).sqrt();
                    let class = usize::from(psfc_sim::ai::move_priority(st, side, i).min(3));
                    let band = usize::from(sp1 >= SPEED_BAND_MPS);
                    a.ticks_class[class] += 1;
                    a.ticks_band[band] += 1;
                    let dv = ((vel.0 - pp.vel.0).powi(2) + (vel.1 - pp.vel.1).powi(2)).sqrt();
                    let acc_mps2 = dv * f64::from(TICKS_PER_SECOND);
                    a.accel.add(acc_mps2);
                    if acc_mps2 > 10.0 {
                        a.accel_gt10 += 1;
                    }
                    let hz = f64::from(TICKS_PER_SECOND);
                    let pv = ((pos.0 - pp.pos.0) * hz, (pos.1 - pp.pos.1) * hz);
                    // Position-correction velocity: displacement not explained by the published vel.
                    a.corr_vel
                        .push(((pv.0 - vel.0).powi(2) + (pv.1 - vel.1).powi(2)).sqrt() as f32);
                    if let Some(ppv) = pp.pos_vel {
                        a.accel_pos
                            .add(((pv.0 - ppv.0).powi(2) + (pv.1 - ppv.1).powi(2)).sqrt() * hz);
                    }
                    pos_vel = Some(pv);
                    a.max_dspeed = a.max_dspeed.max((sp1 - sp0).abs());
                    let ds = sp1 - sp0;
                    if let Some(pds) = pp.dspeed {
                        if pds.abs() > 0.05 && ds.abs() > 0.05 {
                            a.speed_osc_n += 1;
                            if pds.signum() != ds.signum() {
                                a.speed_osc += 1;
                            }
                        }
                    }
                    dspeed = Some(ds);
                    if sp0 > MOVING_SPEED && sp1 > MOVING_SPEED {
                        let dot = pp.vel.0 * vel.0 + pp.vel.1 * vel.1;
                        let cross = pp.vel.0 * vel.1 - pp.vel.1 * vel.0;
                        let deg = cross.atan2(dot).abs().to_degrees();
                        if sp0 > HEADING_P95_MIN_SPEED && sp1 > HEADING_P95_MIN_SPEED {
                            a.heading_gt3.push(deg as f32);
                        }
                        // Side flip: the signed lateral component flips versus the previous sample.
                        let l = cross / sp0;
                        // Steering-stage laterals versus the same reference (previous velocity).
                        let dbg = p.steer_debug;
                        if dbg.tick == st.tick {
                            for (k, v) in [dbg.desired, dbg.selected, dbg.filtered]
                                .into_iter()
                                .enumerate()
                            {
                                let v = v2(v);
                                lat_stage[k] = Some((pp.vel.0 * v.1 - pp.vel.1 * v.0) / sp0);
                            }
                        }
                        let flipped = |a: Option<f64>, b: Option<f64>| match (a, b) {
                            (Some(a), Some(b)) => {
                                a.abs() > SIDE_FLIP_EPS
                                    && b.abs() > SIDE_FLIP_EPS
                                    && a.signum() != b.signum()
                            }
                            _ => false,
                        };
                        if flipped(pp.lat, Some(l)) {
                            a.side_flips += 1;
                            a.flips_class[class] += 1;
                            a.flips_band[band] += 1;
                            // Cause: the earliest steering stage whose own lateral flipped too;
                            // none of them (or no fresh diagnostics) = the motion limits/contact.
                            let cause = (0..3)
                                .find(|&k| flipped(pp.lat_stage[k], lat_stage[k]))
                                .unwrap_or(3);
                            a.flips_cause[cause] += 1;
                        }
                        lat = Some(l);
                        // Schmitt variant: the debounced side changes only after the lateral has
                        // stayed beyond the opposite +-0.05 m/s band for 4 consecutive samples.
                        sch_side = pp.sch_side;
                        let sgn = if l > SIDE_FLIP_SCHMITT_EPS {
                            1
                        } else if l < -SIDE_FLIP_SCHMITT_EPS {
                            -1
                        } else {
                            0
                        };
                        // Any sample not beyond the opposite band breaks the run.
                        if sgn != 0 && sgn != sch_side {
                            sch_run = if pp.sch_cand == sgn { pp.sch_run.saturating_add(1) } else { 1 };
                            sch_cand = sgn;
                            if sch_run >= SIDE_FLIP_SCHMITT_DWELL {
                                if sch_side != 0 {
                                    a.flips_schmitt += 1;
                                }
                                sch_side = sgn;
                                sch_cand = 0;
                                sch_run = 0;
                            }
                        }
                        let th = cross.atan2(dot).to_degrees();
                        if let Some(pth) = pp.theta {
                            if pth.abs() > HEADING_FLIP_MIN_DEG
                                && th.abs() > HEADING_FLIP_MIN_DEG
                                && pth.signum() != th.signum()
                            {
                                a.heading_flips += 1;
                            }
                        }
                        theta = Some(th);
                        a.turn_rate.add(deg * f64::from(TICKS_PER_SECOND));
                        // Component of the velocity change perpendicular to the previous heading.
                        a.lateral_accel
                            .add(cross.abs() / sp0 * f64::from(TICKS_PER_SECOND));
                        let b = if deg < 10.0 {
                            0
                        } else if deg < 45.0 {
                            1
                        } else if deg < 90.0 {
                            2
                        } else {
                            3
                        };
                        a.heading_hist[b] += 1;
                        if dot < 0.0 {
                            a.reversals += 1;
                        }
                    }
                    if pp.z <= AIRBORNE_Z && z > AIRBORNE_Z {
                        if i == 0 {
                            a.jumps_gk += 1;
                        } else {
                            a.jumps_outfield += 1;
                        }
                    }
                    if i != 0 && z > HEADER_MIN_Z && p.recent_kick_ticks > pp.recent_kick_ticks {
                        a.headers += 1;
                        match p.recent_kick_kind {
                            4 | 5 => a.headers_shot += 1,
                            3 => a.headers_clear += 1,
                            _ => a.headers_pass += 1,
                        }
                    }
                }
                prev[s][i] = Some(PrevPlayer {
                    id: p.footballer_id.clone(),
                    vel,
                    pos,
                    pos_vel,
                    dspeed,
                    z,
                    recent_kick_ticks: p.recent_kick_ticks,
                    lat,
                    theta,
                    lat_stage,
                    sch_side,
                    sch_cand,
                    sch_run,
                });
            }
            a.max_airborne = a.max_airborne.max(airborne);
        }
    }

    for a in 0..n_flat {
        for b in (a + 1)..n_flat {
            let run = pair_run[a * n_flat + b];
            if run > 0 {
                coll[COLL_ANY].close_run(run);
                coll[pair_class(a, b)].close_run(run);
            }
        }
    }

    let st = sim.state();
    let final_hash_hex = format!("{:016x}", sim.state_hash());
    let total_poss = (st.possession_ticks[0] + st.possession_ticks[1]).max(1) as f64;

    let mut sides: [Metrics; 2] = [Metrics::new(), Metrics::new()];
    for side in [TeamSide::Home, TeamSide::Away] {
        let s = side.index();
        let a = &acc[s];
        let m = &mut sides[s];
        let stats = st.combined_player_stats(side);
        let sum = |g: fn(&psfc_sim::PlayerMatchStats) -> u16| {
            stats.iter().map(|p| f64::from(g(p))).sum::<f64>()
        };
        let open_min = a.open_ticks as f64 / TICKS_PER_MIN;
        let per10 = |c: u64| {
            if open_min > 0.0 {
                c as f64 * 10.0 / open_min
            } else {
                0.0
            }
        };
        let pct = |c: u64, n: u64| {
            if n == 0 {
                0.0
            } else {
                100.0 * c as f64 / n as f64
            }
        };

        m.insert("match.goals".into(), f64::from(st.score[s]));
        m.insert("match.shots".into(), sum(|p| p.shots));
        m.insert("match.shots_on_target".into(), sum(|p| p.shots_on_target));
        m.insert(
            "match.possession_pct".into(),
            100.0 * st.possession_ticks[s] as f64 / total_poss,
        );
        m.insert("match.offsides".into(), a.offsides as f64);
        m.insert("match.fouls".into(), sum(|p| p.fouls_committed));
        m.insert("match.passes_completed_pct".into(), {
            let att = sum(|p| p.passes_attempted);
            if att > 0.0 {
                100.0 * sum(|p| p.passes_completed) / att
            } else {
                0.0
            }
        });

        m.insert("gk.pct_outside_6yd".into(), pct(a.gk_out6, a.open_ticks));
        m.insert("gk.pct_outside_box".into(), pct(a.gk_outbox, a.open_ticks));
        m.insert("gk.rush_per_10min".into(), per10(a.gk_rush));
        m.insert("gk.rush_count".into(), a.gk_rush as f64);
        m.insert("gk.excursion_mean_s".into(), a.gk_excursions.get());
        m.insert(
            "gk.excursions_gt1s_per_10min".into(),
            per10(a.gk_excursions_gt1s),
        );
        m.insert("gk.line_dist_mean_m".into(), a.gk_line_dist.get());
        m.insert("gk.line_dist_max_m".into(), a.gk_line_dist_max);
        let gk_id = &st.players[s][0].footballer_id;
        m.insert(
            "gk.saves".into(),
            stats
                .iter()
                .filter(|p| &p.footballer_id == gk_id)
                .map(|p| f64::from(p.saves))
                .sum(),
        );
        m.insert("gk.catches".into(), a.gk_catches as f64);
        m.insert("gk.parries".into(), a.gk_parries as f64);
        m.insert("gk.tips_over".into(), a.gk_tips as f64);
        m.insert("gk.gathers".into(), a.gk_gathers as f64);
        m.insert(
            "gk.claims_non_save".into(),
            a.gk_gathers.saturating_sub(a.gk_catches) as f64,
        );
        m.insert("gk.goals_conceded".into(), a.goals_conceded as f64);
        m.insert(
            "gk.goals_conceded_gk_gt3m_off_line".into(),
            a.goals_conceded_gk_off_line as f64,
        );
        m.insert(
            "gk.goals_conceded_no_shot_found".into(),
            a.goals_conceded_no_shot as f64,
        );
        m.insert(
            "gk.line_dist_at_conceded_shot_mean_m".into(),
            a.gk_line_dist_at_conceded_shot.get(),
        );
        if probe.key.is_some() {
            m.insert(
                "gk.scenario_switches_per_10min".into(),
                per10(a.gk_scenario_switches),
            );
            m.insert(
                "gk.scenario_distinct_values".into(),
                a.gk_scenario_ticks.len() as f64,
            );
        }

        for (ph, ph_name) in [(0, "attacking"), (1, "defending")] {
            let pa = &a.phase[ph];
            m.insert(
                format!("shape.{ph_name}.pct_of_open_play"),
                pct(pa.ticks, a.open_ticks),
            );
            for (ri, (rname, _)) in ROLES.iter().enumerate() {
                if pa.anchor_dist[ri].n > 0 {
                    m.insert(
                        format!("shape.{ph_name}.anchor_dist_m.{rname}"),
                        pa.anchor_dist[ri].get(),
                    );
                }
                if pa.anchor_dist_shape[ri].n > 0 {
                    m.insert(
                        format!("shape.{ph_name}.anchor_dist_shape_m.{rname}"),
                        pa.anchor_dist_shape[ri].get(),
                    );
                }
            }
            m.insert(format!("shape.{ph_name}.bbox_area_m2"), pa.bbox_area.get());
            m.insert(format!("shape.{ph_name}.bbox_depth_m"), pa.bbox_depth.get());
            m.insert(format!("shape.{ph_name}.bbox_width_m"), pa.bbox_width.get());
        }
        let d = &a.phase[1];
        m.insert(
            "shape.defending.line_x_variance_m2".into(),
            d.line_var.get(),
        );
        m.insert("shape.defending.line_x_std_m".into(), d.line_std.get());
        m.insert(
            "shape.defending.pct_outfield_behind_ball".into(),
            100.0 * d.behind_ball.get(),
        );
        m.insert(
            "shape.defending.swarm2_pct".into(),
            pct(d.swarm2, d.carrier_ticks),
        );
        m.insert(
            "shape.defending.swarm3_pct".into(),
            pct(d.swarm3, d.carrier_ticks),
        );

        let hn: u64 = a.heading_hist.iter().sum();
        for (b, name) in ["0_10", "10_45", "45_90", "gt90"].iter().enumerate() {
            m.insert(
                format!("move.heading_change_pct.{name}"),
                pct(a.heading_hist[b], hn),
            );
        }
        let pmin = a.player_ticks as f64 / TICKS_PER_MIN;
        m.insert(
            "move.reversals_per_player_min".into(),
            if pmin > 0.0 {
                a.reversals as f64 / pmin
            } else {
                0.0
            },
        );
        m.insert("move.accel_mean_mps2".into(), a.accel.get());
        m.insert("move.turn_rate_mean_deg_per_s".into(), a.turn_rate.get());
        m.insert("move.lateral_accel_mean_mps2".into(), a.lateral_accel.get());
        m.insert(
            "move.accel_gt10_pct".into(),
            pct(a.accel_gt10, a.player_ticks),
        );
        m.insert(
            "move.speed_oscillation_pct".into(),
            pct(a.speed_osc, a.speed_osc_n),
        );
        m.insert("move.accel_from_pos_mean_mps2".into(), a.accel_pos.get());
        m.insert("move.dspeed_max_mps_per_tick".into(), a.max_dspeed);

        // ---------------- Round 3: collisions and coordination ----------------
        let per_min = |c: u64| {
            if open_min > 0.0 {
                c as f64 / open_min
            } else {
                0.0
            }
        };
        for (name, ci) in [
            ("any", COLL_ANY),
            ("teammate", 1 + s),
            ("opponent", COLL_OPP),
        ] {
            let c = &coll[ci];
            m.insert(format!("collision.{name}.ticks_per_min"), per_min(c.ticks));
            m.insert(
                format!("collision.{name}.ticks_gt1cm_per_min"),
                per_min(c.deep_ticks),
            );
            m.insert(
                format!("collision.{name}.events_per_min"),
                per_min(c.events),
            );
            m.insert(format!("collision.{name}.depth_mean_m"), c.depth.get());
            m.insert(format!("collision.{name}.depth_max_m"), c.depth_max);
            m.insert(format!("collision.{name}.stuck_pairs"), c.stuck as f64);
            m.insert(
                format!("collision.{name}.stuck_mean_ticks"),
                c.stuck_dur.get(),
            );
        }
        let mut cv = a.corr_vel.clone();
        let over = cv
            .iter()
            .filter(|&&v| f64::from(v) > CORRECTION_VEL_GATE)
            .count() as u64;
        m.insert(
            "collision.correction_vel_pct_gt_0_3".into(),
            pct(over, cv.len() as u64),
        );
        m.insert(
            "collision.correction_vel_p50_mps".into(),
            percentile(&mut cv, 0.50),
        );
        m.insert(
            "collision.correction_vel_p95_mps".into(),
            percentile(&mut cv, 0.95),
        );
        m.insert(
            "collision.correction_vel_p99_mps".into(),
            percentile(&mut cv, 0.99),
        );
        // Conditional on a correction actually happening (contact is rare, so the unconditional
        // percentiles are dominated by exact-zero samples).
        let mut corrected: Vec<f32> = cv
            .iter()
            .copied()
            .filter(|&v| f64::from(v) > CORRECTED_MIN_MPS)
            .collect();
        m.insert(
            "collision.correction_vel_corrected_pct".into(),
            pct(corrected.len() as u64, cv.len() as u64),
        );
        m.insert(
            "collision.correction_vel_corrected_p50_mps".into(),
            percentile(&mut corrected, 0.50),
        );
        m.insert(
            "collision.correction_vel_corrected_p99_mps".into(),
            percentile(&mut corrected, 0.99),
        );
        m.insert(
            "collision.correction_vel_max_mps".into(),
            cv.iter().fold(0.0_f64, |m, &v| m.max(f64::from(v))),
        );

        m.insert(
            "coord.dual_chaser_ticks_per_min".into(),
            per_min(a.dual_chaser_ticks),
        );
        m.insert(
            "coord.dual_chaser_pct_of_loose".into(),
            pct(a.dual_chaser_ticks, a.loose_ticks),
        );
        m.insert(
            "coord.max_simultaneous_chasers".into(),
            a.max_chasers as f64,
        );
        m.insert("coord.engagers3_ticks_per_min".into(), per_min(a.engager3));
        m.insert(
            "coord.engagers3_pct_of_carrier_ticks".into(),
            pct(a.engager3, a.phase[1].carrier_ticks),
        );
        m.insert(
            "coord.cruise_over72_pct".into(),
            pct(a.cruise_over, a.cruise_n),
        );

        m.insert(
            "move.side_flips_per_player_min".into(),
            if pmin > 0.0 {
                a.side_flips as f64 / pmin
            } else {
                0.0
            },
        );
        let mut hg = a.heading_gt3.clone();
        m.insert(
            "move.heading_change_p95_deg_per_tick_gt3mps".into(),
            percentile(&mut hg, 0.95),
        );

        for (ph, ph_name) in [(0, "attacking"), (1, "defending")] {
            let pa = &a.phase[ph];
            m.insert(format!("shape.{ph_name}.length_m"), pa.length.get());
            m.insert(format!("shape.{ph_name}.width_m"), pa.width.get());
            m.insert(format!("shape.{ph_name}.stretch_m"), pa.stretch.get());
            let mut nm = pa.nearest_mate.clone();
            let mean = if nm.is_empty() {
                0.0
            } else {
                nm.iter().map(|&v| f64::from(v)).sum::<f64>() / nm.len() as f64
            };
            m.insert(format!("shape.{ph_name}.nearest_mate_mean_m"), mean);
            m.insert(
                format!("shape.{ph_name}.nearest_mate_p10_m"),
                percentile(&mut nm, 0.10),
            );
        }

        m.insert("aerial.jumps_outfield".into(), a.jumps_outfield as f64);
        m.insert("aerial.jumps_gk".into(), a.jumps_gk as f64);
        m.insert("aerial.max_airborne_outfield".into(), a.max_airborne as f64);
        m.insert("aerial.headers".into(), a.headers as f64);
        m.insert("aerial.headers_shot".into(), a.headers_shot as f64);
        m.insert("aerial.headers_clear".into(), a.headers_clear as f64);
        m.insert("aerial.headers_pass".into(), a.headers_pass as f64);

        // ---------------- Round 4 ----------------
        // Conditional means are only emitted when they have samples (mean_maps skips missing keys).
        let put_mean = |m: &mut Metrics, k: &str, v: Mean, scale: f64| {
            if v.n > 0 {
                m.insert(k.into(), v.get() * scale);
            }
        };
        m.insert("trans.count_per_10min".into(), per10(a.trans_count));
        let shape_n = a.trans_shape.n;
        put_mean(m, "trans.time_to_shape_s", a.trans_shape, 1.0);
        if shape_n > 0 {
            m.insert(
                "trans.time_to_shape_capped_pct".into(),
                pct(a.trans_shape_capped, shape_n),
            );
            m.insert(
                "trans.in_shape_at_turnover_pct".into(),
                pct(a.trans_shape_at_t0, shape_n),
            );
        }
        put_mean(m, "trans.time_to_shape_static_s", a.trans_shape_static, 1.0);
        if a.trans_shape_static.n > 0 {
            m.insert(
                "trans.time_to_shape_static_capped_pct".into(),
                pct(a.trans_shape_static_capped, a.trans_shape_static.n),
            );
        }
        if a.trans_count > 0 {
            m.insert(
                "trans.time_to_shape_excluded_pct".into(),
                pct(a.trans_shape_excluded, a.trans_count),
            );
            m.insert(
                "trans.time_to_shape_static_excluded_pct".into(),
                pct(a.trans_shape_static_excluded, a.trans_count),
            );
            m.insert(
                "trans.counterpress_pct".into(),
                pct(a.trans_counterpress, a.trans_count),
            );
        }
        put_mean(
            m,
            "trans.pct_outfield_behind_ball_at_3s",
            a.trans_behind3,
            100.0,
        );
        put_mean(
            m,
            "trans.pct_outfield_behind_ball_at_6s",
            a.trans_behind6,
            100.0,
        );
        if a.trans_fwd_speed.n > 0 {
            m.insert(
                "trans.forward_recovery_speed_mps_0_5s".into(),
                a.trans_fwd_speed.get(),
            );
            m.insert(
                "trans.forward_sprint_share_0_5s".into(),
                pct(a.trans_fwd_sprint, a.trans_fwd_speed.n),
            );
        }
        if a.trans_mid_speed.n > 0 {
            m.insert(
                "trans.mid_recovery_speed_mps_0_5s".into(),
                a.trans_mid_speed.get(),
            );
            m.insert(
                "trans.mid_sprint_share_0_5s".into(),
                pct(a.trans_mid_sprint, a.trans_mid_speed.n),
            );
        }
        put_mean(m, "trans.forwards_staying_high_mean", a.trans_fwd_high, 1.0);

        if a.sup_ticks > 0 {
            let mut so = a.sup_open.clone();
            let mean = so.iter().map(|&v| f64::from(v)).sum::<f64>() / so.len() as f64;
            m.insert("support.open_options_mean".into(), mean);
            m.insert("support.open_options_p10".into(), percentile(&mut so, 0.10));
            m.insert(
                "support.ticks_with_le1_option_pct".into(),
                pct(a.sup_le1, a.sup_ticks),
            );
            m.insert("support.forward_options_mean".into(), a.sup_fwd.get());
            m.insert(
                "support.cluster_pct".into(),
                pct(a.sup_cluster, a.sup_ticks),
            );
            m.insert("support.width_m".into(), a.sup_width.get());
            m.insert("support.depth_m".into(), a.sup_depth.get());
            m.insert(
                "support.carrier_pct_of_open_play".into(),
                pct(a.sup_ticks, a.open_ticks),
            );
        }
        put_mean(m, "support.nearest_option_dist_m", a.sup_nearest, 1.0);
        m.insert("support.runs_in_behind_per_10min".into(), per10(a.sup_runs));
        m.insert("support.run_abandon_per_10min".into(), per10(a.sup_run_abandon));
        m.insert("support.releases_per_10min".into(), per10(a.sup_releases));
        if a.rel_passes > 0 {
            m.insert("support.release_pass_completed_pct".into(), pct(a.rel_completed, a.rel_passes));
        }
        m.insert(
            "support.passes_completed_tracked".into(),
            a.passes_completed as f64,
        );
        if a.passes_completed > 0 {
            m.insert(
                "support.pass_target_open_pct".into(),
                pct(a.passes_open, a.passes_completed),
            );
        }

        let per_pmin = |c: u64, t: u64| {
            let mins = t as f64 / TICKS_PER_MIN;
            if mins > 0.0 {
                c as f64 / mins
            } else {
                0.0
            }
        };
        for c in 0..4 {
            m.insert(
                format!("move.side_flips_by_class.{c}_per_player_min"),
                per_pmin(a.flips_class[c], a.ticks_class[c]),
            );
            m.insert(
                format!("move.side_flips_by_class.{c}_share_pct"),
                pct(a.flips_class[c], a.side_flips),
            );
            m.insert(
                format!("move.class_time_pct.{c}"),
                pct(a.ticks_class[c], a.player_ticks),
            );
        }
        m.insert(
            "move.side_flips_speed_lt3_per_player_min".into(),
            per_pmin(a.flips_band[0], a.ticks_band[0]),
        );
        m.insert(
            "move.side_flips_speed_ge3_per_player_min".into(),
            per_pmin(a.flips_band[1], a.ticks_band[1]),
        );
        m.insert(
            "move.heading_sign_flips_per_player_min".into(),
            per_pmin(a.heading_flips, a.player_ticks),
        );
        m.insert(
            "move.side_flips_schmitt_per_player_min".into(),
            per_pmin(a.flips_schmitt, a.player_ticks),
        );
        for (c, name) in FLIP_CAUSES.iter().enumerate() {
            m.insert(
                format!("move.side_flips_cause.{name}_per_player_min"),
                per_pmin(a.flips_cause[c], a.player_ticks),
            );
        }
    }

    let mut match_metrics = Metrics::new();
    match_metrics.insert("goals_total".into(), f64::from(st.score[0] + st.score[1]));
    match_metrics.insert(
        "shots_total".into(),
        sides[0]["match.shots"] + sides[1]["match.shots"],
    );
    match_metrics.insert("ticks_simulated".into(), f64::from(st.tick));
    match_metrics.insert(
        "open_play_minutes".into(),
        open_ticks_total as f64 / TICKS_PER_MIN,
    );
    match_metrics.insert(
        "open_play_pct_of_ticks".into(),
        100.0 * open_ticks_total as f64 / f64::from(st.tick.max(1)),
    );

    let [home, away] = sides;
    Ok(SeedReport {
        seed,
        final_hash_hex,
        ticks_simulated: st.tick,
        finished: sim.is_finished(),
        score: st.score,
        match_metrics,
        home,
        away,
        gk_scenario_field: probe.key.map(|(in_p, k)| {
            if in_p {
                format!("PlayerState.{k}")
            } else {
                format!("MatchState.{k}")
            }
        }),
        gk_scenario_ticks: [
            acc[0].gk_scenario_ticks.clone(),
            acc[1].gk_scenario_ticks.clone(),
        ],
    })
}

fn mean_maps<'a>(maps: impl Iterator<Item = &'a Metrics>) -> Metrics {
    let mut sum: BTreeMap<String, (f64, u32)> = BTreeMap::new();
    for m in maps {
        for (k, v) in m {
            let e = sum.entry(k.clone()).or_insert((0.0, 0));
            e.0 += v;
            e.1 += 1;
        }
    }
    sum.into_iter()
        .map(|(k, (s, n))| (k, s / f64::from(n)))
        .collect()
}

/// Runs all seeds of `cfg` (in parallel threads; each match is independent and deterministic).
pub fn analyze_config(cfg: &AnalyzeConfig, keep_per_seed: bool) -> Result<ConfigReport, String> {
    let seeds: Vec<u64> = (0..u64::from(cfg.seeds.max(1)))
        .map(|i| cfg.seed_base + i)
        .collect();
    let results: Vec<Result<SeedReport, String>> = std::thread::scope(|scope| {
        let handles: Vec<_> = seeds
            .iter()
            .map(|&seed| scope.spawn(move || analyze_seed(cfg, seed)))
            .collect();
        handles
            .into_iter()
            .map(|h| {
                h.join()
                    .unwrap_or_else(|_| Err("analysis thread panicked".to_string()))
            })
            .collect()
    });
    let per_seed = results.into_iter().collect::<Result<Vec<_>, _>>()?;
    let mean_home = mean_maps(per_seed.iter().map(|r| &r.home));
    let mean_away = mean_maps(per_seed.iter().map(|r| &r.away));
    let mean_teams = mean_maps([&mean_home, &mean_away].into_iter());
    Ok(ConfigReport {
        id: cfg.id(),
        label: cfg.label(),
        command: cfg.command_line(),
        config: cfg.clone(),
        gameplay_version: psfc_sim::GAMEPLAY_VERSION.to_string(),
        final_hashes: per_seed.iter().map(|r| r.final_hash_hex.clone()).collect(),
        mean_match: mean_maps(per_seed.iter().map(|r| &r.match_metrics)),
        mean_home,
        mean_away,
        mean_teams,
        per_seed: if keep_per_seed { per_seed } else { Vec::new() },
    })
}

// ---------------------------------------------------------------------------------------------
// Definitions
// ---------------------------------------------------------------------------------------------

pub fn metric_definitions() -> BTreeMap<String, String> {
    let d: &[(&str, &str)] = &[
        ("general.sampling", "The sim is ticked at 60 Hz with both controllers set to SideController::Cpu(difficulty) and idle external input; MatchState is sampled after every tick. 'Open play' = MatchPhase::OpenPlay. Per-player deltas only use pairs of consecutive open-play ticks with the same footballer in the slot; sent-off players are skipped. Coordinates are metres, x along the pitch, own goal line at x = -attack_sign(second_half_swapped) * pitch_length/2."),
        ("general.phase", "Per side and open-play tick: 'attacking' if ball.owner is that side, else (ball loose) if ball.last_touch_team is that side; 'defending' if the owner / last toucher is the opponent; ticks with no owner and no last touch are excluded from phase metrics."),
        ("general.means", "mean_home / mean_away are means over seeds; mean_teams is the mean of mean_home and mean_away (for *_max keys this is a mean of per-match maxima)."),
        ("match.goals", "Final score for the side."),
        ("match.shots", "Sum of PlayerMatchStats.shots over all players who appeared (starters + subs)."),
        ("match.shots_on_target", "Sum of PlayerMatchStats.shots_on_target (the sim rolls this back on blocks/woodwork misses)."),
        ("match.possession_pct", "100 * possession_ticks[side] / (possession_ticks[home] + possession_ticks[away]) as kept by the sim."),
        ("match.offsides", "Count of MatchEventType::Offside events whose team is this side (the offending side)."),
        ("match.fouls", "Sum of PlayerMatchStats.fouls_committed."),
        ("match.passes_completed_pct", "100 * sum(passes_completed) / sum(passes_attempted)."),
        ("match(meta).goals_total / shots_total", "Both sides combined."),
        ("match(meta).open_play_minutes", "Open-play ticks / 3600."),
        ("match(meta).open_play_pct_of_ticks", "Open-play ticks as % of all simulated ticks (includes kick-offs, set pieces, celebrations, half time)."),
        ("final_hashes", "MatchSimulation::state_hash() after the final tick, one per seed, hex. Identical hashes on a rerun prove determinism; any sim change will change them."),
        ("gk.pct_outside_6yd", "% of open-play ticks where the GK (slot 0) is outside the goal area: depth from own goal line > 5.5 m OR |y| > goal_width/2 + 5.5 m. Same FIFA goal-area geometry for 5v5 and 11v11 (MatchFormat has no goal-area field)."),
        ("gk.pct_outside_box", "% of open-play ticks where the GK is outside the penalty area: depth > penalty_box_depth OR |y| > penalty_box_width/2 (from MatchFormat)."),
        ("gk.rush_count / gk.rush_per_10min", "Rush event = open-play tick where the GK was inside the goal area on the previous open-play tick, is outside it now, and dot(gk.vel, ball.xy - gk.pos) > 0 (moving toward the ball). Per 10 min = count * 10 / open-play minutes."),
        ("gk.excursion_mean_s / gk.excursions_gt1s_per_10min", "Excursion = maximal run of consecutive open-play ticks with the GK outside the goal area (ended by re-entering or by open play stopping; one still open at full time is dropped). Mean duration in seconds, and the rate of excursions longer than 1 s per 10 open-play minutes (filters edge flicker out of rush counts)."),
        ("gk.line_dist_mean_m / gk.line_dist_max_m", "GK distance |gk.x - own_goal_x| in metres over open-play ticks (mean / max)."),
        ("gk.saves", "PlayerMatchStats.saves of the GK slot's footballer."),
        ("gk.catches / gk.parries / gk.tips_over", "MatchEventType::Save events for this side classified by detail text: 'catches' -> catch, 'tips' -> tip over, anything else -> parry."),
        ("gk.gathers", "Open-play ticks where ball.owner becomes (side, 0) after not being (side, 0) on the previous open-play tick (any GK gaining possession: catches, claims, picking up loose balls)."),
        ("gk.claims_non_save", "gk.gathers - gk.catches (possession gains that were not a shot catch: crosses, loose balls, back passes; approximate)."),
        ("gk.goals_conceded", "Opponent score increments (includes own goals)."),
        ("gk.goals_conceded_gk_gt3m_off_line", "Conceded goals where the GK's distance from his own goal line at the tick of the scoring side's last Shot event (within 10 s before the goal) was > 3 m."),
        ("gk.goals_conceded_no_shot_found", "Conceded goals with no Shot event by the scoring side in the preceding 10 s (own goals, deflections); excluded from the line-distance check."),
        ("gk.line_dist_at_conceded_shot_mean_m", "Mean GK line distance at the attributed shot tick over conceded goals with a shot found."),
        ("gk.scenario_switches_per_10min / gk.scenario_distinct_values", "Only present when a GK scenario field exists in the public state: the first serde key of the GK PlayerState (then of MatchState; per-side arrays indexed by side) containing 'scenario', 'gk_mode', 'keeper_mode' or equal to 'gk_state'. Switches = open-play ticks where the serialized value differs from the previous sample (MatchState fields are sampled every 30 ticks). seed reports also list ticks per value."),
        ("shape.<phase>.pct_of_open_play", "% of the side's open-play ticks classified in that phase."),
        ("shape.<phase>.anchor_dist_m.<role>", "Mean distance in metres between each player and psfc_sim::ai::world_anchor_for_player(state, side, idx, 0) (static formation slot, no shift) by PlayerRole."),
        ("shape.<phase>.anchor_dist_shape_m.<role>", "Same for outfielders after subtracting the outfield block's mean x-offset from anchors on that tick: measures loss of shape independent of the whole team pushing up / dropping as a unit."),
        ("shape.<phase>.bbox_area_m2 / bbox_depth_m / bbox_width_m", "Axis-aligned bounding box of on-pitch outfielders (GK excluded): depth = x-extent, width = y-extent, area = depth * width."),
        ("shape.defending.line_x_variance_m2 / line_x_std_m", "Population variance (and std dev) of x of the back line while defending. Back line = outfielders with role Defender; if fewer than 2 (5v5 1-2-1 has one), the 2 outfielders with the deepest base_anchor_permille.x."),
        ("shape.defending.pct_outfield_behind_ball", "Mean % of the side's outfielders strictly goal-side of the ball ((x - ball.x) * attack_sign < 0) while defending."),
        ("shape.defending.swarm2_pct / swarm3_pct", "% of defending ticks where the opponent owns the ball (a carrier exists) with >= 2 (>= 3) of this side's outfielders within 1.6 m of the carrier."),
        ("move.heading_change_pct.<bucket>", "Per player per consecutive open-play tick pair with speed > 1 m/s on both ticks: |angle between vel_t-1 and vel_t| bucketed 0-10, 10-45, 45-90, >90 degrees; % of samples. All on-pitch players incl. GK."),
        ("move.turn_rate_mean_deg_per_s", "Mean of the per-tick heading change (degrees) * 60 over the heading-histogram samples (speed > 1 m/s on both ticks)."),
        ("move.lateral_accel_mean_mps2", "Mean |cross(vel_t-1, vel_t)| / |vel_t-1| * 60 over the same samples: velocity change perpendicular to the previous heading (m/s^2)."),
        ("move.reversals_per_player_min", "Count of those samples with dot(vel_t, vel_t-1) < 0, divided by player-minutes (all consecutive open-play player-ticks / 3600)."),
        ("move.accel_mean_mps2", "Mean |vel_t - vel_t-1| * 60 over all consecutive open-play player-tick pairs (m/s^2)."),
        ("move.accel_gt10_pct", "% of those player-tick pairs with |vel_t - vel_t-1| * 60 > 10 m/s^2 (roughly above a sprinter's peak acceleration)."),
        ("move.accel_from_pos_mean_mps2", "Same as accel_mean_mps2 but from the second difference of positions over three consecutive open-play ticks (|(p_t - p_t-1) - (p_t-1 - p_t-2)| * 3600): what the renderer actually sees, independent of the vel field."),
        ("move.speed_oscillation_pct", "Over three consecutive open-play ticks where both speed changes exceed 0.05 m/s: % where the speed change flips sign (speed up / slow down / speed up sawtooth). A smooth accelerate-cruise-decelerate profile scores near 0."),
        ("move.dspeed_max_mps_per_tick", "Max ||vel_t| - |vel_t-1|| over the match (m/s per tick)."),
        ("collision.<class>.ticks_per_min", "Round 3. Pair classes: any (every on-pitch pair, both teams, GKs included), teammate (pairs of this side's players only), opponent (home-away pairs). An overlap is a pair whose centres are closer than 0.8 m (the sim's min_sep = 2 x 0.4 m body radius) on an open-play tick, sampled after the sim's separation pass. ticks_per_min = open-play ticks with at least one overlapping pair of the class / open-play minutes. 'any' and 'opponent' are match-level and identical in the home and away maps; 'teammate' is the side's own pairs, so match any = home teammate + away teammate + opponent for events."),
        ("collision.<class>.ticks_gt1cm_per_min", "Open-play ticks with at least one pair of the class overlapping by more than 1 cm (centre distance < 0.79 m), per open-play minute. The state is sampled after the sim's separation pass, which leaves resolved pairs at 0.8 m minus fixed-point rounding (resting contact); this variant counts only residual penetration the pass did not resolve."),
        ("collision.correction_vel_corrected_pct / corrected_p50_mps / corrected_p99_mps / max_mps", "Same samples restricted to corrected ones (> 0.01 m/s; fixed-point noise is ~1e-7 m/s): % of samples corrected, nearest-rank p50 / p99 of the corrected samples, and the match maximum over all samples. Contact is rare, so the unconditional p50/p95 are ~0 and the conditional ones characterise the push-out itself."),
        ("collision.<class>.events_per_min","Overlap episodes per open-play minute: a pair's maximal run of consecutive open-play ticks below 0.8 m counts once (runs are cut when open play stops)."),
        ("collision.<class>.depth_mean_m / depth_max_m", "Overlap depth 0.8 m - centre distance over all overlapping pair-ticks of the class: mean, and maximum over the match."),
        ("collision.<class>.stuck_pairs / stuck_mean_ticks", "Stuck pair = overlap episode lasting more than 10 consecutive open-play ticks (no relative-speed condition). Count per match and mean episode length in ticks (60 ticks = 1 s; 0 when there are none)."),
        ("collision.correction_vel_p50_mps / p95 / p99 / pct_gt_0_3", "Position-correction velocity per player per consecutive open-play tick pair: |(pos_t - pos_t-1) * 60 - vel_t| in m/s, all on-pitch players incl. GK. vel_t is the published end-of-tick velocity, which the sim integrated (pos += vel * dt) before the separation pass, so with no contact the value is 0 apart from touchline clamps; with contact it is push-out displacement / dt minus the separation pass's own velocity impulse/slip (both act along the contact, so this is a lower bound of the pure push-out speed). Nearest-rank percentiles over the match; pct_gt_0_3 = % of samples above 0.3 m/s (the research report's p99 target)."),
        ("coord.dual_chaser_ticks_per_min / dual_chaser_pct_of_loose", "Open-play ticks with ball.owner == None (includes passes and shots in flight) on which >= 2 of this side's on-pitch players (GK included) are within 8 m of the ball (xy) and closing on it at > 1.5 m/s (vel . unit(ball - pos)); per open-play minute, and as % of loose-ball ticks."),
        ("coord.max_simultaneous_chasers", "Maximum number of this side's players meeting the chaser condition on one loose-ball tick."),
        ("coord.engagers3_ticks_per_min / engagers3_pct_of_carrier_ticks", "Open-play ticks where the opponent owns the ball and >= 3 of this side's outfielders are within 2.5 m of the carrier; per open-play minute, and as % of ticks with an opposing carrier (same denominator as swarm3_pct)."),
        ("coord.cruise_over72_pct", "Over outfielder-ticks of eligible players (not the ball carrier, more than 8 m from the ball, and not one of the side's two outfielders closest to the ball, ties by slot): % with speed > 0.72 * top speed. Top speed approximates the sim's step_all_players at full stamina with full sprint throttle and no carrier/jockey/direction penalty: (5.45 + 0.026 * pace) m/s * 1.205 (5v5) or * 1.220 (11v11); stamina, the facing-alignment scale and run boosts are ignored, so a tired player's real top speed is lower than assumed (the metric is a lower bound on violations)."),
        ("move.side_flips_per_player_min", "Per player, over the heading-histogram samples (speed > 1 m/s on both ticks): lateral component l_t = cross(vel_t-1, vel_t) / |vel_t-1| (m/s, sign = turning left/right of the previous heading). A side flip is a sample whose l_t and the previous tick's l_t-1 are both above 0.02 m/s in magnitude with opposite signs (avoidance oscillation). Count / player-minutes."),
        ("move.heading_change_p95_deg_per_tick_gt3mps", "95th percentile (nearest rank) of the per-tick heading change |angle(vel_t-1, vel_t)| in degrees, over player-tick pairs with speed > 3 m/s on both ticks."),
        ("shape.<phase>.length_m / width_m / stretch_m", "Round 3 team shape over on-pitch outfielders (GK excluded): length = x extent, width = y extent (same numbers as bbox_depth_m / bbox_width_m, named after the literature), stretch index = mean distance of outfielders to their centroid. Phase attacking = in possession, defending = out of possession (general.phase). Peer-reviewed 11v11 envelope: length 31-46 m, width 35-48 m, stretch 7-16 m; there is no published 5v5 envelope."),
        ("shape.<phase>.nearest_mate_mean_m / nearest_mate_p10_m", "For every on-pitch outfielder and tick of the phase, distance to the nearest other outfielder of the same side; mean and 10th percentile (nearest rank)."),
        ("aerial.jumps_outfield / aerial.jumps_gk","Count of z going from <= 0 to > 0 between consecutive open-play ticks (outfielders / GK; GK dives that leave the ground count)."),
        ("aerial.max_airborne_outfield", "Max number of the side's outfielders with z > 0 on one open-play tick."),
        ("general.autopilot", "Round 4. --home-controller / --away-controller autopilot sets that side to SideController::Human and, before every tick, feeds it psfc_sim::ai::suggest_cpu_input(sim.state(), side, difficulty) (the side-effect-free CPU input, computed from the state after the previous tick, i.e. one tick of input latency; the other side's frame is idle and ignored). This drives the human-side paths (auto-switch, controlled player, teammate contain) that CPU-vs-CPU never runs. Config ids/labels name such a side 'autopilot-<difficulty>'; CPU-only ids are unchanged. Final hashes are deterministic: the unit test autopilot_match_is_deterministic runs the same autopilot config twice and asserts identical state_hash."),
        ("general.turnover", "Round 4. A turnover against side L happens on the open-play tick t0 on which the opponent O becomes ball.owner while the previous open-play owner was a player of L whose last owned tick is at most 120 ticks (2 s) earlier, with no stoppage in between (any non-open-play tick clears the last owner). So the ball goes from L to O directly (tackle, interception of a pass, keeper claim of a shot or cross) or after a loose spell of at most 2 s; a loose ball contested for longer, and restarts (throw-ins, goal kicks, free kicks), are not turnovers. A transition episode for L starts at t0 and ends at the first of: a stoppage, L owning the ball again, or t0 + 15 s. All trans.* metrics are in the losing side's map."),
        ("trans.count_per_10min", "Turnovers suffered by the side (general.turnover) per 10 open-play minutes."),
        ("trans.time_to_shape_s / time_to_shape_capped_pct / time_to_shape_excluded_pct", "Seconds from t0 to the first episode tick on which at least 80% of the side's on-pitch outfielders are either within 5 m of psfc_sim::ai::defensive_shape_anchor(state, side, idx) (the defensive-block slot the AI is assigned, ball-relative via the team memory's ref_ball; falls back to the static slot for unassigned players) or strictly goal-side of the ball ((x - ball.x) * attack_sign < 0). Episodes reaching 15 s without it score 15 s and count as capped (capped_pct = % of scored episodes). Episodes that end earlier by a stoppage or by the side regaining the ball, before reaching shape, are not scored (excluded_pct = % of all turnovers). The tick t0 itself is checked (0 s = already in shape)."),
        ("trans.in_shape_at_turnover_pct", "% of scored time_to_shape episodes (defensive_shape_anchor variant) already in shape on the turnover tick t0 (time 0 s). A high value means the criterion is met by position at the moment of the loss (players already goal-side of the ball), not by a recovery run."),
        ("trans.time_to_shape_static_s /_static_capped_pct / _static_excluded_pct", "Same with the static formation slot world_anchor_for_player(state, side, idx, 0) instead of defensive_shape_anchor: a reference that does not depend on the AI's own block logic, for before/after comparisons across AI changes."),
        ("trans.pct_outfield_behind_ball_at_3s / _at_6s", "% of the side's on-pitch outfielders strictly goal-side of the ball at exactly t0 + 3 s (180 ticks) and t0 + 6 s (360 ticks), over episodes still running at that tick (no stoppage, not regained)."),
        ("trans.forward_recovery_speed_mps_0_5s / forward_sprint_share_0_5s", "Over episode ticks t0 .. t0 + 5 s (exclusive) and the side's on-pitch players with PlayerRole::Forward: mean |vel| (m/s), and % of those player-ticks with |vel| > 6 m/s."),
        ("trans.mid_recovery_speed_mps_0_5s / mid_sprint_share_0_5s", "Same for PlayerRole::Midfielder."),
        ("trans.forwards_staying_high_mean", "Mean number of the side's on-pitch Forwards ahead of the ball ((x - ball.x) * attack_sign > 0) at t0 + 6 s, over episodes still running then (rest offence kept high)."),
        ("trans.counterpress_pct", "% of turnovers after which the side owns the ball again within 6 s (t0 + 360 ticks) with no stoppage in between."),
        ("general.support", "Round 4. Support metrics are sampled on open-play ticks where an outfielder (slot != 0) of the side is ball.owner (carrier ticks). Open lane to a teammate: no on-pitch opponent (GK included) within 1.5 m of the segment carrier -> teammate. Offside: mirrors the sim's is_player_offside_at_pass: offsides_enabled, teammate (not the GK) past the offside-zone line (halfway in 11v11, +half_length/3 in 5v5 Rush), more than 0.15 m ahead of the ball and more than 0.2 m ahead of the second-last on-pitch opponent (GK included), all along the attacking axis (this is the task's 'ahead of the second-last opponent' rule with the sim's own zone, ball and tolerance conditions, so it matches what the referee would flag). Open option: an outfield teammate 6-30 m from the carrier with an open lane and not offside."),
        ("support.open_options_mean / open_options_p10 / ticks_with_le1_option_pct", "Number of open options per carrier tick: mean, 10th percentile (nearest rank) across carrier ticks, and % of carrier ticks with 0 or 1 open option."),
        ("support.nearest_option_dist_m", "Mean distance carrier -> nearest open option, over carrier ticks with at least one open option."),
        ("support.forward_options_mean", "Mean number of open options at least 3 m ahead of the carrier along the attacking axis ((x_mate - x_carrier) * attack_sign >= 3)."),
        ("support.runs_in_behind_per_10min", "Per outfielder other than the owner, on open-play ticks while his side owns the ball (any owner incl. GK), against the line L = second-last opponent's x along the attacking axis on that tick: a run counts when the player was at or behind L on an earlier such tick (armed) and is now ahead of L with |vel| > 5 m/s. Since the work-area-3 second pass, the ticks of the side's own pass in flight also count for the pass's intended receiver (the other players' states are held), and so does the reception tick on which that receiver becomes the owner. After crossing (counted or not) he must drop more than 1 m behind L to be armed again; losing possession or a stoppage disarms everyone. Count per 10 open-play minutes."),
        ("support.run_abandon_per_10min", "Round 4 (work area 3). Committed runs abandoned per 10 open-play minutes: a run role in the sim's TeamAiMemory (run_role / run_since: penetrate, support-from-behind, box, decoy, called) that ends or is replaced fewer than 60 ticks after it began, on an open-play tick on which the opponent does not own the ball, the ball is not loose (no owner and no pass of the side in flight), and the runner neither owns it nor is the intended receiver of the side's pass in flight. Runs ended by a stoppage (non-open-play tick), by possession loss, or by receiving the ball are not abandons (FIFA 12's 'change their minds three or four times' failure)."),
        ("support.releases_per_10min", "Round 4 (work area 3). Releases of committed runners per 10 open-play minutes: a CPU pass decision selecting a committed runner (TeamAiMemory::run_release with a new run_release_tick), or a ground or through pass of the side (the passer's recent_kick_kind 1 or 2; lofted switches excluded) launched toward a penetrate or called runner (run_role 1 or 5) that no CPU release of the same runner preceded within 60 ticks."),
        ("support.release_pass_completed_pct", "% of passes launched toward a released / committed runner (as counted for releases_per_10min) whose next open-play owner is that runner; an opponent owning it, another teammate collecting it, or a stoppage (offside, out of play) before any owner is a failure."),
        ("support.cluster_pct", "% of carrier ticks where two outfield teammates other than the carrier are within 4 m of each other and both within 15 m of the carrier."),
        ("support.width_m / support.depth_m", "y-extent / x-extent of the side's on-pitch outfielders (carrier included) on carrier ticks, mean."),
        ("support.carrier_pct_of_open_play", "Carrier ticks as % of the side's open-play ticks (sample size of the support metrics)."),
        ("support.pass_target_open_pct / passes_completed_tracked", "From the event stream plus owner transitions (the sim's Pass event carries no receiver): a MatchEventType::Pass of the side within 2 ticks after its last carrier tick starts a pass by that carrier, remembering which teammates (GK included, no distance band) had an open, non-offside lane on that last carrier tick. The pass is completed when the next open-play owner is a teammate other than the passer; any other next owner, or a stoppage, ends it. pass_target_open_pct = % of completed passes whose receiver had an open lane at the moment of the pass; passes_completed_tracked = their count per match (open-play passes only; Pass events also cover clearances, which count when a teammate collects them; set-piece passes are not tracked)."),
        ("move.side_flips_by_class.<c>_per_player_min / _share_pct", "Round 4. Side flips (move.side_flips_per_player_min) split by psfc_sim::ai::move_priority(state, side, idx) on the tick of the flip (0 carrier / elected chaser / committed keeper, 1 engager or human-controlled, 2 tactical mover, 3 idle / cruising / yielding; outside a fresh plan everyone but the carrier reads 2). per_player_min = flips in class c / player-minutes spent in class c (all consecutive open-play player-tick pairs whose current tick is in class c); share_pct = % of all the side's flips that occurred in class c."),
        ("move.class_time_pct.<c>", "% of consecutive open-play player-tick pairs whose current tick has move_priority c (the exposure behind the per-class rates)."),
        ("move.side_flips_speed_lt3_per_player_min / _ge3_per_player_min", "Side flips split by the current tick's speed |vel_t| < 3 m/s or >= 3 m/s, each divided by the player-minutes spent in that speed band."),
        ("move.side_flips_schmitt_per_player_min", "Round 5 (WA1). Schmitt-band variant of move.side_flips_per_player_min over the same samples: a per-player debounced side (left/right of the previous heading) changes only after the lateral component l_t has stayed beyond the opposite +-0.05 m/s band for 4 consecutive samples; each change from one side to the other counts one flip (the first side taken after a gap does not count; the state resets when a tick has no heading sample). Ignores sub-quantum tick-to-tick noise and counts weaving whose half-period is >= 4 ticks. Count / player-minutes."),
        ("move.side_flips_cause.<cause>_per_player_min", "Round 5 (WA1). Raw side flips (move.side_flips_per_player_min) split by the earliest steering stage whose own lateral component (versus the same previous-tick velocity) flipped sign with the same 0.02 m/s dead band on the same tick pair: desired = the desired velocity before local avoidance flipped (decision side: intent, target, input); sampler = the avoidance-selected velocity flipped while the desired one did not; filter = only the filtered command flipped; limiter = none of the three flipped (motion limits, turn limiter, contact constraint; also flips without fresh psfc_sim PlayerState::steer_debug diagnostics, e.g. scripted actions). Each divided by all player-minutes, so the four sum to the raw rate."),
        ("move.heading_sign_flips_per_player_min", "Over the heading-histogram samples (speed > 1 m/s on both ticks): signed heading change theta_t = atan2(cross(vel_t-1, vel_t), dot(vel_t-1, vel_t)) in degrees. A heading sign flip is a sample where |theta_t| > 1 deg and the previous consecutive sample's |theta_t-1| > 1 deg with opposite signs (left-right-left steering wobble, independent of the 0.02 m/s lateral dead band of side flips). Count / player-minutes."),
        ("aerial.headers (+ _shot/_clear/_pass)", "Outfielder with z > 0.14 m whose recent_kick_ticks rose versus the previous tick (a new contact); split by recent_kick_kind: 4/5 shot, 3 clearance/lob, else pass/flick."),
    ];
    d.iter()
        .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
        .collect()
}

// ---------------------------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------------------------

fn fmt_num(v: f64) -> String {
    if v.fract() == 0.0 && v.abs() < 1e9 {
        format!("{v:.0}")
    } else if v.abs() >= 100.0 {
        format!("{v:.1}")
    } else {
        format!("{v:.2}")
    }
}

const SECTIONS: [(&str, &str); 9] = [
    ("match.", "Match"),
    ("gk.", "Goalkeeper"),
    ("shape.", "Defence and shape"),
    ("trans.", "Defensive transition"),
    ("support.", "Attacking support"),
    ("move.", "Movement smoothness"),
    ("collision.", "Collisions and push-out"),
    ("coord.", "Coordination"),
    ("aerial.", "Aerial"),
];

/// Peer-reviewed 11v11 team-shape envelope (research report, Parameters table).
const SHAPE_ENVELOPE: [(&str, f64, f64); 3] = [
    ("length_m", 31.0, 46.0),
    ("width_m", 35.0, 48.0),
    ("stretch_m", 7.0, 16.0),
];

fn render_shape_envelope(out: &mut String, r: &ConfigReport) {
    let _ = writeln!(out, "### Team shape vs envelope (teams mean)\n");
    let eleven = matches!(r.config.format, FormatKind::ElevenASide);
    let _ = writeln!(
        out,
        "| metric | in possession | out of possession | 11v11 envelope (m) |\n|---|---:|---:|---|"
    );
    let get = |k: String| {
        r.mean_teams
            .get(&k)
            .map_or_else(|| "n/a".to_string(), |v| fmt_num(*v))
    };
    for (k, lo, hi) in SHAPE_ENVELOPE {
        // For 11v11 mark values outside the envelope; 5v5 has none to compare against.
        let cell = |ph: &str| {
            let key = format!("shape.{ph}.{k}");
            match r.mean_teams.get(&key) {
                Some(v) if eleven && (*v < lo || *v > hi) => format!("{} (outside)", fmt_num(*v)),
                _ => get(key),
            }
        };
        let _ = writeln!(
            out,
            "| {k} | {} | {} | {lo:.0}-{hi:.0} |",
            cell("attacking"),
            cell("defending")
        );
    }
    for k in ["nearest_mate_mean_m", "nearest_mate_p10_m"] {
        let _ = writeln!(
            out,
            "| {k} | {} | {} | none published (in-possession mean near 9-10 m, estimate) |",
            get(format!("shape.attacking.{k}")),
            get(format!("shape.defending.{k}"))
        );
    }
    if eleven {
        let _ = writeln!(
            out,
            "\nEnvelope: peer-reviewed 11v11 ranges (length 31-46 m, width 35-48 m, stretch index 7-16 m) quoted in the research report; the report's tighter targets are 31-36 / 35-40 / 7-10 m out of possession and 40-46 / 42-48 / 12-16 m in possession (estimates).\n"
        );
    } else {
        let _ = writeln!(
            out,
            "\nNote: 5v5 has no published team-shape envelope; the 11v11 ranges are printed for reference only and do not apply to a 4-outfielder team on a small pitch.\n"
        );
    }
}

pub fn render_markdown(report: &AnalysisReport, include_definitions: bool) -> String {
    let mut out = String::new();
    for r in &report.reports {
        let _ = writeln!(out, "## {}\n", r.label);
        let _ = writeln!(out, "Command: `{}`\n", r.command);
        let _ = writeln!(out, "Gameplay version: `{}`\n", r.gameplay_version);
        let _ = writeln!(
            out,
            "| seed | final state_hash | score (H-A) |\n|---|---|---|"
        );
        if r.per_seed.is_empty() {
            for (i, h) in r.final_hashes.iter().enumerate() {
                let _ = writeln!(out, "| {} | `{h}` | |", r.config.seed_base + i as u64);
            }
        } else {
            for s in &r.per_seed {
                let _ = writeln!(
                    out,
                    "| {} | `{}` | {}-{} |",
                    s.seed, s.final_hash_hex, s.score[0], s.score[1]
                );
            }
        }
        out.push('\n');
        let _ = writeln!(
            out,
            "Match meta (mean over seeds): {}\n",
            r.mean_match
                .iter()
                .map(|(k, v)| format!("{k} = {}", fmt_num(*v)))
                .collect::<Vec<_>>()
                .join(", ")
        );
        for (prefix, title) in SECTIONS {
            let _ = writeln!(out, "### {title}\n");
            let _ = writeln!(
                out,
                "| metric | home | away | teams mean |\n|---|---:|---:|---:|"
            );
            for (k, v) in r.mean_teams.iter().filter(|(k, _)| k.starts_with(prefix)) {
                let h = r.mean_home.get(k).copied().unwrap_or(f64::NAN);
                let a = r.mean_away.get(k).copied().unwrap_or(f64::NAN);
                let _ = writeln!(
                    out,
                    "| {} | {} | {} | **{}** |",
                    &k[prefix.len()..],
                    fmt_num(h),
                    fmt_num(a),
                    fmt_num(*v)
                );
            }
            out.push('\n');
        }
        render_shape_envelope(&mut out, r);
        if !r.per_seed.is_empty() {
            let _ = writeln!(out, "### Per seed (home / away)\n");
            let mut hdr = "| metric |".to_string();
            let mut sep = "|---|".to_string();
            for s in &r.per_seed {
                let _ = write!(hdr, " seed {} |", s.seed);
                sep.push_str("---:|");
            }
            let _ = writeln!(out, "{hdr}\n{sep}");
            for k in r.mean_teams.keys() {
                let mut row = format!("| {k} |");
                for s in &r.per_seed {
                    let h = s.home.get(k).copied().unwrap_or(f64::NAN);
                    let a = s.away.get(k).copied().unwrap_or(f64::NAN);
                    let _ = write!(row, " {} / {} |", fmt_num(h), fmt_num(a));
                }
                let _ = writeln!(out, "{row}");
            }
            out.push('\n');
            for s in &r.per_seed {
                if let Some(field) = &s.gk_scenario_field {
                    let _ = writeln!(
                        out,
                        "GK scenario field `{field}` (seed {}): home {:?}; away {:?}\n",
                        s.seed, s.gk_scenario_ticks[0], s.gk_scenario_ticks[1]
                    );
                }
            }
        }
        if r.per_seed.iter().all(|s| s.gk_scenario_field.is_none()) && !r.per_seed.is_empty() {
            let _ = writeln!(out, "GK scenario field: not present in the public state.\n");
        }
    }
    if include_definitions {
        let _ = writeln!(out, "## Metric definitions\n");
        for (k, v) in &report.definitions {
            let _ = writeln!(out, "- **{k}**: {v}");
        }
        out.push('\n');
    }
    out
}

/// Before/after table for every report id present in both.
pub fn render_compare(before: &AnalysisReport, after: &AnalysisReport) -> String {
    let mut out = String::new();
    for a in &after.reports {
        let Some(b) = before.reports.iter().find(|b| b.id == a.id) else {
            let _ = writeln!(out, "## {} (no baseline with id `{}`)\n", a.label, a.id);
            continue;
        };
        let _ = writeln!(out, "## {} — before vs after (teams mean)\n", a.label);
        let _ = writeln!(
            out,
            "| metric | before | after | delta |\n|---|---:|---:|---:|"
        );
        let keys: std::collections::BTreeSet<&String> =
            a.mean_teams.keys().chain(b.mean_teams.keys()).collect();
        for k in keys {
            let bv = b.mean_teams.get(k).copied();
            let av = a.mean_teams.get(k).copied();
            // Keys missing on either side (e.g. round-3 metrics against an older baseline) are "n/a".
            let cell = |v: Option<f64>| v.map_or_else(|| "n/a".to_string(), fmt_num);
            let delta = match (bv, av) {
                (Some(x), Some(y)) => format!("{:+.2}", y - x),
                _ => "n/a".to_string(),
            };
            let _ = writeln!(out, "| {k} | {} | {} | {delta} |", cell(bv), cell(av));
        }
        let same = a.final_hashes == b.final_hashes;
        let _ = writeln!(out, "\nFinal hashes identical to baseline: {same}\n");
    }
    out
}

// ---------------------------------------------------------------------------------------------
// CLI
// ---------------------------------------------------------------------------------------------

pub const USAGE: &str =
    "psfc-replay analyze [--format 5v5|11v11] [--seeds N=3] [--seed-base B=1] [--minutes M=10] \
[--home-difficulty easy|normal|hard] [--away-difficulty easy|normal|hard] \
[--home-controller cpu|autopilot] [--away-controller cpu|autopilot] [--preset baseline] \
[--json <path>] [--md <path>] [--per-seed] [--compare <baseline.json>]";

pub fn run_analyze(args: &[String]) -> Result<(), String> {
    let mut format = FormatKind::FiveASide;
    let mut seeds = 3_u32;
    let mut seed_base = 1_u64;
    let mut minutes = 10_u32;
    let mut home = CpuDifficulty::Normal;
    let mut away = CpuDifficulty::Normal;
    let mut json_path: Option<String> = None;
    let mut md_path: Option<String> = None;
    let mut compare_path: Option<String> = None;
    let mut per_seed = false;
    let mut preset: Option<String> = None;
    let mut home_ap = false;
    let mut away_ap = false;
    let parse_controller = |v: &str| -> Result<bool, String> {
        match v.to_ascii_lowercase().as_str() {
            "cpu" => Ok(false),
            "autopilot" => Ok(true),
            o => Err(format!("invalid controller '{o}' (expected cpu|autopilot)")),
        }
    };

    let mut i = 0;
    let val = |i: usize, name: &str| -> Result<String, String> {
        args.get(i)
            .cloned()
            .ok_or_else(|| format!("missing {name} value"))
    };
    while i < args.len() {
        let a = args[i].as_str();
        match a {
            "--format" => {
                i += 1;
                format = FormatKind::parse(&val(i, a)?)?;
            }
            "--seeds" => {
                i += 1;
                seeds = val(i, a)?.parse().map_err(|_| "invalid --seeds")?;
            }
            "--seed-base" => {
                i += 1;
                seed_base = val(i, a)?.parse().map_err(|_| "invalid --seed-base")?;
            }
            "--minutes" => {
                i += 1;
                minutes = val(i, a)?.parse().map_err(|_| "invalid --minutes")?;
            }
            "--home-difficulty" => {
                i += 1;
                home = parse_difficulty(&val(i, a)?)?;
            }
            "--away-difficulty" => {
                i += 1;
                away = parse_difficulty(&val(i, a)?)?;
            }
            "--home-controller" => {
                i += 1;
                home_ap = parse_controller(&val(i, a)?)?;
            }
            "--away-controller" => {
                i += 1;
                away_ap = parse_controller(&val(i, a)?)?;
            }
            "--json" => {
                i += 1;
                json_path = Some(val(i, a)?);
            }
            "--md" => {
                i += 1;
                md_path = Some(val(i, a)?);
            }
            "--compare" => {
                i += 1;
                compare_path = Some(val(i, a)?);
            }
            "--preset" => {
                i += 1;
                preset = Some(val(i, a)?);
            }
            "--per-seed" => per_seed = true,
            "--help" | "-h" => {
                println!("{USAGE}");
                return Ok(());
            }
            other => return Err(format!("unknown argument '{other}'\n{USAGE}")),
        }
        i += 1;
    }
    if minutes == 0 {
        return Err("--minutes must be >= 1".into());
    }

    let configs: Vec<AnalyzeConfig> = match preset.as_deref() {
        None => vec![AnalyzeConfig {
            format,
            seeds,
            seed_base,
            minutes,
            home_difficulty: home,
            away_difficulty: away,
            home_autopilot: home_ap,
            away_autopilot: away_ap,
        }],
        Some("baseline") => vec![
            AnalyzeConfig {
                format: FormatKind::FiveASide,
                seeds,
                seed_base,
                minutes,
                home_difficulty: CpuDifficulty::Hard,
                away_difficulty: CpuDifficulty::Hard,
                home_autopilot: false,
                away_autopilot: false,
            },
            AnalyzeConfig {
                format: FormatKind::FiveASide,
                seeds,
                seed_base,
                minutes,
                home_difficulty: CpuDifficulty::Normal,
                away_difficulty: CpuDifficulty::Normal,
                home_autopilot: false,
                away_autopilot: false,
            },
            AnalyzeConfig {
                format: FormatKind::ElevenASide,
                seeds,
                seed_base,
                minutes,
                home_difficulty: CpuDifficulty::Normal,
                away_difficulty: CpuDifficulty::Normal,
                home_autopilot: false,
                away_autopilot: false,
            },
            // Round 4: human-side code paths (auto-switch, controlled player, teammate contain).
            AnalyzeConfig {
                format: FormatKind::FiveASide,
                seeds,
                seed_base,
                minutes,
                home_difficulty: CpuDifficulty::Hard,
                away_difficulty: CpuDifficulty::Hard,
                home_autopilot: true,
                away_autopilot: false,
            },
            AnalyzeConfig {
                format: FormatKind::ElevenASide,
                seeds,
                seed_base,
                minutes,
                home_difficulty: CpuDifficulty::Normal,
                away_difficulty: CpuDifficulty::Normal,
                home_autopilot: true,
                away_autopilot: false,
            },
        ],
        Some(o) => return Err(format!("unknown --preset '{o}' (expected baseline)")),
    };

    let results: Vec<Result<ConfigReport, String>> = std::thread::scope(|scope| {
        let handles: Vec<_> = configs
            .iter()
            .map(|c| scope.spawn(move || analyze_config(c, true)))
            .collect();
        handles
            .into_iter()
            .map(|h| {
                h.join()
                    .unwrap_or_else(|_| Err("analysis thread panicked".to_string()))
            })
            .collect()
    });
    let mut reports = results.into_iter().collect::<Result<Vec<_>, _>>()?;
    let report = AnalysisReport {
        tool: format!("psfc-replay analyze {}", env!("CARGO_PKG_VERSION")),
        reports: reports.clone(),
        definitions: metric_definitions(),
    };

    // Console: tables (per-seed only when asked), definitions go to the md file.
    if !per_seed {
        for r in &mut reports {
            r.per_seed.clear();
        }
    }
    let console = AnalysisReport {
        reports,
        ..report.clone()
    };
    println!("{}", render_markdown(&console, false));

    if let Some(p) = &json_path {
        let s = serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?;
        fs::write(p, s).map_err(|e| format!("failed to write '{p}': {e}"))?;
    }
    if let Some(p) = &md_path {
        let md_report = if per_seed {
            report.clone()
        } else {
            console.clone()
        };
        let md = format!("# Match analytics\n\n{}", render_markdown(&md_report, true));
        fs::write(p, md).map_err(|e| format!("failed to write '{p}': {e}"))?;
    }
    if let Some(p) = &compare_path {
        let raw = fs::read_to_string(p).map_err(|e| format!("failed to read '{p}': {e}"))?;
        let before: AnalysisReport =
            serde_json::from_str(&raw).map_err(|e| format!("invalid baseline JSON '{p}': {e}"))?;
        println!("{}", render_compare(&before, &report));
    }
    if json_path.is_none() {
        // JSON is always available on stdout when no file is requested.
        println!(
            "{}",
            serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_minute_match_is_populated_and_deterministic() {
        let cfg = AnalyzeConfig {
            format: FormatKind::FiveASide,
            seeds: 1,
            seed_base: 7,
            minutes: 1,
            home_difficulty: CpuDifficulty::Hard,
            away_difficulty: CpuDifficulty::Normal,
            home_autopilot: false,
            away_autopilot: false,
        };
        let a = analyze_seed(&cfg, 7).expect("run a");
        let b = analyze_seed(&cfg, 7).expect("run b");
        assert!(a.finished, "1-minute match should reach full time");
        assert_eq!(a.final_hash_hex, b.final_hash_hex);
        assert_eq!(a.home, b.home);
        assert_eq!(a.away, b.away);
        assert_eq!(a.match_metrics, b.match_metrics);
        assert!(a.match_metrics["open_play_minutes"] > 0.5);
        for side in [&a.home, &a.away] {
            for key in [
                "match.possession_pct",
                "gk.pct_outside_6yd",
                "gk.line_dist_mean_m",
                "shape.defending.line_x_variance_m2",
                "shape.defending.swarm2_pct",
                "shape.defending.pct_outfield_behind_ball",
                "shape.attacking.bbox_area_m2",
                "shape.defending.anchor_dist_m.defender",
                "move.heading_change_pct.0_10",
                "move.reversals_per_player_min",
                "move.accel_mean_mps2",
                "aerial.jumps_outfield",
            ] {
                let v = side
                    .get(key)
                    .unwrap_or_else(|| panic!("missing metric {key}"));
                assert!(v.is_finite(), "{key} not finite");
            }
            assert!(side["gk.line_dist_mean_m"] > 0.0);
            assert!(side["shape.attacking.bbox_area_m2"] > 0.0);
            assert!(side["move.accel_mean_mps2"] > 0.0);
            let hist: f64 = ["0_10", "10_45", "45_90", "gt90"]
                .iter()
                .map(|b| side[&format!("move.heading_change_pct.{b}")])
                .sum();
            assert!(
                (hist - 100.0).abs() < 1e-6,
                "heading histogram sums to {hist}"
            );
        }
        let poss = a.home["match.possession_pct"] + a.away["match.possession_pct"];
        assert!((poss - 100.0).abs() < 1e-6);

        // Round-3 movement-coordination metrics: present, finite, sane and deterministic.
        for side in [&a.home, &a.away] {
            let mut keys: Vec<String> = Vec::new();
            for class in ["any", "teammate", "opponent"] {
                for k in [
                    "ticks_per_min",
                    "events_per_min",
                    "depth_mean_m",
                    "depth_max_m",
                    "stuck_pairs",
                    "stuck_mean_ticks",
                ] {
                    keys.push(format!("collision.{class}.{k}"));
                }
            }
            for ph in ["attacking", "defending"] {
                for k in [
                    "length_m",
                    "width_m",
                    "stretch_m",
                    "nearest_mate_mean_m",
                    "nearest_mate_p10_m",
                ] {
                    keys.push(format!("shape.{ph}.{k}"));
                }
            }
            for k in [
                "collision.correction_vel_p50_mps",
                "collision.correction_vel_p95_mps",
                "collision.correction_vel_p99_mps",
                "collision.correction_vel_pct_gt_0_3",
                "coord.dual_chaser_ticks_per_min",
                "coord.dual_chaser_pct_of_loose",
                "coord.max_simultaneous_chasers",
                "coord.engagers3_ticks_per_min",
                "coord.engagers3_pct_of_carrier_ticks",
                "coord.cruise_over72_pct",
                "move.side_flips_per_player_min",
                "move.heading_change_p95_deg_per_tick_gt3mps",
            ] {
                keys.push(k.to_string());
            }
            for key in &keys {
                let v = side
                    .get(key)
                    .unwrap_or_else(|| panic!("missing metric {key}"));
                assert!(v.is_finite() && *v >= 0.0, "{key} = {v}");
            }
            let c = |k: &str| side[k];
            assert!(c("collision.correction_vel_p50_mps") <= c("collision.correction_vel_p95_mps"));
            assert!(c("collision.correction_vel_p95_mps") <= c("collision.correction_vel_p99_mps"));
            assert!(c("collision.any.events_per_min") >= c("collision.teammate.events_per_min"));
            assert!(c("collision.any.events_per_min") >= c("collision.opponent.events_per_min"));
            assert!(c("collision.any.depth_max_m") <= OVERLAP_DIST_M);
            assert!(c("coord.max_simultaneous_chasers") >= 1.0);
            assert!(c("move.heading_change_p95_deg_per_tick_gt3mps") <= 180.0);
            for ph in ["attacking", "defending"] {
                assert!(c(&format!("shape.{ph}.length_m")) > 0.0);
                assert!(c(&format!("shape.{ph}.width_m")) > 0.0);
                assert!(c(&format!("shape.{ph}.stretch_m")) > 0.0);
                assert!(
                    c(&format!("shape.{ph}.nearest_mate_p10_m"))
                        <= c(&format!("shape.{ph}.nearest_mate_mean_m")) + 1e-9
                );
            }
        }
        // Match-level collision classes are identical in both side maps.
        for k in [
            "collision.any.events_per_min",
            "collision.opponent.ticks_per_min",
        ] {
            assert_eq!(a.home[k], a.away[k], "{k}");
        }
        // Teammate events of both sides + opponent events = all events (same open-play minutes).
        let sum = a.home["collision.teammate.events_per_min"]
            + a.away["collision.teammate.events_per_min"]
            + a.home["collision.opponent.events_per_min"];
        assert!((sum - a.home["collision.any.events_per_min"]).abs() < 1e-9);
    }

    #[test]
    fn autopilot_match_is_deterministic() {
        let cfg = AnalyzeConfig {
            format: FormatKind::FiveASide,
            seeds: 1,
            seed_base: 3,
            minutes: 2,
            home_difficulty: CpuDifficulty::Hard,
            away_difficulty: CpuDifficulty::Hard,
            home_autopilot: true,
            away_autopilot: false,
        };
        assert_eq!(cfg.id(), "5v5_autopilot-hard_vs_hard_2min_1x3");
        let a = analyze_seed(&cfg, 3).expect("run a");
        let b = analyze_seed(&cfg, 3).expect("run b");
        assert!(a.finished);
        assert_eq!(a.final_hash_hex, b.final_hash_hex);
        assert_eq!(a.home, b.home);
        assert_eq!(a.away, b.away);
        // The auto-piloted side actually plays: it touches the ball and has carrier ticks.
        assert!(
            a.home["match.possession_pct"] > 5.0,
            "{}",
            a.home["match.possession_pct"]
        );
        assert!(a.home["support.carrier_pct_of_open_play"] > 0.0);
        // And it differs from the CPU-vs-CPU match on the same seed.
        let cpu = analyze_seed(
            &AnalyzeConfig {
                home_autopilot: false,
                ..cfg.clone()
            },
            3,
        )
        .expect("cpu run");
        assert_ne!(cpu.final_hash_hex, a.final_hash_hex);
    }

    #[test]
    fn round4_metrics_are_present_and_sane() {
        let cfg = AnalyzeConfig {
            format: FormatKind::FiveASide,
            seeds: 1,
            seed_base: 1,
            minutes: 3,
            home_difficulty: CpuDifficulty::Hard,
            away_difficulty: CpuDifficulty::Hard,
            home_autopilot: false,
            away_autopilot: false,
        };
        let r = analyze_seed(&cfg, 1).expect("run");
        for side in [&r.home, &r.away] {
            for k in [
                "trans.count_per_10min",
                "support.open_options_mean",
                "support.open_options_p10",
                "support.ticks_with_le1_option_pct",
                "support.forward_options_mean",
                "support.runs_in_behind_per_10min",
                "support.cluster_pct",
                "support.width_m",
                "support.depth_m",
                "move.side_flips_speed_lt3_per_player_min",
                "move.side_flips_speed_ge3_per_player_min",
                "move.heading_sign_flips_per_player_min",
            ] {
                let v = side.get(k).unwrap_or_else(|| panic!("missing {k}"));
                assert!(v.is_finite() && *v >= 0.0, "{k} = {v}");
            }
            assert!(side["trans.count_per_10min"] > 0.0);
            assert!(side["support.open_options_p10"] <= side["support.open_options_mean"] + 1e-9);
            assert!(
                side["support.forward_options_mean"] <= side["support.open_options_mean"] + 1e-9
            );
            let share: f64 = (0..4)
                .map(|c| side[&format!("move.side_flips_by_class.{c}_share_pct")])
                .sum();
            let time: f64 = (0..4)
                .map(|c| side[&format!("move.class_time_pct.{c}")])
                .sum();
            assert!((time - 100.0).abs() < 1e-6, "class time sums to {time}");
            if side["move.side_flips_per_player_min"] > 0.0 {
                assert!((share - 100.0).abs() < 1e-6, "flip shares sum to {share}");
            }
            if let Some(t) = side.get("trans.time_to_shape_s") {
                assert!((0.0..=15.0).contains(t));
            }
            for k in ["trans.counterpress_pct", "support.pass_target_open_pct"] {
                if let Some(v) = side.get(k) {
                    assert!((0.0..=100.0).contains(v), "{k} = {v}");
                }
            }
        }
    }

    #[test]
    fn segment_distance_and_lane() {
        assert!((dist_to_segment((0.0, 1.0), (-1.0, 0.0), (1.0, 0.0)) - 1.0).abs() < 1e-12);
        assert!((dist_to_segment((3.0, 0.0), (-1.0, 0.0), (1.0, 0.0)) - 2.0).abs() < 1e-12);
        assert!((dist_to_segment((1.0, 1.0), (0.0, 0.0), (0.0, 0.0)) - 2f64.sqrt()).abs() < 1e-12);
    }

    #[test]
    fn percentile_is_nearest_rank() {
        let mut v = vec![5.0_f32, 1.0, 4.0, 2.0, 3.0];
        assert_eq!(percentile(&mut v, 0.5), 3.0);
        assert_eq!(percentile(&mut v, 0.99), 5.0);
        assert_eq!(percentile(&mut v, 0.1), 1.0);
        assert_eq!(percentile(&mut [], 0.5), 0.0);
    }

    #[test]
    fn compare_tolerates_missing_keys() {
        let mk = |keys: &[(&str, f64)]| {
            let cfg = AnalyzeConfig {
                format: FormatKind::FiveASide,
                seeds: 1,
                seed_base: 1,
                minutes: 1,
                home_difficulty: CpuDifficulty::Normal,
                away_difficulty: CpuDifficulty::Normal,
                home_autopilot: false,
                away_autopilot: false,
            };
            let m: Metrics = keys.iter().map(|(k, v)| ((*k).to_string(), *v)).collect();
            AnalysisReport {
                tool: String::new(),
                definitions: BTreeMap::new(),
                reports: vec![ConfigReport {
                    id: cfg.id(),
                    label: cfg.label(),
                    command: cfg.command_line(),
                    config: cfg,
                    gameplay_version: String::new(),
                    final_hashes: vec![],
                    mean_match: Metrics::new(),
                    mean_home: m.clone(),
                    mean_away: m.clone(),
                    mean_teams: m,
                    per_seed: vec![],
                }],
            }
        };
        let before = mk(&[("move.accel_mean_mps2", 1.0)]);
        let after = mk(&[
            ("move.accel_mean_mps2", 1.5),
            ("collision.any.events_per_min", 2.0),
        ]);
        let out = render_compare(&before, &after);
        assert!(
            out.contains("| collision.any.events_per_min | n/a | 2 | n/a |"),
            "{out}"
        );
        assert!(
            out.contains("| move.accel_mean_mps2 | 1 | 1.50 | +0.50 |"),
            "{out}"
        );
    }
}
