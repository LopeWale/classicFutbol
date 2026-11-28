//! Round 4, area T: off-ball defending (`docs/transition-and-defending.md`): the human's
//! teammate-contain mate as the planned second engager, runner tracking with the Konami cover
//! check, and the shared back line. Constructed-state scenarios through the public API.

use psfc_sim::ai::committed_presser;
use psfc_sim::state::AI_NONE;
use psfc_sim::{
    AutoSwitchMode, CpuDifficulty, FixtureSnapshot, Fp, InputFrame, MatchPhase, MatchSimulation, MatchState,
    PlayerActionState, PlayerRole, SideController, TeamSide, Vec2, Vec3, BTN_TEAMMATE_CONTAIN,
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

fn lab(eleven: bool, home: SideController, away: SideController) -> MatchSimulation {
    let mut snap = if eleven {
        FixtureSnapshot::sample_eleven_a_side(5_151)
    } else {
        FixtureSnapshot::sample_five_a_side(5_151)
    };
    snap.rules.home_controller = home;
    snap.rules.away_controller = away;
    let mut sim = MatchSimulation::new(&snap).unwrap();
    edit(&mut sim, |st| {
        st.phase = MatchPhase::OpenPlay;
        for cs in &mut st.control_settings {
            cs.auto_switch_mode = AutoSwitchMode::Manual;
        }
        for side in &mut st.players {
            for p in side.iter_mut() {
                p.vel = Vec2::ZERO;
                p.action = PlayerActionState::Normal;
            }
        }
        st.ball.vel = Vec3::ZERO;
        st.ball.pass_in_flight = None;
        st.restart_touch_state = None;
    });
    sim
}

/// Stick input toward `to` from `from` (full deflection).
fn stick(from: Vec2, to: Vec2, buttons: u16) -> InputFrame {
    let d = (to - from).normalize_or_zero();
    InputFrame {
        move_x: (d.x.to_milli() * 127 / 1_000) as i8,
        move_y: (d.y.to_milli() * 127 / 1_000) as i8,
        aim_x: 0,
        aim_y: 0,
        buttons,
        reserved: 0,
    }
}

/// 5v5: a CPU home carrier dribbles at the away goal while the away human holds teammate contain
/// and closes him down with the controlled player. The contain mate is the planned second engager
/// (never the presser or cover), the AI presser stands down, and no tick has three away outfielders
/// within 2.5 m of the carrier.
#[test]
fn teammate_contain_never_yields_three_engagers() {
    let mut sim = lab(false, SideController::Cpu(CpuDifficulty::Normal), SideController::Human);
    let mut engaged3 = 0;
    let mut registered = 0;
    let mut held = 0;
    let mut ran = 0;
    for episode in 0..8_i64 {
        let dy = (episode % 4 - 2) * 3_000;
        edit(&mut sim, |st| {
            // Home attacks +x in the first half; the away goal is at +x.
            st.phase = MatchPhase::OpenPlay;
            st.players[HOME][2].pos = Vec2::from_milli(2_000, dy);
            st.players[HOME][2].facing_bam = 0;
            st.ball.pos = Vec3::from_milli(2_400, dy, 0);
            st.ball.vel = Vec3::ZERO;
            st.ball.pass_in_flight = None;
            st.ball.owner = Some((TeamSide::Home, 2));
            st.players[HOME][1].pos = Vec2::from_milli(-8_000, 6_000);
            st.players[HOME][3].pos = Vec2::from_milli(-6_000, -7_000);
            st.players[HOME][4].pos = Vec2::from_milli(8_000, 9_000);
            st.players[AWAY][1].pos = Vec2::from_milli(7_000, dy + 1_500);
            st.players[AWAY][2].pos = Vec2::from_milli(6_500, dy - 2_000);
            st.players[AWAY][3].pos = Vec2::from_milli(9_000, 3_500);
            st.players[AWAY][4].pos = Vec2::from_milli(10_000, -4_000);
            for side in &mut st.players {
                for p in side.iter_mut() {
                    p.vel = Vec2::ZERO;
                    p.action = PlayerActionState::Normal;
                    p.contain_stamina = p.contain_stamina.max(200);
                    p.contain_cooldown = 0;
                }
            }
            st.controlled_idx = [2, 1];
        });
        for _ in 0..240 {
            ran += 1;
            let st = sim.state();
            let Some((TeamSide::Home, ci)) = st.ball.owner else { break };
            let carrier = st.players[HOME][ci].pos;
            let me = st.players[AWAY][st.controlled_idx[AWAY]].pos;
            let input = stick(me, carrier, BTN_TEAMMATE_CONTAIN);
            sim.tick(&[InputFrame::IDLE, input]);
            let st = sim.state();
            if !matches!(st.phase, MatchPhase::OpenPlay) {
                break;
            }
            let Some((TeamSide::Home, ci)) = st.ball.owner else { break };
            let c = st.players[HOME][ci].pos;
            let near = st.players[AWAY].iter().skip(1).filter(|p| !p.sent_off && p.pos.dist(c) <= Fp::from_milli(2_500)).count();
            if near >= 3 {
                engaged3 += 1;
            }
            if let Some(tc) = st.teammate_contain_idx[AWAY] {
                held += 1;
                let m = &st.ai_memory.borrow()[AWAY];
                if m.contain_mate as usize == tc {
                    registered += 1;
                    assert_ne!(m.presser as usize, tc, "the contain mate is never the AI presser");
                    assert_ne!(m.cover as usize, tc, "the contain mate is never the cover");
                    assert_eq!(m.mark_of[tc], AI_NONE, "the contain mate marks nobody");
                    assert_eq!(committed_presser(st, TeamSide::Away), None, "the AI presser stands down while contain is held");
                }
            }
        }
    }
    println!("ran {ran} ticks, contain {held}, registered {registered}");
    assert!(registered > 20 && registered * 10 >= held * 9, "teammate contain was held and registered ({registered} of {held} ticks)");
    assert_eq!(engaged3, 0, "ticks with three engagers");
}

/// 11v11 settled defence with the ball held still in midfield and no opponents to mark: the back
/// line (Defender-role players other than the presser and the cover) stays within 1.5 m in x.
#[test]
fn shared_line_keeps_back_liners_level_in_settled_defence() {
    let mut sim = lab(true, SideController::Human, SideController::Human);
    edit(&mut sim, |st| {
        st.players[AWAY][5].pos = Vec2::from_milli(2_000, 3_000);
        st.ball.pos = Vec3::from_milli(1_600, 3_000, 0);
        st.ball.owner = Some((TeamSide::Away, 5));
        st.controlled_idx = [4, 5];
        for (i, p) in st.players[AWAY].iter_mut().enumerate() {
            if i != 0 && i != 5 {
                p.sent_off = true;
            }
        }
    });
    let mut worst = 0.0_f64;
    for t in 0..420 {
        edit(&mut sim, |st| {
            // The carrier stands still with the ball.
            st.players[AWAY][5].pos = Vec2::from_milli(2_000, 3_000);
            st.players[AWAY][5].vel = Vec2::ZERO;
            st.ball.pos = Vec3::from_milli(1_600, 3_000, 0);
            st.ball.vel = Vec3::ZERO;
            st.ball.owner = Some((TeamSide::Away, 5));
        });
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
        if t < 300 {
            continue;
        }
        let st = sim.state();
        let m = st.ai_memory.borrow()[HOME].clone();
        let xs: Vec<f64> = (1..st.players[HOME].len())
            .filter(|&i| {
                st.players[HOME][i].role == PlayerRole::Defender
                    && i != st.controlled_idx[HOME]
                    && m.presser as usize != i
                    && m.cover as usize != i
            })
            .map(|i| f(st.players[HOME][i].pos.x))
            .collect();
        assert!(xs.len() >= 3, "three or more back-liners hold the line");
        let spread = xs.iter().cloned().fold(f64::MIN, f64::max) - xs.iter().cloned().fold(f64::MAX, f64::min);
        worst = worst.max(spread);
    }
    assert!(worst <= 1.5, "back-line x spread {worst:.2} m");
}

/// A runner the back line marks who breaks in behind it is tracked by his marker (the mark is kept
/// and the gap does not open) while no deeper teammate covers his line to goal; once a teammate
/// stands on that line (Konami's La cover check) the marker hands him over.
#[test]
fn marked_runner_in_behind_is_tracked_until_a_cover_exists() {
    let mut sim = lab(true, SideController::Human, SideController::Human);
    // Home defends the -x goal. Away carrier in midfield, one away runner starting level with the
    // home back line; the other away outfielders are off the pitch.
    edit(&mut sim, |st| {
        st.players[AWAY][5].pos = Vec2::from_milli(5_000, 0);
        st.ball.pos = Vec3::from_milli(4_600, 0, 0);
        st.ball.owner = Some((TeamSide::Away, 5));
        st.controlled_idx = [4, 5];
        for (i, p) in st.players[AWAY].iter_mut().enumerate() {
            if i != 0 && i != 5 && i != 10 {
                p.sent_off = true;
            }
        }
        st.players[AWAY][10].pos = Vec2::from_milli(-23_500, 5_500);
    });
    let hold = |sim: &mut MatchSimulation, runner_x: i64, moving: bool| {
        edit(sim, |st| {
            st.players[AWAY][5].pos = Vec2::from_milli(5_000, 0);
            st.players[AWAY][5].vel = Vec2::ZERO;
            st.ball.pos = Vec3::from_milli(4_600, 0, 0);
            st.ball.vel = Vec3::ZERO;
            st.ball.owner = Some((TeamSide::Away, 5));
            st.players[AWAY][10].pos = Vec2::from_milli(runner_x, 5_500);
            st.players[AWAY][10].vel = Vec2::from_milli(if moving { -7_000 } else { 0 }, 0);
        });
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
    };
    // Settle with the runner standing level with the line, so a back-liner picks him up.
    for _ in 0..240 {
        hold(&mut sim, -23_500, false);
    }
    let marker = {
        let st = sim.state();
        let m = &st.ai_memory.borrow()[HOME];
        (1..st.players[HOME].len()).find(|&i| m.mark_of[i] == 10)
    };
    let marker = marker.expect("a home defender marks the runner");
    assert_eq!(sim.state().players[HOME][marker].role, PlayerRole::Defender, "a back-liner owns the runner");
    // The runner breaks in behind at 7 m/s for 2 s.
    let mut x = -23_500_i64;
    let mut max_gap = 0.0_f64;
    for _ in 0..120 {
        x -= 7_000 / 60;
        hold(&mut sim, x, true);
        let st = sim.state();
        let m = &st.ai_memory.borrow()[HOME];
        assert_eq!(m.mark_of[marker], 10, "the marker keeps the runner in behind");
        max_gap = max_gap.max(f(st.players[HOME][marker].pos.dist(st.players[AWAY][10].pos)));
    }
    assert!(max_gap < 9.0, "the marker tracked the runner (max gap {max_gap:.1} m)");
    // A deeper teammate now stands on the runner's line to goal: the mark is handed over.
    let cover_idx = (1..sim.state().players[HOME].len()).find(|&i| i != marker && i != 4).unwrap();
    let mut released = false;
    for _ in 0..30 {
        let rx = x;
        edit(&mut sim, |st| {
            st.players[HOME][cover_idx].pos = Vec2::from_milli((rx - 52_500) / 2, 1_000);
        });
        hold(&mut sim, x, true);
        let st = sim.state();
        if st.ai_memory.borrow()[HOME].mark_of[marker] != 10 {
            released = true;
            break;
        }
    }
    assert!(released, "the marker hands the runner over once a cover exists");
}
