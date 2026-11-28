//! Space-ownership primitives for off-ball AI (round 4, work area 3; `docs/attacking-support-and-runs.md`).
//!
//! Everything here is pure, deterministic Q32.32 arithmetic on positions and velocities, with no
//! dependency on the match state, so the attacking support scorer (ai.rs) and later the defensive
//! marking arbiter can share it:
//!
//! - [`time_to_intercept`]: Spearman's simple arrival time, `0.7 s` reaction (carrying the current
//!   velocity) plus straight-line distance at `5 m/s` [peer-reviewed; LaurieOnTracking code].
//! - [`control`]: pitch control of a point for the attacking side, the discrete integration of
//!   Spearman (2018) with a logistic arrival spread `sigma = 0.45 s`, control rate
//!   `lambda = 4.3 /s` for attackers and `x1.72` for defenders, offside attackers zeroed.
//! - [`value_at`]: the negative-exponential distance-to-goal value `0.93 e^(-0.14 d) + 0.02`.
//! - [`lane_open`]: the xPass-style lane window, `5 speeds x 5 angles within +-16 deg` of the
//!   pass line, counting the passes the receiver reaches first.
//! - [`opponent_voronoi_vertices`] / [`nearest_voronoi_vertex`]: vertices of the Voronoi diagram of
//!   the opponents' positions (Gliders2d receiver points; EA US 7,588,493 run end-points).
//! - [`offside_line_point`]: the point on the onside limit nearest a reference point.
//!
//! The exponential and the logistic are fixed-point lookup tables (`EXP_NEG_LUT`, `LOGISTIC_LUT`)
//! with linear interpolation; no floating point is used anywhere.

use crate::math::{Fp, Vec2};

/// Reaction time of the time-to-intercept race (s) [peer-reviewed: Spearman 2018; code: LaurieOnTracking].
pub const TTI_REACTION: Fp = Fp::from_milli(700);
/// Top running speed of the time-to-intercept race (m/s) [peer-reviewed; code].
pub const TTI_MAX_SPEED: Fp = Fp::from_int(5);
/// Logistic arrival spread sigma (s) [peer-reviewed: Spearman 2018, 0.45-0.54 s].
pub const CONTROL_SIGMA_MS: i64 = 450;
/// `pi / (sqrt(3) * sigma)` for `sigma = 0.45 s` (1/s): the logistic slope of the arrival spread.
pub const CONTROL_LOGISTIC_K: Fp = Fp::from_milli(4_031);
/// Control rate lambda of attackers (1/s) [peer-reviewed; code].
pub const CONTROL_LAMBDA_ATT: Fp = Fp::from_milli(4_300);
/// Defenders control the ball 1.72x faster than attackers [peer-reviewed; code].
pub const CONTROL_DEF_FACTOR_PM: i64 = 1_720;
/// Integration step of the control model (s): 4 ticks at 60 Hz (the reference uses 0.04 s; the
/// coarser step keeps `lambda_def * dt` = 0.49 inside the stable range) [estimate].
pub const CONTROL_INT_DT: Fp = Fp::from_ratio(1, 15);
/// Integration horizon after the start of the race (s) [estimate; the reference uses 10 s but
/// stops at 99% convergence, which a 2.7 s window almost always reaches].
pub const CONTROL_INT_STEPS: usize = 40;
/// Convergence threshold of the integration (total control probability) [code].
pub const CONTROL_CONVERGED: Fp = Fp::from_milli(990);
/// Time for an attacker to be 99.9% sure of control, `3 ln 10 (sqrt3 sigma/pi + 1/lambda)` (s) [code].
pub const CONTROL_TTC_ATT: Fp = Fp::from_milli(3_320);
/// The same for a defender (s) [code].
pub const CONTROL_TTC_DEF: Fp = Fp::from_milli(2_648);

/// Value curve `VALUE_A * e^(-VALUE_K * d) + VALUE_B`, `d` = metres to the goal centre [peer-reviewed].
pub const VALUE_A: Fp = Fp::from_milli(930);
pub const VALUE_K: Fp = Fp::from_milli(140);
pub const VALUE_B: Fp = Fp::from_milli(20);

/// Pass speeds sampled by [`lane_open`] (m/s): a soft to a driven ground pass [estimate].
pub const LANE_SPEEDS_MPS: [i32; 5] = [10, 13, 16, 19, 22];
/// `(cos, sin)` of the sampled lane angles -16, -8, 0, +8, +16 degrees (xPass completed-pass
/// direction window 32.7 +- 16.3 deg [peer-reviewed]; five samples [estimate]).
pub const LANE_ANGLES: [(Fp, Fp); 5] = [
    (Fp::from_milli(961), Fp::from_milli(-276)),
    (Fp::from_milli(990), Fp::from_milli(-139)),
    (Fp::ONE, Fp::ZERO),
    (Fp::from_milli(990), Fp::from_milli(139)),
    (Fp::from_milli(961), Fp::from_milli(276)),
];
/// Reaction time of a defender stepping into a lane (s): a foot in the lane needs no settling of
/// the ball, so it is shorter than [`TTI_REACTION`] [estimate].
pub const LANE_REACTION: Fp = Fp::from_milli(300);
/// Leg reach of a defender at a lane (m) [estimate, one stride].
pub const LANE_REACH: Fp = Fp::from_milli(750);
/// How long after the ball a receiver may arrive at the pass end-point and still collect it (s) [estimate].
pub const LANE_RECEIVE_SLACK: Fp = Fp::from_milli(600);
/// Points sampled along each pass ray for defender interception (end-point included) [estimate].
pub const LANE_RAY_SAMPLES: i64 = 5;
/// Pressure radius at the receiver (m) [peer-reviewed: Anzer & Bauer, closest-opponent feature].
pub const PRESSURE_RADIUS: Fp = Fp::from_milli(4_600);

/// `e^(-x)` for `x = 0, 0.25, .., 16`, in units of `1/65536`.
const EXP_NEG_LUT: [i64; 65] = [
    65536, 51039, 39750, 30957, 24109, 18776, 14623, 11388, 8869, 6907, 5380, 4190, 3263, 2541, 1979,
    1541, 1200, 935, 728, 567, 442, 344, 268, 209, 162, 127, 99, 77, 60, 47, 36, 28, 22, 17, 13, 10, 8,
    6, 5, 4, 3, 2, 2, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
/// `1 / (1 + e^(-x))` for `x = -8, -7.75, .., 8`, in units of `1/65536`.
const LOGISTIC_LUT: [i64; 65] = [
    22, 28, 36, 47, 60, 77, 98, 126, 162, 208, 267, 342, 439, 562, 720, 922, 1179, 1506, 1921, 2446,
    3108, 3938, 4971, 6249, 7812, 9702, 11955, 14595, 17625, 21025, 24743, 28693, 32768, 36843, 40793,
    44511, 47911, 50941, 53581, 55834, 57724, 59287, 60565, 61598, 62428, 63090, 63615, 64030, 64357,
    64614, 64816, 64974, 65097, 65194, 65269, 65328, 65374, 65410, 65438, 65459, 65476, 65489, 65500,
    65508, 65514,
];
/// Grid step of both tables (0.25) as a raw Q32.32 value.
const LUT_STEP_RAW: i64 = Fp::ONE_RAW / 4;

/// Linear interpolation in a 65-entry, 0.25-step table starting at `x0`; clamped at both ends.
fn lut_interp(table: &[i64; 65], x: Fp, x0: Fp) -> Fp {
    let off = (x - x0).raw();
    if off <= 0 {
        return Fp::from_raw(table[0] << 16);
    }
    let i = (off / LUT_STEP_RAW) as usize;
    if i >= table.len() - 1 {
        return Fp::from_raw(table[table.len() - 1] << 16);
    }
    let frac = off % LUT_STEP_RAW;
    let a = table[i];
    let b = table[i + 1];
    // Q16 interpolation, then widen to Q32.32.
    let q16 = a + ((b - a) * (frac >> 14)) / (LUT_STEP_RAW >> 14);
    Fp::from_raw(q16 << 16)
}

/// `e^(-x)` for `x >= 0` (fixed-point LUT; `x` beyond 16 returns 0).
pub fn exp_neg(x: Fp) -> Fp {
    lut_interp(&EXP_NEG_LUT, x.max(Fp::ZERO), Fp::ZERO)
}

/// Logistic `1 / (1 + e^(-x))` (fixed-point LUT, saturating beyond +-8).
pub fn logistic(x: Fp) -> Fp {
    lut_interp(&LOGISTIC_LUT, x, Fp::from_int(-8))
}

/// One player in a space race: position, velocity and whether he is in an offside position (only
/// meaningful for attackers; an offside attacker cannot control the ball).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mover {
    pub pos: Vec2,
    pub vel: Vec2,
    pub offside: bool,
}

impl Mover {
    pub const fn new(pos: Vec2, vel: Vec2) -> Self {
        Self { pos, vel, offside: false }
    }
}

/// Spearman's simple time to intercept (s): keep the current velocity for the reaction time, then
/// run straight at `TTI_MAX_SPEED`: `0.7 s + |target - (pos + 0.7 vel)| / 5 m/s`.
pub fn time_to_intercept(m: &Mover, target: Vec2) -> Fp {
    let r = m.pos + m.vel.scale(TTI_REACTION);
    TTI_REACTION + r.dist(target).div(TTI_MAX_SPEED)
}

/// Probability that a player with arrival time `tti` has reached the ball by time `t`.
fn p_intercept(t: Fp, tti: Fp) -> Fp {
    logistic((t - tti).mul(CONTROL_LOGISTIC_K))
}

/// Maximum number of players per side a race considers.
const MAX_RACERS: usize = 12;

/// Pitch control of `target` for the attacking side (0..1): the probability that one of `attackers`
/// controls a ball arriving there after `ball_time` seconds, against `defenders` (Spearman 2018,
/// discrete integration as in LaurieOnTracking's `calculate_pitch_control_at_target`). Offside
/// attackers are ignored; with no eligible attacker the result is 0, with no defender it is 1.
pub fn control(target: Vec2, ball_time: Fp, attackers: &[Mover], defenders: &[Mover]) -> Fp {
    let mut att_t = [Fp::ZERO; MAX_RACERS];
    let mut def_t = [Fp::ZERO; MAX_RACERS];
    let mut na = 0;
    let mut nd = 0;
    let mut tau_att = Fp::from_int(1_000);
    let mut tau_def = Fp::from_int(1_000);
    for a in attackers.iter().filter(|a| !a.offside).take(MAX_RACERS) {
        let t = time_to_intercept(a, target);
        tau_att = tau_att.min(t);
        att_t[na] = t;
        na += 1;
    }
    if na == 0 {
        return Fp::ZERO;
    }
    for d in defenders.iter().take(MAX_RACERS) {
        let t = time_to_intercept(d, target);
        tau_def = tau_def.min(t);
        def_t[nd] = t;
        nd += 1;
    }
    if nd == 0 {
        return Fp::ONE;
    }
    // Shortcuts: one side arrives so much earlier that the other has no chance.
    if tau_att - ball_time.max(tau_def) >= CONTROL_TTC_DEF {
        return Fp::ZERO;
    }
    if tau_def - ball_time.max(tau_att) >= CONTROL_TTC_ATT {
        return Fp::ONE;
    }
    let lambda_def = CONTROL_LAMBDA_ATT.mul(Fp::from_milli(CONTROL_DEF_FACTOR_PM));
    let mut p_att = [Fp::ZERO; MAX_RACERS];
    let mut p_def = [Fp::ZERO; MAX_RACERS];
    let mut tot_att = Fp::ZERO;
    let mut tot_def = Fp::ZERO;
    // Start where the ball arrives, or shortly before the first player when he is later.
    let first = tau_att.min(tau_def) - Fp::from_milli(CONTROL_SIGMA_MS);
    let mut t = ball_time.max(first);
    for _ in 0..CONTROL_INT_STEPS {
        let free = Fp::ONE - tot_att - tot_def;
        if free <= Fp::ONE - CONTROL_CONVERGED {
            break;
        }
        let mut new_att = Fp::ZERO;
        for j in 0..na {
            // Only players within reach of the race contribute (the reference filters the same way).
            if att_t[j] - tau_att < CONTROL_TTC_ATT {
                let dp = free.mul(p_intercept(t, att_t[j])).mul(CONTROL_LAMBDA_ATT);
                p_att[j] += dp.mul(CONTROL_INT_DT);
            }
            new_att += p_att[j];
        }
        let mut new_def = Fp::ZERO;
        for j in 0..nd {
            if def_t[j] - tau_def < CONTROL_TTC_DEF {
                let dp = free.mul(p_intercept(t, def_t[j])).mul(lambda_def);
                p_def[j] += dp.mul(CONTROL_INT_DT);
            }
            new_def += p_def[j];
        }
        tot_att = new_att;
        tot_def = new_def;
        t += CONTROL_INT_DT;
    }
    let tot = tot_att + tot_def;
    if tot <= Fp::from_milli(1) {
        return Fp::HALF;
    }
    // Normalise an unconverged race (the remaining probability splits in proportion).
    tot_att.div(tot).clamp(Fp::ZERO, Fp::ONE)
}

/// Value of receiving at `point` for a side attacking `goal`: `0.93 e^(-0.14 d) + 0.02`, `d` in m.
pub fn value_at(point: Vec2, goal: Vec2) -> Fp {
    VALUE_A.mul(exp_neg(VALUE_K.mul(point.dist(goal)))) + VALUE_B
}

/// Lane arrival time of a defender at `pt` (s): short reaction, leg reach, `TTI_MAX_SPEED`.
fn lane_def_time(d: &Mover, pt: Vec2) -> Fp {
    let r = d.pos + d.vel.scale(LANE_REACTION);
    LANE_REACTION + (r.dist(pt) - LANE_REACH).max(Fp::ZERO).div(TTI_MAX_SPEED)
}

/// Fraction (0..1) of the `5 x 5` sampled passes from `from` toward `to` (speeds
/// [`LANE_SPEEDS_MPS`], angles [`LANE_ANGLES`] around the line) that `receiver` reaches first: no
/// defender gets to any sampled point of the ray before the ball, and the receiver reaches the end
/// of the ray no later than [`LANE_RECEIVE_SLACK`] after the ball and before every defender.
pub fn lane_open(from: Vec2, to: Vec2, receiver: &Mover, defenders: &[Mover]) -> Fp {
    let line = to - from;
    let d = line.len();
    if d <= Fp::from_milli(500) {
        return Fp::ONE;
    }
    let u = line.normalize_or_zero();
    let mut open = 0_i64;
    for &(c, s) in &LANE_ANGLES {
        let dir = Vec2::new(u.x.mul(c) - u.y.mul(s), u.x.mul(s) + u.y.mul(c));
        let end = from + dir.scale(d);
        let rec_t = time_to_intercept(receiver, end).min(lane_def_time(receiver, end) + Fp::from_milli(200));
        let mut def_end = Fp::from_int(1_000);
        for df in defenders {
            def_end = def_end.min(lane_def_time(df, end));
        }
        for &spd in &LANE_SPEEDS_MPS {
            let v = Fp::from_int(spd);
            let ball_end = d.div(v);
            if rec_t > ball_end + LANE_RECEIVE_SLACK || rec_t >= def_end {
                continue;
            }
            let mut cut = false;
            'ray: for k in 1..LANE_RAY_SAMPLES {
                let r = d.mul(Fp::from_ratio(k, LANE_RAY_SAMPLES));
                let pt = from + dir.scale(r);
                let tb = r.div(v);
                for df in defenders {
                    if lane_def_time(df, pt) <= tb {
                        cut = true;
                        break 'ray;
                    }
                }
            }
            if !cut {
                open += 1;
            }
        }
    }
    Fp::from_ratio(open, 25)
}

/// Pressure factor at a receiving point: 1 when no defender is within [`PRESSURE_RADIUS`], falling
/// linearly to 0.25 for a defender on the point.
pub fn pressure_factor(point: Vec2, defenders: &[Mover]) -> Fp {
    let mut near = PRESSURE_RADIUS;
    for d in defenders {
        near = near.min(d.pos.dist(point));
    }
    Fp::from_milli(250) + Fp::from_milli(750).mul(near.div(PRESSURE_RADIUS))
}

/// Circumcentre of the triangle `a b c`, or `None` when it is (nearly) degenerate.
fn circumcentre(a: Vec2, b: Vec2, c: Vec2) -> Option<Vec2> {
    // Work relative to `a` to keep the products small.
    let bx = b.x - a.x;
    let by = b.y - a.y;
    let cx = c.x - a.x;
    let cy = c.y - a.y;
    let d = (bx.mul(cy) - by.mul(cx)).mul(Fp::from_int(2));
    if d.abs() < Fp::from_milli(500) {
        return None;
    }
    let b2 = bx.mul(bx) + by.mul(by);
    let c2 = cx.mul(cx) + cy.mul(cy);
    let ux = (cy.mul(b2) - by.mul(c2)).div(d);
    let uy = (bx.mul(c2) - cx.mul(b2)).div(d);
    Some(Vec2::new(a.x + ux, a.y + uy))
}

/// Vertices of the Voronoi diagram of `sites` (normally the opponents' positions) that lie inside
/// the box `|x| <= half_len, |y| <= half_wid`: circumcentres of site triples with no other site
/// strictly closer (the empty-circle property). Order follows the site order, so the result is
/// deterministic. `O(n^4)` with `n <= 11`: about 1,800 distance checks.
pub fn opponent_voronoi_vertices(sites: &[Vec2], half_len: Fp, half_wid: Fp) -> Vec<Vec2> {
    let n = sites.len();
    let mut out = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                let Some(c) = circumcentre(sites[i], sites[j], sites[k]) else {
                    continue;
                };
                if c.x.abs() > half_len || c.y.abs() > half_wid {
                    continue;
                }
                let r = c.dist(sites[i]) - Fp::from_milli(10);
                if (0..n).all(|m| m == i || m == j || m == k || sites[m].dist(c) >= r) {
                    out.push(c);
                }
            }
        }
    }
    out
}

/// The Voronoi vertex of `sites` nearest to `query` within `max_dist`, inside the pitch box.
pub fn nearest_voronoi_vertex(sites: &[Vec2], query: Vec2, max_dist: Fp, half_len: Fp, half_wid: Fp) -> Option<Vec2> {
    let mut best: Option<(Fp, Vec2)> = None;
    for v in opponent_voronoi_vertices(sites, half_len, half_wid) {
        let d = v.dist(query);
        if d <= max_dist && best.is_none_or(|(bd, _)| d < bd) {
            best = Some((d, v));
        }
    }
    best.map(|(_, v)| v)
}

/// The point on the onside limit `line_x` (already including the safety margin) nearest `near`,
/// kept inside the touchlines.
pub fn offside_line_point(line_x: Fp, near: Vec2, half_wid: Fp) -> Vec2 {
    Vec2::new(line_x, near.y.clamp(-half_wid, half_wid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn luts_match_reference_values() {
        assert_eq!(exp_neg(Fp::ZERO), Fp::ONE);
        let e1 = exp_neg(Fp::ONE).to_milli();
        assert!((e1 - 368).abs() <= 1, "{e1}");
        assert_eq!(logistic(Fp::ZERO).to_milli(), 500);
        let l2 = logistic(Fp::from_int(2)).to_milli();
        assert!((l2 - 881).abs() <= 1, "{l2}");
        assert!(logistic(Fp::from_int(20)) <= Fp::ONE);
        assert!(logistic(Fp::from_int(-20)) >= Fp::ZERO);
    }

    #[test]
    fn control_favours_the_nearer_side_and_zeroes_offside() {
        let target = Vec2::from_milli(0, 0);
        let att = [Mover::new(Vec2::from_milli(2_000, 0), Vec2::ZERO)];
        let def = [Mover::new(Vec2::from_milli(12_000, 0), Vec2::ZERO)];
        let c = control(target, Fp::from_milli(800), &att, &def);
        assert!(c > Fp::from_milli(900), "{c:?}");
        let c2 = control(target, Fp::from_milli(800), &def.map(|m| Mover { offside: false, ..m }), &att);
        assert!(c2 < Fp::from_milli(100), "{c2:?}");
        let off = [Mover { offside: true, ..att[0] }];
        assert_eq!(control(target, Fp::ONE, &off, &def), Fp::ZERO);
        let even = control(target, Fp::ZERO, &[Mover::new(Vec2::from_milli(5_000, 0), Vec2::ZERO)], &[Mover::new(Vec2::from_milli(-5_000, 0), Vec2::ZERO)]);
        // Defenders control faster: an even race leans to the defence.
        assert!(even < Fp::HALF && even > Fp::from_milli(250), "{even:?}");
    }

    #[test]
    fn value_and_lane() {
        let goal = Vec2::from_milli(52_500, 0);
        assert!(value_at(Vec2::from_milli(42_500, 0), goal) > value_at(Vec2::from_milli(22_500, 0), goal));
        let rec = Mover::new(Vec2::from_milli(15_000, 0), Vec2::ZERO);
        let open = lane_open(Vec2::ZERO, rec.pos, &rec, &[]);
        assert!(open > Fp::from_milli(400), "{open:?}");
        let block = [Mover::new(Vec2::from_milli(7_000, 0), Vec2::ZERO)];
        assert!(lane_open(Vec2::ZERO, rec.pos, &rec, &block) < open);
    }

    #[test]
    fn voronoi_vertex_is_equidistant() {
        let sites = [Vec2::from_milli(0, 0), Vec2::from_milli(10_000, 0), Vec2::from_milli(0, 10_000)];
        let v = opponent_voronoi_vertices(&sites, Fp::from_int(60), Fp::from_int(40));
        assert_eq!(v.len(), 1);
        assert!((v[0].x.to_milli() - 5_000).abs() <= 2 && (v[0].y.to_milli() - 5_000).abs() <= 2);
        assert!(nearest_voronoi_vertex(&sites, Vec2::ZERO, Fp::from_int(25), Fp::from_int(60), Fp::from_int(40)).is_some());
    }
}
