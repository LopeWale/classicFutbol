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

/// Tactical role of a formation slot (derived from the formation string, not from the player in it).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SlotRole {
    CentreBack,
    FullBack,
    /// Defensive midfielder screening the zone in front of the centre-backs.
    Holding,
    CentralMid,
    WideMid,
    Winger,
    Striker,
    /// 5v5 last man / pivot that always stays behind the ball.
    Fixo,
    /// 5v5 wide midfielder ("ala").
    Ala,
    /// 5v5 target striker ("pivo").
    Pivo,
}

impl SlotRole {
    /// Matching `PlayerRole` band (`1` Defender, `2` Midfielder, `3` Forward).
    #[inline]
    pub const fn band(self) -> i64 {
        match self {
            Self::CentreBack | Self::FullBack | Self::Fixo => 1,
            Self::Holding | Self::CentralMid | Self::WideMid | Self::Ala => 2,
            Self::Winger | Self::Striker | Self::Pivo => 3,
        }
    }

    #[inline]
    pub const fn is_back_line(self) -> bool {
        matches!(self, Self::CentreBack | Self::FullBack | Self::Fixo)
    }
}

/// One outfield formation slot with distinct ATTACKING and DEFENDING positions.
///
/// `depth` is measured inside the team block (`0` = back line, `1000` = front line) and `y` is
/// permille of the pitch half-width (negative = left when attacking `+X`), before the phase's
/// width scale and ball-side shift are applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShapeSlot {
    pub role: SlotRole,
    pub attack_depth: i64,
    pub attack_y: i64,
    pub defend_depth: i64,
    pub defend_y: i64,
}

/// Parses a formation string such as `"4-3-3"`, `"4-2-3-1"`, `"1-2-1"` into row counts.
/// Returns `None` if it does not describe exactly `outfield` players.
fn parse_formation_rows(formation: &str, outfield: usize) -> Option<Vec<usize>> {
    let mut rows = Vec::new();
    for part in formation.split('-') {
        let n: usize = part.trim().parse().ok()?;
        if n == 0 || n > 6 {
            return None;
        }
        rows.push(n);
    }
    if rows.len() < 2 || rows.iter().sum::<usize>() != outfield {
        return None;
    }
    Some(rows)
}

fn spread_y(count: usize, half_spread: i64) -> Vec<i64> {
    if count == 1 {
        return vec![0];
    }
    (0..count)
        .map(|i| -half_spread + (2 * half_spread * i as i64) / ((count - 1) as i64))
        .collect()
}

/// Role-specific transform from a base `(depth, y)` into distinct attacking & defending positions.
/// - Centre-backs hold the line in both phases and split wider to build up.
/// - Full-backs tuck in when defending and overlap high & wide when attacking.
/// - The holding midfielder screens in front of the centre-backs and stays behind the ball.
/// - Wide midfielders keep their width in both phases; wingers drop to the midfield line when defending.
/// - Strikers lead the line (front of block) in both phases.
fn shape_slot(role: SlotRole, base_depth: i64, base_y: i64) -> ShapeSlot {
    let (ad, ay, dd, dy) = match role {
        SlotRole::CentreBack => (0, base_y * 125 / 100, 0, base_y),
        SlotRole::FullBack => (540, base_y * 112 / 100, 30, base_y * 70 / 100),
        SlotRole::Holding => (190, base_y / 2, 280, base_y / 2),
        SlotRole::CentralMid => (base_depth.max(480) + 60, base_y * 105 / 100, base_depth.max(400), base_y * 85 / 100),
        SlotRole::WideMid => (720, base_y * 112 / 100, 420, base_y * 96 / 100),
        SlotRole::Winger => (900, base_y * 108 / 100, 640, base_y * 86 / 100),
        SlotRole::Striker => (1000, base_y, 1000, base_y * 80 / 100),
        SlotRole::Fixo => (0, base_y, 0, base_y),
        SlotRole::Ala => (660, base_y * 115 / 100, 430, base_y * 90 / 100),
        SlotRole::Pivo => (1000, base_y, 1000, base_y),
    };
    ShapeSlot {
        role,
        attack_depth: ad,
        attack_y: ay.clamp(-960, 960),
        defend_depth: dd,
        defend_y: dy.clamp(-960, 960),
    }
}

/// Phase-aware outfield shape for `formation` (index `0` = first outfielder, i.e. starter `1`).
///
/// Any `"a-b-c[-d]"` formation string whose rows sum to the outfield count is supported; unknown
/// strings fall back to the same defence/midfield/attack split as [`formation_anchors`].
pub fn formation_shape(formation: &str, players_per_side: usize) -> Vec<ShapeSlot> {
    let outfield = players_per_side.saturating_sub(1);
    if outfield == 0 {
        return Vec::new();
    }
    let rows = parse_formation_rows(formation, outfield).unwrap_or_else(|| {
        if outfield == 4 {
            // Default 5v5 diamond "1-2-1".
            vec![1, 2, 1]
        } else if outfield == 10 {
            vec![4, 3, 3]
        } else {
            let def = outfield.div_ceil(3).max(1);
            let fwd = (outfield / 3).max(1);
            let mid = outfield.saturating_sub(def + fwd);
            [def, mid, fwd].into_iter().filter(|&c| c > 0).collect()
        }
    });
    let small = outfield <= 5;
    let n_rows = rows.len();
    let mut out = Vec::with_capacity(outfield);
    for (r, &count) in rows.iter().enumerate() {
        let is_back = r == 0;
        let is_front = r + 1 == n_rows;
        let base_depth = if n_rows <= 1 { 0 } else { (r as i64 * 1000) / ((n_rows - 1) as i64) };
        let roles_y: Vec<(SlotRole, i64)> = if small {
            if is_back {
                match count {
                    1 => vec![(SlotRole::Fixo, 0)],
                    _ => spread_y(count, 420).into_iter().map(|y| (SlotRole::CentreBack, y)).collect(),
                }
            } else if is_front {
                match count {
                    1 => vec![(SlotRole::Pivo, 0)],
                    _ => spread_y(count, 520).into_iter().map(|y| (SlotRole::Winger, y)).collect(),
                }
            } else {
                match count {
                    1 => vec![(SlotRole::CentralMid, 0)],
                    _ => spread_y(count, 600).into_iter().map(|y| (SlotRole::Ala, y)).collect(),
                }
            }
        } else if is_back {
            match count {
                4 => vec![
                    (SlotRole::FullBack, -820),
                    (SlotRole::CentreBack, -270),
                    (SlotRole::CentreBack, 270),
                    (SlotRole::FullBack, 820),
                ],
                5 => vec![
                    (SlotRole::FullBack, -860),
                    (SlotRole::CentreBack, -420),
                    (SlotRole::CentreBack, 0),
                    (SlotRole::CentreBack, 420),
                    (SlotRole::FullBack, 860),
                ],
                c => spread_y(c, 440).into_iter().map(|y| (SlotRole::CentreBack, y)).collect(),
            }
        } else if is_front {
            match count {
                1 => vec![(SlotRole::Striker, 0)],
                2 => vec![(SlotRole::Striker, -260), (SlotRole::Striker, 260)],
                3 => vec![(SlotRole::Winger, -720), (SlotRole::Striker, 0), (SlotRole::Winger, 720)],
                c => spread_y(c, 720).into_iter().map(|y| (SlotRole::Striker, y)).collect(),
            }
        } else {
            let first_mid = r == 1;
            let more_mid_after = r + 2 < n_rows;
            match count {
                1 if first_mid => vec![(SlotRole::Holding, 0)],
                1 => vec![(SlotRole::CentralMid, 0)],
                2 if first_mid && more_mid_after => {
                    vec![(SlotRole::Holding, -300), (SlotRole::Holding, 300)]
                }
                2 => vec![(SlotRole::CentralMid, -300), (SlotRole::CentralMid, 300)],
                3 if first_mid && !more_mid_after => vec![
                    (SlotRole::CentralMid, -420),
                    (SlotRole::Holding, 0),
                    (SlotRole::CentralMid, 420),
                ],
                3 => vec![(SlotRole::Winger, -700), (SlotRole::CentralMid, 0), (SlotRole::Winger, 700)],
                4 => vec![
                    (SlotRole::WideMid, -800),
                    (SlotRole::CentralMid, -250),
                    (SlotRole::CentralMid, 250),
                    (SlotRole::WideMid, 800),
                ],
                c => {
                    let ys = spread_y(c, 840);
                    ys.into_iter()
                        .enumerate()
                        .map(|(i, y)| {
                            let role = if i == 0 || i + 1 == c {
                                SlotRole::WideMid
                            } else if i == c / 2 {
                                SlotRole::Holding
                            } else {
                                SlotRole::CentralMid
                            };
                            (role, y)
                        })
                        .collect()
                }
            }
        };
        for (role, y) in roles_y {
            out.push(shape_slot(role, base_depth, y));
        }
    }
    out.truncate(outfield);
    while out.len() < outfield {
        out.push(shape_slot(SlotRole::CentralMid, 500, 0));
    }
    out
}

/// [`formation_anchors`] mapped onto starters by ROLE instead of list order, so a squad listed as
/// `GK, DEF, MID, MID, FWD, DEF, ...` does not put its forward at right-back. Slots of each role band
/// (defence / midfield / attack, taken from [`formation_shape`]) go to players of the same band in
/// starter order; any surplus players fill the remaining slots in order. `roles[0]` is the keeper.
pub fn role_aware_formation_anchors(
    formation: &str,
    players_per_side: usize,
    roles: &[crate::snapshot::PlayerRole],
) -> Vec<(i64, i64)> {
    use crate::snapshot::PlayerRole;
    let anchors = formation_anchors(formation, players_per_side);
    let shape = formation_shape(formation, players_per_side);
    let n = roles.len().min(anchors.len());
    if n <= 1 || shape.len() + 1 < n {
        return anchors;
    }
    let player_band = |r: PlayerRole| -> i64 {
        match r {
            PlayerRole::Goalkeeper => 0,
            PlayerRole::Defender => 1,
            PlayerRole::Midfielder => 2,
            PlayerRole::Forward => 3,
        }
    };
    let mut out = anchors.clone();
    let mut slot_used = vec![false; n];
    let mut player_done = vec![false; n];
    slot_used[0] = true;
    player_done[0] = true;
    for band in 1..=3 {
        for pi in 1..n {
            if player_done[pi] || player_band(roles[pi]) != band {
                continue;
            }
            if let Some(si) = (1..n).find(|&si| !slot_used[si] && shape[si - 1].role.band() == band) {
                slot_used[si] = true;
                player_done[pi] = true;
                out[pi] = anchors[si];
            }
        }
    }
    for pi in 1..n {
        if !player_done[pi] {
            if let Some(si) = (1..n).find(|&si| !slot_used[si]) {
                slot_used[si] = true;
                out[pi] = anchors[si];
            }
        }
    }
    out
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
