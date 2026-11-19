//! 8-byte per-tick controller `InputFrame` shared across client, rollback netcode, and replay logs.

use crate::math::{Fp, Vec2};
use serde::{Deserialize, Serialize};

pub const BTN_PASS: u16 = 1 << 0;
pub const BTN_THROUGH: u16 = 1 << 1;
pub const BTN_LOB: u16 = 1 << 2;
pub const BTN_SHOOT: u16 = 1 << 3;
pub const BTN_SPRINT: u16 = 1 << 4;
pub const BTN_SWITCH: u16 = 1 << 5;
pub const BTN_JOCKEY: u16 = 1 << 6;
pub const BTN_TACKLE: u16 = 1 << 7;
pub const BTN_SLIDE: u16 = 1 << 8;
pub const BTN_CALL_RUN: u16 = 1 << 9;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InputFrame {
    pub move_x: i8,
    pub move_y: i8,
    pub aim_x: i8,
    pub aim_y: i8,
    pub buttons: u16,
    #[serde(default)]
    pub reserved: u16,
}

impl InputFrame {
    pub const IDLE: Self = Self {
        move_x: 0,
        move_y: 0,
        aim_x: 0,
        aim_y: 0,
        buttons: 0,
        reserved: 0,
    };

    #[inline]
    pub const fn pressed(self, mask: u16) -> bool {
        (self.buttons & mask) != 0
    }

    #[inline]
    pub fn move_vec(self) -> Vec2 {
        stick_to_vec2(self.move_x, self.move_y)
    }

    #[inline]
    pub fn aim_vec(self) -> Vec2 {
        stick_to_vec2(self.aim_x, self.aim_y)
    }

    #[inline]
    pub const fn to_bytes(self) -> [u8; 8] {
        let btns = self.buttons.to_le_bytes();
        let res = self.reserved.to_le_bytes();
        [
            self.move_x as u8,
            self.move_y as u8,
            self.aim_x as u8,
            self.aim_y as u8,
            btns[0],
            btns[1],
            res[0],
            res[1],
        ]
    }

    #[inline]
    pub const fn from_bytes(bytes: [u8; 8]) -> Self {
        Self {
            move_x: bytes[0] as i8,
            move_y: bytes[1] as i8,
            aim_x: bytes[2] as i8,
            aim_y: bytes[3] as i8,
            buttons: u16::from_le_bytes([bytes[4], bytes[5]]),
            reserved: u16::from_le_bytes([bytes[6], bytes[7]]),
        }
    }
}

fn stick_to_vec2(ax: i8, ay: i8) -> Vec2 {
    let cx = (ax as i64).clamp(-127, 127);
    let cy = (ay as i64).clamp(-127, 127);
    if cx.abs() < 10 && cy.abs() < 10 {
        return Vec2::ZERO;
    }
    let v = Vec2::new(Fp::from_ratio(cx, 127), Fp::from_ratio(cy, 127));
    v.clamp_len(Fp::ONE)
}
