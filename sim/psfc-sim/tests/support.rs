//! Off-ball attacking support and runs (round 4 work area 3, `docs/attacking-support-and-runs.md`):
//! scored support candidates, the run arbiter, hold-and-release at the offside line, and the
//! decision-side hysteresis. Public API only.

use psfc_sim::ai::{RUN_CALLED, RUN_PENETRATE, RUN_SUPPORT};
use psfc_sim::input::{BTN_CALL_RUN, BTN_THROUGH};
use psfc_sim::{
    AutoSwitchMode, CpuDifficulty, FixtureSnapshot, Fp, InputFrame, MatchPhase, MatchSimulation, MatchState,
    PlayerActionState, SideController, TeamSide, Vec2, Vec3,
};

const HOME: usize = 0;
const AWAY: usize = 1;

fn f(v: Fp) -> f64 {
    v.to_milli() as f64 / 1000.0
}

fn edit(sim: &mut MatchSimulation, func: impl FnOnce(&mut MatchState)) {
    let mut st: MatchState = serde_json::from_slice(&sim.save_state().unwrap()).unwrap();
    func(&mut st);
    sim.load_state(&serde_json::to_vec(&st).unwrap()).unwrap();
}

/// Human vs human open-play lab with the home outfielder `carrier` owning a still ball; every
/// player placed by `place` (`(side, idx) -> (x_mm, y_mm)`, home attacking +x).
fn lab(eleven: bool, carrier: usize, place: impl Fn(usize, usize) -> (i64, i64)) -> MatchSimulation {
    let mut snap = if eleven {
        FixtureSnapshot::sample_eleven_a_side(4_242)
    } else {
        FixtureSnapshot::sample_five_a_side(4_242)
    };
    snap.rules.home_controller = SideController::Human;
    snap.rules.away_controller = SideController::Human;
    let mut sim = MatchSimulation::new(&snap).unwrap();
    edit(&mut sim, |st| {
        st.phase = MatchPhase::OpenPlay;
        for cs in &mut st.control_settings {
            cs.auto_switch_mode = AutoSwitchMode::Manual;
        }
        let n = st.players[HOME].len();
        for side in [HOME, AWAY] {
            for idx in 0..n {
                let (x, y) = place(side, idx);
                let p = &mut st.players[side][idx];
                p.pos = Vec2::from_milli(x, y);
                p.vel = Vec2::ZERO;
                p.action = PlayerActionState::Normal;
                p.facing_bam = if side == HOME { 0 } else { 32_768 };
            }
        }
        st.controlled_idx = [carrier, n - 1];
        let c = st.players[HOME][carrier].pos;
        st.ball.pos = Vec3::new(c.x + Fp::from_milli(300), c.y, Fp::ZERO);
        st.ball.vel = Vec3::ZERO;
        st.ball.owner = Some((TeamSide::Home, carrier));
        st.ball.pass_in_flight = None;
        st.ball.last_touch_team = Some(TeamSide::Home);
        st.restart_touch_state = None;
    });
    sim
}

/// Open options of the home carrier as the analytics count them: an outfield teammate 6-30 m away
/// with no opponent within 1.5 m of the segment and not beyond the second-last opponent.
fn open_options(st: &MatchState) -> usize {
    let Some((TeamSide::Home, ci)) = st.ball.owner else {
        return 0;
    };
    let c = st.players[HOME][ci].pos;
    let mut xs: Vec<f64> = st.players[AWAY].iter().filter(|o| !o.sent_off).map(|o| f(o.pos.x)).collect();
    xs.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let line = xs.get(1).copied().unwrap_or(0.0).max(f(st.ball.pos.x));
    let mut n = 0;
    for (j, p) in st.players[HOME].iter().enumerate().skip(1) {
        if j == ci || p.sent_off {
            continue;
        }
        let d = f(p.pos.dist(c));
        if !(6.0..=30.0).contains(&d) || f(p.pos.x) > line + 0.2 {
            continue;
        }
        let (ax, ay, bx, by) = (f(c.x), f(c.y), f(p.pos.x), f(p.pos.y));
        let (dx, dy) = (bx - ax, by - ay);
        let l2 = dx * dx + dy * dy;
        let blocked = st.players[AWAY].iter().filter(|o| !o.sent_off).any(|o| {
            let (ox, oy) = (f(o.pos.x), f(o.pos.y));
            let t = (((ox - ax) * dx + (oy - ay) * dy) / l2).clamp(0.0, 1.0);
            let (px, py) = (ax + t * dx - ox, ay + t * dy - oy);
            (px * px + py * py).sqrt() < 1.5
        });
        if !blocked {
            n += 1;
        }
    }
    n
}

fn input(mx: i8, my: i8, ax: i8, ay: i8, buttons: u16) -> InputFrame {
    InputFrame {
        move_x: mx,
        move_y: my,
        aim_x: ax,
        aim_y: ay,
        buttons,
        reserved: 0,
    }
}

/// 5v5: the carrier starts with one open option (the back pass); the two forward teammates stand
/// in the shadows of their markers. Within 2 s the scored candidates open a second lane.
#[test]
fn carrier_with_one_open_option_gains_a_second_within_two_seconds() {
    let mut sim = lab(false, 1, |side, idx| match (side, idx) {
        (HOME, 0) => (-26_000, 0),
        (HOME, 1) => (-6_000, 0),
        (HOME, 2) => (6_000, 0),
        (HOME, 3) => (4_000, 8_000),
        (HOME, _) => (-14_000, -7_000),
        (AWAY, 0) => (26_000, 0),
        (AWAY, 1) => (0, 0),
        (AWAY, 2) => (-1_000, 4_000),
        (AWAY, 3) => (9_000, -9_000),
        _ => (14_000, 8_000),
    });
    assert_eq!(open_options(sim.state()), 1, "setup: exactly one open option");
    let mut best = 0;
    for _ in 0..120 {
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
        assert_eq!(sim.state().ball.owner, Some((TeamSide::Home, 1)), "the carrier keeps the ball");
        best = best.max(open_options(sim.state()));
        if best >= 2 {
            break;
        }
    }
    assert!(best >= 2, "a second open option appears within 2 s (best {best})");
}

/// 11v11: the human carrier calls a run (`BTN_CALL_RUN`) toward the forward he faces; the runner
/// gets an end-point beyond the offside line, holds at the onside limit (never beyond the line,
/// never reversing while onside), and is released by the through ball, after which he goes in
/// behind the line the defence held at the kick and is not caught offside.
#[test]
fn called_runner_holds_at_the_line_and_is_released_by_the_pass() {
    // Home 9 (a forward) 3 m behind the away line at x = 20 m; the carrier at the halfway line.
    let mut sim = lab(true, 6, |side, idx| match (side, idx) {
        (HOME, 0) => (-50_000, 0),
        (HOME, 6) => (0, 0),
        (HOME, 9) => (17_000, 2_000),
        (HOME, i) => (-30_000 + (i as i64) * 2_000, -20_000 + (i as i64) * 3_000),
        (AWAY, 0) => (50_000, 0),
        (_, i) if i <= 4 => (20_000, -15_000 + (i as i64 - 1) * 10_000),
        (_, i) => (9_000, if i % 2 == 0 { -24_000 + (i as i64) * 1_000 } else { 14_000 + (i as i64) * 700 }),
    });
    sim.tick(&[input(0, 0, 0, 0, BTN_CALL_RUN), InputFrame::IDLE]);
    let runner = {
        let st = sim.state();
        let m = st.ai_memory.borrow();
        let r = m[HOME].run_role.iter().position(|&r| r == RUN_CALLED);
        r.expect("BTN_CALL_RUN commits a called runner")
    };
    assert_ne!(runner, 6);
    let end = sim.state().ai_memory.borrow()[HOME].run_end[runner];
    let line = |st: &MatchState| {
        let mut xs: Vec<f64> = st.players[AWAY].iter().map(|o| f(o.pos.x)).collect();
        xs.sort_by(|a, b| b.partial_cmp(a).unwrap());
        xs[1]
    };
    assert!(f(end.x) > line(sim.state()), "end-point {end:?} of runner {runner} beyond the offside line {}", line(sim.state()));
    // Hold for 40 ticks: the runner approaches but never crosses the line.
    let mut prev_x = f(sim.state().players[HOME][runner].pos.x);
    for t in 0..40 {
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
        let st = sim.state();
        let x = f(st.players[HOME][runner].pos.x);
        assert!(x <= line(st) + 0.05, "tick {t}: runner at {x:.2} beyond the line {:.2}", line(st));
        if x < line(st) - 0.4 {
            assert!(x >= prev_x - 0.02, "tick {t}: runner reversed while onside ({prev_x:.2} -> {x:.2})");
        }
        prev_x = x;
    }
    // Release: a through ball aimed at the runner's end-point.
    let c = sim.state().players[HOME][6].pos;
    let aim = (end - c).normalize_or_zero();
    let (ax, ay) = ((aim.x.to_milli() * 127 / 1000) as i8, (aim.y.to_milli() * 127 / 1000) as i8);
    for _ in 0..10 {
        sim.tick(&[input(ax, ay, ax, ay, BTN_THROUGH), InputFrame::IDLE]);
    }
    let mut launched = false;
    let mut went_in_behind = false;
    let mut line_at_kick: Option<f64> = None;
    for _ in 0..90 {
        sim.tick(&[input(ax, ay, ax, ay, 0), InputFrame::IDLE]);
        let st = sim.state();
        if let Some(pf) = st.ball.pass_in_flight.as_ref().filter(|pf| pf.team == TeamSide::Home) {
            assert_eq!(pf.intended_receiver_idx, runner, "the through ball goes to the runner");
            launched = true;
        }
        if launched && line_at_kick.is_none() {
            line_at_kick = Some(line(st));
        }
        if line_at_kick.is_some_and(|l| f(st.players[HOME][runner].pos.x) > l + 1.0) {
            went_in_behind = true;
        }
        assert!(
            !matches!(st.phase, MatchPhase::SetPiece { .. }),
            "no offside or other stoppage after the release"
        );
    }
    assert!(launched, "the through ball was played");
    assert!(went_in_behind, "the released runner went in behind the line held at the kick");
}

/// CPU vs CPU, 5 match-minutes in each format: never more than one penetrate runner and one
/// support-from-behind runner per team on any tick.
#[test]
fn at_most_one_penetrate_runner_per_team() {
    for eleven in [false, true] {
        let mut snap = if eleven {
            FixtureSnapshot::sample_eleven_a_side(7)
        } else {
            FixtureSnapshot::sample_five_a_side(7)
        };
        let d = if eleven { CpuDifficulty::Normal } else { CpuDifficulty::Hard };
        snap.rules.home_controller = SideController::Cpu(d);
        snap.rules.away_controller = SideController::Cpu(d);
        snap.rules.half_duration_ticks = 150 * 60;
        let mut sim = MatchSimulation::new(&snap).unwrap();
        let mut runs = 0_u64;
        for _ in 0..(300 * 60) {
            sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
            let st = sim.state();
            let m = st.ai_memory.borrow();
            for side in [HOME, AWAY] {
                let pen = m[side].run_role.iter().filter(|&&r| r == RUN_PENETRATE).count();
                let sup = m[side].run_role.iter().filter(|&&r| r == RUN_SUPPORT).count();
                assert!(pen <= 1, "tick {}: {pen} penetrate runners", st.tick);
                assert!(sup <= 1, "tick {}: {sup} support runners", st.tick);
                runs += pen as u64;
            }
        }
        assert!(runs > 0, "eleven={eleven}: the arbiter assigned runs");
    }
}

/// A carrier dribbling straight up the middle: no supporter's committed support point alternates
/// sides of the carrier's line (at most one side change per supporter in 3 s).
#[test]
fn support_choice_does_not_alternate_sides_for_a_straight_dribble() {
    let mut sim = lab(true, 6, |side, idx| match (side, idx) {
        (HOME, 0) => (-50_000, 0),
        (HOME, 6) => (-20_000, 0),
        (HOME, i) => (-30_000 + (i as i64) * 2_500, -24_000 + (i as i64) * 4_500),
        (_, 0) => (50_000, 0),
        (_, i) => (12_000 + (i as i64 % 3) * 6_000, -22_000 + (i as i64) * 4_000),
    });
    let n = sim.state().players[HOME].len();
    let mut side_of: Vec<i8> = vec![0; n];
    let mut changes: Vec<u32> = vec![0; n];
    for _ in 0..180 {
        sim.tick(&[input(127, 0, 0, 0, 0), InputFrame::IDLE]);
        let st = sim.state();
        if st.ball.owner != Some((TeamSide::Home, 6)) {
            break;
        }
        let cy = st.players[HOME][6].pos.y;
        let m = st.ai_memory.borrow();
        for j in 1..n {
            if j == 6 || m[HOME].sup_kind[j] == u8::MAX {
                continue;
            }
            let dy = m[HOME].sup_pt[j].y - cy;
            if dy.abs() < Fp::from_milli(1_500) {
                continue;
            }
            let s: i8 = if dy > Fp::ZERO { 1 } else { -1 };
            if side_of[j] != 0 && side_of[j] != s {
                changes[j] += 1;
            }
            side_of[j] = s;
        }
    }
    for (j, c) in changes.iter().enumerate() {
        assert!(*c <= 1, "supporter {j} switched sides {c} times");
    }
}

/// Determinism: two runs of the same CPU match give identical hash series.
#[test]
fn hash_series_identical_over_two_runs() {
    let run = || {
        let mut snap = FixtureSnapshot::sample_eleven_a_side(11);
        snap.rules.home_controller = SideController::Cpu(CpuDifficulty::Normal);
        snap.rules.away_controller = SideController::Cpu(CpuDifficulty::Hard);
        let mut sim = MatchSimulation::new(&snap).unwrap();
        (0..3_000)
            .map(|_| {
                sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
                sim.state_hash()
            })
            .collect::<Vec<u64>>()
    };
    assert_eq!(run(), run());
}

/// A saved state from before round 4 (no support / run fields in the AI memory) still loads and
/// runs; the new fields start empty and are sized on first use.
#[test]
fn old_snapshot_without_support_fields_loads() {
    let mut snap = FixtureSnapshot::sample_five_a_side(3);
    snap.rules.home_controller = SideController::Cpu(CpuDifficulty::Normal);
    snap.rules.away_controller = SideController::Cpu(CpuDifficulty::Normal);
    let mut sim = MatchSimulation::new(&snap).unwrap();
    for _ in 0..600 {
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
    }
    let mut v: serde_json::Value = serde_json::from_slice(&sim.save_state().unwrap()).unwrap();
    let new_fields = [
        "sup_kind", "sup_since", "sup_eval", "sup_pt", "sup_carrier", "sup_nudge", "yield_sign", "gk_yield_sign",
        "bend_sign", "bend_tick", "res_yield", "tgt_vel", "tgt_raw", "run_role", "run_since", "run_end", "tri_idx",
        "tri_since", "tri_eval", "pr_on", "pr_since", "pr_eval", "pr_carrier", "run_eval", "run_round",
        "run_release", "run_release_tick",
    ];
    for mem in v["ai_memory"].as_array_mut().unwrap() {
        let obj = mem.as_object_mut().unwrap();
        for k in new_fields {
            assert!(obj.remove(k).is_some(), "field {k} is serialized");
        }
    }
    let mut sim2 = MatchSimulation::new(&snap).unwrap();
    sim2.load_state(&serde_json::to_vec(&v).unwrap()).unwrap();
    for _ in 0..600 {
        sim2.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
    }
    assert_eq!(sim2.state().ai_memory.borrow()[0].run_role.len(), sim2.state().players[0].len());
}

/// 11v11, CPU home side: a committed penetrate runner held at the line inside the carrier's
/// forward cone, with a nearer teammate also in the cone. The CPU's through ball is assigned to
/// the runner (the assisted receiver override) and led beyond the line, and he is onside.
#[test]
fn cpu_through_ball_goes_to_the_released_runner_inside_the_cone() {
    let mut snap = FixtureSnapshot::sample_eleven_a_side(4_242);
    snap.rules.home_controller = SideController::Cpu(CpuDifficulty::Hard);
    snap.rules.away_controller = SideController::Human;
    let mut sim = MatchSimulation::new(&snap).unwrap();
    let runner = 9;
    edit(&mut sim, |st| {
        st.phase = MatchPhase::OpenPlay;
        for cs in &mut st.control_settings {
            cs.auto_switch_mode = AutoSwitchMode::Manual;
        }
        let n = st.players[HOME].len();
        for side in [HOME, AWAY] {
            for idx in 0..n {
                let (x, y) = match (side, idx) {
                    (HOME, 0) => (-50_000, 0),
                    (HOME, 6) => (0, 0),
                    (HOME, 8) => (9_000, 3_000),
                    (HOME, 9) => (19_000, 5_000),
                    (HOME, i) => (-30_000 + (i as i64) * 2_000, if i % 2 == 0 { -26_000 } else { 26_000 }),
                    (_, 0) => (50_000, 0),
                    (_, i) if i <= 4 => (20_000, [-30_000, -18_000, 18_000, 30_000][i - 1]),
                    (_, i) => (40_000, -20_000 + (i as i64) * 3_000),
                };
                let p = &mut st.players[side][idx];
                p.pos = Vec2::from_milli(x, y);
                p.vel = Vec2::ZERO;
                p.action = PlayerActionState::Normal;
                p.facing_bam = if side == HOME { 0 } else { 32_768 };
            }
        }
        st.controlled_idx = [6, n - 1];
        st.ball.pos = Vec3::from_milli(300, 0, 0);
        st.ball.vel = Vec3::ZERO;
        st.ball.owner = Some((TeamSide::Home, 6));
        st.ball.pass_in_flight = None;
        st.ball.last_touch_team = Some(TeamSide::Home);
        st.restart_touch_state = None;
        let tick = st.tick;
        let mut m = st.ai_memory.borrow_mut();
        let mem = &mut m[HOME];
        mem.run_role = vec![0; n];
        mem.run_since = vec![0; n];
        mem.run_end = vec![Vec2::ZERO; n];
        mem.run_role[runner] = RUN_PENETRATE;
        mem.run_since[runner] = tick + 1;
        mem.run_end[runner] = Vec2::from_milli(30_000, 6_000);
    });
    let mut receiver = None;
    for _ in 0..180 {
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
        let st = sim.state();
        if let Some(pf) = st.ball.pass_in_flight.as_ref().filter(|pf| pf.team == TeamSide::Home) {
            receiver = Some((pf.intended_receiver_idx, f(pf.target_pos.x), pf.receiver_was_offside));
            break;
        }
    }
    let (r, tx, offside) = receiver.expect("the CPU carrier passes");
    assert_eq!(r, runner, "the pass goes to the committed runner");
    assert!(tx > 20.0, "led beyond the line (target x {tx:.1})");
    assert!(!offside, "the runner is onside at the kick");
}
