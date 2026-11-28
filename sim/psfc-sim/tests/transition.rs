//! Round 4, area T: transition urgency and work-rate-driven recovery runs
//! (`docs/transition-and-defending.md`). Constructed-state scenarios through the public API.
//! `cargo test --release -p psfc-sim --test transition -- --nocapture` prints the traces.

use psfc_sim::ai::{move_priority, team_phase};
use psfc_sim::{
    AutoSwitchMode, CpuDifficulty, FixtureSnapshot, Fp, InputFrame, MatchPhase, MatchSimulation, MatchState,
    PlayerActionState, PlayerRole, SideController, TeamSide, Vec2, Vec3,
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

fn idle(sim: &mut MatchSimulation, ticks: u32) {
    for _ in 0..ticks {
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
    }
}

/// 11v11 CPU vs CPU (Normal) in open play: the home side attacks +x. Home has the ball high in the
/// away half with its three forwards ahead of it; after `settle` ticks the ball is handed to the
/// nearest away player (a turnover) who then counters toward the home goal.
fn staged_turnover(settle: u32) -> MatchSimulation {
    let mut snap = FixtureSnapshot::sample_eleven_a_side(4_242);
    snap.rules.home_controller = SideController::Cpu(CpuDifficulty::Normal);
    snap.rules.away_controller = SideController::Cpu(CpuDifficulty::Normal);
    let mut sim = MatchSimulation::new(&snap).unwrap();
    edit(&mut sim, |st| {
        st.phase = MatchPhase::OpenPlay;
        for cs in &mut st.control_settings {
            cs.auto_switch_mode = AutoSwitchMode::Manual;
        }
        let n = st.players[HOME].len();
        for side in [HOME, AWAY] {
            let mut k = 0_i64;
            for idx in 0..n {
                let p = &mut st.players[side][idx];
                p.vel = Vec2::ZERO;
                p.action = PlayerActionState::Normal;
                let (x, y) = if idx == 0 {
                    (if side == HOME { -50_000 } else { 50_000 }, 0)
                } else if side == HOME {
                    k += 1;
                    match p.role {
                        PlayerRole::Forward => (38_000, -14_000 + k * 3_000),
                        PlayerRole::Midfielder => (12_000, -16_000 + k * 3_000),
                        _ => (-2_000, -20_000 + k * 4_000),
                    }
                } else {
                    k += 1;
                    match p.role {
                        PlayerRole::Forward => (2_000, -12_000 + k * 3_000),
                        PlayerRole::Midfielder => (18_000, -14_000 + k * 3_000),
                        _ => (34_000, -18_000 + k * 3_500),
                    }
                };
                p.pos = Vec2::from_milli(x, y);
            }
        }
        // Home midfielder 2 carries the ball at x = 16 m.
        st.players[HOME][2].pos = Vec2::from_milli(16_000, 0);
        st.ball.pos = Vec3::from_milli(16_400, 0, 0);
        st.ball.vel = Vec3::ZERO;
        st.ball.owner = Some((TeamSide::Home, 2));
        st.ball.pass_in_flight = None;
        st.controlled_idx = [2, 1];
    });
    idle(&mut sim, settle);
    edit(&mut sim, |st| {
        // Turnover: an away midfielder takes the ball off him and faces the home goal.
        let ball = st.players[HOME][2].pos;
        let (ai, _) = st.players[AWAY]
            .iter()
            .enumerate()
            .skip(1)
            .filter(|(_, p)| p.role == PlayerRole::Midfielder)
            .map(|(i, p)| (i, p.pos.dist(ball).to_milli()))
            .min_by_key(|&(_, d)| d)
            .unwrap();
        st.players[AWAY][ai].pos = ball + Vec2::from_milli(-800, 0);
        st.players[AWAY][ai].facing_bam = 32_768;
        st.players[HOME][2].pos = ball + Vec2::from_milli(600, 1_200);
        st.ball.pos = Vec3::new(st.players[AWAY][ai].pos.x - Fp::from_milli(400), ball.y, Fp::ZERO);
        st.ball.vel = Vec3::ZERO;
        st.ball.owner = Some((TeamSide::Away, ai));
        st.controlled_idx[AWAY] = ai;
    });
    sim
}

/// Ahead-of-ball forwards on a turnover: every forward but the rest offence reaches sprint speed
/// (> 7 m/s) within 30 ticks and is never an idle class-3 cruiser while more than 1 m ahead of the
/// ball, until he is level with it; exactly one Low-work-rate forward (the rest offence) holds high.
#[test]
fn turnover_forwards_sprint_back_and_one_low_dwr_forward_holds_high() {
    let mut sim = staged_turnover(60);
    let fwd: Vec<usize> = (1..sim.state().players[HOME].len())
        .filter(|&i| sim.state().players[HOME][i].role == PlayerRole::Forward)
        .collect();
    assert_eq!(fwd.len(), 3, "4-3-3 sample squad");
    let mut fast = vec![false; fwd.len()];
    let mut level = vec![false; fwd.len()];
    let mut d_first = vec![f64::NAN; fwd.len()];
    let mut d_min = vec![f64::MAX; fwd.len()];
    let mut rest_seen: Option<usize> = None;
    for t in 0..360 {
        idle(&mut sim, 1);
        let st = sim.state();
        if st.ball.owner.is_some_and(|(s, _)| s == TeamSide::Home) || !matches!(st.phase, MatchPhase::OpenPlay) {
            break;
        }
        let rest = st.ai_memory.borrow()[HOME].rest_offence as usize;
        let bx = f(st.ball.pos.x);
        for (k, &i) in fwd.iter().enumerate() {
            let p = &st.players[HOME][i];
            let d = f(p.pos.x) - bx;
            if i == rest {
                rest_seen = Some(i);
                continue;
            }
            if f(p.vel.len()) > 7.0 && t < 30 {
                fast[k] = true;
            }
            if d_first[k].is_nan() {
                d_first[k] = d;
            }
            d_min[k] = d_min[k].min(d);
            if d <= 0.0 {
                level[k] = true;
            }
            if !level[k] && t >= 30 && d > 1.0 {
                let c = move_priority(st, TeamSide::Home, i);
                assert!(c <= 2, "t{t}: forward {i} {d:.1} m ahead of the ball is an idle class-{c} cruiser");
            }
        }
    }
    let rest = rest_seen.expect("one Low-DWR forward is the rest offence");
    let st = sim.state();
    assert!(st.players[HOME][rest].attributes.work_rates(PlayerRole::Forward).1 <= 37, "rest offence has Low DWR");
    for (k, &i) in fwd.iter().enumerate() {
        if i != rest {
            assert!(fast[k], "forward {i} did not reach 7 m/s within 30 ticks");
            // Level with the ball, or at least 10 m (or 70%) of the deficit recovered before the
            // episode ended. The 70% clause is a round-4 merge change: a runner now jogs while a
            // loose ball (a pass in flight) is within 25 m, which is the exclusivity stand-off.
            assert!(
                level[k] || d_min[k] < d_first[k] - 10.0_f64.min(0.7 * d_first[k]),
                "forward {i} recovered only {:.1} m of {:.1} m",
                d_first[k] - d_min[k],
                d_first[k]
            );
        }
    }
    assert!(f(st.players[HOME][rest].pos.x) > f(st.ball.pos.x), "the rest offence holds high");
}

/// Two runs of the same 11v11 CPU match produce the same state-hash series.
#[test]
fn hash_series_is_identical_over_two_runs() {
    let run = || -> Vec<u64> {
        let mut snap = FixtureSnapshot::sample_eleven_a_side(77);
        snap.rules.home_controller = SideController::Cpu(CpuDifficulty::Hard);
        snap.rules.away_controller = SideController::Cpu(CpuDifficulty::Normal);
        let mut sim = MatchSimulation::new(&snap).unwrap();
        let mut v = Vec::new();
        for t in 0..3_600 {
            idle(&mut sim, 1);
            if t % 60 == 0 {
                v.push(sim.state_hash());
            }
        }
        v
    };
    assert_eq!(run(), run());
}

/// A saved state without the round-4 area-T memory fields still loads and plays on.
#[test]
fn old_snapshot_without_area_t_fields_loads() {
    let mut sim = staged_turnover(30);
    idle(&mut sim, 30);
    let mut v: serde_json::Value = serde_json::from_slice(&sim.save_state().unwrap()).unwrap();
    let fields = [
        "cp_decided",
        "third_presser",
        "closer_lane",
        "contain_mate",
        "rest_offence",
        "line_prog",
        "press_trigger_ticks",
        "rec_flags",
        "burst_ticks",
        "hi_dist_mm",
    ];
    for m in v["ai_memory"].as_array_mut().unwrap() {
        let o = m.as_object_mut().unwrap();
        for k in fields {
            assert!(o.remove(k).is_some(), "field {k} is serialized");
        }
    }
    sim.load_state(&serde_json::to_vec(&v).unwrap()).unwrap();
    idle(&mut sim, 120);
    assert!(sim.state().tick > 0);
}

#[test]
#[ignore]
fn trace_staged_turnover() {
    let mut sim = staged_turnover(60);
    let fwd: Vec<usize> = (1..sim.state().players[HOME].len())
        .filter(|&i| sim.state().players[HOME][i].role == PlayerRole::Forward)
        .collect();
    for t in 0..420 {
        idle(&mut sim, 1);
        let st = sim.state();
        if t % 20 == 0 {
            let bx = f(st.ball.pos.x);
            let row: Vec<String> = fwd
                .iter()
                .map(|&i| {
                    let p = &st.players[HOME][i];
                    format!("#{i} x{:.1} v{:.1} c{}", f(p.pos.x), f(p.vel.len()), move_priority(st, TeamSide::Home, i))
                })
                .collect();
            println!("t{t:3} ph{} ball {bx:.1} own {:?} | {}", team_phase(st, TeamSide::Home), st.ball.owner, row.join(" | "));
        }
    }
}

/// Mean on-pitch outfield stamina (permille) of both sides, and of the midfielders only.
fn stamina_means(st: &MatchState) -> (f64, f64) {
    let (mut all, mut na, mut mid, mut nm) = (0.0, 0.0, 0.0, 0.0);
    for side in &st.players {
        for p in side.iter().skip(1).filter(|p| !p.sent_off) {
            all += f64::from(p.stamina_permille);
            na += 1.0;
            if p.role == PlayerRole::Midfielder {
                mid += f64::from(p.stamina_permille);
                nm += 1.0;
            }
        }
    }
    (all / na, mid / nm)
}

/// 11v11 long-term stamina: a 10-minute CPU match (5-minute halves) ends with outfielders above
/// 800 permille (the drain used to empty them to the 150 floor by minute 10).
#[test]
fn eleven_a_side_ten_minute_match_keeps_stamina_above_800() {
    let mut snap = FixtureSnapshot::sample_eleven_a_side(1);
    snap.rules.home_controller = SideController::Cpu(CpuDifficulty::Normal);
    snap.rules.away_controller = SideController::Cpu(CpuDifficulty::Normal);
    snap.rules.half_duration_ticks = 5 * 60 * 60;
    let mut sim = MatchSimulation::new(&snap).unwrap();
    while !sim.is_finished() {
        idle(&mut sim, 1);
    }
    let (all, mid) = stamina_means(sim.state());
    println!("10-minute 11v11: outfield stamina {all:.0}, midfielders {mid:.0}");
    assert!(all > 800.0, "outfield stamina {all:.0} permille at 10 minutes");
}

/// Full 90-minute check (slow): prints outfield and midfielder stamina at 10, 45 and 90 minutes.
#[test]
#[ignore]
fn eleven_a_side_ninety_minute_stamina_curve() {
    let mut snap = FixtureSnapshot::sample_eleven_a_side(1);
    snap.rules.home_controller = SideController::Cpu(CpuDifficulty::Normal);
    snap.rules.away_controller = SideController::Cpu(CpuDifficulty::Normal);
    snap.rules.half_duration_ticks = 45 * 60 * 60;
    let mut sim = MatchSimulation::new(&snap).unwrap();
    let mut marks = vec![10_u32, 45, 90];
    while !sim.is_finished() {
        idle(&mut sim, 1);
        let mins = sim.state().clock_ticks / 3_600;
        if marks.first().is_some_and(|&m| mins >= m) || (sim.is_finished() && !marks.is_empty()) {
            let (all, mid) = stamina_means(sim.state());
            println!("minute {}: outfield {all:.0}, midfielders {mid:.0}", marks.remove(0));
        }
    }
}
