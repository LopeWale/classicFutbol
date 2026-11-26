//! Predictive local avoidance, motion limits and position-only push-out.
//!
//! Pipeline per tick (see `docs/avoidance-and-contact.md`):
//!
//! 1. [`build_frame`] snapshots every player (position, velocity, priority class) once at the start
//!    of `step_all_players`, so the result never depends on the order players are stepped in.
//! 2. [`select_velocity`] (AI) / [`human_adjust`] (human) adjust the *desired* velocity with a
//!    DetourCrowd-style sampled reciprocal velocity selection, before any motion limiter runs.
//! 3. [`motion_step`] applies the two-regime acceleration curve, mode-dependent braking caps and a
//!    speed-dependent heading-rate cap; [`contact_constrain`] then removes only the velocity
//!    component that would drive into a body already in contact (tangential slide is kept).
//! 4. [`resolve_overlaps`] removes residual body overlap by moving positions only (slop, relaxation,
//!    fixed iterations, per-tick cap, yield/mass weights, lateral nudge for stuck pairs). It never
//!    reads or writes a velocity that feeds the simulation.
//!
//! Everything is Q32.32 fixed point; loops run in ascending player id with fixed iteration counts.

use crate::config::TeamSide;
use crate::math::{angle_diff_bam, turn_toward_bam, Fp, Vec2};
use crate::snapshot::PlayerAttributes;
use crate::state::{MatchPhase, MatchState, PlayerActionState};

// ---------------------------------------------------------------------------------------------
// Parameters (units in the names; evidence labels in docs/avoidance-and-contact.md)
// ---------------------------------------------------------------------------------------------

/// Body radius used by the push-out (centre distance `2 * 0.405 = 0.81 m`). Overlap deeper than the
/// 1 cm slop is resolved toward the full 0.81 m, so a resolved pair sits at `>= 0.80 m`.
pub const BODY_RADIUS_MM: i64 = 405;
/// Comfort radius per player, velocity stage only (pair comfort distance `1.0 m`).
pub const COMFORT_RADIUS_MM: i64 = 500;
/// Velocity-stage pair distance between opponents (just outside body contact, so markers and
/// challengers reach tackling range without driving into the other body).
pub const OPPONENT_CONTACT_MM: i64 = 850;
/// Time-to-collision horizon of the velocity stage.
pub const HORIZON_MS: i64 = 1_000;
/// Responsibility share between equal priority classes (RVO uses 0.5; each agent here assumes it
/// must realise 70% of the correction, a 1.4x over-avoid that covers the motion-limiter lag).
pub const SHARE_EQUAL_MILLI: i64 = 700;
/// Desired-velocity weight multiplier while giving way to a higher priority class.
pub const YIELD_WDES_MILLI: i64 = 400;
/// Time-to-impact weight multiplier while giving way to a higher priority class.
pub const YIELD_WTOI_MILLI: i64 = 2_000;
/// Comfort margin per unit closing speed, in ticks of motion-limiter lag.
pub const LAG_TICKS: i64 = 2;
/// Neighbours considered per agent, sorted by `(distance squared, id)`.
pub const MAX_NEIGHBOURS: usize = 8;
/// Frame-offset prediction instances (EA US 12,268,964 uses 4 and 8 frames as its examples).
pub const PREDICT_FRAMES: [i64; 3] = [4, 8, 12];
/// Sampling depth of the adaptive pattern (`17` samples per level).
pub const SAMPLE_DEPTH: usize = 3;
/// DetourCrowd defaults.
pub const VEL_BIAS_MILLI: i64 = 400;
pub const W_DES_MILLI: i64 = 2_000;
pub const W_CUR_MILLI: i64 = 750;
pub const W_SIDE_MILLI: i64 = 750;
pub const W_TOI_MILLI: i64 = 2_500;
/// Push-out: slop, relaxation, iterations, per-tick cap.
pub const PUSH_SLOP_MM: i64 = 10;
pub const PUSH_RELAX_MILLI: i64 = 600;
pub const PUSH_ITERS: usize = 3;
pub const PUSH_CAP_MM: i64 = 30;
/// Overlap depth above which the per-tick cap grows (10% of the extra depth) up to 6 cm.
pub const PUSH_DEEP_MM: i64 = 100;
pub const PUSH_CAP_DEEP_MM: i64 = 60;
/// Ticks a player may stay overlapped before the lateral "move to the side" nudge (FC 26).
pub const STUCK_TICKS: u8 = 10;
/// Lateral nudge per tick for a stuck pair (inside the per-tick cap).
pub const STUCK_NUDGE_MM: i64 = 15;
/// High-speed regime base of `a(v)` (mm/s^2, plus `20 * acceleration`).
pub const ACCEL_HI_BASE_MM_S2: i64 = 4_000;
/// Share of speed a blocked runner keeps as tangential slide along the body it touches.
pub const SLIDE_KEEP_MILLI: i64 = 600;
/// Human players: adjustment only above this speed (the first tick from rest is never touched).
pub const HUMAN_MIN_SPEED_MM_S: i64 = 1_500;
/// Human players: forward speed is kept at >= 90% of the command, lateral <= 30% of it.
pub const HUMAN_MIN_FORWARD_MILLI: i64 = 900;
pub const HUMAN_MAX_LATERAL_MILLI: i64 = 300;

/// Priority classes (lower = higher priority).
pub const CLASS_LOCKED: u8 = 0;
pub const CLASS_ENGAGER: u8 = 1;
pub const CLASS_TACTICAL: u8 = 2;
pub const CLASS_IDLE: u8 = 3;

// ---------------------------------------------------------------------------------------------
// Frame snapshot
// ---------------------------------------------------------------------------------------------

/// One player as seen by the avoidance stage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Agent {
    pub pos: Vec2,
    pub vel: Vec2,
    pub class: u8,
    /// `0` home, `1` away.
    pub team: u8,
    /// `true` when the agent carries the ball.
    pub carrier: bool,
    pub active: bool,
}

/// All players at the start of the tick, flattened as `id = team * stride + idx`.
#[derive(Clone, Debug, Default)]
pub struct Frame {
    pub agents: Vec<Agent>,
    pub stride: usize,
}

impl Frame {
    /// Build a frame from an explicit agent list (tests and tools); `stride` = agents per team.
    pub fn from_agents(agents: Vec<Agent>, stride: usize) -> Self {
        Self {
            agents,
            stride: stride.max(1),
        }
    }

    #[inline]
    pub fn id(&self, side_idx: usize, idx: usize) -> usize {
        side_idx * self.stride + idx
    }
}

/// THE single priority call site. Area I's team plan decides the class through
/// [`crate::ai::move_priority`] (0 carrier / elected chaser / committed keeper, 1 engager or the
/// human-controlled player, 2 tactical mover, 3 idle or yielding). Three avoidance-domain rules sit
/// on top of it:
///
/// - a player in a scripted action (tackle, kick, dive, stumble) is never steered or displaced;
/// - the intended receiver of a pass in flight is never steered off the ball's line;
/// - the keeper is never steered by avoidance, so outfielders always give way to him (class `<= 1`
///   even while he is not committed to a loose ball).
#[inline]
fn priority_class(state: &MatchState, side: TeamSide, idx: usize) -> u8 {
    let s = side.index();
    let Some(p) = state.players[s].get(idx) else {
        return CLASS_IDLE;
    };
    if p.sent_off {
        return CLASS_IDLE;
    }
    if !matches!(
        p.action,
        PlayerActionState::Normal | PlayerActionState::Cooldown { .. }
    ) {
        return CLASS_LOCKED;
    }
    if state.ball.owner.is_none()
        && matches!(state.phase, MatchPhase::OpenPlay)
        && state
            .ball
            .pass_in_flight
            .as_ref()
            .is_some_and(|pf| pf.team == side && pf.intended_receiver_idx == idx)
    {
        return CLASS_LOCKED;
    }
    let class = crate::ai::move_priority(state, side, idx);
    if idx == 0 {
        class.min(CLASS_ENGAGER)
    } else {
        class
    }
}

/// Snapshot all players and their priority classes (call once per tick, before any player moves).
pub fn build_frame(state: &MatchState) -> Frame {
    let stride = state.players[0].len().max(state.players[1].len()).max(1);
    let mut agents = vec![
        Agent {
            pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            class: CLASS_LOCKED,
            team: 0,
            carrier: false,
            active: false,
        };
        stride * 2
    ];
    for side in [TeamSide::Home, TeamSide::Away] {
        let s = side.index();
        for (i, p) in state.players[s].iter().enumerate() {
            agents[s * stride + i] = Agent {
                pos: p.pos,
                vel: p.vel,
                class: priority_class(state, side, i),
                team: s as u8,
                carrier: state.ball.owner == Some((side, i)),
                active: !p.sent_off,
            };
        }
    }
    Frame { agents, stride }
}

/// Priority class of every player this tick (`[home, away]`), for tests and tools.
pub fn priority_classes(state: &MatchState) -> [Vec<u8>; 2] {
    let f = build_frame(state);
    let mut out = [Vec::new(), Vec::new()];
    for (s, team) in state.players.iter().enumerate() {
        out[s] = (0..team.len())
            .map(|i| f.agents[f.id(s, i)].class)
            .collect();
    }
    out
}

// ---------------------------------------------------------------------------------------------
// Velocity stage
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct Circle {
    pos: Vec2,
    vel: Vec2,
    r: Fp,
    dp: Vec2,
    np: Vec2,
    /// `true`: the neighbour outranks us, we take all the responsibility.
    full: bool,
}

#[inline]
fn cross(a: Vec2, b: Vec2) -> Fp {
    a.x.mul(b.y) - a.y.mul(b.x)
}

/// Swept circle vs circle along relative velocity `v`: entry / exit times, `None` if no hit.
fn sweep(rel_pos: Vec2, r: Fp, v: Vec2) -> Option<(Fp, Fp)> {
    let a = v.len_sq();
    if a.raw() < Fp::from_milli(1).raw() {
        return None;
    }
    let c = rel_pos.len_sq() - r.mul(r);
    let b = v.dot(rel_pos);
    let d = b.mul(b) - a.mul(c);
    if d < Fp::ZERO {
        return None;
    }
    let rd = d.sqrt();
    Some(((b - rd).div(a), (b + rd).div(a)))
}

/// Pair comfort distance used only by the velocity stage. Opponents (markers, challengers on the
/// carrier) may close to just outside body contact; teammates keep the full comfort distance.
fn pair_radius(me: &Agent, other: &Agent) -> Fp {
    if other.team != me.team {
        Fp::from_milli(OPPONENT_CONTACT_MM)
    } else {
        Fp::from_milli(2 * COMFORT_RADIUS_MM)
    }
}

/// Neighbours relevant to `id` (farfield broadphase), sorted by `(dist^2, id)`, max 8.
fn gather(frame: &Frame, id: usize, v_des: Vec2) -> (Vec<Circle>, bool) {
    let me = frame.agents[id];
    let horizon = Fp::from_milli(HORIZON_MS);
    let my_reach = v_des.len().max(me.vel.len()).mul(horizon);
    let mut cand: Vec<(Fp, usize)> = Vec::with_capacity(MAX_NEIGHBOURS + 1);
    for (j, o) in frame.agents.iter().enumerate() {
        if j == id || !o.active {
            continue;
        }
        // Priority: a lower-priority neighbour yields to us entirely (we ignore it).
        if o.class > me.class {
            continue;
        }
        // Engagers (presser, second presser, cover, chaser, the human-controlled player: class <= 1)
        // do not avoid the opposing ball carrier: closing him down is their job, and his body is
        // handled by the contact rules (non-penetration slide, shoulder challenge, tackle). Without
        // this the 1 s constant-velocity sweep sees a presser's run as a collision and throttles him
        // to a crawl 4 m out, because the sweep does not know he will stop at his contain point.
        if o.carrier && o.team != me.team && me.class <= CLASS_ENGAGER {
            continue;
        }
        let rel = o.pos - me.pos;
        let r = pair_radius(&me, o);
        // Farfield: the two swept discs over the horizon cannot meet.
        let reach = my_reach + o.vel.len().mul(horizon) + r;
        if rel.x.abs() > reach || rel.y.abs() > reach {
            continue;
        }
        let d2 = rel.len_sq();
        if d2 > reach.mul(reach) {
            continue;
        }
        let pos = cand
            .iter()
            .position(|&(cd, cj)| d2 < cd || (d2 == cd && j < cj))
            .unwrap_or(cand.len());
        if pos < MAX_NEIGHBOURS {
            cand.insert(pos, (d2, j));
            cand.truncate(MAX_NEIGHBOURS);
        }
    }
    let mut out = Vec::with_capacity(cand.len());
    let mut imminent = false;
    for &(_, j) in &cand {
        let o = frame.agents[j];
        let rel = o.pos - me.pos;
        let dp = rel.normalize_or_zero();
        // Pass side: keep the neighbour on the side it already lies on relative to the approach
        // velocity (stable as the pair diverges); an exact tie keeps it on the left, which is the
        // complementary choice for both agents (Detour `np`, made order- and flip-free).
        let mut w = me.vel - o.vel;
        if w.len() < Fp::from_milli(200) {
            w = v_des - o.vel;
        }
        let np = if cross(w, rel) < Fp::from_milli(-10) {
            Vec2::new(dp.y, -dp.x)
        } else {
            Vec2::new(-dp.y, dp.x)
        };
        // EA frame-offset instances: predicted separation 4 / 8 / 12 frames ahead.
        let body = Fp::from_milli(2 * BODY_RADIUS_MM);
        for &k in &PREDICT_FRAMES {
            let t = Fp::from_ratio(k, 60);
            let d = (rel + (o.vel - v_des).scale(t)).len();
            if d < body {
                imminent = true;
            }
        }
        // Limiter-lag margin: the motion limits realise a new velocity over a few ticks, so the
        // comfort distance grows with the closing speed (`+ 2 ticks x |v_rel|`, at most 0.3 m).
        let margin = (me.vel - o.vel)
            .len()
            .mul(Fp::from_ratio(LAG_TICKS, 60))
            .min(Fp::from_milli(300));
        out.push(Circle {
            pos: o.pos,
            vel: o.vel,
            r: pair_radius(&me, &o) + margin,
            dp,
            np,
            full: o.class < me.class,
        });
    }
    (out, imminent)
}

/// Relative velocity for candidate `vc` under the responsibility split: all of it against a
/// higher-priority neighbour (`vc - v_o`), otherwise share `s`: `(vc - v_me) / s + v_me - v_o`
/// (`s = 0.5` is Detour/RVO's reciprocal `2 vc - v_me - v_o`).
#[inline]
fn rel_vel(me: &Agent, c: &Circle, vc: Vec2) -> Vec2 {
    if c.full {
        vc - c.vel
    } else {
        let d = vc - me.vel;
        let inv = Fp::ONE.div(Fp::from_milli(SHARE_EQUAL_MILLI));
        d.scale(inv) + me.vel - c.vel
    }
}

/// `true` when following `v` hits some comfort disc inside the horizon (or is already inside one).
fn threatened(me: &Agent, circles: &[Circle], v: Vec2) -> bool {
    let horizon = Fp::from_milli(HORIZON_MS);
    for c in circles {
        let rel = c.pos - me.pos;
        if rel.len_sq() < c.r.mul(c.r) {
            return true;
        }
        let vab = rel_vel(me, c, v);
        if let Some((t0, _)) = sweep(rel, c.r, vab) {
            if t0 >= Fp::ZERO && t0 < horizon {
                return true;
            }
        }
    }
    false
}

/// Detour `processSample`: cost of candidate velocity `vc`.
fn sample_cost(
    me: &Agent,
    circles: &[Circle],
    vc: Vec2,
    v_des: Vec2,
    inv_vmax: Fp,
    best: Fp,
    (w_des, w_toi): (Fp, Fp),
) -> Fp {
    let horizon = Fp::from_milli(HORIZON_MS);
    let vpen = w_des.mul((vc - v_des).len().mul(inv_vmax));
    let vcpen = Fp::from_milli(W_CUR_MILLI).mul((vc - me.vel).len().mul(inv_vmax));
    let base = vpen + vcpen;
    if base >= best {
        return best;
    }
    let mut tmin = horizon;
    let mut side = Fp::ZERO;
    for c in circles {
        let vab = rel_vel(me, c, vc);
        let s1 = c.dp.dot(vab).mul(Fp::HALF) + Fp::HALF;
        let s2 = c.np.dot(vab).mul(Fp::from_int(2));
        side += s1.min(s2).clamp(Fp::ZERO, Fp::ONE);
        let rel = c.pos - me.pos;
        if rel.len_sq() < c.r.mul(c.r) {
            // Already inside the comfort disc (Detour "avoid more strongly"): the longer this
            // candidate keeps us inside, the higher the cost; no relative motion = maximum cost.
            // Closing further (or passing through the neighbour) counts as an immediate hit.
            let t_exit = if vab.dot(rel) > Fp::ZERO {
                horizon
            } else {
                sweep(rel, c.r, vab).map_or(horizon, |(_, t1)| t1.min(horizon))
            };
            let t_eff = (horizon - t_exit).max(Fp::ZERO).mul(Fp::HALF);
            if t_eff < tmin {
                tmin = t_eff;
            }
        } else if let Some((t0, _)) = sweep(rel, c.r, vab) {
            if t0 >= Fp::ZERO && t0 < tmin {
                tmin = t0;
            }
        }
    }
    if !circles.is_empty() {
        side = side.div(Fp::from_int(circles.len() as i32));
    }
    let spen = Fp::from_milli(W_SIDE_MILLI).mul(side);
    let tpen = w_toi.div(Fp::from_milli(100) + tmin.div(horizon));
    base + spen + tpen
}

/// Sample pattern `(bam offset from the desired heading, radius permille)`: centre, ring of 8 at
/// radius 1, ring of 8 at radius 1/2 rotated half a step (Detour adaptive, 2 rings x 8 divisions).
const PATTERN: [(i32, i64); 17] = [
    (0, 0),
    (0, 1_000),
    (-8_192, 1_000),
    (8_192, 1_000),
    (-16_384, 1_000),
    (16_384, 1_000),
    (-24_576, 1_000),
    (24_576, 1_000),
    (32_768, 1_000),
    (4_096, 500),
    (-4_096, 500),
    (12_288, 500),
    (-12_288, 500),
    (20_480, 500),
    (-20_480, 500),
    (28_672, 500),
    (-28_672, 500),
];

fn solve(me: &Agent, circles: &[Circle], v_des: Vec2, v_max: Fp) -> Vec2 {
    let inv_vmax = Fp::ONE.div(v_max);
    // Giving way to a higher class: the desired-velocity weight is scaled down (report: w_priority).
    let yielding = circles.iter().any(|c| c.full);
    let weights = if yielding {
        (
            Fp::from_milli(W_DES_MILLI * YIELD_WDES_MILLI / 1_000),
            Fp::from_milli(W_TOI_MILLI * YIELD_WTOI_MILLI / 1_000),
        )
    } else {
        (Fp::from_milli(W_DES_MILLI), Fp::from_milli(W_TOI_MILLI))
    };
    let dir = if v_des.len() > Fp::from_milli(50) {
        v_des.to_bam()
    } else if me.vel.len() > Fp::from_milli(50) {
        me.vel.to_bam()
    } else {
        0
    };
    let units: Vec<Vec2> = PATTERN
        .iter()
        .map(|&(off, r)| {
            if r == 0 {
                Vec2::ZERO
            } else {
                Vec2::from_bam(dir.wrapping_add(off as u16), Fp::from_milli(r))
            }
        })
        .collect();
    let vmax_sq = (v_max + Fp::from_milli(1)).mul(v_max + Fp::from_milli(1));
    let mut res = v_des.scale(Fp::from_milli(VEL_BIAS_MILLI));
    let mut cr = v_max.mul(Fp::from_milli(1_000 - VEL_BIAS_MILLI));
    let mut best = Fp::from_int(1_000_000);
    for _ in 0..SAMPLE_DEPTH {
        let mut bvel = res;
        for u in &units {
            let vc = res + u.scale(cr);
            if vc.len_sq() > vmax_sq {
                continue;
            }
            let pen = sample_cost(me, circles, vc, v_des, inv_vmax, best, weights);
            if pen < best {
                best = pen;
                bvel = vc;
            }
        }
        res = bvel;
        cr = cr.mul(Fp::HALF);
    }
    res
}

/// Full avoidance for an AI-driven player: returns the adjusted desired velocity. Class-0 agents
/// and agents with no predicted conflict inside the horizon get `v_des` back unchanged.
pub fn select_velocity(frame: &Frame, id: usize, v_des: Vec2, v_max: Fp) -> Vec2 {
    let Some(me) = frame.agents.get(id).copied() else {
        return v_des;
    };
    if me.class == CLASS_LOCKED || !me.active {
        return v_des;
    }
    let (circles, _) = gather(frame, id, v_des);
    if circles.is_empty() || !threatened(&me, &circles, v_des) {
        return v_des;
    }
    let v_max = v_max.max(v_des.len()).max(Fp::from_milli(1_000));
    solve(&me, &circles, v_des, v_max)
}

/// Bounded avoidance for a human-controlled player: only while already moving (`>= 1.5 m/s`) and
/// only when body contact is predicted within 12 frames; forward speed along the stick is kept at
/// `>= 90%` of the command and the lateral offset is `<= 30%` of it, so heading never flips.
pub fn human_adjust(frame: &Frame, id: usize, v_cmd: Vec2, v_max: Fp) -> Vec2 {
    let Some(me) = frame.agents.get(id).copied() else {
        return v_cmd;
    };
    let cmd_sp = v_cmd.len();
    if me.class == CLASS_LOCKED
        || !me.active
        || me.vel.len() < Fp::from_milli(HUMAN_MIN_SPEED_MM_S)
        || cmd_sp < Fp::from_milli(500)
    {
        return v_cmd;
    }
    let (circles, imminent) = gather(frame, id, v_cmd);
    if !imminent || circles.is_empty() || !threatened(&me, &circles, v_cmd) {
        return v_cmd;
    }
    let sel = solve(&me, &circles, v_cmd, v_max.max(cmd_sp));
    let u = v_cmd.normalize_or_zero();
    let along = sel.dot(u);
    let fwd = along.clamp(cmd_sp.mul(Fp::from_milli(HUMAN_MIN_FORWARD_MILLI)), cmd_sp);
    let lat = (sel - u.scale(along)).clamp_len(cmd_sp.mul(Fp::from_milli(HUMAN_MAX_LATERAL_MILLI)));
    u.scale(fwd) + lat
}

/// Non-penetration constraint on the limited velocity (EA US10692266 style): a player may not close
/// on a body it touches (centre distance < body sum + 5 cm) faster than that body recedes plus the
/// remaining gap per tick. Only the closing component is removed, so contact slides tangentially
/// ("slippery", FC 26) instead of sticking. Applies to every class, the carrier included: it is a
/// contact rule, not avoidance steering.
pub fn contact_constrain(frame: &Frame, id: usize, v: Vec2) -> Vec2 {
    let Some(me) = frame.agents.get(id).copied() else {
        return v;
    };
    let body = Fp::from_milli(2 * BODY_RADIUS_MM);
    let touch = body + Fp::from_milli(50);
    let mut out = v;
    for (j, o) in frame.agents.iter().enumerate() {
        if j == id || !o.active {
            continue;
        }
        let rel = o.pos - me.pos;
        if rel.x.abs() > touch || rel.y.abs() > touch {
            continue;
        }
        let dist = rel.len();
        if dist >= touch || dist < Fp::from_milli(1) {
            continue;
        }
        let n = rel.normalize_or_zero();
        let gap_rate = (dist - body).max(Fp::ZERO).mul(Fp::from_int(60));
        let allowed = o.vel.dot(n).max(Fp::ZERO) + gap_rate;
        let closing = out.dot(n);
        if closing > allowed {
            let before = out.len();
            out -= n.scale(closing - allowed);
            // "Slippery" contact (FC 26): a runner blocked head-on slides along the body instead of
            // stalling, keeping 60% of its speed on the side it was already drifting to.
            let keep = before.mul(Fp::from_milli(SLIDE_KEEP_MILLI));
            if before > Fp::from_milli(1_000) && out.len() < keep {
                let t = Vec2::new(-n.y, n.x);
                let side = if out.dot(t) < Fp::ZERO {
                    Fp::NEG_ONE
                } else {
                    Fp::ONE
                };
                out = n.scale(out.dot(n)) + t.scale(keep.mul(side));
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------
// Motion limits
// ---------------------------------------------------------------------------------------------

/// Two-regime acceleration `a(v)` in mm/s^2: `8.8 + 0.012 * acc` (9.2-10.0) m/s^2 up to 4.2 m/s,
/// linear down to `3.0 + 0.02 * acc` (3.7-5.0) m/s^2 at 7.0 m/s, flat above.
pub fn accel_cap_mm_s2(speed_mm_s: i64, acceleration: u8) -> i64 {
    let acc = acceleration as i64;
    let lo = 8_800 + acc * 12;
    let hi = ACCEL_HI_BASE_MM_S2 + acc * 20;
    const V1: i64 = 4_200;
    const V2: i64 = 7_000;
    if speed_mm_s <= V1 {
        lo
    } else if speed_mm_s >= V2 {
        hi
    } else {
        lo - (lo - hi) * (speed_mm_s - V1) / (V2 - V1)
    }
}

/// Braking cap in mm/s^2 by mode: normal `9 + 0.03 * acc`; sprint (> 5 m/s) into jockey is slower,
/// `6.5 + 0.025 * acc`.
pub fn decel_cap_mm_s2(speed_mm_s: i64, acceleration: u8, jockey: bool) -> i64 {
    let acc = acceleration as i64;
    if jockey && speed_mm_s > 5_000 {
        6_500 + acc * 25
    } else {
        9_000 + acc * 30
    }
}

/// Heading-rate cap in BAM16 per tick: `a_lat / v` with `a_lat = 8 + 0.06 * agility` m/s^2, clamped
/// to `120..=720 deg/s` (2..12 deg per tick).
pub fn heading_cap_bam(speed_mm_s: i64, attrs: &PlayerAttributes) -> u16 {
    let lat = 8_000 + crate::ai::agility(attrs) * 60;
    // rad/s -> BAM16 per tick: * 65536 / (2 pi) / 60 = * 173.84
    ((lat * 174) / speed_mm_s.max(1)).clamp(364, 2_184) as u16
}

/// Polar velocity step for AI-driven outfielders off the ball (replaces the flat
/// `ai::realistic_velocity_step` caps with the two-regime curve and mode braking).
pub fn motion_step(old_vel: Vec2, desired: Vec2, attrs: &PlayerAttributes, jockey: bool) -> Vec2 {
    let s = old_vel.len().to_milli();
    let sd = desired.len().to_milli();
    let acc_cap = accel_cap_mm_s2(s, attrs.acceleration);
    let dec_cap = decel_cap_mm_s2(s, attrs.acceleration, jockey);
    if s < 500 {
        return old_vel + (desired - old_vel).clamp_len(Fp::from_milli(acc_cap / 60));
    }
    let oh = old_vel.to_bam();
    if sd < 300 {
        return Vec2::from_bam(oh, Fp::from_milli((s - dec_cap / 60).max(sd)));
    }
    let dh = desired.to_bam();
    let diff = (angle_diff_bam(oh, dh) as i32).abs();
    let h = turn_toward_bam(oh, dh, heading_cap_bam(s, attrs));
    let target_sp = if diff > 16_384 { 0 } else { sd };
    let ns = if target_sp >= s {
        (s + acc_cap / 60).min(target_sp)
    } else {
        (s - dec_cap / 60).max(target_sp)
    };
    Vec2::from_bam(h, Fp::from_milli(ns))
}

// ---------------------------------------------------------------------------------------------
// Push-out
// ---------------------------------------------------------------------------------------------

/// A body for the position-only push-out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Body {
    pub pos: Vec2,
    /// Read only (tangent direction of the stuck nudge); the push-out never changes it.
    pub vel: Vec2,
    pub class: u8,
    /// Mass proxy (`68 + 0.3 * strength`, plus shielding bonus).
    pub mass: i64,
    pub active: bool,
    /// Consecutive ticks this body has been overlapping someone (updated by `resolve_overlaps`).
    pub stuck_ticks: u8,
}

const YIELD: [i64; 4] = [0, 1, 2, 3];

/// Damped, capped, position-only overlap resolution in ascending pair order. Returns the
/// per-body displacement applied this tick.
pub fn resolve_overlaps(bodies: &mut [Body]) -> Vec<Vec2> {
    let n = bodies.len();
    let r = Fp::from_milli(2 * BODY_RADIUS_MM);
    let slop = Fp::from_milli(PUSH_SLOP_MM);
    let relax = Fp::from_milli(PUSH_RELAX_MILLI);
    let cap = Fp::from_milli(PUSH_CAP_MM);
    let nudge = Fp::from_milli(STUCK_NUDGE_MM);
    let mut total = vec![Vec2::ZERO; n];
    // Deepest overlap per body at the start of the pass (sets its per-tick cap).
    let mut deepest = vec![Fp::ZERO; n];
    for iter in 0..PUSH_ITERS {
        let mut disp = vec![Vec2::ZERO; n];
        let mut cnt = vec![0_i32; n];
        for a in 0..n {
            if !bodies[a].active {
                continue;
            }
            for b in (a + 1)..n {
                if !bodies[b].active {
                    continue;
                }
                let d = bodies[b].pos - bodies[a].pos;
                let dist = d.len();
                if dist + slop >= r {
                    continue;
                }
                let dir = if dist > Fp::from_milli(1) {
                    d.normalize_or_zero()
                } else {
                    // Coincident: separate along the perpendicular of a's velocity (index order).
                    let pv = bodies[a].vel.normalize_or_zero();
                    if pv == Vec2::ZERO {
                        Vec2::new(Fp::ONE, Fp::ZERO)
                    } else {
                        Vec2::new(-pv.y, pv.x)
                    }
                };
                if iter == 0 {
                    let depth = r - dist;
                    deepest[a] = deepest[a].max(depth);
                    deepest[b] = deepest[b].max(depth);
                }
                let pen = (r - dist).mul(relax);
                let wa = YIELD[bodies[a].class.min(3) as usize] * 1_000 / bodies[a].mass.max(1);
                let wb = YIELD[bodies[b].class.min(3) as usize] * 1_000 / bodies[b].mass.max(1);
                let (sa, sb) = if wa + wb == 0 {
                    // Two class-0 bodies (e.g. both teams' chasers): symmetric separation.
                    (Fp::HALF, Fp::HALF)
                } else {
                    (Fp::from_ratio(wa, wa + wb), Fp::from_ratio(wb, wa + wb))
                };
                let mut step = dir.scale(pen);
                if iter == 0
                    && (bodies[a].stuck_ticks > STUCK_TICKS || bodies[b].stuck_ticks > STUCK_TICKS)
                {
                    // FC 26: stuck pairs are moved to the side so they can detach.
                    let t = Vec2::new(-dir.y, dir.x);
                    let tv = (bodies[b].vel - bodies[a].vel).dot(t);
                    let sign = if tv < Fp::ZERO { Fp::NEG_ONE } else { Fp::ONE };
                    step += t.scale(nudge.mul(sign));
                }
                disp[a] -= step.scale(sa);
                disp[b] += step.scale(sb);
                cnt[a] += 1;
                cnt[b] += 1;
            }
        }
        for i in 0..n {
            if cnt[i] == 0 {
                continue;
            }
            let avg = Vec2::new(
                disp[i].x.div(Fp::from_int(cnt[i])),
                disp[i].y.div(Fp::from_int(cnt[i])),
            );
            // 3 cm per tick; overlaps deeper than 10 cm (restart placements, scripted lunges) may
            // take up to 6 cm per tick so they never persist.
            let deep_extra = (deepest[i] - Fp::from_milli(PUSH_DEEP_MM))
                .max(Fp::ZERO)
                .mul(Fp::from_milli(100));
            let cap_i = (cap + deep_extra).min(Fp::from_milli(PUSH_CAP_DEEP_MM));
            let want = (total[i] + avg).clamp_len(cap_i);
            let applied = want - total[i];
            bodies[i].pos += applied;
            total[i] = want;
        }
    }
    // Stuck counters: still overlapping after the pass.
    let mut over = vec![false; n];
    for a in 0..n {
        if !bodies[a].active {
            continue;
        }
        for b in (a + 1)..n {
            if bodies[b].active && (bodies[b].pos - bodies[a].pos).len() + slop < r {
                over[a] = true;
                over[b] = true;
            }
        }
    }
    for (b, o) in bodies.iter_mut().zip(over) {
        b.stuck_ticks = if o {
            b.stuck_ticks.saturating_add(1)
        } else {
            0
        };
    }
    total
}
