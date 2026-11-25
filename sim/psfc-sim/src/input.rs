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
pub const BTN_SKILL: u16 = 1 << 10;
/// Attack: Finesse Shot / Driven Pass / Whipped Cross Modifier (`RB / R1`).
pub const BTN_FINESSE: u16 = 1 << 11;
/// Defense: Contain / Pressure Hold (`A / Cross`) — shadow in Tactical mode, auto-close + tackle in Legacy mode.
pub const BTN_CONTAIN: u16 = 1 << 12;
/// Defense: Teammate Contain / Second-Man Press Hold (`RB / R1`).
pub const BTN_TEAMMATE_CONTAIN: u16 = 1 << 13;
/// Defense: Rush Goalkeeper Hold (`Y / Triangle`).
pub const BTN_GK_RUSH: u16 = 1 << 14;
/// Attack: Pace Control / Close Shield / Skill Move Modifier (`LT / L2`).
pub const BTN_PACE_CONTROL: u16 = 1 << 15;

pub const RESERVED_SUB_FLAG: u16 = 0x8000;
pub const RESERVED_SETTINGS_FLAG: u16 = 0x4000;

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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AutoSwitchMode {
    /// FIFA-style ball-following auto-switch on defense, loose balls, rebounds, and air balls (default).
    #[default]
    AssistedBall = 0,
    /// Auto-switch only on loose balls and aerial balls (`z > 0.85m`).
    AirAndLoose = 1,
    /// Full manual switching (`BTN_SWITCH`, Right Stick flick, or pass receiver only).
    Manual = 2,
}

impl AutoSwitchMode {
    #[inline]
    pub const fn from_bits(bits: u16) -> Self {
        match bits & 0x3 {
            1 => Self::AirAndLoose,
            2 => Self::Manual,
            _ => Self::AssistedBall,
        }
    }
}

/// FIFA-style directional & power assistance level per action category.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssistLevel {
    /// Snap direction to best target and automatically scale power to target distance.
    #[default]
    Assisted = 0,
    /// Blend aim direction toward nearby targets while keeping power manual.
    Semi = 1,
    /// 100% raw stick direction and 100% manual power bar control.
    Manual = 2,
}

impl AssistLevel {
    #[inline]
    pub const fn from_bits(bits: u16) -> Self {
        match bits & 0x3 {
            1 => Self::Semi,
            2 => Self::Manual,
            _ => Self::Assisted,
        }
    }
}

/// FIFA 12 defending scheme toggle: Tactical (Contain shadows + manual timed tackle) vs Legacy (Contain presses + auto-tackle).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefendingMode {
    #[default]
    Tactical = 0,
    Legacy = 1,
}

impl DefendingMode {
    #[inline]
    pub const fn from_bit(bit: u16) -> Self {
        if (bit & 1) != 0 {
            Self::Legacy
        } else {
            Self::Tactical
        }
    }
}

/// Brief momentum-preservation window right after switching to a moving player.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AutoSwitchMoveAssist {
    #[default]
    None = 0,
    Low = 1,
    High = 2,
}

impl AutoSwitchMoveAssist {
    #[inline]
    pub const fn from_bits(bits: u16) -> Self {
        match bits & 0x3 {
            1 => Self::Low,
            2 => Self::High,
            _ => Self::None,
        }
    }

    #[inline]
    pub const fn duration_ticks(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Low => 18,  // ~300ms smooth momentum carry
            Self::High => 42, // ~700ms strong momentum carry
        }
    }
}

/// Per-side deterministic gameplay & assistance settings synced via 8-byte `InputFrame.reserved` (`0x4000`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ControlSettings {
    pub pass_assist: AssistLevel,
    pub through_assist: AssistLevel,
    pub shot_assist: AssistLevel,
    pub cross_assist: AssistLevel,
    pub defending_mode: DefendingMode,
    pub auto_switch_mode: AutoSwitchMode,
    pub auto_switch_move_assist: AutoSwitchMoveAssist,
    pub analog_sprint: bool,
}

impl Default for ControlSettings {
    fn default() -> Self {
        Self {
            pass_assist: AssistLevel::Assisted,
            through_assist: AssistLevel::Assisted,
            shot_assist: AssistLevel::Assisted,
            cross_assist: AssistLevel::Assisted,
            defending_mode: DefendingMode::Tactical,
            auto_switch_mode: AutoSwitchMode::AssistedBall,
            auto_switch_move_assist: AutoSwitchMoveAssist::None,
            analog_sprint: true,
        }
    }
}

impl ControlSettings {
    /// Packs all 14 settings bits into a deterministic `InputFrame.reserved` word tagged with `0x4000`.
    #[inline]
    pub const fn encode_reserved(self) -> u16 {
        RESERVED_SETTINGS_FLAG
            | ((self.pass_assist as u16) & 0x3)
            | (((self.through_assist as u16) & 0x3) << 2)
            | (((self.shot_assist as u16) & 0x3) << 4)
            | (((self.cross_assist as u16) & 0x3) << 6)
            | (((self.defending_mode as u16) & 0x1) << 8)
            | (((self.auto_switch_move_assist as u16) & 0x3) << 9)
            | ((if self.analog_sprint { 1 } else { 0 }) << 11)
            | (((self.auto_switch_mode as u16) & 0x3) << 12)
    }

    /// Decodes a `0x4000`-tagged settings word (`bit 15 == 0 && bit 14 == 1`).
    #[inline]
    pub const fn decode_reserved(reserved: u16) -> Option<Self> {
        if (reserved & (RESERVED_SUB_FLAG | RESERVED_SETTINGS_FLAG)) != RESERVED_SETTINGS_FLAG {
            return None;
        }
        Some(Self {
            pass_assist: AssistLevel::from_bits(reserved),
            through_assist: AssistLevel::from_bits(reserved >> 2),
            shot_assist: AssistLevel::from_bits(reserved >> 4),
            cross_assist: AssistLevel::from_bits(reserved >> 6),
            defending_mode: DefendingMode::from_bit(reserved >> 8),
            auto_switch_move_assist: AutoSwitchMoveAssist::from_bits(reserved >> 9),
            analog_sprint: ((reserved >> 11) & 1) != 0,
            auto_switch_mode: AutoSwitchMode::from_bits(reserved >> 12),
        })
    }
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
    pub const fn encode_substitution(pitch_slot: u8, bench_idx: u8) -> u16 {
        RESERVED_SUB_FLAG | (((pitch_slot as u16) & 0x1F) << 5) | ((bench_idx as u16) & 0x1F)
    }

    #[inline]
    pub const fn decode_substitution(self) -> Option<(usize, usize)> {
        if (self.reserved & RESERVED_SUB_FLAG) == 0 {
            None
        } else {
            let pitch_slot = ((self.reserved >> 5) & 0x1F) as usize;
            let bench_idx = (self.reserved & 0x1F) as usize;
            Some((pitch_slot, bench_idx))
        }
    }

    #[inline]
    pub const fn encode_settings(settings: ControlSettings) -> u16 {
        settings.encode_reserved()
    }

    #[inline]
    pub const fn decode_settings(self) -> Option<ControlSettings> {
        ControlSettings::decode_reserved(self.reserved)
    }

    #[inline]
    pub const fn encode_auto_switch_mode(mode: AutoSwitchMode) -> u16 {
        ((mode as u16) & 0x3) << 12
    }

    /// Encodes normal gameplay `reserved` carrying `AutoSwitchMode` (bits `12..=13`)
    /// and 7-bit analog sprint throttle `0..=127` (bits `0..=6`).
    #[inline]
    pub const fn encode_gameplay_reserved(mode: AutoSwitchMode, sprint_analog: u8) -> u16 {
        let clamped = if sprint_analog > 127 { 127 } else { sprint_analog };
        (((mode as u16) & 0x3) << 12) | ((clamped as u16) & 0x7F)
    }

    #[inline]
    pub const fn auto_switch_mode(self) -> AutoSwitchMode {
        AutoSwitchMode::from_bits(self.reserved >> 12)
    }

    /// Returns raw 7-bit analog sprint level (`0..=127`) on normal gameplay ticks (`bits 14..15 == 0`),
    /// or `0` on special substitution / settings ticks (interpreted as full digital sprint if `BTN_SPRINT` is set).
    #[inline]
    pub const fn sprint_analog_raw(self) -> u8 {
        if (self.reserved & (RESERVED_SUB_FLAG | RESERVED_SETTINGS_FLAG)) != 0 {
            0
        } else {
            (self.reserved & 0x7F) as u8
        }
    }

    /// Returns sprint throttle in `0..=1000` permille respecting `analog_sprint_enabled`.
    /// When `analog_sprint_enabled` is false (digital mode) or raw analog value is `0` (keyboard/legacy),
    /// any `BTN_SPRINT` press yields full `1000`.
    #[inline]
    pub const fn sprint_throttle_permille(self, analog_sprint_enabled: bool) -> i64 {
        if !self.pressed(BTN_SPRINT) {
            return 0;
        }
        if !analog_sprint_enabled {
            return 1_000;
        }
        let raw = self.sprint_analog_raw();
        if raw == 0 {
            1_000
        } else {
            ((raw as i64) * 1_000) / 127
        }
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

/// Converts raw `[-127, 127]` 360-degree analog stick components into a clamped `Vec2`
/// using a radial circular deadzone (`r < 10 / 127 ≈ 0.08`) so diagonals never snap to 8 axes.
fn stick_to_vec2(ax: i8, ay: i8) -> Vec2 {
    let cx = (ax as i64).clamp(-127, 127);
    let cy = (ay as i64).clamp(-127, 127);
    if cx * cx + cy * cy < 100 {
        return Vec2::ZERO;
    }
    let v = Vec2::new(Fp::from_ratio(cx, 127), Fp::from_ratio(cy, 127));
    v.clamp_len(Fp::ONE)
}
