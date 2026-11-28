//! Deterministic Goalkeeper AI, 3-Tier Outfield Tactical AI, and Physics-Based CPU Controller.

use crate::config::{CpuDifficulty, TeamSide};
use crate::input::{
    charge_units_to_ticks, InputFrame, BTN_CONTAIN, BTN_FINESSE, BTN_JOCKEY, BTN_LOB, BTN_PASS,
    BTN_SHOOT, BTN_SKILL, BTN_SPRINT, BTN_TACKLE, BTN_THROUGH,
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

/// Computes exact desired charge ticks (`1..=22`) matching `execute_kick`'s `pref_dist` curve
/// so CPU passes/lobs arrive crisp at the intended teammate instead of falling short.
pub fn desired_pass_charge_ticks(dist: Fp, is_5v5: bool, is_lob: bool) -> u8 {
    let base_mm = if is_5v5 { 3_800 } else { 4_500 };
    let span_mm = if is_5v5 { 22_500 } else { 28_500 };
    let raw = (((dist.to_milli() - base_mm).max(0) * 22) / span_mm).clamp(4, 20) as u8;
    if is_lob {
        raw.clamp(9, 19)
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

/// Scenario-based goalkeeper movement / dive intent for `players[side][0]`.
///
/// Pure view over [`decide_goalkeeper`] (which also returns the updated [`GoalkeeperBrain`]); the
/// simulation calls `decide_goalkeeper` and stores the brain so commitments and hysteresis persist.
pub fn compute_goalkeeper_intent(
    state: &MatchState,
    side: TeamSide,
    human_gk_rush: bool,
) -> AiMoveIntent {
    decide_goalkeeper(state, side, human_gk_rush).0
}

/// Returns `(button, charge_ticks, aim)`; `charge_ticks` is already scaled to the button's power bar.
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
        let short_max = Fp::from_milli(if is_5v5 { 26_000 } else { 32_000 });
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
        return (BTN_PASS, charge_units_to_ticks(charge.clamp(6, 18), BTN_PASS), dir);
    }

    if let Some((dir, dist, _)) = best_long {
        let charge = desired_pass_charge_ticks(dist, is_5v5, true) as u16;
        return (BTN_LOB, charge_units_to_ticks(charge.clamp(11, 19), BTN_LOB), dir);
    }

    let flank_y = if gk_pos.y >= Fp::ZERO {
        Fp::from_milli(-380)
    } else {
        Fp::from_milli(380)
    };
    (
        BTN_LOB,
        charge_units_to_ticks(14, BTN_LOB),
        Vec2::new(forward_vec.x, flank_y).normalize_or_zero(),
    )
}

// ---------------------------------------------------------------------------------------------
// Goalkeeper scenario brain (see `docs/goalkeeper-ai.md`).
// ---------------------------------------------------------------------------------------------

use crate::state::{GkScenario, GoalkeeperBrain};

/// Keeper style derived from attributes (documented in `docs/goalkeeper-ai.md`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GkStyle {
    /// `positioning >= 80 && pace >= 60`: large sweep radius, smaller sweep margin.
    Sweeper,
    /// `goalkeeping >= 85 && strength >= 70`: larger cross command zone, smaller claim margin.
    Commanding,
    /// Everyone else: stays close to the goal, sweeps only inside a short radius.
    Line,
}

/// Attribute + difficulty derived goalkeeper decision parameters (all integer millimetres / ticks).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GkProfile {
    pub style: GkStyle,
    pub difficulty: CpuDifficulty,
    /// Extra set-position depth at mid range (`-500..=1500` mm): good keepers narrow the angle, poor ones hug the line.
    pub set_depth_bonus_mm: i64,
    /// Max distance from the goal centre at which the keeper will sweep a loose ball.
    pub sweep_radius_mm: i64,
    /// Max distance from the goal centre at which the keeper will claim a high ball.
    pub command_radius_mm: i64,
    /// Arrival advantage (ticks) over the best attacker required to claim a cross / collect in the box.
    pub claim_margin_ticks: i64,
    /// Arrival advantage (ticks) over the best attacker required to sweep outside the box.
    pub sweep_margin_ticks: i64,
    /// Added shot reflex latency (ticks; negative = faster).
    pub react_adjust_ticks: i64,
    /// Max deterministic judgement error (ticks) applied to the keeper's own arrival estimate.
    pub judgement_err_ticks: i64,
    /// 1v1 stand-off distance from the ball.
    pub standoff_mm: i64,
}

/// Derives the keeper style from attributes.
pub const fn goalkeeper_style(a: &crate::snapshot::PlayerAttributes) -> GkStyle {
    if a.positioning >= 80 && a.pace >= 60 {
        GkStyle::Sweeper
    } else if a.goalkeeping >= 85 && a.strength >= 70 {
        GkStyle::Commanding
    } else {
        GkStyle::Line
    }
}

/// CPU difficulty driving `side`'s keeper (`Normal` for a human-controlled side: the keeper is always AI).
pub fn side_gk_difficulty(state: &MatchState, side: TeamSide) -> CpuDifficulty {
    let ctrl = match side {
        TeamSide::Home => state.rules.home_controller,
        TeamSide::Away => state.rules.away_controller,
    };
    match ctrl {
        crate::config::SideController::Cpu(d) => d,
        crate::config::SideController::Human => CpuDifficulty::Normal,
    }
}

/// Builds the decision profile for `side`'s keeper.
pub fn goalkeeper_profile(state: &MatchState, side: TeamSide) -> GkProfile {
    let a = state.players[side.index()][0].attributes;
    let gk = a.goalkeeping as i64;
    let pos = a.positioning as i64;
    let pace = a.pace as i64;
    let difficulty = side_gk_difficulty(state, side);
    let style = goalkeeper_style(&a);
    let box_mm = state.format.penalty_box_depth_mm;
    let ga_mm = box_mm / 3;

    let set_depth_bonus_mm = ((pos - 60) * 37).clamp(-500, 1_500);

    // Line / commanding keepers sweep inside the box; a sweeper keeper reaches up to half a box beyond it.
    let line_f = ((pos + pace - 100) * 1_000 / 90).clamp(250, 800);
    let sweep_radius_mm = match style {
        GkStyle::Sweeper => box_mm + (((pos + pace - 140).clamp(0, 100)) * box_mm) / 200,
        GkStyle::Commanding => ga_mm + ((box_mm - ga_mm) * line_f * 1_100) / 1_000_000,
        GkStyle::Line => ga_mm + ((box_mm - ga_mm) * line_f) / 1_000,
    };

    let mut cmd_f = (900 + (gk + pos - 150) * 8).clamp(750, 1_350);
    if style == GkStyle::Commanding {
        cmd_f = cmd_f * 1_150 / 1_000;
    }
    let command_radius_mm = (ga_mm * cmd_f) / 1_000;

    let (claim_base, sweep_base, react_adjust_ticks, judgement_err_ticks) = match difficulty {
        CpuDifficulty::Easy => (9, 12, 3, 6),
        CpuDifficulty::Normal => (6, 9, 0, 2),
        CpuDifficulty::Hard => (4, 7, -1, 0),
    };
    let claim_margin_ticks = (claim_base - (gk - 75) / 10
        - i64::from(style == GkStyle::Commanding))
    .max(2);
    let sweep_margin_ticks =
        (sweep_base - (pos - 75) / 10 - if style == GkStyle::Sweeper { 2 } else { 0 }).max(4);
    let standoff_mm = 2_500 - (pos.clamp(50, 100) - 50) * 20;

    GkProfile {
        style,
        difficulty,
        set_depth_bonus_mm,
        sweep_radius_mm,
        command_radius_mm,
        claim_margin_ticks,
        sweep_margin_ticks,
        react_adjust_ticks,
        judgement_err_ticks,
        standoff_mm,
    }
}

/// Own-goal relative geometry: `depth` is distance from the own goal line into the pitch.
#[derive(Clone, Copy)]
struct GkGeo {
    goal_x: Fp,
    into: Fp,
    goal_hw: Fp,
    box_depth: Fp,
    box_hw: Fp,
    ga_depth: Fp,
    ga_hw: Fp,
    is_5v5: bool,
}

impl GkGeo {
    fn new(state: &MatchState, side: TeamSide) -> Self {
        let attack_sign = side.attack_sign(state.second_half_swapped());
        let half_len = state.format.half_length();
        let box_depth = state.format.penalty_box_depth();
        let ga_depth = Fp::from_milli(state.format.penalty_box_depth_mm / 3);
        let goal_hw = state.format.goal_half_width();
        Self {
            goal_x: Fp::from_milli(-attack_sign * half_len.to_milli()),
            into: Fp::from_int(attack_sign as i32),
            goal_hw,
            box_depth,
            box_hw: state.format.penalty_box_half_width(),
            ga_depth,
            ga_hw: goal_hw + ga_depth,
            is_5v5: state.format.players_per_side <= 6,
        }
    }

    #[inline]
    fn depth(&self, pt: Vec2) -> Fp {
        (pt.x - self.goal_x).mul(self.into)
    }

    #[inline]
    fn point(&self, depth: Fp, y: Fp) -> Vec2 {
        Vec2::new(self.goal_x + self.into.mul(depth), y)
    }

    #[inline]
    fn goal_center(&self) -> Vec2 {
        Vec2::new(self.goal_x, Fp::ZERO)
    }

    #[inline]
    fn in_box(&self, pt: Vec2) -> bool {
        let d = self.depth(pt);
        d >= Fp::from_milli(-300) && d <= self.box_depth && pt.y.abs() <= self.box_hw
    }

    #[inline]
    fn in_goal_area(&self, pt: Vec2) -> bool {
        let d = self.depth(pt);
        d <= self.ga_depth && pt.y.abs() <= self.ga_hw
    }

    /// Deepest set-position depth (stay well inside the goal area, momentum overshoot included).
    #[inline]
    fn max_set_depth(&self) -> Fp {
        self.ga_depth - Fp::from_milli(600)
    }

    /// Post-to-post angle-bisector anchor on the goal line for a ball at `ball_xy`.
    fn bisect_anchor(&self, ball_xy: Vec2) -> Vec2 {
        let post_neg = Vec2::new(self.goal_x, -self.goal_hw);
        let post_pos = Vec2::new(self.goal_x, self.goal_hw);
        let d_neg = ball_xy.dist(post_neg);
        let d_pos = ball_xy.dist(post_pos);
        let d_sum = (d_neg + d_pos).max(Fp::ONE);
        let y = self
            .goal_hw
            .mul(d_neg - d_pos)
            .div(d_sum)
            .clamp(-(self.goal_hw - Fp::from_milli(250)), self.goal_hw - Fp::from_milli(250));
        Vec2::new(self.goal_x, y)
    }
}

/// Deterministic judgement noise in `[-max, max]` ticks, re-drawn every half second.
fn gk_judgement_noise(state: &MatchState, side: TeamSide, max: i64) -> i64 {
    if max <= 0 {
        return 0;
    }
    let mut z = ((state.tick / 30) as u64) ^ ((side.index() as u64 + 1) << 40) ^ 0x9E37_79B9_7F4A_7C15;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    (z % (2 * max as u64 + 1)) as i64 - max
}

/// Race read for a loose ball: where / when the keeper meets it versus the best attacker and defender.
#[derive(Clone, Copy, Debug)]
struct LooseRead {
    gk_pt: Vec2,
    gk_t: i64,
    opp_t: i64,
    mate_t: i64,
}

const NO_REACH_TICKS: i64 = 200;

/// First waypoint tick `p` can reach with the ball no higher than `reach_z`.
fn first_reach_ticks(p: &PlayerState, waypoints: &[Vec3; 21], is_5v5: bool, reach_z: Fp) -> i64 {
    for (i, pt3) in waypoints.iter().enumerate() {
        let t = (i as u32 + 1) * 4;
        if pt3.z <= reach_z && estimate_ticks_to_point(p, pt3.xy(), is_5v5) <= t {
            return t as i64;
        }
    }
    NO_REACH_TICKS
}

/// Reads the loose-ball race. The keeper's point is the first waypoint he reaches (with `err` ticks of
/// judgement error) that satisfies `region` at a height he can play (`2.9m` with hands in the box, else `1.2m`).
fn read_loose_ball(
    state: &MatchState,
    side: TeamSide,
    geo: &GkGeo,
    waypoints: &[Vec3; 21],
    err: i64,
    opp_reach_z: Fp,
    region: impl Fn(Vec2) -> bool,
) -> Option<LooseRead> {
    let gk = &state.players[side.index()][0];
    let mut found: Option<(Vec2, i64)> = None;
    for (i, pt3) in waypoints.iter().enumerate() {
        let t = ((i as i64) + 1) * 4;
        let pt = pt3.xy();
        if !region(pt) {
            continue;
        }
        let reach_z = if geo.in_box(pt) {
            Fp::from_milli(2_900)
        } else {
            Fp::from_milli(1_200)
        };
        if pt3.z > reach_z {
            continue;
        }
        let need = estimate_ticks_to_point(gk, pt, geo.is_5v5) as i64 + err;
        if need <= t {
            found = Some((pt, t));
            break;
        }
    }
    let (gk_pt, gk_t) = found?;
    let mut opp_t = NO_REACH_TICKS;
    for op in state.players[side.opponent().index()].iter().skip(1) {
        if !op.sent_off {
            opp_t = opp_t.min(first_reach_ticks(op, waypoints, geo.is_5v5, opp_reach_z));
        }
    }
    let mut mate_t = NO_REACH_TICKS;
    for mate in state.players[side.index()].iter().skip(1) {
        if !mate.sent_off {
            mate_t = mate_t.min(first_reach_ticks(mate, waypoints, geo.is_5v5, Fp::from_milli(2_300)));
        }
    }
    Some(LooseRead {
        gk_pt,
        gk_t,
        opp_t,
        mate_t,
    })
}

#[inline]
fn gk_intent(move_dir: Vec2, sprint: bool, jockey: bool) -> AiMoveIntent {
    AiMoveIntent {
        move_dir,
        sprint,
        jockey,
        trigger_dive: None,
    }
}

/// Steers to `target`, sprinting only beyond `sprint_beyond` and setting the feet within `jockey_within`.
fn gk_move_to(p: &PlayerState, target: Vec2, sprint_beyond: Fp, jockey_within: Fp) -> AiMoveIntent {
    let delta = target - p.pos;
    let dist = delta.len();
    if dist < Fp::from_milli(160) {
        return gk_intent(Vec2::ZERO, false, true);
    }
    gk_intent(delta.normalize_or_zero(), dist > sprint_beyond, dist < jockey_within)
}

/// Stage 2 shot-stopping: reflex-latency gated (`goalkeeping` + difficulty + line-of-sight screen) lateral
/// shuffle or dive. Returns `None` when no shot is incoming or it is not on a guardable line.
fn gk_shot_save(
    state: &MatchState,
    side: TeamSide,
    geo: &GkGeo,
    profile: &GkProfile,
) -> Option<AiMoveIntent> {
    let p = &state.players[side.index()][0];
    let ball_xy = state.ball.pos.xy();
    let ball_vel_xy = state.ball.vel.xy();
    let vel_toward_goal = -ball_vel_xy.x.mul(geo.into);
    let dist_ball_to_goal_x = geo.depth(ball_xy).abs();
    let max_shot_react_dist = Fp::from_milli(if geo.is_5v5 { 26_500 } else { 31_000 });
    let live_shot = state.ball.shot_launched_tick > 0;
    // A fast ball on frame (deflection / header) counts as a shot even without a struck-shot flag.
    let on_frame_fast = vel_toward_goal > Fp::from_milli(9_000) && {
        let t = dist_ball_to_goal_x.div(vel_toward_goal.max(Fp::ONE));
        (ball_xy.y + ball_vel_xy.y.mul(t)).abs() <= geo.goal_hw + Fp::from_milli(1_000)
    };
    if state.ball.owner.is_some()
        || !matches!(p.action, PlayerActionState::Normal)
        || vel_toward_goal <= Fp::from_milli(5_200)
        || dist_ball_to_goal_x >= max_shot_react_dist
        || dist_ball_to_goal_x <= Fp::from_milli(220)
        || !(live_shot || on_frame_fast)
    {
        return None;
    }

    let gk_skill = p.attributes.goalkeeping as i64;
    // Human GK reflex latency (~117ms elite Hard .. ~250ms Easy) + FIFA 23 Line-of-Sight screen penalty (+50ms).
    let base_react = ((13 - gk_skill / 20).clamp(7, 12) + profile.react_adjust_ticks).max(6) as u32;
    let mut screened = false;
    if live_shot {
        let ray = p.pos - state.ball.shot_origin;
        let ray_len = ray.len();
        if ray_len > Fp::from_milli(1_500) {
            let ray_dir = ray.normalize_or_zero();
            'outer: for (t_i, team_arr) in state.players.iter().enumerate() {
                for (p_i, other) in team_arr.iter().enumerate() {
                    if other.sent_off || (t_i == side.index() && p_i == 0) {
                        continue;
                    }
                    let rel = other.pos - state.ball.shot_origin;
                    let along = rel.dot(ray_dir);
                    if along > Fp::from_milli(700) && along < ray_len - Fp::from_milli(700) {
                        let perp = (rel - ray_dir.scale(along)).len();
                        if perp <= Fp::from_milli(850) {
                            screened = true;
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    let react_ticks = base_react + if screened { 3 } else { 0 };
    if live_shot && state.tick.saturating_sub(state.ball.shot_launched_tick) < react_ticks {
        // Still inside the reflex latency window: hold a balanced set stance.
        return Some(gk_intent(Vec2::ZERO, false, true));
    }

    let dist_to_gk_x = (ball_xy.x - p.pos.x).abs();
    let t_to_gk = dist_to_gk_x.div(vel_toward_goal);
    let pred_ticks = ((t_to_gk.to_milli() * 60 + 500) / 1_000).clamp(1, 60) as u32;
    let proj_3d = predict_ball_3d_at_ticks(state, pred_ticks);
    let projected_y = proj_3d.y;
    let projected_z = proj_3d.z.max(Fp::ZERO);
    let max_guard_y = geo.goal_hw + Fp::from_milli(950);
    if projected_y.abs() > max_guard_y {
        return None;
    }

    let lateral_gap = projected_y - p.pos.y;
    let dive_reach = Fp::from_milli(1_850 + gk_skill * 12);
    let close_enough_in_time =
        t_to_gk <= Fp::from_milli(440) || dist_to_gk_x <= Fp::from_milli(9_500);
    if close_enough_in_time
        && lateral_gap.abs() > Fp::from_milli(480)
        && lateral_gap.abs() <= dive_reach
    {
        let dive_dir = Vec2::new(
            geo.into.mul(Fp::from_milli(180)),
            Fp::from_int(lateral_gap.signum_int()),
        )
        .normalize_or_zero();
        let dive_vz_mm = if projected_z > Fp::from_milli(880) {
            let z_excess = (projected_z.to_milli() - 880).clamp(0, 1_600);
            (2_150 + (z_excess * 12) / 10).clamp(2_150, 4_000)
        } else {
            280
        };
        return Some(AiMoveIntent {
            move_dir: dive_dir,
            sprint: true,
            jockey: false,
            trigger_dive: Some((dive_dir, dive_vz_mm)),
        });
    }
    if lateral_gap.abs() <= dive_reach + Fp::from_milli(650) {
        let intercept = Vec2::new(p.pos.x, projected_y.clamp(-geo.goal_hw, geo.goal_hw));
        return Some(gk_intent(
            (intercept - p.pos).normalize_or_zero(),
            lateral_gap.abs() > Fp::from_milli(900),
            true,
        ));
    }
    None
}

/// Scenario 1 (Set / ready): angle-bisector arc whose depth grows with ball distance and `positioning`.
fn gk_set_target(geo: &GkGeo, profile: &GkProfile, ball_xy: Vec2) -> Vec2 {
    let anchor = geo.bisect_anchor(ball_xy);
    let to_ball = ball_xy - anchor;
    let dist_ab = to_ball.len().max(Fp::ONE);
    let dir_ab = to_ball.normalize_or_zero();
    let (min_arc_mm, max_arc_mm) = if geo.is_5v5 { (1_380, 3_800) } else { (1_450, 4_250) };
    let d_mm = dist_ab.to_milli();
    // Depth grows with ball distance up to a plateau (no in-out stepping on every midfield pass).
    let plateau_mm = if geo.is_5v5 { 22_000 } else { 28_000 };
    let base_mm = min_arc_mm
        + ((d_mm.clamp(4_000, plateau_mm) - 4_000) * (max_arc_mm - min_arc_mm)) / (plateau_mm - 4_000);
    // Positioning bonus ramps in from 5m to 12m of ball distance (no bonus point-blank).
    let ramp_permille = ((d_mm - 5_000).clamp(0, 7_000) * 1_000) / 7_000;
    let r_mm = (base_mm + (profile.set_depth_bonus_mm * ramp_permille) / 1_000)
        .min(geo.max_set_depth().to_milli())
        .min(d_mm - 1_000)
        .max(if geo.is_5v5 { 880 } else { 950 });
    let raw = anchor + dir_ab.scale(Fp::from_milli(r_mm));
    let max_y = (geo.goal_hw - Fp::from_milli(220)).max(Fp::from_milli(850));
    let depth = geo
        .depth(raw)
        .max(Fp::from_milli(if geo.is_5v5 { 880 } else { 950 }));
    geo.point(depth, raw.y.clamp(-max_y, max_y))
}

/// Scenario 7b (Support): own outfielder in possession -> the set depth for the ball, shaded central so the
/// keeper is a square back-pass outlet (same depth as Set: no in/out step on every turnover).
fn gk_support_target(geo: &GkGeo, profile: &GkProfile, ball_xy: Vec2) -> Vec2 {
    let set = gk_set_target(geo, profile, ball_xy);
    geo.point(geo.depth(set), set.y.mul(Fp::from_milli(500)))
}

/// Scenario 3b (Cross hold): near-post / central ready stance for a high ball the keeper will not claim.
fn gk_cross_hold_target(state: &MatchState, side: TeamSide, geo: &GkGeo, landing: Vec2) -> Vec2 {
    let corner_live = state.restart_touch_state.is_some_and(|rt| {
        rt.kind == Some(RestartKind::CornerKick) && rt.taker_side == side.opponent()
    });
    let depth = Fp::from_milli(if geo.is_5v5 { 1_250 } else { 1_450 });
    let max_y = geo.goal_hw - Fp::from_milli(600);
    let y = if corner_live {
        Fp::from_milli(if landing.y >= Fp::ZERO { 420 } else { -420 })
    } else {
        landing.y.mul(Fp::from_milli(350)).clamp(-max_y, max_y)
    };
    geo.point(depth, y)
}

/// Dead-ball stance (corner / free kick wall-side / penalty). Only reached when the simulation steps the
/// keeper outside open play; live deliveries are handled by the Cross / Shot scenarios.
pub fn goalkeeper_set_piece_stance(state: &MatchState, side: TeamSide) -> Option<Vec2> {
    let MatchPhase::SetPiece {
        kind, team, spot, ..
    } = state.phase
    else {
        return None;
    };
    if team == side {
        return None;
    }
    let geo = GkGeo::new(state, side);
    match kind {
        RestartKind::CornerKick => Some(geo.point(
            Fp::from_milli(if geo.is_5v5 { 1_450 } else { 1_650 }),
            Fp::from_milli(if spot.y >= Fp::ZERO { 420 } else { -420 }),
        )),
        RestartKind::PenaltyKick => Some(geo.point(Fp::from_milli(100), Fp::ZERO)),
        RestartKind::FreeKick if geo.depth(spot) <= geo.box_depth + Fp::from_milli(14_000) => {
            // The wall covers the near post: keeper shades toward the far post.
            let anchor = geo.bisect_anchor(spot);
            let far_shift = Fp::from_milli(if spot.y >= Fp::ZERO { -450 } else { 450 });
            let max_y = geo.goal_hw - Fp::from_milli(400);
            Some(geo.point(
                Fp::from_milli(if geo.is_5v5 { 900 } else { 1_000 }),
                (anchor.y + far_shift).clamp(-max_y, max_y),
            ))
        }
        _ => None,
    }
}

/// Scenario 5 (1v1) check: opposing carrier through on goal with no goal-side covering defender.
/// `staying` loosens the thresholds (hysteresis) once the keeper is already in the 1v1.
fn gk_one_v_one_carrier(state: &MatchState, side: TeamSide, geo: &GkGeo, staying: bool) -> Option<Vec2> {
    let (o_side, o_idx) = state.ball.owner?;
    if o_side != side.opponent() {
        return None;
    }
    let carrier = &state.players[o_side.index()][o_idx];
    let c = state.ball.pos.xy();
    let c_depth = geo.depth(c);
    let extra = Fp::from_milli(if geo.is_5v5 { 2_500 } else { 3_500 })
        + if staying { Fp::from_milli(3_000) } else { Fp::ZERO };
    if c_depth > geo.box_depth + extra
        || c_depth < Fp::from_milli(600)
        || c.y.abs() > geo.box_hw + if staying { Fp::from_milli(2_000) } else { Fp::ZERO }
    {
        return None;
    }
    let goal = geo.goal_center();
    let to_goal = goal - c;
    let carrier_attacking = carrier.vel.dot(to_goal.normalize_or_zero()) > Fp::from_milli(1_500);
    if !(carrier_attacking || c_depth <= geo.box_depth) {
        return None;
    }
    let cover_w = Fp::from_milli(if staying { 1_400 } else { 2_200 });
    let tight_r = Fp::from_milli(if staying { 1_000 } else { 1_600 });
    let seg_len = to_goal.len().max(Fp::ONE);
    let seg_dir = to_goal.normalize_or_zero();
    for mate in state.players[side.index()].iter().skip(1) {
        if mate.sent_off {
            continue;
        }
        let m_depth = geo.depth(mate.pos);
        let rel = mate.pos - c;
        let along = rel.dot(seg_dir);
        let perp = (rel - seg_dir.scale(along)).len();
        let goal_side = m_depth < c_depth - Fp::from_milli(200);
        if goal_side && along > Fp::ZERO && along < seg_len && perp <= cover_w {
            return None;
        }
        if mate.pos.dist(c) <= tight_r && m_depth < c_depth + Fp::from_milli(300) {
            return None;
        }
    }
    Some(c)
}

/// Scenario 5 (1v1) movement: advance on the bisector, decelerate, stand big at the stand-off. Never dives.
fn gk_one_v_one_intent(p: &PlayerState, geo: &GkGeo, profile: &GkProfile, c: Vec2) -> (AiMoveIntent, Vec2) {
    let anchor = geo.bisect_anchor(c);
    let to_ball = c - anchor;
    let dist_ab = to_ball.len().max(Fp::ONE);
    // Far out the keeper advances gradually (`0.42 x` ball distance); from half the box depth inwards he
    // closes to the stand-off. Capped by a style-scaled limit.
    let out_f = match profile.style {
        GkStyle::Line => 250,
        GkStyle::Commanding => 350,
        GkStyle::Sweeper => 500,
    };
    let max_out = geo.ga_depth + (geo.box_depth - geo.ga_depth).mul(Fp::from_milli(out_f));
    let half_box = geo.box_depth.mul(Fp::from_milli(500));
    let w = ((dist_ab - half_box).div(half_box)).clamp(Fp::ZERO, Fp::ONE);
    let r_close = dist_ab - Fp::from_milli(profile.standoff_mm);
    let r_far = dist_ab.mul(Fp::from_milli(420));
    let r = (r_close.mul(Fp::ONE - w) + r_far.mul(w))
        .min(r_close)
        .min(max_out)
        .max(Fp::from_milli(600));
    let target = anchor + to_ball.normalize_or_zero().scale(r);
    let delta = target - p.pos;
    let d = delta.len();
    let d_carrier = p.pos.dist(c);
    let intent = gk_intent(
        if d > Fp::from_milli(150) {
            delta.normalize_or_zero()
        } else {
            Vec2::ZERO
        },
        d > Fp::from_milli(2_800) && d_carrier > Fp::from_milli(6_000),
        d < Fp::from_milli(2_200) || d_carrier < Fp::from_milli(4_500),
    );
    (intent, target)
}

/// Outcome of the scenario selection for one tick.
struct GkChoice {
    scenario: GkScenario,
    intent: AiMoveIntent,
    target: Vec2,
    /// `Some(n)` starts / refreshes a commitment window of `n` ticks.
    commit: Option<u16>,
}

/// Loose-ball scenarios (3 Cross, 4 Sweep, 6 Collect). `only` restricts the evaluation to one committed
/// scenario (continuation check with abort slack); `extra_margin` is the post-abort re-entry penalty.
fn gk_loose_ball_choice(
    state: &MatchState,
    side: TeamSide,
    geo: &GkGeo,
    profile: &GkProfile,
    prev: &GoalkeeperBrain,
    only: Option<GkScenario>,
    extra_margin: i64,
) -> Option<GkChoice> {
    if state.ball.owner.is_some() || !matches!(state.phase, MatchPhase::OpenPlay) {
        return None;
    }
    let p = &state.players[side.index()][0];
    let waypoints = sample_ball_3d_waypoints(state);
    let err = gk_judgement_noise(state, side, profile.judgement_err_ticks);
    let ball_xy = state.ball.pos.xy();
    let goal = geo.goal_center();
    let continuing = only.is_some();
    // Continuing a commitment only aborts once an attacker is clearly first (2-tick slack).
    let margin_for = |base: i64, sc: GkScenario| -> i64 {
        if continuing {
            -2
        } else if prev.scenario == sc {
            base - 2 + extra_margin
        } else {
            base + extra_margin
        }
    };
    // A committed claim whose ball has dropped may finish as a collect (and vice versa for a bounce).
    let allow = |sc: GkScenario| {
        only.is_none_or(|o| {
            o == sc
                || (o == GkScenario::CrossClaim && sc == GkScenario::Collect)
                || (o == GkScenario::Collect && sc == GkScenario::CrossClaim)
        })
    };

    let high = state.ball.pos.z > Fp::from_milli(1_200)
        || (state.ball.vel.z > Fp::from_milli(2_500) && state.ball.pos.z > Fp::from_milli(250));
    if high && only.is_none_or(|o| o != GkScenario::Sweep) {
        let cmd_r = Fp::from_milli(profile.command_radius_mm);
        let enters_box = waypoints
            .iter()
            .any(|w| geo.in_box(w.xy()) && w.z <= Fp::from_milli(2_900));
        if enters_box {
            if allow(GkScenario::CrossClaim) {
                if let Some(read) = read_loose_ball(
                    state,
                    side,
                    geo,
                    &waypoints,
                    err,
                    Fp::from_milli(2_500),
                    |pt| geo.in_box(pt) && pt.dist(goal) <= cmd_r,
                ) {
                    let outside_ga = !geo.in_goal_area(read.gk_pt);
                    let margin = margin_for(profile.claim_margin_ticks, GkScenario::CrossClaim)
                        + if outside_ga && !continuing { 6 } else { 0 };
                    // Inside the 6-yard box a contested ball (attacker level within 2 ticks) is attacked
                    // anyway: the contact code punches it clear when crowded.
                    let contested_six = !outside_ga && read.gk_t <= read.opp_t + 2;
                    if read.gk_t + margin <= read.opp_t || contested_six {
                        let intent = gk_intent((read.gk_pt - p.pos).normalize_or_zero(), true, false);
                        return Some(GkChoice {
                            scenario: GkScenario::CrossClaim,
                            intent,
                            target: read.gk_pt,
                            commit: (!continuing).then_some((read.gk_t + 8).clamp(12, 70) as u16),
                        });
                    }
                }
            }
            if only.is_none() {
                // Hold a near-post / central stance and get ready to react.
                let landing = waypoints
                    .iter()
                    .find(|w| geo.in_box(w.xy()) && w.z <= Fp::from_milli(2_900))
                    .map_or(ball_xy, |w| w.xy());
                let target = gk_cross_hold_target(state, side, geo, landing);
                return Some(GkChoice {
                    scenario: GkScenario::CrossHold,
                    intent: gk_move_to(p, target, Fp::from_milli(1_200), Fp::from_milli(1_000)),
                    target,
                    commit: None,
                });
            }
            return None;
        }
    }

    // Ground / dropping loose ball: collect inside the box, sweep behind the line.
    let sweep_r = Fp::from_milli(profile.sweep_radius_mm);
    let reach_r = sweep_r.max(geo.box_depth);
    let read = read_loose_ball(state, side, geo, &waypoints, err, Fp::from_milli(2_300), |pt| {
        geo.depth(pt) >= Fp::from_milli(-300)
            && pt.dist(goal) <= reach_r
            && pt.y.abs() <= geo.box_hw + Fp::from_milli(3_000)
    })?;
    // A defender clearly first (and ahead of the attacker) deals with it.
    let mate_handles = read.mate_t + 4 <= read.gk_t && read.mate_t + 2 <= read.opp_t;

    if geo.in_box(read.gk_pt) {
        if !allow(GkScenario::Collect) {
            return None;
        }
        let attacker_on_it = state.players[side.opponent().index()]
            .iter()
            .skip(1)
            .any(|op| {
                !op.sent_off
                    && (op.pos.dist(ball_xy) <= Fp::from_milli(1_500)
                        || op.pos.dist(read.gk_pt) <= Fp::from_milli(1_500))
            });
        let deep = !geo.in_goal_area(read.gk_pt);
        let margin = margin_for(profile.claim_margin_ticks, GkScenario::Collect)
            + if deep && !continuing { 3 } else { 0 };
        if (continuing || (!attacker_on_it && !mate_handles)) && read.gk_t + margin <= read.opp_t {
            return Some(GkChoice {
                scenario: GkScenario::Collect,
                intent: gk_intent((read.gk_pt - p.pos).normalize_or_zero(), true, false),
                target: read.gk_pt,
                commit: (!continuing).then_some((read.gk_t + 6).clamp(10, 60) as u16),
            });
        }
        return None;
    }

    if !allow(GkScenario::Sweep) || read.gk_pt.dist(goal) > sweep_r {
        return None;
    }
    // Behind the defensive line: deeper than every own outfielder, or no defender can beat the attacker.
    let mut line_depth = Fp::from_milli(200_000);
    for mate in state.players[side.index()].iter().skip(1) {
        if !mate.sent_off {
            line_depth = line_depth.min(geo.depth(mate.pos));
        }
    }
    let behind_line =
        geo.depth(read.gk_pt) < line_depth + Fp::from_milli(1_000) || read.mate_t >= read.opp_t;
    let margin = margin_for(profile.sweep_margin_ticks, GkScenario::Sweep);
    if (continuing || (behind_line && !mate_handles)) && read.gk_t + margin <= read.opp_t {
        return Some(GkChoice {
            scenario: GkScenario::Sweep,
            intent: gk_intent((read.gk_pt - p.pos).normalize_or_zero(), true, false),
            target: read.gk_pt,
            commit: (!continuing).then_some((read.gk_t + 6).clamp(12, 75) as u16),
        });
    }
    None
}

/// Human-held `BTN_GK_RUSH`: rush a loose ball / contain the carrier (EA FC *GK Rush to Contain*).
fn gk_human_rush(state: &MatchState, side: TeamSide, geo: &GkGeo) -> Option<AiMoveIntent> {
    if !matches!(state.phase, MatchPhase::OpenPlay) {
        return None;
    }
    let p = &state.players[side.index()][0];
    let ball_xy = state.ball.pos.xy();
    let dist_ball_to_goal_x = geo.depth(ball_xy).abs();
    if state.ball.owner.is_none() {
        if dist_ball_to_goal_x < geo.box_depth + Fp::from_milli(4_500) {
            return Some(gk_intent((ball_xy - p.pos).normalize_or_zero(), true, false));
        }
        return None;
    }
    let opp_owns_ball = state.ball.owner.map(|(s, _)| s) == Some(side.opponent());
    if !opp_owns_ball || dist_ball_to_goal_x >= geo.box_depth + Fp::from_milli(5_500) {
        return None;
    }
    let anchor = geo.bisect_anchor(ball_xy);
    let to_ball = ball_xy - anchor;
    let dist_ab = to_ball.len().max(Fp::ONE);
    let rush_r = (dist_ab - Fp::from_milli(1_450)).max(Fp::from_milli(1_100));
    let contain_pt = anchor + to_ball.normalize_or_zero().scale(rush_r);
    let delta = contain_pt - p.pos;
    Some(gk_intent(
        if delta.len() > Fp::from_milli(180) {
            delta.normalize_or_zero()
        } else {
            Vec2::ZERO
        },
        delta.len() > Fp::from_milli(1_150),
        delta.len() <= Fp::from_milli(1_800),
    ))
}

/// Deterministic goalkeeper scenario state machine. Returns this tick's movement / dive intent and the
/// updated brain (the caller stores it in `MatchState::gk_brain`). Scenario table: `docs/goalkeeper-ai.md`.
pub fn decide_goalkeeper(
    state: &MatchState,
    side: TeamSide,
    human_gk_rush: bool,
) -> (AiMoveIntent, GoalkeeperBrain) {
    let prev = state.gk_brain[side.index()];
    let choice = select_goalkeeper_scenario(state, side, human_gk_rush, &prev);
    let mut brain = prev;
    if choice.scenario != prev.scenario {
        brain.switches = brain.switches.saturating_add(1);
        brain.entered_tick = state.tick;
        brain.commit_ticks = 0;
    }
    let aborted = prev.scenario.is_committed()
        && prev.commit_ticks > 0
        && choice.scenario != prev.scenario
        && !choice.scenario.is_outward()
        && !matches!(choice.scenario, GkScenario::Shot | GkScenario::Possession);
    brain.scenario = choice.scenario;
    brain.target = choice.target;
    brain.last_decision_tick = state.tick;
    brain.commit_ticks = match choice.commit {
        Some(n) => n,
        None if choice.scenario == prev.scenario => prev.commit_ticks.saturating_sub(1),
        None => 0,
    };
    brain.reentry_block_ticks = if aborted {
        24
    } else {
        prev.reentry_block_ticks.saturating_sub(1)
    };
    (choice.intent, brain)
}

fn select_goalkeeper_scenario(
    state: &MatchState,
    side: TeamSide,
    human_gk_rush: bool,
    prev: &GoalkeeperBrain,
) -> GkChoice {
    let p = &state.players[side.index()][0];
    let geo = GkGeo::new(state, side);
    let profile = goalkeeper_profile(state, side);
    let ball_xy = state.ball.pos.xy();

    // 7. Possession: holding / controlling the ball -> step toward an open outlet angle.
    if state.ball.owner == Some((side, 0)) {
        let (_, _, safe_aim) = choose_safe_gk_distribution(state, side);
        let target = geo.point(
            Fp::from_milli(if geo.is_5v5 { 3_800 } else { 4_600 }),
            safe_aim.y.mul(Fp::from_milli(1_600)).clamp(-geo.goal_hw, geo.goal_hw),
        );
        let delta = target - p.pos;
        let intent = if delta.len() > Fp::from_milli(320) {
            gk_intent(delta.normalize_or_zero(), false, false)
        } else {
            gk_intent(Vec2::ZERO, false, false)
        };
        return GkChoice {
            scenario: GkScenario::Possession,
            intent,
            target,
            commit: None,
        };
    }

    // 9. Dead-ball stance (outside open play).
    if !matches!(state.phase, MatchPhase::OpenPlay) {
        let target = goalkeeper_set_piece_stance(state, side).unwrap_or_else(|| gk_set_target(&geo, &profile, ball_xy));
        let d = target - p.pos;
        return GkChoice {
            scenario: GkScenario::SetPiece,
            intent: gk_intent(
                if d.len() > Fp::from_milli(150) {
                    d.normalize_or_zero()
                } else {
                    Vec2::ZERO
                },
                false,
                true,
            ),
            target,
            commit: None,
        };
    }

    // Human-held GK rush (held = rush out; release = recover through the normal scenarios).
    if human_gk_rush {
        if let Some(intent) = gk_human_rush(state, side, &geo) {
            return GkChoice {
                scenario: GkScenario::HumanRush,
                intent,
                target: ball_xy,
                commit: None,
            };
        }
    }

    // 2. Shot: highest priority over every positional scenario (including a committed 1v1).
    if let Some(intent) = gk_shot_save(state, side, &geo, &profile) {
        return GkChoice {
            scenario: GkScenario::Shot,
            intent,
            target: p.pos,
            commit: None,
        };
    }

    // Commitment: a committed scenario holds until its window ends or it is invalidated.
    let mut just_aborted = false;
    if prev.scenario.is_committed() && prev.commit_ticks > 0 {
        let kept = match prev.scenario {
            GkScenario::OneVOne => gk_one_v_one_carrier(state, side, &geo, true).map(|c| {
                let (intent, target) = gk_one_v_one_intent(p, &geo, &profile, c);
                GkChoice {
                    scenario: GkScenario::OneVOne,
                    intent,
                    target,
                    commit: None,
                }
            }),
            sc => gk_loose_ball_choice(state, side, &geo, &profile, prev, Some(sc), 0),
        };
        if let Some(mut choice) = kept {
            if choice.scenario != prev.scenario {
                // Hand-over between committed loose-ball scenarios keeps the remaining window.
                choice.commit = Some(prev.commit_ticks.saturating_sub(1).max(1));
            }
            return choice;
        }
        just_aborted = true;
    }

    let blocked = just_aborted || prev.reentry_block_ticks > 0;
    let extra_margin = if prev.reentry_block_ticks > 0 { 4 } else { 0 };

    // 3 / 4 / 6. Loose ball: cross claim or hold, collect in the box, sweep behind the line.
    if state.ball.owner.is_none() && !just_aborted {
        if let Some(choice) =
            gk_loose_ball_choice(state, side, &geo, &profile, prev, None, extra_margin)
        {
            return choice;
        }
    }

    // Live penalty (before the strike registers as a shot): stay on the line.
    let pen_live = state.restart_touch_state.is_some_and(|rt| {
        rt.kind == Some(RestartKind::PenaltyKick) && rt.taker_side == side.opponent()
    });
    if pen_live {
        let target = geo.point(Fp::from_milli(150), Fp::ZERO);
        return GkChoice {
            scenario: GkScenario::SetPiece,
            intent: gk_move_to(p, target, Fp::from_milli(1_500), Fp::from_milli(1_500)),
            target,
            commit: None,
        };
    }

    // 5. 1v1: attacker through on goal with no covering defender.
    if !blocked {
        if let Some(c) =
            gk_one_v_one_carrier(state, side, &geo, prev.scenario == GkScenario::OneVOne)
        {
            let (intent, target) = gk_one_v_one_intent(p, &geo, &profile, c);
            return GkChoice {
                scenario: GkScenario::OneVOne,
                intent,
                target,
                commit: (prev.scenario != GkScenario::OneVOne).then_some(24),
            };
        }
    }

    // 1 / 7b / 8. Set (or Support), with Recover when caught out of position.
    // Support persists through the team's own passes / loose balls it is first to (no Set<->Support churn).
    let own_outfield_ball = match state.ball.owner {
        Some((s, i)) => s == side && i != 0,
        None => {
            prev.scenario == GkScenario::Support
                && (state.ball.pass_in_flight.as_ref().is_some_and(|pf| pf.team == side)
                    || (state.ball.last_touch_team == Some(side)
                        && state.fastest_outfield_intercept_ticks(side)
                            <= state.fastest_outfield_intercept_ticks(side.opponent())))
        }
    };
    let (base_scenario, fresh_target) = if own_outfield_ball {
        (GkScenario::Support, gk_support_target(&geo, &profile, ball_xy))
    } else {
        (GkScenario::Set, gk_set_target(&geo, &profile, ball_xy))
    };
    // Target deadband: keep the previous set point while the new one is within 0.7m in depth / 0.4m across.
    let positional = |sc: GkScenario| matches!(sc, GkScenario::Set | GkScenario::Support);
    let target = if positional(prev.scenario)
        && (geo.depth(prev.target) - geo.depth(fresh_target)).abs() <= Fp::from_milli(700)
        && (prev.target.y - fresh_target.y).abs() <= Fp::from_milli(400)
    {
        prev.target
    } else {
        fresh_target
    };
    let dist = p.pos.dist(target);
    let recovering = dist > Fp::from_milli(2_500)
        || (prev.scenario == GkScenario::Recover && dist > Fp::from_milli(1_000));
    if recovering {
        return GkChoice {
            scenario: GkScenario::Recover,
            intent: gk_intent((target - p.pos).normalize_or_zero(), true, false),
            target,
            commit: None,
        };
    }
    // Feet set: never sprint outward in Set; jockey near the target and whenever the carrier is in range.
    let shooting_range = Fp::from_milli(if geo.is_5v5 { 20_000 } else { 25_000 });
    let threat = state.ball.owner.is_some_and(|(s, _)| s == side.opponent())
        && ball_xy.dist(geo.goal_center()) <= shooting_range;
    let outward = geo.depth(target) > geo.depth(p.pos) + Fp::from_milli(300);
    let delta = target - p.pos;
    let intent = if dist < Fp::from_milli(160) {
        gk_intent(Vec2::ZERO, false, true)
    } else {
        gk_intent(
            delta.normalize_or_zero(),
            dist > Fp::from_milli(1_550) && !outward,
            dist < Fp::from_milli(1_250) || (threat && dist < Fp::from_milli(2_000)),
        )
    };
    GkChoice {
        scenario: base_scenario,
        intent,
        target,
        commit: None,
    }
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

/// Incumbent dwell of a support decision (pocket / candidate, support triangle): 18 ticks (0.3 s)
/// [community utility-AI practice; estimate].
const INCUMBENT_DWELL_TICKS: u32 = 18;
/// Incumbent score bonus of a support decision: a challenger must beat the incumbent by 1.15x
/// [community utility-AI inertia; estimate].
const INCUMBENT_BONUS_PM: i64 = 1_150;
/// A support-triangle challenger must be 2 m closer to the 9.5 m check distance [estimate].
const TRI_SWITCH_MARGIN_MM: i64 = 2_000;

/// Identifies one close teammate (`support_triangle_idx`) to check toward the ball carrier at an
/// open passing angle. Committed with hysteresis (round 4): the incumbent keeps the role for at
/// least [`INCUMBENT_DWELL_TICKS`] and then until a challenger is [`TRI_SWITCH_MARGIN_MM`] closer to
/// the 9.5 m target; evaluated once per tick per team (stamped like the team plan) so every
/// player processed this tick sees the same pick.
fn designated_support_triangle_idx(state: &MatchState, side: TeamSide) -> Option<usize> {
    let s_idx = side.index();
    let ball_xy = state.ball.pos.xy();
    let stamp = state.tick.wrapping_add(1);
    if let Ok(mems) = state.ai_memory.try_borrow() {
        let m = &mems[s_idx];
        if m.tri_eval == stamp {
            return (m.tri_idx != 0).then_some(m.tri_idx as usize);
        }
    }
    let carrier_idx = match state.ball.owner {
        Some((o_side, o_idx)) if o_side == side => Some(o_idx),
        _ => None,
    };
    let players = &state.players[s_idx];
    let cost_of = |idx: usize| -> Option<i64> {
        let p = players.get(idx)?;
        if idx == 0 || Some(idx) == carrier_idx || p.sent_off {
            return None;
        }
        let d_mm = p.pos.dist(ball_xy).to_milli();
        let role_bonus = if p.role == PlayerRole::Midfielder { -2_500 } else { 0 };
        Some((d_mm - 9_500).abs() + role_bonus)
    };
    let mut best: Option<(i64, usize)> = None;
    for idx in 1..players.len() {
        if let Some(cost) = cost_of(idx) {
            if best.is_none_or(|(bc, _)| cost < bc) {
                best = Some((cost, idx));
            }
        }
    }
    let Ok(mut mems) = state.ai_memory.try_borrow_mut() else {
        return best.map(|(_, i)| i);
    };
    let m = &mut mems[s_idx];
    let inc = m.tri_idx as usize;
    let pick = match (cost_of(inc), best) {
        (Some(ic), Some((bc, bi))) => {
            let held = stamp.wrapping_sub(m.tri_since);
            if bi != inc && held >= INCUMBENT_DWELL_TICKS && bc + TRI_SWITCH_MARGIN_MM < ic {
                Some(bi)
            } else {
                Some(inc)
            }
        }
        (_, b) => b.map(|(_, i)| i),
    };
    let pick_u8 = pick.map_or(0, |i| i as u8);
    if pick_u8 != m.tri_idx {
        m.tri_idx = pick_u8;
        m.tri_since = stamp;
    }
    m.tri_eval = stamp;
    pick
}

/// `passer_ready` Schmitt trigger (round 4): the carrier counts as facing forward when
/// `facing.x * attack_sign` rises above `-0.10` and stops when it falls below `-0.30`, with at
/// least 12 ticks between toggles, so the run push and the runners' move class do not toggle while
/// the carrier turns [estimate]. A new carrier starts from the old single threshold (`-0.15`).
/// With no carrier of ours (pass in flight, loose ball) it is `true`, as before. Evaluated once
/// per tick per team (stamped).
const PASSER_READY_ON_PM: i64 = -100;
const PASSER_READY_OFF_PM: i64 = -300;
const PASSER_READY_HOLD_TICKS: u32 = 12;
fn passer_ready_schmitt(state: &MatchState, side: TeamSide) -> bool {
    let Some((_, ci)) = state.ball.owner.filter(|&(os, _)| os == side) else {
        return true;
    };
    let attack_sign = side.attack_sign(state.second_half_swapped());
    let carrier = &state.players[side.index()][ci];
    let f = Vec2::from_bam(carrier.facing_bam, Fp::ONE).x.mul(Fp::from_int(attack_sign as i32));
    let stamp = state.tick.wrapping_add(1);
    let Ok(mut mems) = state.ai_memory.try_borrow_mut() else {
        return f > Fp::from_milli(-150);
    };
    let m = &mut mems[side.index()];
    if m.pr_eval == stamp {
        return m.pr_on;
    }
    if m.pr_carrier != ci as u8 || m.pr_eval.wrapping_add(1) != stamp {
        m.pr_on = f > Fp::from_milli(-150);
        m.pr_since = stamp;
        m.pr_carrier = ci as u8;
    } else if stamp.wrapping_sub(m.pr_since) >= PASSER_READY_HOLD_TICKS {
        if m.pr_on && f < Fp::from_milli(PASSER_READY_OFF_PM) {
            m.pr_on = false;
            m.pr_since = stamp;
        } else if !m.pr_on && f > Fp::from_milli(PASSER_READY_ON_PM) {
            m.pr_on = true;
            m.pr_since = stamp;
        }
    }
    m.pr_eval = stamp;
    m.pr_on
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
            note_intent_target(state, side, idx, intercept_pt, false); // target memory (no snap on return)
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
        note_intent_target(state, side, idx, anchor, false); // target memory (no snap on return)
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

    // 2b. FIFA 22 / EA FC 27 Teammate Contain (`RB / R1` held by human on defense):
    //     Conservative `1.48m` contain gap in Tactical mode so user must switch & time manual Stand Tackle!
    //     Round 4, area T: the contain mate is registered in the team plan as the second engager
    //     (excluded from presser / cover / mark election, AI presser demoted to cover), his target
    //     is low-passed through the plan's target memory and his move class is 1.
    if state.teammate_contain_idx[side.index()] == Some(idx)
        && state.ball.owner.map(|(s, _)| s) != Some(side)
    {
        update_team_plan(state, side);
        let own_goal = Vec2::new(Fp::from_milli(-attack_sign * half_len.to_milli()), Fp::ZERO);
        let pred_ball = predict_ball_xy_at_ticks(state, 5);
        let is_legacy = state.control_settings[side.index()].defending_mode
            == crate::input::DefendingMode::Legacy;
        let hold_gap = Fp::from_milli(if is_legacy { 650 } else { 1_480 });
        let raw = pred_ball + (own_goal - pred_ball).normalize_or_zero().scale(hold_gap);
        let contain_pt = {
            let mut mems = state.ai_memory.borrow_mut();
            let m = &mut mems[side.index()];
            let t = match (m.target.get(idx).copied(), m.target_tick.get(idx).copied()) {
                (Some(prev), Some(tt)) if tt != 0 && tt.wrapping_add(1) == state.tick => {
                    prev.lerp(raw, Fp::from_milli(650))
                }
                _ => raw,
            };
            if idx < m.target.len() {
                m.target[idx] = t;
                m.target_tick[idx] = state.tick;
            }
            if let Some(c) = m.move_class.get_mut(idx) {
                *c = 1;
            }
            t
        };
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

    // 3. Loose ball: exactly ONE elected chaser per team (exclusivity layer, `elect_chaser`). Everyone
    //    else goes to his tactical slot, yielding around the chaser's path.
    if state.ball.owner.is_none() {
        update_team_plan(state, side);
        if elected_chaser(state, side) == Some(idx) {
            let ci = chase_intent(state, side, idx);
            let chase_pt = state.ai_memory.borrow()[side.index()].chaser_pt;
            note_intent_target(state, side, idx, chase_pt, true); // target memory (no snap on return)
            return ci;
        }
    }

    // 4. Phase-aware formation shape, role behaviours, committed marking/pressing & smoothed targets.
    outfield_shape_intent(state, side, idx)
}

/// Checks if an opposing outfielder stands (or will step) directly in the shooting corridor between shooter and goal mouth.
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
    let check_len = (dist - Fp::from_milli(1_400)).max(Fp::from_milli(3_500)).min(dist);
    for opp in state.players[side.opponent().index()].iter().skip(1) {
        if opp.sent_off {
            continue;
        }
        let rel_now = opp.pos - shooter_pos;
        let along_now = rel_now.dot(dir);
        if along_now > Fp::from_milli(180) && along_now < check_len {
            let ball_t = along_now.div(Fp::from_milli(22_500)).min(Fp::from_milli(450));
            let pred_opp = opp.pos + opp.vel.scale(ball_t);
            let rel = pred_opp - shooter_pos;
            let along = rel.dot(dir).clamp(Fp::ZERO, dist);
            let perp = (rel - dir.scale(along)).len();
            let block_r = Fp::from_milli(640) + along.mul(Fp::from_milli(55));
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

/// Side-effect-free variant of [`generate_cpu_input`] for callers OUTSIDE `MatchSimulation::tick`
/// (auto-pilot of a human side, HUD suggestions, tools). `generate_cpu_input` runs the team plan and
/// the shape intent for the controlled player, which write into `MatchState::ai_memory` (hashed
/// state). Inside `tick` that is wanted, once per tick per CPU side; outside it the writes would
/// not be reproduced by a replay of the recorded inputs, so this wrapper snapshots and restores the
/// AI memory around the call and leaves `state_hash` unchanged.
pub fn suggest_cpu_input(state: &MatchState, side: TeamSide, difficulty: CpuDifficulty) -> InputFrame {
    let saved = state.ai_memory.borrow().clone();
    let frame = generate_cpu_input(state, side, difficulty);
    *state.ai_memory.borrow_mut() = saved;
    frame
}

/// Generates a deterministic `InputFrame` for a CPU-controlled team (`Easy`, `Normal`, `Hard`).
/// NOTE: writes the tick's team plan and the controlled player's smoothed shape target into
/// `state.ai_memory` (interior mutability; part of the hashed state). Call it exactly once per tick
/// per CPU side from `MatchSimulation::tick`; anything outside the tick must use
/// [`suggest_cpu_input`], or a replay of the recorded inputs will diverge.
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
                    RestartKind::PenaltyKick => 11,
                    RestartKind::CornerKick => {
                        if (button & BTN_PASS) != 0 {
                            7
                        } else {
                            12 + ((state.tick % 4) as u16)
                        }
                    }
                    RestartKind::GoalKick => 13,
                    RestartKind::ThrowIn => 9,
                    RestartKind::FreeKick => 11,
                };
                let hold_aim = if stored_aim.len() > Fp::from_milli(120) {
                    stored_aim.normalize_or_zero()
                } else {
                    Vec2::from_bam(taker.facing_bam, Fp::ONE)
                };
                if ticks >= charge_units_to_ticks(target_charge, button) {
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
                            if d < Fp::from_milli(2_500) || d > Fp::from_milli(21_000) {
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
                        // EA SPORTS FC 25 *5v5 Rush*: all 5v5 Free Kicks (and any IFAB indirect FK) MUST be passed to a teammate!
                        let is_indirect_fk =
                            is_5v5 || state.restart_touch_state.is_some_and(|rt| rt.indirect);
                        let goal_vec = Vec2::new(opp_goal_x, Fp::ZERO) - spot;
                        if !is_indirect_fk && goal_vec.len() <= Fp::from_milli(25_000) {
                            (BTN_SHOOT | BTN_FINESSE, goal_vec.normalize_or_zero())
                        } else {
                            let mut best_dir =
                                Vec2::new(Fp::from_int(attack_sign as i32), Fp::ZERO);
                            let mut best_sc = Fp::from_int(-999_999);
                            for (m_idx, mate) in state.players[side_idx].iter().enumerate().skip(1) {
                                if m_idx == taker_idx || mate.sent_off {
                                    continue;
                                }
                                let d = spot.dist(mate.pos);
                                if d < Fp::from_milli(2_200) || d > Fp::from_milli(24_000) {
                                    continue;
                                }
                                let mut opp_clr = Fp::from_milli(9_000);
                                for opp in &state.players[side.opponent().index()] {
                                    if !opp.sent_off {
                                        opp_clr = opp_clr.min(opp.pos.dist(mate.pos));
                                    }
                                }
                                let sc = opp_clr.mul(Fp::from_int(2))
                                    + (mate.pos.x - spot.x).mul(Fp::from_milli(attack_sign * 220))
                                    - d.mul(Fp::from_milli(120));
                                if sc > best_sc {
                                    best_sc = sc;
                                    best_dir = (mate.pos - spot).normalize_or_zero();
                                }
                            }
                            (BTN_PASS, best_dir)
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
            let lateral = if evade_side(state, side, ctrl_idx, score_side(perp_a), score_side(perp_b)) {
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
            let base = if planned >= 4 { planned } else { 11 };
            if point_blank_press { base.min(7) } else { base }
        } else {
            let base = if planned >= 4 { planned } else { 9 };
            if point_blank_press { base.min(6) } else { base }
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
        if ticks >= charge_units_to_ticks(target_charge, button) {
            return vec_and_buttons_to_input(glide_dir, hold_aim, 0);
        } else {
            return vec_and_buttons_to_input(glide_dir, hold_aim, hold_btns);
        }
    }

    // --- ON-BALL CPU DECISION BRAIN (FIFA Space-Based Attacking: Angle-Gated Finishing + Byline Cutback + TTI Passing) ---
    if state.ball.owner == Some((side, ctrl_idx)) {
        let opp_gk = &state.players[side.opponent().index()][0];
        let opp_gk_y = opp_gk.pos.y;
        let primary_corner_y = if opp_gk_y >= Fp::ZERO {
            -goal_half_w.mul(Fp::from_milli(780))
        } else {
            goal_half_w.mul(Fp::from_milli(780))
        };
        let alt_corner_y = -primary_corner_y;
        let primary_target = Vec2::new(opp_goal_x, primary_corner_y);
        let alt_target = Vec2::new(opp_goal_x, alt_corner_y);
        let goal_center = Vec2::new(opp_goal_x, Fp::ZERO);
        let dist_to_goal = me.pos.dist(goal_center).max(Fp::ONE);
        let dx_goal = (opp_goal_x - me.pos.x).abs();
        let cos_goal_angle = dx_goal.div(dist_to_goal);
        let box_finish_dist = match difficulty {
            CpuDifficulty::Easy => Fp::from_milli(if is_5v5 { 12_500 } else { 16_500 }),
            CpuDifficulty::Normal => Fp::from_milli(if is_5v5 { 14_200 } else { 19_000 }),
            CpuDifficulty::Hard => Fp::from_milli(if is_5v5 { 15_000 } else { 20_500 }),
        };
        let long_shot_ok = me.attributes.shooting >= 80
            && nearest_opp_dist >= Fp::from_milli(2_500)
            && dist_to_goal <= Fp::from_milli(if is_5v5 { 18_200 } else { 24_500 });

        let urgent_press = difficulty != CpuDifficulty::Easy
            && nearest_opp_dist < Fp::from_milli(1_850)
            && cadence_tick.is_multiple_of(3);
        let on_decision_beat = cadence_tick.is_multiple_of(decision_cadence) || urgent_press;

        // 1. Strict Angle-Gated Dual-Corner Finishing: NEVER shoot from `79°` dead byline angles outside the posts!
        let central_six_yard =
            dist_to_goal <= Fp::from_milli(5_800) && me.pos.y.abs() <= goal_half_w + Fp::from_milli(700);
        let good_shot_angle = cos_goal_angle >= Fp::from_milli(440)
            || (central_six_yard && cos_goal_angle >= Fp::from_milli(350));
        let chosen_shot_target = if !is_shot_corridor_blocked(state, side, me.pos, primary_target) {
            Some(primary_target)
        } else if (opp_gk_y - alt_corner_y).abs() >= Fp::from_milli(1_450)
            && !is_shot_corridor_blocked(state, side, me.pos, alt_target)
        {
            Some(alt_target)
        } else {
            None
        };
        if on_decision_beat
            && (dist_to_goal <= box_finish_dist || long_shot_ok)
            && good_shot_angle
            && chosen_shot_target.is_some()
        {
            let shot_target = chosen_shot_target.unwrap_or(primary_target);
            let aim = (shot_target - me.pos).normalize_or_zero();
            let gk_off_line = (opp_gk.pos.x - opp_goal_x).abs();
            let shot_btns = if difficulty == CpuDifficulty::Hard
                && gk_off_line > Fp::from_milli(if is_5v5 { 4_400 } else { 5_800 })
                && dist_to_goal > Fp::from_milli(7_500)
                && nearest_opp_dist > Fp::from_milli(1_800)
            {
                BTN_SHOOT | BTN_LOB
            } else if difficulty != CpuDifficulty::Easy
                && me.pos.y.abs() > Fp::from_milli(2_200)
                && dist_to_goal >= Fp::from_milli(7_800)
            {
                BTN_SHOOT | BTN_FINESSE
            } else {
                BTN_SHOOT
            };
            return vec_and_buttons_to_input(aim, aim, shot_btns);
        }

        // 2. Byline Cutback Pass & Wing Crossing Zone:
        //    When wide near the endline/box (`!good_shot_angle`), look FIRST for a crisp ground cutback to a central box teammate!
        let prog_x = me.pos.x.mul(Fp::from_int(attack_sign as i32));
        let in_byline_cutback_zone = prog_x > half_len - Fp::from_milli(if is_5v5 { 11_500 } else { 15_500 })
            && me.pos.y.abs() > goal_half_w + Fp::from_milli(700);
        if on_decision_beat && in_byline_cutback_zone && difficulty != CpuDifficulty::Easy {
            let mut best_cutback: Option<(Vec2, Fp)> = None;
            for (m_idx, mate) in state.players[side_idx].iter().enumerate().skip(1) {
                if m_idx == ctrl_idx || mate.sent_off {
                    continue;
                }
                let mate_dx_g = (opp_goal_x - mate.pos.x).abs();
                if mate_dx_g > Fp::from_milli(if is_5v5 { 14_000 } else { 18_500 })
                    || mate.pos.y.abs() > state.format.penalty_box_half_width()
                {
                    continue;
                }
                let d_mate = me.pos.dist(mate.pos);
                if d_mate < Fp::from_milli(2_400) || d_mate > Fp::from_milli(20_000) {
                    continue;
                }
                let mut opp_clr = Fp::from_milli(10_000);
                for opp in state.players[side.opponent().index()].iter().skip(1) {
                    if !opp.sent_off {
                        opp_clr = opp_clr.min(opp.pos.dist(mate.pos));
                    }
                }
                if opp_clr >= Fp::from_milli(2_100) {
                    let sc = opp_clr.mul(Fp::from_int(2)) - mate.pos.y.abs().mul(Fp::from_milli(350));
                    if best_cutback.is_none_or(|(_, bs)| sc > bs) {
                        let target_pt = mate.pos + mate.vel.scale(Fp::from_milli(180));
                        best_cutback = Some(((target_pt - me.pos).normalize_or_zero(), sc));
                    }
                }
            }
            if let Some((cb_dir, _)) = best_cutback {
                let cb_btn = if difficulty == CpuDifficulty::Hard {
                    BTN_PASS | BTN_FINESSE
                } else {
                    BTN_PASS
                };
                return vec_and_buttons_to_input(cb_dir, cb_dir, cb_btn);
            }
        }

        let in_wing_cross_zone = prog_x > half_len.mul(Fp::from_milli(440))
            && me.pos.y.abs() > state.format.penalty_box_half_width().mul(Fp::from_milli(700));
        if on_decision_beat && in_wing_cross_zone && difficulty != CpuDifficulty::Easy {
            let box_spot = Vec2::new(
                Fp::from_milli(attack_sign * (half_len.to_milli() - if is_5v5 { 6_200 } else { 9_600 })),
                Fp::ZERO,
            );
            let mut has_box_runner = false;
            for (m_idx, mate) in state.players[side_idx].iter().enumerate().skip(1) {
                if m_idx != ctrl_idx
                    && !mate.sent_off
                    && (mate.pos.x - opp_goal_x).abs() <= Fp::from_milli(if is_5v5 { 13_500 } else { 20_000 })
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

        let goal_drive = (goal_center - me.pos).normalize_or_zero();
        let drive_corridor = evade_stride(goal_drive);
        let free_space_ahead = measure_free_space_ahead(state, side, me.pos, drive_corridor);

        // 3. Time-To-Intercept (`TTI`) & Rush Offside-Aware Pass / Through-Ball / Switch Evaluation!
        if on_decision_beat {
            let in_own_half = prog_x < Fp::ZERO;
            let offside_zone_mm = if is_5v5 { half_len.to_milli() / 3 } else { 0 };
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

            let mut best_pass: Option<(Vec2, Fp, u16, usize)> = None;
            // Committed runners (round 4): evaluated at their post-release look-ahead, so the pass
            // choice and the run commit together; selecting one releases him.
            let (run_roles, run_ends) = {
                let mems = state.ai_memory.borrow();
                (mems[side_idx].run_role.clone(), mems[side_idx].run_end.clone())
            };
            // Explicit through ball to a committed runner: safe by the runner's own control race
            // (he is moving at the release, the generic margin assumes a standing receiver), and
            // a killer ball when the carrier faces forward and is not tight-pressed.
            let runner_pass_ok = passer_ready_schmitt(state, side) && nearest_opp_dist >= Fp::from_milli(1_950);
            let opp_defs = opp_movers(state, side);

            for (m_idx, mate) in state.players[side_idx].iter().enumerate() {
                if m_idx == ctrl_idx || mate.sent_off {
                    continue;
                }
                let dist_feet = me.pos.dist(mate.pos);
                if m_idx == 0 {
                    if !in_own_half
                        || nearest_opp_dist > Fp::from_milli(1_850)
                        || dist_feet < Fp::from_milli(5_500)
                        || dist_feet > Fp::from_milli(22_000)
                    {
                        continue;
                    }
                } else if dist_feet < Fp::from_milli(2_600)
                    || dist_feet > Fp::from_milli(if is_5v5 { 31_000 } else { 36_000 })
                {
                    continue;
                }

                // Never pass to an offside teammate (respecting 5v5 Rush `1/3` offside line)!
                let mate_proj = mate.pos.x.to_milli() * attack_sign;
                let my_proj = me.pos.x.to_milli() * attack_sign;
                if state.rules.offsides_enabled
                    && m_idx != 0
                    && mate_proj > offside_zone_mm
                    && mate_proj > my_proj + 100
                    && mate_proj > onside_limit_proj + 100
                {
                    continue;
                }

                let feet_target = mate.pos + mate.vel.scale(Fp::from_milli(200));
                let lead_step = Fp::from_milli(if is_5v5 { 3_600 } else { 5_200 });
                let committed_end = matches!(run_roles.get(m_idx).copied(), Some(RUN_PENETRATE | RUN_CALLED))
                    .then(|| run_ends.get(m_idx).copied())
                    .flatten();
                let run_dir = if let Some(end) = committed_end {
                    (end - mate.pos).normalize_or_zero()
                } else if mate.vel.len() > Fp::from_milli(1_100) {
                    mate.vel.normalize_or_zero()
                } else {
                    Vec2::new(Fp::from_int(attack_sign as i32), Fp::ZERO)
                };
                let raw_through = match committed_end {
                    Some(end) => runner_release_point(mate, end),
                    None => mate.pos + run_dir.scale(lead_step),
                };
                let through_in_bounds = raw_through.x.abs() < half_len - Fp::from_milli(2_600)
                    && raw_through.y.abs() < half_wid - Fp::from_milli(1_600);
                let through_onside = !state.rules.offsides_enabled
                    || mate_proj <= offside_zone_mm
                    || mate_proj <= my_proj + 100
                    || mate_proj <= onside_limit_proj + 100;
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
                        && d_cand >= Fp::from_milli(11_000)
                        && m_idx != 0;

                    let avg_ball_spd_mm_s: i64 = if will_drive { 18_400 } else { 15_400 };
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
                    let runner_cand = is_through_cand && m_idx != 0 && committed_end.is_some();
                    let runner_safe = runner_cand && !point_blank_blocked && {
                        let dir = (cand_pt - mate.pos).normalize_or_zero();
                        let runner = crate::space::Mover::new(mate.pos, dir.scale(RUN_FORECAST_SPEED));
                        let ball_t = Fp::from_milli(d_cand.to_milli() * 1_000 / avg_ball_spd_mm_s);
                        crate::space::control(cand_pt, ball_t, &[runner], &opp_defs) > RUNNER_PASS_CONTROL_MIN
                    };
                    let ground_safe = if runner_cand {
                        runner_safe
                    } else {
                        !point_blank_blocked && worst_margin_ticks >= req_safety
                    };
                    let can_loft_switch = !ground_safe
                        && !is_through_cand
                        // A committed runner is served on the ground into his run, not by a switch.
                        && committed_end.is_none()
                        && m_idx != 0
                        && difficulty != CpuDifficulty::Easy
                        && d_cand >= Fp::from_milli(if is_5v5 { 21_000 } else { 26_000 })
                        && (cand_pt.y - me.pos.y).abs() >= Fp::from_milli(12_000)
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

                    let runner_bonus = if runner_safe && runner_pass_ok { RUNNER_KILLER_BONUS } else { Fp::ZERO };
                    let score = forward_gain.mul(Fp::from_milli(880))
                        + x_threat_gain
                        + space_bonus
                        + tti_bonus
                        + through_bonus
                        + runner_bonus
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

                    if best_pass.is_none_or(|(_, bs, _, _)| score > bs) {
                        best_pass = Some((lane_dir, score, btn, m_idx));
                    }
                }
            }

            let tight_pressure = nearest_opp_dist < Fp::from_milli(1_950);
            let moderate_pressure = nearest_opp_dist < Fp::from_milli(3_300);
            let open_carry_lane =
                free_space_ahead >= Fp::from_milli(7_400) && nearest_opp_dist >= Fp::from_milli(3_400);

            if let Some((aim_dir, score, btn, target_idx)) = best_pass {
                let killer_ball = score > Fp::from_milli(if open_carry_lane { 8_200 } else { 4_600 });
                let pressured_progression = moderate_pressure && score > Fp::from_milli(2_200);
                let safe_recycle = tight_pressure && score > Fp::from_milli(-1_800);
                if killer_ball || pressured_progression || safe_recycle {
                    if matches!(run_roles.get(target_idx).copied(), Some(RUN_PENETRATE | RUN_CALLED)) {
                        // The decision that selects a committed runner releases him.
                        let mut mems = state.ai_memory.borrow_mut();
                        mems[side_idx].run_release = target_idx as u8;
                        mems[side_idx].run_release_tick = state.tick.wrapping_add(1);
                    }
                    return vec_and_buttons_to_input(aim_dir, aim_dir, btn);
                }
            }

            // Deep defensive-third safety clearance ONLY when heavily swarmed with zero safe pass outlets!
            let clear_line = if is_5v5 {
                -half_len.div(Fp::from_int(3))
            } else {
                -half_len.mul(Fp::from_milli(550))
            };
            if prog_x < clear_line && nearest_opp_dist < Fp::from_milli(1_550) {
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

        // 4. Progressive 360° Ball Carrying (single unchained skill feint when `skill_chain_count == 0`, zero double-roulette spam!)
        let try_skill = difficulty == CpuDifficulty::Hard
            && me.skill_chain_count == 0
            && me.attributes.dribbling >= 82
            && nearest_opp_dist >= Fp::from_milli(1_300)
            && nearest_opp_dist <= Fp::from_milli(1_900)
            && nearest_opp_vec.normalize_or_zero().dot(goal_drive) > Fp::from_milli(580)
            && cadence_tick.is_multiple_of(43);
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

    // --- OFF-BALL CPU PLAYER ---
    // Exclusivity layer: the CPU-controlled player is an ordinary member of the team plan. He chases a
    // loose ball only as the elected chaser and closes a carrier only as the committed presser (or when
    // nobody is); otherwise he follows his tactical slot like the AI teammates (no third engager).
    update_team_plan(state, side);
    let (plan_presser, plan_cover) = {
        let mems = state.ai_memory.borrow();
        let m = &mems[side_idx];
        (m.presser, m.cover)
    };
    let opp_owns_ball = state.ball.owner.is_some_and(|(os, _)| os != side);
    let engage = match state.ball.owner {
        None => elected_chaser(state, side) == Some(ctrl_idx),
        Some((os, _)) if os == side => false,
        Some(_) => plan_presser as usize == ctrl_idx || (plan_presser == AI_NONE && plan_cover as usize != ctrl_idx),
    };
    let (move_dir, mut buttons) = if engage {
        cpu_engage_move(state, side, difficulty, me, ctrl_idx, is_5v5, own_goal_x)
    } else {
        let intent = outfield_shape_intent(state, side, ctrl_idx);
        let mut b = 0_u16;
        if intent.sprint {
            b |= BTN_SPRINT;
        }
        if intent.jockey && opp_owns_ball {
            b |= BTN_JOCKEY;
        }
        (intent.move_dir, b)
    };
    let my_dist = me.pos.dist(state.ball.pos.xy());

    // Tackle timing: difficulty sets the base cadence/reach, the defender's `tackling` sets how
    // often he commits (confident tacklers pick the moment sooner) and how rash he is (a low
    // tackler also lunges at a carrier who is turned away from him).
    let (base_cadence, base_reach_mm): (i64, i64) = match difficulty {
        CpuDifficulty::Easy => (30, 1_020),
        CpuDifficulty::Normal => (20, 1_100),
        CpuDifficulty::Hard => (12, 1_180),
    };
    let tackle_conf = player_style(me).tackle_confidence;
    let tackle_cadence = (base_cadence * (1_400 - tackle_conf) / 540).clamp(6, 48) as u32;
    let tackle_reach_mm = base_reach_mm + (tackle_conf - 860) / 5;
    let rash_pm = 480 + (600 - tackle_conf).max(0);
    if let Some((o_side, o_idx)) = state.ball.owner {
        if o_side != side && state.ball.possession_grace_ticks == 0 {
            let carrier = &state.players[o_side.index()][o_idx];
            let c_fwd = Vec2::from_bam(carrier.facing_bam, Fp::ONE);
            let def_to_c = (carrier.pos - me.pos).normalize_or_zero();
            let front_or_side = def_to_c.dot(c_fwd) < Fp::from_milli(rash_pm);
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

/// The CPU-controlled player's own chase / contain movement (he is the elected chaser or the
/// committed presser): arrive at the goal-side contain point, sprint/jockey with hysteresis.
fn cpu_engage_move(
    state: &MatchState,
    side: TeamSide,
    difficulty: CpuDifficulty,
    me: &PlayerState,
    ctrl_idx: usize,
    is_5v5: bool,
    own_goal_x: Fp,
) -> (Vec2, u16) {
    let ball_xy = state.ball.pos.xy();
    let my_dist = me.pos.dist(ball_xy);
    let (mut my_intercept, _) = best_ball_intercept_for_player(state, side, ctrl_idx);
    if state.ball.owner.is_none() {
        // Continuous chase target (no waypoint snapping); attack the ball itself once on top of it.
        my_intercept = if my_dist <= Fp::from_milli(1_500) && state.ball.pos.z <= Fp::from_milli(900) {
            predict_ball_xy_at_ticks(state, 3)
        } else {
            smooth_ball_intercept(me, &sample_ball_3d_waypoints(state), is_5v5)
        };
    }

    let goal_side_pt = if state.ball.owner.is_some() {
        let hold_dist = match difficulty {
            CpuDifficulty::Easy => Fp::from_milli(1_350),
            CpuDifficulty::Normal => Fp::from_milli(1_020),
            CpuDifficulty::Hard => Fp::from_milli(780),
        };
        let to_own_goal = (Vec2::new(own_goal_x, Fp::ZERO) - my_intercept)
            .normalize_or_zero()
            .scale(hold_dist);
        my_intercept + to_own_goal
    } else {
        my_intercept
    };

    // Arrive at the contain point instead of bang-bang stepping across it every tick.
    let to_pt = goal_side_pt - me.pos;
    let d_pt = to_pt.len();
    let move_dir = if state.ball.owner.is_none() || d_pt > Fp::from_milli(900) {
        to_pt.normalize_or_zero()
    } else if d_pt <= Fp::from_milli(120) {
        Vec2::ZERO
    } else {
        to_pt.normalize_or_zero().scale(d_pt.div(Fp::from_milli(900)))
    };
    let mut buttons = 0_u16;
    let sprint_close_dist = match difficulty {
        CpuDifficulty::Easy => Fp::from_milli(3_200),
        CpuDifficulty::Normal => Fp::from_milli(2_550),
        CpuDifficulty::Hard => Fp::from_milli(1_950),
    };
    // Sprint/jockey hysteresis: a player already at sprint speed keeps sprinting until well inside.
    let at_sprint_speed = me.vel.len() > Fp::from_milli(6_000);
    if my_dist > sprint_close_dist
        || state.ball.owner.is_none()
        || (at_sprint_speed && my_dist > sprint_close_dist.mul(Fp::from_milli(700)))
    {
        buttons |= BTN_SPRINT;
    } else {
        buttons |= BTN_JOCKEY | BTN_CONTAIN;
    }
    (move_dir, buttons)
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
    let min_offside_zone = if state.format.players_per_side <= 6 {
        half_len.to_milli() / 3
    } else {
        0
    };
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
    let line_mm = second_last_mm.max(ball_proj_mm).max(min_offside_zone) - 350;
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

// =================================================================================================
// Phase-aware team shape, role behaviours, committed assignments and attribute-driven style.
// See `docs/team-shape-ai.md` for the shapes, derivations and constants.
// =================================================================================================

use crate::config::{formation_shape, ShapeSlot, SideController, SlotRole};
use crate::state::{
    TeamAiMemory, AI_NONE, AI_PHASE_ATTACK, AI_PHASE_COUNTER_ATTACK, AI_PHASE_COUNTER_PRESS,
    AI_PHASE_DEFEND, AI_PHASE_RETREAT,
};

const FLAG_SPRINT: u8 = 1;
/// Defensive line moves 0.55 m per metre of ball progress (block slides, does not lurch).
const DEF_LINE_RATE_PM: i64 = 550;
/// Rest-defence line moves 0.6 m per metre of ball progress when attacking.
const ATT_LINE_RATE_PM: i64 = 600;
const FLAG_JOCKEY: u8 = 2;

/// Team playing style derived from squad attributes and the formation name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TeamStyle {
    /// Higher rest-defence line, widest attacking shape, medium counter-press.
    Possession,
    /// Highest defensive block, most compact, longest counter-press.
    Press,
    /// Deepest block, quick retreat on loss, long forward-run window after a regain.
    Counter,
}

/// Per-player behavioural style (`0..=1000` each) derived from attributes and current stamina.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlayerStyle {
    /// `positioning`: how precisely and how early the player reaches his anchor; line judgement.
    pub discipline: i64,
    /// `tackling + strength - 2 * positioning`: aggressive presser (high) vs disciplined holder (low).
    pub aggression: i64,
    /// `pace + acceleration`: quality of recovery runs.
    pub recovery: i64,
    /// `stamina` attribute scaled by current stamina: pressing intensity, decays during a match.
    pub work_rate: i64,
    /// `tackling` (+ a little `positioning`): tackle timing confidence.
    pub tackle_confidence: i64,
    /// FIFA 11 defensive work rate on `0..=100` (`PlayerAttributes::work_rates`): whether the
    /// player is threatened on a turnover, how long he bursts and how deep his defensive slot sits.
    pub def_work_rate: i64,
}

pub fn player_style(p: &PlayerState) -> PlayerStyle {
    let a = p.attributes;
    let pos = a.positioning as i64;
    PlayerStyle {
        discipline: (pos * 10).clamp(0, 1_000),
        aggression: (500 + (a.tackling as i64 + a.strength as i64 - 2 * pos) * 12).clamp(0, 1_000),
        recovery: ((a.pace as i64 + a.acceleration as i64) * 5).clamp(0, 1_000),
        work_rate: ((a.stamina as i64 * 10) * (300 + (p.stamina_permille as i64) * 7 / 10) / 1_000)
            .clamp(0, 1_000),
        tackle_confidence: (a.tackling as i64 * 8 + pos * 2).clamp(0, 1_000),
        def_work_rate: a.work_rates(p.role).1 as i64,
    }
}

// -------------------------------------------------------------------------------------------------
// Round 4, area T: transition urgency, work-rate recovery runs and off-ball defending
// (`docs/transition-and-defending.md`; parameters from `docs/05-fifa-pes-off-ball-teammate-
// intelligence.md` Work areas 2 and 4, evidence label in each comment).
// -------------------------------------------------------------------------------------------------

/// Defensive work-rate bands on the 0..100 scale: Low <= 37 < Medium < 63 <= High (FIFA's Low 25 /
/// Medium 50 / High 75 centres) [official FIFA 11 attribute; band edges estimate].
const DWR_LOW_MAX: i64 = 37;
const DWR_HIGH_MIN: i64 = 63;
/// Reaction before the press-or-fall-back decision: Hard 6 / Normal 9 / Easy 12 ticks [estimate].
const CP_REACT_TICKS: [u16; 3] = [12, 9, 6];
/// Superiority radius around the ball for the counter-press decision (10 m; 5v5 7 m) [peer-reviewed
/// Bauer & Anzer 2021; 5v5 scaling estimate].
const CP_SUPERIORITY_MM: [i64; 2] = [10_000, 7_000];
/// Own outfielders behind the ball needed to counter-press: >= 4 (11v11), >= 2 (5v5)
/// [peer-reviewed; 5v5 estimate].
const CP_BEHIND_MIN: [usize; 2] = [4, 2];
/// Counter-press window 180 ticks x style (Press 1.2 / Possession 1.0 / Counter 0.6) x work rate,
/// floor 90. The report's 300 / floor 180 [peer-reviewed 5 s; official FIFA 19 7 s] was measured
/// worse here (11v11, 16 seeds, stamina fixed: behind-ball at 3 s / 6 s -2 pt, line_x_std +0.6 m):
/// the compressed counter-press block keeps non-pressers high for too long [sim-calibrated].
const CP_WINDOW_TICKS: i64 = 180;
const CP_WINDOW_FLOOR: i64 = 90;
/// Team work rate that maps to a 1.0 window factor (typical full-stamina squad) [estimate].
const CP_WORK_REF: i64 = 750;
/// A pass longer than this beats the counter-press and ends it (16 m) [peer-reviewed].
const CP_BEATEN_PASS_MM: i64 = 16_000;
/// RETREAT (the 4.5 m deeper block) hands over to DEFEND after 75 ticks. The report's "until shape,
/// cap 540-600 ticks" [peer-reviewed 9-10 s] kept the deeper block for most of the defending time
/// and was measured worse (line_x_std 6.72 vs 6.12 m, 11v11, 16 seeds); the recovery runs carry the
/// re-forming instead [sim-calibrated].
const RETREAT_TICKS: u16 = 75;
/// Stamina drain multiplier on pressing engagers while counter-pressing: x2.5 [official FIFA 21 2.2-3].
const PRESS_DRAIN_PM: i64 = 2_500;
/// Recovery trigger: the player is ahead of the ball and more than 30 ticks from his target
/// (replaces `2.5 m + recovery x 2.5 mm`) [estimate 30-45; audit].
const RECOVERY_TRIGGER_TICKS: u32 = 30;
/// ... or, whatever the distance, when the ball travels toward our goal faster than 2.5 m/s (a
/// counter: the deficit only grows); then also players not yet 3 m goal-side latch. Measured on
/// 11v11 (16 seeds): 4 m/s -> 2.5 m/s and the goal-side latch moved behind-ball at 3 s 67.5 -> 70.4%
/// and forwards staying high 1.68 -> 1.51 [estimate, sim-calibrated].
const COUNTER_BALL_MM_S: i64 = 2_500;
/// Recovery sprint ends 3 m goal-side of the ball [peer-reviewed shape]; ending it level with the
/// ball left forwards hovering across it as the counter moved on, so forwards use 3 m too [estimate].
const RECOVERY_EXIT_MM: i64 = 3_000;
/// Loose-ball stand-off for latched recovery runners within 25 m of the ball (the exclusivity
/// test's behavioural-chaser radius): jog at most 4.3 m/s, under its 4.5 m/s floor [estimate].
const LOOSE_STANDOFF_MM: i64 = 25_000;
const LOOSE_STANDOFF_MM_S: i64 = 4_300;
/// ... and only for a plausible second chaser: intercept within 40 ticks of the elected chaser.
const LOOSE_STANDOFF_TICKS: u32 = 40;
/// Recovery burst caps (ticks) by defensive work rate: Low 210 / Medium 240 / High 240, inside the
/// 180-240 (3-4 s, ~26 m) range [peer-reviewed Ju et al. 2023]; the low end left Low-DWR forwards
/// short of the ball [sim-calibrated].
const BURST_CAP_TICKS: [u16; 3] = [210, 240, 240];
/// After a capped burst a player still ahead of the ball may start a new one after 45 ticks in the
/// high-speed band (repeated-sprint bouts) [estimate].
const BURST_REST_TICKS: u16 = 45;
/// A Low-work-rate player is threatened only within this distance ahead of the ball [estimate].
const LOW_DWR_REACH_MM: i64 = 40_000;
/// High-speed band ceiling 7 m/s (5.5-7 m/s band) [peer-reviewed speed band].
const HIGH_BAND_MM_S: i64 = 7_000;
/// High-intensity running threshold 5.5 m/s (19.8 km/h) and the 60 s window of the fatigue
/// integrator; above the peak the burst cap is multiplied by 0.65 and the band by 0.8
/// [peer-reviewed Ju et al. 2022, 20-53% post-peak drop]. The peak is 300 m, not the real-football
/// ~70 m: sim outfielders run 2-4x more at high intensity (measured 100-290 m in 11v11, round 4),
/// so 70 m (or 200 m) would mark most players as fatigued all match [estimate, sim-calibrated].
const HI_SPEED_MM_S: i64 = 5_500;
const HI_WINDOW_TICKS: u32 = 3_600;
const HI_PEAK_MM: u32 = 300_000;
const FATIGUE_PM: i64 = 650;
const FATIGUE_BAND_PM: i64 = 800;
/// Forward holding line goal-side of the ball by work rate: Low 8 m / Medium 10 m / High 12 m
/// (5v5: 2 / 3.5 / 5 m), and the shallowest block depth (permille of the block) each may drop to:
/// Low 400 / Medium 300 / High 150 [community FIFA 23 zones; line about 10 m estimate; 11v11 depths
/// deepened from 4/7/10 m after the stamina fix, 16 seeds: forwards staying high 1.80 -> 1.68].
const FWD_HOLD_MM: [[i64; 3]; 2] = [[8_000, 10_000, 12_000], [2_000, 3_500, 5_000]];
const FWD_FLOOR_PM: [i64; 3] = [400, 300, 150];
/// Presser lateral shift toward the most dangerous lane (cover shadow): 0.75 m [estimate 0.5-1 m].
const SHADOW_SHIFT_MM: i64 = 750;
/// Zonal holders block the best lane when within 12 m of the ball, on a 3 m leash from the slot
/// [official eFootball two-tier rule; distances estimate].
const LANE_BLOCK_BALL_MM: i64 = 12_000;
const LANE_BLOCK_LEASH_MM: i64 = 3_000;
/// Mark arbiter: incumbent bonus 3 m, dwell 24 ticks [official FIFA 14; values estimate 20-28].
const MARK_INCUMBENT_MM: i64 = 3_000;
const MARK_DWELL_TICKS: u16 = 24;
/// Runner handover (Konami US8597094): cover within La = 3 m of the runner-to-goal line; intercept
/// line only when the carrier faces the marked man within theta_a = 25 deg (cos = 0.906)
/// [patent; La, theta_a estimate].
const COVER_LA_MM: i64 = 3_000;
const COS_THETA_A_PM: i64 = 906;
/// A runner in behind is tracked up to this distance from his marker [estimate].
const RUNNER_TRACK_MM: i64 = 15_000;
/// Shared line: back-liners lag-track one team value at 120 permille per tick [estimate].
const LINE_LAG_PM: i64 = 120;
/// Presser zone handover: released beyond 12 m lateral from his slot or 8 m beyond his line
/// [official FIFA 12 Barca rule; width estimate].
const ZONE_HANDOVER_LAT_MM: i64 = 12_000;
const ZONE_HANDOVER_DEPTH_MM: i64 = 8_000;
const ZONE_HANDOVER_HYST_MM: i64 = 2_000;
/// Press triggers [official FIFA 19 "pressure on heavy touch"]: ball more than 1.6 m off the
/// carrier (heavy touch), carrier facing his own goal, carrier within 5 m of the touchline; the
/// trigger holds 45 ticks [radius and hold estimate].
const HEAVY_TOUCH_MM: i64 = 1_600;
const TOUCHLINE_TRAP_MM: i64 = 5_000;
const PRESS_TRIGGER_HOLD: u16 = 45;
/// Second man: shadow 3.5 m behind-side of the presser, never inside 3 m of the carrier
/// [official FIFA 12 "cover off"; distance estimate 3-4 m].
const SECOND_MAN_MM: i64 = 3_500;
/// Non-engagers keep at least this gap from the carrier so at most two players engage (2.5 m is
/// the engager radius of `coord.engagers3`) [audit; estimate].
const NON_ENGAGER_GAP_MM: i64 = 2_900;

/// `Some(idx)` for a plan index field that names an outfielder (`AI_NONE` and `0` mean none).
#[inline]
const fn plan_idx(v: u8) -> Option<usize> {
    if v == AI_NONE || v == 0 { None } else { Some(v as usize) }
}

/// Work-rate band: 0 Low, 1 Medium, 2 High.
#[inline]
const fn dwr_band(dwr: i64) -> usize {
    if dwr <= DWR_LOW_MAX {
        0
    } else if dwr >= DWR_HIGH_MIN {
        2
    } else {
        1
    }
}

/// How a front-line slot (striker, pivo, winger) sits in the defensive block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FrontMode {
    /// Not a front slot, or attacking: the slot as the block places it.
    Plain,
    /// The rest offence: outlet up to 6 m beyond the ball, never past halfway (11v11).
    Outlet,
    /// Work-rate band of the holder: holding line goal-side of the ball and depth floor.
    Hold(usize),
}

/// Front mode of slot `k` from the plan's assignment (`slot_of`) and rest offence.
fn front_mode(ctx: &ShapeCtx, state: &MatchState, slot_of: &[u8], rest_offence: u8, k: usize) -> FrontMode {
    let Some(slot) = ctx.slots.get(k) else { return FrontMode::Plain };
    if !matches!(slot.role, SlotRole::Striker | SlotRole::Pivo | SlotRole::Winger) {
        return FrontMode::Plain;
    }
    let players = &state.players[ctx.side.index()];
    let holder = (1..players.len()).find(|&i| slot_of.get(i).copied() == Some(k as u8) && !players[i].sent_off);
    match holder {
        Some(h) if Some(h) == plan_idx(rest_offence) => FrontMode::Outlet,
        Some(h) => FrontMode::Hold(dwr_band(player_style(&players[h]).def_work_rate)),
        None => FrontMode::Hold(0),
    }
}

/// Stamina drain multiplier (permille) for `players[side][idx]` in the sim's sprint-drain path
/// (`sim.rs` `step_all_players`, "5v5 Rush Short-Term Stamina"): x2.5 for the presser and the two
/// lane-closers while the side counter-presses, 1.0 otherwise.
pub fn stamina_drain_mult_pm(state: &MatchState, side: TeamSide, idx: usize) -> i64 {
    let Ok(mems) = state.ai_memory.try_borrow() else { return 1_000 };
    let m = &mems[side.index()];
    if !plan_is_fresh(state, m) || m.phase != AI_PHASE_COUNTER_PRESS || idx == 0 {
        return 1_000;
    }
    let pressing = [m.presser, m.second_presser, m.third_presser, m.chaser]
        .iter()
        .any(|&v| plan_idx(v) == Some(idx));
    if pressing { PRESS_DRAIN_PM } else { 1_000 }
}

/// `true` when `players[side][idx]` is a back-liner currently tracking a marked runner in behind
/// (he must not be pulled out as the teammate-contain mate).
pub fn owns_runner(state: &MatchState, side: TeamSide, idx: usize) -> bool {
    let Ok(mems) = state.ai_memory.try_borrow() else { return false };
    let m = &mems[side.index()];
    let Some(o) = m.mark_of.get(idx).copied().and_then(plan_idx) else { return false };
    let ctx = ShapeCtx::new(state, side);
    let back = ctx.slot(m.slot_of.get(idx).copied().unwrap_or(AI_NONE)).is_some_and(|s| s.role.is_back_line());
    let opps = &state.players[side.opponent().index()];
    back && o < opps.len() && plan_is_fresh(state, m) && m.line_prog != 0 && ctx.prog(opps[o].pos.x) < m.line_prog
}

/// Derives the team style: averages of outfield `stamina+pace+tackling` (press), `passing+dribbling`
/// (possession) and forward `pace` (counter, `-6` handicap because pace runs high), plus a `+3`
/// bias from the formation name. Ties resolve Possession > Press > Counter.
pub fn team_style(state: &MatchState, side: TeamSide) -> TeamStyle {
    let mut press = 0_i64;
    let mut poss = 0_i64;
    let mut n = 0_i64;
    let mut fwd_pace = 0_i64;
    let mut nf = 0_i64;
    for p in state.players[side.index()].iter().skip(1).filter(|p| !p.sent_off) {
        let a = p.attributes;
        press += (a.stamina as i64 + a.pace as i64 + a.tackling as i64) * 1_000 / 3;
        poss += (a.passing as i64 + a.dribbling as i64) * 1_000 / 2;
        n += 1;
        if p.role == PlayerRole::Forward {
            fwd_pace += a.pace as i64 * 1_000;
            nf += 1;
        }
    }
    let n = n.max(1);
    let mut press = press / n;
    let mut poss = poss / n;
    let mut counter = if nf > 0 { fwd_pace / nf - 6_000 } else { 0 };
    match state.teams[side.index()].formation.as_str() {
        "4-3-3" | "2-2" | "4-2-4" | "3-4-3" => press += 3_000,
        "4-4-2" | "1-1-2" | "5-3-2" | "3-5-2" | "4-5-1" | "5-4-1" => counter += 3_000,
        "4-2-3-1" | "1-2-1" | "4-1-4-1" | "4-3-2-1" => poss += 3_000,
        _ => {}
    }
    if poss >= press && poss >= counter {
        TeamStyle::Possession
    } else if press >= counter {
        TeamStyle::Press
    } else {
        TeamStyle::Counter
    }
}

#[derive(Clone, Copy, Debug)]
struct DiffProfile {
    /// Per-tick low-pass weight of the shape's ball reference (reaction delay; higher = faster).
    react_pm: i64,
    /// Judgement-error scale (permille) on line stepping.
    err_pm: i64,
    /// Minimum ticks a presser / mark is held before a marginal improvement may replace it.
    hold: u16,
    /// Tick advantage a new presser needs to replace the committed one.
    margin: u32,
}

fn side_difficulty(state: &MatchState, side: TeamSide) -> CpuDifficulty {
    let ctrl = match side {
        TeamSide::Home => state.rules.home_controller,
        TeamSide::Away => state.rules.away_controller,
    };
    match ctrl {
        SideController::Cpu(d) => d,
        SideController::Human => CpuDifficulty::Normal,
    }
}

const fn diff_profile(d: CpuDifficulty) -> DiffProfile {
    match d {
        CpuDifficulty::Easy => DiffProfile { react_pm: 140, err_pm: 1_600, hold: 40, margin: 10 },
        CpuDifficulty::Normal => DiffProfile { react_pm: 230, err_pm: 1_000, hold: 28, margin: 8 },
        CpuDifficulty::Hard => DiffProfile { react_pm: 360, err_pm: 550, hold: 20, margin: 6 },
    }
}

/// Deterministic smooth noise in `[-1000, 1000]` for `(key_a, key_b)`, interpolated across
/// `period`-tick buckets so judgement error drifts instead of stepping.
fn smooth_noise_pm(tick: u32, period: u32, key_a: u32, key_b: u32) -> i64 {
    let hash = |bucket: u32| -> i64 {
        let mut z = ((bucket as u64) << 32) ^ ((key_a as u64) << 16) ^ (key_b as u64) ^ 0x9E37_79B9_7F4A_7C15;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^= z >> 31;
        (z % 2_001) as i64 - 1_000
    };
    let period = period.max(1);
    let b = tick / period;
    let frac = (tick % period) as i64;
    let n0 = hash(b);
    let n1 = hash(b + 1);
    n0 + ((n1 - n0) * frac) / period as i64
}

/// Geometry helper in the team's attacking frame: `prog` = millimetres from own goal line
/// towards the opponent goal; `yf` = lateral millimetres (negative = left when attacking).
struct ShapeCtx {
    side: TeamSide,
    sign: i64,
    half_len: i64,
    half_wid: i64,
    len: i64,
    is_5v5: bool,
    style: TeamStyle,
    slots: Vec<ShapeSlot>,
}

impl ShapeCtx {
    fn new(state: &MatchState, side: TeamSide) -> Self {
        let half_len = state.format.half_length().to_milli();
        Self {
            side,
            sign: side.attack_sign(state.second_half_swapped()),
            half_len,
            half_wid: state.format.half_width().to_milli(),
            len: half_len * 2,
            is_5v5: state.format.players_per_side <= 6,
            style: team_style(state, side),
            slots: formation_shape(
                &state.teams[side.index()].formation,
                state.format.players_per_side,
            ),
        }
    }

    #[inline]
    fn prog(&self, x: Fp) -> i64 {
        x.to_milli() * self.sign + self.half_len
    }

    #[inline]
    fn yf(&self, y: Fp) -> i64 {
        y.to_milli() * self.sign
    }

    #[inline]
    fn world(&self, prog: i64, yf: i64) -> Vec2 {
        Vec2::from_milli((prog - self.half_len) * self.sign, yf * self.sign)
    }

    fn slot(&self, k: u8) -> Option<ShapeSlot> {
        self.slots.get(k as usize).copied()
    }
}

#[derive(Clone, Copy, Debug)]
struct Block {
    line: i64,
    len: i64,
    width_pm: i64,
    shift_pm: i64,
    ref_yf: i64,
    ref_prog: i64,
    front_cap: i64,
}

fn avg_positioning_pm(state: &MatchState, side: TeamSide) -> i64 {
    let mut sum = 0_i64;
    let mut n = 0_i64;
    for p in state.players[side.index()].iter().skip(1).filter(|p| !p.sent_off) {
        sum += p.attributes.positioning as i64 * 10;
        n += 1;
    }
    (sum / n.max(1)).clamp(0, 1_000)
}

/// Defensive block: back-line height follows the (reaction-delayed) ball and the offside zone,
/// drops when the carrier is unpressured and facing our goal, and the block is compact.
fn defend_block(ctx: &ShapeCtx, state: &MatchState, ref_ball: Vec2, retreat: bool) -> Block {
    let ball_p = ctx.prog(ref_ball.x);
    // Line height = min_line + 0.55 x (ball progress - pivot), never closer than gap_min to the ball.
    let (mut len_def, pivot, gap_min, min_line, front_ahead, len_min, mut width, shift, drop) = if ctx.is_5v5 {
        (17_000, 10_000, 8_000, 4_500, 2_500, 9_000, 700, 320, 2_000)
    } else {
        (30_000, 16_000, 14_000, 7_000, 4_000, 26_000, 720, 380, 3_500)
    };
    let max_line = match (ctx.is_5v5, ctx.style) {
        (true, TeamStyle::Press) => 24_000,
        (true, TeamStyle::Possession) => 21_000,
        (true, TeamStyle::Counter) => 17_000,
        (false, TeamStyle::Press) => 48_000,
        (false, TeamStyle::Possession) => 42_000,
        (false, TeamStyle::Counter) => 34_000,
    };
    match ctx.style {
        TeamStyle::Press => {
            len_def -= if ctx.is_5v5 { 1_000 } else { 2_000 };
            width -= 30;
        }
        TeamStyle::Counter => len_def += if ctx.is_5v5 { 1_000 } else { 2_000 },
        TeamStyle::Possession => {}
    }
    let mut line = (min_line + (ball_p - pivot) * DEF_LINE_RATE_PM / 1_000).min(ball_p - gap_min);
    // Offside-line tie: only hold a line above an attacker where offside protects it.
    let off_zone = if ctx.is_5v5 { ctx.len / 3 } else { ctx.len / 2 };
    let deepest_opp = state.players[ctx.side.opponent().index()]
        .iter()
        .skip(1)
        .filter(|o| !o.sent_off)
        .map(|o| ctx.prog(o.pos.x))
        .min()
        .unwrap_or(ctx.len);
    line = line.min((deepest_opp - 800).max(off_zone));
    // Run-tracking policy by style, FC 25's line bands [official, unitless]: Counter = Deep (the line
    // drops with any runner in behind), Possession = Balanced (only when the runner's 1 s look-ahead
    // beats it by 2 m), Press = High (rarely: by 6 m). Runners: opponents running at our goal faster
    // than 3.5 m/s from within 2 m in front of the line to 10 m beyond it [trigger estimate].
    let beat_mm = match ctx.style {
        TeamStyle::Counter => 0,
        TeamStyle::Possession => 2_000,
        TeamStyle::Press => 6_000,
    };
    for o in state.players[ctx.side.opponent().index()].iter().skip(1).filter(|o| !o.sent_off) {
        let v_goal = -o.vel.x.to_milli() * ctx.sign;
        let o_p = ctx.prog(o.pos.x);
        if v_goal > 3_500 && o_p >= line - 2_000 && o_p <= line + 10_000 {
            let look = o_p - v_goal;
            if look < line - beat_mm {
                line = line.min(look + beat_mm);
            }
        }
    }
    // Drop off an unpressured carrier facing our goal (judged by team positioning).
    if let Some((os, oi)) = state.ball.owner {
        if os != ctx.side {
            let carrier = &state.players[os.index()][oi];
            let pressured = state.players[ctx.side.index()]
                .iter()
                .skip(1)
                .any(|d| !d.sent_off && d.pos.dist(carrier.pos) <= Fp::from_milli(2_800));
            let fwd = Vec2::from_bam(carrier.facing_bam, Fp::ONE);
            let facing_our_goal = -fwd.x.to_milli() * ctx.sign > 300;
            if !pressured && facing_our_goal {
                line -= drop * avg_positioning_pm(state, ctx.side) / 1_000;
            }
        }
    }
    if retreat {
        line -= if ctx.is_5v5 { 2_500 } else { 4_500 };
    }
    line = line.min(ball_p - 1_500).clamp(min_line, max_line);
    let len = len_def.min((ball_p + front_ahead - line).max(len_min));
    Block {
        line,
        len,
        width_pm: width,
        shift_pm: shift,
        ref_yf: ctx.yf(ref_ball.y),
        ref_prog: ball_p,
        front_cap: i64::MAX,
    }
}

/// Attacking block: rest-defence line behind the ball, front pinned to the opponent's offside line,
/// stretched long and wide. `counter_press` compresses it around the ball after a loss.
fn attack_block(ctx: &ShapeCtx, state: &MatchState, ref_ball: Vec2, counter_press: bool) -> Block {
    let ball_p = ctx.prog(ref_ball.x);
    // Rest-defence line = min_line + 0.6 x (ball progress - pivot), always `rest_gap` behind the ball.
    let (pivot, rest_gap, mut min_line, max_line, len_lo, len_hi, shift) = if ctx.is_5v5 {
        (21_000, 8_000, 9_000, ctx.len / 2 - 3_000, 14_000, 32_000, 180)
    } else {
        (30_000, 18_000, 16_000, ctx.len / 2 + 8_000, 34_000, 56_000, 240)
    };
    let mut width = 1_000;
    match ctx.style {
        TeamStyle::Possession => min_line += if ctx.is_5v5 { 1_000 } else { 2_000 },
        TeamStyle::Counter => {
            min_line -= if ctx.is_5v5 { 1_500 } else { 3_000 };
            width = 940;
        }
        TeamStyle::Press => {}
    }
    let onside_p = ctx.prog(compute_onside_limit_x(state, ctx.side));
    let front = (onside_p - 400).min(ctx.len - 3_000);
    if counter_press {
        let line = (ball_p - if ctx.is_5v5 { 9_000 } else { 17_000 })
            .clamp(min_line.min(ball_p - 2_000).max(3_000), max_line);
        return Block {
            line,
            len: if ctx.is_5v5 { 13_000 } else { 24_000 },
            width_pm: 780,
            shift_pm: 500,
            ref_yf: ctx.yf(ref_ball.y),
            ref_prog: ball_p,
            front_cap: front,
        };
    }
    // Rest defence (Forcher et al. 2023): the back line holds 5-10 m goal-side of the deepest
    // opposing forward (7 m; 5v5 4 m) and, with the ball in the final third, no further than 45 m
    // (5v5 18 m) from our goal line [peer-reviewed geometry; offsets estimate].
    let opp_fwd = state.players[ctx.side.opponent().index()]
        .iter()
        .skip(1)
        .filter(|o| !o.sent_off)
        .map(|o| ctx.prog(o.pos.x))
        .min()
        .unwrap_or(ctx.len);
    let rest_cap = (opp_fwd - if ctx.is_5v5 { 4_000 } else { 7_000 }).min(if ball_p > ctx.len * 2 / 3 {
        if ctx.is_5v5 { 18_000 } else { 45_000 }
    } else {
        i64::MAX
    });
    let line = (min_line + (ball_p - pivot) * ATT_LINE_RATE_PM / 1_000)
        .min(ball_p - rest_gap)
        .min(rest_cap)
        .clamp(min_line.min(ball_p - 3_000).max(3_000), max_line);
    let len = (front - line).clamp(len_lo, len_hi);
    Block {
        line,
        len,
        width_pm: width,
        shift_pm: shift,
        ref_yf: ctx.yf(ref_ball.y),
        ref_prog: ball_p,
        front_cap: front,
    }
}

/// World position of `slot` inside `blk`. Ball-side full-backs overlap, far-side full-backs tuck in
/// as rest defence; strikers are pinned to the last defender (offside line).
///
/// Defending front slots (striker, pivo, winger) follow `front`: the rest offence keeps the
/// 11v11 outlet (up to 6 m beyond the ball, never past halfway); every other holder drops to a
/// work-rate holding line goal-side of the ball, never deeper than his band's floor in the block.
fn slot_pos(ctx: &ShapeCtx, blk: &Block, slot: &ShapeSlot, attacking: bool, front: FrontMode) -> Vec2 {
    let (mut depth, mut y) = if attacking {
        (slot.attack_depth, slot.attack_y)
    } else {
        (slot.defend_depth, slot.defend_y)
    };
    if attacking && slot.role == SlotRole::FullBack && blk.ref_yf.abs() > 3_000 {
        if (y >= 0) == (blk.ref_yf >= 0) {
            depth += 150;
        } else {
            // Rest defence 3+1 (round 4, area T): the far-side full-back sits near the CB line
            // (depth 90, was 260); measured -0.4 m on line_x_std_m (11v11, 8 seeds).
            depth = 90;
            y = y * 65 / 100;
        }
    }
    let mut prog = blk.line + blk.len * depth / 1_000;
    if attacking && matches!(slot.role, SlotRole::Striker | SlotRole::Pivo) {
        prog = prog.min(blk.front_cap);
    }
    if !attacking {
        match front {
            FrontMode::Outlet if !ctx.is_5v5 => {
                // 11v11 rest offence stays as the outlet: up to 6 m beyond the ball, never past halfway.
                prog = prog.max((blk.ref_prog + 6_000).min(ctx.len / 2));
            }
            FrontMode::Hold(band) => {
                let hold = FWD_HOLD_MM[usize::from(ctx.is_5v5)][band];
                let floor = blk.line + blk.len * FWD_FLOOR_PM[band] / 1_000;
                prog = prog.min(blk.ref_prog - hold).max(floor.min(prog));
            }
            _ => {}
        }
    }
    let mut lim_y = ctx.half_wid - 1_500;
    if attacking && slot.role == SlotRole::CentreBack && !ctx.is_5v5 && blk.front_cap != i64::MAX {
        // Rest defence about 28 m wide [peer-reviewed Forcher et al. 2023]. Not in 5v5: the fixo
        // clamped to +-8 m sat within 4 m of the alas (merge fix; nearest teammate < 4 m in
        // possession 15.5% -> 14.8%, exclusivity bound 15%).
        lim_y = lim_y.min(14_000);
    }
    let yf = ((ctx.half_wid * y / 1_000) * blk.width_pm / 1_000 + blk.ref_yf * blk.shift_pm / 1_000)
        .clamp(-lim_y, lim_y);
    ctx.world(prog.clamp(1_500, ctx.len - 1_500), yf)
}

#[inline]
const fn is_defensive_phase(phase: u8) -> bool {
    matches!(phase, AI_PHASE_DEFEND | AI_PHASE_RETREAT | AI_PHASE_COUNTER_PRESS)
}

fn phase_block(ctx: &ShapeCtx, state: &MatchState, ref_ball: Vec2, phase: u8) -> (Block, bool) {
    match phase {
        AI_PHASE_DEFEND => (defend_block(ctx, state, ref_ball, false), false),
        AI_PHASE_RETREAT => (defend_block(ctx, state, ref_ball, true), false),
        AI_PHASE_COUNTER_PRESS => (attack_block(ctx, state, ref_ball, true), true),
        _ => (attack_block(ctx, state, ref_ball, false), true),
    }
}

fn dist_dm_sq(a: Vec2, b: Vec2) -> i64 {
    let d = a - b;
    let dx = d.x.to_milli() / 100;
    let dy = d.y.to_milli() / 100;
    dx * dx + dy * dy
}

/// Role-aware slot assignment with hysteresis. Keeps the 24-permutation optimum for 4 outfielders
/// (5v5) and 2-opt from the previous assignment otherwise; a new assignment is only adopted when the
/// previous one is invalid or, after `30` held ticks, when it is strictly cheaper including a ~3 m
/// switching penalty per moved player.
fn assign_slots(ctx: &ShapeCtx, state: &MatchState, mem: &mut TeamAiMemory, anchors: &[Vec2]) {
    let players = &state.players[ctx.side.index()];
    let active: Vec<usize> = (1..players.len()).filter(|&i| !players[i].sent_off).collect();
    let n = active.len();
    if n == 0 || anchors.is_empty() {
        return;
    }
    // Choose which slots are manned (drop the most advanced slots first after red cards).
    let mut kept: Vec<usize> = (0..anchors.len()).collect();
    while kept.len() > n {
        let drop_pos = kept
            .iter()
            .enumerate()
            .max_by_key(|(_, &k)| (ctx.slots[k].attack_depth, k))
            .map(|(pos, _)| pos)
            .unwrap_or(0);
        kept.remove(drop_pos);
    }
    let m = kept.len().min(n);
    let cost = |pi: usize, kk: usize| -> i64 {
        let p = &players[active[pi]];
        let k = kept[kk];
        let band = (role_band(p.role) - ctx.slots[k].role.band()).abs();
        let mut c = dist_dm_sq(p.pos, anchors[k]) + band * band * 60_000;
        if mem.slot_of[active[pi]] != k as u8 {
            c += 900;
        }
        c
    };
    // Previous assignment as positions into `kept`.
    let mut prev: Vec<usize> = Vec::with_capacity(n);
    let mut prev_valid = true;
    for &pi in &active {
        match kept.iter().position(|&k| k as u8 == mem.slot_of[pi]) {
            Some(kk) if !prev.contains(&kk) => prev.push(kk),
            _ => {
                prev_valid = false;
                break;
            }
        }
    }
    let total = |assign: &[usize]| -> i64 { (0..m).map(|pi| cost(pi, assign[pi])).sum() };

    let mut best: Vec<usize>;
    if n == 4 && m == 4 {
        const PERMS: [[usize; 4]; 24] = [
            [0, 1, 2, 3], [0, 1, 3, 2], [0, 2, 1, 3], [0, 2, 3, 1], [0, 3, 1, 2], [0, 3, 2, 1],
            [1, 0, 2, 3], [1, 0, 3, 2], [1, 2, 0, 3], [1, 2, 3, 0], [1, 3, 0, 2], [1, 3, 2, 0],
            [2, 0, 1, 3], [2, 0, 3, 1], [2, 1, 0, 3], [2, 1, 3, 0], [2, 3, 0, 1], [2, 3, 1, 0],
            [3, 0, 1, 2], [3, 0, 2, 1], [3, 1, 0, 2], [3, 1, 2, 0], [3, 2, 0, 1], [3, 2, 1, 0],
        ];
        let mut best_c = i64::MAX;
        best = PERMS[0].to_vec();
        for perm in &PERMS {
            let c = total(perm);
            if c < best_c {
                best_c = c;
                best = perm.to_vec();
            }
        }
    } else {
        best = if prev_valid {
            prev.clone()
        } else {
            // Greedy role-aware seed: each player (defenders first) takes his cheapest free slot.
            let mut order: Vec<usize> = (0..m).collect();
            order.sort_by_key(|&pi| (role_band(players[active[pi]].role), active[pi]));
            let mut seed = vec![usize::MAX; m];
            let mut used = vec![false; m];
            for &pi in &order {
                let mut bk = 0;
                let mut bc = i64::MAX;
                for (kk, &u) in used.iter().enumerate() {
                    if !u {
                        let c = cost(pi, kk);
                        if c < bc {
                            bc = c;
                            bk = kk;
                        }
                    }
                }
                used[bk] = true;
                seed[pi] = bk;
            }
            seed
        };
        for _ in 0..3 {
            let mut improved = false;
            for a in 0..m {
                for b in (a + 1)..m {
                    let cur = cost(a, best[a]) + cost(b, best[b]);
                    let swp = cost(a, best[b]) + cost(b, best[a]);
                    if swp < cur {
                        best.swap(a, b);
                        improved = true;
                    }
                }
            }
            if !improved {
                break;
            }
        }
    }
    let adopt = !prev_valid || (mem.slot_ticks >= 30 && total(&best) < total(&prev));
    if adopt && (!prev_valid || best != prev) {
        for (pi, &kk) in best.iter().enumerate().take(m) {
            mem.slot_of[active[pi]] = kept[kk] as u8;
        }
        mem.slot_ticks = 0;
    } else {
        mem.slot_ticks = mem.slot_ticks.saturating_add(1);
    }
    mem.slot_of[0] = AI_NONE;
}

/// Updates the side's team plan once per tick: reaction-delayed ball reference, phase
/// (attack / defend / counter-press / retreat / counter-attack), slots, presser + cover, marks.
fn update_team_plan(state: &MatchState, side: TeamSide) {
    let tick = state.tick;
    {
        let mems = state.ai_memory.borrow();
        if mems[side.index()].plan_tick == tick.wrapping_add(1) {
            return;
        }
    }
    let ctx = ShapeCtx::new(state, side);
    let diff = diff_profile(side_difficulty(state, side));
    let players = &state.players[side.index()];
    let opps = &state.players[side.opponent().index()];
    let n = players.len();
    let ball_xy = state.ball.pos.xy();
    let controlled = state.controlled_idx[side.index()];
    let is_5v5 = ctx.is_5v5;

    let mut mems = state.ai_memory.borrow_mut();
    let mem = &mut mems[side.index()];
    mem.ensure_len(n);
    let fresh = mem.plan_tick == 0;
    let continuous = !fresh && mem.plan_tick == tick;

    // 1. Reaction-delayed ball reference (difficulty x team positioning).
    let lead = state.ball.vel.xy().scale(Fp::from_milli(120));
    let ball_ref_raw = ball_xy + lead;
    let react = (diff.react_pm * (700 + avg_positioning_pm(state, side) * 3 / 10) / 1_000).clamp(60, 900);
    mem.ref_ball = if continuous {
        mem.ref_ball.lerp(ball_ref_raw, Fp::from_milli(react))
    } else {
        ball_ref_raw
    };
    let ref_ball = mem.ref_ball;

    // 2. Phase with transitions (round 4, area T): on a loss the team reacts as if counter-pressing,
    //    then after a 6-12 tick reaction decides press-or-fall-back from superiority within 10 m and
    //    the count behind the ball (Bauer & Anzer 2021). The counter-press lasts 300 ticks x style x
    //    work rate (floor 180) and ends early on a regain, a stoppage, a pass > 16 m that beats it or
    //    the ball escaping back past the origin; RETREAT then runs until the shape is reached or 570
    //    ticks.
    let team_work = {
        let mut s = 0;
        let mut c = 0;
        for p in players.iter().skip(1).filter(|p| !p.sent_off) {
            s += player_style(p).work_rate;
            c += 1;
        }
        s / c.max(1)
    };
    let f5 = usize::from(is_5v5);
    let owner_ours = state.ball.owner.map(|(s, _)| s == side);
    let mut phase = mem.phase;
    let mut ph_ticks = mem.phase_ticks.saturating_add(1);
    let style_pm: i64 = match ctx.style {
        TeamStyle::Press => 1_200,
        TeamStyle::Possession => 1_000,
        TeamStyle::Counter => 600,
    };
    let work_pm = (team_work * 1_000 / CP_WORK_REF).clamp(700, 1_200);
    let cp_window = (CP_WINDOW_TICKS * style_pm / 1_000 * work_pm / 1_000).max(CP_WINDOW_FLOOR) as u16;
    let react = CP_REACT_TICKS[match side_difficulty(state, side) {
        CpuDifficulty::Easy => 0,
        CpuDifficulty::Normal => 1,
        CpuDifficulty::Hard => 2,
    }];
    let ca_window: u16 = match ctx.style {
        TeamStyle::Counter => 150,
        TeamStyle::Press => 100,
        TeamStyle::Possession => 60,
    };
    let ball_p_now = ctx.prog(ball_xy.x);
    match owner_ours {
        Some(true) => {
            if fresh {
                phase = AI_PHASE_ATTACK;
            } else if is_defensive_phase(phase) {
                phase = AI_PHASE_COUNTER_ATTACK;
                ph_ticks = 0;
                mem.phase_origin = ball_xy;
            } else if phase == AI_PHASE_COUNTER_ATTACK && ph_ticks > ca_window {
                phase = AI_PHASE_ATTACK;
                ph_ticks = 0;
            }
        }
        Some(false) => {
            if fresh {
                phase = AI_PHASE_DEFEND;
            } else if !is_defensive_phase(phase) {
                phase = AI_PHASE_COUNTER_PRESS;
                ph_ticks = 0;
                mem.phase_origin = ball_xy;
                mem.cp_decided = false;
            } else if phase == AI_PHASE_COUNTER_PRESS {
                let escape = if is_5v5 { 9_000 } else { 14_000 };
                if !continuous || ball_p_now < ctx.prog(mem.phase_origin.x) - escape || ph_ticks > cp_window {
                    phase = AI_PHASE_RETREAT;
                    ph_ticks = 0;
                } else if !mem.cp_decided && ph_ticks >= react {
                    mem.cp_decided = true;
                    let r = Fp::from_milli(CP_SUPERIORITY_MM[f5]);
                    let near = |ps: &[PlayerState]| ps.iter().skip(1).filter(|p| !p.sent_off && p.pos.dist(ball_xy) <= r).count();
                    let behind = players
                        .iter()
                        .skip(1)
                        .filter(|p| !p.sent_off && ctx.prog(p.pos.x) < ball_p_now)
                        .count();
                    let press_ok = near(players) >= near(opps)
                        && behind >= CP_BEHIND_MIN[f5]
                        && ball_p_now > ctx.len * 35 / 100
                        && team_work > 450;
                    if !press_ok {
                        phase = AI_PHASE_RETREAT;
                        ph_ticks = 0;
                    }
                }
            } else if phase == AI_PHASE_RETREAT && ph_ticks > RETREAT_TICKS {
                phase = AI_PHASE_DEFEND;
                ph_ticks = 0;
            }
        }
        None => {
            if fresh {
                let bal = state.possession_balance_permille * if side == TeamSide::Home { 1 } else { -1 };
                phase = if bal >= 0 { AI_PHASE_ATTACK } else { AI_PHASE_DEFEND };
            } else if phase == AI_PHASE_COUNTER_PRESS {
                // A pass longer than 16 m beats the counter-press: fall back.
                let beaten = state
                    .ball
                    .pass_in_flight
                    .as_ref()
                    .is_some_and(|pf| pf.team != side && pf.target_pos.dist(ball_xy) > Fp::from_milli(CP_BEATEN_PASS_MM));
                if beaten || !continuous {
                    phase = AI_PHASE_RETREAT;
                    ph_ticks = 0;
                }
            }
        }
    }

    // 2b. High-intensity distance (fatigue integrator) and press triggers.
    for (i, p) in players.iter().enumerate() {
        let sp = p.vel.len().to_milli();
        let hi = mem.hi_dist_mm[i];
        let add = if sp > HI_SPEED_MM_S { (sp / 60) as u32 } else { 0 };
        mem.hi_dist_mm[i] = hi - hi / HI_WINDOW_TICKS + add;
    }
    let carrier_idx = state.ball.owner.and_then(|(os, oi)| (os != side).then_some(oi));
    let trigger_now = carrier_idx.is_some_and(|ci| {
        let c = &opps[ci];
        let heavy = state.ball.pos.xy().dist(c.pos) > Fp::from_milli(HEAVY_TOUCH_MM);
        let fwd = Vec2::from_bam(c.facing_bam, Fp::ONE);
        // The carrier attacks our goal (-sign); facing +sign means facing his own goal.
        let back_to_goal = fwd.x.to_milli() * ctx.sign > 500;
        let touchline = c.pos.y.abs() > state.format.half_width() - Fp::from_milli(TOUCHLINE_TRAP_MM);
        heavy || back_to_goal || touchline
    });
    mem.press_trigger_ticks = if trigger_now && matches!(phase, AI_PHASE_DEFEND | AI_PHASE_RETREAT) {
        PRESS_TRIGGER_HOLD
    } else {
        mem.press_trigger_ticks.saturating_sub(1)
    };

    // 2c. Rest offence: exactly one Low-work-rate front-slot forward stays high (the incumbent keeps
    //     it while eligible; otherwise the lowest work rate, then the most advanced slot).
    {
        let eligible = |i: usize| -> Option<(i64, i64)> {
            let p = &players[i];
            let s = ctx.slot(mem.slot_of[i])?;
            let dwr = player_style(p).def_work_rate;
            (!p.sent_off && matches!(s.role, SlotRole::Striker | SlotRole::Pivo) && dwr_band(dwr) == 0)
                .then_some((dwr, -s.attack_depth))
        };
        let inc = plan_idx(mem.rest_offence).filter(|&i| i < n && eligible(i).is_some());
        mem.rest_offence = match inc {
            Some(i) => i as u8,
            None => (1..n)
                .filter_map(|i| eligible(i).map(|(d, a)| (d, a, i)))
                .min()
                .map_or(AI_NONE, |(_, _, i)| i as u8),
        };
    }
    let rest_offence = mem.rest_offence;

    mem.phase = phase;
    mem.phase_ticks = ph_ticks;

    // 3. Slots against this phase's anchors.
    let (blk, attacking) = phase_block(&ctx, state, ref_ball, phase);
    let anchors: Vec<Vec2> = ctx
        .slots
        .iter()
        .enumerate()
        .map(|(k, s)| slot_pos(&ctx, &blk, s, attacking, front_mode(&ctx, state, &mem.slot_of, rest_offence, k)))
        .collect();
    assign_slots(&ctx, state, mem, &anchors);

    // 3a. Shared back line (one value per team, lag-tracked by the back-liners).
    if is_defensive_phase(phase) {
        let target = defend_block(&ctx, state, ref_ball, phase == AI_PHASE_RETREAT).line;
        mem.line_prog = if mem.line_prog == 0 || !continuous {
            target
        } else {
            mem.line_prog + (target - mem.line_prog) * LINE_LAG_PM / 1_000
        };
    } else {
        mem.line_prog = 0;
    }

    // 3b. Exclusivity layer: elect the one loose-ball chaser (agent2d margins + GRF ratio hysteresis).
    let cpu_side = side_is_cpu(state, side);
    // Same role cost as the presser ranking: centre-backs hold the line instead of chasing a ball
    // far in front of it.
    let chase_role_cost: Vec<u32> = {
        let line = defend_block(&ctx, state, ref_ball, false).line;
        let ball_p = ctx.prog(ball_xy.x);
        (0..n)
            .map(|i| {
                let back = ctx.slot(mem.slot_of[i]).is_some_and(|s| s.role.is_back_line());
                if back && ball_p > line + 8_000 { 14 } else { 0 }
            })
            .collect()
    };
    elect_chaser(state, side, mem, controlled, cpu_side, &chase_role_cost);
    let chaser = mem.chaser;
    // On a loose ball the elected chaser (or the committed keeper) is the only player going to the
    // ball, so there is no presser on it: the presser role is for a carrier.
    let loose_exclusive = state.ball.owner.is_none() && (chaser != 0 || mem.keeper_claim);

    // 4. One presser, the counter-press lane-closers (or the second man on a press trigger) and one
    //    cover, with hysteresis. As before, an AI presser is committed only when clearly faster than
    //    the controlled player; when one is, a CPU-controlled player stands off (cover / mark / slot)
    //    so at most two players engage. The chaser is never the presser, and a human-controlled
    //    player is never planned for. The human's teammate-contain mate is the second engager: he is
    //    excluded from every election and the AI presser is demoted to cover while contain is held.
    let opp_owns = owner_ours == Some(false);
    let defensive = is_defensive_phase(phase) && owner_ours != Some(true);
    let def_blk = defend_block(&ctx, state, ref_ball, false);
    let contain = (!cpu_side && opp_owns)
        .then(|| state.teammate_contain_idx[side.index()])
        .flatten()
        .filter(|&c| c > 0 && c < n && c != controlled && !players[c].sent_off);
    mem.contain_mate = contain.map_or(AI_NONE, |c| c as u8);
    let def_anchor: Vec<Vec2> = (0..n)
        .map(|i| match ctx.slot(mem.slot_of[i]) {
            Some(s) => {
                let fm = front_mode(&ctx, state, &mem.slot_of, rest_offence, mem.slot_of[i] as usize);
                slot_pos(&ctx, &def_blk, &s, false, fm)
            }
            None => players[i].pos,
        })
        .collect();
    let mut new_presser = AI_NONE;
    let mut new_second = AI_NONE;
    let mut new_third = AI_NONE;
    let mut demoted = AI_NONE;
    if defensive {
        let waypoints = if state.ball.owner.is_some() {
            [state.ball.pos; 21]
        } else {
            sample_ball_3d_waypoints(state)
        };
        let owned_pt = predict_ball_xy_at_ticks(state, 8);
        let eval = |i: usize| -> u32 {
            let p = &players[i];
            if state.ball.owner.is_some() {
                estimate_ticks_to_point(p, owned_pt, is_5v5)
            } else {
                best_intercept_from_waypoints(p, &waypoints, is_5v5, false).1
            }
        };
        let radius = Fp::from_milli(match (is_5v5, phase == AI_PHASE_COUNTER_PRESS) {
            (true, false) => 11_500,
            (true, true) => 14_000,
            (false, false) => 16_000,
            (false, true) => 20_000,
        });
        let ball_p = ctx.prog(ball_xy.x);
        let ball_yf = ctx.yf(ball_xy.y);
        let middle_third = ball_p > ctx.len / 3 && ball_p < ctx.len * 2 / 3;
        let press_cost = |i: usize| -> u32 {
            let p = &players[i];
            let st = player_style(p);
            let mut c = eval(i) as i64;
            let slot = ctx.slot(mem.slot_of[i]);
            // Centre-backs hold the line instead of chasing a ball far in front of it.
            if slot.is_some_and(|s| s.role.is_back_line()) && ball_p > def_blk.line + 8_000 {
                c += 14;
            }
            // Prefer a midfielder as presser in the middle third [official FIFA 12-13; estimate].
            if middle_third
                && slot.is_some_and(|s| matches!(s.role, SlotRole::CentralMid | SlotRole::Holding | SlotRole::WideMid))
            {
                c -= 3;
            }
            // Zone handover: a presser far outside his zone costs 2 ticks per metre beyond 12 m.
            let lat = (ctx.yf(def_anchor[i].y) - ball_yf).abs();
            if lat > ZONE_HANDOVER_LAT_MM {
                c += (lat - ZONE_HANDOVER_LAT_MM) / 500;
            }
            c -= (st.aggression - 500) / 100;
            c += (500 - st.work_rate).max(0) / 100;
            // No bonus for the CPU-controlled player here: control follows the presser via the
            // auto-switch, and a bonus that moves with control would flip the presser back and forth.
            c.max(0) as u32
        };
        let ctrl_ticks = if controlled > 0 && controlled < n && !players[controlled].sent_off {
            eval(controlled)
        } else {
            u32::MAX / 4
        };
        // Zone handover: outside the counter-press, the incumbent is released once the carrier is
        // beyond 12 m laterally from his slot or 8 m beyond his line (back-liners excepted).
        let released = plan_idx(mem.presser).filter(|&cur| {
            // Hysteresis (merge fix): the incumbent is held `diff.hold` ticks first and released only
            // 2 m beyond the zone edge the challengers are charged from, so the handover cannot
            // flip the presser back and forth at the edge.
            cur < n
                && phase != AI_PHASE_COUNTER_PRESS
                && mem.presser_ticks >= diff.hold
                && ((ctx.yf(def_anchor[cur].y) - ball_yf).abs() > ZONE_HANDOVER_LAT_MM + ZONE_HANDOVER_HYST_MM
                    || (!ctx.slot(mem.slot_of[cur]).is_some_and(|s| s.role.is_back_line())
                        && ball_p < ctx.prog(def_anchor[cur].x) - ZONE_HANDOVER_DEPTH_MM - ZONE_HANDOVER_HYST_MM))
        });
        let mut ranked: Vec<(u32, usize)> = (1..n)
            .filter(|&i| {
                i != controlled
                    && i as u8 != chaser
                    && Some(i) != contain
                    && Some(i) != released
                    && !loose_exclusive
                    && !players[i].sent_off
                    && players[i].pos.dist(ball_xy) <= radius
            })
            .map(|i| (press_cost(i), i))
            .collect();
        ranked.sort_unstable();
        let cur = mem.presser as usize;
        let cur_valid = mem.presser != AI_NONE && ranked.iter().any(|&(_, i)| i == cur);
        if let Some(&(best_c, best_i)) = ranked.first() {
            let chosen = if cur_valid {
                let cur_c = ranked.iter().find(|&&(_, i)| i == cur).map_or(u32::MAX, |&(c, _)| c);
                let margin = if mem.presser_ticks >= diff.hold { 3 } else { diff.margin };
                if best_i != cur && best_c + margin < cur_c { best_i } else { cur }
            } else {
                best_i
            };
            let chosen_ticks = eval(chosen);
            let ctrl_margin = if cur_valid && chosen == cur { 2 } else { 5 };
            if chosen_ticks + ctrl_margin < ctrl_ticks || (cur_valid && chosen == cur && chosen_ticks < ctrl_ticks + 3) {
                new_presser = chosen as u8;
            }
            if contain.is_some() && new_presser != AI_NONE {
                // The human and his contain mate are the two engagers: the AI presser covers.
                demoted = new_presser;
                new_presser = AI_NONE;
            }
            let free = |i: usize| -> bool {
                i != chosen
                    && i as u8 != new_presser
                    && i as u8 != demoted
                    && !ctx.slot(mem.slot_of[i]).is_some_and(|s| s.role.is_back_line())
            };
            if phase == AI_PHASE_COUNTER_PRESS && opp_owns && contain.is_none() {
                // Two lane-closers (the rest-defence back line never joins the counter-press).
                let mut closers = ranked.iter().filter(|&&(c, i)| c < 70 && free(i)).map(|&(_, i)| i);
                new_second = closers.next().map_or(AI_NONE, |i| i as u8);
                new_third = closers.next().map_or(AI_NONE, |i| i as u8);
            } else if mem.press_trigger_ticks > 0 && opp_owns && contain.is_none() {
                // Press trigger outside the counter-press: a second man shadows the presser.
                new_second = ranked
                    .iter()
                    .find(|&&(c, i)| c < 50 && free(i))
                    .map_or(AI_NONE, |&(_, i)| i as u8);
            }
        }
    }
    if new_presser != mem.presser {
        mem.presser_ticks = 0;
    } else {
        mem.presser_ticks = mem.presser_ticks.saturating_add(1);
    }
    mem.presser = new_presser;
    mem.second_presser = new_second;
    mem.third_presser = new_third;

    // 4b. Lanes for the closers: the two best receiver lanes from the carrier (incumbent lanes are
    //     kept while still among the candidates), assigned to minimise the closers' total distance.
    if let Some(ci) = carrier_idx.filter(|_| new_second != AI_NONE && phase == AI_PHASE_COUNTER_PRESS) {
        let lanes = best_lanes(&ctx, state, side, ci, &mem.closer_lane);
        let pos = |k: u8| plan_idx(k).map(|i| players[i].pos);
        let lane_pt = |o: usize| opps[ci].pos.lerp(opps[o].pos, Fp::HALF);
        mem.closer_lane = match (lanes.first().copied(), lanes.get(1).copied(), pos(new_second), pos(new_third)) {
            (Some(a), Some(b), Some(p2), Some(p3)) => {
                if p2.dist(lane_pt(a)) + p3.dist(lane_pt(b)) <= p2.dist(lane_pt(b)) + p3.dist(lane_pt(a)) {
                    [a as u8, b as u8]
                } else {
                    [b as u8, a as u8]
                }
            }
            (Some(a), _, _, _) => [a as u8, AI_NONE],
            _ => [AI_NONE; 2],
        };
    } else {
        mem.closer_lane = [AI_NONE; 2];
    }

    let mut new_cover = AI_NONE;
    // On a loose ball the chaser (or keeper) is the only player with a job on it: everyone else, the
    // would-be cover included, goes to his tactical slot.
    if defensive && !loose_exclusive {
        let pressing_pos = if new_presser != AI_NONE {
            players[new_presser as usize].pos
        } else if chaser != 0 {
            players[chaser as usize].pos
        } else if controlled > 0 && controlled < n {
            players[controlled].pos
        } else {
            ball_xy
        };
        let own_goal = ctx.world(0, 0);
        let depth = Fp::from_milli(if is_5v5 { 4_500 } else { 6_500 });
        let cover_pt = ball_xy.lerp(pressing_pos, Fp::HALF) + (own_goal - ball_xy).normalize_or_zero().scale(depth);
        let mut best: Option<(i64, usize)> = None;
        for (i, cand) in players.iter().enumerate().skip(1) {
            if cand.sent_off
                || (i == controlled && (!cpu_side || new_presser == AI_NONE))
                || i as u8 == chaser
                || i as u8 == new_presser
                || i as u8 == new_second
                || i as u8 == new_third
                || Some(i) == contain
            {
                continue;
            }
            let pref = match ctx.slot(mem.slot_of[i]).map(|s| s.role) {
                Some(SlotRole::CentreBack | SlotRole::Holding | SlotRole::Fixo) => 0,
                Some(SlotRole::FullBack | SlotRole::CentralMid) => 3_000,
                _ => 8_000,
            };
            let mut c = cand.pos.dist(cover_pt).to_milli() + pref;
            if i as u8 == mem.cover {
                c -= 3_000;
            }
            if best.is_none_or(|(bc, _)| c < bc) {
                best = Some((c, i));
            }
        }
        new_cover = best.map_or(AI_NONE, |(_, i)| i as u8);
        if demoted != AI_NONE {
            new_cover = demoted;
        }
        // A CPU-controlled player standing off an AI presser is the cover (the second engager),
        // so standing off never pulls an extra AI player out of the shape.
        if cpu_side
            && new_presser != AI_NONE
            && controlled > 0
            && controlled < n
            && !players[controlled].sent_off
            && controlled as u8 != new_presser
            && controlled as u8 != new_second
            && controlled as u8 != new_third
            && controlled as u8 != chaser
        {
            new_cover = controlled as u8;
        }
    }
    if new_cover != mem.cover {
        mem.cover_ticks = 0;
    } else {
        mem.cover_ticks = mem.cover_ticks.saturating_add(1);
    }
    mem.cover = new_cover;

    // 5. Marking by one arbiter (round 4, area T; FIFA 14 "most dangerous player"): incumbents first
    //    (kept for a 24-tick dwell, then only beaten by a candidate 3 m closer), then the free
    //    opponents in danger order to the nearest free defender. A back-liner keeps a runner in
    //    behind until a deeper teammate covers the runner-to-goal line (Konami's La check).
    //    Defend / retreat: everyone but the rest offence and Low-work-rate front slots; counter-press:
    //    the back line and the holding slot (the rest-defence core).
    let zone = Fp::from_milli(if is_5v5 { 7_000 } else { 9_000 });
    let front_p = def_blk.line + def_blk.len + 2_000;
    let own_goal = ctx.world(0, 0);
    let eligible: Vec<bool> = (0..n)
        .map(|i| {
            let p = &players[i];
            let slot = ctx.slot(mem.slot_of[i]);
            let front = slot.is_some_and(|s| matches!(s.role, SlotRole::Striker | SlotRole::Pivo | SlotRole::Winger));
            let core = slot.is_some_and(|s| s.role.is_back_line() || s.role == SlotRole::Holding);
            let phase_ok = match phase {
                AI_PHASE_DEFEND | AI_PHASE_RETREAT => {
                    !front || (dwr_band(player_style(p).def_work_rate) > 0 && Some(i) != plan_idx(rest_offence))
                }
                AI_PHASE_COUNTER_PRESS => core,
                _ => false,
            };
            i > 0
                && slot.is_some()
                && phase_ok
                && !p.sent_off
                && (i != controlled || (cpu_side && new_presser != AI_NONE))
                && i as u8 != chaser
                && i as u8 != new_presser
                && i as u8 != new_cover
                && i as u8 != new_second
                && i as u8 != new_third
                && Some(i) != contain
        })
        .collect();
    let opp_ok = |o: usize| -> bool {
        o < opps.len() && o != 0 && !opps[o].sent_off && Some(o) != carrier_idx && ctx.prog(opps[o].pos.x) <= front_p
    };
    // Konami cover check: a teammate other than `me` within La of the opponent-to-goal line and
    // closer to goal than the opponent.
    let covered = |o: usize, me: usize| -> bool {
        let op = opps[o].pos;
        let op_p = ctx.prog(op.x);
        players.iter().enumerate().skip(1).any(|(j, t)| {
            j != me
                && !t.sent_off
                && ctx.prog(t.pos.x) < op_p
                && closest_on_segment(op, own_goal, t.pos).dist(t.pos) <= Fp::from_milli(COVER_LA_MM)
        })
    };
    let line_p = mem.line_prog;
    let mut taken = vec![false; opps.len()];
    let mut new_mark = vec![AI_NONE; n];
    let mut new_ticks = vec![0_u16; n];
    // (a) Incumbents.
    for i in 1..n {
        if !eligible[i] {
            continue;
        }
        let Some(o) = plan_idx(mem.mark_of[i]).filter(|&o| opp_ok(o) && !taken[o]) else { continue };
        let d_anchor = def_anchor[i].dist(opps[o].pos);
        let back = ctx.slot(mem.slot_of[i]).is_some_and(|s| s.role.is_back_line());
        let runner = back && line_p != 0 && ctx.prog(opps[o].pos.x) < line_p;
        let keep = if runner {
            players[i].pos.dist(opps[o].pos) <= Fp::from_milli(RUNNER_TRACK_MM) && !covered(o, i)
        } else {
            d_anchor <= zone.mul(Fp::from_milli(1_350))
        };
        if keep {
            new_mark[i] = o as u8;
            new_ticks[i] = mem.mark_ticks[i].saturating_add(1);
            taken[o] = true;
        }
    }
    // (b) Free opponents by danger: value (closeness to our goal of the 1 s look-ahead point, central
    //     bonus) x reachability (closeness to the ball).
    let mut free_opps: Vec<(i64, usize)> = (1..opps.len())
        .filter(|&o| opp_ok(o) && !taken[o])
        .map(|o| {
            let look = opps[o].pos + opps[o].vel;
            let value = (1_000 - ctx.prog(look.x) * 1_000 / ctx.len).clamp(0, 1_000)
                + if ctx.yf(look.y).abs() < ctx.half_wid / 3 { 200 } else { 0 };
            let reach = 15_000_000 / (15_000 + look.dist(ball_xy).to_milli());
            (-(value * reach / 1_000), o)
        })
        .collect();
    free_opps.sort_unstable();
    for &(_, o) in &free_opps {
        let mut best: Option<(Fp, usize)> = None;
        for i in 1..n {
            if !eligible[i] || new_mark[i] != AI_NONE {
                continue;
            }
            let d = def_anchor[i].dist(opps[o].pos);
            if d <= zone && best.is_none_or(|(bd, _)| d < bd) {
                best = Some((d, i));
            }
        }
        if let Some((_, i)) = best {
            new_mark[i] = o as u8;
            new_ticks[i] = 0;
            taken[o] = true;
        }
    }
    // (c) After the dwell, an incumbent may switch to a still-free opponent 3 m closer.
    for i in 1..n {
        let Some(cur) = plan_idx(new_mark[i]) else { continue };
        if new_ticks[i] < MARK_DWELL_TICKS || new_ticks[i] == 0 {
            continue;
        }
        let cd = def_anchor[i].dist(opps[cur].pos);
        let mut best: Option<(Fp, usize)> = None;
        for o in 1..opps.len() {
            if opp_ok(o) && !taken[o] {
                let d = def_anchor[i].dist(opps[o].pos);
                if d <= zone && d + Fp::from_milli(MARK_INCUMBENT_MM) < cd && best.is_none_or(|(bd, _)| d < bd) {
                    best = Some((d, o));
                }
            }
        }
        if let Some((_, o)) = best {
            taken[cur] = false;
            taken[o] = true;
            new_mark[i] = o as u8;
            new_ticks[i] = 0;
        }
    }
    mem.mark_of = new_mark;
    mem.mark_ticks = new_ticks;
    assign_move_classes(state, side, mem, controlled, cpu_side);
    mem.plan_tick = tick.wrapping_add(1);
}

/// The best (up to two) receiver lanes from opponent carrier `ci` for the counter-press closers:
/// receivers within 25 m ranked by distance minus half their progress toward our goal; a lane in
/// `prev` stays while it is within 3 m of the best score.
fn best_lanes(ctx: &ShapeCtx, state: &MatchState, side: TeamSide, ci: usize, prev: &[u8; 2]) -> Vec<usize> {
    let opps = &state.players[side.opponent().index()];
    let c = opps[ci].pos;
    let c_p = ctx.prog(c.x);
    let mut cands: Vec<(i64, usize)> = opps
        .iter()
        .enumerate()
        .skip(1)
        .filter(|&(o, op)| {
            o != ci
                && !op.sent_off
                && op.pos.dist(c) <= Fp::from_milli(25_000)
                // Only lanes toward our goal (receivers no more than 3 m behind the carrier): a
                // closer on a back-pass lane stood ahead of the ball.
                && ctx.prog(op.pos.x) <= c_p + 3_000
        })
        .map(|(o, op)| {
            let mut score = op.pos.dist(c).to_milli() - (c_p - ctx.prog(op.pos.x)) / 2;
            if prev.contains(&(o as u8)) {
                score -= 3_000;
            }
            (score, o)
        })
        .collect();
    cands.sort_unstable();
    cands.into_iter().take(2).map(|(_, o)| o).collect()
}

/// Defensive-shape anchor `players[side][idx]` currently owns (public for metrics and tests).
pub fn defensive_shape_anchor(state: &MatchState, side: TeamSide, idx: usize) -> Vec2 {
    let ctx = ShapeCtx::new(state, side);
    let (slot, ref_ball, slot_of, rest) = {
        let mems = state.ai_memory.borrow();
        let m = &mems[side.index()];
        (m.slot_of.get(idx).copied().unwrap_or(AI_NONE), m.ref_ball, m.slot_of.clone(), m.rest_offence)
    };
    match ctx.slot(slot) {
        Some(s) => {
            let fm = front_mode(&ctx, state, &slot_of, rest, slot as usize);
            slot_pos(&ctx, &defend_block(&ctx, state, ref_ball, false), &s, false, fm)
        }
        None => world_anchor_for_player(state, side, idx, Fp::ZERO),
    }
}

/// The AI outfielder currently committed to pressing for `side`, if any (metrics and tests).
pub fn committed_presser(state: &MatchState, side: TeamSide) -> Option<usize> {
    let p = state.ai_memory.borrow()[side.index()].presser;
    (p != AI_NONE).then_some(p as usize)
}

/// Current AI team phase (`AI_PHASE_*`) for `side` (public for metrics and tests).
pub fn team_phase(state: &MatchState, side: TeamSide) -> u8 {
    state.ai_memory.borrow()[side.index()].phase
}

/// Formation-slot anchor `players[side][idx]` owns in the current phase's block, before role
/// refinements, spacing and yield (public for metrics and tests).
pub fn shape_slot_anchor(state: &MatchState, side: TeamSide, idx: usize) -> Vec2 {
    let ctx = ShapeCtx::new(state, side);
    let (slot, ref_ball, phase, slot_of, rest) = {
        let mems = state.ai_memory.borrow();
        let m = &mems[side.index()];
        (m.slot_of.get(idx).copied().unwrap_or(AI_NONE), m.ref_ball, m.phase, m.slot_of.clone(), m.rest_offence)
    };
    match ctx.slot(slot) {
        Some(s) => {
            let (blk, attacking) = phase_block(&ctx, state, ref_ball, phase);
            let fm = front_mode(&ctx, state, &slot_of, rest, slot as usize);
            slot_pos(&ctx, &blk, &s, attacking, fm)
        }
        None => world_anchor_for_player(state, side, idx, Fp::ZERO),
    }
}

// -------------------------------------------------------------------------------------------------
// Exclusivity layer (see `docs/exclusivity-layer.md`): one chaser per team, engager cap, one runner
// per destination, teammate yield radius, off-ball cruise speed, keeper-defender arbitration and
// movement priority classes. All decisions are made once per side per tick inside the team plan
// and stored in `TeamAiMemory`, so the read-outs below are cheap and deterministic.
// -------------------------------------------------------------------------------------------------

/// Challengers to an incumbent chaser are evaluated no more often than every 8 ticks (133 ms).
const CHASER_EVAL_PERIOD: u32 = 8;
/// GRF designated-player smoothing: `(best + K) / (incumbent + K) < 0.8` with `K` = 500 ms.
const CHASER_RATIO_K: u32 = 30;
/// agent2d contest rule `self <= 3 || self < opp + 3` in 100 ms cycles, converted to 60 Hz ticks.
const CHASE_CONTEST_TICKS: u32 = 18;
/// An incumbent chaser keeps the ball until the opponent is clearly first by this many ticks.
const CHASE_RELEASE_TICKS: u32 = 30;
/// Tick bonus for the CPU-controlled player in chaser / presser ranking (agrees with auto-switch).
const CPU_CTRL_CHASE_BONUS: u32 = 4;
/// Off-ball cruise speed of an uninvolved (class 3) player: 70% of top (sprint) speed.
const CRUISE_TOP_SPEED_PM: i64 = 700;

#[inline]
fn side_is_cpu(state: &MatchState, side: TeamSide) -> bool {
    let ctrl = match side {
        TeamSide::Home => state.rules.home_controller,
        TeamSide::Away => state.rules.away_controller,
    };
    ctrl.is_cpu()
}

/// The keeper is committed to a loose ball (Collect / CrossClaim / Sweep, or a human-held rush).
pub fn keeper_committed(state: &MatchState, side: TeamSide) -> bool {
    state.ball.owner.is_none()
        && matches!(
            state.gk_brain[side.index()].scenario,
            GkScenario::Collect | GkScenario::CrossClaim | GkScenario::Sweep | GkScenario::HumanRush
        )
}

/// `true` when `best` beats `incumbent` by the GRF ratio rule `(best + K) / (incumbent + K) < 0.8`.
#[inline]
const fn clearly_better(best: u32, incumbent: u32) -> bool {
    (best as u64 + CHASER_RATIO_K as u64) * 10 < (incumbent as u64 + CHASER_RATIO_K as u64) * 8
}

/// A human-controlled player is "closing" when he is on the ball or running at >= 2 m/s toward his
/// intercept point.
fn human_closing(p: &PlayerState, intercept: Vec2, ball: Vec2) -> bool {
    p.pos.dist(ball) <= Fp::from_milli(2_500)
        || p.vel.dot((intercept - p.pos).normalize_or_zero()) >= Fp::from_milli(2_000)
}

/// Elects at most one loose-ball chaser for `side` and stores it in `mem.chaser` / `mem.chaser_pt`.
///
/// - Ball owned, or the keeper committed to it: no chaser (`keeper_claim` records the second case).
/// - Our own pass in flight: the intended receiver is the chaser.
/// - Human side: the human-controlled player is never elected but counts as the chaser while he is
///   about as fast as the best AI teammate, or closing and not clearly beaten (GRF ratio rule).
/// - Otherwise the best intercept time wins if contested (agent2d: `t <= 18` or `t < opp + 18`
///   ticks); the incumbent keeps the role unless a challenger is clearly better (ratio < 0.8,
///   checked every `CHASER_EVAL_PERIOD` ticks) or the opponent is first by `CHASE_RELEASE_TICKS`.
fn elect_chaser(
    state: &MatchState,
    side: TeamSide,
    mem: &mut TeamAiMemory,
    controlled: usize,
    cpu_side: bool,
    role_cost: &[u32],
) {
    let players = &state.players[side.index()];
    let n = players.len();
    let tick = state.tick;
    let is_5v5 = state.format.players_per_side <= 6;
    let ball_xy = state.ball.pos.xy();
    let prev = mem.chaser;
    let mut chosen: u8 = 0;
    let mut chosen_pt = ball_xy;
    let mut eval_now = false;
    mem.keeper_claim = false;
    if state.ball.owner.is_none() {
        let receiver = state.ball.pass_in_flight.as_ref().and_then(|pf| {
            (pf.team == side && pf.intended_receiver_idx > 0 && pf.intended_receiver_idx < n)
                .then_some(pf.intended_receiver_idx)
                .filter(|&r| !players[r].sent_off)
        });
        let waypoints = sample_ball_3d_waypoints(state);
        if keeper_committed(state, side) {
            mem.keeper_claim = true;
        } else if let Some(r) = receiver {
            chosen = r as u8;
            chosen_pt = best_intercept_from_waypoints(&players[r], &waypoints, is_5v5, false).0;
        } else {
            let human = !cpu_side && controlled > 0 && controlled < n && !players[controlled].sent_off;
            // Candidate AI outfielders: (ranking cost, index, raw ticks).
            let mut best: Option<(u32, usize)> = None;
            let mut inc_cost: Option<u32> = None;
            for (i, p) in players.iter().enumerate().skip(1) {
                if p.sent_off || (human && i == controlled) {
                    continue;
                }
                let t = best_intercept_from_waypoints(p, &waypoints, is_5v5, false).1;
                let mut c = t + role_cost.get(i).copied().unwrap_or(0);
                if cpu_side && i == controlled {
                    c = c.saturating_sub(CPU_CTRL_CHASE_BONUS);
                }
                if best.is_none_or(|(bc, _)| c < bc) {
                    best = Some((c, i));
                }
                if i as u8 == prev {
                    inc_cost = Some(c);
                }
            }
            let mut opp_best = u32::MAX / 4;
            for (oi, op) in state.players[side.opponent().index()].iter().enumerate() {
                if !op.sent_off {
                    opp_best = opp_best.min(best_intercept_from_waypoints(op, &waypoints, is_5v5, oi == 0).1);
                }
            }
            let human_chaser = human && {
                let hp = &players[controlled];
                let (h_pt, h_t) = best_intercept_from_waypoints(hp, &waypoints, is_5v5, false);
                let best_ai = best.map_or(u32::MAX / 4, |(c, _)| c);
                h_t <= best_ai + 6 || (human_closing(hp, h_pt, ball_xy) && !clearly_better(best_ai, h_t))
            };
            if human_chaser {
                chosen = controlled as u8;
                chosen_pt = smooth_ball_intercept(&players[controlled], &waypoints, is_5v5);
            } else if let Some((best_c, best_i)) = best {
                let due = mem.chaser_eval_tick == 0
                    || tick.wrapping_sub(mem.chaser_eval_tick.wrapping_sub(1)) >= CHASER_EVAL_PERIOD;
                chosen = match inc_cost {
                    Some(ic) if ic > CHASE_CONTEST_TICKS && ic >= opp_best + CHASE_RELEASE_TICKS => 0,
                    Some(ic) => {
                        if due {
                            eval_now = true;
                        }
                        if due && best_i as u8 != prev && clearly_better(best_c, ic) {
                            best_i as u8
                        } else {
                            prev
                        }
                    }
                    None if best_c <= CHASE_CONTEST_TICKS || best_c < opp_best + CHASE_CONTEST_TICKS => {
                        eval_now = true;
                        best_i as u8
                    }
                    None => 0,
                };
                if chosen != 0 {
                    chosen_pt = smooth_ball_intercept(&players[chosen as usize], &waypoints, is_5v5);
                }
            }
        }
    }
    if chosen != prev {
        mem.chaser_ticks = 0;
        eval_now = chosen != 0;
    } else {
        mem.chaser_ticks = mem.chaser_ticks.saturating_add(1);
    }
    if chosen == 0 {
        mem.chaser_eval_tick = 0;
    } else if eval_now {
        mem.chaser_eval_tick = tick.wrapping_add(1);
    }
    mem.chaser = chosen;
    mem.chaser_pt = chosen_pt;
}

/// Baseline movement priority class per player for this tick (refined per player by
/// `outfield_shape_intent`): 0 = carrier / elected chaser / committed or ball-holding keeper,
/// 1 = presser, lane-closers / second man, cover, the human's contain mate and the human-controlled
/// player, 2 = everyone else.
fn assign_move_classes(state: &MatchState, side: TeamSide, mem: &mut TeamAiMemory, controlled: usize, cpu_side: bool) {
    let n = state.players[side.index()].len();
    for i in 0..n {
        let c = if state.ball.owner == Some((side, i)) || (i != 0 && i as u8 == mem.chaser) {
            0
        } else if i == 0 {
            if keeper_committed(state, side) { 0 } else { 2 }
        } else if i as u8 == mem.presser
            || i as u8 == mem.second_presser
            || i as u8 == mem.third_presser
            || i as u8 == mem.contain_mate
            || i as u8 == mem.cover
            || (!cpu_side && i == controlled)
        {
            1
        } else {
            2
        };
        mem.move_class[i] = c;
    }
}

#[inline]
fn plan_is_fresh(state: &MatchState, m: &TeamAiMemory) -> bool {
    matches!(state.phase, MatchPhase::OpenPlay)
        && (m.plan_tick == state.tick.wrapping_add(1) || m.plan_tick == state.tick)
}

/// The one outfielder elected to chase the loose ball for `side` (the human-controlled player when
/// he is the one closing on it), or `None` (ball owned, keeper committed, or nobody contests it).
pub fn elected_chaser(state: &MatchState, side: TeamSide) -> Option<usize> {
    if state.ball.owner.is_some() || keeper_committed(state, side) {
        return None;
    }
    let mems = state.ai_memory.try_borrow().ok()?;
    let m = &mems[side.index()];
    (plan_is_fresh(state, m) && m.chaser != 0).then_some(m.chaser as usize)
}

/// Movement priority class of `players[side][idx]` for local avoidance (lower = higher priority):
///
/// - `0`: the ball carrier, the elected chaser, or the keeper while committed to a loose ball;
///   never yields to teammates.
/// - `1`: an engager (presser, second presser, cover) or the human-controlled player.
/// - `2`: a tactical mover (recovery run, reserved attacking run, marking, far from his slot).
/// - `3`: idle, cruising to his slot, or yielding (most yielding).
///
/// Reads the tick's team plan only (no recomputation). Outside open play, or before the side has
/// been planned this tick, it returns `2` for everyone but the carrier.
pub fn move_priority(state: &MatchState, side: TeamSide, idx: usize) -> u8 {
    let players = &state.players[side.index()];
    if idx >= players.len() || players[idx].sent_off {
        return 3;
    }
    if state.ball.owner == Some((side, idx)) {
        return 0;
    }
    if idx == 0 {
        return if keeper_committed(state, side) { 0 } else { 2 };
    }
    let Ok(mems) = state.ai_memory.try_borrow() else {
        return 2;
    };
    let m = &mems[side.index()];
    if !plan_is_fresh(state, m) {
        return 2;
    }
    if m.chaser as usize == idx && !keeper_committed(state, side) && state.ball.owner.is_none() {
        return 0;
    }
    m.move_class.get(idx).copied().map_or(2, |c| c.max(1))
}

/// Chaser intercept anti-dither (UvA Trilearn `WorldModelPredict`: keep the previous intercept
/// unless the new one moved materially [code]): a new intercept point closer than
/// `min(0.4 m, 15% of the distance to it)` to the previous tick's accepted point is not taken
/// (0.4 m = one body radius, the WA1 re-target deadband [estimate]); the 15% bound keeps the
/// heading error under ~9 degrees as the chaser closes in.
const CHASE_DEADBAND_MM: i64 = 400;
const CHASE_DEADBAND_REL_MILLI: i64 = 150;
/// Attack-the-ball mode (aim 3 ticks ahead) enters within 1.5 m of a low ball and, with
/// hysteresis, leaves beyond 1.9 m [estimate], so the chase point does not toggle between the
/// two predictors at the boundary.
const CHASE_NEAR_ENTER_MM: i64 = 1_500;
const CHASE_NEAR_EXIT_MM: i64 = 1_900;

/// Loose-ball chase for the elected AI chaser: slide along the ball path (continuous intercept),
/// attack the ball itself once on top of it. The accepted point is remembered per player
/// (`steer_chase_*`, hashed) so it only moves when the new intercept moved materially; a second
/// call on the same tick returns the same point.
fn chase_intent(state: &MatchState, side: TeamSide, idx: usize) -> AiMoveIntent {
    let p = &state.players[side.index()][idx];
    let is_5v5 = state.format.players_per_side <= 6;
    let tick = state.tick;
    let count = state.players[side.index()].len();
    let (prev_pt, prev_tick, prev_near) = {
        let mut mems = state.ai_memory.borrow_mut();
        let m = &mut mems[side.index()];
        m.ensure_len(count);
        (m.steer_chase_pt[idx], m.steer_chase_tick[idx], m.steer_chase_near[idx] != 0)
    };
    let chase_pt = if prev_tick == tick && tick != 0 {
        prev_pt
    } else {
        let continuous = prev_tick != 0 && prev_tick.wrapping_add(1) == tick;
        let near_lim = if continuous && prev_near {
            CHASE_NEAR_EXIT_MM
        } else {
            CHASE_NEAR_ENTER_MM
        };
        let near = p.pos.dist(state.ball.pos.xy()) <= Fp::from_milli(near_lim)
            && state.ball.pos.z <= Fp::from_milli(900);
        let raw = if near {
            predict_ball_xy_at_ticks(state, 3)
        } else {
            smooth_ball_intercept(p, &sample_ball_3d_waypoints(state), is_5v5)
        };
        let band = Fp::from_milli(CHASE_DEADBAND_MM)
            .min(raw.dist(p.pos).mul(Fp::from_milli(CHASE_DEADBAND_REL_MILLI)));
        let pt = if continuous && near == prev_near && raw.dist(prev_pt) <= band {
            prev_pt
        } else {
            raw
        };
        let mut mems = state.ai_memory.borrow_mut();
        let m = &mut mems[side.index()];
        m.steer_chase_pt[idx] = pt;
        m.steer_chase_tick[idx] = tick;
        m.steer_chase_near[idx] = u8::from(near);
        pt
    };
    AiMoveIntent {
        move_dir: (chase_pt - p.pos).normalize_or_zero(),
        sprint: true,
        jockey: false,
        trigger_dive: None,
    }
}

/// Closest point to `pt` on the segment `a..b`.
fn closest_on_segment(a: Vec2, b: Vec2, pt: Vec2) -> Vec2 {
    let ab = b - a;
    let len = ab.len();
    if len <= Fp::from_milli(5) {
        return a;
    }
    let dir = ab.normalize_or_zero();
    a + dir.scale((pt - a).dot(dir).clamp(Fp::ZERO, len))
}

/// Konami US 2010/0240429 teammate yield: a `target` within `r` of the reserved segment `a..b`
/// (the carrier's near-term path or the chaser's run) moves outward along the radial from the
/// nearest path point through the teammate's current position, to `r + l * (1 - d / r)` (continuous
/// at `d = r`, at least `r`). If that radial runs along the path (or off the pitch) the
/// perpendicular on the teammate's side is used instead (the patent's rotation by theta).
#[allow(clippy::too_many_arguments)]
pub fn yield_target(a: Vec2, b: Vec2, target: Vec2, mate_pos: Vec2, r: Fp, l: Fp, half_len: Fp, half_wid: Fp) -> Option<Vec2> {
    yield_target_sided(a, b, target, mate_pos, r, l, half_len, half_wid, 0).map(|(t, _)| t)
}

/// [`yield_target`] with a sticky side (round 4): `sticky` (`+1` / `-1`, `0` = free) is the side of
/// the path (sign of the left-hand perpendicular) chosen earlier in this yield episode; a radial
/// that now points to the other side (the teammate crossed the path line) is replaced by the
/// perpendicular on the sticky side, so the yield cannot flip sides mid-episode. Returns the target
/// and the side used.
#[allow(clippy::too_many_arguments)]
fn yield_target_sided(
    a: Vec2,
    b: Vec2,
    target: Vec2,
    mate_pos: Vec2,
    r: Fp,
    l: Fp,
    half_len: Fp,
    half_wid: Fp,
    sticky: i8,
) -> Option<(Vec2, i8)> {
    let closest = closest_on_segment(a, b, target);
    let d = target.dist(closest);
    if d >= r {
        return None;
    }
    let push = r + l.mul(Fp::ONE - d.div(r));
    let inside = |q: Vec2| q.x.abs() <= half_len - Fp::from_milli(1_500) && q.y.abs() <= half_wid - Fp::from_milli(1_500);
    let clear = |q: Vec2| closest_on_segment(a, b, q).dist(q) >= r - Fp::from_milli(2);
    let path = b - a;
    let path_dir = if path.len() > Fp::from_milli(5) {
        path.normalize_or_zero()
    } else {
        (target - a).normalize_or_zero()
    };
    let perp = Vec2::new(-path_dir.y, path_dir.x);
    let side_of = |v: Vec2| -> i8 { if perp.dot(v) >= Fp::ZERO { 1 } else { -1 } };
    let mut radial = mate_pos - closest;
    if radial.len() < Fp::from_milli(200) {
        radial = target - closest;
    }
    if radial.len() >= Fp::from_milli(200) && (sticky == 0 || side_of(radial) == sticky) {
        let cand = closest + radial.normalize_or_zero().scale(push);
        if clear(cand) && inside(cand) {
            return Some((cand, side_of(radial)));
        }
    }
    // Perpendicular to the path, on the side the teammate is on (lateral fallback), or the sticky side.
    let side_ref = if (mate_pos - a).len() > Fp::from_milli(50) { mate_pos - a } else { target - a };
    let sg = if sticky != 0 { sticky } else { side_of(side_ref) };
    let sgn = Fp::from_int(i32::from(sg));
    let c1 = closest + perp.scale(sgn.mul(push));
    if inside(c1) || perp == Vec2::ZERO {
        return Some((c1, sg));
    }
    Some((closest - perp.scale(sgn.mul(push)), -sg))
}

/// Re-target deadband of the committed target (m): one body radius [estimate].
const RETARGET_DEADBAND: Fp = Fp::from_milli(400);
/// Alpha-beta target filter beta (alpha is the role's low-pass gain, 0.12-0.32 for shape) [estimate].
const TARGET_BETA: Fp = Fp::from_milli(20);
/// A yielding player's filtered target is kept this far (m) beyond the yield radius, so the arrive
/// dead zone does not leave him inside it [estimate].
const YIELD_TARGET_MARGIN: Fp = Fp::from_milli(500);
/// Cap of the filter's target velocity estimate (m per tick; 7.2 m/s) [estimate].
const TARGET_VEL_CAP: Fp = Fp::from_milli(120);
/// The curved-run bend side holds for this many ticks after it was last applied [estimate].
const BEND_HOLD_TICKS: u32 = 30;

/// Cross-cutting target memory (round 4): every intent path that bypasses the shape low-pass (pass
/// receiver, non-open-play drift, teammate contain, loose-ball chase) records the point it moves
/// toward as the player's committed target, so returning to `outfield_shape_intent` continues the
/// low-pass from there instead of snapping, and `reserve_destination` sees a live reservation.
fn note_intent_target(state: &MatchState, side: TeamSide, idx: usize, pt: Vec2, sprint: bool) {
    let Ok(mut mems) = state.ai_memory.try_borrow_mut() else {
        return;
    };
    let m = &mut mems[side.index()];
    if idx < m.target.len() && idx < m.target_tick.len() && idx < m.flags.len() {
        m.target[idx] = pt;
        m.target_tick[idx] = state.tick;
        m.flags[idx] = u8::from(sprint) * FLAG_SPRINT;
    }
}

/// Per-player slice of the round-4 attacking-support memory (`TeamAiMemory::sup_*`, sticky side
/// signs, the alpha-beta target filter), read at the start of `outfield_shape_intent` and written
/// back together with the smoothed target. Run roles are written by the run arbiter only.
#[derive(Clone, Copy, Debug)]
struct UMem {
    sup_kind: u8,
    sup_since: u32,
    sup_eval: u32,
    sup_pt: Vec2,
    sup_carrier: u8,
    sup_nudge: i8,
    yield_sign: i8,
    gk_yield_sign: i8,
    bend_sign: i8,
    bend_tick: u32,
    res_yield: u8,
    tgt_vel: Vec2,
    tgt_raw: Vec2,
    /// The attacking branch ran this tick (otherwise the support commitment is dropped).
    touched: bool,
}

impl UMem {
    fn read(m: &TeamAiMemory, idx: usize) -> Self {
        Self {
            sup_kind: m.sup_kind.get(idx).copied().unwrap_or(AI_NONE),
            sup_since: m.sup_since.get(idx).copied().unwrap_or(0),
            sup_eval: m.sup_eval.get(idx).copied().unwrap_or(0),
            sup_pt: m.sup_pt.get(idx).copied().unwrap_or(Vec2::ZERO),
            sup_carrier: m.sup_carrier.get(idx).copied().unwrap_or(AI_NONE),
            sup_nudge: m.sup_nudge.get(idx).copied().unwrap_or(0),
            yield_sign: m.yield_sign.get(idx).copied().unwrap_or(0),
            gk_yield_sign: m.gk_yield_sign.get(idx).copied().unwrap_or(0),
            bend_sign: m.bend_sign.get(idx).copied().unwrap_or(0),
            bend_tick: m.bend_tick.get(idx).copied().unwrap_or(0),
            res_yield: m.res_yield.get(idx).copied().unwrap_or(AI_NONE),
            tgt_vel: m.tgt_vel.get(idx).copied().unwrap_or(Vec2::ZERO),
            tgt_raw: m.tgt_raw.get(idx).copied().unwrap_or(Vec2::ZERO),
            touched: false,
        }
    }

    /// Adopts support candidate `k` (restarting the dwell when it changes).
    fn set_kind(&mut self, k: u8, tick: u32) {
        self.touched = true;
        if k != self.sup_kind {
            self.sup_kind = k;
            self.sup_since = tick.wrapping_add(1);
        }
    }

    fn write(&self, m: &mut TeamAiMemory, idx: usize) {
        if idx >= m.sup_kind.len() || idx >= m.tgt_raw.len() || idx >= m.res_yield.len() || idx >= m.bend_tick.len() {
            return;
        }
        m.sup_kind[idx] = if self.touched { self.sup_kind } else { AI_NONE };
        m.sup_since[idx] = self.sup_since;
        m.sup_eval[idx] = self.sup_eval;
        m.sup_pt[idx] = self.sup_pt;
        m.sup_carrier[idx] = self.sup_carrier;
        m.sup_nudge[idx] = if self.touched { self.sup_nudge } else { 0 };
        m.yield_sign[idx] = self.yield_sign;
        m.gk_yield_sign[idx] = self.gk_yield_sign;
        m.bend_sign[idx] = self.bend_sign;
        m.bend_tick[idx] = self.bend_tick;
        m.res_yield[idx] = self.res_yield;
        m.tgt_vel[idx] = self.tgt_vel;
        m.tgt_raw[idx] = self.tgt_raw;
    }
}

// --- Scored support candidates (round 4 work area 3, `docs/attacking-support-and-runs.md`) ---

/// Candidate kinds stored in `TeamAiMemory::sup_kind`: the ball-attracted slot, eight PES ring
/// points (ahead, left, right, behind at `L`; the four diagonals at `L sqrt 2`), the nearest opponent
/// Voronoi vertex and the offside-line point.
const SUP_SLOT: u8 = 0;
const SUP_RING_FIRST: u8 = 1;
const SUP_RING_COUNT: u8 = 8;
const SUP_VORONOI: u8 = 9;
const SUP_OFFSIDE: u8 = 10;
/// A near-support point assigned by [`update_near_support`].
const SUP_NEAR: u8 = 11;
/// Ring points considered per supporter besides the incumbent: the four nearest his slot, so a
/// supporter weighs 6-8 candidates (slot + 4-5 ring + vertex + line point) [estimate].
const SUP_RING_PER_PLAYER: usize = 4;
/// A support candidate, once adopted, is held at least this long unless it becomes invalid
/// (rejected / zero score) [estimate; the report's 60-tick commitment].
const SUP_COMMIT_TICKS: u32 = 60;
/// Re-score cadence (ticks): half a lane lifetime of 0.9-1.3 s [peer-reviewed; 15-30 estimate].
pub const SUP_RESCORE_TICKS: u32 = 20;
/// Support Range `L` (mm): 14 m in 11v11 (the 13-24 m support band [peer-reviewed]); 9 m in 5v5
/// on the 56 x 36 m pitch [estimate, pitch-scaled]. Diagonals sit at `L sqrt 2`.
pub const SUP_RING_L_MM: (i64, i64) = (14_000, 9_000);
/// sqrt 2 / 2 in permille (the diagonal components of a point at `L sqrt 2` are `L`).
const HALF_SQRT2_PM: i64 = 707;
/// Reject candidates within this distance (mm) of another teammate's reserved destination unless
/// it is the player's own slot: 10 m [peer-reviewed]; 7.5 m in 5v5 [estimate, pitch-scaled; 6 m let the 5v5 nearest-teammate < 4 m share reach 15%].
const SUP_RESERVE_EXCL_MM: (i64, i64) = (10_000, 7_500);
/// A candidate may also not lie this close (mm) to where another teammate stands now (a teammate
/// whose target is elsewhere still occupies the space until he leaves) [estimate].
const SUP_MATE_POS_SEP_MM: (i64, i64) = (5_000, 5_000);
/// Formation leash (mm): a candidate farther than this from the player's slot is not considered
/// (Konami US 9,180,371's base-position condition) [patent; distance estimate].
const SUP_LEASH_MM: (i64, i64) = (18_000, 10_000);
/// Voronoi vertex search radius around the slot (mm): 25 m [code: Gliders2d]; 15 m in 5v5 [estimate].
const SUP_VORONOI_RANGE_MM: (i64, i64) = (25_000, 15_000);
/// Pass-distance band of a ring / vertex / line candidate (mm): the analytics' option band 6-30 m.
const SUP_BALL_BAND_MM: (i64, i64) = (6_000, 30_000);
/// Average ground-pass speed for the ball's arrival time in the control race (m/s) [estimate; the
/// CPU pass scorer's 15.4 m/s].
const SUP_BALL_SPEED: Fp = Fp::from_int(15);
/// "Chance Creation passing risk" weight: a safe-option bias added to Value before the product;
/// lower values favour runs into dangerous space over safe options [official slider semantics;
/// value estimate].
pub const SUP_SAFE_BIAS: Fp = Fp::from_milli(50);
/// Final-lane check: an opponent within this distance (m) of the carrier -> target segment blocks it
/// (the analytics' open-lane test uses 1.5 m) [estimate].
const SUP_LANE_CLEAR: Fp = Fp::from_milli(1_600);
/// Score factor of a candidate whose straight lane from the carrier is blocked within
/// [`SUP_LANE_CLEAR`] [estimate].
const SUP_BLOCKED_LANE_FACTOR: Fp = Fp::from_milli(350);
/// Lateral step of the final-lane nudge (m) and the largest nudge (steps).
const SUP_NUDGE_STEP: Fp = Fp::from_milli(2_000);
const SUP_NUDGE_MAX: i8 = 2;

/// `(11v11, 5v5)` constant pair selection.
#[inline]
const fn fmt_pick(is_5v5: bool, v: (i64, i64)) -> i64 {
    if is_5v5 {
        v.1
    } else {
        v.0
    }
}

/// The opponents of `side` as space-race movers (keeper included).
fn opp_movers(state: &MatchState, side: TeamSide) -> Vec<crate::space::Mover> {
    state.players[side.opponent().index()]
        .iter()
        .filter(|o| !o.sent_off)
        .map(|o| crate::space::Mover::new(o.pos, o.vel))
        .collect()
}

/// Is `pt` beyond the true offside line? (`onside_x` already includes the 0.35 m margin.)
fn beyond_line(pt: Vec2, onside_x: Fp, attack_sign: i64) -> bool {
    (pt.x - onside_x).mul(Fp::from_int(attack_sign as i32)) > Fp::from_milli(350)
}

/// Geometry of support candidate `kind` for a player whose (ball-attracted) slot is `slot`, with
/// the carrier at `c`; `None` when the kind does not exist now or falls off the pitch. The Voronoi
/// vertex is recomputed only on re-score ticks (`stored` in between), so its point cannot jump
/// with the triangulation.
#[allow(clippy::too_many_arguments)]
fn sup_candidate_point(
    state: &MatchState,
    side: TeamSide,
    kind: u8,
    slot: Vec2,
    c: Vec2,
    stored: Vec2,
    fresh_vertex: bool,
    is_5v5: bool,
) -> Option<Vec2> {
    let attack_sign = side.attack_sign(state.second_half_swapped());
    let hl = state.format.half_length() - Fp::from_milli(2_000);
    let hw = state.format.half_width() - Fp::from_milli(1_500);
    let pt = match kind {
        SUP_SLOT => slot,
        k if (SUP_RING_FIRST..SUP_RING_FIRST + SUP_RING_COUNT).contains(&k) => {
            let l = fmt_pick(is_5v5, SUP_RING_L_MM);
            let s = attack_sign;
            let dg = l * HALF_SQRT2_PM * 1_414 / 1_000_000;
            let (dx, dy) = match k - SUP_RING_FIRST {
                0 => (s * l, 0),
                1 => (0, l),
                2 => (0, -l),
                3 => (-s * l, 0),
                4 => (s * dg, dg),
                5 => (s * dg, -dg),
                6 => (-s * dg, dg),
                _ => (-s * dg, -dg),
            };
            c + Vec2::from_milli(dx, dy)
        }
        SUP_VORONOI => {
            if fresh_vertex {
                let sites: Vec<Vec2> = state.players[side.opponent().index()]
                    .iter()
                    .filter(|o| !o.sent_off)
                    .map(|o| o.pos)
                    .collect();
                let range = Fp::from_milli(fmt_pick(is_5v5, SUP_VORONOI_RANGE_MM));
                crate::space::nearest_voronoi_vertex(&sites, slot, range, hl, hw)?
            } else {
                stored
            }
        }
        SUP_OFFSIDE => {
            if !state.rules.offsides_enabled {
                return None;
            }
            crate::space::offside_line_point(compute_onside_limit_x(state, side), slot, hw)
        }
        _ => return None,
    };
    (pt.x.abs() <= hl && pt.y.abs() <= hw).then_some(pt)
}

/// Score of receiving at `pt` for supporter `idx` (0 = rejected): Control (his time-to-intercept
/// race against every opponent, the ball arriving at 15 m/s from the carrier) x LaneOpen (5 x 5
/// pass window) x (Value + safe bias) x pressure factor (4.6 m). Rejected: off the leash, outside
/// the 6-30 m pass band (the slot excepted), within the reservation exclusion of a teammate's live
/// destination (the slot excepted), or offside.
#[allow(clippy::too_many_arguments)]
fn sup_score(
    state: &MatchState,
    side: TeamSide,
    idx: usize,
    kind: u8,
    pt: Vec2,
    slot: Vec2,
    c: Vec2,
    defs: &[crate::space::Mover],
    mate_targets: &[Vec2],
    mate_target_ticks: &[u32],
    chaser: u8,
    is_5v5: bool,
) -> Fp {
    let attack_sign = side.attack_sign(state.second_half_swapped());
    let d_ball = pt.dist(c);
    if kind == SUP_SLOT {
        if d_ball < Fp::from_milli(3_000) {
            return Fp::ZERO;
        }
    } else {
        if pt.dist(slot) > Fp::from_milli(fmt_pick(is_5v5, SUP_LEASH_MM)) {
            return Fp::ZERO;
        }
        if d_ball < Fp::from_milli(SUP_BALL_BAND_MM.0) || d_ball > Fp::from_milli(SUP_BALL_BAND_MM.1) {
            return Fp::ZERO;
        }
        let excl = Fp::from_milli(fmt_pick(is_5v5, SUP_RESERVE_EXCL_MM));
        let carrier = state.ball.owner.and_then(|(os, oi)| (os == side).then_some(oi));
        for (j, mt) in mate_targets.iter().enumerate().skip(1) {
            if j == idx || Some(j) == carrier || j as u8 == chaser || state.players[side.index()].get(j).is_none_or(|m| m.sent_off) {
                continue;
            }
            // A teammate with no live reservation (the human-controlled player, a stale target)
            // reserves where he stands.
            let tt = mate_target_ticks.get(j).copied().unwrap_or(0);
            let live = tt != 0 && state.tick.wrapping_sub(tt) <= 1;
            let reserved = if live { *mt } else { state.players[side.index()][j].pos };
            if pt.dist(reserved) < excl
                || pt.dist(state.players[side.index()][j].pos) < Fp::from_milli(fmt_pick(is_5v5, SUP_MATE_POS_SEP_MM))
            {
                return Fp::ZERO;
            }
        }
    }
    if state.rules.offsides_enabled && beyond_line(pt, compute_onside_limit_x(state, side), attack_sign) {
        return Fp::ZERO;
    }
    let p = &state.players[side.index()][idx];
    let me = crate::space::Mover::new(p.pos, p.vel);
    let ctl = crate::space::control(pt, d_ball.div(SUP_BALL_SPEED), &[me], defs);
    if ctl <= Fp::from_milli(20) {
        return Fp::ZERO;
    }
    let mut lane = crate::space::lane_open(c, pt, &crate::space::Mover::new(pt, Vec2::ZERO), defs);
    if !segment_clear(state, side, c, pt) {
        // A body in the straight lane: the pass needs a bend the window samples may not see.
        lane = lane.mul(SUP_BLOCKED_LANE_FACTOR);
    }
    let goal = Vec2::new(Fp::from_milli(attack_sign * state.format.half_length().to_milli()), Fp::ZERO);
    let value = crate::space::value_at(pt, goal) + SUP_SAFE_BIAS;
    ctl.mul(lane).mul(value).mul(crate::space::pressure_factor(pt, defs))
}

/// Scored support target for supporter `idx` while his side owns the ball (round 4). Re-scores
/// every [`SUP_RESCORE_TICKS`] (or at once when the carrier changes): the slot, the four ring
/// points nearest the slot (plus the incumbent), the nearest opponent Voronoi vertex and the
/// offside-line point; the incumbent stays for [`SUP_COMMIT_TICKS`] and then until a
/// challenger scores 1.15x higher (or at once when the incumbent is rejected). Between re-scores the committed
/// kind's geometry is followed. During our pass in flight the last committed point is held.
/// Positions are read as they stand when the player is processed (players stepped earlier this
/// tick have moved), which is fixed by the stepping order and therefore deterministic.
/// `None`: no scored candidate (GRF spacing fallback).
#[allow(clippy::too_many_arguments)]
fn support_candidate_target(
    state: &MatchState,
    side: TeamSide,
    idx: usize,
    slot: Vec2,
    um: &mut UMem,
    mate_targets: &[Vec2],
    mate_target_ticks: &[u32],
    chaser: u8,
    is_5v5: bool,
) -> Option<Vec2> {
    let stamp = state.tick.wrapping_add(1);
    um.touched = true;
    let Some((carrier, c)) = support_focus(state, side) else {
        // Loose ball (or the keeper on the ball): hold the last committed point for a while.
        let fresh = um.sup_kind != AI_NONE && stamp.wrapping_sub(um.sup_eval) <= 2 * SUP_RESCORE_TICKS;
        return fresh.then_some(um.sup_pt);
    };
    if carrier == idx {
        return None;
    }
    let due = um.sup_carrier != carrier as u8 || um.sup_kind == SUP_NEAR || stamp.wrapping_sub(um.sup_eval) >= SUP_RESCORE_TICKS;
    if !due {
        if um.sup_kind == AI_NONE {
            return None;
        }
        let pt = sup_candidate_point(state, side, um.sup_kind, slot, c, um.sup_pt, false, is_5v5);
        um.sup_pt = pt.unwrap_or(um.sup_pt);
        return pt;
    }
    // Re-score.
    let defs = opp_movers(state, side);
    let mut ring: Vec<(Fp, u8)> = (SUP_RING_FIRST..SUP_RING_FIRST + SUP_RING_COUNT)
        .filter_map(|k| sup_candidate_point(state, side, k, slot, c, Vec2::ZERO, false, is_5v5).map(|q| (q.dist(slot), k)))
        .collect();
    ring.sort_unstable();
    let mut kinds: Vec<u8> = vec![SUP_SLOT];
    kinds.extend(ring.iter().take(SUP_RING_PER_PLAYER).map(|&(_, k)| k));
    if (SUP_RING_FIRST..SUP_RING_FIRST + SUP_RING_COUNT).contains(&um.sup_kind) && !kinds.contains(&um.sup_kind) {
        kinds.push(um.sup_kind);
    }
    kinds.push(SUP_VORONOI);
    kinds.push(SUP_OFFSIDE);
    let same_carrier = um.sup_carrier == carrier as u8;
    let score = |k: u8, pt: Vec2| sup_score(state, side, idx, k, pt, slot, c, &defs, mate_targets, mate_target_ticks, chaser, is_5v5);
    let mut best: Option<(Fp, u8, Vec2)> = None;
    let mut inc: Option<(Fp, Vec2)> = None;
    for &k in &kinds {
        let Some(pt) = sup_candidate_point(state, side, k, slot, c, um.sup_pt, true, is_5v5) else {
            continue;
        };
        let sc = score(k, pt);
        if same_carrier && k == um.sup_kind {
            // The incumbent vertex is the stored one; the fresh vertex is its challenger.
            inc = Some(if k == SUP_VORONOI { (score(k, um.sup_pt), um.sup_pt) } else { (sc, pt) });
        }
        if sc > Fp::ZERO && best.is_none_or(|(bs, _, _)| sc > bs) {
            best = Some((sc, k, pt));
        }
    }
    um.sup_eval = stamp;
    um.sup_carrier = carrier as u8;
    let held = stamp.wrapping_sub(um.sup_since);
    let pick = match (inc, best) {
        (Some((isc, ipt)), Some((bs, bk, bpt))) if isc > Fp::ZERO => {
            let beats = bs > isc.mul(Fp::from_milli(INCUMBENT_BONUS_PM));
            let may_switch = held >= SUP_COMMIT_TICKS || bk == um.sup_kind;
            if may_switch && beats {
                (bk, bpt)
            } else {
                (um.sup_kind, ipt)
            }
        }
        (Some((isc, ipt)), None) if isc > Fp::ZERO => (um.sup_kind, ipt),
        (_, Some((_, bk, bpt))) => (bk, bpt),
        (_, None) => {
            um.set_kind(AI_NONE, state.tick);
            return None;
        }
    };
    um.set_kind(pick.0, state.tick);
    um.sup_pt = pick.1;
    Some(pick.1)
}

/// No opponent of `side` within [`SUP_LANE_CLEAR`] of the segment `c -> q` (past the first 0.5 m).
fn segment_clear(state: &MatchState, side: TeamSide, c: Vec2, q: Vec2) -> bool {
    let d = q - c;
    let len = d.len();
    let dir = d.normalize_or_zero();
    state.players[side.opponent().index()].iter().filter(|o| !o.sent_off).all(|o| {
        let rel = o.pos - c;
        let proj = rel.dot(dir);
        let perp = if proj >= len { o.pos.dist(q) } else { (rel - dir.scale(proj)).len() };
        proj <= Fp::from_milli(500) || perp >= SUP_LANE_CLEAR
    })
}

/// Final-lane check (round 4): the lane from the carrier to the FINAL target (after spacing,
/// reservation and run push) must be clear of opponents by [`SUP_LANE_CLEAR`]; on re-score ticks a
/// blocked lane is reopened by a lateral nudge of up to 4 m (2 m steps; the current nudge is tried
/// first, then none, then the others), and the nudge is held between re-scores (sticky).
fn final_lane_nudge(state: &MatchState, side: TeamSide, t: Vec2, um: &mut UMem) -> Vec2 {
    let Some((_, ci)) = state.ball.owner.filter(|&(os, _)| os == side) else {
        return t;
    };
    let c = state.players[side.index()][ci].pos;
    let lane = t - c;
    if lane.len() < Fp::from_milli(3_000) {
        um.sup_nudge = 0;
        return t;
    }
    let u = lane.normalize_or_zero();
    let perp = Vec2::new(-u.y, u.x);
    let at = |n: i8| t + perp.scale(SUP_NUDGE_STEP.mul(Fp::from_int(i32::from(n))));
    let hl = state.format.half_length() - Fp::from_milli(1_500);
    let hw = state.format.half_width() - Fp::from_milli(1_500);
    let clear = |q: Vec2| q.x.abs() <= hl && q.y.abs() <= hw && segment_clear(state, side, c, q);
    // Only on re-score ticks (the scorer just ran) may the nudge change.
    if um.sup_eval == state.tick.wrapping_add(1) {
        let cur = um.sup_nudge;
        let mut order: Vec<i8> = vec![cur, 0];
        for n in 1..=SUP_NUDGE_MAX {
            order.push(n);
            order.push(-n);
        }
        um.sup_nudge = order.into_iter().find(|&n| clear(at(n))).unwrap_or(0);
    }
    at(um.sup_nudge)
}

// --- Runs: one arbiter per team (round 4 work area 3, `docs/attacking-support-and-runs.md`) ---

/// Run roles stored in `TeamAiMemory::run_role` (`0` = none).
pub const RUN_NONE: u8 = 0;
/// The one run in behind (Queen in B-Human's role split): end-point beyond the offside line.
pub const RUN_PENETRATE: u8 = 1;
/// The one support-from-behind runner: arrives late a few metres behind the line.
pub const RUN_SUPPORT: u8 = 2;
/// Up to three box runners on a cross (near post, back post, centre).
pub const RUN_BOX: u8 = 3;
/// A decoy whose marker follows him and so frees space for a teammate.
pub const RUN_DECOY: u8 = 4;
/// A run called by the human carrier (`BTN_CALL_RUN`).
pub const RUN_CALLED: u8 = 5;

/// The through ball to a committed runner is safe when the runner's control of the landing point
/// (he is moving toward it at the release) exceeds this [estimate].
const RUNNER_PASS_CONTROL_MIN: Fp = Fp::from_milli(700);
/// "Killer ball" bonus of a safe through ball to a committed runner when the carrier faces forward
/// and is not tight-pressed, so it beats a sideways recycle [estimate; FIFA CC passing [official]].
const RUNNER_KILLER_BONUS: Fp = Fp::from_milli(4_000);
/// Arbiter re-score cadence (ticks) [estimate; the support cadence].
const RUN_RESCORE_TICKS: u32 = 20;
/// Minimum commitment of a new run (ticks) [estimate; FIFA 13 "he sticks with it" [official]].
pub const RUN_COMMIT_TICKS: u32 = 60;
/// A committed run that is never released expires after this many ticks [estimate].
const RUN_MAX_TICKS: u32 = 240;
/// Commitment of a called run (ticks): the existing 1.5 s `BTN_CALL_RUN` burst.
pub const RUN_CALLED_TICKS: u32 = 90;
/// Carrier pressure gate of the run trigger (m): no defender within 4.6 m [peer-reviewed]; for the
/// CPU carrier the sim's 1.85 m "tight" radius [audit].
const RUN_PRESSURE_RADIUS: Fp = Fp::from_milli(4_600);
const RUN_PRESSURE_RADIUS_CPU: Fp = Fp::from_milli(1_850);
/// Run trigger: the runner's 1 s look-ahead point must have Control above 0.5 [estimate].
const RUN_CONTROL_MIN: Fp = Fp::HALF;
/// Look-ahead horizon (s) and the speed above which it is used (m/s) [peer-reviewed].
const LOOKAHEAD_S: Fp = Fp::ONE;
const LOOKAHEAD_MIN_SPEED: Fp = Fp::from_int(2);
/// A runner holding at the line "stutters" at no more than 3 m/s, as a fraction of the ~7 m/s top
/// speed the move vector is scaled to [official FIFA 13; speed estimate].
const RUN_STUTTER_FRAC: Fp = Fp::from_milli(430);
/// Within this distance (m) of the onside limit the held runner stutters.
const RUN_STUTTER_ZONE: Fp = Fp::from_milli(2_500);
/// The held runner goes when the CPU carrier's remaining pass charge is at most this many ticks
/// (offside is judged at the kick, so the runner may not cross the line before it) [estimate].
const RUN_RELEASE_LEAD_TICKS: u32 = 3;
/// A CPU release lasts this long (ticks) while the pass is charged [estimate].
const RUN_RELEASE_TTL: u32 = 40;
/// Forecast speed of a timed runner toward a candidate end-point (m/s): the run-up he has at the
/// release [estimate; EA forecast sites [patent]].
const RUN_FORECAST_SPEED: Fp = Fp::from_int(5);
/// Sprint speed used for the post-release look-ahead (m/s) [estimate].
const RUN_RELEASE_SPEED_MPS: i64 = 7;
/// The support-from-behind runner aims this far (mm) behind the onside limit [estimate].
const RUN_SUPPORT_DEPTH_MM: (i64, i64) = (6_000, 4_000);
/// End-point search radius around the runner (mm) [code: Gliders2d 25 m; estimate for 5v5].
const RUN_END_RANGE_MM: (i64, i64) = (25_000, 14_000);
/// Grid fallback end-points: depth beyond the true line (mm) and lateral offsets (mm).
const RUN_END_DEPTHS_MM: [i64; 2] = [5_000, 9_000];
const RUN_END_LATERAL_MM: [i64; 3] = [-6_000, 0, 6_000];
/// Decoy: a marker within 3 m moving with the runner; the decoy is worth it when removing that
/// marker raises a teammate's Control by more than 0.1 [estimate].
const DECOY_MARKER_RADIUS: Fp = Fp::from_milli(3_000);
const DECOY_GAIN_MIN: Fp = Fp::from_milli(100);
/// Cross situation: carrier beyond 60% of the half, wider than the box half-width minus 2 m.
const CROSS_PROG_PM: i64 = 600;
/// Box runner spots `(depth from the goal line, y toward the ball side)` (mm): near post, back
/// post, centre (FIFA 09 Crossing slider [official]; positions estimate). 5v5 scaled.
const BOX_SPOTS_MM: [(i64, i64); 3] = [(5_500, 2_000), (6_000, -3_500), (8_500, 0)];
const BOX_SPOTS_5V5_MM: [(i64, i64); 3] = [(4_000, 1_500), (4_500, -2_500), (6_000, 0)];
/// "Crossing" box-runner count (FIFA 09 slider: how many players get into the box) [official].
pub const BOX_RUNNERS_MAX: usize = 3;

/// `(rest_role, runner_role)` of the player in slot `slot_k`: the rest-defence player of the
/// attacking shape (fixo, or the most defensive 5v5 slot) and the roles that make forward runs.
fn attack_roles(ctx: &ShapeCtx, slot_k: u8, p: &PlayerState) -> (bool, bool) {
    let slot = ctx.slot(slot_k);
    let role = slot.map(|s| s.role);
    let rest_role = match role {
        Some(SlotRole::Fixo) => true,
        Some(_) if ctx.is_5v5 => {
            // Formations without a fixo: the most defensive slot keeps rest defence.
            let min_depth = ctx.slots.iter().map(|s| s.attack_depth).min().unwrap_or(0);
            slot.is_some_and(|s| s.attack_depth == min_depth)
                && ctx.slots.iter().position(|s| s.attack_depth == min_depth) == Some(slot_k as usize)
        }
        _ => false,
    };
    let runner_role = matches!(
        role,
        Some(SlotRole::Striker | SlotRole::Pivo | SlotRole::Winger | SlotRole::Ala | SlotRole::WideMid)
    ) || p.role == PlayerRole::Forward;
    (rest_role, runner_role)
}

/// Attacking work-rate cadence: a High (>= 67) work rate considers a run every arbiter round,
/// Medium every second, Low (< 34) every fourth (FIFA's "how often the player considers making
/// forward runs" [official]; cadence [estimate]).
const fn work_rate_cadence(att: u8) -> u32 {
    if att >= 67 {
        1
    } else if att >= 34 {
        2
    } else {
        4
    }
}

/// The 1 s look-ahead position of `p` (his position when he moves at 2 m/s or less).
fn lookahead(p: &PlayerState) -> Vec2 {
    if p.vel.len() > LOOKAHEAD_MIN_SPEED {
        p.pos + p.vel.scale(LOOKAHEAD_S)
    } else {
        p.pos
    }
}

/// The owned point beyond the offside line for runner `p` (EA US 7,588,493: the Voronoi vertex
/// beyond the line the runner reaches first; Gliders2d grid points as a fallback): among the
/// points he reaches before every opponent (time-to-intercept), the one maximising Control (the
/// runner released, against every opponent) x Value. Returns `(point, control x value)`.
fn run_end_point(state: &MatchState, side: TeamSide, p: &PlayerState, c: Vec2, defs: &[crate::space::Mover]) -> Option<(Vec2, Fp)> {
    let attack_sign = side.attack_sign(state.second_half_swapped());
    let is_5v5 = state.format.players_per_side <= 6;
    let s = Fp::from_int(attack_sign as i32);
    let hl = state.format.half_length() - Fp::from_milli(3_000);
    // End-points stay infield (6 m from the touchline, 3 m in 5v5): a through ball along the
    // touchline runs out of play or away from the runner [estimate].
    let hw = state.format.half_width() - Fp::from_milli(if is_5v5 { 3_000 } else { 6_000 });
    let line = compute_onside_limit_x(state, side) + s.mul(Fp::from_milli(350));
    let goal = Vec2::new(s.mul(state.format.half_length()), Fp::ZERO);
    let range = Fp::from_milli(fmt_pick(is_5v5, RUN_END_RANGE_MM));
    let sites: Vec<Vec2> = defs.iter().map(|d| d.pos).collect();
    let mut cands = crate::space::opponent_voronoi_vertices(&sites, hl, hw);
    for &dd in &RUN_END_DEPTHS_MM {
        for &dy in &RUN_END_LATERAL_MM {
            let depth = if is_5v5 { dd * 6 / 10 } else { dd };
            let lat = if is_5v5 { dy * 6 / 10 } else { dy };
            cands.push(Vec2::new(line + s.mul(Fp::from_milli(depth)), p.pos.y + Fp::from_milli(lat)));
        }
    }
    let mut best: Option<(Vec2, Fp)> = None;
    for q in cands {
        let beyond = (q.x - line).mul(s);
        if beyond < Fp::from_milli(1_500) || q.x.abs() > hl || q.y.abs() > hw || q.dist(p.pos) > range {
            continue;
        }
        // The runner is forecast already moving toward the point at the timed-run speed (EA's
        // forecast sites: "offsetting the player's position forward ... in the direction of
        // motion, and in proportion to the player's speed" [patent]); defenders as they are.
        let me = crate::space::Mover::new(p.pos, (q - p.pos).normalize_or_zero().scale(RUN_FORECAST_SPEED));
        // The runner must reach the point first (EA's rule: "he can get to that piece of space
        // before anyone else"); Control then weights how securely.
        let t_me = crate::space::time_to_intercept(&me, q);
        if defs.iter().any(|d| crate::space::time_to_intercept(d, q) <= t_me) {
            continue;
        }
        let ctl = crate::space::control(q, q.dist(c).div(SUP_BALL_SPEED), &[me], defs);
        let sc = ctl.mul(crate::space::value_at(q, goal));
        if best.is_none_or(|(_, bs)| sc > bs) {
            best = Some((q, sc));
        }
    }
    best
}

/// Is the human or CPU carrier of `side` under no more than moderate pressure?
fn carrier_unpressured(state: &MatchState, side: TeamSide, c: Vec2) -> bool {
    let r = if side_is_cpu(state, side) { RUN_PRESSURE_RADIUS_CPU } else { RUN_PRESSURE_RADIUS };
    state.players[side.opponent().index()]
        .iter()
        .filter(|o| !o.sent_off)
        .all(|o| o.pos.dist(c) > r)
}

/// The team's run arbiter (round 4): one decision per team per tick, computed lazily by the first
/// supporter processed this tick (stamped `run_eval`, idempotent like the team plan), on positions
/// as they stand at that moment. Ends runs on possession loss or a stoppage, on expiry, and when a
/// runner receives the ball; every [`RUN_RESCORE_TICKS`] it re-assigns at most one penetrate runner
/// and one support-from-behind runner (highest and second score), up to three box runners on a
/// cross and one decoy, with a [`RUN_COMMIT_TICKS`] commitment and the 1.15x incumbent margin.
fn update_run_arbiter(state: &MatchState, side: TeamSide) {
    let stamp = state.tick.wrapping_add(1);
    {
        let Ok(mems) = state.ai_memory.try_borrow() else {
            return;
        };
        if mems[side.index()].run_eval == stamp {
            return;
        }
    }
    // Read-only inputs first (the Schmitt trigger and the plan borrow the memory themselves).
    let passer_ready = passer_ready_schmitt(state, side);
    let ctx = ShapeCtx::new(state, side);
    let is_5v5 = ctx.is_5v5;
    let attack_sign = ctx.sign;
    let s = Fp::from_int(attack_sign as i32);
    let players = &state.players[side.index()];
    let n = players.len();
    let open = matches!(state.phase, MatchPhase::OpenPlay);
    let carrier = state.ball.owner.and_then(|(os, oi)| (os == side).then_some(oi));
    let opp_owns = state.ball.owner.is_some_and(|(os, _)| os != side);
    let our_pass_to = state.ball.pass_in_flight.as_ref().filter(|pf| pf.team == side).map(|pf| pf.intended_receiver_idx);
    let Ok(mut mems) = state.ai_memory.try_borrow_mut() else {
        return;
    };
    let m = &mut mems[side.index()];
    m.ensure_len(n);
    m.run_eval = stamp;
    // 1. Ends: stoppage, possession loss (everyone); receiving the ball, sent off, expiry (one).
    for (j, pj) in players.iter().enumerate() {
        let role = m.run_role[j];
        if role == RUN_NONE {
            continue;
        }
        let age = stamp.wrapping_sub(m.run_since[j]);
        let limit = if role == RUN_CALLED { RUN_CALLED_TICKS } else { RUN_MAX_TICKS };
        // A loose ball (no owner, no pass of ours in flight) ends every run but a box run: the
        // pass to the runner failed or the carrier lost it; the chase and shape take over.
        let loose = state.ball.owner.is_none() && our_pass_to.is_none();
        let lost = !open || opp_owns || (loose && role != RUN_BOX);
        let received = carrier == Some(j);
        if lost || received || pj.sent_off || age >= limit {
            m.run_role[j] = RUN_NONE;
        }
    }
    if m.run_release != 0 && stamp.wrapping_sub(m.run_release_tick) > RUN_RELEASE_TTL {
        m.run_release = 0;
    }
    let Some(ci) = carrier.filter(|_| open) else {
        return;
    };
    // While our pass is in flight nothing is re-assigned (the receiver path owns the receiver).
    if our_pass_to.is_some() {
        return;
    }
    if !state.tick.is_multiple_of(RUN_RESCORE_TICKS) {
        return;
    }
    m.run_round = m.run_round.wrapping_add(1);
    let round = m.run_round;
    let c = players[ci].pos;
    let defs = opp_movers(state, side);
    let onside_x = compute_onside_limit_x(state, side);
    let half_len = state.format.half_length();
    let half_wid = state.format.half_width();
    let committed = |m: &TeamAiMemory, j: usize| m.run_role[j] != RUN_NONE && stamp.wrapping_sub(m.run_since[j]) < RUN_COMMIT_TICKS;
    // 2. Box runners on a cross.
    let cross = (c.x.mul(s)) > half_len.mul(Fp::from_milli(CROSS_PROG_PM))
        && c.y.abs() > state.format.penalty_box_half_width() - Fp::from_milli(2_000);
    for j in 0..n {
        if m.run_role[j] == RUN_BOX && !cross && !committed(m, j) {
            m.run_role[j] = RUN_NONE;
        }
    }
    if cross {
        let spots = if is_5v5 { BOX_SPOTS_5V5_MM } else { BOX_SPOTS_MM };
        let ball_side = if c.y >= Fp::ZERO { 1 } else { -1 };
        let goal_x = s.mul(half_len);
        let mut taken: Vec<usize> = (0..n).filter(|&j| m.run_role[j] == RUN_BOX).collect();
        for &(depth, lat) in spots.iter().take(BOX_RUNNERS_MAX) {
            if taken.len() >= BOX_RUNNERS_MAX {
                break;
            }
            let spot = Vec2::new(goal_x - s.mul(Fp::from_milli(depth)), Fp::from_milli(lat * ball_side));
            let spot = Vec2::new(if attack_sign > 0 { spot.x.min(onside_x) } else { spot.x.max(onside_x) }, spot.y);
            if taken.iter().any(|&j| m.run_end[j].dist(spot) < Fp::from_milli(1_000)) {
                continue;
            }
            let mut best: Option<(Fp, usize)> = None;
            for (j, p) in players.iter().enumerate().skip(1) {
                if j == ci || p.sent_off || m.run_role[j] != RUN_NONE {
                    continue;
                }
                let (rest, runner) = attack_roles(&ctx, m.slot_of.get(j).copied().unwrap_or(AI_NONE), p);
                let mid = p.role == PlayerRole::Midfielder;
                if rest || !(runner || mid) {
                    continue;
                }
                let d = p.pos.dist(spot);
                if best.is_none_or(|(bd, _)| d < bd) {
                    best = Some((d, j));
                }
            }
            if let Some((_, j)) = best {
                m.run_role[j] = RUN_BOX;
                m.run_since[j] = stamp;
                m.run_end[j] = spot;
                taken.push(j);
            }
        }
        return;
    }
    // 3. Penetrate and support-from-behind runners.
    let incumbent_pen = (0..n).find(|&j| m.run_role[j] == RUN_PENETRATE);
    let incumbent_sup = (0..n).find(|&j| m.run_role[j] == RUN_SUPPORT);
    let trigger = passer_ready && carrier_unpressured(state, side, c);
    let mut scored: Vec<(Fp, usize, Vec2)> = Vec::new();
    if trigger {
        for (j, p) in players.iter().enumerate().skip(1) {
            if j == ci || p.sent_off || matches!(m.run_role[j], RUN_BOX | RUN_CALLED | RUN_DECOY) {
                continue;
            }
            let (rest, runner) = attack_roles(&ctx, m.slot_of.get(j).copied().unwrap_or(AI_NONE), p);
            if rest || !runner {
                continue;
            }
            let incumbent = Some(j) == incumbent_pen || Some(j) == incumbent_sup;
            let cadence = work_rate_cadence(p.attributes.work_rates(p.role).0);
            if !incumbent && !(round.wrapping_add(j as u32)).is_multiple_of(cadence) {
                continue;
            }
            // Onside (judged against the true line), and Control > 0.5 at his 1 s look-ahead.
            if state.rules.offsides_enabled && beyond_line(p.pos, onside_x, attack_sign) {
                continue;
            }
            let la = lookahead(p);
            let me = crate::space::Mover::new(p.pos, p.vel);
            if crate::space::control(la, la.dist(c).div(SUP_BALL_SPEED), &[me], &defs) <= RUN_CONTROL_MIN {
                continue;
            }
            if let Some((end, sc)) = run_end_point(state, side, p, c, &defs) {
                scored.push((sc, j, end));
            }
        }
    }
    scored.sort_unstable_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    let assign = |m: &mut TeamAiMemory, role: u8, incumbent: Option<usize>, pick: Option<(Fp, usize, Vec2)>| -> Option<usize> {
        if let Some(ij) = incumbent {
            let inc_sc = scored.iter().find(|e| e.1 == ij).map(|e| e.0);
            let keep = committed(m, ij)
                || match (inc_sc, pick) {
                    (Some(isc), Some((bs, bj, _))) => bj == ij || bs <= isc.mul(Fp::from_milli(INCUMBENT_BONUS_PM)),
                    (Some(_), None) => true,
                    (None, _) => false,
                };
            if keep {
                if let Some(e) = scored.iter().find(|e| e.1 == ij) {
                    if role == RUN_PENETRATE {
                        m.run_end[ij] = e.2;
                    }
                }
                return Some(ij);
            }
            m.run_role[ij] = RUN_NONE;
        }
        let (_, j, end) = pick?;
        m.run_role[j] = role;
        m.run_since[j] = stamp;
        m.run_end[j] = end;
        Some(j)
    };
    let pen = assign(m, RUN_PENETRATE, incumbent_pen, scored.first().copied());
    let sup_pick = scored.iter().find(|e| Some(e.1) != pen).copied().map(|(sc, j, end)| {
        let depth = Fp::from_milli(fmt_pick(is_5v5, RUN_SUPPORT_DEPTH_MM));
        (sc, j, Vec2::new(onside_x - s.mul(depth), end.y.mul(Fp::from_milli(600))))
    });
    let incumbent_sup = incumbent_sup.filter(|&j| Some(j) != pen && m.run_role[j] == RUN_SUPPORT);
    let _sup = assign(m, RUN_SUPPORT, incumbent_sup, sup_pick);
    // 4. Decoy: a forward whose marker moves with him and whose drag frees a teammate's target.
    if !trigger || (0..n).any(|j| m.run_role[j] == RUN_DECOY) {
        return;
    }
    let mut decoy: Option<(usize, Vec2)> = None;
    'cand: for (j, p) in players.iter().enumerate().skip(1) {
        if j == ci || p.sent_off || m.run_role[j] != RUN_NONE {
            continue;
        }
        let (rest, runner) = attack_roles(&ctx, m.slot_of.get(j).copied().unwrap_or(AI_NONE), p);
        if rest || !runner || (state.rules.offsides_enabled && beyond_line(p.pos, onside_x, attack_sign)) {
            continue;
        }
        let Some(mk) = defs.iter().position(|d| d.pos.dist(p.pos) < DECOY_MARKER_RADIUS && d.vel.dot(p.vel) >= Fp::ZERO) else {
            continue;
        };
        let end = Vec2::new(onside_x, (p.pos.y + s.mul(Fp::from_milli(2_000))).clamp(-half_wid, half_wid));
        let mut dragged = defs.clone();
        dragged[mk].pos += end - p.pos;
        for (k, mate) in players.iter().enumerate().skip(1) {
            if k == j || k == ci || mate.sent_off {
                continue;
            }
            let tgt = m.target.get(k).copied().unwrap_or(mate.pos);
            let mm = crate::space::Mover::new(mate.pos, mate.vel);
            let bt = tgt.dist(c).div(SUP_BALL_SPEED);
            let before = crate::space::control(tgt, bt, &[mm], &defs);
            let after = crate::space::control(tgt, bt, &[mm], &dragged);
            if after - before > DECOY_GAIN_MIN {
                decoy = Some((j, end));
                break 'cand;
            }
        }
    }
    if let Some((j, end)) = decoy {
        m.run_role[j] = RUN_DECOY;
        m.run_since[j] = stamp;
        m.run_end[j] = end;
    }
}

/// Is runner `idx` released? Our pass launched toward him (step 1 of the intent already meets it),
/// or the CPU carrier's pass decision selected him and the pass charge is nearly complete.
fn run_released(state: &MatchState, side: TeamSide, idx: usize, release: u8) -> bool {
    if state.ball.pass_in_flight.as_ref().is_some_and(|pf| pf.team == side && pf.intended_receiver_idx == idx) {
        return true;
    }
    if release as usize != idx || idx == 0 {
        return false;
    }
    let Some((_, ci)) = state.ball.owner.filter(|&(os, _)| os == side) else {
        return false;
    };
    match state.players[side.index()][ci].action {
        PlayerActionState::Charging { button, ticks, .. } => {
            let planned = state.cpu_target_charge[side.index()] as u16;
            let target = charge_units_to_ticks(if planned >= 4 { planned } else { 9 }, button);
            u32::from(target.saturating_sub(ticks)) <= RUN_RELEASE_LEAD_TICKS
        }
        _ => false,
    }
}

/// The carrier's facing cone for a called run: within 45 degrees of his facing (FIFA's L1 "the
/// teammate you are facing" [official]; cone estimate).
const CALL_RUN_CONE_COS: Fp = Fp::from_milli(707);

/// `BTN_CALL_RUN` (round 4): among the outfielders other than the keeper and the controlled
/// carrier, the teammate inside the carrier's facing cone with the best Control at his 1 s
/// look-ahead (onside players first; if nobody is inside the cone, the one most in line with the
/// facing) becomes a [`RUN_CALLED`] runner with an end-point beyond the offside line (or, with no
/// owned point there, 9 m ahead of him toward goal), committed for [`RUN_CALLED_TICKS`] with the
/// same hold-and-release as an arbiter run. Writes the run into the team memory and returns the
/// runner, for the caller to set his `run_boost_ticks`.
pub(crate) fn assign_called_run(state: &MatchState, side: TeamSide) -> Option<usize> {
    let s_idx = side.index();
    let ctrl = state.controlled_idx[s_idx];
    let players = &state.players[s_idx];
    let carrier = players.get(ctrl)?;
    let facing = Vec2::from_bam(carrier.facing_bam, Fp::ONE);
    let attack_sign = side.attack_sign(state.second_half_swapped());
    let onside_x = compute_onside_limit_x(state, side);
    let defs = opp_movers(state, side);
    let c = carrier.pos;
    // (in cone, onside, control or alignment) ordered lexicographically.
    let mut best: Option<((bool, bool, Fp), usize)> = None;
    for (j, p) in players.iter().enumerate() {
        if j == 0 || j == ctrl || p.sent_off {
            continue;
        }
        let dir = (p.pos - c).normalize_or_zero();
        let align = dir.dot(facing);
        let in_cone = align >= CALL_RUN_CONE_COS;
        let onside = !state.rules.offsides_enabled || !beyond_line(p.pos, onside_x, attack_sign);
        let la = lookahead(p);
        let me = crate::space::Mover::new(p.pos, p.vel);
        let ctl = crate::space::control(la, la.dist(c).div(SUP_BALL_SPEED), &[me], &defs);
        let key = (in_cone, onside, if in_cone { ctl } else { align });
        if best.is_none_or(|(bk, _)| key > bk) {
            best = Some((key, j));
        }
    }
    let (_, j) = best?;
    let p = &players[j];
    let s = Fp::from_int(attack_sign as i32);
    let end = run_end_point(state, side, p, c, &defs).map_or_else(
        || {
            let hl = state.format.half_length() - Fp::from_milli(3_000);
            let x = (p.pos.x + s.mul(Fp::from_int(9))).clamp(-hl, hl);
            Vec2::new(x, p.pos.y.mul(Fp::from_milli(850)))
        },
        |(e, _)| e,
    );
    let mut mems = state.ai_memory.try_borrow_mut().ok()?;
    let m = &mut mems[s_idx];
    m.ensure_len(players.len());
    for r in m.run_role.iter_mut() {
        if *r == RUN_CALLED {
            *r = RUN_NONE;
        }
    }
    m.run_role[j] = RUN_CALLED;
    m.run_since[j] = state.tick.wrapping_add(1);
    m.run_end[j] = end;
    Some(j)
}

// --- Near support: "players closest to the ball will come to support" (FIFA build-up [official]) ---

/// Near-support candidate radii around the carrier (mm): 9-17 m in 11v11, 8-12.5 m in 5v5 (5v5 raised from 6.5 m so the pair and the carrier do not bunch) [estimate].
const NEAR_RADII_MM: [(i64, i64); 3] = [(9_000, 8_000), (13_000, 10_000), (17_000, 12_500)];
/// Bearings sampled around the carrier (30 degrees apart): `(cos, sin)` in permille.
const NEAR_BEARINGS_PM: [(i64, i64); 12] = [
    (1_000, 0), (866, 500), (500, 866), (0, 1_000), (-500, 866), (-866, 500),
    (-1_000, 0), (-866, -500), (-500, -866), (0, -1_000), (500, -866), (866, -500),
];
/// A near-support point needs LaneOpen of at least 0.6 [estimate].
const NEAR_LANE_MIN: Fp = Fp::from_milli(600);
/// The two near supporters' points are at least 30 degrees apart around the carrier (cos 30).
const NEAR_ANGLE_COS: Fp = Fp::from_milli(866);
/// Slot leash of a near supporter (mm): midfielders 25 m in 11v11 (13 m in 5v5), others the
/// candidate leash [estimate].
const NEAR_LEASH_MID_MM: (i64, i64) = (25_000, 13_000);
/// The near-support assignment is made when the carrier has fewer than 2 open options, held at
/// least 60 ticks, and dropped once he has 3 or more [estimate].
const NEAR_NEED_BELOW: usize = 2;
const NEAR_DROP_AT: usize = 3;
const NEAR_COMMIT_TICKS: u32 = 60;
/// Travel scale of the near-support score (m): the score falls as `1 / (1 + d / 4 m)^2` with the
/// supporter's distance to the point [estimate].
const NEAR_TRAVEL_SCALE: Fp = Fp::from_int(4);
/// Only supporters within this distance of the carrier (mm) are recruited for near support [estimate].
const NEAR_RECRUIT_MM: (i64, i64) = (24_000, 15_000);
/// Keep near points this far (mm) from the other teammates and from each other: 5 m in 11v11, 7 m in 5v5 [estimate].
const NEAR_MATE_SEP_MM: (i64, i64) = (5_000, 7_000);

/// The carrier support is organised around (round 4 second pass): our carrier at his position, or,
/// while our pass is in flight, its intended receiver at the pass target (support anticipates the
/// reception instead of waiting for it). `None` for the keeper or without either.
fn support_focus(state: &MatchState, side: TeamSide) -> Option<(usize, Vec2)> {
    if let Some((os, oi)) = state.ball.owner {
        return (os == side && oi != 0).then(|| (oi, state.players[side.index()][oi].pos));
    }
    let pf = state.ball.pass_in_flight.as_ref().filter(|pf| pf.team == side && pf.intended_receiver_idx != 0)?;
    Some((pf.intended_receiver_idx, pf.target_pos))
}

/// Open options of `side`'s carrier as the analytics count them: an outfielder 6-30 m away, no
/// opponent within 1.5 m of the straight lane, not beyond the second-last defender and the ball.
fn carrier_open_options(state: &MatchState, side: TeamSide, ci: usize, c: Vec2) -> usize {
    let attack_sign = side.attack_sign(state.second_half_swapped());
    let onside_x = compute_onside_limit_x(state, side);
    let mut n = 0;
    for (j, p) in state.players[side.index()].iter().enumerate().skip(1) {
        if j == ci || p.sent_off {
            continue;
        }
        let d = p.pos.dist(c);
        if d < Fp::from_int(6) || d > Fp::from_int(30) {
            continue;
        }
        if state.rules.offsides_enabled && beyond_line(p.pos, onside_x, attack_sign) {
            continue;
        }
        let dir = (p.pos - c).normalize_or_zero();
        let blocked = state.players[side.opponent().index()].iter().filter(|o| !o.sent_off).any(|o| {
            let rel = o.pos - c;
            let proj = rel.dot(dir).clamp(Fp::ZERO, d);
            (rel - dir.scale(proj)).len() < Fp::from_milli(1_500)
        });
        if !blocked {
            n += 1;
        }
    }
    n
}

/// Near-support assignment of the team (lazy, stamped `near_eval`, once per tick): when the
/// carrier has fewer than [`NEAR_NEED_BELOW`] open options, the two outfielders nearest him (no
/// run role, not the rest-defence player) get support points 8-18 m from him with LaneOpen >= 0.6,
/// at least 30 degrees apart, scored Control x LaneOpen^2 x (Value + safe bias) (the lane weighted
/// over value); re-scored every [`SUP_RESCORE_TICKS`], held [`NEAR_COMMIT_TICKS`], dropped when the
/// carrier changes or has [`NEAR_DROP_AT`] or more open options.
fn update_near_support(state: &MatchState, side: TeamSide) {
    let stamp = state.tick.wrapping_add(1);
    {
        let Ok(mems) = state.ai_memory.try_borrow() else {
            return;
        };
        if mems[side.index()].near_eval == stamp {
            return;
        }
    }
    let focus = support_focus(state, side);
    let carrier = focus.map(|(ci, _)| ci);
    let n_open = focus.map_or(0, |(ci, c)| carrier_open_options(state, side, ci, c));
    let ctx = ShapeCtx::new(state, side);
    let is_5v5 = ctx.is_5v5;
    let Ok(mut mems) = state.ai_memory.try_borrow_mut() else {
        return;
    };
    let m = &mut mems[side.index()];
    m.ensure_len(state.players[side.index()].len());
    m.near_eval = stamp;
    let Some(ci) = carrier.filter(|_| matches!(state.phase, MatchPhase::OpenPlay)) else {
        m.near_idx = vec![AI_NONE; 2];
        return;
    };
    let active = m.near_idx.iter().any(|&j| j != AI_NONE) && m.near_carrier == ci as u8;
    let held = stamp.wrapping_sub(m.near_since);
    if active && held >= NEAR_COMMIT_TICKS && n_open >= NEAR_DROP_AT {
        m.near_idx = vec![AI_NONE; 2];
        return;
    }
    if !active && n_open >= NEAR_NEED_BELOW {
        m.near_idx = vec![AI_NONE; 2];
        return;
    }
    if active && !held.is_multiple_of(SUP_RESCORE_TICKS) {
        return;
    }
    // (Re-)assign: the two nearest eligible supporters.
    let players = &state.players[side.index()];
    let c = focus.map_or(players[ci].pos, |(_, c)| c);
    let mut near: Vec<(Fp, usize)> = players
        .iter()
        .enumerate()
        .skip(1)
        .filter(|&(j, p)| {
            j != ci
                && !p.sent_off
                && m.run_role.get(j).copied().unwrap_or(RUN_NONE) == RUN_NONE
                && !attack_roles(&ctx, m.slot_of.get(j).copied().unwrap_or(AI_NONE), p).0
        })
        .map(|(j, p)| (p.pos.dist(c), j))
        .filter(|&(d, _)| d <= Fp::from_milli(fmt_pick(is_5v5, NEAR_RECRUIT_MM)))
        .collect();
    near.sort_unstable();
    let pair: Vec<usize> = near.iter().take(2).map(|&(_, j)| j).collect();
    let defs = opp_movers(state, side);
    let attack_sign = side.attack_sign(state.second_half_swapped());
    let onside_x = compute_onside_limit_x(state, side);
    let hl = state.format.half_length() - Fp::from_milli(2_000);
    let hw = state.format.half_width() - Fp::from_milli(1_500);
    let goal = Vec2::new(Fp::from_milli(attack_sign * state.format.half_length().to_milli()), Fp::ZERO);
    // Candidate points with their lane (receiver-independent).
    let mut pts: Vec<(Vec2, Fp)> = Vec::new();
    let mate_sep = Fp::from_milli(fmt_pick(is_5v5, NEAR_MATE_SEP_MM));
    for &(r11, r5) in &NEAR_RADII_MM {
        let r = if is_5v5 { r5 } else { r11 };
        for &(cx, sy) in &NEAR_BEARINGS_PM {
            let q = c + Vec2::from_milli(r * cx / 1_000, r * sy / 1_000);
            if q.x.abs() > hl || q.y.abs() > hw {
                continue;
            }
            if state.rules.offsides_enabled && beyond_line(q, onside_x, attack_sign) {
                continue;
            }
            let crowded = players.iter().enumerate().skip(1).any(|(k, mate)| {
                k != ci
                    && !pair.contains(&k)
                    && !mate.sent_off
                    && (mate.pos.dist(q) < mate_sep || m.target.get(k).is_some_and(|t| t.dist(q) < mate_sep))
            });
            if crowded {
                continue;
            }
            let lane = crate::space::lane_open(c, q, &crate::space::Mover::new(q, Vec2::ZERO), &defs);
            if lane >= NEAR_LANE_MIN && segment_clear(state, side, c, q) {
                pts.push((q, lane));
            }
        }
    }
    let score = |j: usize, q: Vec2, lane: Fp| -> Fp {
        let p = &players[j];
        let leash = if p.role == PlayerRole::Midfielder {
            fmt_pick(is_5v5, NEAR_LEASH_MID_MM)
        } else {
            fmt_pick(is_5v5, SUP_LEASH_MM)
        };
        let home = m.target.get(j).copied().unwrap_or(p.pos);
        if q.dist(home) > Fp::from_milli(leash) {
            return Fp::ZERO;
        }
        let me = crate::space::Mover::new(p.pos, p.vel);
        let ctl = crate::space::control(q, q.dist(c).div(SUP_BALL_SPEED), &[me], &defs);
        // The nearest open lane wins unless a farther one is much better: possessions are short,
        // so a point the supporter reaches in time beats a better one he does not (EA's "teammates
        // move towards these lines" [patent]).
        let travel = Fp::ONE + lookahead(p).dist(q).div(NEAR_TRAVEL_SCALE);
        ctl.mul(lane).mul(lane).mul(crate::space::value_at(q, goal) + SUP_SAFE_BIAS).div(travel.mul(travel))
    };
    let mut idx_out = vec![AI_NONE; 2];
    let mut off_out = vec![Vec2::ZERO; 2];
    let mut first_dir: Option<Vec2> = None;
    let mut first_pt: Option<Vec2> = None;
    for (slot, &j) in pair.iter().enumerate() {
        let mut best: Option<(Fp, Vec2)> = None;
        for &(q, lane) in &pts {
            if let (Some(fd), Some(fp)) = (first_dir, first_pt) {
                if (q - c).normalize_or_zero().dot(fd) > NEAR_ANGLE_COS || q.dist(fp) < mate_sep {
                    continue;
                }
            }
            let sc = score(j, q, lane);
            if sc > Fp::ZERO && best.is_none_or(|(bs, _)| sc > bs) {
                best = Some((sc, q));
            }
        }
        if let Some((_, q)) = best {
            idx_out[slot] = j as u8;
            off_out[slot] = q - c;
            first_dir.get_or_insert((q - c).normalize_or_zero());
            first_pt.get_or_insert(q);
        }
    }
    if !active || m.near_idx != idx_out {
        m.near_since = stamp;
    }
    m.near_carrier = ci as u8;
    m.near_idx = idx_out;
    m.near_off = off_out;
}

/// Assisted pass receiver override (round 4): the called runner of `side`, or the penetrate runner
/// the CPU pass decision released (other than the kicker), onside now, whose post-release look-ahead lies inside the kick's aim
/// cone (`aim . dir >= min_align`); the best-aligned one with that lead point. Offside is judged
/// at the kick from his current position, so leading him beyond the line is legal.
pub(crate) fn released_runner_target(
    state: &MatchState,
    side: TeamSide,
    kicker_idx: usize,
    kicker_pos: Vec2,
    aim: Vec2,
    min_align: Fp,
) -> Option<(usize, Vec2)> {
    let mems = state.ai_memory.try_borrow().ok()?;
    let m = &mems[side.index()];
    let attack_sign = side.attack_sign(state.second_half_swapped());
    let onside_x = compute_onside_limit_x(state, side);
    let mut best: Option<(Fp, usize, Vec2)> = None;
    for (j, p) in state.players[side.index()].iter().enumerate().skip(1) {
        let role = m.run_role.get(j).copied().unwrap_or(RUN_NONE);
        // A called run is the human's own request; a penetrate runner only once the CPU pass
        // decision released him (every other pass keeps the cone's default pick).
        let released = role == RUN_CALLED
            || (role == RUN_PENETRATE
                && m.run_release as usize == j
                && state.tick.wrapping_add(1).wrapping_sub(m.run_release_tick) <= RUN_RELEASE_TTL);
        if j == kicker_idx || p.sent_off || !released {
            continue;
        }
        if state.rules.offsides_enabled && beyond_line(p.pos, onside_x, attack_sign) {
            continue;
        }
        let lead = runner_release_point(p, m.run_end.get(j).copied().unwrap_or(p.pos));
        let to = lead - kicker_pos;
        if to.len() < Fp::from_milli(4_000) {
            continue;
        }
        let align = to.normalize_or_zero().dot(aim);
        if align >= min_align && best.is_none_or(|(ba, _, _)| align > ba) {
            best = Some((align, j, lead));
        }
    }
    best.map(|(_, j, lead)| (j, lead))
}

/// Post-release look-ahead of a committed runner: where he is 1 s after the release, sprinting
/// from his position toward his end-point.
pub(crate) fn runner_release_point(p: &PlayerState, end: Vec2) -> Vec2 {
    let to = end - p.pos;
    let reach = Fp::from_int(RUN_RELEASE_SPEED_MPS as i32).mul(LOOKAHEAD_S);
    p.pos + to.normalize_or_zero().scale(to.len().min(reach))
}

/// Shape, role behaviours and committed movement for an AI outfielder in open play.
fn outfield_shape_intent(state: &MatchState, side: TeamSide, idx: usize) -> AiMoveIntent {
    update_team_plan(state, side);
    let ctx = ShapeCtx::new(state, side);
    let diff = diff_profile(side_difficulty(state, side));
    let p = &state.players[side.index()][idx];
    let style = player_style(p);
    let is_5v5 = ctx.is_5v5;
    let attack_sign = ctx.sign;
    let half_len = state.format.half_length();
    let ball_xy = state.ball.pos.xy();
    let goal_half_w = state.format.goal_half_width();
    let own_goal_x = Fp::from_milli(-attack_sign * half_len.to_milli());
    let own_goal = Vec2::new(own_goal_x, Fp::ZERO);
    let my_dist = p.pos.dist(ball_xy);

    let (phase, presser, second, cover, slot_k, mark, ref_ball, prev_target, prev_tick, prev_flags) = {
        let mems = state.ai_memory.borrow();
        let m = &mems[side.index()];
        (
            m.phase,
            m.presser,
            m.second_presser,
            m.cover,
            m.slot_of.get(idx).copied().unwrap_or(AI_NONE),
            m.mark_of.get(idx).copied().unwrap_or(AI_NONE),
            m.ref_ball,
            m.target.get(idx).copied().unwrap_or(p.pos),
            m.target_tick.get(idx).copied().unwrap_or(0),
            m.flags.get(idx).copied().unwrap_or(0),
        )
    };
    let (chaser, chaser_pt, mate_targets, mate_target_ticks, mate_res_yield) = {
        let mems = state.ai_memory.borrow();
        let m = &mems[side.index()];
        (m.chaser, m.chaser_pt, m.target.clone(), m.target_tick.clone(), m.res_yield.clone())
    };
    // Attacking support, runs and decision hysteresis memory (round 4 work area 3).
    let mut um = {
        let mems = state.ai_memory.borrow();
        UMem::read(&mems[side.index()], idx)
    };
    // Round 4, area T plan read-outs.
    let (third, closer_lane, rest_offence, line_prog, press_trigger, rec_prev, burst_prev, hi_dist, slot_of) = {
        let mems = state.ai_memory.borrow();
        let m = &mems[side.index()];
        (
            m.third_presser,
            m.closer_lane,
            m.rest_offence,
            m.line_prog,
            m.press_trigger_ticks,
            m.rec_flags.get(idx).copied().unwrap_or(0),
            m.burst_ticks.get(idx).copied().unwrap_or(0),
            m.hi_dist_mm.get(idx).copied().unwrap_or(0),
            m.slot_of.clone(),
        )
    };
    let me = idx as u8;
    let slot = ctx.slot(slot_k);
    let role = slot.map(|s| s.role);
    let opp_carrier = state.ball.owner.and_then(|(os, oi)| (os != side).then_some(oi));
    let opp_owns = opp_carrier.is_some();
    let we_own = state.ball.owner.is_some_and(|(os, _)| os == side);
    let (blk, attacking) = phase_block(&ctx, state, ref_ball, phase);
    let anchor = match slot {
        Some(s) => slot_pos(&ctx, &blk, &s, attacking, front_mode(&ctx, state, &slot_of, rest_offence, slot_k as usize)),
        None => world_anchor_for_player(state, side, idx, Fp::ZERO),
    };

    let mut raw: Vec2;
    let mut press_role = false;
    let mut force_sprint = false;
    let mut counter_run = false;
    let mut reserved_run = false;
    let mut yielding = false;
    let mut alpha_pm = 120 + style.discipline / 5;
    // A committed runner holding at the onside limit (stutter, never reversing while onside).
    let mut run_hold = false;
    // Round 4, area T: recovery urgency (high-speed band, latched recovery sprint and its burst).
    let mut high_band = false;
    let mut rec_now = false;
    let mut burst_now: u16 = 0;
    let mut trigger_sprint = false;
    let fatigue_pm = if hi_dist > HI_PEAK_MM { FATIGUE_PM } else { 1_000 };
    let continuous_prev = prev_tick != 0 && prev_tick.wrapping_add(1) == state.tick;
    let opp_side = side.opponent().index();

    if is_defensive_phase(phase) && !we_own {
        let pred_ball = predict_ball_xy_at_ticks(state, 5);
        let goal_dir = (own_goal - pred_ball).normalize_or_zero();
        // Most dangerous receiver lane from the carrier (cover shadows and lane blocks): the receiver
        // with the best value (closeness to our goal of his 1 s look-ahead) x reach (near the carrier).
        let lanes: Vec<(i64, Vec2, Vec2)> = match opp_carrier {
            Some(ci) => {
                let c_pos = state.players[opp_side][ci].pos;
                let mut v: Vec<(i64, Vec2, Vec2)> = state.players[opp_side]
                    .iter()
                    .enumerate()
                    .skip(1)
                    .filter(|&(o, op)| o != ci && !op.sent_off && op.pos.dist(c_pos) <= Fp::from_milli(30_000))
                    .map(|(_, op)| {
                        let look = op.pos + op.vel;
                        let value = (1_000 - ctx.prog(look.x) * 1_000 / ctx.len).clamp(0, 1_000) + 200;
                        let reach = 15_000_000 / (15_000 + look.dist(c_pos).to_milli());
                        (-(value * reach / 1_000), c_pos, op.pos)
                    })
                    .collect();
                v.sort_by_key(|l| l.0);
                v
            }
            None => Vec::new(),
        };
        let shadow_shift = |base: Vec2| -> Vec2 {
            // Cover shadow: shift the contain point up to 0.75 m toward the best lane's side.
            match lanes.first() {
                Some(&(_, c, r)) => {
                    let perp = Vec2::new(-goal_dir.y, goal_dir.x);
                    let sgn = if perp.dot(r - c) >= Fp::ZERO { Fp::ONE } else { Fp::NEG_ONE };
                    base + perp.scale(sgn.mul(Fp::from_milli(SHADOW_SHIFT_MM)))
                }
                None => base,
            }
        };
        if phase == AI_PHASE_COUNTER_PRESS && me == presser {
            // Counter-press: hunt the ball tight while the window lasts.
            let gap = 950 + (1_000 - p.stamina_permille as i64).max(0) / 3;
            raw = shadow_shift(pred_ball + goal_dir.scale(Fp::from_milli(gap)));
            press_role = true;
            force_sprint = true;
            alpha_pm = 650;
        } else if phase == AI_PHASE_COUNTER_PRESS && (me == second || me == third) {
            // Lane-closers shut the two best receiver lanes (nearest point on the lane, at least 3 m
            // from the carrier and no more than 70% along it), at the high-speed band.
            let k = usize::from(me == third);
            let mut lane = pred_ball + goal_dir.scale(Fp::from_milli(3_000));
            if let (Some(ci), Some(o)) = (opp_carrier, plan_idx(closer_lane[k])) {
                let c_pos = state.players[opp_side][ci].pos;
                let r_pos = state.players[opp_side].get(o).map_or(c_pos, |r| r.pos);
                let len = c_pos.dist(r_pos);
                let lo = Fp::from_milli(3_000).min(len);
                let hi = len.mul(Fp::from_milli(700)).max(lo);
                let dir = (r_pos - c_pos).normalize_or_zero();
                let along = (p.pos - c_pos).dot(dir).clamp(lo, hi);
                lane = c_pos + dir.scale(along);
            }
            raw = lane;
            high_band = p.pos.dist(raw) > Fp::from_milli(2_500);
            alpha_pm = 500;
        } else if me == second && phase != AI_PHASE_COUNTER_PRESS {
            // Second man on a press trigger: shadow 3.5 m behind-side of the presser, toward the best
            // lane, never a tackler and never inside 3 m of the carrier.
            let presser_pos = if presser != AI_NONE {
                state.players[side.index()][presser as usize].pos
            } else {
                state.players[side.index()][state.controlled_idx[side.index()]].pos
            };
            let perp = Vec2::new(-goal_dir.y, goal_dir.x);
            let sgn = match lanes.first() {
                Some(&(_, c, r)) if perp.dot(r - c) < Fp::ZERO => Fp::NEG_ONE,
                _ => Fp::ONE,
            };
            let off = (goal_dir.scale(Fp::from_milli(700)) + perp.scale(sgn.mul(Fp::from_milli(700)))).normalize_or_zero();
            raw = presser_pos + off.scale(Fp::from_milli(SECOND_MAN_MM));
            high_band = p.pos.dist(raw) > Fp::from_milli(2_500);
            alpha_pm = 500;
        } else if me == presser && phase != AI_PHASE_COUNTER_PRESS {
            // ONE presser contains goal-side; gap set by aggression, stamina decay & difficulty.
            let diff_adj = match side_difficulty(state, side) {
                CpuDifficulty::Hard => -100,
                CpuDifficulty::Normal => 0,
                CpuDifficulty::Easy => 150,
            };
            let gap = (1_550 - (style.aggression - 500) * 3 / 5
                + (1_000 - p.stamina_permille as i64).max(0) / 2
                + diff_adj)
                .clamp(1_050, 2_100);
            raw = shadow_shift(pred_ball + goal_dir.scale(Fp::from_milli(gap)));
            press_role = true;
            alpha_pm = 600;
            trigger_sprint = press_trigger > 0;
        } else if me == cover {
            // Cover: goal-side behind whoever presses, never higher than the back line + 2 m.
            let depth = Fp::from_milli(if is_5v5 { 4_500 } else { 6_500 });
            let presser_pos = if presser != AI_NONE {
                state.players[side.index()][presser as usize].pos
            } else if chaser != 0 {
                state.players[side.index()][chaser as usize].pos
            } else {
                state.players[side.index()][state.controlled_idx[side.index()]].pos
            };
            let mut c = ref_ball.lerp(presser_pos, Fp::HALF) + (own_goal - ref_ball).normalize_or_zero().scale(depth);
            if phase != AI_PHASE_COUNTER_PRESS {
                let def_line = defend_block(&ctx, state, ref_ball, phase == AI_PHASE_RETREAT).line;
                let cp = ctx.prog(c.x).min(def_line + 2_000);
                c = ctx.world(cp, ctx.yf(c.y));
            }
            raw = c;
            alpha_pm = 380;
        } else {
            // Every other defender, in every defensive phase (the counter-press included).
            let back_line = role.is_some_and(|r| r.is_back_line());
            let back_or_screen = back_line || role == Some(SlotRole::Holding);
            let is_front = role.is_some_and(|r| matches!(r, SlotRole::Striker | SlotRole::Pivo | SlotRole::Winger))
                || p.role == PlayerRole::Forward;
            let is_rest = plan_idx(rest_offence) == Some(idx);
            let band = dwr_band(style.def_work_rate);
            let ball_p = ctx.prog(ball_xy.x);
            let opp_wide_cutback_threat = opp_owns
                && (ball_xy.x - own_goal_x).abs() <= Fp::from_milli(if is_5v5 { 13_500 } else { 18_000 })
                && ball_xy.y.abs() > goal_half_w + Fp::from_milli(1_600);
            // Back-liners hold the defend-block slot on the shared line in every defensive phase (the
            // rest-defence core never joins the counter-press).
            let (anchor, blk) = match (back_line && phase == AI_PHASE_COUNTER_PRESS, slot) {
                (true, Some(s)) => {
                    let dblk = defend_block(&ctx, state, ref_ball, false);
                    (slot_pos(&ctx, &dblk, &s, false, FrontMode::Plain), dblk)
                }
                _ => (anchor, blk),
            };
            let anchor_p = ctx.prog(anchor.x);
            let mut tgt_p = if back_line && line_prog != 0 {
                // Shared line: one team value, the same judgement error for the whole back line.
                let err_mm = smooth_noise_pm(state.tick, 150, side.index() as u32 * 31 + 7, 0)
                    * (100 - avg_positioning_pm(state, side) / 10).max(0)
                    * 30
                    / 1_000
                    * diff.err_pm
                    / 1_000;
                line_prog + (anchor_p - blk.line) + err_mm
            } else {
                // Judgement error on stepping: positioning x difficulty, drifting smoothly (60%).
                let err_mm = smooth_noise_pm(state.tick, 150, side.index() as u32 * 31 + 7, idx as u32)
                    * (100 - p.attributes.positioning as i64).max(0)
                    * 30
                    / 1_000
                    * diff.err_pm
                    / 1_000;
                anchor_p + if back_or_screen { err_mm } else { err_mm * 6 / 10 }
            };
            if phase == AI_PHASE_COUNTER_PRESS && !is_rest {
                // Counter-press: whoever is ahead of the ball gets goal-side of it (forwards to their
                // work-rate holding line).
                let hold = if is_front { FWD_HOLD_MM[usize::from(is_5v5)][band] } else { RECOVERY_EXIT_MM };
                tgt_p = tgt_p.min(ball_p - hold);
            }
            let mut tgt_yf = ctx.yf(anchor.y);
            // Cover tuck: the ball-far full-back tucks in once the carrier is at or past our line.
            if role == Some(SlotRole::FullBack)
                && opp_owns
                && line_prog != 0
                && ball_p <= line_prog + 1_000
                && (tgt_yf >= 0) != (ctx.yf(ball_xy.y) >= 0)
            {
                tgt_yf = tgt_yf * 6 / 10;
            }
            raw = ctx.world(tgt_p, tgt_yf);
            let zonal_pt = raw;
            if opp_wide_cutback_threat && back_or_screen {
                // Inside-box cutback zone marking: drop into the penalty-spot lane, mark box receivers.
                let cutback_hub = Vec2::new(
                    own_goal_x + Fp::from_milli(attack_sign * (if is_5v5 { 5_800 } else { 7_800 })),
                    ball_xy.y.mul(Fp::from_milli(220)),
                );
                let mut cutback_mark: Option<Vec2> = None;
                let mut best_d = Fp::from_milli(11_000);
                for (o_idx, opp) in state.players[opp_side].iter().enumerate().skip(1) {
                    if Some(o_idx) == opp_carrier || opp.sent_off {
                        continue;
                    }
                    if (opp.pos.x - own_goal_x).abs() <= Fp::from_milli(15_000)
                        && opp.pos.y.abs() <= state.format.penalty_box_half_width()
                    {
                        let d = p.pos.dist(opp.pos);
                        if d < best_d {
                            best_d = d;
                            let pass_lane = (ball_xy - opp.pos).normalize_or_zero();
                            let goal_side = (own_goal - opp.pos).normalize_or_zero();
                            let shade = (pass_lane.scale(Fp::from_milli(580)) + goal_side.scale(Fp::from_milli(420)))
                                .normalize_or_zero();
                            cutback_mark = Some(opp.pos + shade.scale(Fp::from_milli(1_100)));
                        }
                    }
                }
                raw = cutback_mark.unwrap_or(cutback_hub);
            } else if let Some(mo) = plan_idx(mark).filter(|&o| o < state.players[opp_side].len()) {
                let opp = &state.players[opp_side][mo];
                let to_goal = (own_goal - opp.pos).normalize_or_zero();
                let to_ball = (ball_xy - opp.pos).normalize_or_zero();
                // Konami intercept line: only when the carrier faces the marked man within theta_a and
                // a teammate covers the marked man's line to goal; otherwise the goal-side shadow.
                let covered = state.players[side.index()].iter().enumerate().skip(1).any(|(j, t)| {
                    j != idx
                        && !t.sent_off
                        && ctx.prog(t.pos.x) < ctx.prog(opp.pos.x)
                        && closest_on_segment(opp.pos, own_goal, t.pos).dist(t.pos) <= Fp::from_milli(COVER_LA_MM)
                });
                let faced = opp_carrier.is_some_and(|ci| {
                    let c = &state.players[opp_side][ci];
                    let f = Vec2::from_bam(c.facing_bam, Fp::ONE);
                    f.dot((opp.pos - c.pos).normalize_or_zero()) > Fp::from_milli(COS_THETA_A_PM)
                });
                let shadow = if faced && covered {
                    to_ball
                } else {
                    (to_goal.scale(Fp::from_milli(650)) + to_ball.scale(Fp::from_milli(350))).normalize_or_zero()
                };
                let mark_pt = opp.pos + shadow.scale(Fp::from_milli(1_350));
                let runner = back_line && line_prog != 0 && ctx.prog(opp.pos.x) < line_prog;
                let w = if runner {
                    // A runner in behind is tracked regardless of the block anchor.
                    900
                } else {
                    match role {
                        Some(SlotRole::CentreBack | SlotRole::Fixo | SlotRole::Holding) => 520,
                        _ => 600,
                    }
                };
                let mut t = raw.lerp(mark_pt, Fp::from_milli(w));
                if back_or_screen && !runner {
                    // The back line holds its shared height: marking may not drag it more than
                    // 2.5 m deep / 1.2 m high (holding screen: 4 m / 3 m).
                    let line = ctx.prog(raw.x);
                    let (lo, hi) = if back_line { (2_500, 1_200) } else { (4_000, 3_000) };
                    t = ctx.world(ctx.prog(t.x).clamp(line - lo, line + hi), ctx.yf(t.y));
                }
                raw = t;
            } else if !back_line && !is_rest && zonal_pt.dist(ball_xy) <= Fp::from_milli(LANE_BLOCK_BALL_MM) {
                // Zonal holder near the ball: stand on the best lane within a 3 m leash of the slot.
                let leash = Fp::from_milli(LANE_BLOCK_LEASH_MM);
                if let Some(pt) = lanes
                    .iter()
                    .map(|&(_, c, r)| closest_on_segment(c, r, zonal_pt))
                    .find(|q| q.dist(zonal_pt) <= leash && q.dist(ball_xy) >= Fp::from_milli(NON_ENGAGER_GAP_MM))
                {
                    raw = pt;
                }
            }
            // Urgency by deficit d = progress ahead of the ball: ahead and more than 30 ticks from the
            // target (or the ball runs at our goal > 4 m/s) -> latched recovery sprint until 3 m
            // goal-side, kept in the high-speed band while the counter runs; bursts capped by work
            // rate and fatigue (back line uncapped), then the band; far from the target -> the
            // `dist_sprint` rule below; goal-side near the slot -> 70% cruise (class 3).
            let d = ctx.prog(p.pos.x) - ball_p;
            let threatened = !is_rest && (band > 0 || d <= LOW_DWR_REACH_MM);
            let exit = -RECOVERY_EXIT_MM;
            let was_rec = (rec_prev & 1) != 0 && continuous_prev;
            let ball_to_goal = -state.ball.vel.x.to_milli() * attack_sign;
            // On a loose ball (a pass in flight) only players more than 15 m from it latch on the
            // counter: nearer, a run along the ball path is a second chaser (exclusivity layer).
            let counter_on = ball_to_goal > COUNTER_BALL_MM_S && (opp_owns || my_dist > Fp::from_milli(15_000));
            let rec = threatened
                && if was_rec {
                    // While the ball still travels at our goal the run is kept (goal-side tracking).
                    d > exit || counter_on
                } else {
                    // A counter running at us also catches players not yet 3 m goal-side.
                    (counter_on && d > exit)
                        || (d > 1_000 && estimate_ticks_to_point(p, raw, is_5v5) > RECOVERY_TRIGGER_TICKS)
                };
            // Back-liners caught upfield are exempt from the burst cap: full-backs sprint the most in
            // defensive transitions (Bortnik et al. 2024) and a capped one leaves the line uneven.
            let cap = if back_line { u16::MAX / 2 } else { (BURST_CAP_TICKS[band] as i64 * fatigue_pm / 1_000) as u16 };
            if rec && d <= exit {
                // Goal-side already: track the counter in the high-speed band, no sprint.
                rec_now = true;
                burst_now = burst_prev;
                high_band = true;
            } else if rec {
                rec_now = true;
                burst_now = if was_rec { burst_prev.saturating_add(1) } else { 1 };
                if burst_now > cap.saturating_add(BURST_REST_TICKS) {
                    burst_now = 1;
                }
                if burst_now <= cap {
                    force_sprint = true;
                } else {
                    high_band = true;
                }
            }
        }
    } else {
        // --- Attacking shape refinements (rest-defence fixo, support triangle, pockets, runs) ---
        um.touched = true;
        let mut att_target = anchor;
        let (rest_role, runner_role) = attack_roles(&ctx, slot_k, p);
        // One run decision per team per tick (lazily, by the first supporter processed).
        update_run_arbiter(state, side);
        update_near_support(state, side);
        let (run_role, run_end, run_release, near_off) = {
            let mems = state.ai_memory.borrow();
            let m = &mems[side.index()];
            (
                m.run_role.get(idx).copied().unwrap_or(RUN_NONE),
                m.run_end.get(idx).copied().unwrap_or(anchor),
                m.run_release,
                m.near_idx.iter().position(|&j| j == idx as u8).and_then(|k| m.near_off.get(k).copied()),
            )
        };
        let opp_goal_x = Fp::from_milli(attack_sign * half_len.to_milli());
        if is_5v5 && rest_role && p.run_boost_ticks == 0 {
            let pivot_proj = (ball_xy.x.to_milli() * attack_sign - 8_800)
                .clamp(-(half_len.to_milli() - 7_500), 6_500);
            let cur_proj = att_target.x.to_milli() * attack_sign;
            att_target = Vec2::new(
                Fp::from_milli(attack_sign * pivot_proj.min(cur_proj)),
                att_target.y.lerp(ball_xy.y.mul(Fp::from_milli(340)), Fp::HALF),
            );
        } else if is_5v5 {
            let carrier_opt = state.ball.owner.and_then(|(os, oi)| (os == side).then_some(oi));
            let mut central_box_occupied = false;
            for (m_i, mate) in state.players[side.index()].iter().enumerate().skip(1) {
                if m_i != idx
                    && Some(m_i) != carrier_opt
                    && !mate.sent_off
                    && (mate.pos.x - opp_goal_x).abs() <= Fp::from_milli(11_500)
                    && mate.pos.y.abs() <= Fp::from_milli(4_800)
                {
                    central_box_occupied = true;
                }
            }
            if !central_box_occupied
                && ball_xy.x.to_milli() * attack_sign > half_len.to_milli() / 3
                && ball_xy.y.abs() > goal_half_w + Fp::from_milli(1_500)
                && designated_support_triangle_idx(state, side) != Some(idx)
            {
                att_target = Vec2::new(
                    opp_goal_x - Fp::from_milli(attack_sign * 6_800),
                    -ball_xy.y.clamp(Fp::from_milli(-6_000), Fp::from_milli(6_000)).mul(Fp::from_milli(220)),
                );
            }
        }
        if we_own && designated_support_triangle_idx(state, side) == Some(idx) && !rest_role {
            let to_me = att_target - ball_xy;
            let dist_b = to_me.len();
            if dist_b > Fp::from_milli(if is_5v5 { 13_800 } else { 14_500 }) {
                let check_dir = to_me.normalize_or_zero();
                att_target = ball_xy + check_dir.scale(Fp::from_milli(if is_5v5 { 9_500 } else { 11_800 }));
            }
        }
        // Scored support candidates (round 4 work area 3; replaces the five fixed pockets): the
        // slot, the PES ring around the carrier, the nearest opponent Voronoi vertex and the
        // offside-line point, scored Control x Value x LaneOpen and committed with hysteresis.
        let mut scored = false;
        let near_focus = support_focus(state, side).filter(|&(ci, _)| ci != idx);
        if let (Some(off), Some((_, c))) = (near_off, near_focus) {
            // Near support: one of the two players closest to the carrier comes to an open lane
            // (a support run: class 2, he may sprint to it).
            att_target = c + off;
            um.set_kind(SUP_NEAR, state.tick);
            um.sup_pt = att_target;
            scored = true;
            reserved_run = true;
        } else if !rest_role {
            if let Some(t) = support_candidate_target(
                state,
                side,
                idx,
                att_target,
                &mut um,
                &mate_targets,
                &mate_target_ticks,
                chaser,
                is_5v5,
            ) {
                att_target = t;
                scored = true;
            }
        }
        // Onside channel runs: strikers lead the line and pin the last defender; counter-attacks
        // send every forward/wide player in behind without waiting for the passer.
        let passer_ready = passer_ready_schmitt(state, side);
        // Exclusivity: GRF support-field spacing and one runner per destination (possession only).
        let in_possession = we_own
            || (state.ball.owner.is_none() && state.ball.pass_in_flight.as_ref().is_some_and(|pf| pf.team == side));
        if in_possession && !scored {
            // GRF spacing is the fallback for players without a scored candidate.
            att_target = support_spacing(state, side, idx, att_target, is_5v5);
        }
        let counter = phase == AI_PHASE_COUNTER_ATTACK;
        if p.run_boost_ticks > 0 || (runner_role && !rest_role && !scored && (passer_ready || counter) && !opp_owns) {
            let onside_limit_x = compute_onside_limit_x(state, side);
            let run_push = Fp::from_milli(if p.run_boost_ticks > 0 {
                if is_5v5 { 7_800 } else { 11_500 }
            } else if matches!(role, Some(SlotRole::Striker | SlotRole::Pivo)) {
                if is_5v5 { 5_600 } else { 7_800 }
            } else if counter {
                if is_5v5 { 5_000 } else { 8_000 }
            } else {
                if is_5v5 { 3_800 } else { 5_600 }
            });
            let desired_x = att_target.x + Fp::from_int(attack_sign as i32).mul(run_push);
            att_target.x = if attack_sign > 0 {
                desired_x.min(onside_limit_x)
            } else {
                desired_x.max(onside_limit_x)
            };
            if counter {
                counter_run = true;
            }
            reserved_run = true;
        }
        // Committed runs (arbiter roles): the end-point beyond the line, held at the onside limit
        // until released (our pass toward him, or the CPU pass decision that selected him).
        let is_runner = run_role != RUN_NONE && !opp_owns;
        // A near supporter keeps his destination too (the others yield to him).
        let holds_destination = is_runner || um.sup_kind == SUP_NEAR;
        if is_runner {
            let onside_x = compute_onside_limit_x(state, side);
            let on = |x: Fp| if attack_sign > 0 { x.min(onside_x) } else { x.max(onside_x) };
            let released = matches!(run_role, RUN_PENETRATE | RUN_CALLED) && run_released(state, side, idx, run_release);
            let pass_to_me = state.ball.pass_in_flight.as_ref().is_some_and(|pf| pf.team == side && pf.intended_receiver_idx == idx);
            att_target = if pass_to_me {
                // The CPU-controlled receiver reaches this path (not the receive intent): meet the
                // ball, do not keep running to the end-point.
                best_ball_intercept_for_player(state, side, idx).0
            } else if released || !state.rules.offsides_enabled {
                run_end
            } else {
                Vec2::new(on(run_end.x), run_end.y)
            };
            run_hold = !released && matches!(run_role, RUN_PENETRATE | RUN_CALLED | RUN_DECOY);
            force_sprint = force_sprint || released;
            reserved_run = true;
        }
        um.res_yield = AI_NONE;
        if in_possession && !holds_destination {
            let (t, yielded_to) = reserve_destination(
                state,
                side,
                idx,
                att_target,
                &mate_targets,
                &mate_target_ticks,
                &mate_res_yield,
                chaser,
                is_5v5,
            );
            att_target = t;
            um.res_yield = yielded_to;
        }
        if we_own && scored && !is_runner {
            // Re-check the lane on the FINAL target (after spacing, reservation and the run push).
            att_target = final_lane_nudge(state, side, att_target, &mut um);
        }
        raw = att_target;
    }

    // Anti-swarm spacing (engager cap): only the committed presser engages inside the contain gap;
    // everyone else (the lane-closers, the second man and the cover included) keeps a goal-side gap
    // of 2.9 m (3 m for the cover and the second man) from the carrier, outside the 2.5 m engager
    // radius, so at most two players (presser or human plus the contain mate) ever engage him.
    if let Some(ci) = opp_carrier {
        if !press_role {
            let c_pos = state.players[side.opponent().index()][ci].pos;
            let min_gap = Fp::from_milli(if me == cover || me == second || me == third {
                3_000
            } else {
                NON_ENGAGER_GAP_MM
            });
            let off = raw - c_pos;
            if me == cover && p.pos.dist(c_pos) < Fp::from_milli(4_500) {
                alpha_pm = alpha_pm.max(650);
            }
            if off.len() < min_gap {
                let away = (off.normalize_or_zero().scale(Fp::from_milli(400))
                    + (own_goal - c_pos).normalize_or_zero().scale(Fp::from_milli(600)))
                .normalize_or_zero();
                raw = c_pos + away.scale(min_gap);
            }
        }
    }

    let half_len_fp = state.format.half_length();
    let half_wid_fp = state.format.half_width();

    // Teammate yield radius (Konami US 2010/0240429): clear the class-0 teammate's near-term path
    // (our carrier's next 0.5 s, the elected chaser's run to his intercept point, or on a loose ball
    // nobody is elected for, the presser's run to his contain point). A teammate already standing
    // inside the radius steps out from where he is, so he does not cut across the reserved path.
    let loose = state.ball.owner.is_none() && !keeper_committed(state, side);
    let path = if let Some((os, oi)) = state.ball.owner.filter(|&(os, oi)| os == side && oi != idx) {
        let c = &state.players[os.index()][oi];
        Some((c.pos, c.pos + c.vel.scale(Fp::from_milli(500))))
    } else if loose && chaser != 0 && chaser != me {
        Some((state.players[side.index()][chaser as usize].pos, chaser_pt))
    } else if loose && chaser == 0 && presser != AI_NONE && presser != me {
        let pi = presser as usize;
        Some((state.players[side.index()][pi].pos, mate_targets.get(pi).copied().unwrap_or(ball_xy)))
    } else {
        None
    };
    if let Some((a, b)) = path {
        let (r, l) = yield_radius(is_5v5);
        // Back-line players hold the line height while defending: they yield only by target, and
        // sideways (same depth) whenever that clears the radius.
        let holds_line = is_defensive_phase(phase) && role.is_some_and(|r| r.is_back_line());
        let sticky = um.yield_sign;
        let yielded = yield_target_sided(a, b, raw, p.pos, r, l, half_len_fp, half_wid_fp, sticky).or_else(|| {
            (!holds_line)
                .then(|| yield_target_sided(a, b, p.pos, p.pos, r, l, half_len_fp, half_wid_fp, sticky))
                .flatten()
        });
        if let Some((t, sg)) = yielded {
            let sideways = Vec2::new(raw.x, t.y);
            raw = if holds_line && closest_on_segment(a, b, sideways).dist(sideways) >= r { sideways } else { t };
            yielding = true;
            um.yield_sign = sg;
        } else {
            um.yield_sign = 0;
        }
    } else {
        um.yield_sign = 0;
    }
    // Keeper-defender arbitration: while the keeper is committed to the loose ball, his line to the
    // claim point is reserved; defenders inside the corridor step aside laterally.
    if keeper_committed(state, side) {
        let gk = &state.players[side.index()][0];
        let brain = &state.gk_brain[side.index()];
        let claim = if brain.target == Vec2::ZERO { ball_xy } else { brain.target };
        if let Some((t, sg)) = keeper_corridor_yield(gk.pos, claim, raw, p.pos, is_5v5, um.gk_yield_sign) {
            raw = t;
            yielding = true;
            um.gk_yield_sign = sg;
        } else {
            um.gk_yield_sign = 0;
        }
    } else {
        um.gk_yield_sign = 0;
    }

    raw.x = raw.x.clamp(-(half_len_fp - Fp::from_milli(1_500)), half_len_fp - Fp::from_milli(1_500));
    raw.y = raw.y.clamp(-(half_wid_fp - Fp::from_milli(1_500)), half_wid_fp - Fp::from_milli(1_500));

    // Commit: filter the target so it cannot flip tick to tick (discipline -> faster tracking).
    // Round 4: a 0.4 m re-target deadband (a raw target that moves by less than a body radius does
    // not move the player) and an alpha-beta filter (role alpha, beta 0.02 ~ critically damped)
    // that tracks a moving slot without the first-order lag [peer-reviewed stability region; estimate].
    let continuous = prev_tick != 0 && prev_tick.wrapping_add(1) == state.tick;
    let raw = if continuous && raw.dist(um.tgt_raw) < RETARGET_DEADBAND { um.tgt_raw } else { raw };
    um.tgt_raw = raw;
    let target = if continuous {
        let pred = prev_target + um.tgt_vel;
        let resid = raw - pred;
        um.tgt_vel = (um.tgt_vel + resid.scale(TARGET_BETA)).clamp_len(TARGET_VEL_CAP);
        pred + resid.scale(Fp::from_milli(alpha_pm.clamp(80, 900)))
    } else {
        um.tgt_vel = Vec2::ZERO;
        raw
    };
    // A yielder's filtered target may not cut back inside the yield radius: the low-pass follows a
    // raw yield point that moves round the radius as he moves (a chord through the circle), so the
    // filtered target is projected out to R + margin from the reserved path, and the filter's
    // velocity is reset so it does not pull the target back in.
    let mut target = target;
    if let (true, Some((a, b))) = (yielding, path) {
        let (r, _) = yield_radius(is_5v5);
        let floor = r + YIELD_TARGET_MARGIN;
        let cl = closest_on_segment(a, b, target);
        let d = target.dist(cl);
        if d < floor {
            let out = if d > Fp::from_milli(5) { target - cl } else { p.pos - cl };
            let out = if out.len() > Fp::from_milli(5) { out.normalize_or_zero() } else { Vec2::new(Fp::ZERO, Fp::ONE) };
            target = cl + out.scale(floor);
            um.tgt_vel = Vec2::ZERO;
        }
    }

    // Arrive: proportional speed inside the slow radius (braking-consistent, d_slow = v^2 / 2a),
    // dead-zone set by positioning (160-510 mm, already the report's 0.4-0.5 m target radius band
    // for an average player; a hard 0.4 m floor left yielders short of the yield radius).
    let delta = target - p.pos;
    let dist = delta.len();
    let dead = Fp::from_milli(160 + (1_000 - style.discipline) * 35 / 100);
    let speed_now = p.vel.len();
    let mut move_vec = if press_role {
        let r = Fp::from_milli(450);
        if dist <= Fp::from_milli(120) {
            Vec2::ZERO
        } else {
            delta.normalize_or_zero().scale(dist.div(r).min(Fp::ONE))
        }
    } else if dist <= dead || (speed_now < Fp::from_milli(400) && dist <= dead.mul(Fp::from_int(2))) {
        Vec2::ZERO
    } else {
        // Braking-consistent arrive: command the speed from which a ~9 m/s^2 stop (with margin)
        // ends at the target, v = 0.75 * sqrt(2 * 9 * d), as a fraction of a ~7 m/s top speed.
        let v_ok = (dist - dead).mul(Fp::from_int(18)).sqrt().mul(Fp::from_milli(750));
        delta.normalize_or_zero().scale(v_ok.div(Fp::from_int(7)).min(Fp::ONE))
    };

    // Curved run around a blocker directly in the path (attacking phases only, decisive side).
    if !is_defensive_phase(phase) && move_vec.len() > Fp::from_milli(500) {
        let mv_n = move_vec.normalize_or_zero();
        for opp in state.players[side.opponent().index()].iter().skip(1) {
            if opp.sent_off {
                continue;
            }
            let rel = opp.pos - p.pos;
            let along = rel.dot(mv_n);
            if along > Fp::from_milli(150) && along < Fp::from_milli(2_200) {
                let perp_vec = rel - mv_n.scale(along);
                if perp_vec.len() < Fp::from_milli(1_050) {
                    let bend = Vec2::new(-mv_n.y, mv_n.x);
                    let side_dot = perp_vec.dot(bend);
                    // Sticky bend for the run (round 4): a bend chosen within the last
                    // BEND_HOLD_TICKS keeps its side, so a blocker almost dead ahead cannot flip it.
                    let fresh: i8 = if side_dot >= Fp::ZERO { -1 } else { 1 };
                    let held = um.bend_sign != 0 && state.tick.wrapping_sub(um.bend_tick) <= BEND_HOLD_TICKS;
                    let sg = if held { um.bend_sign } else { fresh };
                    um.bend_sign = sg;
                    um.bend_tick = state.tick;
                    let bend_sign = Fp::from_int(i32::from(sg));
                    move_vec = (mv_n.scale(Fp::from_milli(700)) + bend.scale(bend_sign.mul(Fp::from_milli(520))))
                        .normalize_or_zero()
                        .scale(move_vec.len());
                    break;
                }
            }
        }
    }

    // Held runner at the line: stutter at <= 3 m/s and never step back while onside (FIFA 13's fix
    // of runners reversing to stay onside [official]); an offside runner may drift back onside.
    if run_hold && !yielding {
        let onside_x = compute_onside_limit_x(state, side);
        let gap = (onside_x - p.pos.x).mul(Fp::from_int(attack_sign as i32));
        if gap < RUN_STUTTER_ZONE {
            move_vec = move_vec.clamp_len(RUN_STUTTER_FRAC);
            if gap > Fp::ZERO && move_vec.x.mul(Fp::from_int(attack_sign as i32)) < Fp::ZERO {
                move_vec.x = Fp::ZERO;
            }
        }
    }

    // Sprint & jockey decisions with hysteresis (no on/off toggling at a threshold).
    let sprint_hi = Fp::from_milli(if is_5v5 { 3_900 } else { 5_200 });
    let was_sprint = (prev_flags & FLAG_SPRINT) != 0 && continuous;
    let dist_sprint = if was_sprint { dist > sprint_hi.mul(Fp::from_milli(550)) } else { dist > sprint_hi };
    let press_sprint = press_role
        && dist > Fp::from_milli(1_400)
        && (trigger_sprint || (p.stamina_permille as i64) > 400 + (1_000 - style.work_rate) / 4);
    let counter_sprint = counter_run && dist > Fp::from_milli(3_000);
    // Round 4, area T: the report's high-speed band for defenders behind the ball but 10-12 m from
    // their slot was measured to slow the re-forming (11v11, 8 seeds: line_x_std 7.45 vs 6.67 m,
    // behind-ball at 6 s -1.5 pt), so far-from-target players keep sprinting (`dist_sprint`, the
    // pre-round-4 rule); the band is used after a capped recovery burst and by the lane-closers.
    let mut sprint = (p.run_boost_ticks > 0 && !opp_owns) || force_sprint || press_sprint || counter_sprint || dist_sprint;
    let was_jockey = (prev_flags & FLAG_JOCKEY) != 0 && continuous;
    let jockey_r = Fp::from_milli(if was_jockey { 3_200 } else { 2_600 });
    let jockey = opp_owns && is_defensive_phase(phase) && my_dist < jockey_r && !force_sprint;

    // Movement priority class and off-ball cruise speed: an uninvolved (class 3) player moves at
    // <= 70% of his top (sprint) speed and never sprints; only engagers and triggered runs sprint.
    // A recovery sprinter or a high-speed-band runner is class 2 (triggered), never class 3, even
    // while yielding (round 4, area T).
    let engaged = press_role || me == cover || me == second || me == third;
    let triggered = force_sprint
        || high_band
        || counter_run
        || reserved_run
        || p.run_boost_ticks > 0
        || (mark != AI_NONE && is_defensive_phase(phase))
        // Holding the defensive line is a tactical task, not idle cruising.
        || (is_defensive_phase(phase) && role.is_some_and(|r| r.is_back_line() || r == SlotRole::Holding))
        || dist_sprint;
    let class = if engaged {
        1
    } else if force_sprint || high_band || (!yielding && triggered) {
        2
    } else {
        3
    };
    if class == 3 {
        sprint = false;
        let sprint_mult = if is_5v5 { 1_205 } else { 1_220 };
        move_vec = move_vec.clamp_len(Fp::from_milli(CRUISE_TOP_SPEED_PM * sprint_mult / 1_000));
    } else if high_band && !force_sprint && !press_sprint {
        // High-speed band: sprint gait capped at a real 7 m/s (x 0.8 after a high-intensity peak),
        // relative to the stamina-scaled sprint top speed of sim.rs step_all_players, so a tired
        // player still reaches the band where his legs allow it.
        sprint = true;
        let sprint_mult = if is_5v5 { 1_205 } else { 1_220 };
        let top_mm_s = (5_450 + (p.attributes.pace as i64) * 26) * (760 + (p.stamina_permille as i64) * 240 / 1_000) / 1_000
            * sprint_mult
            / 1_000;
        let band_fat = if fatigue_pm < 1_000 { FATIGUE_BAND_PM } else { 1_000 };
        let band_pm = (HIGH_BAND_MM_S * band_fat / top_mm_s.max(1)).min(1_000);
        move_vec = move_vec.clamp_len(Fp::from_milli(band_pm));
    }
    if rec_now
        && state.ball.owner.is_none()
        && me != chaser
        && my_dist <= Fp::from_milli(LOOSE_STANDOFF_MM)
        && plan_idx(chaser).is_some_and(|c| {
            // Only a plausible second chaser: intercept within 40 ticks of the elected chaser.
            let wp = sample_ball_3d_waypoints(state);
            let mine = best_intercept_from_waypoints(p, &wp, is_5v5, false).1;
            mine <= best_intercept_from_waypoints(&state.players[side.index()][c], &wp, is_5v5, false).1 + LOOSE_STANDOFF_TICKS
        })
    {
        // Exclusivity on a loose ball (a pass or clearance in flight): a recovery runner near it
        // stands off at a jog instead of sprinting along its path, so only the elected chaser
        // runs at the ball (round 4 merge fix; the run stays latched and resumes on possession).
        sprint = false;
        let pace_mm_s = (5_450 + (p.attributes.pace as i64) * 26) * (760 + (p.stamina_permille as i64) * 240 / 1_000) / 1_000;
        move_vec = move_vec.clamp_len(Fp::from_milli((LOOSE_STANDOFF_MM_S * 1_000 / pace_mm_s.max(1)).min(1_000)));
    }

    {
        let mut mems = state.ai_memory.borrow_mut();
        let m = &mut mems[side.index()];
        if idx < m.target.len() {
            m.target[idx] = target;
            m.target_tick[idx] = state.tick;
            m.flags[idx] = (u8::from(sprint) * FLAG_SPRINT) | (u8::from(jockey) * FLAG_JOCKEY);
        }
        if let Some(c) = m.move_class.get_mut(idx) {
            *c = class;
        }
        um.write(m, idx);
        if let Some(r) = m.rec_flags.get_mut(idx) {
            *r = u8::from(rec_now);
        }
        if let Some(b) = m.burst_ticks.get_mut(idx) {
            *b = burst_now;
        }
    }

    AiMoveIntent {
        move_dir: move_vec,
        sprint,
        jockey,
        trigger_dive: None,
    }
}


/// Teammate yield radius `R` and displacement `L` (metres, estimate on Konami's unspecified values).
const fn yield_radius(is_5v5: bool) -> (Fp, Fp) {
    if is_5v5 {
        (Fp::from_milli(2_500), Fp::from_milli(1_500))
    } else {
        (Fp::from_milli(3_000), Fp::from_milli(2_000))
    }
}

/// GRF `GetSupportPosition_ForceField` teammate term, in possession only: the six nearest teammates
/// repel the target with weight 0.4 over a 10.5 m scale (6.5 m in 5v5), total shift capped.
fn support_spacing(state: &MatchState, side: TeamSide, idx: usize, target: Vec2, is_5v5: bool) -> Vec2 {
    let scale = Fp::from_milli(if is_5v5 { 6_500 } else { 10_500 });
    let cap = Fp::from_milli(if is_5v5 { 2_000 } else { 3_000 });
    let mut near: Vec<(Fp, usize)> = state.players[side.index()]
        .iter()
        .enumerate()
        .skip(1)
        .filter(|&(j, m)| j != idx && !m.sent_off)
        .map(|(j, m)| (m.pos.dist(target), j))
        .collect();
    near.sort_unstable();
    let mut push = Vec2::ZERO;
    for &(d, j) in near.iter().take(6) {
        if d >= scale {
            break;
        }
        let away = target - state.players[side.index()][j].pos;
        let dir = if away.len() > Fp::from_milli(150) {
            away.normalize_or_zero()
        } else if idx > j {
            Vec2::new(Fp::ZERO, Fp::ONE)
        } else {
            Vec2::new(Fp::ZERO, Fp::NEG_ONE)
        };
        push += dir.scale((scale - d).mul(Fp::from_milli(400)));
    }
    target + push.clamp_len(cap)
}

/// Hysteresis of the destination reservation: the pair's stored assignment holds unless the
/// player keeping the destination becomes 1.5 m farther from it than the yielder [estimate].
const RES_YIELD_MARGIN: Fp = Fp::from_milli(1_500);

/// One runner per destination (Konami US 8,616,946): if a teammate's committed target lies within
/// 6 m (5 m in 5v5) of ours, the player who is farther from that destination (ties: higher index)
/// moves his target out to that distance. The carrier and the chaser hold no destination.
/// Round 4: an assignment with memory (`res_yield`, who yielded to whom last tick): whoever
/// yielded last tick keeps yielding unless the other is farther by [`RES_YIELD_MARGIN`], so a
/// near-tie no longer swaps who moves. Returns the target and the teammate yielded to (`AI_NONE`).
#[allow(clippy::too_many_arguments)]
fn reserve_destination(
    state: &MatchState,
    side: TeamSide,
    idx: usize,
    mut target: Vec2,
    mate_targets: &[Vec2],
    mate_target_ticks: &[u32],
    res_yield: &[u8],
    chaser: u8,
    is_5v5: bool,
) -> (Vec2, u8) {
    let mut yielded_to = AI_NONE;
    let res = Fp::from_milli(if is_5v5 { 5_000 } else { 6_000 });
    let players = &state.players[side.index()];
    let me = &players[idx];
    let carrier = state.ball.owner.and_then(|(os, oi)| (os == side).then_some(oi));
    for _ in 0..2 {
        let mut moved = false;
        for (j, mate) in players.iter().enumerate().skip(1) {
            if j == idx || mate.sent_off || Some(j) == carrier || j as u8 == chaser || j >= mate_targets.len() {
                continue;
            }
            // Only live reservations (refreshed this tick or the previous one).
            let tt = mate_target_ticks.get(j).copied().unwrap_or(0);
            if tt == 0 || state.tick.wrapping_sub(tt) > 1 {
                continue;
            }
            let rj = mate_targets[j];
            let d = target.dist(rj);
            if d >= res {
                continue;
            }
            let d_mate = mate.pos.dist(rj);
            let d_me = me.pos.dist(rj);
            let i_yielded = res_yield.get(idx).copied() == Some(j as u8);
            let mate_yielded = res_yield.get(j).copied() == Some(idx as u8);
            let keep = if i_yielded {
                // I yielded last tick: keep yielding unless I am clearly (1.5 m) nearer.
                d_me + RES_YIELD_MARGIN < d_mate
            } else if mate_yielded {
                // He yielded to me last tick: I keep the destination unless clearly farther.
                d_me <= d_mate + RES_YIELD_MARGIN
            } else {
                d_me < d_mate || (d_me == d_mate && idx < j)
            };
            if keep {
                continue;
            }
            yielded_to = j as u8;
            let away = target - rj;
            let dir = if away.len() > Fp::from_milli(200) {
                away.normalize_or_zero()
            } else {
                let lat = Vec2::new(Fp::ZERO, if idx > j { Fp::ONE } else { Fp::NEG_ONE });
                if (rj + lat.scale(res)).y.abs() > state.format.half_width() - Fp::from_milli(1_500) {
                    lat.scale(Fp::NEG_ONE)
                } else {
                    lat
                }
            };
            target = rj + dir.scale(res);
            moved = true;
        }
        if !moved {
            break;
        }
    }
    (target, yielded_to)
}

/// While the keeper is committed, the corridor from the keeper to his claim point (extended 1.5 m
/// past it, half-width 1.8 m / 1.5 m in 5v5) is reserved: a defender whose target or position is
/// inside steps aside laterally to 1.2 m outside the corridor, on the side he is already on.
/// Round 4: the side is sticky for the episode (`sticky` = the sign chosen earlier, `0` = free),
/// so a defender near the keeper->claim line cannot flip sides; returns the target and the side.
fn keeper_corridor_yield(gk: Vec2, claim: Vec2, target: Vec2, me: Vec2, is_5v5: bool, sticky: i8) -> Option<(Vec2, i8)> {
    let half_w = Fp::from_milli(if is_5v5 { 1_500 } else { 1_800 });
    let dir = (claim - gk).normalize_or_zero();
    if dir == Vec2::ZERO {
        return None;
    }
    let end = claim + dir.scale(Fp::from_milli(1_500));
    let len = end.dist(gk);
    let perp = Vec2::new(-dir.y, dir.x);
    let inside = |q: Vec2| {
        let along = (q - gk).dot(dir);
        along > Fp::from_milli(-1_000) && along < len && (q - gk).dot(perp).abs() < half_w
    };
    if !inside(target) && !inside(me) {
        return None;
    }
    let lat_me = (me - gk).dot(perp);
    let sg: i8 = if sticky != 0 {
        sticky
    } else if lat_me >= Fp::ZERO {
        1
    } else {
        -1
    };
    let sgn = Fp::from_int(i32::from(sg));
    let base = if inside(target) { target } else { me };
    let along = (base - gk).dot(dir).clamp(Fp::ZERO, len);
    Some((gk + dir.scale(along) + perp.scale(sgn.mul(half_w + Fp::from_milli(1_200))), sg))
}

/// Continuous loose-ball intercept for an outfield chaser: linearly interpolates the zero crossing
/// of `(ticks needed - ticks available)` between consecutive 4-tick waypoints, so the chase target
/// slides along the ball path instead of snapping between waypoints (a major source of flicker).
pub fn smooth_ball_intercept(p: &PlayerState, waypoints: &[Vec3; 21], is_5v5: bool) -> Vec2 {
    let mut prev: Option<(i64, Vec2)> = None;
    for (i, pt3) in waypoints.iter().enumerate() {
        let t = ((i as u32 + 1) * 4) as i64;
        let pt = pt3.xy();
        let slack = estimate_ticks_to_point(p, pt, is_5v5) as i64 - t;
        if slack <= 0 && pt3.z <= Fp::from_milli(2_250) {
            return match prev {
                Some((ps, ppt)) if ps > 0 => {
                    let frac = Fp::from_ratio(ps, (ps - slack).max(1));
                    ppt.lerp(pt, frac)
                }
                _ => pt,
            };
        }
        prev = Some((slack, pt));
    }
    waypoints[20].xy()
}

/// CPU carrier evade-side hysteresis (WA1 decision side): once the carrier steps to one side of
/// the nearest opponent he keeps that side for at least `EVADE_DWELL_TICKS` (0.3 s, the WA1
/// incumbent dwell), and afterwards switches only when the other side scores `EVADE_MARGIN`
/// better (hysteresis band, control practice; values [estimate]). Memory lapses after
/// `EVADE_FORGET_TICKS` without an evade, or when the carrier changes.
const EVADE_DWELL_TICKS: u32 = 18;
const EVADE_MARGIN_MILLI: i64 = 200;
const EVADE_FORGET_TICKS: u32 = 10;

/// `true` = evade on the `a` side. Idempotent within a tick (a second call returns the same side).
fn evade_side(state: &MatchState, side: TeamSide, carrier: usize, score_a: Fp, score_b: Fp) -> bool {
    let tick = state.tick;
    let mut mems = state.ai_memory.borrow_mut();
    let m = &mut mems[side.index()];
    let fresh = m.steer_evade_sign != 0
        && m.steer_evade_idx as usize == carrier
        && m.steer_evade_tick != 0
        && tick.wrapping_sub(m.steer_evade_tick) <= EVADE_FORGET_TICKS;
    let raw_a = score_a >= score_b;
    let pick_a = if !fresh {
        raw_a
    } else if m.steer_evade_tick == tick {
        m.steer_evade_sign > 0
    } else {
        let held_a = m.steer_evade_sign > 0;
        let held_for = tick.wrapping_sub(m.steer_evade_since);
        let (held, other) = if held_a { (score_a, score_b) } else { (score_b, score_a) };
        if held_for < EVADE_DWELL_TICKS || other < held + Fp::from_milli(EVADE_MARGIN_MILLI) {
            held_a
        } else {
            !held_a
        }
    };
    let sign: i8 = if pick_a { 1 } else { -1 };
    if !fresh || sign != m.steer_evade_sign {
        m.steer_evade_since = tick;
    }
    m.steer_evade_sign = sign;
    m.steer_evade_idx = carrier as u8;
    m.steer_evade_tick = tick;
    pick_a
}

// -------------------------------------------------------------------------------------------------
// Movement model helpers used by the motion integration in `sim.rs`.
// -------------------------------------------------------------------------------------------------

/// Agility `0..=99` = mean of `acceleration` and `dribbling`.
#[inline]
pub fn agility(a: &crate::snapshot::PlayerAttributes) -> i64 {
    (a.acceleration as i64 + a.dribbling as i64) / 2
}

/// Closed-form per-tick coefficients of a critically damped spring (Juckett "Damped Springs";
/// charmbracelet/harmonica `spring.go`), precomputed offline for `dt = 1/60 s` so no `exp` runs
/// at 60 Hz. With smoothing time `T` (= the lag when tracking constant motion), `w = 2 / T`:
///
/// ```text
/// e = exp(-w dt)    te = dt e    tew = te w
/// pos_pos = tew + e    pos_vel = te    vel_pos = -w tew    vel_vel = e - tew
/// ```
///
/// - `T = 0.15 s` (off-ball movers): `w = 13.3333`, `e = 0.800737` ->
///   `0.978679, 0.013346 s, -2.372555 /s, 0.622796`.
/// - `T = 0.09 s` (CPU ball carrier): `w = 22.2222`, `e = 0.690479` ->
///   `0.946211, 0.011508 s, -5.682951 /s, 0.434746`.
///
/// `dt < 0.5 T` holds for both, the validity bound of the critically damped approximation.
struct SpringCoefs {
    pos_pos: Fp,
    pos_vel: Fp,
    vel_pos: Fp,
    vel_vel: Fp,
}

const STEER_SPRING_T150: SpringCoefs = SpringCoefs {
    pos_pos: Fp::from_ratio(978_679, 1_000_000),
    pos_vel: Fp::from_ratio(13_346, 1_000_000),
    vel_pos: Fp::from_ratio(-2_372_555, 1_000_000),
    vel_vel: Fp::from_ratio(622_796, 1_000_000),
};
/// The intended receiver of a pass is not filtered at all within this distance of the ball (he
/// must meet its line exactly); farther out he gets the short smoothing time [estimate].
pub const RECEIVER_UNFILTERED_MM: i64 = 5_000;
/// Below this commanded speed (arriving, jockeying, jogging into a slot) the short smoothing time
/// is used: the long lag would carry a slow mover past his target and back [estimate].
const STEER_FAST_BELOW_MM_S: i64 = 2_500;
const STEER_SPRING_T090: SpringCoefs = SpringCoefs {
    pos_pos: Fp::from_ratio(946_211, 1_000_000),
    pos_vel: Fp::from_ratio(11_508, 1_000_000),
    vel_pos: Fp::from_ratio(-5_682_951, 1_000_000),
    vel_vel: Fp::from_ratio(434_746, 1_000_000),
};

/// Steering-command filter of an AI/CPU-driven player (WA1). The command is split into direction
/// and speed:
///
/// - DIRECTION: a critically damped second-order filter (unit vector `x` with rate `r`,
///   equilibrium = the desired unit direction `u`), smoothing time `T = 0.15 s`; `0.09 s` for the
///   CPU ball carrier, the intended receiver of a pass and any command slower than 2.5 m/s
///   (arriving / jockeying), where the long lag would carry the player past his target [code for
///   the closed form; T estimate]. It never overshoots its equilibrium, and its rate state absorbs
///   a one-tick reversal of the desired direction that a first-order low-pass would pass on.
///   While the filtered direction vector is shorter than 0.2 (mid-reversal) the speed is scaled
///   down with it, so a reversal brakes through zero instead of swinging.
/// - SPEED: the previous first-order low-pass, `k = 0.25 + 0.002 * agility` per tick (~2-tick time
///   constant), which keeps arrive-ramp speed noise out of the speed profile.
///
/// The acceleration and turn-rate caps after it stay the hard limits. Resets to the raw command
/// whenever the filter was not run on the previous tick; a (near-)zero command keeps the direction
/// state and only ramps the speed down.
pub fn filter_velocity_command(
    mem: &mut TeamAiMemory,
    count: usize,
    idx: usize,
    tick: u32,
    attrs: &crate::snapshot::PlayerAttributes,
    fast: bool,
    desired: Vec2,
) -> Vec2 {
    mem.ensure_len(count);
    if idx >= mem.cmd_vel.len() {
        return desired;
    }
    let continuous = mem.cmd_tick[idx] != 0 && mem.cmd_tick[idx].wrapping_add(1) == tick;
    mem.cmd_tick[idx] = tick;
    let speed = desired.len();
    let moving = speed >= Fp::from_milli(50);
    let u = desired.normalize_or_zero();
    if !continuous {
        mem.cmd_vel[idx] = if moving { u } else { Vec2::ZERO };
        mem.steer_cmd_rate[idx] = Vec2::ZERO;
        mem.steer_cmd_speed[idx] = speed;
        return desired;
    }
    let k = Fp::from_milli((250 + agility(attrs) * 2).clamp(250, 450));
    let sp = mem.steer_cmd_speed[idx].lerp(speed, k);
    mem.steer_cmd_speed[idx] = sp;
    if mem.cmd_vel[idx] == Vec2::ZERO {
        if moving {
            mem.cmd_vel[idx] = u;
            mem.steer_cmd_rate[idx] = Vec2::ZERO;
        }
        return u.scale(sp);
    }
    if moving {
        let c = if fast || speed < Fp::from_milli(STEER_FAST_BELOW_MM_S) {
            &STEER_SPRING_T090
        } else {
            &STEER_SPRING_T150
        };
        let e = mem.cmd_vel[idx] - u;
        let r = mem.steer_cmd_rate[idx];
        mem.cmd_vel[idx] = u + e.scale(c.pos_pos) + r.scale(c.pos_vel);
        mem.steer_cmd_rate[idx] = e.scale(c.vel_pos) + r.scale(c.vel_vel);
    } else {
        mem.steer_cmd_rate[idx] = Vec2::ZERO;
    }
    let x = mem.cmd_vel[idx];
    let m = x.len();
    if m < Fp::from_milli(1) {
        return Vec2::ZERO;
    }
    let keep = m.mul(Fp::from_int(5)).min(Fp::ONE);
    x.normalize_or_zero().scale(sp.mul(keep))
}

/// Caps the per-tick heading change of a moving player to a lateral acceleration of
/// `11 + 0.08 * agility` m/s^2 (x1.6 for human-controlled players), i.e. `max_turn = a_lat / v`.
/// Applies only above `1.2 m/s` (AI) / `4.5 m/s` (human), so the first tick from rest and
/// straight-line braking are never limited; a turn beyond the cap bleeds up to 30% speed.
pub fn limit_turn_rate(
    old_vel: Vec2,
    new_vel: Vec2,
    attrs: &crate::snapshot::PlayerAttributes,
    human: bool,
) -> Vec2 {
    let old_sp = old_vel.len().to_milli();
    let new_sp = new_vel.len().to_milli();
    let min_sp = if human { 4_500 } else { 1_200 };
    if old_sp <= min_sp || new_sp <= 300 {
        return new_vel;
    }
    let mut a_lat = 11_000 + agility(attrs) * 80;
    if human {
        a_lat = a_lat * 16 / 10;
    }
    // 1 rad/tick == 60 rad/s; BAM16 per rad = 65536 / (2 pi) ~= 10430.4 -> per (rad/s) = 173.84.
    let max_bam = ((a_lat * 174) / old_sp).clamp(150, 4_000) as i32;
    let oh = old_vel.to_bam();
    let nh = new_vel.to_bam();
    let diff = (crate::math::angle_diff_bam(oh, nh) as i32).abs();
    if diff <= max_bam {
        return new_vel;
    }
    let h = crate::math::turn_toward_bam(oh, nh, max_bam as u16);
    let excess = (diff - max_bam) as i64;
    let bleed = (1_000 - excess * 300 / 16_384).clamp(700, 1_000);
    let sp = new_sp.min(old_sp) * bleed / 1_000;
    Vec2::from_bam(h, Fp::from_milli(sp))
}

/// Realistic velocity step for AI/CPU-driven outfielders off the ball, in polar form so it cannot
/// overshoot: speed changes by at most `6 + 0.03 * acceleration` m/s^2 when speeding up (6-9) and
/// `9 + 0.03 * acceleration` m/s^2 when braking (9-12); heading turns toward the command by at most
/// `a_lat / v` with `a_lat = 8 + 0.06 * agility` m/s^2 (the turn-rate limit). Asking for more than
/// a 90 degree turn brakes while turning. From (near) rest the change uses the accelerate cap.
pub fn realistic_velocity_step(
    old_vel: Vec2,
    desired: Vec2,
    attrs: &crate::snapshot::PlayerAttributes,
    _is_5v5: bool,
) -> Vec2 {
    let acc = attrs.acceleration as i64;
    let acc_cap = 6_000 + acc * 30; // mm/s^2
    let dec_cap = 9_000 + acc * 30;
    let lat_cap = 8_000 + agility(attrs) * 60;
    let s = old_vel.len().to_milli();
    let sd = desired.len().to_milli();
    if s < 500 {
        return old_vel + (desired - old_vel).clamp_len(Fp::from_milli(acc_cap / 60));
    }
    let oh = old_vel.to_bam();
    if sd < 300 {
        return Vec2::from_bam(oh, Fp::from_milli((s - dec_cap / 60).max(sd)));
    }
    let dh = desired.to_bam();
    let diff = (crate::math::angle_diff_bam(oh, dh) as i32).abs();
    // a_lat / v [rad/s] -> per tick in BAM16: * 65536 / (2 pi) / 60 = * 173.84
    let max_turn = ((lat_cap * 174) / s).clamp(120, 6_000) as u16;
    let h = crate::math::turn_toward_bam(oh, dh, max_turn);
    let target_sp = if diff > 16_384 { 0 } else { sd };
    let ns = if target_sp >= s {
        (s + acc_cap / 60).min(target_sp)
    } else {
        (s - dec_cap / 60).max(target_sp)
    };
    Vec2::from_bam(h, Fp::from_milli(ns))
}

/// Body-facing turn rate for AI/CPU-driven outfielders: `20 deg/tick` at rest falling to
/// `9 deg/tick` at 8.5 m/s, scaled `0.8 + 0.004 * agility`. While (nearly) standing the body pivots
/// at most 3.5 deg/tick and ignores changes under 4 deg; while moving changes under 1 deg are ignored,
/// so facing never twitches or dithers.
pub fn limit_facing_turn(
    cur: u16,
    target: u16,
    vel: Vec2,
    attrs: &crate::snapshot::PlayerAttributes,
) -> u16 {
    let spd = vel.len().to_milli().clamp(0, 8_500);
    let diff = (crate::math::angle_diff_bam(cur, target) as i32).abs();
    let dead = if spd < 500 { 728 } else { 182 };
    if diff < dead {
        return cur;
    }
    let scale = 800 + agility(attrs) * 4;
    let max_bam = if spd < 500 {
        // Slow, deliberate pivot while (nearly) standing: <= 3.5 deg/tick.
        637
    } else {
        let deg_x10 = 200 - (110 * spd) / 8_500;
        ((deg_x10 * 65_536 / 3_600) * scale / 1_000).clamp(600, 5_000) as u16
    };
    crate::math::turn_toward_bam(cur, target, max_bam)
}
