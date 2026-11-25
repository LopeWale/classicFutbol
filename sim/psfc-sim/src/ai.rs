//! Deterministic Goalkeeper AI, 3-Tier Outfield Tactical AI, and Physics-Based CPU Controller.

use crate::config::{CpuDifficulty, TeamSide};
use crate::input::{
    InputFrame, BTN_CONTAIN, BTN_FINESSE, BTN_JOCKEY, BTN_LOB, BTN_PASS, BTN_SHOOT, BTN_SKILL,
    BTN_SPRINT, BTN_TACKLE, BTN_THROUGH,
};
use crate::math::{Fp, Vec2, Vec3};
use crate::sim::step_free_ball_kinematics;
use crate::snapshot::PlayerRole;
use crate::state::{MatchPhase, MatchState, PlayerActionState, PlayerState, RestartKind};

#[derive(Clone, Copy, Debug)]
pub struct AiMoveIntent {
    pub move_dir: Vec2,
    pub sprint: bool,
    pub jockey: bool,
    /// Optional `(horizontal_dive_dir, vertical_takeoff_vz_mm_s)` for low skimming vs airborne top-corner dives.
    pub trigger_dive: Option<(Vec2, i64)>,
}

/// Computes exact desired charge ticks (`1..=28`) matching `execute_kick`'s `pref_dist` curve
/// so CPU passes/lobs arrive crisp at the intended teammate instead of falling short.
pub fn desired_pass_charge_ticks(dist: Fp, is_5v5: bool, is_lob: bool) -> u8 {
    let base_mm = if is_5v5 { 3_800 } else { 4_500 };
    let span_mm = if is_5v5 { 20_500 } else { 28_500 };
    let raw = (((dist.to_milli() - base_mm).max(0) * 28) / span_mm).clamp(5, 25) as u8;
    if is_lob {
        raw.clamp(11, 24)
    } else {
        raw
    }
}

/// Deterministically projects the ball's full 3D `(x, y, z)` coordinates `ticks_ahead` into the future
/// using the exact shared `step_free_ball_kinematics` physics stepper (`step_ball` parity).
pub fn predict_ball_3d_at_ticks(state: &MatchState, ticks_ahead: u32) -> Vec3 {
    if ticks_ahead == 0 {
        return state.ball.pos;
    }
    if let Some((owner_side, owner_idx)) = state.ball.owner {
        let p = &state.players[owner_side.index()][owner_idx];
        let dt_total = Fp::from_ratio(ticks_ahead.min(18) as i64, 60);
        let xy = (state.ball.pos.xy() + p.vel.scale(dt_total)).clamp_box(
            state.format.half_length(),
            state.format.half_width(),
        );
        return Vec3::new(xy.x, xy.y, state.ball.pos.z);
    }

    let mut pos = state.ball.pos;
    let mut vel = state.ball.vel;
    let mut spin_side = state.ball.spin_side;
    let mut spin_top = state.ball.spin_top;
    let is_5v5 = state.format.players_per_side <= 6;
    let half_len = state.format.half_length();
    let half_wid = state.format.half_width();

    let steps = ticks_ahead.min(96);
    for _ in 0..steps {
        step_free_ball_kinematics(&mut pos, &mut vel, &mut spin_side, &mut spin_top, is_5v5);
        let clamped_xy = pos.xy().clamp_box(half_len, half_wid);
        pos.x = clamped_xy.x;
        pos.y = clamped_xy.y;
        if pos.z <= Fp::from_milli(10) && vel.xy().len_sq() < Fp::from_milli(25) {
            break;
        }
    }
    pos
}

/// Deterministically projects the ball's `(x, y)` ground coordinates `ticks_ahead` into the future.
#[inline]
pub fn predict_ball_xy_at_ticks(state: &MatchState, ticks_ahead: u32) -> Vec2 {
    predict_ball_3d_at_ticks(state, ticks_ahead).xy()
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

    // If holding or controlling the ball, step toward an open outlet angle.
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

    // Check incoming shot threat toward own goal using exact 3D trajectory rollout!
    let vel_toward_goal = if attack_sign > 0 {
        -ball_vel_xy.x
    } else {
        ball_vel_xy.x
    };
    let dist_ball_to_goal_x = (ball_xy.x - own_goal_x).abs();
    let max_shot_react_dist = Fp::from_milli(if is_5v5 { 24_500 } else { 31_000 });

    if state.ball.owner.is_none()
        && matches!(p.action, PlayerActionState::Normal)
        && vel_toward_goal > Fp::from_milli(5_200)
        && dist_ball_to_goal_x < max_shot_react_dist
        && dist_ball_to_goal_x > Fp::from_milli(220)
    {
        let dist_to_gk_x = (ball_xy.x - p.pos.x).abs();
        let t_to_gk = dist_to_gk_x.div(vel_toward_goal);
        let pred_ticks = ((t_to_gk.to_milli() * 60 + 500) / 1_000).clamp(1, 60) as u32;
        let proj_3d = predict_ball_3d_at_ticks(state, pred_ticks);
        let projected_y = proj_3d.y;
        let projected_z = proj_3d.z.max(Fp::ZERO);
        let max_guard_y = goal_half_w + Fp::from_milli(1_050);

        if projected_y.abs() <= max_guard_y {
            let lateral_gap = projected_y - p.pos.y;
            let gk_skill = p.attributes.goalkeeping as i64;
            let dive_reach = Fp::from_milli(1_650 + gk_skill * 12);
            let close_enough_in_time =
                t_to_gk <= Fp::from_milli(460) || dist_to_gk_x <= Fp::from_milli(9_800);
            if close_enough_in_time
                && lateral_gap.abs() > Fp::from_milli(460)
                && lateral_gap.abs() <= dive_reach
            {
                let dive_dir = Vec2::new(
                    into_pitch_sign.mul(Fp::from_milli(190)),
                    Fp::from_int(lateral_gap.signum_int()),
                )
                .normalize_or_zero();
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

    // High Aerial Cross / Corner Command Zone: step out to pluck/punch high balls in the 6-yard area!
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
            claim_xy
                .y
                .clamp(-(goal_half_w + Fp::from_milli(2_000)), goal_half_w + Fp::from_milli(2_000)),
        );
        return AiMoveIntent {
            move_dir: (clamped_claim - p.pos).normalize_or_zero(),
            sprint: true,
            jockey: false,
            trigger_dive: None,
        };
    }

    // Rush out on loose ball inside box, explicit human GK rush, OR automatic 1v1 breakaway angle-narrowing!
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

    // Automatic 1v1 Breakaway Containment: step off the line along goal-to-ball ray to cut shooting angle!
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
            let goal_side = (mate.pos.x - ball_xy.x).mul(Fp::from_int(-attack_sign as i32))
                > Fp::from_milli(-400);
            if d_carrier <= Fp::from_milli(2_650) && goal_side {
                defender_covering = true;
                break;
            }
        }
        if !defender_covering {
            let to_ball = ball_xy - goal_center;
            let dist_gb = to_ball.len().max(Fp::ONE);
            let rush_r = (dist_gb.mul(Fp::from_milli(520))).clamp(
                Fp::from_milli(1_600),
                (dist_gb - Fp::from_milli(1_150)).max(Fp::from_milli(1_600)),
            );
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
        + ((dist_gb.to_milli().clamp(4_000, 34_000) - 4_000) * (max_arc_r_mm - min_arc_r_mm))
            / 30_000;
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

    let mut best_short: Option<(Vec2, Fp, Fp)> = None;
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
        if dist <= short_max
            && lane_clear >= Fp::from_milli(2_400)
            && opp_clear >= Fp::from_milli(3_100)
        {
            let short_score = opp_clear.mul(Fp::from_milli(2_100))
                + lane_clear.mul(Fp::from_milli(2_400))
                + fwd_gain.mul(Fp::from_milli(260))
                - (dist - Fp::from_milli(12_000)).abs().mul(Fp::from_milli(280));
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
        let charge = desired_pass_charge_ticks(dist, is_5v5, false) as u16;
        return (BTN_PASS, charge.clamp(7, 22), dir);
    }

    if let Some((dir, dist, _)) = best_long {
        let charge = desired_pass_charge_ticks(dist, is_5v5, true) as u16;
        return (BTN_LOB, charge.clamp(14, 24), dir);
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

/// Two-phase agile time-to-ball estimator (Bastiaan Konings' momentum phase -> max sprint circle).
pub fn estimate_ticks_to_point(p: &crate::state::PlayerState, target: Vec2, is_5v5: bool) -> u32 {
    // Phase 1: ~9 ticks (150ms) carrying current momentum vector while turning hips
    let momentum_end = p.pos + p.vel.scale(Fp::from_milli(140));
    let rem_dist = momentum_end.dist(target);
    if rem_dist <= Fp::from_milli(620) {
        return 4;
    }
    let pace_mm_s = 5_450 + (p.attributes.pace as i64) * 26;
    let stam_scale = 760 + (p.stamina_permille as i64) * 240 / 1_000;
    let sprint_mult = if is_5v5 { 1_205 } else { 1_220 };
    let max_speed_mm_s = (((pace_mm_s * stam_scale) / 1_000) * sprint_mult) / 1_000;
    let eff_dist_mm = (rem_dist.to_milli() - 520).max(0);
    let sprint_ticks = ((eff_dist_mm * 60) / max_speed_mm_s.max(3_500)) as u32;
    7 + sprint_ticks
}

/// Samples the free ball's 3D trajectory at `t = 4, 8, ..., 84` ticks (`21` waypoints) in a SINGLE
/// linear pass of `step_free_ball_kinematics` (early-stopping once settled on grass), avoiding
/// `O(T^2)` re-integration from `t = 0` across multi-player intercept evaluations.
pub fn sample_ball_3d_waypoints(state: &MatchState) -> [Vec3; 21] {
    let mut out = [state.ball.pos; 21];
    if let Some((owner_side, owner_idx)) = state.ball.owner {
        let p = &state.players[owner_side.index()][owner_idx];
        let half_len = state.format.half_length();
        let half_wid = state.format.half_width();
        for (i, slot) in out.iter_mut().enumerate() {
            let t = ((i as u32 + 1) * 4).min(18);
            let dt_total = Fp::from_ratio(t as i64, 60);
            let xy = (state.ball.pos.xy() + p.vel.scale(dt_total)).clamp_box(half_len, half_wid);
            *slot = Vec3::new(xy.x, xy.y, state.ball.pos.z);
        }
        return out;
    }

    let mut pos = state.ball.pos;
    let mut vel = state.ball.vel;
    let mut spin_side = state.ball.spin_side;
    let mut spin_top = state.ball.spin_top;
    let is_5v5 = state.format.players_per_side <= 6;
    let half_len = state.format.half_length();
    let half_wid = state.format.half_width();
    let mut settled = pos.z <= Fp::from_milli(10) && vel.xy().len_sq() < Fp::from_milli(25) && vel.z.abs() <= Fp::from_milli(50);

    for slot in &mut out {
        if !settled {
            for _ in 0..4 {
                step_free_ball_kinematics(&mut pos, &mut vel, &mut spin_side, &mut spin_top, is_5v5);
                let clamped_xy = pos.xy().clamp_box(half_len, half_wid);
                pos.x = clamped_xy.x;
                pos.y = clamped_xy.y;
                if pos.z <= Fp::from_milli(10) && vel.xy().len_sq() < Fp::from_milli(25) && vel.z.abs() <= Fp::from_milli(50) {
                    settled = true;
                    break;
                }
            }
        }
        *slot = pos;
    }
    out
}

#[inline]
pub fn best_intercept_from_waypoints(
    p: &PlayerState,
    waypoints: &[Vec3; 21],
    is_5v5: bool,
    is_gk: bool,
) -> (Vec2, u32) {
    let mut fallback = (waypoints[0].xy(), 96_u32);
    for (i, pt3) in waypoints.iter().enumerate() {
        let t = (i as u32 + 1) * 4;
        let pt = pt3.xy();
        let need = estimate_ticks_to_point(p, pt, is_5v5);
        fallback = (pt, need);
        if need <= t && (is_gk || pt3.z <= Fp::from_milli(2_250)) {
            return (pt, t);
        }
    }
    fallback
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
        let pt = predict_ball_xy_at_ticks(state, 8);
        let t = estimate_ticks_to_point(p, pt, is_5v5);
        return (pt, t);
    }
    let waypoints = sample_ball_3d_waypoints(state);
    best_intercept_from_waypoints(p, &waypoints, is_5v5, idx == 0)
}

#[inline]
fn role_band(role: PlayerRole) -> i64 {
    match role {
        PlayerRole::Goalkeeper => 0,
        PlayerRole::Defender => 1,
        PlayerRole::Midfielder => 2,
        PlayerRole::Forward => 3,
    }
}

/// Dynamic Formation Slot Assignment across BOTH 5v5 and 11v11 (and red-card lineups!):
/// maps active outfielders to tactical anchors minimizing squared travel + role-band mismatch
/// so midfielders seamlessly cover overlapping fullbacks without strikers & CBs trading places!
fn dynamically_assigned_anchor(
    state: &MatchState,
    side: TeamSide,
    idx: usize,
    shift_norm: Fp,
) -> Vec2 {
    let players = &state.players[side.index()];
    let count = players.len();
    if idx == 0 || idx >= count || players[idx].sent_off {
        return world_anchor_for_player(state, side, idx, shift_norm);
    }

    let active_indices: Vec<usize> = (1..count).filter(|&i| !players[i].sent_off).collect();
    let n = active_indices.len();
    if n <= 1 {
        return world_anchor_for_player(state, side, idx, shift_norm);
    }

    let anchors: Vec<Vec2> = active_indices
        .iter()
        .map(|&slot_i| world_anchor_for_player(state, side, slot_i, shift_norm))
        .collect();

    let slot_cost = |p_pos_idx: usize, slot_k: usize| -> i64 {
        let p_i = active_indices[p_pos_idx];
        let s_i = active_indices[slot_k];
        let d = players[p_i].pos - anchors[slot_k];
        let dx_dm = d.x.to_milli() / 100;
        let dy_dm = d.y.to_milli() / 100;
        let mut sq = dx_dm * dx_dm + dy_dm * dy_dm;
        if p_pos_idx != slot_k {
            // ~2.6m positional hysteresis + role-band penalty (prevents CB <-> ST churn while allowing MF <-> DF cover!)
            let band_diff = (role_band(players[p_i].role) - role_band(players[s_i].role)).abs();
            sq += 680 + band_diff * band_diff * 1_450;
        }
        sq
    };

    let my_pos_in_active = active_indices.iter().position(|&i| i == idx).unwrap_or(0);

    // Exact 4! = 24 permutation optimum for 4 outfielders (5v5)
    if n == 4 {
        const PERMS: [[usize; 4]; 24] = [
            [0, 1, 2, 3], [0, 1, 3, 2], [0, 2, 1, 3], [0, 2, 3, 1], [0, 3, 1, 2], [0, 3, 2, 1],
            [1, 0, 2, 3], [1, 0, 3, 2], [1, 2, 0, 3], [1, 2, 3, 0], [1, 3, 0, 2], [1, 3, 2, 0],
            [2, 0, 1, 3], [2, 0, 3, 1], [2, 1, 0, 3], [2, 1, 3, 0], [2, 3, 0, 1], [2, 3, 1, 0],
            [3, 0, 1, 2], [3, 0, 2, 1], [3, 1, 0, 2], [3, 1, 2, 0], [3, 2, 0, 1], [3, 2, 1, 0],
        ];
        let mut best_cost = i64::MAX;
        let mut best_slot = my_pos_in_active;
        for perm in &PERMS {
            let cost: i64 = (0..4).map(|p_off| slot_cost(p_off, perm[p_off])).sum();
            if cost < best_cost {
                best_cost = cost;
                best_slot = perm[my_pos_in_active];
            }
        }
        return anchors[best_slot];
    }

    // Deterministic 2-pass pairwise 2-opt assignment for 11v11 (and arbitrary active counts)
    let mut assign: Vec<usize> = (0..n).collect();
    for _pass in 0..2 {
        let mut improved = false;
        for a in 0..n {
            for b in (a + 1)..n {
                let cur = slot_cost(a, assign[a]) + slot_cost(b, assign[b]);
                let swp = slot_cost(a, assign[b]) + slot_cost(b, assign[a]);
                if swp < cur {
                    assign.swap(a, b);
                    improved = true;
                }
            }
        }
        if !improved {
            break;
        }
    }
    anchors[assign[my_pos_in_active]]
}

/// Identifies the SINGLE best non-controlled AI outfielder to apply primary ball pressure when defending,
/// preventing multi-defender ball-swarming while keeping remaining defenders in marking/lane-shadow shape.
fn designated_ai_presser_idx(state: &MatchState, side: TeamSide) -> Option<usize> {
    let s_idx = side.index();
    let is_5v5 = state.format.players_per_side <= 6;
    let controlled = state.controlled_idx[s_idx];
    let ball_xy = state.ball.pos.xy();
    let owned = state.ball.owner.is_some();
    let waypoints = if owned {
        [state.ball.pos; 21]
    } else {
        sample_ball_3d_waypoints(state)
    };
    let owned_pt = if owned {
        predict_ball_xy_at_ticks(state, 8)
    } else {
        ball_xy
    };
    let eval_ticks = |idx: usize| -> u32 {
        let p = &state.players[s_idx][idx];
        if owned {
            estimate_ticks_to_point(p, owned_pt, is_5v5)
        } else {
            best_intercept_from_waypoints(p, &waypoints, is_5v5, idx == 0).1
        }
    };

    let ctrl_ticks = if controlled > 0 && controlled < state.players[s_idx].len() {
        eval_ticks(controlled)
    } else {
        120
    };

    let mut best_idx: Option<usize> = None;
    let mut best_ticks = u32::MAX;
    for (idx, p) in state.players[s_idx].iter().enumerate().skip(1) {
        if idx == controlled || p.sent_off {
            continue;
        }
        if p.pos.dist(ball_xy) > Fp::from_milli(11_500) {
            continue;
        }
        let t = eval_ticks(idx);
        if t < best_ticks {
            best_ticks = t;
            best_idx = Some(idx);
        }
    }

    if let Some(b_idx) = best_idx {
        if best_ticks + 5 < ctrl_ticks {
            return Some(b_idx);
        }
    }
    None
}

/// Identifies one close teammate (`support_triangle_idx`) to check toward the ball carrier at an open passing angle.
fn designated_support_triangle_idx(state: &MatchState, side: TeamSide) -> Option<usize> {
    let s_idx = side.index();
    let ball_xy = state.ball.pos.xy();
    let carrier_idx = match state.ball.owner {
        Some((o_side, o_idx)) if o_side == side => Some(o_idx),
        _ => None,
    };
    let mut best: Option<(i64, usize)> = None;
    for (idx, p) in state.players[s_idx].iter().enumerate().skip(1) {
        if Some(idx) == carrier_idx || p.sent_off {
            continue;
        }
        let d_mm = p.pos.dist(ball_xy).to_milli();
        let role_bonus = if p.role == PlayerRole::Midfielder { -2_500 } else { 0 };
        let cost = (d_mm - 9_500).abs() + role_bonus;
        if best.is_none_or(|(bc, _)| cost < bc) {
            best = Some((cost, idx));
        }
    }
    best.map(|(_, i)| i)
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

    // 1. Active Pass Receiver Meeting: attack the incoming pass trajectory decisively!
    if let Some(ref pass) = state.ball.pass_in_flight {
        if pass.team == side && pass.intended_receiver_idx == idx {
            let (intercept_pt, _) = best_ball_intercept_for_player(state, side, idx);
            let delta_meet = intercept_pt - p.pos;
            let mut contested_close = false;
            for opp in &state.players[side.opponent().index()] {
                if !opp.sent_off && opp.pos.dist(intercept_pt) <= Fp::from_milli(4_200) {
                    contested_close = true;
                    break;
                }
            }
            if delta_meet.len() <= Fp::from_milli(220) && !contested_close {
                return AiMoveIntent {
                    move_dir: Vec2::ZERO,
                    sprint: false,
                    jockey: true,
                    trigger_dive: None,
                };
            }
            let step_to_ball = if delta_meet.len() <= Fp::from_milli(280) {
                (ball_xy - p.pos).normalize_or_zero()
            } else {
                delta_meet.normalize_or_zero()
            };
            return AiMoveIntent {
                move_dir: step_to_ball,
                sprint: contested_close || delta_meet.len() > Fp::from_milli(650),
                jockey: !contested_close && delta_meet.len() <= Fp::from_milli(550),
                trigger_dive: None,
            };
        }
    }

    // 2. During non-OpenPlay phases, drift smoothly toward tactical home anchor.
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

    // 2b. FIFA 12 Teammate Contain / Second-Man Press (`RB / R1` held by human on defense)
    if state.teammate_contain_idx[side.index()] == Some(idx)
        && state.ball.owner.map(|(s, _)| s) != Some(side)
    {
        let own_goal = Vec2::new(Fp::from_milli(-attack_sign * half_len.to_milli()), Fp::ZERO);
        let pred_ball = predict_ball_xy_at_ticks(state, 5);
        let is_legacy = state.control_settings[side.index()].defending_mode
            == crate::input::DefendingMode::Legacy;
        let hold_gap = Fp::from_milli(if is_legacy { 650 } else { 1_250 });
        let contain_pt = pred_ball + (own_goal - pred_ball).normalize_or_zero().scale(hold_gap);
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

    // 3. Loose ball pursuit: ONLY the single fastest outfielder sprints to intercept if faster than controlled!
    if state.ball.owner.is_none() {
        let waypoints = sample_ball_3d_waypoints(state);
        let controlled = state.controlled_idx[side.index()];
        let (_, ctrl_ticks) = best_intercept_from_waypoints(
            &state.players[side.index()][controlled],
            &waypoints,
            is_5v5,
            controlled == 0,
        );
        let (my_intercept, my_ticks) = best_intercept_from_waypoints(p, &waypoints, is_5v5, false);
        let mut fastest_outfield = true;
        for (other_idx, other) in state.players[side.index()].iter().enumerate() {
            if other_idx == 0 || other_idx == idx || other.sent_off {
                continue;
            }
            let (_, other_ticks) = best_intercept_from_waypoints(other, &waypoints, is_5v5, false);
            if other_ticks + 2 < my_ticks || (other_ticks == my_ticks && other_idx < idx) {
                fastest_outfield = false;
                break;
            }
        }
        if fastest_outfield && my_ticks + 6 < ctrl_ticks {
            return AiMoveIntent {
                move_dir: (my_intercept - p.pos).normalize_or_zero(),
                sprint: true,
                jockey: false,
                trigger_dive: None,
            };
        }
    }

    // Smoothed team possession blend in `[0, 1000]` (`1000` = full attack, `0` = full defense)
    let side_bal_sign = if side == TeamSide::Home { 1 } else { -1 };
    let signed_bal = (state.possession_balance_permille * side_bal_sign).clamp(-1_000, 1_000);
    let attack_blend = Fp::from_milli(((signed_bal + 1_000) / 2) as i64);

    let ball_attack_norm = ball_xy
        .x
        .mul(Fp::from_int(attack_sign as i32))
        .div(half_len)
        .clamp(Fp::NEG_ONE, Fp::ONE);

    // --- Tier 1 Defensive Target: Compact Zonal Block + 1 Primary Presser + Goal-Side Man-Marking / Lane Shadow ---
    let def_shift = ball_attack_norm.mul(Fp::from_milli(340)) - Fp::from_milli(175);
    let mut def_target = dynamically_assigned_anchor(state, side, idx, def_shift);
    def_target.y = def_target.y.lerp(ball_xy.y, Fp::from_milli(310));
    let own_goal = Vec2::new(Fp::from_milli(-attack_sign * half_len.to_milli()), Fp::ZERO);
    let my_dist = p.pos.dist(ball_xy);

    if signed_bal < 150 && designated_ai_presser_idx(state, side) == Some(idx) {
        // Strictly ONE AI presser steps up to contain goal-side!
        let goal_side_offset = (own_goal - ball_xy)
            .normalize_or_zero()
            .scale(Fp::from_milli(1_500));
        def_target = ball_xy + goal_side_offset;
    } else if signed_bal < 200 {
        // Remaining defenders mark nearest dangerous off-ball attacker in their zone & shadow passing lane!
        let opp_carrier = state.ball.owner.and_then(|(os, oi)| {
            if os != side {
                Some(oi)
            } else {
                None
            }
        });
        let mut mark_target: Option<Vec2> = None;
        let mut best_mark_d = Fp::from_milli(if is_5v5 { 6_800 } else { 8_500 });
        for (o_idx, opp) in state.players[side.opponent().index()].iter().enumerate().skip(1) {
            if Some(o_idx) == opp_carrier || opp.sent_off {
                continue;
            }
            // Only mark attackers in dangerous territory (our half or central progression)
            if opp.pos.x.mul(Fp::from_int(attack_sign as i32)) > half_len.mul(Fp::from_milli(220)) {
                continue;
            }
            let d_zone = def_target.dist(opp.pos);
            if d_zone < best_mark_d {
                best_mark_d = d_zone;
                let to_goal = (own_goal - opp.pos).normalize_or_zero();
                let to_ball = (ball_xy - opp.pos).normalize_or_zero();
                let shadow_dir = (to_goal.scale(Fp::from_milli(650))
                    + to_ball.scale(Fp::from_milli(350)))
                .normalize_or_zero();
                mark_target = Some(opp.pos + shadow_dir.scale(Fp::from_milli(1_350)));
            }
        }
        if let Some(mt) = mark_target {
            def_target = def_target.lerp(mt, Fp::from_milli(520));
        }
    }

    // --- Tier 1 Attacking Target: Dynamic Slot + Triangle Support + Shadow-Evading Pocket + Onside Runs ---
    let att_shift = if is_5v5 {
        ball_attack_norm.mul(Fp::from_milli(460)) + Fp::from_milli(210)
    } else {
        ball_attack_norm.mul(Fp::from_milli(440)) + Fp::from_milli(200)
    };
    let mut att_target = dynamically_assigned_anchor(state, side, idx, att_shift);
    let lateral_pull = if is_5v5 {
        Fp::from_milli(210)
    } else {
        Fp::from_milli(240)
    };
    att_target.y = att_target.y.lerp(ball_xy.y, lateral_pull);

    // Designated short-support triangle player checks to an open 8.5–12.5m angle from the ball carrier!
    if signed_bal > 100 && designated_support_triangle_idx(state, side) == Some(idx) {
        let to_me = att_target - ball_xy;
        let dist_b = to_me.len();
        if dist_b > Fp::from_milli(13_500) {
            let check_dir = if dist_b > Fp::from_milli(100) {
                to_me.normalize_or_zero()
            } else {
                Vec2::new(Fp::from_int(attack_sign as i32), Fp::from_milli(500)).normalize_or_zero()
            };
            att_target = ball_xy + check_dir.scale(Fp::from_milli(if is_5v5 { 8_800 } else { 11_800 }));
        }
    }

    // Force-Field Pocket Search: step out from behind opposing cover shadows AND maintain teammate spacing!
    let mut best_pocket = att_target;
    let mut best_pocket_score = Fp::from_int(-999);
    let cand_offsets = [
        Vec2::ZERO,
        Vec2::from_milli(attack_sign * 2_600, 2_200),
        Vec2::from_milli(attack_sign * 2_600, -2_200),
        Vec2::from_milli(-attack_sign * 1_800, if idx.is_multiple_of(2) { 3_000 } else { -3_000 }),
        Vec2::from_milli(attack_sign * 3_400, 0),
    ];
    for (c_i, off) in cand_offsets.iter().enumerate() {
        let cand = att_target + *off;
        let d_ball = cand.dist(ball_xy);
        if d_ball < Fp::from_milli(4_800) {
            continue;
        }
        let lane_dir = (cand - ball_xy).normalize_or_zero();
        let mut nearest_opp = Fp::from_int(30);
        let mut lane_shadow_pen = Fp::ZERO;
        for opp in &state.players[side.opponent().index()] {
            if !opp.sent_off {
                nearest_opp = nearest_opp.min(cand.dist(opp.pos));
                let rel = opp.pos - ball_xy;
                let proj = rel.dot(lane_dir);
                if proj > Fp::from_milli(800) && proj < d_ball - Fp::from_milli(600) {
                    let perp = (rel - lane_dir.scale(proj)).len();
                    if perp < Fp::from_milli(1_800) {
                        lane_shadow_pen = lane_shadow_pen.max(Fp::from_milli(2_800) - perp);
                    }
                }
            }
        }
        let mut mate_crowd_pen = Fp::ZERO;
        for (m_i, mate) in state.players[side.index()].iter().enumerate().skip(1) {
            if m_i != idx && !mate.sent_off {
                let dm = cand.dist(mate.pos);
                if dm < Fp::from_milli(5_500) {
                    mate_crowd_pen += Fp::from_milli(5_500) - dm;
                }
            }
        }
        let anchor_bias = if c_i == 0 { Fp::from_milli(950) } else { Fp::ZERO };
        let sc = nearest_opp + anchor_bias - lane_shadow_pen.mul(Fp::from_int(2)) - mate_crowd_pen;
        if sc > best_pocket_score {
            best_pocket_score = sc;
            best_pocket = cand;
        }
    }
    att_target = best_pocket;

    // Timed Onside Channel Runs for Forwards AND attacking/wide Midfielders so 11v11 attacks penetrate the box!
    let is_runner_role = p.role == PlayerRole::Forward
        || (p.role == PlayerRole::Midfielder
            && ball_attack_norm > Fp::from_milli(-80)
            && (p.base_anchor_permille.0 >= -50 || p.base_anchor_permille.1.abs() >= 350));
    if p.run_boost_ticks > 0 || (is_runner_role && signed_bal > 120) {
        let onside_limit_x = compute_onside_limit_x(state, side);
        let run_push = Fp::from_milli(if p.run_boost_ticks > 0 {
            if is_5v5 { 8_200 } else { 11_500 }
        } else if p.role == PlayerRole::Forward {
            if is_5v5 { 5_400 } else { 7_800 }
        } else {
            if is_5v5 { 3_600 } else { 5_600 }
        });
        let desired_x = att_target.x + Fp::from_int(attack_sign as i32).mul(run_push);
        att_target.x = if attack_sign > 0 {
            desired_x.min(onside_limit_x)
        } else {
            desired_x.max(onside_limit_x)
        };
    }

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
        Fp::from_milli(3_900)
    } else {
        Fp::from_milli(5_200)
    };
    AiMoveIntent {
        move_dir: if dist > Fp::from_milli(300) {
            delta.normalize_or_zero()
        } else {
            Vec2::ZERO
        },
        sprint: (p.run_boost_ticks > 0 && signed_bal >= 0) || dist > sprint_thresh,
        jockey: signed_bal < -150 && my_dist < Fp::from_milli(2_600),
        trigger_dive: None,
    }
}

/// Checks if an opposing outfielder stands directly in the shooting corridor near the shooter.
fn is_shot_corridor_blocked(
    state: &MatchState,
    side: TeamSide,
    shooter_pos: Vec2,
    shot_target: Vec2,
) -> bool {
    let ray = shot_target - shooter_pos;
    let dist = ray.len();
    if dist <= Fp::from_milli(500) {
        return false;
    }
    let dir = ray.normalize_or_zero();
    let check_len = dist.min(Fp::from_milli(5_200));
    for opp in state.players[side.opponent().index()].iter().skip(1) {
        if opp.sent_off {
            continue;
        }
        let rel = opp.pos - shooter_pos;
        let along = rel.dot(dir);
        if along > Fp::from_milli(180) && along < check_len {
            let perp = (rel - dir.scale(along)).len();
            let block_r = Fp::from_milli(560) + along.mul(Fp::from_milli(65));
            if perp < block_r {
                return true;
            }
        }
    }
    false
}

/// Measures free forward dribbling space (`0m..18m`) along `dir` before encountering an opponent.
fn measure_free_space_ahead(
    state: &MatchState,
    side: TeamSide,
    origin: Vec2,
    dir: Vec2,
) -> Fp {
    let mut free = Fp::from_milli(18_000);
    for opp in state.players[side.opponent().index()].iter().skip(1) {
        if opp.sent_off {
            continue;
        }
        let rel = opp.pos - origin;
        let along = rel.dot(dir);
        if along > Fp::from_milli(-350) && along < free {
            let perp = (rel - dir.scale(along.max(Fp::ZERO))).len();
            if perp < Fp::from_milli(2_600) {
                free = free.min(along.max(Fp::ZERO));
            }
        }
    }
    free
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
    let is_5v5 = state.format.players_per_side <= 6;
    let attack_sign = side.attack_sign(state.second_half_swapped());
    let half_len = state.format.half_length();
    let half_wid = state.format.half_width();
    let goal_half_w = state.format.goal_half_width();
    let opp_goal_x = Fp::from_milli(attack_sign * half_len.to_milli());
    let own_goal_x = -opp_goal_x;

    // Handle KickOff & SetPiece restarts
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
                            9
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
                                Vec2::new(
                                    Fp::from_milli(-attack_sign * 780),
                                    y_in.mul(Fp::from_milli(620)),
                                )
                                .normalize_or_zero(),
                            )
                        } else {
                            let pen_x = Fp::from_milli(
                                attack_sign
                                    * (half_len.to_milli() - if is_5v5 { 5_800 } else { 9_800 }),
                            );
                            (BTN_LOB, (Vec2::new(pen_x, Fp::ZERO) - spot).normalize_or_zero())
                        }
                    }
                    RestartKind::ThrowIn => {
                        let y_in = if spot.y >= Fp::ZERO { Fp::NEG_ONE } else { Fp::ONE };
                        let mut best_dir =
                            Vec2::new(Fp::from_milli(attack_sign * 420), y_in).normalize_or_zero();
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
                        let shoot_r = Fp::from_milli(if is_5v5 { 16_500 } else { 25_000 });
                        if goal_vec.len() <= shoot_r {
                            (BTN_SHOOT | BTN_FINESSE, goal_vec.normalize_or_zero())
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

    // GK possession distribution (both hand catches after brief scan AND immediate foot back-passes!)
    if state.ball.owner == Some((side, 0)) && state.ball.gk_hold_ticks < 88 {
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

    let decision_cadence: u32 = match difficulty {
        CpuDifficulty::Easy => 16,
        CpuDifficulty::Normal => 9,
        CpuDifficulty::Hard => 5,
    };
    let cadence_tick = state.tick.wrapping_add((side_idx as u32) * 7);

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

    let evade_stride = |desired: Vec2| -> Vec2 {
        if nearest_opp_dist < Fp::from_milli(3_800) && nearest_opp_vec.len() > Fp::from_milli(80) {
            let opp_dir = nearest_opp_vec.normalize_or_zero();
            let perp_a = Vec2::new(-opp_dir.y, opp_dir.x);
            let perp_b = Vec2::new(opp_dir.y, -opp_dir.x);
            let score_side = |cand: Vec2| -> Fp {
                let next_y = me.pos.y + cand.y.mul(Fp::from_milli(2_500));
                let wall_margin = (half_wid - next_y.abs())
                    .clamp(Fp::from_milli(-3_000), Fp::from_milli(6_000));
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

    // Active Charging: release at exact distance-matched `cpu_target_charge` (or briskly if point-blank pressured!)
    if let PlayerActionState::Charging {
        button,
        ticks,
        finesse,
        aim: stored_aim,
    } = me.action
    {
        let planned = state.cpu_target_charge[side_idx] as u16;
        let point_blank_press = nearest_opp_dist < Fp::from_milli(1_550);
        let target_charge: u16 = if (button & BTN_SHOOT) != 0 {
            let base = if planned >= 4 { planned } else { 14 };
            if point_blank_press { base.min(10) } else { base }
        } else {
            let base = if planned >= 4 { planned } else { 11 };
            if point_blank_press { base.min(7) } else { base }
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
        let hold_btns = if finesse { button | BTN_FINESSE } else { button };
        if ticks >= target_charge {
            return vec_and_buttons_to_input(glide_dir, hold_aim, 0);
        } else {
            return vec_and_buttons_to_input(glide_dir, hold_aim, hold_btns);
        }
    }

    // --- ON-BALL CPU DECISION BRAIN (`xG` Finishing + Crossing + Progressive Carry + `TTI` Passing) ---
    if state.ball.owner == Some((side, ctrl_idx)) {
        let opp_gk = &state.players[side.opponent().index()][0];
        let opp_gk_y = opp_gk.pos.y;
        let corner_y = if opp_gk_y >= Fp::ZERO {
            -goal_half_w.mul(Fp::from_milli(680))
        } else {
            goal_half_w.mul(Fp::from_milli(680))
        };
        let shot_target = Vec2::new(opp_goal_x, corner_y);
        let goal_center = Vec2::new(opp_goal_x, Fp::ZERO);
        let dist_to_goal = me.pos.dist(goal_center).max(Fp::ONE);
        let dx_goal = (opp_goal_x - me.pos.x).abs();
        let cos_goal_angle = dx_goal.div(dist_to_goal);
        let max_shoot_dist = match difficulty {
            CpuDifficulty::Easy => Fp::from_milli(if is_5v5 { 12_500 } else { 18_500 }),
            CpuDifficulty::Normal => Fp::from_milli(if is_5v5 { 14_800 } else { 21_500 }),
            CpuDifficulty::Hard => Fp::from_milli(if is_5v5 { 17_200 } else { 25_800 }),
        };

        let urgent_press = difficulty != CpuDifficulty::Easy
            && nearest_opp_dist < Fp::from_milli(1_850)
            && cadence_tick.is_multiple_of(3);
        let on_decision_beat = cadence_tick.is_multiple_of(decision_cadence) || urgent_press;

        // 1. xG & Corridor-Gated Finishing (never shoot from dead touchline angles or straight into a blocker!)
        let good_shot_angle = cos_goal_angle >= Fp::from_milli(360) || dist_to_goal <= Fp::from_milli(6_500);
        let shot_clear = !is_shot_corridor_blocked(state, side, me.pos, shot_target);
        if on_decision_beat && dist_to_goal <= max_shoot_dist && good_shot_angle && shot_clear {
            let aim = (shot_target - me.pos).normalize_or_zero();
            let gk_off_line = (opp_gk.pos.x - opp_goal_x).abs();
            // 1v1 Chip Shot if GK rushed far off line; Finesse Curler on angled strikes; Driven Laces otherwise!
            let shot_btns = if difficulty == CpuDifficulty::Hard
                && gk_off_line > Fp::from_milli(if is_5v5 { 4_200 } else { 5_800 })
                && dist_to_goal > Fp::from_milli(7_500)
                && nearest_opp_dist > Fp::from_milli(1_800)
            {
                BTN_SHOOT | BTN_LOB
            } else if difficulty != CpuDifficulty::Easy
                && me.pos.y.abs() > Fp::from_milli(2_200)
                && dist_to_goal >= Fp::from_milli(8_500)
            {
                BTN_SHOOT | BTN_FINESSE
            } else {
                BTN_SHOOT
            };
            return vec_and_buttons_to_input(aim, aim, shot_btns);
        }

        // 2. Wing Crossing Zone: whip aerial cross (`BTN_LOB`) or driven cutback when wide in final third!
        let prog_x = me.pos.x.mul(Fp::from_int(attack_sign as i32));
        let in_wing_cross_zone = prog_x > half_len.mul(Fp::from_milli(440))
            && me.pos.y.abs() > state.format.penalty_box_half_width().mul(Fp::from_milli(700));
        if on_decision_beat && in_wing_cross_zone && difficulty != CpuDifficulty::Easy {
            let box_spot = Vec2::new(
                Fp::from_milli(attack_sign * (half_len.to_milli() - if is_5v5 { 5_600 } else { 9_600 })),
                Fp::ZERO,
            );
            let mut has_box_runner = false;
            for (m_idx, mate) in state.players[side_idx].iter().enumerate().skip(1) {
                if m_idx != ctrl_idx
                    && !mate.sent_off
                    && (mate.pos.x - opp_goal_x).abs() <= Fp::from_milli(if is_5v5 { 12_500 } else { 20_000 })
                    && mate.pos.y.abs() <= state.format.penalty_box_half_width()
                {
                    has_box_runner = true;
                    break;
                }
            }
            if has_box_runner {
                let cross_aim = (box_spot - me.pos).normalize_or_zero();
                return vec_and_buttons_to_input(cross_aim, cross_aim, BTN_LOB);
            }
        }

        // Measure open grass ahead along progressive drive corridor!
        let goal_drive = (goal_center - me.pos).normalize_or_zero();
        let drive_corridor = evade_stride(goal_drive);
        let free_space_ahead = measure_free_space_ahead(state, side, me.pos, drive_corridor);

        // 3. Time-To-Intercept (`TTI`) & Offside-Aware Pass / Through-Ball / Switch Evaluation!
        if on_decision_beat {
            let in_own_half = prog_x < Fp::ZERO;
            let onside_limit_proj = compute_onside_limit_x(state, side).to_milli() * attack_sign;
            let recent_passer_id = state
                .ball
                .assist_candidate
                .as_ref()
                .filter(|ac| ac.team == side && state.tick.saturating_sub(ac.completed_tick) < 95)
                .map(|ac| ac.passer_id.as_str());

            let tti_safety_ticks: i32 = match difficulty {
                CpuDifficulty::Easy => -2,
                CpuDifficulty::Normal => 4,
                CpuDifficulty::Hard => 5,
            };

            let mut best_pass: Option<(Vec2, Fp, u16)> = None; // (aim_dir, score, buttons)

            for (m_idx, mate) in state.players[side_idx].iter().enumerate() {
                if m_idx == ctrl_idx || mate.sent_off {
                    continue;
                }
                let dist_feet = me.pos.dist(mate.pos);
                if m_idx == 0 {
                    // Recycle to GK (played on feet!) only under heavy press in own half with clean lane
                    if !in_own_half
                        || nearest_opp_dist > Fp::from_milli(1_850)
                        || dist_feet < Fp::from_milli(5_500)
                        || dist_feet > Fp::from_milli(22_000)
                    {
                        continue;
                    }
                } else if dist_feet < Fp::from_milli(2_600)
                    || dist_feet > Fp::from_milli(if is_5v5 { 27_000 } else { 36_000 })
                {
                    continue;
                }

                // Never pass to an offside teammate!
                let mate_proj = mate.pos.x.to_milli() * attack_sign;
                let my_proj = me.pos.x.to_milli() * attack_sign;
                if state.rules.offsides_enabled
                    && m_idx != 0
                    && mate_proj > 0
                    && mate_proj > my_proj + 100
                    && mate_proj > onside_limit_proj + 100
                {
                    continue;
                }

                // Candidate A: Crisp pass to teammate's stride (`feet_target`)
                // Candidate B: Space-splitting Through-Ball ahead of runner (`through_target`)
                let feet_target = mate.pos + mate.vel.scale(Fp::from_milli(200));
                let lead_step = Fp::from_milli(if is_5v5 { 3_400 } else { 5_200 });
                let run_dir = if mate.vel.len() > Fp::from_milli(1_100) {
                    mate.vel.normalize_or_zero()
                } else {
                    Vec2::new(Fp::from_int(attack_sign as i32), Fp::ZERO)
                };
                let raw_through = mate.pos + run_dir.scale(lead_step);
                let through_in_bounds = raw_through.x.abs() < half_len - Fp::from_milli(2_600)
                    && raw_through.y.abs() < half_wid - Fp::from_milli(1_600);
                let through_proj = raw_through.x.to_milli() * attack_sign;
                let through_onside = !state.rules.offsides_enabled
                    || mate_proj <= 0
                    || mate_proj <= my_proj + 100
                    || mate_proj <= onside_limit_proj + 100;
                let _ = through_proj;
                let can_try_through = m_idx != 0
                    && difficulty != CpuDifficulty::Easy
                    && through_in_bounds
                    && through_onside
                    && run_dir.x.mul(Fp::from_int(attack_sign as i32)) > Fp::from_milli(350);

                for (is_through_cand, cand_pt) in [
                    (false, feet_target),
                    (true, raw_through),
                ] {
                    if is_through_cand && !can_try_through {
                        continue;
                    }
                    let lane_vec = cand_pt - me.pos;
                    let d_cand = lane_vec.len();
                    if d_cand < Fp::from_milli(2_500) {
                        continue;
                    }
                    let lane_dir = lane_vec.normalize_or_zero();
                    let will_drive = !is_through_cand
                        && difficulty == CpuDifficulty::Hard
                        && d_cand >= Fp::from_milli(10_500)
                        && m_idx != 0;

                    // Physics Time-To-Intercept (`TTI`) check along the entire pass segment!
                    let avg_ball_spd_mm_s: i64 = if will_drive { 18_400 } else { 15_200 };
                    let mate_arrive_ticks = estimate_ticks_to_point(mate, cand_pt, is_5v5) as i32;
                    let total_ball_ticks = ((d_cand.to_milli() * 60) / avg_ball_spd_mm_s).max(4) as i32;
                    let recv_ready_ticks = mate_arrive_ticks.max(total_ball_ticks);

                    let mut worst_margin_ticks: i32 = 99;
                    let mut receiver_space = Fp::from_milli(15_000);
                    let mut point_blank_blocked = false;

                    for (o_idx, opp) in state.players[side.opponent().index()].iter().enumerate() {
                        if opp.sent_off {
                            continue;
                        }
                        receiver_space = receiver_space.min(opp.pos.dist(cand_pt));
                        let opp_to_end = estimate_ticks_to_point(opp, cand_pt, is_5v5) as i32;
                        worst_margin_ticks = worst_margin_ticks.min(opp_to_end - recv_ready_ticks);

                        if o_idx != 0 {
                            let rel = opp.pos - me.pos;
                            let proj = rel.dot(lane_dir);
                            if proj > Fp::from_milli(150) && proj < d_cand - Fp::from_milli(400) {
                                let perp_d = (rel - lane_dir.scale(proj)).len();
                                if proj < Fp::from_milli(1_800) && perp_d < Fp::from_milli(1_050) {
                                    point_blank_blocked = true;
                                }
                                let closest_pt = me.pos + lane_dir.scale(proj);
                                let ball_t_at_proj =
                                    ((proj.to_milli() * 60) / (avg_ball_spd_mm_s + 2_400)).max(2) as i32;
                                let opp_t_to_lane =
                                    estimate_ticks_to_point(opp, closest_pt, is_5v5) as i32;
                                worst_margin_ticks =
                                    worst_margin_ticks.min(opp_t_to_lane - ball_t_at_proj);
                            }
                        }
                    }

                    let req_safety = if is_through_cand {
                        tti_safety_ticks + 3
                    } else {
                        tti_safety_ticks
                    };
                    let ground_safe = !point_blank_blocked && worst_margin_ticks >= req_safety;
                    // Reserve aerial `BTN_LOB` strictly for wide cross-field switches to an unguarded winger!
                    let can_loft_switch = !ground_safe
                        && !is_through_cand
                        && m_idx != 0
                        && difficulty != CpuDifficulty::Easy
                        && d_cand >= Fp::from_milli(if is_5v5 { 19_000 } else { 26_000 })
                        && (cand_pt.y - me.pos.y).abs() >= Fp::from_milli(10_500)
                        && receiver_space >= Fp::from_milli(6_800);

                    if !ground_safe && !can_loft_switch {
                        continue;
                    }

                    let forward_gain = (cand_pt.x - me.pos.x).mul(Fp::from_int(attack_sign as i32));
                    let mate_goal_dist = cand_pt.dist(goal_center);
                    let x_threat_gain = (dist_to_goal - mate_goal_dist).mul(Fp::from_milli(680));
                    let space_bonus = receiver_space.min(Fp::from_milli(7_500)).mul(Fp::from_milli(620));
                    let tti_weight: i64 = match difficulty {
                        CpuDifficulty::Easy => 60,
                        CpuDifficulty::Normal => 150,
                        CpuDifficulty::Hard => 230,
                    };
                    let tti_bonus = Fp::from_milli((worst_margin_ticks.clamp(0, 24) as i64) * tti_weight);
                    let return_pen = if recent_passer_id == Some(mate.footballer_id.as_str())
                        && forward_gain < Fp::from_milli(3_000)
                    {
                        Fp::from_milli(4_200)
                    } else {
                        Fp::ZERO
                    };
                    let gk_pen = if m_idx == 0 {
                        Fp::from_milli(5_500)
                    } else {
                        Fp::ZERO
                    };
                    let through_bonus = if is_through_cand && forward_gain > Fp::from_milli(3_500) {
                        Fp::from_milli(1_900)
                    } else {
                        Fp::ZERO
                    };

                    let score = forward_gain.mul(Fp::from_milli(880))
                        + x_threat_gain
                        + space_bonus
                        + tti_bonus
                        + through_bonus
                        - return_pen
                        - gk_pen;

                    let btn = if can_loft_switch {
                        BTN_LOB
                    } else if is_through_cand {
                        BTN_THROUGH
                    } else if will_drive {
                        BTN_PASS | BTN_FINESSE
                    } else {
                        BTN_PASS
                    };

                    if best_pass.is_none_or(|(_, bs, _)| score > bs) {
                        best_pass = Some((lane_dir, score, btn));
                    }
                }
            }

            let tight_pressure = nearest_opp_dist < Fp::from_milli(1_950);
            let moderate_pressure = nearest_opp_dist < Fp::from_milli(3_300);
            let open_carry_lane =
                free_space_ahead >= Fp::from_milli(7_400) && nearest_opp_dist >= Fp::from_milli(3_400);

            if let Some((aim_dir, score, btn)) = best_pass {
                // Combine crisply through midfield before the press closes (`4.6` threshold), while still
                // surging forward on the dribble when `7.4m+` of wide-open grass opens ahead (`8.2`)!
                let killer_ball = score > Fp::from_milli(if open_carry_lane { 8_200 } else { 4_600 });
                let pressured_progression = moderate_pressure && score > Fp::from_milli(2_200);
                let safe_recycle = tight_pressure && score > Fp::from_milli(-1_800);
                if killer_ball || pressured_progression || safe_recycle {
                    return vec_and_buttons_to_input(aim_dir, aim_dir, btn);
                }
            }

            // Deep defensive-third safety clearance ONLY when heavily swarmed with zero safe pass outlets!
            if prog_x < -half_len.mul(Fp::from_milli(550))
                && nearest_opp_dist < Fp::from_milli(1_550)
            {
                let clear_y = if me.pos.y >= Fp::ZERO {
                    Fp::from_milli(380)
                } else {
                    Fp::from_milli(-380)
                };
                let clear_aim =
                    Vec2::new(Fp::from_int(attack_sign as i32), clear_y).normalize_or_zero();
                return vec_and_buttons_to_input(clear_aim, clear_aim, BTN_LOB);
            }
        }

        // 4. Progressive 360° Ball Carrying: scale sprint decisiveness & 1v1 feints cleanly up the difficulty ladder!
        let try_skill = difficulty == CpuDifficulty::Hard
            && nearest_opp_dist >= Fp::from_milli(1_250)
            && nearest_opp_dist <= Fp::from_milli(1_950)
            && nearest_opp_vec.normalize_or_zero().dot(goal_drive) > Fp::from_milli(520)
            && cadence_tick.is_multiple_of(19);
        let sprint_carry = match difficulty {
            CpuDifficulty::Hard => {
                free_space_ahead >= Fp::from_milli(3_600) && nearest_opp_dist > Fp::from_milli(2_000)
            }
            CpuDifficulty::Normal => {
                free_space_ahead >= Fp::from_milli(5_600) && nearest_opp_dist > Fp::from_milli(2_800)
            }
            CpuDifficulty::Easy => {
                free_space_ahead >= Fp::from_milli(8_500) && nearest_opp_dist > Fp::from_milli(3_600)
            }
        };
        let btns = if try_skill {
            BTN_SKILL
        } else if sprint_carry {
            BTN_SPRINT
        } else {
            0
        };
        return vec_and_buttons_to_input(drive_corridor, Vec2::ZERO, btns);
    }

    // --- OFF-BALL CPU DEFENDING (`auto_switch_player` handles switching without 75-tick manual locks!) ---
    let ball_xy = state.ball.pos.xy();
    let my_dist = me.pos.dist(ball_xy);
    let (my_intercept, _) = best_ball_intercept_for_player(state, side, ctrl_idx);

    let goal_side_pt = if state.ball.owner.is_some() {
        let hold_dist = match difficulty {
            CpuDifficulty::Easy => Fp::from_milli(1_300),
            CpuDifficulty::Normal => Fp::from_milli(920),
            CpuDifficulty::Hard => Fp::from_milli(620),
        };
        let to_own_goal = (Vec2::new(own_goal_x, Fp::ZERO) - my_intercept)
            .normalize_or_zero()
            .scale(hold_dist);
        my_intercept + to_own_goal
    } else {
        my_intercept
    };

    let move_dir = (goal_side_pt - me.pos).normalize_or_zero();
    let mut buttons = 0_u16;
    let sprint_close_dist = match difficulty {
        CpuDifficulty::Easy => Fp::from_milli(3_200),
        CpuDifficulty::Normal => Fp::from_milli(2_550),
        CpuDifficulty::Hard => Fp::from_milli(1_850),
    };
    if my_dist > sprint_close_dist || state.ball.owner.is_none() {
        buttons |= BTN_SPRINT;
    } else {
        buttons |= BTN_JOCKEY | BTN_CONTAIN;
    }

    let (tackle_cadence, tackle_reach_mm): (u32, i64) = match difficulty {
        CpuDifficulty::Easy => (28, 1_040),
        CpuDifficulty::Normal => (18, 1_120),
        CpuDifficulty::Hard => (8, 1_220),
    };
    if let Some((o_side, o_idx)) = state.ball.owner {
        if o_side != side {
            let carrier = &state.players[o_side.index()][o_idx];
            let c_fwd = Vec2::from_bam(carrier.facing_bam, Fp::ONE);
            let def_to_c = (carrier.pos - me.pos).normalize_or_zero();
            let front_or_side = def_to_c.dot(c_fwd) < Fp::from_milli(520);
            if front_or_side
                && my_dist <= Fp::from_milli(tackle_reach_mm)
                && cadence_tick.is_multiple_of(tackle_cadence)
            {
                buttons |= BTN_TACKLE;
            }
        }
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

pub(crate) fn compute_onside_limit_x(state: &MatchState, attacking_side: TeamSide) -> Fp {
    let attack_sign = attacking_side.attack_sign(state.second_half_swapped());
    let half_len = state.format.half_length();
    if !state.rules.offsides_enabled {
        return Fp::from_milli(attack_sign * (half_len.to_milli() - 1_800));
    }
    let opp_side = attacking_side.opponent();
    let mut opp_proj: Vec<i64> = state.players[opp_side.index()]
        .iter()
        .filter(|p| !p.sent_off)
        .map(|p| p.pos.x.to_milli() * attack_sign)
        .collect();
    opp_proj.sort_unstable();
    let second_last_mm = if opp_proj.len() >= 2 {
        opp_proj[opp_proj.len() - 2]
    } else {
        half_len.to_milli()
    };
    let ball_proj_mm = state.ball.pos.x.to_milli() * attack_sign;
    let line_mm = second_last_mm.max(ball_proj_mm).max(0) - 450;
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
