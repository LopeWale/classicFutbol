//! Player-vs-player contact metrics (CPU-vs-CPU) through the public API only.
//!
//! `cargo test --release -p psfc-sim --test avoidance -- --nocapture` prints the table.

use psfc_sim::{
    CpuDifficulty, FixtureSnapshot, InputFrame, MatchPhase, MatchSimulation, SideController,
};

const BODY_SUM_M: f64 = 0.8;

fn f(v: psfc_sim::Fp) -> f64 {
    v.raw() as f64 / 4_294_967_296.0
}

#[derive(Default, Debug, Clone)]
struct ContactMetrics {
    open_ticks: u64,
    corr: Vec<f64>,
    overlap_pair_ticks: u64,
    collision_onsets: u64,
    stuck_pairs: u64,
    stuck_pairs_slow: u64,
    max_overlap_m: f64,
}

impl ContactMetrics {
    fn minutes(&self) -> f64 {
        self.open_ticks as f64 / 3_600.0
    }
    fn pctl(&self, q: f64) -> f64 {
        if self.corr.is_empty() {
            return 0.0;
        }
        let mut v = self.corr.clone();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        v[((v.len() - 1) as f64 * q) as usize]
    }
}

fn run_match(snap: &FixtureSnapshot, m: &mut ContactMetrics) {
    let mut sim = MatchSimulation::new(snap).unwrap();
    let mut prev: Option<Vec<(f64, f64)>> = None;
    let mut streak: Vec<u32> = Vec::new();
    let mut slow_streak: Vec<u32> = Vec::new();
    let mut guard = 0_u32;
    while !sim.is_finished() && guard < 200_000 {
        guard += 1;
        let was_open = matches!(sim.state().phase, MatchPhase::OpenPlay);
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
        let st = sim.state();
        let open = was_open && matches!(st.phase, MatchPhase::OpenPlay);
        let mut flat = Vec::new();
        for side in &st.players {
            for p in side {
                flat.push(p);
            }
        }
        let pos: Vec<(f64, f64)> = flat.iter().map(|p| (f(p.pos.x), f(p.pos.y))).collect();
        let n = flat.len();
        if streak.len() != n * n {
            streak = vec![0; n * n];
            slow_streak = vec![0; n * n];
        }
        if !open {
            prev = None;
            streak.iter_mut().for_each(|s| *s = 0);
            slow_streak.iter_mut().for_each(|s| *s = 0);
            continue;
        }
        m.open_ticks += 1;
        if let Some(pp) = &prev {
            for (i, p) in flat.iter().enumerate() {
                if p.sent_off {
                    continue;
                }
                let dx = (pos[i].0 - pp[i].0) * 60.0 - f(p.vel.x);
                let dy = (pos[i].1 - pp[i].1) * 60.0 - f(p.vel.y);
                m.corr.push((dx * dx + dy * dy).sqrt());
            }
        }
        for a in 0..n {
            for b in (a + 1)..n {
                if flat[a].sent_off || flat[b].sent_off {
                    continue;
                }
                let d = ((pos[a].0 - pos[b].0).powi(2) + (pos[a].1 - pos[b].1).powi(2)).sqrt();
                let k = a * n + b;
                if d < BODY_SUM_M {
                    m.overlap_pair_ticks += 1;
                    m.max_overlap_m = m.max_overlap_m.max(BODY_SUM_M - d);
                    if streak[k] == 0 {
                        m.collision_onsets += 1;
                    }
                    streak[k] += 1;
                    if std::env::var("AVOID_TRACE").is_ok() && streak[k] <= 12 {
                        let cls = psfc_sim::avoidance::priority_classes(st);
                        let fl: Vec<u8> = cls[0].iter().chain(cls[1].iter()).copied().collect();
                        println!(
                            "trace k{k} s{} d {:.3} a {}:{} c{} p({:.2},{:.2}) v({:.2},{:.2}) {:?} | b {}:{} c{} p({:.2},{:.2}) v({:.2},{:.2}) {:?} own {:?}",
                            streak[k], d, flat[a].team.index(), flat[a].index, fl[a], pos[a].0, pos[a].1, f(flat[a].vel.x), f(flat[a].vel.y), flat[a].action,
                            flat[b].team.index(), flat[b].index, fl[b], pos[b].0, pos[b].1, f(flat[b].vel.x), f(flat[b].vel.y), flat[b].action, st.ball.owner
                        );
                    }
                    if streak[k] == 11 {
                        m.stuck_pairs += 1;
                        if std::env::var("AVOID_DEBUG").is_ok() {
                            let cls = psfc_sim::avoidance::priority_classes(st);
                            let fl: Vec<u8> = cls[0].iter().chain(cls[1].iter()).copied().collect();
                            let own = st.ball.owner;
                            println!(
                                "stuck {}:{} {:?} c{} v{:.1} | {}:{} {:?} c{} v{:.1} | d {:.2} owner {:?} phase {:?}",
                                flat[a].team.index(), flat[a].index, flat[a].action, fl[a], f(flat[a].vel.len()),
                                flat[b].team.index(), flat[b].index, flat[b].action, fl[b], f(flat[b].vel.len()),
                                d, own, st.phase
                            );
                        }
                    }
                    let rv = flat[a].vel - flat[b].vel;
                    if f(rv.len()) < 0.5 {
                        slow_streak[k] += 1;
                        if slow_streak[k] == 11 {
                            m.stuck_pairs_slow += 1;
                        }
                    } else {
                        slow_streak[k] = 0;
                    }
                } else {
                    streak[k] = 0;
                    slow_streak[k] = 0;
                }
            }
        }
        prev = Some(pos);
    }
}

fn run_suite(eleven: bool) -> ContactMetrics {
    let mut m = ContactMetrics::default();
    for seed in [11_u64, 2026, 90_210] {
        let mut snap = if eleven {
            FixtureSnapshot::sample_eleven_a_side(seed)
        } else {
            FixtureSnapshot::sample_five_a_side(seed)
        };
        let d = if eleven {
            CpuDifficulty::Normal
        } else {
            CpuDifficulty::Hard
        };
        snap.rules.home_controller = SideController::Cpu(d);
        snap.rules.away_controller = SideController::Cpu(d);
        snap.rules.half_duration_ticks = 5 * 60 * 60;
        run_match(&snap, &mut m);
    }
    let label = if eleven { "11v11 Normal" } else { "5v5 Hard" };
    println!(
        "{label}: corr p50 {:.3} p99 {:.3} p99.9 {:.3} max {:.3} m/s, >0.3 m/s {:.3}% | overlaps {:.1} pair-ticks/min | collisions {:.2}/min | stuck>10t {} (slow {}) | max overlap {:.3} m | {:.1} open min",
        m.pctl(0.5),
        m.pctl(0.99),
        m.pctl(0.999),
        m.pctl(1.0),
        100.0 * m.corr.iter().filter(|&&c| c > 0.3).count() as f64 / m.corr.len().max(1) as f64,
        m.overlap_pair_ticks as f64 / m.minutes(),
        m.collision_onsets as f64 / m.minutes(),
        m.stuck_pairs,
        m.stuck_pairs_slow,
        m.max_overlap_m,
        m.minutes()
    );
    m
}

/// Before this change (2dd0e3b, same harness): 5v5 Hard corr p99 0.145 m/s, 53.8 collisions/min,
/// 96 overlaps > 10 ticks (14 slow); 11v11 Normal 50.2 collisions/min, 77 (12 slow).
#[test]
fn contact_metrics_5v5_and_11v11() {
    for m in [run_suite(false), run_suite(true)] {
        // Push-out never writes velocity and is capped: corrections stay small.
        assert!(
            m.pctl(0.99) < 0.3,
            "position-correction p99 {:.3} m/s",
            m.pctl(0.99)
        );
        // Stuck pairs (report definition: overlapping > 10 ticks with relative speed < 0.5 m/s).
        // Engagers (class <= 1) do not avoid the opposing carrier, so a presser can sit on the
        // carrier's shoulder at low relative speed (a jostle, resolved by the contact rules), which
        // this metric cannot tell from a genuinely stuck pair: allow a handful per 30 minutes.
        assert!(m.stuck_pairs_slow <= 3, "stuck pairs: {}", m.stuck_pairs_slow);
        // Any overlap lasting > 10 ticks (mostly restart placements resolving): rare.
        assert!(
            m.stuck_pairs <= 10,
            "overlaps > 10 ticks: {}",
            m.stuck_pairs
        );
        assert!(
            m.collision_onsets as f64 / m.minutes() < 10.0,
            "collisions / min"
        );
    }
}

fn open_play_sim(home_human: bool, seed: u64) -> MatchSimulation {
    let mut snap = FixtureSnapshot::sample_five_a_side(seed);
    snap.rules.home_controller = if home_human {
        SideController::Human
    } else {
        SideController::Cpu(CpuDifficulty::Hard)
    };
    snap.rules.away_controller = SideController::Cpu(CpuDifficulty::Hard);
    let mut sim = MatchSimulation::new(&snap).unwrap();
    let mut n = 0;
    while n < 20_000 {
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
        n += 1;
        let st = sim.state();
        if n > 600
            && matches!(st.phase, MatchPhase::OpenPlay)
            && matches!(st.ball.owner, Some((psfc_sim::TeamSide::Away, _)))
        {
            break;
        }
    }
    sim
}

#[test]
fn human_first_tick_response_unchanged_by_neighbour() {
    let mut sim = open_play_sim(true, 7);
    let saved = sim.save_state().unwrap();
    let ci = sim.state().controlled_idx[0];
    assert_ne!(ci, 0);
    let stick = InputFrame {
        move_x: 127,
        ..InputFrame::IDLE
    };
    // Neighbour: the away outfielder farthest from the controlled player (never the carrier).
    let st = sim.state();
    let me_pos = st.players[0][ci].pos;
    let owner = st.ball.owner.map(|(_, i)| i);
    let other = (1..st.players[1].len())
        .filter(|&i| Some(i) != owner)
        .max_by_key(|&i| st.players[1][i].pos.dist(me_pos).raw())
        .unwrap();
    assert!(st.players[1][other].pos.dist(me_pos) > Fp::from_int(4));
    let mut run = |neighbour_dx_mm: Option<i64>| -> (Vec2, usize) {
        let mut v: serde_json::Value = serde_json::from_slice(&saved).unwrap();
        let px = v["players"][0][ci]["pos"]["x"].as_i64().unwrap();
        let py = v["players"][0][ci]["pos"]["y"].as_i64().unwrap();
        v["players"][0][ci]["vel"]["x"] = 0.into();
        v["players"][0][ci]["vel"]["y"] = 0.into();
        // Nearest away outfielder placed ahead of the stick direction (inside the avoidance range,
        // outside body contact) or far away.
        // Manual switching so the controlled player stays the same in both runs.
        v["control_settings"][0]["auto_switch_mode"] = "Manual".into();
        v["switch_cooldown"][0] = 200.into();
        if let Some(dx) = neighbour_dx_mm {
            v["players"][1][other]["pos"]["x"] = (px + Fp::from_milli(dx).raw()).into();
            v["players"][1][other]["pos"]["y"] = py.into();
            v["players"][1][other]["vel"]["x"] = Fp::from_milli(-3_000).raw().into();
            v["players"][1][other]["vel"]["y"] = 0.into();
        }
        sim.load_state(&serde_json::to_vec(&v).unwrap()).unwrap();
        sim.tick(&[stick, InputFrame::IDLE]);
        (
            sim.state().players[0][ci].vel,
            sim.state().controlled_idx[0],
        )
    };
    let (far, c_far) = run(None);
    let (near, c_near) = run(Some(1_500));
    assert_eq!(c_far, ci);
    assert_eq!(c_near, ci);
    assert!(far.len() > Fp::ZERO, "stick produced no motion");
    assert_eq!(
        far, near,
        "first-tick human response must not depend on neighbours"
    );
}

#[test]
fn human_adjustment_is_bounded_lateral_only() {
    // A human sprinting at a defender gets at most a small lateral nudge, never a speed cut > 10%
    // or a heading flip.
    let me = Agent {
        pos: Vec2::ZERO,
        vel: Vec2::from_milli(7_000, 0),
        class: 1,
        team: 0,
        carrier: false,
        active: true,
    };
    let opp = Agent {
        pos: Vec2::from_milli(1_300, 100),
        vel: Vec2::from_milli(-4_000, 0),
        class: 2,
        team: 1,
        carrier: false,
        active: true,
    };
    let frame = Frame::from_agents(vec![me, opp], 2);
    let cmd = Vec2::from_milli(7_000, 0);
    let out = avoidance::human_adjust(&frame, 0, cmd, Fp::from_milli(8_000));
    assert!(
        out.x >= Fp::from_milli(6_300),
        "forward speed cut: {:?}",
        out
    );
    assert!(
        out.y.abs() <= Fp::from_milli(2_101),
        "lateral too large: {:?}",
        out
    );
    // From rest the command passes through untouched.
    let rest = Agent {
        vel: Vec2::ZERO,
        ..me
    };
    let frame = Frame::from_agents(vec![rest, opp], 2);
    assert_eq!(
        avoidance::human_adjust(&frame, 0, cmd, Fp::from_milli(8_000)),
        cmd
    );
}

#[test]
fn deterministic_hash_series_rollback_and_old_state() {
    let series = |seed: u64| -> Vec<u64> {
        let mut snap = FixtureSnapshot::sample_eleven_a_side(seed);
        snap.rules.home_controller = SideController::Cpu(CpuDifficulty::Normal);
        snap.rules.away_controller = SideController::Cpu(CpuDifficulty::Hard);
        let mut sim = MatchSimulation::new(&snap).unwrap();
        (0..3_000)
            .map(|_| {
                sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
                sim.state_hash()
            })
            .collect()
    };
    assert_eq!(
        series(99),
        series(99),
        "hash series differs between identical runs"
    );

    let mut sim = open_play_sim(false, 11);
    let saved = sim.save_state().unwrap();
    for _ in 0..900 {
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
    }
    let target = sim.state_hash();
    sim.load_state(&saved).unwrap();
    for _ in 0..900 {
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
    }
    assert_eq!(sim.state_hash(), target, "rollback replay diverged");

    // A state saved before `overlap_ticks` existed still loads and runs.
    let mut v: serde_json::Value = serde_json::from_slice(&saved).unwrap();
    for side in v["players"].as_array_mut().unwrap() {
        for p in side.as_array_mut().unwrap() {
            assert!(p.as_object_mut().unwrap().remove("overlap_ticks").is_some());
        }
    }
    sim.load_state(&serde_json::to_vec(&v).unwrap()).unwrap();
    sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
}

#[test]
fn push_out_is_position_only_capped_and_spares_class0() {
    let mk = |x: i64, class: u8| Body {
        pos: Vec2::from_milli(x, 0),
        vel: Vec2::from_milli(1_000, 500),
        class,
        mass: 90,
        active: true,
        stuck_ticks: 0,
    };
    let mut b = vec![mk(0, 0), mk(300, 2), mk(600, 3)];
    let before = b.clone();
    let d = avoidance::resolve_overlaps(&mut b);
    assert_eq!(b[0].pos, before[0].pos, "class 0 displaced");
    for (i, x) in b.iter().enumerate() {
        assert_eq!(x.vel, before[i].vel);
        assert!(d[i].len() <= Fp::from_milli(avoidance::PUSH_CAP_DEEP_MM + 1));
    }
    // A shallow overlap moves at most the 3 cm cap per tick.
    let mut b = vec![mk(0, 2), mk(700, 2)];
    let d = avoidance::resolve_overlaps(&mut b);
    assert!(d[0].len() <= Fp::from_milli(avoidance::PUSH_CAP_MM + 1));
    assert!(b[1].pos.x > Fp::from_milli(700) && b[0].pos.x < Fp::ZERO);
}

// -------------------------------------------------------------------------------------------------
// Scenario tests on the public avoidance pipeline (velocity stage -> motion limit -> push-out).
// -------------------------------------------------------------------------------------------------

use psfc_sim::avoidance::{self, Agent, Body, Frame};
use psfc_sim::{Fp, PlayerAttributes, Vec2};

fn attrs() -> PlayerAttributes {
    FixtureSnapshot::sample_five_a_side(1).home.starters[1].attributes
}

struct Runner {
    pos: Vec2,
    vel: Vec2,
    target: Vec2,
    speed: Fp,
    class: u8,
    team: u8,
    carrier: bool,
}

#[derive(Debug, Default)]
struct ScenarioResult {
    min_dist: f64,
    reversals: u32,
    side_flips: u32,
    carrier_max_dev: f64,
}

fn run_scenario(mut rs: Vec<Runner>, ticks: u32) -> ScenarioResult {
    let a = attrs();
    let dt = Fp::from_ratio(1, 60);
    let mut out = ScenarioResult {
        min_dist: 1e9,
        ..Default::default()
    };
    let n = rs.len();
    let init_dir: Vec<Vec2> = rs
        .iter()
        .map(|r| (r.target - r.pos).normalize_or_zero())
        .collect();
    let mut last_side = vec![0_i32; n];
    let mut stuck = vec![0_u8; n];
    for _ in 0..ticks {
        let frame = Frame::from_agents(
            rs.iter()
                .map(|r| Agent {
                    pos: r.pos,
                    vel: r.vel,
                    class: r.class,
                    team: r.team,
                    carrier: r.carrier,
                    active: true,
                })
                .collect(),
            n,
        );
        let mut new_v = Vec::new();
        for (i, r) in rs.iter().enumerate() {
            let to = r.target - r.pos;
            // Idle (class 3) players hold still unless avoidance moves them.
            let v_des = if r.class == avoidance::CLASS_IDLE || to.len() <= Fp::from_milli(300) {
                Vec2::ZERO
            } else {
                to.normalize_or_zero().scale(r.speed)
            };
            let v = avoidance::select_velocity(&frame, i, v_des, r.speed);
            if r.class == avoidance::CLASS_LOCKED {
                assert_eq!(v, v_des, "class-0 agent must never be steered");
            }
            new_v.push(avoidance::motion_step(r.vel, v, &a, false));
        }
        for (i, r) in rs.iter_mut().enumerate() {
            r.vel = new_v[i];
            r.pos += r.vel.scale(dt);
        }
        let mut bodies: Vec<Body> = rs
            .iter()
            .enumerate()
            .map(|(i, r)| Body {
                pos: r.pos,
                vel: r.vel,
                class: r.class,
                mass: 90,
                active: true,
                stuck_ticks: stuck[i],
            })
            .collect();
        let before: Vec<Vec2> = bodies.iter().map(|b| b.vel).collect();
        avoidance::resolve_overlaps(&mut bodies);
        for (i, b) in bodies.iter().enumerate() {
            assert_eq!(b.vel, before[i], "push-out must never change velocity");
            if rs[i].class == avoidance::CLASS_LOCKED {
                assert_eq!(b.pos, rs[i].pos, "class-0 agent must never be displaced");
            }
            rs[i].pos = b.pos;
            stuck[i] = b.stuck_ticks;
        }
        for i in 0..n {
            let r = &rs[i];
            if r.vel.len() > Fp::from_milli(500) && f(r.vel.dot(init_dir[i])) < 0.0 {
                out.reversals += 1;
            }
            let lat = f(r.vel.x.mul(init_dir[i].y) - r.vel.y.mul(init_dir[i].x));
            let side = if lat > 0.3 {
                1
            } else if lat < -0.3 {
                -1
            } else {
                0
            };
            if side != 0 {
                if last_side[i] != 0 && side != last_side[i] {
                    out.side_flips += 1;
                }
                last_side[i] = side;
            }
            if r.class == avoidance::CLASS_LOCKED {
                out.carrier_max_dev = out.carrier_max_dev.max(lat.abs());
            }
            for j in (i + 1)..n {
                out.min_dist = out.min_dist.min(f((rs[i].pos - rs[j].pos).len()));
            }
        }
    }
    out
}

fn runner(px: i64, py: i64, tx: i64, ty: i64, speed: i64, class: u8, team: u8) -> Runner {
    Runner {
        pos: Vec2::from_milli(px, py),
        vel: (Vec2::from_milli(tx, ty) - Vec2::from_milli(px, py))
            .normalize_or_zero()
            .scale(Fp::from_milli(speed)),
        target: Vec2::from_milli(tx, ty),
        speed: Fp::from_milli(speed),
        class,
        team,
        carrier: false,
    }
}

#[test]
fn crossing_runs_do_not_interpenetrate_or_flip() {
    let cases: Vec<(&str, Vec<Runner>)> = vec![
        (
            "head-on",
            vec![
                runner(-15_000, 0, 60_000, 0, 6_500, 2, 0),
                runner(15_000, 0, -60_000, 0, 6_500, 2, 1),
            ],
        ),
        (
            "head-on offset",
            vec![
                runner(-15_000, 0, 60_000, 0, 7_000, 2, 0),
                runner(15_000, 200, -60_000, 200, 7_000, 2, 0),
            ],
        ),
        (
            "90 deg",
            vec![
                runner(-12_000, 0, 60_000, 0, 6_500, 2, 0),
                runner(0, -12_000, 0, 60_000, 6_500, 2, 1),
            ],
        ),
        (
            "overtaking",
            vec![
                runner(-14_000, 0, 60_000, 0, 8_500, 2, 0),
                runner(-9_000, 0, 60_000, 0, 4_500, 2, 0),
            ],
        ),
    ];
    for (name, rs) in cases {
        let r = run_scenario(rs, 360);
        println!("{name}: {r:?}");
        assert!(
            r.min_dist >= 0.8,
            "{name}: centre distance {:.3} < 0.8 m",
            r.min_dist
        );
        assert_eq!(r.reversals, 0, "{name}: heading reversed");
        assert_eq!(r.side_flips, 0, "{name}: side flips");
    }
}

#[test]
fn class0_carrier_never_displaced_or_steered() {
    // Carrier runs straight through a tactical mover's path and past an idle player in his lane.
    let mut carrier = runner(-12_000, 0, 60_000, 0, 7_000, avoidance::CLASS_LOCKED, 0);
    carrier.carrier = true;
    let rs = vec![
        carrier,
        runner(0, -10_000, 0, 60_000, 6_000, 2, 1),
        runner(4_000, 300, 4_000, 300, 6_000, 3, 0),
    ];
    let r = run_scenario(rs, 300);
    println!("carrier: {r:?}");
    assert!(
        r.carrier_max_dev < 1e-6,
        "carrier deviated {}",
        r.carrier_max_dev
    );
    assert!(r.min_dist >= 0.8, "centre distance {:.3}", r.min_dist);
}
