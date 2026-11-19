//! Fixed-point Q32.32 arithmetic and deterministic 2D/3D vector math.
//!
//! No floating-point types or operations are used anywhere in simulation state.

use core::fmt;
use core::ops::{Add, AddAssign, Neg, Sub, SubAssign};
use serde::{Deserialize, Serialize};

/// Q32.32 signed fixed-point number (`1.0 == 1 << 32`).
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Fp(pub i64);

impl Fp {
    pub const FRAC_BITS: u32 = 32;
    pub const ONE_RAW: i64 = 1_i64 << Self::FRAC_BITS;
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(Self::ONE_RAW);
    pub const NEG_ONE: Self = Self(-Self::ONE_RAW);
    pub const HALF: Self = Self(Self::ONE_RAW >> 1);

    /// `pi / 2` in Q32.32 (`1.5707963267948966 * 2^32`).
    const PI_HALF_RAW: i64 = 6_746_518_852;

    #[inline]
    pub const fn from_raw(raw: i64) -> Self {
        Self(raw)
    }

    #[inline]
    pub const fn raw(self) -> i64 {
        self.0
    }

    #[inline]
    pub const fn from_int(v: i32) -> Self {
        Self((v as i64) << Self::FRAC_BITS)
    }

    /// Construct from millimeters (or thousandths of a unit): `1000 -> 1.0`.
    #[inline]
    pub const fn from_milli(mm: i64) -> Self {
        let wide = ((mm as i128) << Self::FRAC_BITS) / 1000_i128;
        Self(clamp_i128_to_i64(wide))
    }

    /// Convert to integer millimeters (thousandths of a unit), truncated toward zero.
    #[inline]
    pub const fn to_milli(self) -> i64 {
        let wide = ((self.0 as i128) * 1000_i128) / (Self::ONE_RAW as i128);
        clamp_i128_to_i64(wide)
    }

    /// Construct from integer ratio `num / den`. Panics if `den == 0`.
    #[inline]
    pub const fn from_ratio(num: i64, den: i64) -> Self {
        let wide = ((num as i128) << Self::FRAC_BITS) / (den as i128);
        Self(clamp_i128_to_i64(wide))
    }

    #[inline]
    pub const fn abs(self) -> Self {
        Self(self.0.saturating_abs())
    }

    #[inline]
    pub const fn min(self, other: Self) -> Self {
        if self.0 < other.0 { self } else { other }
    }

    #[inline]
    pub const fn max(self, other: Self) -> Self {
        if self.0 > other.0 { self } else { other }
    }

    #[inline]
    pub const fn clamp(self, lo: Self, hi: Self) -> Self {
        if self.0 < lo.0 {
            lo
        } else if self.0 > hi.0 {
            hi
        } else {
            self
        }
    }

    #[inline]
    pub const fn signum_int(self) -> i32 {
        if self.0 > 0 {
            1
        } else if self.0 < 0 {
            -1
        } else {
            0
        }
    }

    #[inline]
    pub const fn mul(self, rhs: Self) -> Self {
        let prod = ((self.0 as i128) * (rhs.0 as i128)) >> Self::FRAC_BITS;
        Self(clamp_i128_to_i64(prod))
    }

    #[inline]
    pub const fn div(self, rhs: Self) -> Self {
        if rhs.0 == 0 {
            return Self::ZERO;
        }
        let quot = ((self.0 as i128) << Self::FRAC_BITS) / (rhs.0 as i128);
        Self(clamp_i128_to_i64(quot))
    }

    /// Deterministic integer square root: `sqrt(x)` in Q32.32.
    #[inline]
    pub const fn sqrt(self) -> Self {
        if self.0 <= 0 {
            return Self::ZERO;
        }
        let radicand = (self.0 as u128) << Self::FRAC_BITS;
        let root = radicand.isqrt();
        Self(root as i64)
    }

    #[inline]
    pub const fn lerp(self, target: Self, t: Self) -> Self {
        let diff = Self(target.0.saturating_sub(self.0));
        Self(self.0.saturating_add(diff.mul(t).0))
    }
}

#[inline]
const fn clamp_i128_to_i64(v: i128) -> i64 {
    if v > (i64::MAX as i128) {
        i64::MAX
    } else if v < (i64::MIN as i128) {
        i64::MIN
    } else {
        v as i64
    }
}

impl Add for Fp {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(self.0.saturating_add(rhs.0))
    }
}

impl AddAssign for Fp {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0.saturating_add(rhs.0);
    }
}

impl Sub for Fp {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }
}

impl SubAssign for Fp {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0.saturating_sub(rhs.0);
    }
}

impl Neg for Fp {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(self.0.saturating_neg())
    }
}

impl fmt::Debug for Fp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mm = self.to_milli();
        let sign = if mm < 0 { "-" } else { "" };
        let abs_mm = mm.unsigned_abs();
        write!(f, "{}{}.{:03}", sign, abs_mm / 1000, abs_mm % 1000)
    }
}

/// Deterministic sine for 16-bit Binary Angle Measurement (`0..65536` = `[0, 2π)`).
pub fn sin_bam(angle: u16) -> Fp {
    let quadrant = angle >> 14; // 0..3
    let offset = (angle & 0x3FFF) as u32; // 0..16383
    let q_steps = match quadrant {
        0 => offset,
        1 => 16384 - offset,
        2 => offset,
        _ => 16384 - offset,
    };
    let mag = sin_quadrant(q_steps);
    if quadrant >= 2 { -mag } else { mag }
}

/// Deterministic cosine for 16-bit Binary Angle Measurement.
#[inline]
pub fn cos_bam(angle: u16) -> Fp {
    sin_bam(angle.wrapping_add(16384))
}

fn sin_quadrant(q_steps: u32) -> Fp {
    if q_steps == 0 {
        return Fp::ZERO;
    }
    if q_steps >= 16384 {
        return Fp::ONE;
    }
    // x = (q_steps / 16384) * (pi / 2) in Q32.32
    let x_raw = ((q_steps as i128) * (Fp::PI_HALF_RAW as i128)) >> 14;
    let x = Fp(x_raw as i64);
    let x2 = x.mul(x);
    let x3 = x2.mul(x);
    let x5 = x3.mul(x2);
    let x7 = x5.mul(x2);
    let x9 = x7.mul(x2);

    // sin(x) = x - x^3/6 + x^5/120 - x^7/5040 + x^9/362880
    let t3 = x3.div(Fp::from_int(6));
    let t5 = x5.div(Fp::from_int(120));
    let t7 = x7.div(Fp::from_int(5040));
    let t9 = x9.div(Fp::from_int(362_880));

    (x - t3 + t5 - t7 + t9).clamp(Fp::ZERO, Fp::ONE)
}

/// Deterministic 16-step integer CORDIC `atan2(y, x)` returning BAM16 (`0..65536`).
pub fn atan2_bam(y: Fp, x: Fp) -> u16 {
    if x.0 == 0 && y.0 == 0 {
        return 0;
    }
    const CORDIC_ATAN_BAM: [i32; 16] = [
        8192, 4836, 2555, 1297, 651, 326, 163, 81, 41, 20, 10, 5, 3, 1, 1, 0,
    ];

    let mut cx = (x.0 as i128).abs();
    let mut cy = (y.0 as i128).abs();
    let mut angle: i32 = 0;

    for (i, &step_angle) in CORDIC_ATAN_BAM.iter().enumerate() {
        if cy == 0 {
            break;
        }
        let dx = cy >> i;
        let dy = cx >> i;
        if cy >= 0 {
            cx += dx;
            cy -= dy;
            angle += step_angle;
        } else {
            cx -= dx;
            cy += dy;
            angle -= step_angle;
        }
    }

    let q_angle = angle.clamp(0, 16384) as u16;
    match (x.0 >= 0, y.0 >= 0) {
        (true, true) => q_angle,
        (false, true) => 32768_u16.wrapping_sub(q_angle),
        (false, false) => 32768_u16.wrapping_add(q_angle),
        (true, false) => 0_u16.wrapping_sub(q_angle),
    }
}

/// Shortest signed angular delta `target - current` in BAM16 (`-32768..=32767`).
#[inline]
pub const fn angle_diff_bam(current: u16, target: u16) -> i16 {
    target.wrapping_sub(current) as i16
}

/// Step `current` toward `target` by at most `max_step` BAM16 units.
#[inline]
pub const fn turn_toward_bam(current: u16, target: u16, max_step: u16) -> u16 {
    let diff = angle_diff_bam(current, target) as i32;
    let limit = max_step as i32;
    if diff.abs() <= limit {
        target
    } else if diff > 0 {
        current.wrapping_add(max_step)
    } else {
        current.wrapping_sub(max_step)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: Fp,
    pub y: Fp,
}

impl Vec2 {
    pub const ZERO: Self = Self {
        x: Fp::ZERO,
        y: Fp::ZERO,
    };

    #[inline]
    pub const fn new(x: Fp, y: Fp) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn from_milli(x_mm: i64, y_mm: i64) -> Self {
        Self {
            x: Fp::from_milli(x_mm),
            y: Fp::from_milli(y_mm),
        }
    }

    #[inline]
    pub fn from_bam(angle: u16, length: Fp) -> Self {
        Self {
            x: cos_bam(angle).mul(length),
            y: sin_bam(angle).mul(length),
        }
    }

    #[inline]
    pub fn to_bam(self) -> u16 {
        atan2_bam(self.y, self.x)
    }

    #[inline]
    pub const fn scale(self, s: Fp) -> Self {
        Self {
            x: self.x.mul(s),
            y: self.y.mul(s),
        }
    }

    #[inline]
    pub const fn dot(self, rhs: Self) -> Fp {
        let px = (self.x.0 as i128) * (rhs.x.0 as i128);
        let py = (self.y.0 as i128) * (rhs.y.0 as i128);
        Fp(clamp_i128_to_i64((px + py) >> Fp::FRAC_BITS))
    }

    #[inline]
    pub const fn len_sq(self) -> Fp {
        self.dot(self)
    }

    #[inline]
    pub fn len(self) -> Fp {
        // Scale down large vectors (> 512 units) before squaring so len_sq never saturates i64.
        let max_c = self.x.abs().max(self.y.abs());
        if max_c.0 == 0 {
            return Fp::ZERO;
        }
        if max_c.0 > Fp::from_int(512).0 {
            let shifted = Self {
                x: Fp(self.x.0 >> 8),
                y: Fp(self.y.0 >> 8),
            };
            return Fp(shifted.len_sq().sqrt().0.saturating_mul(256));
        }
        self.len_sq().sqrt()
    }

    #[inline]
    pub fn dist(self, other: Self) -> Fp {
        (self - other).len()
    }

    #[inline]
    pub fn normalize_or_zero(self) -> Self {
        let l = self.len();
        if l.0 <= 1024 {
            Self::ZERO
        } else {
            Self {
                x: self.x.div(l),
                y: self.y.div(l),
            }
        }
    }

    #[inline]
    pub fn clamp_len(self, max_len: Fp) -> Self {
        let l = self.len();
        if l.0 <= max_len.0 || l.0 == 0 {
            self
        } else {
            let factor = max_len.div(l);
            self.scale(factor)
        }
    }

    #[inline]
    pub const fn lerp(self, target: Self, t: Fp) -> Self {
        Self {
            x: self.x.lerp(target.x, t),
            y: self.y.lerp(target.y, t),
        }
    }
}

impl Add for Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Neg for Vec2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: Fp,
    pub y: Fp,
    pub z: Fp,
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: Fp::ZERO,
        y: Fp::ZERO,
        z: Fp::ZERO,
    };

    #[inline]
    pub const fn new(x: Fp, y: Fp, z: Fp) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn from_milli(x_mm: i64, y_mm: i64, z_mm: i64) -> Self {
        Self {
            x: Fp::from_milli(x_mm),
            y: Fp::from_milli(y_mm),
            z: Fp::from_milli(z_mm),
        }
    }

    #[inline]
    pub const fn xy(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    pub const fn scale(self, s: Fp) -> Self {
        Self {
            x: self.x.mul(s),
            y: self.y.mul(s),
            z: self.z.mul(s),
        }
    }

    #[inline]
    pub fn len(self) -> Fp {
        let max_c = self.x.abs().max(self.y.abs()).max(self.z.abs());
        if max_c.0 == 0 {
            return Fp::ZERO;
        }
        let s = if max_c.0 > Fp::from_int(512).0 { 8 } else { 0 };
        let sx = (self.x.0 >> s) as i128;
        let sy = (self.y.0 >> s) as i128;
        let sz = (self.z.0 >> s) as i128;
        let sum_sq = Fp(clamp_i128_to_i64((sx * sx + sy * sy + sz * sz) >> Fp::FRAC_BITS));
        let root = sum_sq.sqrt();
        if s > 0 {
            Fp(root.0.saturating_mul(256))
        } else {
            root
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_point_arithmetic_and_sqrt_are_accurate() {
        let a = Fp::from_milli(3_000);
        let b = Fp::from_milli(4_000);
        let hyp = Vec2::new(a, b).len();
        assert_eq!(hyp.to_milli(), 5_000);
    }

    #[test]
    fn trig_identities_hold_across_circle() {
        for step in 0..64_u16 {
            let angle = step.wrapping_mul(1024);
            let s = sin_bam(angle);
            let c = cos_bam(angle);
            let norm_sq = s.mul(s) + c.mul(c);
            assert!(
                (norm_sq.to_milli() - 1_000).abs() <= 2,
                "sin^2+cos^2 drift at angle {angle}: {}",
                norm_sq.to_milli()
            );
            let v = Vec2::from_bam(angle, Fp::from_int(10));
            let recovered = v.to_bam();
            let delta = angle_diff_bam(angle, recovered).unsigned_abs();
            assert!(delta <= 64, "atan2 mismatch at {angle}: got {recovered} (delta {delta})");
        }
    }
}
