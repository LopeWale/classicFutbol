//! Deterministic Goalkeeper AI, Outfield Teammate AI, and CPU Opponent Controller.

use crate::config::{CpuDifficulty, TeamSide};
use crate::input::{
    InputFrame, BTN_JOCKEY, BTN_LOB, BTN_PASS, BTN_SHOOT, BTN_SKILL, BTN_SPRINT, BTN_SWITCH,
    BTN_TACKLE, BTN_THROUGH,
};
use crate::math::{Fp, Vec2};
use crate::snapshot::PlayerRole;
use crate::state::{MatchPhase, MatchState, PlayerActionState, RestartKind};

#[derive(Clone, Copy, Debug)]
pub struct AiMoveIntent {
    pub move_dir: Vec2,
    pub sprint: bool,
    pub jockey: bool,
    /// Optional `(horizontal_dive_dir, vertical_takeoff_vz_mm_s)` for low skimming vs airborne top-corner dives.
    pub trigger_dive: Option<(Vec2, i64)>,
}

/// Computes goalkeeper movement, angle-bisector arc, 1v1 rush, high-cross command & 3D shot-stopping dive intent.
pub fn compute_goalkeeper_intent(
    state: &MatchState,
    side: TeamSide,
    human_gk_rush: bool,
) -> AiMoveIntent {
    let p = &state.players[side.index()][0];
    let is_5v5 = state.format.players_per_side <= 6;
    let attack_sign = side.attack_sign(state.second_half_swapped());
    let half_len = state.format.half_length();
    let goal_half_w = state.format.goal_half_width();
    let box_depth = state.format.penalty_box_depth();
    let box_half_w = state.format.penalty_box_half_width();

    // Own goal center X: if attacking +X (sign = +1), own goal is at -half_len.
    let own_goal_x = Fp::from_milli(-attack_sign * half_len.to_milli());
    let into_pitch_sign = Fp::from_int(attack_sign as i32);

    // If holding the ball, step toward the top of the 6-yard area biased toward the safest outlet flank.
    if state.ball.owner == Some((side, 0)) {
        let (_, _, safe_aim) = choose_safe_gk_distribution(state, side);
        let target = Vec2::new(
            own_goal_x + into_pitch_sign.mul(Fp::from_milli(if is_5v5 { 3_400 } else { 4_600 })),
            safe_aim.y.mul(Fp::from_milli(1_400)).clamp(-goal_half_w, goal_half_w),
        );
        let delta = target - p.pos;
        if delta.len() > Fp::from_milli(320) {
            return AiMoveIntent {
                move_dir: delta.normalize_or_zero(),
                sprint: false,
                jockey: false,
                trigger_dive: None,
            };
        }
        return AiMoveIntent {
            move_dir: Vec2::ZERO,
            sprint: false,
            jockey: false,
            trigger_dive: None,
        };
    }

    // Defending Corner Kick stance: hold central/near-six-yard command spot ready to claim or parry!
    if let MatchPhase::SetPiece {
        kind: RestartKind::CornerKick,
        team,
        spot,
        ..
    } = state.phase
    {
        if team != side {
            let y_bias = if spot.y >= Fp::ZERO {
                Fp::from_milli(360)
            } else {
                Fp::from_milli(-360)
            };
            let ck_target = Vec2::new(
                own_goal_x + into_pitch_sign.mul(Fp::from_milli(if is_5v5 { 1_300 } else { 1_650 })),
                y_bias,
            );
            let d = ck_target - p.pos;
            return AiMoveIntent {
                move_dir: if d.len() > Fp::from_milli(150) {
                    d.normalize_or_zero()
                } else {
                    Vec2::ZERO
                },
                sprint: false,
                jockey: true,
                trigger_dive: None,
            };
        }
    }

    let ball_xy = state.ball.pos.xy();
    let ball_vel_xy = state.ball.vel.xy();

    // Check incoming shot threat toward own goal!
    // Velocity toward own goal has sign `-attack_sign`.
    let vel_toward_goal = if attack_sign > 0 {
        -ball_vel_xy.x
    } else {
        ball_vel_xy.x
    };
    let dist_ball_to_goal_x = (ball_xy.x - own_goal_x).abs();
    let max_shot_react_dist = Fp::from_milli(if is_5v5 { 24_500 } else { 30_500 });

    if state.ball.owner.is_none()
        && matches!(p.action, PlayerActionState::Normal)
        && vel_toward_goal > Fp::from_milli(5_200)
        && dist_ball_to_goal_x < max_shot_react_dist
        && dist_ball_to_goal_x > Fp::from_milli(240)
    {
        // Project 3D `(Y, Z)` at GK's X plane including aerodynamic Magnus curl (`spin_side`), gravity & turf bounce!
        let dist_to_gk_x = (ball_xy.x - p.pos.x).abs();
        let t_to_gk = dist_to_gk_x.div(vel_toward_goal);
        let b_spd = ball_vel_xy.len();
        let magnus_y = if state.ball.pos.z > Fp::from_milli(35)
            && b_spd > Fp::from_milli(1_800)
            && state.ball.spin_side.abs() > Fp::from_milli(45)
        {
            let perp_y = ball_vel_xy.normalize_or_zero().x;
            let spd_scale = b_spd
                .div(Fp::from_milli(15_500))
                .clamp(Fp::from_milli(450), Fp::from_milli(1_380));
            let a_y = perp_y
                .mul(state.ball.spin_side)
                .mul(Fp::from_milli(1_050))
                .mul(spd_scale);
            a_y.mul(t_to_gk.mul(t_to_gk)).mul(Fp::HALF)
        } else {
            Fp::ZERO
        };
        let projected_y = ball_xy.y + ball_vel_xy.y.mul(t_to_gk) + magnus_y;
        let raw_proj_z = state.ball.pos.z
            + state.ball.vel.z.mul(t_to_gk)
            - Fp::from_milli(4_905).mul(t_to_gk.mul(t_to_gk));
        let projected_z = if raw_proj_z < Fp::ZERO {
            (-raw_proj_z).mul(Fp::from_milli(520)).min(Fp::from_milli(950))
        } else {
            raw_proj_z
        };
        let max_guard_y = goal_half_w + Fp::from_milli(1_050);

        if projected_y.abs() <= max_guard_y {
            let lateral_gap = projected_y - p.pos.y;
            let gk_skill = p.attributes.goalkeeping as i64;
            let dive_reach = Fp::from_milli(1_650 + gk_skill * 12); // ~2.55m - 2.75m fingertip reach
            let close_enough_in_time = t_to_gk <= Fp::from_milli(440) || dist_to_gk_x <= Fp::from_milli(9_500);
            if close_enough_in_time
                && lateral_gap.abs() > Fp::from_milli(470)
                && lateral_gap.abs() <= dive_reach
            {
                let dive_dir = Vec2::new(
                    into_pitch_sign.mul(Fp::from_milli(190)),
                    Fp::from_int(lateral_gap.signum_int()),
                )
                .normalize_or_zero();
                // High/top-corner shots trigger airborne layout extension (`IMG_0364`); low skimmers stay low (`IMG_0358`, `IMG_0366`)!
                let dive_vz_mm = if projected_z > Fp::from_milli(880) {
                    let z_excess = (projected_z.to_milli() - 880).clamp(0, 1_600);
                    (2_150 + (z_excess * 12) / 10).clamp(2_150, 4_000)
                } else {
                    280
                };
                return AiMoveIntent {
                    move_dir: dive_dir,
                    sprint: true,
                    jockey: false,
                    trigger_dive: Some((dive_dir, dive_vz_mm)),
                };
            } else {
                let intercept = Vec2::new(p.pos.x, projected_y.clamp(-goal_half_w, goal_half_w));
                return AiMoveIntent {
                    move_dir: (intercept - p.pos).normalize_or_zero(),
                    sprint: true,
                    jockey: true,
                    trigger_dive: None,
                };
            }
        }
    }

    // High Aerial Cross / Corner Command Zone (`IMG_0365.jpeg`!): step out to pluck/punch high balls in the 6-yard area!
    let cross_cmd_depth = Fp::from_milli(if is_5v5 { 5_800 } else { 7_600 });
    if state.ball.owner.is_none()
        && matches!(state.phase, MatchPhase::OpenPlay)
        && state.ball.pos.z > Fp::from_milli(1_200)
        && dist_ball_to_goal_x <= cross_cmd_depth
        && ball_xy.y.abs() <= goal_half_w + Fp::from_milli(2_500)
    {
        let claim_xy = predict_ball_xy_at_ticks(state, 11);
        let clamped_claim = Vec2::new(
            if attack_sign > 0 {
                claim_xy.x.clamp(own_goal_x + Fp::from_milli(600), own_goal_x + cross_cmd_depth)
            } else {
                claim_xy.x.clamp(own_goal_x - cross_cmd_depth, own_goal_x - Fp::from_milli(600))
            },
            claim_xy.y.clamp(-(goal_half_w + Fp::from_milli(2_000)), goal_half_w + Fp::from_milli(2_000)),
        );
        return AiMoveIntent {
            move_dir: (clamped_claim - p.pos).normalize_or_zero(),
            sprint: true,
            jockey: false,
            trigger_dive: None,
        };
    }

    // Rush out on loose ball inside box, explicit human GK rush (`BTN_CALL_RUN`), OR automatic 1v1 breakaway angle-narrowing!
    let ball_in_own_box = dist_ball_to_goal_x <= box_depth && ball_xy.y.abs() <= box_half_w;
    let close_loose_ball = state.ball.owner.is_none()
        && ball_in_own_box
        && p.pos.dist(ball_xy) < Fp::from_milli(7_200);

    if (close_loose_ball || (human_gk_rush && dist_ball_to_goal_x < box_depth + Fp::from_milli(4_500)))
        && matches!(state.phase, MatchPhase::OpenPlay)
    {
        return AiMoveIntent {
            move_dir: (ball_xy - p.pos).normalize_or_zero(),
            sprint: true,
            jockey: false,
            trigger_dive: None,
        };
    }

    // Automatic 1v1 Breakaway Containment: when an unopposed striker enters the box with the ball,
    // step off the line along the goal-to-ball ray to cut down the shooting angle!
    let goal_center = Vec2::new(own_goal_x, Fp::ZERO);
    if matches!(state.phase, MatchPhase::OpenPlay)
        && state.ball.owner.map(|(s, _)| s) == Some(side.opponent())
        && ball_in_own_box
        && dist_ball_to_goal_x >= Fp::from_milli(2_200)
    {
        let mut defender_covering = false;
        for mate in state.players[side.index()].iter().skip(1) {
            if mate.sent_off {
                continue;
            }
            let d_carrier = mate.pos.dist(ball_xy);
            let goal_side = (mate.pos.x - ball_xy.x).mul(Fp::from_int(-attack_sign as i32)) > Fp::from_milli(-400);
            if d_carrier <= Fp::from_milli(2_650) && goal_side {
                defender_covering = true;
                break;
            }
        }
        if !defender_covering {
            let to_ball = ball_xy - goal_center;
            let dist_gb = to_ball.len().max(Fp::ONE);
            let rush_r = (dist_gb.mul(Fp::from_milli(520)))
                .clamp(Fp::from_milli(1_600), (dist_gb - Fp::from_milli(1_150)).max(Fp::from_milli(1_600)));
            let contain_pt = goal_center + to_ball.normalize_or_zero().scale(rush_r);
            let delta = contain_pt - p.pos;
            return AiMoveIntent {
                move_dir: if delta.len() > Fp::from_milli(200) {
                    delta.normalize_or_zero()
                } else {
                    Vec2::ZERO
                },
                sprint: delta.len() > Fp::from_milli(1_100),
                jockey: delta.len() <= Fp::from_milli(1_400),
                trigger_dive: None,
            };
        }
    }

    // True Angle-Bisector Arc Positioning between `(own_goal_x, 0)` and `ball_xy`!
    let to_ball = ball_xy - goal_center;
    let dist_gb = to_ball.len().max(Fp::ONE);
    let dir_gb = to_ball.normalize_or_zero();
    let max_arc_r_mm = if is_5v5 { 3_250 } else { 4_250 };
    let min_arc_r_mm = if is_5v5 { 1_280 } else { 1_450 };
    let scaled_r_mm = min_arc_r_mm
        + ((dist_gb.to_milli().clamp(4_000, 34_000) - 4_000) * (max_arc_r_mm - min_arc_r_mm)) / 30_000;
    let raw_arc = goal_center + dir_gb.scale(Fp::from_milli(scaled_r_mm));

    let min_line_clear = Fp::from_milli(if is_5v5 { 820 } else { 950 });
    let target_x = if attack_sign > 0 {
        raw_arc.x.max(own_goal_x + min_line_clear)
    } else {
        raw_arc.x.min(own_goal_x - min_line_clear)
    };
    let max_y = (goal_half_w - Fp::from_milli(180)).max(Fp::from_milli(850));
    let target_y = raw_arc.y.clamp(-max_y, max_y);
    let target = Vec2::new(target_x, target_y);

    let delta = target - p.pos;
    let dist = delta.len();
    if dist < Fp::from_milli(160) {
        AiMoveIntent {
            move_dir: Vec2::ZERO,
            sprint: false,
            jockey: true,
            trigger_dive: None,
        }
    } else {
        AiMoveIntent {
            move_dir: delta.normalize_or_zero(),
            sprint: dist > Fp::from_milli(1_450),
            jockey: dist < Fp::from_milli(1_150),
            trigger_dive: None,
        }
    }
}

/// Scans the pitch for the safest GK outlet pass or lofted clearance so the Goalkeeper
/// never distributes blindly into an opposing striker standing in the box.
pub fn choose_safe_gk_distribution(state: &MatchState, side: TeamSide) -> (u16, u16, Vec2) {
    let s_idx = side.index();
    let opp_idx = side.opponent().index();
    let gk_pos = state.players[s_idx][0].pos;
    let is_5v5 = state.format.players_per_side <= 6;
    let attack_sign = side.attack_sign(state.second_half_swapped());
    let forward_vec = Vec2::new(Fp::from_int(attack_sign as i32), Fp::ZERO);

    let mut best_short: Option<(Vec2, Fp, Fp)> = None; // (dir, dist, score)
    let mut best_long: Option<(Vec2, Fp, Fp)> = None;

    for mate in state.players[s_idx].iter().skip(1) {
        if mate.sent_off {
            continue;
        }
        let to_mate = mate.pos - gk_pos;
        let dist = to_mate.len();
        if dist < Fp::from_milli(3_200) {
            continue;
        }
        let dir = to_mate.normalize_or_zero();

        let mut opp_clear = Fp::from_milli(18_000);
        let mut lane_clear = Fp::from_milli(12_000);
        for opp in &state.players[opp_idx] {
            if opp.sent_off {
                continue;
            }
            opp_clear = opp_clear.min(opp.pos.dist(mate.pos));
            let rel = opp.pos - gk_pos;
            let along = rel.dot(dir);
            if along > Fp::from_milli(900) && along < dist + Fp::from_milli(600) {
                let closest = gk_pos + dir.scale(along);
                lane_clear = lane_clear.min(opp.pos.dist(closest));
            }
        }

        let fwd_gain = (mate.pos.x - gk_pos.x).mul(Fp::from_int(attack_sign as i32));
        let short_max = Fp::from_milli(if is_5v5 { 24_000 } else { 32_000 });
        if dist <= short_max && lane_clear >= Fp::from_milli(2_250) && opp_clear >= Fp::from_milli(2_750) {
            let short_score = opp_clear.mul(Fp::from_milli(2_100))
                + lane_clear.mul(Fp::from_milli(2_400))
                + fwd_gain.mul(Fp::from_milli(220))
                - (dist - Fp::from_milli(11_500)).abs().mul(Fp::from_milli(320));
            if best_short.is_none_or(|(_, _, bs)| short_score > bs) {
                best_short = Some((dir, dist, short_score));
            }
        }

        let long_score = opp_clear.mul(Fp::from_milli(1_800)) + fwd_gain.mul(Fp::from_milli(420));
        if best_long.is_none_or(|(_, _, bl)| long_score > bl) {
            best_long = Some((dir, dist, long_score));
        }
    }

    if let Some((dir, dist, _)) = best_short {
        let charge = (6 + (dist.to_milli() / 1_450)).clamp(7, 20) as u16;
        return (BTN_PASS, charge, dir);
    }

    if let Some((dir, dist, _)) = best_long {
        let charge = (13 + (dist.to_milli() / 2_100)).clamp(14, 24) as u16;
        return (BTN_LOB, charge, dir);
    }

    let flank_y = if gk_pos.y >= Fp::ZERO {
        Fp::from_milli(-380)
    } else {
        Fp::from_milli(380)
    };
    (
        BTN_LOB,
        18,
        Vec2::new(forward_vec.x, flank_y).normalize_or_zero(),
    )
}

/// Deterministically projects the ball's `(x, y)` ground coordinates `ticks_ahead` into the future
/// (`6`-tick / `100ms` coarse steps) accounting for velocity, horizontal Magnus curve, and turf/air drag.
pub fn predict_ball_xy_at_ticks(state: &MatchState, ticks_ahead: u32) -> Vec2 {
    if ticks_ahead == 0 {
        return state.ball.pos.xy();
    }
    if let Some((owner_side, owner_idx)) = state.ball.owner {
        let p = &state.players[owner_side.index()][owner_idx];
        let dt_total = Fp::from_ratio(ticks_ahead.min(18) as i64, 60);
        return (state.ball.pos.xy() + p.vel.scale(dt_total)).clamp_box(
            state.format.half_length(),
            state.format.half_width(),
        );
    }

    let mut pos = state.ball.pos.xy();
    let mut vel = state.ball.vel.xy();
    let mut z = state.ball.pos.z;
    let mut vz = state.ball.vel.z;
    let mut spin_side = state.ball.spin_side;
    let is_5v5 = state.format.players_per_side <= 6;
    let half_len = state.format.half_length();
    let half_wid = state.format.half_width();

    let mut rem = ticks_ahead.min(90);
    while rem > 0 {
        let step = rem.min(6);
        rem -= step;
        let dt = Fp::from_ratio(step as i64, 60);

        let speed = vel.len();
        if speed > Fp::from_milli(2_000) && spin_side.abs() > Fp::from_milli(50) {
            let perp = Vec2::new(-vel.y, vel.x).normalize_or_zero();
            let mag = spin_side.mul(Fp::from_milli(980));
            vel += perp.scale(mag.mul(dt));
            spin_side = spin_side.mul(Fp::from_milli(942));
        }

        if z > Fp::from_milli(60) {
            vel = vel.scale(Fp::from_milli(965));
            vz -= Fp::from_milli(9_810).mul(dt);
        } else {
            let decel = Fp::from_milli(if is_5v5 { 2_250 } else { 2_450 }).mul(dt);
            if speed <= decel {
                vel = Vec2::ZERO;
            } else {
                vel = (vel - vel.normalize_or_zero().scale(decel)).scale(Fp::from_milli(970));
            }
            z = Fp::ZERO;
            if vz < Fp::ZERO {
                vz = Fp::ZERO;
            }
        }

        pos = (pos + vel.scale(dt)).clamp_box(half_len, half_wid);
        z += vz.mul(dt);
        if z < Fp::ZERO {
            z = Fp::ZERO;
            if vz < Fp::from_milli(-850) {
                vz = (-vz).mul(Fp::from_milli(610));
                vel = vel.scale(Fp::from_milli(880));
            } else {
                vz = Fp::ZERO;
            }
        }
        if vel.len_sq() < Fp::from_milli(40) {
            break;
        }
    }
    pos
}

/// Two-phase agile time-to-ball estimator (Bastiaan Konings' momentum phase -> max sprint circle).
pub fn estimate_ticks_to_point(p: &crate::state::PlayerState, target: Vec2, is_5v5: bool) -> u32 {
    // Phase 1: ~8 ticks (133ms) carrying current momentum vector while turning hips
    let momentum_end = p.pos + p.vel.scale(Fp::from_milli(95));
    let rem_dist = momentum_end.dist(target);
    if rem_dist <= Fp::from_milli(650) {
        return 4;
    }
    let pace_mm_s = 5_450 + (p.attributes.pace as i64) * 26;
    let stam_scale = 760 + (p.stamina_permille as i64) * 240 / 1_000;
    let sprint_mult = if is_5v5 { 1_205 } else { 1_220 };
    let max_speed_mm_s = (((pace_mm_s * stam_scale) / 1_000) * sprint_mult) / 1_000;
    let eff_dist_mm = (rem_dist.to_milli() - 550).max(0);
    let sprint_ticks = ((eff_dist_mm * 60) / max_speed_mm_s.max(3_500)) as u32;
    6 + sprint_ticks
}

/// Computes the earliest reachable future ball intercept `(point, ticks)` for `players[side][idx]`.
pub fn best_ball_intercept_for_player(
    state: &MatchState,
    side: TeamSide,
    idx: usize,
) -> (Vec2, u32) {
    let p = &state.players[side.index()][idx];
    let is_5v5 = state.format.players_per_side <= 6;
    if state.ball.owner.is_some() {
        let pt = predict_ball_xy_at_ticks(state, 10);
        let t = estimate_ticks_to_point(p, pt, is_5v5);
        return (pt, t);
    }
    let mut fallback = (state.ball.pos.xy(), 90_u32);
    let mut t = 6_u32;
    while t <= 78 {
        let pt = predict_ball_xy_at_ticks(state, t);
        let need = estimate_ticks_to_point(p, pt, is_5v5);
        fallback = (pt, need);
        if need <= t {
            return (pt, t);
        }
        t += 6;
    }
    fallback
}

/// Dynamic Formation Slot Assignment: maps outfielders `1..count` to tactical formation slots
/// minimizing total squared travel distance so overlapping defenders are covered by midfielders!
fn dynamically_assigned_anchor(
    state: &MatchState,
    side: TeamSide,
    idx: usize,
    shift_norm: Fp,
) -> Vec2 {
    let players = &state.players[side.index()];
    let count = players.len();
    if idx == 0 || idx >= count {
        return world_anchor_for_player(state, side, idx, shift_norm);
    }

    // Exact minimum squared-distance permutation across the 4 outfield slots in 5v5 (`4! = 24`)
    if count == 5
        && !players[1].sent_off
        && !players[2].sent_off
        && !players[3].sent_off
        && !players[4].sent_off
    {
        let anchors = [
            world_anchor_for_player(state, side, 1, shift_norm),
            world_anchor_for_player(state, side, 2, shift_norm),
            world_anchor_for_player(state, side, 3, shift_norm),
            world_anchor_for_player(state, side, 4, shift_norm),
        ];
        const PERMS: [[usize; 4]; 24] = [
            [0, 1, 2, 3], [0, 1, 3, 2], [0, 2, 1, 3], [0, 2, 3, 1], [0, 3, 1, 2], [0, 3, 2, 1],
            [1, 0, 2, 3], [1, 0, 3, 2], [1, 2, 0, 3], [1, 2, 3, 0], [1, 3, 0, 2], [1, 3, 2, 0],
            [2, 0, 1, 3], [2, 0, 3, 1], [2, 1, 0, 3], [2, 1, 3, 0], [2, 3, 0, 1], [2, 3, 1, 0],
            [3, 0, 1, 2], [3, 0, 2, 1], [3, 1, 0, 2], [3, 1, 2, 0], [3, 2, 0, 1], [3, 2, 1, 0],
        ];
        let mut best_cost = i64::MAX;
        let mut best_slot = idx - 1;
        for perm in &PERMS {
            let mut cost: i64 = 0;
            for p_off in 0..4 {
                let slot = perm[p_off];
                let d = players[p_off + 1].pos - anchors[slot];
                let dx_dm = d.x.to_milli() / 100;
                let dy_dm = d.y.to_milli() / 100;
                let mut sq = dx_dm * dx_dm + dy_dm * dy_dm;
                // Hysteresis bonus (`~2.2 m^2`) for staying in natural home role unless overlap warrants cover
                if slot != p_off {
                    sq += 220;
                }
                cost += sq;
            }
            if cost < best_cost {
                best_cost = cost;
                best_slot = perm[idx - 1];
            }
        }
        return anchors[best_slot];
    }

    world_anchor_for_player(state, side, idx, shift_norm)
}

/// Computes tactical movement for a non-controlled outfield player `players[side][idx]`.
pub fn compute_outfield_ai_intent(
    state: &MatchState,
    side: TeamSide,
    idx: usize,
) -> AiMoveIntent {
    let p = &state.players[side.index()][idx];
    let is_5v5 = state.format.players_per_side <= 6;
    let attack_sign = side.attack_sign(state.second_half_swapped());
    let half_len = state.format.half_length();
    let half_wid = state.format.half_width();
    let ball_xy = state.ball.pos.xy();

    // 1. If a pass is in flight to this player, sprint to the predicted ball intercept and face the ball!
    if let Some(ref pass) = state.ball.pass_in_flight {
        if pass.team == side && pass.intended_receiver_idx == idx {
            let (intercept_pt, _) = best_ball_intercept_for_player(state, side, idx);
            let meet = intercept_pt.lerp(pass.target_pos, Fp::from_milli(280));
            let delta_meet = meet - p.pos;
            if delta_meet.len() <= Fp::from_milli(260) {
                return AiMoveIntent {
                    move_dir: Vec2::ZERO,
                    sprint: false,
                    jockey: true, // Face incoming ball squarely for a cushioned trap!
                    trigger_dive: None,
                };
            }
            return AiMoveIntent {
                move_dir: delta_meet.normalize_or_zero(),
                sprint: delta_meet.len() > Fp::from_milli(850),
                jockey: delta_meet.len() <= Fp::from_milli(850),
                trigger_dive: None,
            };
        }
    }

    // 2. During non-OpenPlay phases, drift smoothly toward tactical home anchor (unless in FreeKick wall).
    if !matches!(state.phase, MatchPhase::OpenPlay) {
        let anchor = world_anchor_for_player(state, side, idx, Fp::ZERO);
        let delta = anchor - p.pos;
        if delta.len() < Fp::from_milli(400) {
            return AiMoveIntent {
                move_dir: Vec2::ZERO,
                sprint: false,
                jockey: false,
                trigger_dive: None,
            };
        }
        return AiMoveIntent {
            move_dir: delta.normalize_or_zero(),
            sprint: false,
            jockey: false,
            trigger_dive: None,
        };
    }

    // 2b. FIFA 12 Teammate Contain / Second-Man Press (`RB / R1` held by human on defense):
    // Assigned AI defender sprints directly onto the ball carrier's goal-side shoulder to shadow/press
    // while the human player covers passing lanes manually.
    if state.teammate_contain_idx[side.index()] == Some(idx)
        && state.ball.owner.map(|(s, _)| s) != Some(side)
    {
        let own_goal = Vec2::new(Fp::from_milli(-attack_sign * half_len.to_milli()), Fp::ZERO);
        let pred_ball = predict_ball_xy_at_ticks(state, 5);
        let is_legacy = state.control_settings[side.index()].defending_mode
            == crate::input::DefendingMode::Legacy;
        let hold_gap = Fp::from_milli(if is_legacy { 650 } else { 1_250 });
        let contain_pt =
            pred_ball + (own_goal - pred_ball).normalize_or_zero().scale(hold_gap);
        let delta = contain_pt - p.pos;
        let dist = delta.len();
        return AiMoveIntent {
            move_dir: if dist > Fp::from_milli(140) {
                delta.normalize_or_zero()
            } else {
                Vec2::ZERO
            },
            sprint: dist > Fp::from_milli(1_450),
            jockey: dist <= Fp::from_milli(1_750),
            trigger_dive: None,
        };
    }

    // 3. Loose ball pursuit using Two-Phase Time-To-Ball Intercept (not raw static distance!)
    if state.ball.owner.is_none() {
        let controlled = state.controlled_idx[side.index()];
        let (_, ctrl_ticks) = best_ball_intercept_for_player(state, side, controlled);
        let (my_intercept, my_ticks) = best_ball_intercept_for_player(state, side, idx);
        let mut fastest_outfield = true;
        for (other_idx, other) in state.players[side.index()].iter().enumerate() {
            if other_idx == 0 || other_idx == idx || other.sent_off {
                continue;
            }
            let (_, other_ticks) = best_ball_intercept_for_player(state, side, other_idx);
            if other_ticks + 2 < my_ticks {
                fastest_outfield = false;
                break;
            }
        }
        if fastest_outfield && my_ticks + 7 < ctrl_ticks {
            return AiMoveIntent {
                move_dir: (my_intercept - p.pos).normalize_or_zero(),
                sprint: true,
                jockey: false,
                trigger_dive: None,
            };
        }
    }

    // Smoothed ("laggy") team possession blend in `[0, 1000]` (`1000` = full attack, `0` = full defense)
    let side_bal_sign = if side == TeamSide::Home { 1 } else { -1 };
    let signed_bal = (state.possession_balance_permille * side_bal_sign).clamp(-1_000, 1_000);
    let attack_blend = Fp::from_milli(((signed_bal + 1_000) / 2) as i64);

    // Normalized ball progression in `[-1, +1]` along this team's attacking axis.
    let ball_attack_norm = ball_xy
        .x
        .mul(Fp::from_int(attack_sign as i32))
        .div(half_len)
        .clamp(Fp::NEG_ONE, Fp::ONE);

    // Compute Defensive Target (compact goal-side block + dynamic slot cover)
    let def_shift = ball_attack_norm.mul(Fp::from_milli(330)) - Fp::from_milli(185);
    let mut def_target = dynamically_assigned_anchor(state, side, idx, def_shift);
    def_target.y = def_target.y.lerp(ball_xy.y, Fp::from_milli(330));

    let controlled = state.controlled_idx[side.index()];
    let ctrl_dist = state.players[side.index()][controlled].pos.dist(ball_xy);
    let my_dist = p.pos.dist(ball_xy);
    if signed_bal < 150 && my_dist < Fp::from_milli(9_200) && my_dist + Fp::from_milli(1_500) < ctrl_dist {
        let own_goal = Vec2::new(Fp::from_milli(-attack_sign * half_len.to_milli()), Fp::ZERO);
        let goal_side_offset = (own_goal - ball_xy).normalize_or_zero().scale(Fp::from_milli(1_650));
        def_target = ball_xy + goal_side_offset;
    }

    // Compute Attacking Target (dynamic slot + weighted open-space pocket + off-ball channel runs)
    let att_shift = if is_5v5 {
        ball_attack_norm.mul(Fp::from_milli(450)) + Fp::from_milli(180)
    } else {
        ball_attack_norm.mul(Fp::from_milli(390)) + Fp::from_milli(145)
    };
    let mut att_target = dynamically_assigned_anchor(state, side, idx, att_shift);
    let lateral_pull = if is_5v5 {
        Fp::from_milli(205)
    } else {
        Fp::from_milli(250)
    };
    att_target.y = att_target.y.lerp(ball_xy.y, lateral_pull);

    // Weighted Space Pocket: step out of opponent shadow so passing lanes open cleanly!
    let mut best_pocket = att_target;
    let mut best_pocket_score = Fp::from_int(-999);
    let cand_offsets = [
        Vec2::ZERO,
        Vec2::from_milli(attack_sign * 2_200, 1_800),
        Vec2::from_milli(attack_sign * 2_200, -1_800),
        Vec2::from_milli(-attack_sign * 1_600, if idx.is_multiple_of(2) { 2_600 } else { -2_600 }),
    ];
    for (c_i, off) in cand_offsets.iter().enumerate() {
        let cand = att_target + *off;
        if cand.dist(ball_xy) < Fp::from_milli(4_200) {
            continue;
        }
        let mut nearest_opp = Fp::from_int(30);
        for opp in &state.players[side.opponent().index()] {
            if !opp.sent_off {
                nearest_opp = nearest_opp.min(cand.dist(opp.pos));
            }
        }
        let anchor_bias = if c_i == 0 { Fp::from_milli(1_100) } else { Fp::ZERO };
        let sc = nearest_opp + anchor_bias;
        if sc > best_pocket_score {
            best_pocket_score = sc;
            best_pocket = cand;
        }
    }
    att_target = best_pocket;

    // Forward darting channel run if triggered (`run_boost_ticks > 0`) or forward in final two-thirds
    let auto_overlap = p.role == PlayerRole::Forward && ball_attack_norm > Fp::from_milli(-220);
    if p.run_boost_ticks > 0 || auto_overlap {
        let onside_limit_x = compute_onside_limit_x(state, side);
        let run_push = Fp::from_milli(if p.run_boost_ticks > 0 {
            if is_5v5 { 8_000 } else { 9_800 }
        } else if is_5v5 {
            4_800
        } else {
            5_800
        });
        let desired_x = att_target.x + Fp::from_int(attack_sign as i32).mul(run_push);
        att_target.x = if attack_sign > 0 {
            desired_x.min(onside_limit_x)
        } else {
            desired_x.max(onside_limit_x)
        };
    }

    // Smoothly blend defensive and attacking targets by `attack_blend` (`0..=1`)
    let mut target = def_target.lerp(att_target, attack_blend);
    target.x = target.x.clamp(
        -(half_len - Fp::from_milli(1_500)),
        half_len - Fp::from_milli(1_500),
    );
    target.y = target.y.clamp(
        -(half_wid - Fp::from_milli(1_500)),
        half_wid - Fp::from_milli(1_500),
    );

    let delta = target - p.pos;
    let dist = delta.len();
    let sprint_thresh = if is_5v5 {
        Fp::from_milli(4_200)
    } else {
        Fp::from_milli(5_800)
    };
    AiMoveIntent {
        move_dir: if dist > Fp::from_milli(310) {
            delta.normalize_or_zero()
        } else {
            Vec2::ZERO
        },
        sprint: (p.run_boost_ticks > 0 && signed_bal >= 0) || dist > sprint_thresh,
        jockey: signed_bal < -150 && my_dist < Fp::from_milli(2_600),
        trigger_dive: None,
    }
}

/// Generates a deterministic `InputFrame` for a CPU-controlled team (`Easy`, `Normal`, `Hard`).
pub fn generate_cpu_input(
    state: &MatchState,
    side: TeamSide,
    difficulty: CpuDifficulty,
) -> InputFrame {
    let side_idx = side.index();
    let ctrl_idx = state.controlled_idx[side_idx];
    let me = &state.players[side_idx][ctrl_idx];
    let attack_sign = side.attack_sign(state.second_half_swapped());
    let half_len = state.format.half_length();
    let goal_half_w = state.format.goal_half_width();
    let opp_goal_x = Fp::from_milli(attack_sign * half_len.to_milli());
    let own_goal_x = -opp_goal_x;

    // Handle KickOff & SetPiece restarts after brief natural pause
    match state.phase {
        MatchPhase::KickOff {
            team,
            wait_ticks,
        } if team == side => {
            if wait_ticks >= 24 {
                return vec_and_buttons_to_input(
                    Vec2::new(Fp::from_int(-attack_sign as i32), Fp::from_milli(350)),
                    Vec2::ZERO,
                    BTN_PASS,
                );
            }
            return InputFrame::IDLE;
        }
        MatchPhase::SetPiece {
            kind,
            team,
            spot,
            taker_idx,
            wait_ticks,
        } if team == side => {
            let taker = &state.players[side_idx][taker_idx];
            if let PlayerActionState::Charging {
                button,
                ticks,
                aim: stored_aim,
                ..
            } = taker.action
            {
                let target_charge: u16 = match kind {
                    RestartKind::PenaltyKick => 13,
                    RestartKind::CornerKick => {
                        if (button & BTN_PASS) != 0 {
                            8
                        } else {
                            15 + ((state.tick % 5) as u16)
                        }
                    }
                    RestartKind::GoalKick => 16,
                    RestartKind::ThrowIn => 11,
                    RestartKind::FreeKick => 14,
                };
                let hold_aim = if stored_aim.len() > Fp::from_milli(120) {
                    stored_aim.normalize_or_zero()
                } else {
                    Vec2::from_bam(taker.facing_bam, Fp::ONE)
                };
                if ticks >= target_charge {
                    return vec_and_buttons_to_input(hold_aim, hold_aim, 0);
                }
                return vec_and_buttons_to_input(hold_aim, hold_aim, button);
            }

            if wait_ticks >= 24 {
                let (btn, aim) = match kind {
                    RestartKind::PenaltyKick => (
                        BTN_SHOOT,
                        Vec2::new(
                            Fp::from_int(attack_sign as i32),
                            if (state.tick & 1) == 0 {
                                Fp::from_milli(580)
                            } else {
                                Fp::from_milli(-580)
                            },
                        )
                        .normalize_or_zero(),
                    ),
                    RestartKind::CornerKick => {
                        let short_corner = (state.tick & 3) == 0;
                        if short_corner {
                            let y_in = if spot.y >= Fp::ZERO { Fp::NEG_ONE } else { Fp::ONE };
                            (
                                BTN_PASS,
                                Vec2::new(Fp::from_milli(-attack_sign * 780), y_in.mul(Fp::from_milli(620)))
                                    .normalize_or_zero(),
                            )
                        } else {
                            let pen_x = Fp::from_milli(
                                attack_sign * (half_len.to_milli() - if state.format.players_per_side <= 6 { 5_800 } else { 9_800 }),
                            );
                            (BTN_LOB, (Vec2::new(pen_x, Fp::ZERO) - spot).normalize_or_zero())
                        }
                    }
                    RestartKind::ThrowIn => {
                        let y_in = if spot.y >= Fp::ZERO { Fp::NEG_ONE } else { Fp::ONE };
                        let mut best_dir = Vec2::new(Fp::from_milli(attack_sign * 420), y_in).normalize_or_zero();
                        let mut best_sc = Fp::from_int(-999);
                        for (m_idx, mate) in state.players[side_idx].iter().enumerate().skip(1) {
                            if m_idx == taker_idx || mate.sent_off {
                                continue;
                            }
                            let d = spot.dist(mate.pos);
                            if d < Fp::from_milli(2_500) || d > Fp::from_milli(19_000) {
                                continue;
                            }
                            let mut opp_clr = Fp::from_milli(9_000);
                            for opp in &state.players[side.opponent().index()] {
                                if !opp.sent_off {
                                    opp_clr = opp_clr.min(opp.pos.dist(mate.pos));
                                }
                            }
                            let sc = opp_clr.mul(Fp::from_int(2))
                                + (mate.pos.x - spot.x).mul(Fp::from_milli(attack_sign * 250));
                            if sc > best_sc {
                                best_sc = sc;
                                best_dir = (mate.pos - spot).normalize_or_zero();
                            }
                        }
                        (BTN_PASS, best_dir)
                    }
                    RestartKind::GoalKick => {
                        let (gk_btn, _, gk_aim) = choose_safe_gk_distribution(state, side);
                        (gk_btn, gk_aim)
                    }
                    RestartKind::FreeKick => {
                        let goal_vec = Vec2::new(opp_goal_x, Fp::ZERO) - spot;
                        let shoot_r = if state.format.players_per_side <= 6 {
                            Fp::from_milli(16_500)
                        } else {
                            Fp::from_milli(25_000)
                        };
                        if goal_vec.len() <= shoot_r {
                            (BTN_SHOOT, goal_vec.normalize_or_zero())
                        } else {
                            (BTN_PASS, Vec2::new(Fp::from_int(attack_sign as i32), Fp::ZERO))
                        }
                    }
                };
                return vec_and_buttons_to_input(aim, aim, btn);
            }
            return InputFrame::IDLE;
        }
        MatchPhase::OpenPlay => {}
        _ => return InputFrame::IDLE,
    }

    // If GK holds ball, distribute safely to an unmarked teammate after a brief scan
    if state.ball.owner == Some((side, 0)) && state.ball.gk_hold_ticks < 68 {
        let gk = &state.players[side_idx][0];
        if let PlayerActionState::Charging {
            button,
            ticks,
            aim: stored_aim,
            ..
        } = gk.action
        {
            let (_, target_ch, fallback_aim) = choose_safe_gk_distribution(state, side);
            let hold_aim = if stored_aim.len() > Fp::from_milli(120) {
                stored_aim.normalize_or_zero()
            } else {
                fallback_aim
            };
            if ticks >= target_ch {
                return vec_and_buttons_to_input(Vec2::ZERO, hold_aim, 0);
            }
            return vec_and_buttons_to_input(Vec2::ZERO, hold_aim, button);
        }
        let (gk_btn, _, gk_aim) = choose_safe_gk_distribution(state, side);
        return vec_and_buttons_to_input(gk_aim, gk_aim, gk_btn);
    }

    // Reaction cadence by difficulty (staggered per side so Home & Away CPU never phase-lock on identical ticks!)
    let decision_cadence: u32 = match difficulty {
        CpuDifficulty::Easy => 18,
        CpuDifficulty::Normal => 10,
        CpuDifficulty::Hard => 6,
    };
    let cadence_tick = state.tick.wrapping_add((side_idx as u32) * 7);
    let half_wid = state.format.half_width();

    // Nearest opponent distance & vector (used for both quick-release charging and dribbling/passing)
    let mut nearest_opp_dist = Fp::from_int(999);
    let mut nearest_opp_vec = Vec2::ZERO;
    for opp in &state.players[side.opponent().index()] {
        if opp.sent_off {
            continue;
        }
        let d = me.pos.dist(opp.pos);
        if d < nearest_opp_dist {
            nearest_opp_dist = d;
            nearest_opp_vec = opp.pos - me.pos;
        }
    }

    // Helper: compute lateral sidestep away from a close presser while keeping pitch bounds in mind
    let evade_stride = |desired: Vec2| -> Vec2 {
        if nearest_opp_dist < Fp::from_milli(3_800) && nearest_opp_vec.len() > Fp::from_milli(80) {
            let opp_dir = nearest_opp_vec.normalize_or_zero();
            let perp_a = Vec2::new(-opp_dir.y, opp_dir.x);
            let perp_b = Vec2::new(opp_dir.y, -opp_dir.x);
            let score_side = |cand: Vec2| -> Fp {
                let next_y = me.pos.y + cand.y.mul(Fp::from_milli(2_500));
                let wall_margin = (half_wid - next_y.abs()).clamp(Fp::from_milli(-3_000), Fp::from_milli(6_000));
                cand.dot(desired).mul(Fp::from_milli(1_400)) + wall_margin.mul(Fp::from_milli(220))
            };
            let lateral = if score_side(perp_a) >= score_side(perp_b) {
                perp_a
            } else {
                perp_b
            };
            let head_on = opp_dir.dot(desired);
            if head_on > Fp::from_milli(250) {
                let lat_w = if nearest_opp_dist < Fp::from_milli(1_850) {
                    Fp::from_milli(760)
                } else {
                    Fp::from_milli(560)
                };
                (desired.scale(Fp::ONE - lat_w) + lateral.scale(lat_w) - opp_dir.scale(Fp::from_milli(220)))
                    .normalize_or_zero()
            } else {
                (desired - opp_dir.scale(Fp::from_milli(360))).normalize_or_zero()
            }
        } else {
            desired
        }
    };

    // If already charging a pass/shot, release briskly under close pressure so tacklers never catch windup!
    if let PlayerActionState::Charging {
        button,
        ticks,
        aim: stored_aim,
        ..
    } = me.action
    {
        let tight_press = nearest_opp_dist < Fp::from_milli(2_050);
        let target_charge: u16 = if (button & BTN_SHOOT) != 0 {
            if tight_press { 11 } else { 15 }
        } else if tight_press {
            4
        } else {
            8
        };
        let hold_aim = if stored_aim.len() > Fp::from_milli(120) {
            stored_aim.normalize_or_zero()
        } else {
            Vec2::new(Fp::from_int(attack_sign as i32), Fp::ZERO)
        };
        let base_glide = if me.vel.len() > Fp::from_milli(450) {
            me.vel.normalize_or_zero()
        } else {
            hold_aim
        };
        let glide_dir = evade_stride(base_glide);
        if ticks >= target_charge {
            return vec_and_buttons_to_input(glide_dir, hold_aim, 0);
        } else {
            return vec_and_buttons_to_input(glide_dir, hold_aim, button);
        }
    }

    // Does CPU have the ball on the controlled player?
    if state.ball.owner == Some((side, ctrl_idx)) {
        let opp_gk_y = state.players[side.opponent().index()][0].pos.y;
        let corner_y = if opp_gk_y >= Fp::ZERO {
            -goal_half_w.mul(Fp::from_milli(680))
        } else {
            goal_half_w.mul(Fp::from_milli(680))
        };
        let shot_target = Vec2::new(opp_goal_x, corner_y);
        let dist_to_goal = me.pos.dist(Vec2::new(opp_goal_x, Fp::ZERO));
        let max_shoot_dist = if state.format.players_per_side <= 6 {
            Fp::from_milli(15_800)
        } else {
            Fp::from_milli(22_200)
        };

        // 1. Shoot when in shooting range!
        let urgent_press = difficulty != CpuDifficulty::Easy
            && nearest_opp_dist < Fp::from_milli(1_650)
            && cadence_tick.is_multiple_of(3);
        if dist_to_goal <= max_shoot_dist
            && (cadence_tick.is_multiple_of(decision_cadence) || urgent_press)
        {
            let aim = (shot_target - me.pos).normalize_or_zero();
            return vec_and_buttons_to_input(aim, aim, BTN_SHOOT);
        }

        // 2. Pass, through-ball, or lob over presser when pressured or when a teammate is in a better spot!
        if cadence_tick.is_multiple_of(decision_cadence) || urgent_press {
            let in_own_half = me.pos.x.mul(Fp::from_int(attack_sign as i32)) < Fp::ZERO;
            let recent_passer_id = state
                .ball
                .assist_candidate
                .as_ref()
                .filter(|ac| ac.team == side && state.tick.saturating_sub(ac.completed_tick) < 110)
                .map(|ac| ac.passer_id.as_str());
            let mut best_mate: Option<(usize, Vec2, Fp, u16)> = None;
            for (m_idx, mate) in state.players[side_idx].iter().enumerate() {
                if m_idx == ctrl_idx || mate.sent_off {
                    continue;
                }
                let dist = me.pos.dist(mate.pos);
                if m_idx == 0 {
                    // Only recycle to GK when in own half under heavy pressure and at safe pass distance
                    if !in_own_half
                        || nearest_opp_dist > Fp::from_milli(1_850)
                        || dist < Fp::from_milli(5_200)
                        || dist > Fp::from_milli(22_000)
                    {
                        continue;
                    }
                } else if dist < Fp::from_milli(2_200) || dist > Fp::from_milli(29_000) {
                    continue;
                }

                let lane_vec = mate.pos - me.pos;
                let lane_dir = lane_vec.normalize_or_zero();
                let mut mate_clearance = Fp::from_int(50);
                let mut lane_min_dist = Fp::from_int(50);
                let mut point_blank_blocked = false;
                for opp in &state.players[side.opponent().index()] {
                    if opp.sent_off {
                        continue;
                    }
                    mate_clearance = mate_clearance.min(mate.pos.dist(opp.pos));
                    let rel = opp.pos - me.pos;
                    let proj = rel.dot(lane_dir);
                    if proj > Fp::from_milli(120) && proj < dist - Fp::from_milli(450) {
                        let perp_d = (rel - lane_dir.scale(proj)).len();
                        lane_min_dist = lane_min_dist.min(perp_d);
                        if proj < Fp::from_milli(1_450) && perp_d < Fp::from_milli(950) {
                            point_blank_blocked = true;
                        }
                    }
                }

                if m_idx == 0 && (lane_min_dist < Fp::from_milli(2_200) || point_blank_blocked) {
                    continue;
                }

                let forward_gain = (mate.pos.x - me.pos.x).mul(Fp::from_int(attack_sign as i32));
                let ground_blocked = point_blank_blocked || lane_min_dist < Fp::from_milli(1_350);
                let can_chip_over = ground_blocked
                    && difficulty != CpuDifficulty::Easy
                    && m_idx != 0
                    && dist > Fp::from_milli(7_200)
                    && mate_clearance > Fp::from_milli(2_500);
                let lane_penalty = if ground_blocked && !can_chip_over {
                    Fp::from_milli(9_500)
                } else if lane_min_dist < Fp::from_milli(2_100) {
                    Fp::from_milli(2_200)
                } else {
                    Fp::ZERO
                };
                let gk_back_penalty = if m_idx == 0 {
                    Fp::from_milli(4_200)
                } else {
                    Fp::ZERO
                };
                // Avoid sterile A -> B -> A return ping-pong unless B has made a genuine forward run!
                let return_ping_penalty = if recent_passer_id == Some(mate.footballer_id.as_str())
                    && forward_gain < Fp::from_milli(2_500)
                {
                    Fp::from_milli(3_800)
                } else {
                    Fp::ZERO
                };
                let score = forward_gain + mate_clearance.mul(Fp::from_milli(850))
                    - lane_penalty
                    - gk_back_penalty
                    - return_ping_penalty;

                let pass_btn = if can_chip_over {
                    BTN_LOB
                } else if m_idx != 0
                    && mate.role == PlayerRole::Forward
                    && forward_gain > Fp::from_milli(3_200)
                    && mate_clearance > Fp::from_milli(2_600)
                    && !ground_blocked
                    && difficulty != CpuDifficulty::Easy
                {
                    BTN_THROUGH
                } else {
                    BTN_PASS
                };

                if best_mate.is_none_or(|(_, _, best_s, _)| score > best_s) {
                    best_mate = Some((m_idx, mate.pos, score, pass_btn));
                }
            }

            let tight_pressure = nearest_opp_dist < Fp::from_milli(1_650);
            let moderate_pressure = nearest_opp_dist < Fp::from_milli(2_650);
            if let Some((_, mate_pos, score, btn)) = best_mate {
                let great_forward_option = score > Fp::from_milli(6_800);
                let progressive_outlet = moderate_pressure && score > Fp::from_milli(2_600);
                let tight_escape = tight_pressure && score > Fp::from_milli(-3_500);
                if great_forward_option || progressive_outlet || tight_escape {
                    let aim = (mate_pos - me.pos).normalize_or_zero();
                    return vec_and_buttons_to_input(aim, aim, btn);
                }
            }

            // Deep defensive-third safety valve: hoist clearance upfield if swarmed with all lanes shut!
            if me.pos.x.mul(Fp::from_int(attack_sign as i32)) < -half_len.mul(Fp::from_milli(520))
                && nearest_opp_dist < Fp::from_milli(1_650)
            {
                let clear_y = if me.pos.y >= Fp::ZERO {
                    Fp::from_milli(380)
                } else {
                    Fp::from_milli(-380)
                };
                let clear_aim = Vec2::new(Fp::from_int(attack_sign as i32), clear_y).normalize_or_zero();
                return vec_and_buttons_to_input(clear_aim, clear_aim, BTN_LOB);
            }
        }

        // 3. Dribble progressively with genuine 360° lateral cuts around pressers (+ 1v1 skill move on Hard/Normal)
        let goal_drive = (shot_target - me.pos).normalize_or_zero();
        let drive = evade_stride(goal_drive);
        let try_skill = difficulty != CpuDifficulty::Easy
            && nearest_opp_dist >= Fp::from_milli(1_150)
            && nearest_opp_dist <= Fp::from_milli(2_050)
            && nearest_opp_vec.normalize_or_zero().dot(goal_drive) > Fp::from_milli(420)
            && cadence_tick.is_multiple_of(19);
        let sprint = nearest_opp_dist > Fp::from_milli(2_600) && difficulty != CpuDifficulty::Easy;
        let btns = if try_skill {
            BTN_SKILL
        } else if sprint {
            BTN_SPRINT
        } else {
            0
        };
        return vec_and_buttons_to_input(drive, Vec2::ZERO, btns);
    }

    // CPU does NOT own ball on `ctrl_idx`: defend or intercept loose ball!
    let ball_xy = state.ball.pos.xy();
    let my_dist = me.pos.dist(ball_xy);
    let (my_intercept, my_ticks) = best_ball_intercept_for_player(state, side, ctrl_idx);

    // Switch to a faster-intercepting outfield defender if needed
    if state.switch_cooldown[side_idx] == 0 && cadence_tick.is_multiple_of(decision_cadence) {
        for (idx, mate) in state.players[side_idx].iter().enumerate() {
            if idx == 0 || idx == ctrl_idx || mate.sent_off {
                continue;
            }
            let (_, mate_ticks) = best_ball_intercept_for_player(state, side, idx);
            if mate_ticks + 6 < my_ticks {
                return vec_and_buttons_to_input(Vec2::ZERO, Vec2::ZERO, BTN_SWITCH);
            }
        }
    }

    // Approach goal-side when opponent carries ball, or sprint to future trajectory intercept on loose balls!
    let goal_side_pt = if state.ball.owner.is_some() {
        let to_own_goal = (Vec2::new(own_goal_x, Fp::ZERO) - my_intercept)
            .normalize_or_zero()
            .scale(Fp::from_milli(750));
        my_intercept + to_own_goal
    } else {
        my_intercept
    };

    let move_dir = (goal_side_pt - me.pos).normalize_or_zero();
    let mut buttons = 0_u16;
    if my_dist > Fp::from_milli(2_500) || state.ball.owner.is_none() {
        buttons |= BTN_SPRINT;
    } else {
        buttons |= BTN_JOCKEY;
    }

    let tackle_cadence: u32 = match difficulty {
        CpuDifficulty::Easy => 24,
        CpuDifficulty::Normal => 17,
        CpuDifficulty::Hard => 13,
    };
    let clean_tackle_angle = match state.ball.owner {
        Some((o_side, o_idx)) if o_side != side => {
            let carrier = &state.players[o_side.index()][o_idx];
            let c_fwd = Vec2::from_bam(carrier.facing_bam, Fp::ONE);
            let def_to_c = (carrier.pos - me.pos).normalize_or_zero();
            def_to_c.dot(c_fwd) < Fp::from_milli(550)
        }
        _ => false,
    };
    if clean_tackle_angle
        && my_dist <= Fp::from_milli(1_180)
        && cadence_tick.is_multiple_of(tackle_cadence)
    {
        buttons |= BTN_TACKLE;
    }

    vec_and_buttons_to_input(move_dir, Vec2::ZERO, buttons)
}

pub fn world_anchor_for_player(
    state: &MatchState,
    side: TeamSide,
    idx: usize,
    forward_shift_norm: Fp,
) -> Vec2 {
    let (ax_pm, ay_pm) = state.players[side.index()][idx].base_anchor_permille;
    let attack_sign = side.attack_sign(state.second_half_swapped());
    let half_len = state.format.half_length();
    let half_wid = state.format.half_width();

    let norm_x = (Fp::from_milli(ax_pm) + forward_shift_norm)
        .clamp(Fp::from_milli(-950), Fp::from_milli(880));
    let norm_y = Fp::from_milli(ay_pm);

    let world_x = norm_x
        .mul(half_len)
        .mul(Fp::from_int(attack_sign as i32));
    let world_y = norm_y
        .mul(half_wid)
        .mul(Fp::from_int(attack_sign as i32));
    Vec2::new(world_x, world_y)
}

fn compute_onside_limit_x(state: &MatchState, attacking_side: TeamSide) -> Fp {
    let attack_sign = attacking_side.attack_sign(state.second_half_swapped());
    let half_len = state.format.half_length();
    if !state.rules.offsides_enabled {
        return Fp::from_milli(attack_sign * (half_len.to_milli() - 1_800));
    }
    // Second-last opponent X along attacking direction
    let opp_side = attacking_side.opponent();
    let mut opp_proj: Vec<i64> = state.players[opp_side.index()]
        .iter()
        .filter(|p| !p.sent_off)
        .map(|p| p.pos.x.to_milli() * attack_sign)
        .collect();
    opp_proj.sort_unstable();
    // Highest two `proj` values are closest to opponent's goal line (GK + last defender)
    let second_last_mm = if opp_proj.len() >= 2 {
        opp_proj[opp_proj.len() - 2]
    } else {
        half_len.to_milli()
    };
    let ball_proj_mm = state.ball.pos.x.to_milli() * attack_sign;
    let line_mm = second_last_mm.max(ball_proj_mm).max(0) - 400;
    Fp::from_milli(attack_sign * line_mm)
}

fn vec_and_buttons_to_input(mv: Vec2, aim: Vec2, buttons: u16) -> InputFrame {
    let to_axis = |v: Fp| -> i8 {
        ((v.to_milli() * 127) / 1000).clamp(-127, 127) as i8
    };
    InputFrame {
        move_x: to_axis(mv.x),
        move_y: to_axis(mv.y),
        aim_x: to_axis(aim.x),
        aim_y: to_axis(aim.y),
        buttons,
        reserved: 0,
    }
}
