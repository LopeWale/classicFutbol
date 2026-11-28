//! WA1 steering-oscillation scenarios through the public API: the stateful avoidance sampler
//! (`avoidance::select_velocity_mem` with `state::SteerMemory`), the second-order steering filter
//! (`ai::filter_velocity_command`), determinism of the hash series and loading of old snapshots.
//!
//! `cargo test --release -p psfc-sim --test steering -- --nocapture` prints the scenario numbers.

use psfc_sim::avoidance::{self, Agent, Frame};
use psfc_sim::state::{SteerMemory, TeamAiMemory};
use psfc_sim::{
    CpuDifficulty, FixtureSnapshot, Fp, InputFrame, MatchSimulation, PlayerAttributes,
    SideController, Vec2,
};

fn f(v: Fp) -> f64 {
    v.raw() as f64 / 4_294_967_296.0
}

fn attrs() -> PlayerAttributes {
    FixtureSnapshot::sample_five_a_side(1).home.starters[1].attributes
}

#[derive(Clone)]
struct Runner {
    pos: Vec2,
    vel: Vec2,
    target: Vec2,
    speed: Fp,
    class: u8,
    team: u8,
}

fn runner(px: i64, py: i64, tx: i64, ty: i64, speed: i64, team: u8) -> Runner {
    Runner {
        pos: Vec2::from_milli(px, py),
        vel: (Vec2::from_milli(tx, ty) - Vec2::from_milli(px, py))
            .normalize_or_zero()
            .scale(Fp::from_milli(speed)),
        target: Vec2::from_milli(tx, ty),
        speed: Fp::from_milli(speed),
        class: 2,
        team,
    }
}

#[derive(Debug, Default)]
struct Outcome {
    /// Lateral sign flips (lateral velocity change versus the previous heading, `> 0.05 m/s` on
    /// both ticks, opposite signs) after the first 12 ticks, summed over the runners.
    flips_after_12: u32,
    min_dist: f64,
    /// Sign of each runner's largest lateral excursion from its start line (`+1` left, `-1` right).
    pass_side: Vec<i32>,
    /// Final positions (exact fixed point) for the determinism check.
    final_pos: Vec<Vec2>,
}

/// Velocity stage (stateful sampler) -> second-order filter -> motion limits, like the sim's
/// off-ball AI path, for `ticks` ticks at 60 Hz.
fn run(mut rs: Vec<Runner>, ticks: u32) -> Outcome {
    let a = attrs();
    let dt = Fp::from_ratio(1, 60);
    let n = rs.len();
    let mut mem = SteerMemory::default();
    let mut filt = TeamAiMemory::default();
    let start: Vec<(Vec2, Vec2)> = rs
        .iter()
        .map(|r| (r.pos, (r.target - r.pos).normalize_or_zero()))
        .collect();
    let mut out = Outcome {
        min_dist: 1e9,
        ..Default::default()
    };
    let mut prev_lat = vec![0.0_f64; n];
    let mut max_excursion = vec![0.0_f64; n];
    for tick in 1..=ticks {
        let frame = Frame::from_agents(
            rs.iter()
                .map(|r| Agent {
                    pos: r.pos,
                    vel: r.vel,
                    class: r.class,
                    team: r.team,
                    carrier: false,
                    active: true,
                })
                .collect(),
            n,
        );
        avoidance::begin_tick(&mut mem, &frame);
        let mut new_v = Vec::with_capacity(n);
        for (i, r) in rs.iter().enumerate() {
            let to = r.target - r.pos;
            let v_des = if to.len() <= Fp::from_milli(300) {
                Vec2::ZERO
            } else {
                to.normalize_or_zero().scale(r.speed)
            };
            let sel = avoidance::select_velocity_mem(&frame, &mut mem, tick, i, v_des, r.speed);
            let cmd = psfc_sim::ai::filter_velocity_command(&mut filt, n, i, tick, &a, false, sel);
            new_v.push(avoidance::motion_step(r.vel, cmd, &a, false));
        }
        for (i, r) in rs.iter_mut().enumerate() {
            let old = r.vel;
            r.vel = new_v[i];
            r.pos += r.vel.scale(dt);
            let sp0 = f(old.len());
            if sp0 > 1.0 {
                let lat = f(old.x.mul(r.vel.y) - old.y.mul(r.vel.x)) / sp0;
                if tick > 12 && lat.abs() > 0.05 && prev_lat[i].abs() > 0.05 && lat.signum() != prev_lat[i].signum() {
                    out.flips_after_12 += 1;
                }
                prev_lat[i] = lat;
            }
            let (p0, d0) = start[i];
            let rel = r.pos - p0;
            let exc = f(d0.x.mul(rel.y) - d0.y.mul(rel.x));
            if exc.abs() > max_excursion[i].abs() {
                max_excursion[i] = exc;
            }
        }
        for i in 0..n {
            for j in (i + 1)..n {
                out.min_dist = out.min_dist.min(f((rs[i].pos - rs[j].pos).len()));
            }
        }
    }
    out.pass_side = max_excursion
        .iter()
        .map(|&e| if e > 0.0 { 1 } else if e < 0.0 { -1 } else { 0 })
        .collect();
    out.final_pos = rs.iter().map(|r| r.pos).collect();
    out
}

fn head_on() -> Vec<Runner> {
    vec![
        runner(-15_000, 0, 60_000, 0, 6_500, 0),
        runner(15_000, 0, -60_000, 0, 6_500, 1),
    ]
}

#[test]
fn head_on_and_crossing_pairs_do_not_flip_after_12_ticks() {
    let cases: Vec<(&str, Vec<Runner>)> = vec![
        ("head-on", head_on()),
        (
            "head-on offset",
            vec![
                runner(-15_000, 0, 60_000, 0, 7_000, 0),
                runner(15_000, 200, -60_000, 200, 7_000, 0),
            ],
        ),
        (
            "90 deg crossing",
            vec![
                runner(-12_000, 0, 60_000, 0, 6_500, 0),
                runner(0, -12_000, 0, 60_000, 6_500, 1),
            ],
        ),
        (
            "45 deg crossing",
            vec![
                runner(-12_000, 0, 60_000, 0, 7_000, 0),
                runner(-8_485, -8_485, 42_426, 42_426, 7_000, 0),
            ],
        ),
    ];
    for (name, rs) in cases {
        let o = run(rs, 360);
        println!("{name}: flips_after_12 {} min_dist {:.3} sides {:?}", o.flips_after_12, o.min_dist, o.pass_side);
        assert_eq!(o.flips_after_12, 0, "{name}: lateral sign flips");
        assert!(o.min_dist >= 0.8, "{name}: centre distance {:.3} < 0.8 m", o.min_dist);
    }
}

#[test]
fn symmetric_head_on_resolves_to_the_same_sides_every_run() {
    let a = run(head_on(), 300);
    let b = run(head_on(), 300);
    assert_eq!(a.final_pos, b.final_pos, "trajectories differ between identical runs");
    assert_eq!(a.pass_side, b.pass_side);
    // The two agents step to opposite sides of their own start lines' direction frames in a way
    // that keeps them apart: both pass on the same hand (each keeps the other on one side).
    assert!(a.pass_side.iter().all(|&s| s != 0), "no avoidance happened: {:?}", a.pass_side);
    assert_eq!(a.pass_side[0], a.pass_side[1], "agents chose conflicting sides: {:?}", a.pass_side);
    assert!(a.min_dist >= 0.8, "centre distance {:.3}", a.min_dist);
}

#[test]
fn steering_filter_has_no_overshoot_on_a_step() {
    let a = attrs();
    let mut m = TeamAiMemory::default();
    let sp = Fp::from_milli(6_000);
    let east = Vec2::from_milli(6_000, 0);
    let north = Vec2::from_milli(0, 6_000);
    // Settle on east, then step the desired direction 90 degrees to north.
    for t in 1..=30 {
        let o = psfc_sim::ai::filter_velocity_command(&mut m, 2, 1, t, &a, false, east);
        assert!((o - east).len() <= Fp::from_milli(1), "steady input must pass unchanged: {o:?}");
    }
    let mut prev_y = Fp::ZERO;
    let mut prev_x = sp;
    let mut reached = false;
    for t in 31..=150 {
        let o = psfc_sim::ai::filter_velocity_command(&mut m, 2, 1, t, &a, false, north);
        // Monotone: never swings past north (x never negative), x falls and y rises.
        // (1 mm/s tolerance: fixed-point rounding of the settled state)
        assert!(o.x >= Fp::from_milli(-1), "overshoot past the target direction at tick {t}: {o:?}");
        assert!(o.x <= prev_x + Fp::from_milli(1), "x not monotone at tick {t}");
        assert!(o.y + Fp::from_milli(1) >= prev_y, "y not monotone at tick {t}");
        assert!(o.len() <= sp + Fp::from_milli(5), "speed above the command at tick {t}");
        prev_x = o.x;
        prev_y = o.y;
        if o.x < Fp::from_milli(60) {
            reached = true;
        }
    }
    assert!(reached, "filter never converged");
    // Lag: a third of a second after the step the command is at least 75% of the way round.
    let mut m2 = TeamAiMemory::default();
    for t in 1..=30 {
        psfc_sim::ai::filter_velocity_command(&mut m2, 2, 1, t, &a, false, east);
    }
    let mut o = Vec2::ZERO;
    for t in 31..=50 {
        o = psfc_sim::ai::filter_velocity_command(&mut m2, 2, 1, t, &a, false, north);
    }
    let ang = f(o.y).atan2(f(o.x)).to_degrees();
    assert!(ang > 67.5, "filter too slow: {ang:.1} deg after 20 ticks");
    // A one-tick reversal of the desired direction is absorbed (the output never reverses).
    let mut m3 = TeamAiMemory::default();
    for t in 1..=30 {
        psfc_sim::ai::filter_velocity_command(&mut m3, 2, 1, t, &a, false, east);
    }
    let o = psfc_sim::ai::filter_velocity_command(&mut m3, 2, 1, 31, &a, false, -east);
    assert!(o.x > Fp::ZERO, "one-tick reversal passed through: {o:?}");
}

fn series(seed: u64, home_human: bool) -> Vec<u64> {
    let mut snap = FixtureSnapshot::sample_eleven_a_side(seed);
    snap.rules.home_controller = if home_human {
        SideController::Human
    } else {
        SideController::Cpu(CpuDifficulty::Normal)
    };
    snap.rules.away_controller = SideController::Cpu(CpuDifficulty::Hard);
    let mut sim = MatchSimulation::new(&snap).unwrap();
    (0..2_400)
        .map(|_| {
            let home = if home_human {
                psfc_sim::ai::suggest_cpu_input(sim.state(), psfc_sim::TeamSide::Home, CpuDifficulty::Normal)
            } else {
                InputFrame::IDLE
            };
            sim.tick(&[home, InputFrame::IDLE]);
            sim.state_hash()
        })
        .collect()
}

#[test]
fn hash_series_identical_over_two_runs() {
    assert_eq!(series(5, false), series(5, false), "CPU-vs-CPU hash series differs");
    assert_eq!(series(6, true), series(6, true), "autopilot hash series differs");
}

#[test]
fn old_snapshot_without_steering_fields_still_loads() {
    let mut snap = FixtureSnapshot::sample_five_a_side(3);
    snap.rules.home_controller = SideController::Cpu(CpuDifficulty::Hard);
    snap.rules.away_controller = SideController::Cpu(CpuDifficulty::Hard);
    let mut sim = MatchSimulation::new(&snap).unwrap();
    for _ in 0..1_500 {
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
    }
    let saved = sim.save_state().unwrap();
    let mut v: serde_json::Value = serde_json::from_slice(&saved).unwrap();
    let obj = v.as_object_mut().unwrap();
    assert!(obj.remove("steer_memory").is_some(), "steer_memory not serialized");
    for mem in v["ai_memory"].as_array_mut().unwrap() {
        let m = mem.as_object_mut().unwrap();
        let keys: Vec<String> = m.keys().filter(|k| k.starts_with("steer_")).cloned().collect();
        assert!(!keys.is_empty(), "no steer_* fields in ai_memory");
        for k in keys {
            m.remove(&k);
        }
    }
    for side in v["players"].as_array().unwrap() {
        for p in side.as_array().unwrap() {
            assert!(p.get("steer_debug").is_none(), "diagnostics must not be serialized");
        }
    }
    sim.load_state(&serde_json::to_vec(&v).unwrap()).unwrap();
    for _ in 0..120 {
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
    }
}
