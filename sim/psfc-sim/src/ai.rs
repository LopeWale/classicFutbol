//! Deterministic Goalkeeper AI, Outfield Teammate AI, and CPU Opponent Controller.

use crate::config::{CpuDifficulty, TeamSide};
use crate::input::{
    InputFrame, BTN_JOCKEY, BTN_LOB, BTN_PASS, BTN_SHOOT, BTN_SPRINT, BTN_SWITCH, BTN_TACKLE,
    BTN_THROUGH,
};
use crate::math::{Fp, Vec2};
use crate::snapshot::PlayerRole;
use crate::state::{MatchPhase, MatchState, PlayerActionState, RestartKind};

#[derive(Clone, Copy, Debug)]
pub struct AiMoveIntent {
    pub move_dir: Vec2,
    pub sprint: bool,
    pub jockey: bool,
    pub trigger_dive: Option<Vec2>,
}

/// Computes goalkeeper movement & shot-stopping dive intent for `players[side][0]`.
pub fn compute_goalkeeper_intent(
    state: &MatchState,
    side: TeamSide,
    human_gk_rush: bool,
) -> AiMoveIntent {
    let p = &state.players[side.index()][0];
    let attack_sign = side.attack_sign(state.second_half_swapped());
    let half_len = state.format.half_length();
    let goal_half_w = state.format.goal_half_width();
    let box_depth = state.format.penalty_box_depth();
    let box_half_w = state.format.penalty_box_half_width();

    // Own goal center X: if attacking +X (sign = +1), own goal is at -half_len.
    let own_goal_x = Fp::from_milli(-attack_sign * half_len.to_milli());
    let into_pitch_sign = Fp::from_int(attack_sign as i32);

    // If holding the ball, step slightly toward top of 6-yard area for distribution.
    if state.ball.owner == Some((side, 0)) {
        let target = Vec2::new(
            own_goal_x + into_pitch_sign.mul(Fp::from_milli(3_200)),
            Fp::ZERO,
        );
        let delta = target - p.pos;
        if delta.len() > Fp::from_milli(350) {
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

    if state.ball.owner.is_none()
        && matches!(p.action, PlayerActionState::Normal)
        && vel_toward_goal > Fp::from_milli(8_500)
        && dist_ball_to_goal_x < Fp::from_milli(18_000)
        && dist_ball_to_goal_x > Fp::from_milli(300)
    {
        // Project Y at GK's X plane (or goal line if ball already close)
        let dist_to_gk_x = (ball_xy.x - p.pos.x).abs();
        let t_to_gk = dist_to_gk_x.div(vel_toward_goal);
        let projected_y = ball_xy.y + ball_vel_xy.y.mul(t_to_gk);
        let max_guard_y = goal_half_w + Fp::from_milli(900);

        if projected_y.abs() <= max_guard_y {
            let lateral_gap = projected_y - p.pos.y;
            let gk_skill = p.attributes.goalkeeping as i64;
            let dive_reach = Fp::from_milli(1_450 + gk_skill * 11); // ~2.3m - 2.5m reach
            if lateral_gap.abs() > Fp::from_milli(550) && lateral_gap.abs() <= dive_reach {
                let dive_dir = Vec2::new(
                    into_pitch_sign.mul(Fp::from_milli(180)),
                    Fp::from_int(lateral_gap.signum_int()),
                )
                .normalize_or_zero();
                return AiMoveIntent {
                    move_dir: dive_dir,
                    sprint: true,
                    jockey: false,
                    trigger_dive: Some(dive_dir),
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

    // Rush out if loose ball is inside the box or human triggers GK rush (`BTN_CALL_RUN`)
    let ball_in_own_box = dist_ball_to_goal_x <= box_depth && ball_xy.y.abs() <= box_half_w;
    let close_loose_ball = state.ball.owner.is_none()
        && ball_in_own_box
        && p.pos.dist(ball_xy) < Fp::from_milli(6_500);

    if (close_loose_ball || (human_gk_rush && dist_ball_to_goal_x < box_depth + Fp::from_milli(4_000)))
        && matches!(state.phase, MatchPhase::OpenPlay)
    {
        return AiMoveIntent {
            move_dir: (ball_xy - p.pos).normalize_or_zero(),
            sprint: true,
            jockey: false,
            trigger_dive: None,
        };
    }

    // Standard arc positioning bisecting ball-to-goal-center angle
    let stand_off_mm = if dist_ball_to_goal_x < Fp::from_milli(12_000) {
        1_500
    } else if dist_ball_to_goal_x < Fp::from_milli(24_000) {
        2_400
    } else {
        3_200
    };
    let target_x = own_goal_x + into_pitch_sign.mul(Fp::from_milli(stand_off_mm));
    let max_y = (goal_half_w - Fp::from_milli(350)).max(Fp::from_milli(800));
    let target_y = ball_xy.y.mul(Fp::from_milli(460)).clamp(-max_y, max_y);
    let target = Vec2::new(target_x, target_y);

    let delta = target - p.pos;
    let dist = delta.len();
    if dist < Fp::from_milli(180) {
        AiMoveIntent {
            move_dir: Vec2::ZERO,
            sprint: false,
            jockey: true,
            trigger_dive: None,
        }
    } else {
        AiMoveIntent {
            move_dir: delta.normalize_or_zero(),
            sprint: dist > Fp::from_milli(1_800),
            jockey: dist < Fp::from_milli(1_200),
            trigger_dive: None,
        }
    }
}

/// Computes tactical movement for a non-controlled outfield player `players[side][idx]`.
pub fn compute_outfield_ai_intent(
    state: &MatchState,
    side: TeamSide,
    idx: usize,
) -> AiMoveIntent {
    let p = &state.players[side.index()][idx];
    let attack_sign = side.attack_sign(state.second_half_swapped());
    let half_len = state.format.half_length();
    let half_wid = state.format.half_width();
    let ball_xy = state.ball.pos.xy();

    // 1. If a pass is in flight to this player, sprint to meet the target/ball!
    if let Some(ref pass) = state.ball.pass_in_flight {
        if pass.team == side && pass.intended_receiver_idx == idx {
            let meet = pass.target_pos.lerp(ball_xy, Fp::from_milli(350));
            return AiMoveIntent {
                move_dir: (meet - p.pos).normalize_or_zero(),
                sprint: true,
                jockey: false,
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

    // 3. Loose ball pursuit if this AI teammate is closest on their side (and human isn't right on it).
    if state.ball.owner.is_none() {
        let controlled = state.controlled_idx[side.index()];
        let ctrl_dist = state.players[side.index()][controlled].pos.dist(ball_xy);
        let my_dist = p.pos.dist(ball_xy);
        let mut closest_outfield = true;
        for (other_idx, other) in state.players[side.index()].iter().enumerate() {
            if other_idx == 0 || other_idx == idx || other.sent_off {
                continue;
            }
            if other.pos.dist(ball_xy) + Fp::from_milli(200) < my_dist {
                closest_outfield = false;
                break;
            }
        }
        if closest_outfield && my_dist + Fp::from_milli(1_200) < ctrl_dist {
            return AiMoveIntent {
                move_dir: (ball_xy - p.pos).normalize_or_zero(),
                sprint: true,
                jockey: false,
                trigger_dive: None,
            };
        }
    }

    let own_team_has_ball = match state.ball.owner {
        Some((owner_side, _)) => owner_side == side,
        None => state.ball.last_touch_team == Some(side),
    };

    // Normalized ball progression in `[-1, +1]` along this team's attacking axis.
    let ball_attack_norm = ball_xy
        .x
        .mul(Fp::from_int(attack_sign as i32))
        .div(half_len)
        .clamp(Fp::NEG_ONE, Fp::ONE);

    if own_team_has_ball {
        // Attacking shape: shift anchors forward with ball progression
        let shift = ball_attack_norm.mul(Fp::from_milli(380)) + Fp::from_milli(140);
        let mut target = world_anchor_for_player(state, side, idx, shift);

        // Lateral ball attraction for passing triangles
        target.y = target.y.lerp(ball_xy.y, Fp::from_milli(260));

        // Forward darting run if triggered (`run_boost_ticks > 0`) or striker in final half
        if p.run_boost_ticks > 0 || (p.role == PlayerRole::Forward && ball_attack_norm > Fp::from_milli(-150)) {
            let onside_limit_x = compute_onside_limit_x(state, side);
            let run_push = Fp::from_milli(if p.run_boost_ticks > 0 { 9_500 } else { 5_500 });
            let desired_x = target.x + Fp::from_int(attack_sign as i32).mul(run_push);
            target.x = if attack_sign > 0 {
                desired_x.min(onside_limit_x)
            } else {
                desired_x.max(onside_limit_x)
            };
        }

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
        AiMoveIntent {
            move_dir: if dist > Fp::from_milli(350) {
                delta.normalize_or_zero()
            } else {
                Vec2::ZERO
            },
            sprint: p.run_boost_ticks > 0 || dist > Fp::from_milli(6_000),
            jockey: false,
            trigger_dive: None,
        }
    } else {
        // Defending shape: drop deeper toward own goal and compact laterally
        let shift = ball_attack_norm.mul(Fp::from_milli(340)) - Fp::from_milli(180);
        let mut target = world_anchor_for_player(state, side, idx, shift);
        target.y = target.y.lerp(ball_xy.y, Fp::from_milli(340));

        // If human controlled defender is far from ball carrier, nearest AI midfielder/defender contains goal-side
        let controlled = state.controlled_idx[side.index()];
        let ctrl_dist = state.players[side.index()][controlled].pos.dist(ball_xy);
        let my_dist = p.pos.dist(ball_xy);
        if my_dist < Fp::from_milli(9_000) && my_dist + Fp::from_milli(1_500) < ctrl_dist {
            let own_goal = Vec2::new(Fp::from_milli(-attack_sign * half_len.to_milli()), Fp::ZERO);
            let goal_side_offset = (own_goal - ball_xy).normalize_or_zero().scale(Fp::from_milli(1_600));
            target = ball_xy + goal_side_offset;
        }

        let delta = target - p.pos;
        let dist = delta.len();
        AiMoveIntent {
            move_dir: if dist > Fp::from_milli(300) {
                delta.normalize_or_zero()
            } else {
                Vec2::ZERO
            },
            sprint: dist > Fp::from_milli(4_500),
            jockey: my_dist < Fp::from_milli(2_600),
            trigger_dive: None,
        }
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
            wait_ticks,
            ..
        } if team == side => {
            if wait_ticks >= 30 {
                let aim = match kind {
                    RestartKind::PenaltyKick => Vec2::new(
                        Fp::from_int(attack_sign as i32),
                        if (state.tick & 1) == 0 {
                            Fp::from_milli(550)
                        } else {
                            Fp::from_milli(-550)
                        },
                    ),
                    RestartKind::CornerKick => Vec2::new(
                        Fp::from_int(-attack_sign as i32).mul(Fp::from_milli(250)),
                        -me.pos.y.normalize_scalar(),
                    ),
                    _ => Vec2::new(Fp::from_int(attack_sign as i32), Fp::ZERO),
                };
                let btn = match kind {
                    RestartKind::PenaltyKick => BTN_SHOOT,
                    RestartKind::CornerKick => BTN_LOB,
                    _ => BTN_PASS,
                };
                return vec_and_buttons_to_input(aim, Vec2::ZERO, btn);
            }
            return InputFrame::IDLE;
        }
        MatchPhase::OpenPlay => {}
        _ => return InputFrame::IDLE,
    }

    // If GK holds ball, distribute upfield after short pause
    if state.ball.owner == Some((side, 0)) && state.ball.gk_hold_ticks < 60 {
        let aim = Vec2::new(Fp::from_int(attack_sign as i32), Fp::from_milli(250));
        return vec_and_buttons_to_input(aim, Vec2::ZERO, BTN_PASS);
    }

    // Reaction cadence by difficulty
    let decision_cadence: u32 = match difficulty {
        CpuDifficulty::Easy => 18,
        CpuDifficulty::Normal => 10,
        CpuDifficulty::Hard => 5,
    };

    // If already charging a pass/shot, release at target charge ticks!
    if let PlayerActionState::Charging { button, ticks, .. } = me.action {
        let target_charge: u16 = if (button & BTN_SHOOT) != 0 { 13 } else { 9 };
        let aim = Vec2::new(Fp::from_int(attack_sign as i32), Fp::ZERO);
        if ticks >= target_charge {
            return vec_and_buttons_to_input(aim, Vec2::ZERO, 0);
        } else {
            return vec_and_buttons_to_input(aim, Vec2::ZERO, button);
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
            Fp::from_milli(14_500)
        } else {
            Fp::from_milli(21_000)
        };

        // Nearest opponent distance
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

        // 1. Shoot when in shooting range!
        if dist_to_goal <= max_shoot_dist && state.tick.is_multiple_of(decision_cadence) {
            let aim = (shot_target - me.pos).normalize_or_zero();
            return vec_and_buttons_to_input(aim, aim, BTN_SHOOT);
        }

        // 2. Pass or through-ball if pressured or if a teammate is in a much better forward spot!
        if state.tick.is_multiple_of(decision_cadence) {
            let mut best_mate: Option<(usize, Vec2, Fp, bool)> = None;
            for (m_idx, mate) in state.players[side_idx].iter().enumerate() {
                if m_idx == ctrl_idx || m_idx == 0 || mate.sent_off {
                    continue;
                }
                let dist = me.pos.dist(mate.pos);
                if dist < Fp::from_milli(3_000) || dist > Fp::from_milli(28_000) {
                    continue;
                }
                // Check openness
                let mut mate_clearance = Fp::from_int(50);
                for opp in &state.players[side.opponent().index()] {
                    mate_clearance = mate_clearance.min(mate.pos.dist(opp.pos));
                }
                let forward_gain = (mate.pos.x - me.pos.x).mul(Fp::from_int(attack_sign as i32));
                let score = forward_gain + mate_clearance.mul(Fp::from_milli(800));
                let should_through = mate.role == PlayerRole::Forward
                    && forward_gain > Fp::from_milli(3_500)
                    && mate_clearance > Fp::from_milli(3_000);
                if best_mate.is_none_or(|(_, _, best_s, _)| score > best_s) {
                    best_mate = Some((m_idx, mate.pos, score, should_through));
                }
            }

            if let Some((_, mate_pos, score, should_through)) = best_mate {
                let under_pressure = nearest_opp_dist < Fp::from_milli(2_700);
                let great_forward_option = score > Fp::from_milli(7_500);
                if under_pressure || great_forward_option {
                    let aim = (mate_pos - me.pos).normalize_or_zero();
                    let btn = if should_through && difficulty != CpuDifficulty::Easy {
                        BTN_THROUGH
                    } else {
                        BTN_PASS
                    };
                    return vec_and_buttons_to_input(aim, aim, btn);
                }
            }
        }

        // 3. Dribble progressively toward opponent goal, angling slightly away from nearest tackler
        let mut drive = (shot_target - me.pos).normalize_or_zero();
        if nearest_opp_dist < Fp::from_milli(4_000) && nearest_opp_vec.len() > Fp::from_milli(100) {
            let evade = nearest_opp_vec.normalize_or_zero().scale(Fp::from_milli(550));
            drive = (drive - evade).normalize_or_zero();
        }
        let sprint = nearest_opp_dist > Fp::from_milli(2_800) && difficulty != CpuDifficulty::Easy;
        return vec_and_buttons_to_input(
            drive,
            Vec2::ZERO,
            if sprint { BTN_SPRINT } else { 0 },
        );
    }

    // CPU does NOT own ball on `ctrl_idx`: defend or chase loose ball!
    let ball_xy = state.ball.pos.xy();
    let my_dist = me.pos.dist(ball_xy);

    // Switch to a closer outfield defender if needed
    if state.switch_cooldown[side_idx] == 0 && state.tick.is_multiple_of(decision_cadence) {
        for (idx, mate) in state.players[side_idx].iter().enumerate() {
            if idx == 0 || idx == ctrl_idx || mate.sent_off {
                continue;
            }
            if mate.pos.dist(ball_xy) + Fp::from_milli(1_800) < my_dist {
                return vec_and_buttons_to_input(Vec2::ZERO, Vec2::ZERO, BTN_SWITCH);
            }
        }
    }

    // Approach goal-side and tackle when within reach
    let goal_side_pt = if state.ball.owner.is_some() {
        let to_own_goal = (Vec2::new(own_goal_x, Fp::ZERO) - ball_xy)
            .normalize_or_zero()
            .scale(Fp::from_milli(750));
        ball_xy + to_own_goal
    } else {
        ball_xy
    };

    let move_dir = (goal_side_pt - me.pos).normalize_or_zero();
    let mut buttons = 0_u16;
    if my_dist > Fp::from_milli(2_500) {
        buttons |= BTN_SPRINT;
    } else {
        buttons |= BTN_JOCKEY;
    }
    if state.ball.owner.is_some()
        && state.ball.owner.map(|(s, _)| s) != Some(side)
        && my_dist <= Fp::from_milli(1_250)
        && state.tick.is_multiple_of(decision_cadence)
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

trait ScalarSignExt {
    fn normalize_scalar(self) -> Self;
}

impl ScalarSignExt for Fp {
    fn normalize_scalar(self) -> Self {
        if self.0 >= 0 {
            Fp::ONE
        } else {
            Fp::NEG_ONE
        }
    }
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
