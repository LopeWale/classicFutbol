//! Goalkeeper scenario state machine: constructed-state scenario tests + CPU-vs-CPU behaviour metrics.
//!
//! All coordinates below are for the HOME keeper in the first half (own goal at `x = -half_length`).

use psfc_sim::ai::{decide_goalkeeper, goalkeeper_profile, goalkeeper_style, GkStyle};
use psfc_sim::{
    CpuDifficulty, FixtureSnapshot, Fp, GkScenario, InputFrame, MatchPhase, MatchSimulation,
    MatchState, PlayerActionState, SideController, TeamSide, Vec2, Vec3,
};

const HOME: usize = 0;
const AWAY: usize = 1;

fn edit(sim: &mut MatchSimulation, f: impl FnOnce(&mut MatchState)) {
    let mut st: MatchState = serde_json::from_slice(&sim.save_state().unwrap()).unwrap();
    f(&mut st);
    sim.load_state(&serde_json::to_vec(&st).unwrap()).unwrap();
}

/// Human-vs-human (idle inputs) open-play lab: every outfielder parked far up the pitch, both keepers on their line.
fn lab(eleven: bool) -> MatchSimulation {
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
        let hl = st.format.half_length().to_milli();
        let n = st.players[HOME].len();
        for side in [HOME, AWAY] {
            for idx in 1..n {
                let p = &mut st.players[side][idx];
                // Park everyone deep in the AWAY half, spread across the width.
                p.pos = Vec2::from_milli(
                    hl - 4_000 - (idx as i64) * 600,
                    -12_000 + (idx as i64) * 2_200,
                );
                p.vel = Vec2::ZERO;
                p.action = PlayerActionState::Normal;
            }
        }
        st.players[HOME][0].pos = Vec2::from_milli(-hl + 1_200, 0);
        st.players[HOME][0].vel = Vec2::ZERO;
        st.players[AWAY][0].pos = Vec2::from_milli(hl - 1_200, 0);
        st.ball.owner = None;
        st.ball.vel = Vec3::ZERO;
        st.ball.shot_launched_tick = 0;
        st.ball.pass_in_flight = None;
        st.ball.last_touch_team = None;
        st.ball.last_touch_player = None;
        st.restart_touch_state = None;
    });
    sim
}

fn half_len_mm(sim: &MatchSimulation) -> i64 {
    sim.state().format.half_length().to_milli()
}

/// Keeper depth (mm) from the HOME goal line.
fn home_gk_depth(sim: &MatchSimulation) -> i64 {
    sim.state().players[HOME][0].pos.x.to_milli() + half_len_mm(sim)
}

fn idle(sim: &mut MatchSimulation, ticks: u32) {
    for _ in 0..ticks {
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
    }
}

fn home_scenario(sim: &MatchSimulation) -> GkScenario {
    sim.state().gk_brain[HOME].scenario
}

#[test]
fn style_and_profile_derivation() {
    let sim = lab(true);
    let st = sim.state();
    // Sample keepers: Lammens (82 gk / 81 pos / 56 pace) is a line keeper, Raya (88 / 87) commanding.
    assert_eq!(
        goalkeeper_style(&st.players[HOME][0].attributes),
        GkStyle::Line
    );
    assert_eq!(
        goalkeeper_style(&st.players[AWAY][0].attributes),
        GkStyle::Commanding
    );
    let mut a = st.players[HOME][0].attributes;
    a.positioning = 88;
    a.pace = 72;
    assert_eq!(goalkeeper_style(&a), GkStyle::Sweeper);

    let mut sim2 = lab(true);
    edit(&mut sim2, |st| {
        st.rules.home_controller = SideController::Cpu(CpuDifficulty::Easy);
        st.rules.away_controller = SideController::Cpu(CpuDifficulty::Hard);
        st.players[AWAY][0].attributes = st.players[HOME][0].attributes;
    });
    let easy = goalkeeper_profile(sim2.state(), TeamSide::Home);
    let hard = goalkeeper_profile(sim2.state(), TeamSide::Away);
    assert!(easy.claim_margin_ticks > hard.claim_margin_ticks);
    assert!(easy.react_adjust_ticks > hard.react_adjust_ticks);
    assert!(easy.judgement_err_ticks > 0 && hard.judgement_err_ticks == 0);

    // Positioning drives set depth: elite keepers come further, poor ones hug the line.
    let mut sim3 = lab(true);
    edit(&mut sim3, |st| {
        st.players[HOME][0].attributes.positioning = 95;
        st.players[AWAY][0].attributes.positioning = 45;
    });
    let good = goalkeeper_profile(sim3.state(), TeamSide::Home);
    let poor = goalkeeper_profile(sim3.state(), TeamSide::Away);
    assert!(good.set_depth_bonus_mm >= 1_000, "{good:?}");
    assert!(poor.set_depth_bonus_mm < 0, "{poor:?}");
}

#[test]
fn keeper_stays_set_for_a_carrier_at_15m() {
    let mut sim = lab(false);
    let hl = half_len_mm(&sim);
    edit(&mut sim, |st| {
        // Away #2 (the controlled, idle player) holds the ball 15m out, slightly off centre.
        st.controlled_idx = [1, 2];
        st.players[AWAY][2].pos = Vec2::from_milli(-hl + 15_000, 2_000);
        st.players[AWAY][2].vel = Vec2::ZERO;
        st.ball.pos = Vec3::from_milli(-hl + 14_650, 2_000, 0);
        st.ball.owner = Some((TeamSide::Away, 2));
    });
    let (intent, brain) = decide_goalkeeper(sim.state(), TeamSide::Home, false);
    assert_eq!(brain.scenario, GkScenario::Set);
    assert!(!intent.sprint && intent.trigger_dive.is_none());

    let ga_mm = sim.state().format.penalty_box_depth_mm / 3;
    let mut max_depth = 0;
    for _ in 0..60 {
        idle(&mut sim, 1);
        if sim.state().ball.owner != Some((TeamSide::Away, 2)) {
            break;
        }
        assert!(
            !home_scenario(&sim).is_outward(),
            "{:?}",
            home_scenario(&sim)
        );
        max_depth = max_depth.max(home_gk_depth(&sim));
    }
    assert!(
        max_depth < ga_mm,
        "keeper must hold the set arc inside the goal area (depth {max_depth}mm)"
    );
    assert!(
        max_depth > 1_500,
        "keeper should step off the line to the set arc (depth {max_depth}mm)"
    );
}

#[test]
fn keeper_does_not_chase_a_loose_ball_an_attacker_reaches_first() {
    let mut sim = lab(false);
    let hl = half_len_mm(&sim);
    edit(&mut sim, |st| {
        st.controlled_idx = [1, 2];
        // Loose ball 7m out (inside the 5v5 box and inside the OLD 7.2m rush radius), attacker 0.9m away.
        st.ball.pos = Vec3::from_milli(-hl + 7_000, 500, 0);
        st.ball.vel = Vec3::from_milli(-400, 0, 0);
        st.ball.last_touch_team = Some(TeamSide::Away);
        st.players[AWAY][2].pos = Vec2::from_milli(-hl + 7_900, 600);
        st.players[AWAY][2].vel = Vec2::from_milli(-2_000, 0);
    });
    let (intent, brain) = decide_goalkeeper(sim.state(), TeamSide::Home, false);
    assert!(!brain.scenario.is_outward(), "{:?}", brain.scenario);
    assert!(!intent.sprint);
    let d0 = home_gk_depth(&sim);
    idle(&mut sim, 20);
    assert!(
        home_gk_depth(&sim) < d0 + 1_600,
        "keeper must not charge at a ball the attacker reaches first ({}mm -> {}mm)",
        d0,
        home_gk_depth(&sim)
    );
}

#[test]
fn keeper_collects_a_loose_ball_in_the_box_it_is_first_to() {
    let mut sim = lab(false);
    let hl = half_len_mm(&sim);
    edit(&mut sim, |st| {
        st.ball.pos = Vec3::from_milli(-hl + 6_000, -800, 0);
        st.ball.vel = Vec3::from_milli(-1_500, 0, 0);
        st.ball.last_touch_team = Some(TeamSide::Away);
        st.ball.last_touch_player = Some((TeamSide::Away, 3));
    });
    let (_, brain) = decide_goalkeeper(sim.state(), TeamSide::Home, false);
    assert_eq!(brain.scenario, GkScenario::Collect);
    let mut got_it = false;
    for _ in 0..90 {
        idle(&mut sim, 1);
        if sim.state().ball.owner == Some((TeamSide::Home, 0)) {
            got_it = true;
            break;
        }
    }
    assert!(
        got_it,
        "unchallenged loose ball in the box must be collected"
    );
}

#[test]
fn sweeper_keeper_sweeps_a_through_ball_it_clearly_reaches_first() {
    let mut sim = lab(true);
    let hl = half_len_mm(&sim);
    edit(&mut sim, |st| {
        // Sweeper profile (positioning 88, pace 74).
        st.players[HOME][0].attributes.positioning = 88;
        st.players[HOME][0].attributes.pace = 74;
        st.players[HOME][0].pos = Vec2::from_milli(-hl + 3_000, 0);
        st.controlled_idx = [1, 5];
        // Through ball played in behind: rolling toward goal from 24m, attacker chasing 8m behind it.
        st.ball.pos = Vec3::from_milli(-hl + 24_000, 2_000, 0);
        st.ball.vel = Vec3::from_milli(-6_200, -500, 0);
        st.ball.last_touch_team = Some(TeamSide::Away);
        st.ball.last_touch_player = Some((TeamSide::Away, 9));
        st.players[AWAY][9].pos = Vec2::from_milli(-hl + 32_000, 2_000);
        st.players[AWAY][9].vel = Vec2::from_milli(-5_000, 0);
    });
    let mut swept = false;
    let mut gk_first = false;
    for _ in 0..120 {
        idle(&mut sim, 1);
        swept |= home_scenario(&sim) == GkScenario::Sweep;
        let st = sim.state();
        if st.ball.last_touch_player == Some((TeamSide::Home, 0)) {
            gk_first = true;
            break;
        }
        if st.ball.last_touch_team == Some(TeamSide::Away) && st.ball.owner.is_some() {
            break;
        }
    }
    assert!(swept, "sweeper keeper must enter the Sweep scenario");
    assert!(
        gk_first,
        "sweeper keeper must reach the through ball before the attacker"
    );

    // The same ball against a line keeper outside his sweep radius: no sweep, he stays set.
    let mut sim = lab(true);
    edit(&mut sim, |st| {
        st.players[HOME][0].attributes.positioning = 60;
        st.players[HOME][0].attributes.pace = 50;
        st.players[HOME][0].pos = Vec2::from_milli(-hl + 3_000, 0);
        st.controlled_idx = [1, 9];
        st.ball.pos = Vec3::from_milli(-hl + 30_000, 4_000, 0);
        st.ball.vel = Vec3::from_milli(-6_000, -700, 0);
        st.ball.last_touch_team = Some(TeamSide::Away);
        st.players[AWAY][9].pos = Vec2::from_milli(-hl + 36_000, 4_000);
        st.players[AWAY][9].vel = Vec2::from_milli(-5_000, 0);
    });
    assert_eq!(
        goalkeeper_style(&sim.state().players[HOME][0].attributes),
        GkStyle::Line
    );
    let (_, brain) = decide_goalkeeper(sim.state(), TeamSide::Home, false);
    assert!(
        !matches!(brain.scenario, GkScenario::Sweep),
        "line keeper must not sweep beyond his radius ({:?})",
        brain.scenario
    );
}

/// High ball from the right wing aimed to drop at `(depth, y)`; returns the launch state edit.
fn cross_to(st: &mut MatchState, depth_mm: i64, y_mm: i64) {
    let hl = st.format.half_length().to_milli();
    // Launch 18m wide at the same depth; 1.0s flight -> vy = -(18 - y)/1.0 m/s; vz for a 1.0s parabola from 0.5m.
    st.ball.pos = Vec3::from_milli(-hl + depth_mm, 18_000, 500);
    st.ball.vel = Vec3::from_milli(0, -(18_000 - y_mm), 5_400);
    st.ball.last_touch_team = Some(TeamSide::Away);
    st.ball.last_touch_player = Some((TeamSide::Away, 7));
    st.ball.spin_side = Fp::ZERO;
    st.ball.spin_top = Fp::ZERO;
}

#[test]
fn keeper_claims_an_unchallenged_cross_in_the_six_yard_box() {
    let mut sim = lab(true);
    edit(&mut sim, |st| cross_to(st, 3_500, 800));
    let mut claimed_scenario = false;
    let mut got_it = false;
    for _ in 0..110 {
        idle(&mut sim, 1);
        if home_scenario(&sim) == GkScenario::CrossClaim {
            claimed_scenario = true;
        }
        if sim.state().ball.owner == Some((TeamSide::Home, 0))
            || sim.state().ball.last_touch_player == Some((TeamSide::Home, 0))
        {
            got_it = true;
            break;
        }
    }
    assert!(
        claimed_scenario,
        "unchallenged cross into the 6-yard box must be claimed"
    );
    assert!(got_it, "keeper must take the cross");
}

#[test]
fn keeper_holds_for_a_cross_to_the_penalty_spot_with_attackers_closer() {
    let mut sim = lab(true);
    let hl = half_len_mm(&sim);
    edit(&mut sim, |st| {
        cross_to(st, 11_000, 0);
        // Two attackers attacking the penalty spot.
        st.players[AWAY][9].pos = Vec2::from_milli(-hl + 11_800, 1_200);
        st.players[AWAY][10].pos = Vec2::from_milli(-hl + 12_200, -1_500);
        st.players[AWAY][9].vel = Vec2::ZERO;
        st.players[AWAY][10].vel = Vec2::ZERO;
        st.controlled_idx = [1, 9];
    });
    let mut max_depth = 0;
    for _ in 0..70 {
        idle(&mut sim, 1);
        let sc = home_scenario(&sim);
        assert!(
            sc != GkScenario::CrossClaim && sc != GkScenario::Sweep,
            "{sc:?}"
        );
        max_depth = max_depth.max(home_gk_depth(&sim));
        if sim.state().ball.pos.z < Fp::from_milli(300) {
            break;
        }
    }
    let ga_mm = sim.state().format.penalty_box_depth_mm / 3;
    assert!(
        max_depth < ga_mm,
        "keeper must not leave the 6-yard box for a contested penalty-spot cross ({max_depth}mm)"
    );
}

#[test]
fn one_v_one_approach_stops_short_and_does_not_dive_early() {
    let mut sim = lab(true);
    let hl = half_len_mm(&sim);
    edit(&mut sim, |st| {
        st.controlled_idx = [1, 9];
        st.players[AWAY][9].pos = Vec2::from_milli(-hl + 9_000, 1_000);
        st.players[AWAY][9].vel = Vec2::ZERO;
        st.ball.pos = Vec3::from_milli(-hl + 8_650, 1_000, 0);
        st.ball.owner = Some((TeamSide::Away, 9));
    });
    let (_, brain) = decide_goalkeeper(sim.state(), TeamSide::Home, false);
    assert_eq!(brain.scenario, GkScenario::OneVOne);
    let mut dived = false;
    for _ in 0..90 {
        idle(&mut sim, 1);
        if matches!(
            sim.state().players[HOME][0].action,
            PlayerActionState::Diving { .. }
        ) {
            dived = true;
        }
    }
    let st = sim.state();
    assert_eq!(
        st.ball.owner,
        Some((TeamSide::Away, 9)),
        "keeper must not lunge at the carrier"
    );
    let gap = st.players[HOME][0].pos.dist(st.ball.pos.xy()).to_milli();
    assert!(
        (1_400..=2_900).contains(&gap),
        "keeper must stand big ~1.5-2.5m from the ball (gap {gap}mm)"
    );
    assert!(
        st.players[HOME][0].vel.len() < Fp::from_milli(1_200),
        "keeper must have decelerated"
    );
    assert!(!dived, "keeper must not dive before the shot");
    assert!(
        home_gk_depth(&sim) > 4_000,
        "keeper must have advanced to narrow the angle"
    );

    // A defender recovering goal-side in the shooting lane aborts the 1v1.
    edit(&mut sim, |st| {
        st.players[HOME][1].pos = Vec2::from_milli(-hl + 7_000, 800);
        st.players[HOME][1].vel = Vec2::ZERO;
    });
    let (_, brain) = decide_goalkeeper(sim.state(), TeamSide::Home, false);
    assert!(
        !matches!(brain.scenario, GkScenario::OneVOne),
        "covering defender must abort the 1v1 ({:?})",
        brain.scenario
    );
}

#[test]
fn human_gk_rush_held_rushes_and_release_recovers() {
    let mut sim = lab(false);
    let hl = half_len_mm(&sim);
    edit(&mut sim, |st| {
        st.controlled_idx = [1, 2];
        st.players[AWAY][2].pos = Vec2::from_milli(-hl + 13_000, 0);
        st.players[AWAY][2].vel = Vec2::ZERO;
        st.ball.pos = Vec3::from_milli(-hl + 12_650, 0, 0);
        st.ball.owner = Some((TeamSide::Away, 2));
    });
    let rush = InputFrame {
        buttons: psfc_sim::BTN_GK_RUSH,
        ..InputFrame::IDLE
    };
    for _ in 0..40 {
        sim.tick(&[rush, InputFrame::IDLE]);
    }
    assert_eq!(home_scenario(&sim), GkScenario::HumanRush);
    let rushed = home_gk_depth(&sim);
    assert!(rushed > 5_000, "held GK rush must come out ({rushed}mm)");
    idle(&mut sim, 90);
    assert!(
        home_gk_depth(&sim) < rushed - 1_500,
        "released GK rush must recover toward the set position ({}mm)",
        home_gk_depth(&sim)
    );
}

#[test]
fn brain_is_hashed_and_old_saves_without_it_still_load() {
    let mut sim = lab(false);
    let h0 = sim.state_hash();
    edit(&mut sim, |st| st.gk_brain[HOME].commit_ticks = 7);
    assert_ne!(
        h0,
        sim.state_hash(),
        "goalkeeper brain must be part of the state hash"
    );

    let mut v: serde_json::Value = serde_json::from_slice(&sim.save_state().unwrap()).unwrap();
    v.as_object_mut().unwrap().remove("gk_brain");
    sim.load_state(&serde_json::to_vec(&v).unwrap()).unwrap();
    assert_eq!(sim.state().gk_brain[HOME].commit_ticks, 0);
}

// ---------------------------------------------------------------------------------------------
// CPU vs CPU behaviour metrics.
// ---------------------------------------------------------------------------------------------

#[derive(Debug, Default, Clone, Copy)]
struct KeeperMetrics {
    open_ticks: u64,
    outside_box_ticks: u64,
    /// Leaving the 6-yard box (depth `box/3`, half width `goal/2 + box/3`) while moving toward the ball.
    rush_events: u64,
    /// Rush events after which the keeper touched the ball within 1.5s.
    rush_won: u64,
    scenario_switches: u64,
    /// Switches out of a scenario that lasted <= 4 ticks (tick-to-tick flicker).
    flicker_switches: u64,
    /// GK outward/inward velocity reversals (1 m/s dead-band).
    direction_flips: u64,
    goals_conceded: u64,
}

#[allow(clippy::too_many_lines)]
fn cpu_match_metrics(eleven: bool, seed: u64, half_ticks: u32) -> [KeeperMetrics; 2] {
    let mut snap = if eleven {
        FixtureSnapshot::sample_eleven_a_side(seed)
    } else {
        FixtureSnapshot::sample_five_a_side(seed)
    };
    snap.rules.half_duration_ticks = half_ticks;
    snap.rules.home_controller = SideController::Cpu(CpuDifficulty::Hard);
    snap.rules.away_controller = SideController::Cpu(CpuDifficulty::Hard);
    let mut sim = MatchSimulation::new(&snap).unwrap();
    let mut m = [KeeperMetrics::default(); 2];
    let mut in_ga = [true; 2];
    let mut dir_state = [0_i8; 2];
    let mut prev_sc = [GkScenario::Set; 2];
    let mut since = [0_u32; 2];
    let mut rush_open: [Option<u32>; 2] = [None; 2];
    for _ in 0..(half_ticks * 2 + 20_000) {
        sim.tick(&[InputFrame::IDLE, InputFrame::IDLE]);
        let st = sim.state();
        if matches!(st.phase, MatchPhase::FullTime) {
            break;
        }
        if !matches!(st.phase, MatchPhase::OpenPlay) {
            in_ga = [true; 2];
            continue;
        }
        for side in [TeamSide::Home, TeamSide::Away] {
            let s = side.index();
            let km = &mut m[s];
            km.open_ticks += 1;
            let sign = side.attack_sign(st.second_half_swapped());
            let goal_x = Fp::from_milli(-sign * st.format.half_length().to_milli());
            let gk = &st.players[s][0];
            let depth = (gk.pos.x - goal_x).abs();
            let box_d = st.format.penalty_box_depth();
            if depth > box_d || gk.pos.y.abs() > st.format.penalty_box_half_width() {
                km.outside_box_ticks += 1;
            }
            let ga_d = Fp::from_milli(box_d.to_milli() / 3);
            let now_in = depth <= ga_d && gk.pos.y.abs() <= st.format.goal_half_width() + ga_d;
            if in_ga[s]
                && !now_in
                && st.ball.owner != Some((side, 0))
                && (st.ball.pos.xy() - gk.pos).dot(gk.vel) > Fp::ZERO
            {
                km.rush_events += 1;
                rush_open[s] = Some(st.tick);
            }
            in_ga[s] = now_in;
            if let Some(t0) = rush_open[s] {
                if st.tick > t0 && st.ball.last_touch_player == Some((side, 0)) {
                    km.rush_won += 1;
                    rush_open[s] = None;
                } else if st.tick > t0 + 90 {
                    rush_open[s] = None;
                }
            }
            let sc = st.gk_brain[s].scenario;
            since[s] += 1;
            if sc != prev_sc[s] {
                km.scenario_switches += 1;
                if since[s] <= 4 {
                    km.flicker_switches += 1;
                }
                since[s] = 0;
                prev_sc[s] = sc;
            }
            let out_v = gk.vel.x.mul(Fp::from_int(sign as i32));
            let d = if out_v > Fp::from_milli(1_000) {
                1
            } else if out_v < Fp::from_milli(-1_000) {
                -1
            } else {
                0
            };
            if d != 0 {
                if dir_state[s] != 0 && d != dir_state[s] {
                    km.direction_flips += 1;
                }
                dir_state[s] = d;
            }
        }
    }
    m[0].goals_conceded = u64::from(sim.state().score[1]);
    m[1].goals_conceded = u64::from(sim.state().score[0]);
    m
}

/// 3 seeds x 10 minutes, 5v5 and 11v11, Hard vs Hard (1 seed x 2 minutes in unoptimised debug builds).
///
/// Bounds (per keeper, per match, scaled to 10 minutes of open play):
/// - outside the penalty box `< 2%` of open-play ticks;
/// - rush events (leaving the 6-yard box toward the ball) `<= 16` per 10 minutes (unmodified code: 32-44 in 5v5);
/// - scenario switches `<= 60` per minute (most are genuine Set <-> Support possession changes);
/// - flicker (leaving a scenario after <= 4 ticks) `<= 6` per minute.
#[test]
fn cpu_vs_cpu_keeper_metrics_within_bounds() {
    let (seeds, half_ticks): (&[u64], u32) = if cfg!(debug_assertions) {
        (&[11], 3_600)
    } else {
        (&[11, 22, 33], 18_000)
    };
    let mut handles = Vec::new();
    for &seed in seeds {
        for eleven in [false, true] {
            handles.push(std::thread::spawn(move || {
                (eleven, seed, cpu_match_metrics(eleven, seed, half_ticks))
            }));
        }
    }
    for h in handles {
        let (eleven, seed, ms) = h.join().unwrap();
        for (s, km) in ms.iter().enumerate() {
            let open = km.open_ticks.max(1);
            let per_10min = |v: u64| v * 36_000 / open;
            let per_min_x10 = |v: u64| v * 36_000 / open;
            println!(
                "{} seed {seed} side {s}: open {} outside_box {}.{:02}% rushes/10min {} (won {}) switches/min {}.{} flicker/min {}.{} flips/min {}.{} conceded {}",
                if eleven { "11v11" } else { "5v5" },
                km.open_ticks,
                km.outside_box_ticks * 100 / open,
                (km.outside_box_ticks * 10_000 / open) % 100,
                per_10min(km.rush_events),
                per_10min(km.rush_won),
                per_min_x10(km.scenario_switches) / 10,
                per_min_x10(km.scenario_switches) % 10,
                per_min_x10(km.flicker_switches) / 10,
                per_min_x10(km.flicker_switches) % 10,
                per_min_x10(km.direction_flips) / 10,
                per_min_x10(km.direction_flips) % 10,
                km.goals_conceded,
            );
            assert!(
                km.outside_box_ticks * 50 < open,
                "keeper outside the box for >= 2% of open play ({} of {})",
                km.outside_box_ticks,
                open
            );
            assert!(
                per_10min(km.rush_events) <= 16,
                "too many rush events: {km:?}"
            );
            assert!(
                per_min_x10(km.scenario_switches) <= 600,
                "scenario churn: {km:?}"
            );
            assert!(
                per_min_x10(km.flicker_switches) <= 60,
                "scenario flicker: {km:?}"
            );
        }
    }
}
