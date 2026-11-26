//! CPU-vs-CPU team-shape and movement-smoothness metrics.
//!
//! Runs 3 seeds x 10 match-minutes for 5v5 (Hard vs Hard) and 11v11 (Normal vs Normal) through the
//! public API only, and reports:
//! - mean distance of each role from its assigned defensive anchor while defending (all defending
//!   ticks, and "settled" ticks where the opponent has kept the ball for >= 3 s),
//! - defensive line x-spread (std-dev of the back line's x) and line-height jitter while defending,
//! - % of defending ticks with >= 2 defenders within 1.6 m of the carrier (swarming),
//! - % of outfielders goal-side of the ball while defending,
//! - presser changes per defending minute (assignment churn),
//! - per-tick heading-change histogram and acceleration reversals per player-minute (jerk proxy),
//! - goals, shots and passes per match (play is not broken).
//!
//! `cargo test --release -p psfc-sim --test team_shape -- --nocapture` prints the table.
//! `TEAM_SHAPE_SEEDS=6` widens the sample for tuning.

use psfc_sim::ai::{committed_presser, defensive_shape_anchor};
use psfc_sim::{
    CpuDifficulty, FixtureSnapshot, InputFrame, MatchPhase, MatchSimulation, PlayerRole,
    SideController, TeamSide, Vec2,
};

const HEADING_BUCKETS_DEG: [f64; 5] = [5.0, 15.0, 45.0, 90.0, 180.1];
/// Opponent possession streak after which the defending team counts as "settled".
const SETTLED_TICKS: u32 = 180;

#[derive(Default, Clone, Debug)]
struct Metrics {
    matches: u32,
    goals: u32,
    shots: u32,
    passes_att: u32,
    passes_cmp: u32,
    defending_ticks: u64,
    swarm_ticks: u64,
    behind_ball_num: u64,
    behind_ball_den: u64,
    // role index 1..=3 (Defender, Midfielder, Forward)
    role_anchor_sum_m: [f64; 4],
    role_anchor_n: [u64; 4],
    settled_role_sum_m: [f64; 4],
    settled_role_n: [u64; 4],
    line_spread_sum_m: f64,
    line_jitter_sum_m: f64,
    line_n: u64,
    presser_changes: u64,
    heading_hist: [u64; 5],
    accel_reversals: u64,
    sharp_turns: u64,
    player_ticks: u64,
    // Renderer-facing jitter (outfielders, open play)
    accel_hist: Vec<u64>,
    pos2_hist: Vec<u64>,
    moving_samples: u64,
    dv_dir_reversals: u64,
    facing_hist: Vec<u64>,
    facing_samples: u64,
    facing_reversals: u64,
    facing_jumps: u64,
    idle_samples: u64,
    idle_twitch: u64,
}

const ACC_BIN: f64 = 0.5; // m/s^2
const ACC_BINS: usize = 160;
const FACE_BIN: f64 = 0.25; // degrees
const FACE_BINS: usize = 721;

fn bump(h: &mut Vec<u64>, n: usize, v: f64, bin: f64) {
    if h.len() != n {
        *h = vec![0; n];
    }
    let i = ((v / bin) as usize).min(n - 1);
    h[i] += 1;
}

fn pct(h: &[u64], q: f64, bin: f64) -> f64 {
    let tot: u64 = h.iter().sum();
    if tot == 0 {
        return 0.0;
    }
    let target = (tot as f64 * q).ceil() as u64;
    let mut acc = 0;
    for (i, c) in h.iter().enumerate() {
        acc += c;
        if acc >= target {
            return (i as f64 + 0.5) * bin;
        }
    }
    h.len() as f64 * bin
}

#[derive(Debug, Clone, Copy)]
struct Summary {
    role_anchor_m: [f64; 4],
    settled_anchor_m: [f64; 4],
    line_spread_m: f64,
    line_jitter_mm_per_tick: f64,
    swarm_pct: f64,
    behind_ball_pct: f64,
    presser_changes_per_min: f64,
    heading_pct: [f64; 5],
    accel_reversals_per_pmin: f64,
    sharp_turns_per_pmin: f64,
    goals_per_match: f64,
    shots_per_match: f64,
    passes_per_match: f64,
    pass_pct: f64,
    accel_p50: f64,
    accel_p95: f64,
    pos2_p50: f64,
    pos2_p95: f64,
    dv_dir_rev_per_k: f64,
    facing_p95_deg: f64,
    facing_rev_per_k: f64,
    facing_jump_per_k: f64,
    idle_twitch_pct: f64,
}

fn f(v: psfc_sim::Fp) -> f64 {
    v.to_milli() as f64 / 1000.0
}

fn role_slot(r: PlayerRole) -> usize {
    match r {
        PlayerRole::Goalkeeper => 0,
        PlayerRole::Defender => 1,
        PlayerRole::Midfielder => 2,
        PlayerRole::Forward => 3,
    }
}

fn angle_deg(a: Vec2, b: Vec2) -> f64 {
    let (ax, ay, bx, by) = (f(a.x), f(a.y), f(b.x), f(b.y));
    (ax * by - ay * bx).atan2(ax * bx + ay * by).abs().to_degrees()
}

fn run_match(snap: &FixtureSnapshot, m: &mut Metrics) {
    let mut sim = MatchSimulation::new(snap).unwrap();
    let n_side = snap.format.players_per_side;
    let line_k = if n_side <= 6 { 2 } else { 4 };
    let mut prev_vel: [Vec<Vec2>; 2] = [vec![Vec2::ZERO; n_side], vec![Vec2::ZERO; n_side]];
    let mut prev_dv: [Vec<Vec2>; 2] = [vec![Vec2::ZERO; n_side], vec![Vec2::ZERO; n_side]];
    let mut prev_pos: [Vec<(Vec2, Vec2)>; 2] = [vec![(Vec2::ZERO, Vec2::ZERO); n_side], vec![(Vec2::ZERO, Vec2::ZERO); n_side]];
    let mut prev_face: [Vec<(u16, f64)>; 2] = [vec![(0, 0.0); n_side], vec![(0, 0.0); n_side]];
    let mut open_run = 0_u32;
    let mut prev_line: [Option<f64>; 2] = [None, None];
    let mut prev_presser: [Option<usize>; 2] = [None, None];
    let mut owner_side: Option<TeamSide> = None;
    let mut streak = 0_u32;
    let mut guard = 0_u32;
    while !sim.is_finished() && guard < 200_000 {
        guard += 1;
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
        let st = sim.state();
        let open = matches!(st.phase, MatchPhase::OpenPlay);
        open_run = if open { open_run + 1 } else { 0 };
        match st.ball.owner {
            Some((os, _)) if Some(os) == owner_side => streak += 1,
            Some((os, _)) => {
                owner_side = Some(os);
                streak = 0;
            }
            None => {}
        }
        if !open {
            owner_side = None;
            streak = 0;
        }
        for side in [TeamSide::Home, TeamSide::Away] {
            let s = side.index();
            let sign = side.attack_sign(st.second_half_swapped()) as f64;
            for (i, p) in st.players[s].iter().enumerate().skip(1) {
                if p.sent_off || i >= n_side {
                    continue;
                }
                let v = p.vel;
                let dv = v - prev_vel[s][i];
                if open {
                    m.player_ticks += 1;
                    let sp = f(v.len());
                    if sp > 1.0 && f(prev_vel[s][i].len()) > 1.0 {
                        let a = angle_deg(prev_vel[s][i], v);
                        let b = HEADING_BUCKETS_DEG.iter().position(|&lim| a < lim).unwrap_or(4);
                        m.heading_hist[b] += 1;
                        if a > 45.0 && sp > 2.0 {
                            m.sharp_turns += 1;
                        }
                    }
                    if f(dv.len()) > 0.08
                        && f(prev_dv[s][i].len()) > 0.08
                        && f(dv.dot(prev_dv[s][i])) < 0.0
                    {
                        m.accel_reversals += 1;
                    }
                    if open_run > 3 {
                        bump(&mut m.accel_hist, ACC_BINS, f(dv.len()) * 60.0, ACC_BIN);
                        let (p1, p2) = prev_pos[s][i];
                        let d2 = p.pos - p1.scale(psfc_sim::Fp::from_int(2)) + p2;
                        bump(&mut m.pos2_hist, ACC_BINS, f(d2.len()) * 3600.0, ACC_BIN);
                        if sp > 1.0 {
                            m.moving_samples += 1;
                            if f(dv.len()) > 0.005
                                && f(prev_dv[s][i].len()) > 0.005
                                && f(dv.dot(prev_dv[s][i])) < 0.0
                            {
                                m.dv_dir_reversals += 1;
                            }
                        }
                        let (pf, pdf) = prev_face[s][i];
                        let df = f64::from(psfc_sim::math::angle_diff_bam(pf, p.facing_bam)) * 360.0 / 65_536.0;
                        m.facing_samples += 1;
                        bump(&mut m.facing_hist, FACE_BINS, df.abs(), FACE_BIN);
                        if df.abs() > 0.5 && pdf.abs() > 0.5 && df * pdf < 0.0 {
                            m.facing_reversals += 1;
                        }
                        if df.abs() > 30.0 {
                            m.facing_jumps += 1;
                        }
                        if sp < 0.3 {
                            m.idle_samples += 1;
                            if df.abs() > 4.0 {
                                m.idle_twitch += 1;
                            }
                        }
                    }
                }
                let df_now = f64::from(psfc_sim::math::angle_diff_bam(prev_face[s][i].0, p.facing_bam)) * 360.0 / 65_536.0;
                prev_face[s][i] = (p.facing_bam, df_now);
                prev_pos[s][i] = (p.pos, prev_pos[s][i].0);
                prev_vel[s][i] = v;
                prev_dv[s][i] = dv;
            }

            let defending = open && matches!(st.ball.owner, Some((o, _)) if o != side);
            if !defending {
                prev_line[s] = None;
                prev_presser[s] = None;
                continue;
            }
            let settled = streak >= SETTLED_TICKS;
            m.defending_ticks += 1;
            let (o_side, o_idx) = st.ball.owner.unwrap();
            let carrier = st.players[o_side.index()][o_idx].pos;
            let ball_proj = f(st.ball.pos.x) * sign;
            let ctrl = st.controlled_idx[s];

            let presser = committed_presser(st, side);
            if prev_presser[s].is_some() && presser.is_some() && presser != prev_presser[s] {
                m.presser_changes += 1;
            }
            if presser.is_some() {
                prev_presser[s] = presser;
            }

            let mut near_carrier = 0;
            let mut nearest_ai: Option<(f64, usize)> = None;
            for (i, p) in st.players[s].iter().enumerate().skip(1) {
                if p.sent_off {
                    continue;
                }
                let d = f(p.pos.dist(carrier));
                if d <= 1.6 {
                    near_carrier += 1;
                }
                m.behind_ball_den += 1;
                if f(p.pos.x) * sign < ball_proj {
                    m.behind_ball_num += 1;
                }
                if i != ctrl && nearest_ai.is_none_or(|(bd, _)| d < bd) {
                    nearest_ai = Some((d, i));
                }
            }
            if near_carrier >= 2 {
                m.swarm_ticks += 1;
            }
            // Shape holders: AI outfielders other than the CPU-controlled player and the AI player
            // nearest the ball (whoever is pressing).
            for (i, p) in st.players[s].iter().enumerate().skip(1) {
                if p.sent_off || i == ctrl || nearest_ai.is_some_and(|(_, ni)| ni == i) {
                    continue;
                }
                let d = f(p.pos.dist(defensive_shape_anchor(st, side, i)));
                let r = role_slot(p.role);
                m.role_anchor_sum_m[r] += d;
                m.role_anchor_n[r] += 1;
                if settled {
                    m.settled_role_sum_m[r] += d;
                    m.settled_role_n[r] += 1;
                }
            }
            // Back line = `line_k` deepest outfielders.
            let mut xs: Vec<f64> = st.players[s]
                .iter()
                .skip(1)
                .filter(|p| !p.sent_off)
                .map(|p| f(p.pos.x) * sign)
                .collect();
            xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
            xs.truncate(line_k);
            let mean = xs.iter().sum::<f64>() / xs.len() as f64;
            let var = xs.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / xs.len() as f64;
            m.line_spread_sum_m += var.sqrt();
            if let Some(pl) = prev_line[s] {
                m.line_jitter_sum_m += (mean - pl).abs();
            }
            prev_line[s] = Some(mean);
            m.line_n += 1;
        }
    }
    assert!(sim.is_finished(), "match did not finish");
    let st = sim.state();
    m.matches += 1;
    m.goals += u32::from(st.score[0]) + u32::from(st.score[1]);
    for side in [TeamSide::Home, TeamSide::Away] {
        for ps in st.combined_player_stats(side) {
            m.shots += u32::from(ps.shots);
            m.passes_att += u32::from(ps.passes_attempted);
            m.passes_cmp += u32::from(ps.passes_completed);
        }
    }
}

fn summarize(label: &str, m: &Metrics) -> Summary {
    let avg = |sum: &[f64; 4], n: &[u64; 4]| -> [f64; 4] {
        let mut out = [0.0; 4];
        for r in 1..4 {
            out[r] = sum[r] / (n[r].max(1) as f64);
        }
        out
    };
    let hist_total: u64 = m.heading_hist.iter().sum();
    let mut heading_pct = [0.0; 5];
    for (i, h) in m.heading_hist.iter().enumerate() {
        heading_pct[i] = 100.0 * (*h as f64) / (hist_total.max(1) as f64);
    }
    let pmin = (m.player_ticks as f64) / 3600.0;
    let def_min = (m.defending_ticks as f64) / 3600.0;
    let matches = m.matches.max(1) as f64;
    let s = Summary {
        role_anchor_m: avg(&m.role_anchor_sum_m, &m.role_anchor_n),
        settled_anchor_m: avg(&m.settled_role_sum_m, &m.settled_role_n),
        line_spread_m: m.line_spread_sum_m / (m.line_n.max(1) as f64),
        line_jitter_mm_per_tick: 1000.0 * m.line_jitter_sum_m / (m.line_n.max(1) as f64),
        swarm_pct: 100.0 * (m.swarm_ticks as f64) / (m.defending_ticks.max(1) as f64),
        behind_ball_pct: 100.0 * (m.behind_ball_num as f64) / (m.behind_ball_den.max(1) as f64),
        presser_changes_per_min: (m.presser_changes as f64) / def_min.max(1e-9),
        heading_pct,
        accel_reversals_per_pmin: (m.accel_reversals as f64) / pmin.max(1e-9),
        sharp_turns_per_pmin: (m.sharp_turns as f64) / pmin.max(1e-9),
        goals_per_match: (m.goals as f64) / matches,
        shots_per_match: (m.shots as f64) / matches,
        passes_per_match: (m.passes_att as f64) / matches,
        pass_pct: 100.0 * (m.passes_cmp as f64) / (m.passes_att.max(1) as f64),
        accel_p50: pct(&m.accel_hist, 0.5, ACC_BIN),
        accel_p95: pct(&m.accel_hist, 0.95, ACC_BIN),
        pos2_p50: pct(&m.pos2_hist, 0.5, ACC_BIN),
        pos2_p95: pct(&m.pos2_hist, 0.95, ACC_BIN),
        dv_dir_rev_per_k: 1000.0 * (m.dv_dir_reversals as f64) / (m.moving_samples.max(1) as f64),
        facing_p95_deg: pct(&m.facing_hist, 0.95, FACE_BIN),
        facing_rev_per_k: 1000.0 * (m.facing_reversals as f64) / (m.facing_samples.max(1) as f64),
        facing_jump_per_k: 1000.0 * (m.facing_jumps as f64) / (m.facing_samples.max(1) as f64),
        idle_twitch_pct: 100.0 * (m.idle_twitch as f64) / (m.idle_samples.max(1) as f64),
    };
    println!("=== {label} ({} matches) ===", m.matches);
    println!(
        "anchor dist while defending (m): DEF {:.2}  MID {:.2}  FWD {:.2} | settled (opp >= 3 s on ball): DEF {:.2}  MID {:.2}  FWD {:.2}",
        s.role_anchor_m[1], s.role_anchor_m[2], s.role_anchor_m[3],
        s.settled_anchor_m[1], s.settled_anchor_m[2], s.settled_anchor_m[3]
    );
    println!(
        "back line x-spread {:.2} m | line-height jitter {:.1} mm/tick | presser changes / defending-min {:.1}",
        s.line_spread_m, s.line_jitter_mm_per_tick, s.presser_changes_per_min
    );
    println!(
        "swarming (>=2 within 1.6m of carrier) {:.2}% | outfielders goal-side of ball {:.1}%",
        s.swarm_pct, s.behind_ball_pct
    );
    println!(
        "heading change/tick: <5deg {:.1}% | 5-15 {:.1}% | 15-45 {:.2}% | 45-90 {:.2}% | >90 {:.2}%",
        s.heading_pct[0], s.heading_pct[1], s.heading_pct[2], s.heading_pct[3], s.heading_pct[4]
    );
    println!(
        "accel reversals / player-min {:.1} | sharp turns (>45deg/tick @>2m/s) / player-min {:.2}",
        s.accel_reversals_per_pmin, s.sharp_turns_per_pmin
    );
    println!(
        "|dv|/dt p50 {:.1} p95 {:.1} m/s^2 | pos 2nd diff p50 {:.1} p95 {:.1} m/s^2 | dv-direction reversals @>1m/s {:.1}/1000",
        s.accel_p50, s.accel_p95, s.pos2_p50, s.pos2_p95, s.dv_dir_rev_per_k
    );
    println!(
        "facing change p95 {:.1} deg/tick | facing reversals {:.1}/1000 | >30deg jumps {:.2}/1000 | idle twitch (>4deg) {:.2}%",
        s.facing_p95_deg, s.facing_rev_per_k, s.facing_jump_per_k, s.idle_twitch_pct
    );
    println!(
        "goals / match {:.2} | shots / match {:.2} | passes / match {:.0} ({:.0}% completed)",
        s.goals_per_match, s.shots_per_match, s.passes_per_match, s.pass_pct
    );
    s
}

fn seeds() -> Vec<u64> {
    const ALL: [u64; 8] = [11, 2026, 90_210, 7, 31_337, 424_242, 5, 123_456];
    let n = std::env::var("TEAM_SHAPE_SEEDS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(3)
        .clamp(1, ALL.len());
    ALL[..n].to_vec()
}

fn run_suite(eleven: bool) -> Summary {
    let mut m = Metrics::default();
    for seed in seeds() {
        let mut snap = if eleven {
            FixtureSnapshot::sample_eleven_a_side(seed)
        } else {
            FixtureSnapshot::sample_five_a_side(seed)
        };
        let d = if eleven { CpuDifficulty::Normal } else { CpuDifficulty::Hard };
        snap.rules.home_controller = SideController::Cpu(d);
        snap.rules.away_controller = SideController::Cpu(d);
        snap.rules.half_duration_ticks = 5 * 60 * 60;
        run_match(&snap, &mut m);
    }
    summarize(if eleven { "11v11 Normal vs Normal" } else { "5v5 Hard vs Hard" }, &m)
}

fn check(cond: bool, what: &str, v: f64, failures: &mut Vec<String>) {
    if !cond {
        failures.push(format!("{what} (got {v:.2})"));
    }
}

/// Bounds below were set from the post-change measurement with a safety margin; the 10e07a3
/// baseline fails the movement and line bounds (see `docs/team-shape-ai.md` for both tables).
#[test]
fn team_shape_5v5_hard_vs_hard() {
    let s = run_suite(false);
    let mut fails = Vec::new();
    // Movement smoothness (baseline: 169 reversals/min, 63/1000 dv flips, 26 m/s^2 median, 61/1000 facing flips).
    check(s.accel_reversals_per_pmin < 100.0, "accel reversals / player-min < 100", s.accel_reversals_per_pmin, &mut fails);
    check(s.dv_dir_rev_per_k < 40.0, "dv-direction reversals < 40/1000", s.dv_dir_rev_per_k, &mut fails);
    check(s.accel_p50 < 16.0, "median |dv|/dt < 16 m/s^2", s.accel_p50, &mut fails);
    check(s.facing_rev_per_k < 40.0, "facing reversals < 40/1000", s.facing_rev_per_k, &mut fails);
    check(s.facing_p95_deg < 19.0, "facing change p95 < 19 deg/tick", s.facing_p95_deg, &mut fails);
    check(s.idle_twitch_pct < 1.0, "idle facing twitch < 1%", s.idle_twitch_pct, &mut fails);
    // Shape (baseline: 57% goal-side, 73 mm/tick line jitter).
    check(s.behind_ball_pct > 60.0, "outfielders goal-side of ball > 60%", s.behind_ball_pct, &mut fails);
    check(s.line_jitter_mm_per_tick < 65.0, "line-height jitter < 65 mm/tick", s.line_jitter_mm_per_tick, &mut fails);
    check(s.swarm_pct < 3.5, "swarming < 3.5%", s.swarm_pct, &mut fails);
    check(s.settled_anchor_m[1] < 4.5, "settled DEF anchor dist < 4.5 m", s.settled_anchor_m[1], &mut fails);
    check(s.settled_anchor_m[2] < 4.5, "settled MID anchor dist < 4.5 m", s.settled_anchor_m[2], &mut fails);
    check(s.settled_anchor_m[3] < 5.5, "settled FWD anchor dist < 5.5 m", s.settled_anchor_m[3], &mut fails);
    check(s.presser_changes_per_min < 3.0, "presser changes / defending-min < 3", s.presser_changes_per_min, &mut fails);
    // Play is not broken.
    check(s.goals_per_match >= 1.0, "goals / match >= 1", s.goals_per_match, &mut fails);
    check(s.shots_per_match >= 10.0, "shots / match >= 10", s.shots_per_match, &mut fails);
    check(s.pass_pct >= 70.0, "pass completion >= 70%", s.pass_pct, &mut fails);
    assert!(fails.is_empty(), "5v5 team-shape bounds failed: {fails:#?}");
}

#[test]
fn team_shape_11v11_normal_vs_normal() {
    let s = run_suite(true);
    let mut fails = Vec::new();
    // Movement smoothness (baseline: 116 reversals/min, 49/1000 dv flips, 21 m/s^2 median, 39/1000 facing flips).
    check(s.accel_reversals_per_pmin < 50.0, "accel reversals / player-min < 50", s.accel_reversals_per_pmin, &mut fails);
    check(s.dv_dir_rev_per_k < 25.0, "dv-direction reversals < 25/1000", s.dv_dir_rev_per_k, &mut fails);
    check(s.accel_p50 < 12.0, "median |dv|/dt < 12 m/s^2", s.accel_p50, &mut fails);
    check(s.facing_rev_per_k < 25.0, "facing reversals < 25/1000", s.facing_rev_per_k, &mut fails);
    check(s.facing_p95_deg < 16.5, "facing change p95 < 16.5 deg/tick", s.facing_p95_deg, &mut fails);
    check(s.idle_twitch_pct < 1.5, "idle facing twitch < 1.5%", s.idle_twitch_pct, &mut fails);
    // Shape (baseline: 68% goal-side, 65 mm/tick line jitter).
    check(s.behind_ball_pct > 65.0, "outfielders goal-side of ball > 65%", s.behind_ball_pct, &mut fails);
    check(s.line_jitter_mm_per_tick < 58.0, "line-height jitter < 58 mm/tick", s.line_jitter_mm_per_tick, &mut fails);
    check(s.swarm_pct < 2.0, "swarming < 2%", s.swarm_pct, &mut fails);
    check(s.settled_anchor_m[1] < 4.0, "settled DEF anchor dist < 4 m", s.settled_anchor_m[1], &mut fails);
    check(s.settled_anchor_m[2] < 5.0, "settled MID anchor dist < 5 m", s.settled_anchor_m[2], &mut fails);
    check(s.settled_anchor_m[3] < 6.0, "settled FWD anchor dist < 6 m", s.settled_anchor_m[3], &mut fails);
    check(s.presser_changes_per_min < 3.0, "presser changes / defending-min < 3", s.presser_changes_per_min, &mut fails);
    // Play is not broken.
    check(s.goals_per_match > 0.3, "goals / match > 0.3", s.goals_per_match, &mut fails);
    check(s.shots_per_match >= 5.0, "shots / match >= 5", s.shots_per_match, &mut fails);
    check(s.pass_pct >= 70.0, "pass completion >= 70%", s.pass_pct, &mut fails);
    assert!(fails.is_empty(), "11v11 team-shape bounds failed: {fails:#?}");
}

#[test]
fn formation_shapes_have_distinct_attacking_and_defending_positions() {
    use psfc_sim::config::{formation_shape, SlotRole};
    let s433 = formation_shape("4-3-3", 11);
    assert_eq!(s433.len(), 10);
    for slot in &s433 {
        if slot.role == SlotRole::FullBack {
            assert!(slot.defend_y.abs() < slot.attack_y.abs(), "full-back tucks in when defending");
            assert!(slot.attack_depth > slot.defend_depth + 300, "full-back overlaps when attacking");
        }
        if slot.role == SlotRole::CentreBack {
            assert_eq!(slot.defend_depth, 0, "centre-back holds the line");
        }
        if slot.role == SlotRole::Striker {
            assert_eq!(slot.attack_depth, 1_000, "striker leads the line");
        }
    }
    let holding = s433
        .iter()
        .find(|s| s.role == SlotRole::Holding)
        .expect("4-3-3 has a holding mid");
    let cm = s433.iter().find(|s| s.role == SlotRole::CentralMid).unwrap();
    assert!(holding.defend_depth < cm.defend_depth, "holding mid screens in front of the CBs");
    let s442 = formation_shape("4-4-2", 11);
    let wm = s442
        .iter()
        .find(|s| s.role == SlotRole::WideMid)
        .expect("4-4-2 has wide mids");
    assert!(wm.defend_y.abs() >= 700, "wide mids hold width when defending");
    let s4231 = formation_shape("4-2-3-1", 11);
    assert_eq!(s4231.iter().filter(|s| s.role == SlotRole::Holding).count(), 2);
    let diamond = formation_shape("1-2-1", 5);
    assert_eq!(diamond[0].role, SlotRole::Fixo);
    assert_eq!((diamond[0].attack_depth, diamond[0].defend_depth), (0, 0), "fixo stays deepest");
    assert_eq!(formation_shape("not-a-formation", 11).len(), 10);
}

#[test]
fn formation_anchors_are_assigned_by_role_not_list_order() {
    let snap = FixtureSnapshot::sample_eleven_a_side(1);
    let sim = MatchSimulation::new(&snap).unwrap();
    for p in sim.state().players[0].iter().skip(1) {
        let x = p.base_anchor_permille.0;
        match p.role {
            PlayerRole::Defender => assert!(x <= -500, "{} anchored at {x}", p.name),
            PlayerRole::Midfielder => assert!((-400..=0).contains(&x), "{} anchored at {x}", p.name),
            PlayerRole::Forward => assert!(x >= 300, "{} anchored at {x}", p.name),
            PlayerRole::Goalkeeper => {}
        }
    }
}

#[test]
fn movement_model_caps_acceleration_braking_turning_and_facing() {
    use psfc_sim::ai::{limit_facing_turn, limit_turn_rate, realistic_velocity_step};
    let a = FixtureSnapshot::sample_five_a_side(1).home.starters[1].attributes;
    let mm = |x: i64, y: i64| Vec2::from_milli(x, y);
    // From rest: one tick of at most (6 + 0.03 * acceleration) m/s^2.
    let v = realistic_velocity_step(Vec2::ZERO, mm(7_000, 0), &a, true);
    let cap = (6_000 + a.acceleration as i64 * 30) / 60;
    assert!(v.len().to_milli() <= cap + 2 && v.len().to_milli() > 0);
    // Braking at most (9 + 0.03 * acceleration) m/s^2.
    let v = realistic_velocity_step(mm(7_000, 0), Vec2::ZERO, &a, true);
    let dec = (9_000 + a.acceleration as i64 * 30) / 60;
    assert!((7_000 - v.len().to_milli() - dec).abs() <= 3);
    // A 90 degree request at 7 m/s turns only a few degrees this tick and never overshoots.
    let v = realistic_velocity_step(mm(7_000, 0), mm(0, 7_000), &a, true);
    let turned = (v.y.to_milli() as f64).atan2(v.x.to_milli() as f64).to_degrees();
    assert!(turned > 0.5 && turned < 6.0, "turned {turned} deg");
    // Human first-tick response is never limited (below 4.5 m/s).
    let nv = mm(-300, 400);
    assert_eq!(limit_turn_rate(mm(2_000, 0), nv, &a, true), nv);
    // Idle facing deadband (< 4 deg ignored) and a bounded pivot at speed.
    assert_eq!(limit_facing_turn(0, 500, Vec2::ZERO, &a), 0);
    let f = limit_facing_turn(0, 16_384, mm(8_000, 0), &a);
    let deg = f64::from(f) * 360.0 / 65_536.0;
    assert!(deg > 6.0 && deg < 14.0, "facing step at sprint {deg} deg");
}

#[test]
fn ai_memory_survives_rollback_bit_exactly_in_11v11() {
    let mut snap = FixtureSnapshot::sample_eleven_a_side(4242);
    snap.rules.home_controller = SideController::Cpu(CpuDifficulty::Normal);
    snap.rules.away_controller = SideController::Cpu(CpuDifficulty::Hard);
    let mut sim = MatchSimulation::new(&snap).unwrap();
    for _ in 0..900 {
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
    }
    let saved = sim.save_state().unwrap();
    for _ in 0..600 {
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
    }
    let target = sim.state_hash();
    sim.load_state(&saved).unwrap();
    for _ in 0..600 {
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
    }
    assert_eq!(sim.state_hash(), target);
    // A state saved before this field existed (no `ai_memory` key) still loads.
    let mut v: serde_json::Value = serde_json::from_slice(&saved).unwrap();
    v.as_object_mut().unwrap().remove("ai_memory");
    sim.load_state(&serde_json::to_vec(&v).unwrap()).unwrap();
    sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
}
