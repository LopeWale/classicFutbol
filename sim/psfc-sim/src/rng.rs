//! Seeded deterministic `xoshiro256**` PRNG for reproducible simulation & CPU AI.

use crate::math::Fp;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicRng {
    state: [u64; 4],
}

impl DeterministicRng {
    pub fn new(seed: u64) -> Self {
        let mut sm = seed ^ 0x9E37_79B9_7F4A_7C15;
        let mut state = [0_u64; 4];
        for slot in &mut state {
            sm = sm.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = sm;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            *slot = z ^ (z >> 31);
        }
        if state == [0; 4] {
            state[0] = 1;
        }
        Self { state }
    }

    #[inline]
    pub fn state_words(&self) -> [u64; 4] {
        self.state
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        let result = self.state[1]
            .wrapping_mul(5)
            .rotate_left(7)
            .wrapping_mul(9);
        let t = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(45);

        result
    }

    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// Inclusive integer range `[lo, hi]`.
    pub fn range_i32(&mut self, lo: i32, hi: i32) -> i32 {
        if hi <= lo {
            return lo;
        }
        let span = ((hi as i64) - (lo as i64) + 1) as u64;
        let offset = (self.next_u64() % span) as i64;
        ((lo as i64) + offset) as i32
    }

    /// Inclusive millimeter range `[lo_mm, hi_mm]` as `Fp`.
    pub fn range_milli(&mut self, lo_mm: i64, hi_mm: i64) -> Fp {
        if hi_mm <= lo_mm {
            return Fp::from_milli(lo_mm);
        }
        let span = (hi_mm - lo_mm + 1) as u64;
        let offset = (self.next_u64() % span) as i64;
        Fp::from_milli(lo_mm + offset)
    }

    /// Returns `true` with `pct` percent probability (`0..=100`).
    #[inline]
    pub fn chance_percent(&mut self, pct: u32) -> bool {
        if pct == 0 {
            return false;
        }
        if pct >= 100 {
            return true;
        }
        (self.next_u32() % 100) < pct
    }
}
