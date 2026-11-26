//! Parameterized `MatchFormat` (5v5 default, 11v11 expansion) and `RuleSet` definitions.

use crate::math::Fp;
use serde::{Deserialize, Serialize};

pub const TICKS_PER_SECOND: u32 = 60;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TeamSide {
    Home = 0,
    Away = 1,
}

impl TeamSide {
    #[inline]
    pub const fn index(self) -> usize {
        match self {
            Self::Home => 0,
            Self::Away => 1,
        }
    }

    #[inline]
    pub const fn opponent(self) -> Self {
        match self {
            Self::Home => Self::Away,
            Self::Away => Self::Home,
        }
    }

    /// Returns `+1` if attacking `+X` goal, `-1` if attacking `-X` goal.
    #[inline]
    pub const fn attack_sign(self, second_half_swapped: bool) -> i64 {
        let base = match self {
            Self::Home => 1,
            Self::Away => -1,
        };
        if second_half_swapped { -base } else { base }
    }
}

/// Parameterized pitch geometry and squad count per team (ADR-0003).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchFormat {
    pub name: String,
    /// Total players on the pitch per team, including 1 Goalkeeper (`5` = 4 outfield + 1 GK; `11` = 10 outfield + 1 GK).
    pub players_per_side: usize,
    pub pitch_length_mm: i64,
    pub pitch_width_mm: i64,
    pub goal_width_mm: i64,
    pub goal_height_mm: i64,
    pub penalty_box_depth_mm: i64,
    pub penalty_box_width_mm: i64,
    pub center_circle_radius_mm: i64,
}

impl MatchFormat {
    /// Authentic EA SPORTS FC 25/26 *5v5 Rush* pitch & regulation goal dimensions:
    /// 4 outfield players + 1 goalkeeper per side on a 63.7m x 46.6m pitch (`41.5%` of 11v11 area,
    /// `1.37:1` lateral passing ratio) with full regulation 7.32m x 2.44m goals.
    pub fn five_a_side() -> Self {
        Self {
            name: "5v5".to_string(),
            players_per_side: 5,
            pitch_length_mm: 63_700,
            pitch_width_mm: 46_600,
            goal_width_mm: 7_320,
            goal_height_mm: 2_440,
            penalty_box_depth_mm: 11_500,
            penalty_box_width_mm: 26_000,
            center_circle_radius_mm: 6_500,
        }
    }

    /// Full regulation 11v11 (10 outfield players + 1 goalkeeper per side) on a 105m x 68m pitch.
    pub fn eleven_a_side() -> Self {
        Self {
            name: "11v11".to_string(),
            players_per_side: 11,
            pitch_length_mm: 105_000,
            pitch_width_mm: 68_000,
            goal_width_mm: 7_320,
            goal_height_mm: 2_440,
            penalty_box_depth_mm: 16_500,
            penalty_box_width_mm: 40_320,
            center_circle_radius_mm: 9_150,
        }
    }

    #[inline]
    pub fn half_length(&self) -> Fp {
        Fp::from_milli(self.pitch_length_mm / 2)
    }

    #[inline]
    pub fn half_width(&self) -> Fp {
        Fp::from_milli(self.pitch_width_mm / 2)
    }

    #[inline]
    pub fn goal_half_width(&self) -> Fp {
        Fp::from_milli(self.goal_width_mm / 2)
    }

    #[inline]
    pub fn goal_height(&self) -> Fp {
        Fp::from_milli(self.goal_height_mm)
    }

    #[inline]
    pub fn penalty_box_depth(&self) -> Fp {
        Fp::from_milli(self.penalty_box_depth_mm)
    }

    #[inline]
    pub fn penalty_box_half_width(&self) -> Fp {
        Fp::from_milli(self.penalty_box_width_mm / 2)
    }

    #[inline]
    pub fn center_circle_radius(&self) -> Fp {
        Fp::from_milli(self.center_circle_radius_mm)
    }

    pub fn validate(&self) -> Result<(), String> {
        if !(2..=11).contains(&self.players_per_side) {
            return Err(format!(
                "players_per_side must be in 2..=11, got {}",
                self.players_per_side
            ));
        }
        if self.pitch_length_mm < 20_000 || self.pitch_width_mm < 15_000 {
            return Err("pitch dimensions below minimum playable bounds".to_string());
        }
        if self.goal_width_mm < 2_000 || self.goal_width_mm >= self.pitch_width_mm {
            return Err("invalid goal_width_mm".to_string());
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CpuDifficulty {
    Easy,
    Normal,
    Hard,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "difficulty")]
pub enum SideController {
    Human,
    Cpu(CpuDifficulty),
}

impl SideController {
    #[inline]
    pub const fn is_cpu(self) -> bool {
        matches!(self, Self::Cpu(_))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleSet {
    /// Duration of one half in 60 Hz ticks (`7200` = 2 min half / 4 min match).
    pub half_duration_ticks: u32,
    /// Enabled by default for both 5v5 Rush (Attacking-Third Offside at `±1/3` pitch length) and 11v11 (halfway line).
    pub offsides_enabled: bool,
    pub fouls_enabled: bool,
    #[serde(default)]
    pub extra_time_enabled: bool,
    pub home_controller: SideController,
    pub away_controller: SideController,
}

impl RuleSet {
    pub fn default_five_a_side() -> Self {
        Self {
            half_duration_ticks: 2 * 60 * TICKS_PER_SECOND,
            offsides_enabled: true,
            fouls_enabled: true,
            extra_time_enabled: false,
            home_controller: SideController::Human,
            away_controller: SideController::Cpu(CpuDifficulty::Normal),
        }
    }

    pub fn default_eleven_a_side() -> Self {
        Self {
            half_duration_ticks: 4 * 60 * TICKS_PER_SECOND,
            offsides_enabled: true,
            fouls_enabled: true,
            extra_time_enabled: false,
            home_controller: SideController::Human,
            away_controller: SideController::Cpu(CpuDifficulty::Normal),
        }
    }
}

/// Returns `(x_permille, y_permille)` tactical base anchors in `[-1000, 1000]`
/// assuming the team defends `-X` (`x = -1000`) and attacks `+X` (`x = +1000`).
/// Index `0` is always the Goalkeeper.
pub fn formation_anchors(formation: &str, players_per_side: usize) -> Vec<(i64, i64)> {
    match (players_per_side, formation) {
        (5, "2-2") => vec![
            (-930, 0),    // GK
            (-480, -450), // LD
            (-480, 450),  // RD
            (240, -380),  // LF
            (240, 380),   // RF
        ],
        (5, "1-1-2") => vec![
            (-930, 0),   // GK
            (-550, 0),   // CB
            (-120, 0),   // CM
            (320, -420), // LF
            (320, 420),  // RF
        ],
        (5, _) => vec![
            // Default "1-2-1" Diamond (GK + Sweeper/Pivot + 2 Wide Mid/Wingbacks + Striker)
            (-930, 0),    // GK
            (-540, 0),    // CB / Anchor
            (-120, -520), // LM
            (-120, 520),  // RM
            (360, 0),     // ST
        ],
        (11, "4-4-2") => vec![
            (-950, 0),    // GK
            (-580, -680), // LB
            (-620, -240), // LCB
            (-620, 240),  // RCB
            (-580, 680),  // RB
            (-120, -680), // LM
            (-180, -220), // LCM
            (-180, 220),  // RCM
            (-120, 680),  // RM
            (360, -260),  // LS
            (360, 260),   // RS
        ],
        (11, _) => vec![
            // Default "4-3-3"
            (-950, 0),    // GK
            (-560, -680), // LB
            (-620, -240), // LCB
            (-620, 240),  // RCB
            (-560, 680),  // RB
            (-240, 0),    // CDM
            (-80, -380),  // LCM
            (-80, 380),   // RCM
            (340, -620),  // LW
            (400, 0),     // ST
            (340, 620),   // RW
        ],
        (n, _) => {
            let mut out = Vec::with_capacity(n);
            out.push((-930, 0)); // GK
            let outfield = n.saturating_sub(1);
            let def_count = (outfield + 1) / 3;
            let fwd_count = (outfield / 3).max(1);
            let mid_count = outfield.saturating_sub(def_count + fwd_count);

            for (row_x, count) in [(-520_i64, def_count), (-100_i64, mid_count), (340_i64, fwd_count)] {
                if count == 0 {
                    continue;
                }
                for idx in 0..count {
                    if out.len() >= n {
                        break;
                    }
                    let y = if count == 1 {
                        0
                    } else {
                        -560 + (1120 * (idx as i64)) / ((count - 1) as i64)
                    };
                    out.push((row_x, y));
                }
            }
            while out.len() < n {
                out.push((0, 0));
            }
            out
        }
    }
}
