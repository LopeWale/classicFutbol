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

    // 2b. FIFA 22 / EA FC 27 Teammate Contain (`RB / R1` held by human on defense):
    //     Conservative `1.48m` contain gap in Tactical mode so user must switch & time manual Stand Tackle!
    if state.teammate_contain_idx[side.index()] == Some(idx)
        && state.ball.owner.map(|(s, _)| s) != Some(side)
    {
        let own_goal = Vec2::new(Fp::from_milli(-attack_sign * half_len.to_milli()), Fp::ZERO);
        let pred_ball = predict_ball_xy_at_ticks(state, 5);
        let is_legacy = state.control_settings[side.index()].defending_mode
            == crate::input::DefendingMode::Legacy;
        let hold_gap = Fp::from_milli(if is_legacy { 650 } else { 1_480 });
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

    // --- Without-Ball FC IQ Defensive Shape + EA FC 25/26 *Inside-Box Cutback Zone Marking* ---
    let def_shift = ball_attack_norm.mul(Fp::from_milli(340)) - Fp::from_milli(175);
    let mut def_target = dynamically_assigned_anchor(state, side, idx, def_shift);
    def_target.y = def_target.y.lerp(ball_xy.y, Fp::from_milli(280));
    let own_goal_x = Fp::from_milli(-attack_sign * half_len.to_milli());
    let own_goal = Vec2::new(own_goal_x, Fp::ZERO);
    let goal_half_w = state.format.goal_half_width();
    let my_dist = p.pos.dist(ball_xy);

    let opp_wide_cutback_threat = state.ball.owner.is_some_and(|(os, _)| os != side)
        && (ball_xy.x - own_goal_x).abs() <= Fp::from_milli(if is_5v5 { 13_500 } else { 18_000 })
        && ball_xy.y.abs() > goal_half_w + Fp::from_milli(1_600);

    if signed_bal < 150 && designated_ai_presser_idx(state, side) == Some(idx) {
        // Strictly ONE AI presser shadows goal-side at a disciplined contain gap (`1.55m`) without auto-tackling!
        let goal_side_offset = (own_goal - ball_xy)
            .normalize_or_zero()
            .scale(Fp::from_milli(1_550));
        def_target = ball_xy + goal_side_offset;
    } else if signed_bal < 200 {
        let opp_carrier = state.ball.owner.and_then(|(os, oi)| (os != side).then_some(oi));
        if opp_wide_cutback_threat {
            // EA FC 25/26 *Inside-Box Cutback Zone Marking* & *Defensive Awareness When Beat*:
            // Do NOT swarm the byline carrier — drop centrally into the penalty-spot cutback lane and mark box receivers!
            let cutback_hub = Vec2::new(
                own_goal_x + Fp::from_milli(attack_sign * (if is_5v5 { 5_800 } else { 7_800 })),
                ball_xy.y.mul(Fp::from_milli(220)),
            );
            let mut cutback_mark: Option<Vec2> = None;
            let mut best_d = Fp::from_milli(11_000);
            for (o_idx, opp) in state.players[side.opponent().index()].iter().enumerate().skip(1) {
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
                        let intercept_shade = (pass_lane.scale(Fp::from_milli(580))
                            + goal_side.scale(Fp::from_milli(420)))
                        .normalize_or_zero();
                        cutback_mark = Some(opp.pos + intercept_shade.scale(Fp::from_milli(1_100)));
                    }
                }
            }
            def_target = cutback_mark.unwrap_or(cutback_hub);
        } else {
            let mut mark_target: Option<Vec2> = None;
            let mut best_mark_d = Fp::from_milli(if is_5v5 { 8_200 } else { 8_500 });
            for (o_idx, opp) in state.players[side.opponent().index()].iter().enumerate().skip(1) {
                if Some(o_idx) == opp_carrier || opp.sent_off {
                    continue;
                }
                if opp.pos.x.mul(Fp::from_int(attack_sign as i32)) > half_len.mul(Fp::from_milli(260)) {
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
                def_target = def_target.lerp(mt, Fp::from_milli(560));
            }
        }
    }

    // --- With-Ball FC IQ Role Activity Maps + 5v5 Rush Fluid Positioning & Curved Spatial Runs ---
    let att_shift = if is_5v5 {
        ball_attack_norm.mul(Fp::from_milli(470)) + Fp::from_milli(220)
    } else {
        ball_attack_norm.mul(Fp::from_milli(440)) + Fp::from_milli(200)
    };
    let mut att_target = dynamically_assigned_anchor(state, side, idx, att_shift);
    let lateral_pull = if is_5v5 {
        Fp::from_milli(180)
    } else {
        Fp::from_milli(240)
    };
    att_target.y = att_target.y.lerp(ball_xy.y, lateral_pull);

    // In 5v5 Rush, keep the deepest non-carrier outfielder as a fluid Rest-Defense Pivot/Fixo behind the ball,
    // while pulling an off-ball attacker into vacated central cutback box space (*EA FC 26 Role Flexibility*)!
    if is_5v5 && signed_bal > 80 {
        let carrier_opt = state.ball.owner.and_then(|(os, oi)| (os == side).then_some(oi));
        let mut deepest_idx: Option<usize> = None;
        let mut min_prog = i64::MAX;
        let mut central_box_occupied = false;
        let opp_goal_x = Fp::from_milli(attack_sign * half_len.to_milli());
        for (m_i, mate) in state.players[side.index()].iter().enumerate().skip(1) {
            if Some(m_i) == carrier_opt || mate.sent_off {
                continue;
            }
            let mp = mate.pos.x.to_milli() * attack_sign;
            if mp < min_prog {
                min_prog = mp;
                deepest_idx = Some(m_i);
            }
            if m_i != idx
                && (mate.pos.x - opp_goal_x).abs() <= Fp::from_milli(11_500)
                && mate.pos.y.abs() <= Fp::from_milli(4_800)
            {
                central_box_occupied = true;
            }
        }
        if deepest_idx == Some(idx) && p.run_boost_ticks == 0 {
            let pivot_proj = (ball_xy.x.to_milli() * attack_sign - 8_800)
                .clamp(-(half_len.to_milli() - 7_500), 6_500);
            att_target = Vec2::new(
                Fp::from_milli(attack_sign * pivot_proj),
                ball_xy.y.mul(Fp::from_milli(340)),
            );
        } else if !central_box_occupied
            && ball_xy.x.to_milli() * attack_sign > half_len.to_milli() / 3
            && ball_xy.y.abs() > goal_half_w + Fp::from_milli(1_500)
            && designated_support_triangle_idx(state, side) != Some(idx)
        {
            // Vacated central penalty-spot cutback target!
            att_target = Vec2::new(
                opp_goal_x - Fp::from_milli(attack_sign * 6_800),
                -ball_xy.y.clamp(Fp::from_milli(-6_000), Fp::from_milli(6_000)).mul(Fp::from_milli(220)),
            );
        }
    }

    // Designated short-support triangle player checks to an open 8.5–12.0m angle from the ball carrier!
    if signed_bal > 100 && designated_support_triangle_idx(state, side) == Some(idx) {
        let to_me = att_target - ball_xy;
        let dist_b = to_me.len();
        if dist_b > Fp::from_milli(if is_5v5 { 13_800 } else { 14_500 }) {
            let check_dir = if dist_b > Fp::from_milli(100) {
                to_me.normalize_or_zero()
            } else {
                Vec2::new(Fp::from_int(attack_sign as i32), Fp::from_milli(500)).normalize_or_zero()
            };
            att_target = ball_xy + check_dir.scale(Fp::from_milli(if is_5v5 { 9_500 } else { 11_800 }));
        }
    }

    // EA FC 27 *Attacking Spatial Awareness* Pocket & Curved-Run Search:
    // Bend around opposing defenders & cover shadows while preserving wide Rush spacing (`>= 6.5m`)!
    let mut best_pocket = att_target;
    let mut best_pocket_score = Fp::from_int(-999);
    let cand_offsets = [
        Vec2::ZERO,
        Vec2::from_milli(attack_sign * 2_800, 2_600),
        Vec2::from_milli(attack_sign * 2_800, -2_600),
        Vec2::from_milli(-attack_sign * 1_800, if idx.is_multiple_of(2) { 3_400 } else { -3_400 }),
        Vec2::from_milli(attack_sign * 3_600, 0),
    ];
    let min_mate_sep = Fp::from_milli(if is_5v5 { 6_600 } else { 5_500 });
    for (c_i, off) in cand_offsets.iter().enumerate() {
        let cand = att_target + *off;
        let d_ball = cand.dist(ball_xy);
        if d_ball < Fp::from_milli(5_000) {
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
                    if perp < Fp::from_milli(1_850) {
                        lane_shadow_pen = lane_shadow_pen.max(Fp::from_milli(2_850) - perp);
                    }
                }
            }
        }
        let mut mate_crowd_pen = Fp::ZERO;
        for (m_i, mate) in state.players[side.index()].iter().enumerate().skip(1) {
            if m_i != idx && !mate.sent_off {
                let dm = cand.dist(mate.pos);
                if dm < min_mate_sep {
                    mate_crowd_pen += min_mate_sep - dm;
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

    // FIFA 21 *"Passer Readiness"* + FIFA 23/FC 27 Onside Channel Runs:
    // Time forward darts when our carrier is facing forward (`passer_ready`) or when `run_boost_ticks > 0`,
    // and hold/bend along `onside_limit_x` (`±1/3` Rush attacking-third line in 5v5)!
    let passer_ready = match state.ball.owner {
        Some((o_side, o_idx)) if o_side == side => {
            let carrier = &state.players[o_side.index()][o_idx];
            let c_fwd = Vec2::from_bam(carrier.facing_bam, Fp::ONE);
            c_fwd.x.mul(Fp::from_int(attack_sign as i32)) > Fp::from_milli(-150)
        }
        _ => true,
    };
    let is_runner_role = p.role == PlayerRole::Forward
        || (p.role == PlayerRole::Midfielder
            && ball_attack_norm > Fp::from_milli(-80)
            && (p.base_anchor_permille.0 >= -140 || p.base_anchor_permille.1.abs() >= 350));
    if p.run_boost_ticks > 0 || (is_runner_role && signed_bal > 120 && passer_ready) {
        let onside_limit_x = compute_onside_limit_x(state, side);
        let run_push = Fp::from_milli(if p.run_boost_ticks > 0 {
            if is_5v5 { 7_800 } else { 11_500 }
        } else if p.role == PlayerRole::Forward {
            if is_5v5 { 5_600 } else { 7_800 }
        } else {
            if is_5v5 { 3_800 } else { 5_600 }
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
    let mut move_vec = if delta.len() > Fp::from_milli(300) {
        delta.normalize_or_zero()
    } else {
        Vec2::ZERO
    };
    // EA FC 27 Curved Run Around Blockers: bend off-ball run laterally around an opponent standing directly in path!
    if signed_bal > 80 && move_vec.len() > Fp::from_milli(500) {
        for opp in state.players[side.opponent().index()].iter().skip(1) {
            if opp.sent_off {
                continue;
            }
            let rel = opp.pos - p.pos;
            let along = rel.dot(move_vec);
            if along > Fp::from_milli(150) && along < Fp::from_milli(2_200) {
                let perp_vec = rel - move_vec.scale(along);
                if perp_vec.len() < Fp::from_milli(1_050) {
                    let bend = Vec2::new(-move_vec.y, move_vec.x);
                    let bend_sign = if perp_vec.dot(bend) >= Fp::ZERO {
                        Fp::NEG_ONE
                    } else {
                        Fp::ONE
                    };
                    move_vec = (move_vec.scale(Fp::from_milli(700))
                        + bend.scale(bend_sign.mul(Fp::from_milli(520))))
                    .normalize_or_zero();
                    break;
                }
            }
        }
    }

    let dist = delta.len();
    let sprint_thresh = if is_5v5 {
        Fp::from_milli(3_900)
    } else {
        Fp::from_milli(5_200)
    };
    AiMoveIntent {
        move_dir: move_vec,
        sprint: (p.run_boost_ticks > 0 && signed_bal >= 0) || dist > sprint_thresh,
        jockey: signed_bal < -150 && my_dist < Fp::from_milli(2_600),
        trigger_dive: None,
    }
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

            let mut best_pass: Option<(Vec2, Fp, u16)> = None;

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
                let run_dir = if mate.vel.len() > Fp::from_milli(1_100) {
                    mate.vel.normalize_or_zero()
                } else {
                    Vec2::new(Fp::from_int(attack_sign as i32), Fp::ZERO)
                };
                let raw_through = mate.pos + run_dir.scale(lead_step);
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
                    let ground_safe = !point_blank_blocked && worst_margin_ticks >= req_safety;
                    let can_loft_switch = !ground_safe
                        && !is_through_cand
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
                let killer_ball = score > Fp::from_milli(if open_carry_lane { 8_200 } else { 4_600 });
                let pressured_progression = moderate_pressure && score > Fp::from_milli(2_200);
                let safe_recycle = tight_pressure && score > Fp::from_milli(-1_800);
                if killer_ball || pressured_progression || safe_recycle {
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

    // --- OFF-BALL CPU DEFENDING ---
    let ball_xy = state.ball.pos.xy();
    let my_dist = me.pos.dist(ball_xy);
    let (my_intercept, _) = best_ball_intercept_for_player(state, side, ctrl_idx);

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

    let move_dir = (goal_side_pt - me.pos).normalize_or_zero();
    let mut buttons = 0_u16;
    let sprint_close_dist = match difficulty {
        CpuDifficulty::Easy => Fp::from_milli(3_200),
        CpuDifficulty::Normal => Fp::from_milli(2_550),
        CpuDifficulty::Hard => Fp::from_milli(1_950),
    };
    if my_dist > sprint_close_dist || state.ball.owner.is_none() {
        buttons |= BTN_SPRINT;
    } else {
        buttons |= BTN_JOCKEY | BTN_CONTAIN;
    }

    let (tackle_cadence, tackle_reach_mm): (u32, i64) = match difficulty {
        CpuDifficulty::Easy => (30, 1_020),
        CpuDifficulty::Normal => (20, 1_100),
        CpuDifficulty::Hard => (12, 1_180),
    };
    if let Some((o_side, o_idx)) = state.ball.owner {
        if o_side != side && state.ball.possession_grace_ticks == 0 {
            let carrier = &state.players[o_side.index()][o_idx];
            let c_fwd = Vec2::from_bam(carrier.facing_bam, Fp::ONE);
            let def_to_c = (carrier.pos - me.pos).normalize_or_zero();
            let front_or_side = def_to_c.dot(c_fwd) < Fp::from_milli(480);
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
