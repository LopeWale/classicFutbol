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
}

impl AnalyzeConfig {
    pub fn label(&self) -> String {
        format!(
            "{} {} vs {} ({} min x {} seeds from {})",
            self.format.label(),
            difficulty_label(self.home_difficulty),
            difficulty_label(self.away_difficulty),
            self.minutes,
            self.seeds,
            self.seed_base
        )
    }

    /// Stable id used by `--compare` to pair reports across runs.
    pub fn id(&self) -> String {
        format!(
            "{}_{}_vs_{}_{}min_{}x{}",
            self.format.label(),
            difficulty_label(self.home_difficulty),
            difficulty_label(self.away_difficulty),
            self.minutes,
            self.seeds,
            self.seed_base
        )
    }

    pub fn command_line(&self) -> String {
        format!(
            "psfc-replay analyze --format {} --seeds {} --seed-base {} --minutes {} --home-difficulty {} --away-difficulty {}",
            self.format.label(),
            self.seeds,
            self.seed_base,
            self.minutes,
            difficulty_label(self.home_difficulty),
            difficulty_label(self.away_difficulty)
        )
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
    snapshot.rules.home_controller = SideController::Cpu(cfg.home_difficulty);
    snapshot.rules.away_controller = SideController::Cpu(cfg.away_difficulty);

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

    let safety_cap = cfg
        .half_duration_ticks()
        .saturating_mul(5)
        .saturating_add(6_000);
    let idle = [InputFrame::IDLE, InputFrame::IDLE];
    while !sim.is_finished() && sim.state().tick < safety_cap {
        sim.tick(&idle);
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
            }
            prev_gk_owner = [false, false];
            continue;
        }
        open_ticks_total += 1;

        let ball = (f(st.ball.pos.x), f(st.ball.pos.y));
        let poss_team = st.ball.owner.map(|(s, _)| s).or(st.ball.last_touch_team);
        let carrier = st.ball.owner;

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
                if i != 0 && z > AIRBORNE_Z {
                    airborne += 1;
                }
                if let Some(pp) = prev[s][i].as_ref().filter(|pp| pp.id == p.footballer_id) {
                    a.player_ticks += 1;
                    let sp0 = (pp.vel.0.powi(2) + pp.vel.1.powi(2)).sqrt();
                    let sp1 = (vel.0.powi(2) + vel.1.powi(2)).sqrt();
                    let dv = ((vel.0 - pp.vel.0).powi(2) + (vel.1 - pp.vel.1).powi(2)).sqrt();
                    let acc_mps2 = dv * f64::from(TICKS_PER_SECOND);
                    a.accel.add(acc_mps2);
                    if acc_mps2 > 10.0 {
                        a.accel_gt10 += 1;
                    }
                    let hz = f64::from(TICKS_PER_SECOND);
                    let pv = ((pos.0 - pp.pos.0) * hz, (pos.1 - pp.pos.1) * hz);
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
                });
            }
            a.max_airborne = a.max_airborne.max(airborne);
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

        m.insert("aerial.jumps_outfield".into(), a.jumps_outfield as f64);
        m.insert("aerial.jumps_gk".into(), a.jumps_gk as f64);
        m.insert("aerial.max_airborne_outfield".into(), a.max_airborne as f64);
        m.insert("aerial.headers".into(), a.headers as f64);
        m.insert("aerial.headers_shot".into(), a.headers_shot as f64);
        m.insert("aerial.headers_clear".into(), a.headers_clear as f64);
        m.insert("aerial.headers_pass".into(), a.headers_pass as f64);
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
        ("aerial.jumps_outfield / aerial.jumps_gk", "Count of z going from <= 0 to > 0 between consecutive open-play ticks (outfielders / GK; GK dives that leave the ground count)."),
        ("aerial.max_airborne_outfield", "Max number of the side's outfielders with z > 0 on one open-play tick."),
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

const SECTIONS: [(&str, &str); 5] = [
    ("match.", "Match"),
    ("gk.", "Goalkeeper"),
    ("shape.", "Defence and shape"),
    ("move.", "Movement smoothness"),
    ("aerial.", "Aerial"),
];

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
            let cell = |v: Option<f64>| v.map_or_else(|| "-".to_string(), fmt_num);
            let delta = match (bv, av) {
                (Some(x), Some(y)) => format!("{:+.2}", y - x),
                _ => "-".to_string(),
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
[--home-difficulty easy|normal|hard] [--away-difficulty easy|normal|hard] [--preset baseline] \
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
        }],
        Some("baseline") => vec![
            AnalyzeConfig {
                format: FormatKind::FiveASide,
                seeds,
                seed_base,
                minutes,
                home_difficulty: CpuDifficulty::Hard,
                away_difficulty: CpuDifficulty::Hard,
            },
            AnalyzeConfig {
                format: FormatKind::FiveASide,
                seeds,
                seed_base,
                minutes,
                home_difficulty: CpuDifficulty::Normal,
                away_difficulty: CpuDifficulty::Normal,
            },
            AnalyzeConfig {
                format: FormatKind::ElevenASide,
                seeds,
                seed_base,
                minutes,
                home_difficulty: CpuDifficulty::Normal,
                away_difficulty: CpuDifficulty::Normal,
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
    }
}
