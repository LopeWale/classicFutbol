//! Frozen `FixtureSnapshot`, `TeamSnapshot`, `PlayerSnapshot`, and sample squads (5v5 & 11v11).

use crate::config::{MatchFormat, RuleSet};
use serde::{Deserialize, Serialize};

pub const GAMEPLAY_VERSION: &str = "0.1.0-5v5";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlayerRole {
    Goalkeeper,
    Defender,
    Midfielder,
    Forward,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerAttributes {
    pub pace: u8,
    pub acceleration: u8,
    pub dribbling: u8,
    pub passing: u8,
    pub shooting: u8,
    pub tackling: u8,
    pub positioning: u8,
    pub stamina: u8,
    pub strength: u8,
    pub goalkeeping: u8,
}

impl PlayerAttributes {
    pub const fn keeper(gk: u8, pos: u8, pass: u8) -> Self {
        Self {
            pace: 56,
            acceleration: 58,
            dribbling: 48,
            passing: pass,
            shooting: 35,
            tackling: 42,
            positioning: pos,
            stamina: 78,
            strength: 76,
            goalkeeping: gk,
        }
    }

    pub const fn defender(pace: u8, tackle: u8, pass: u8, strength: u8) -> Self {
        Self {
            pace,
            acceleration: pace.saturating_sub(2),
            dribbling: 70,
            passing: pass,
            shooting: 60,
            tackling: tackle,
            positioning: tackle,
            stamina: 84,
            strength,
            goalkeeping: 15,
        }
    }

    pub const fn midfielder(pace: u8, drib: u8, pass: u8, shoot: u8, tackle: u8) -> Self {
        Self {
            pace,
            acceleration: pace,
            dribbling: drib,
            passing: pass,
            shooting: shoot,
            tackling: tackle,
            positioning: 82,
            stamina: 88,
            strength: 72,
            goalkeeping: 12,
        }
    }

    pub const fn forward(pace: u8, drib: u8, pass: u8, shoot: u8, strength: u8) -> Self {
        Self {
            pace,
            acceleration: if pace >= 97 { 99 } else { pace + 2 },
            dribbling: drib,
            passing: pass,
            shooting: shoot,
            tackling: 44,
            positioning: 85,
            stamina: 82,
            strength,
            goalkeeping: 10,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerSnapshot {
    pub footballer_id: String,
    pub name: String,
    pub shirt_number: u8,
    pub role: PlayerRole,
    pub attributes: PlayerAttributes,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamSnapshot {
    pub club_id: String,
    pub club_name: String,
    pub short_name: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub formation: String,
    pub starters: Vec<PlayerSnapshot>,
    #[serde(default)]
    pub substitutes: Vec<PlayerSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixtureSnapshot {
    pub fixture_id: String,
    pub gameplay_version: String,
    pub seed: u64,
    pub format: MatchFormat,
    pub rules: RuleSet,
    pub home: TeamSnapshot,
    pub away: TeamSnapshot,
}

impl FixtureSnapshot {
    pub fn validate(&self) -> Result<(), String> {
        self.format.validate()?;
        let expected = self.format.players_per_side;
        if self.home.starters.len() != expected {
            return Err(format!(
                "home starters count ({}) != format.players_per_side ({expected})",
                self.home.starters.len()
            ));
        }
        if self.away.starters.len() != expected {
            return Err(format!(
                "away starters count ({}) != format.players_per_side ({expected})",
                self.away.starters.len()
            ));
        }
        if self.home.starters[0].role != PlayerRole::Goalkeeper {
            return Err("home starter[0] must have role Goalkeeper".to_string());
        }
        if self.away.starters[0].role != PlayerRole::Goalkeeper {
            return Err("away starter[0] must have role Goalkeeper".to_string());
        }
        Ok(())
    }

    pub fn sample_five_a_side(seed: u64) -> Self {
        Self {
            fixture_id: "fx-sample-5v5".to_string(),
            gameplay_version: GAMEPLAY_VERSION.to_string(),
            seed,
            format: MatchFormat::five_a_side(),
            rules: RuleSet::default_five_a_side(),
            home: sample_manchester_united(5),
            away: sample_arsenal(5),
        }
    }

    pub fn sample_eleven_a_side(seed: u64) -> Self {
        Self {
            fixture_id: "fx-sample-11v11".to_string(),
            gameplay_version: GAMEPLAY_VERSION.to_string(),
            seed,
            format: MatchFormat::eleven_a_side(),
            rules: RuleSet::default_eleven_a_side(),
            home: sample_manchester_united(11),
            away: sample_arsenal(11),
        }
    }
}

pub fn sample_manchester_united(players_per_side: usize) -> TeamSnapshot {
    let full = vec![
        PlayerSnapshot {
            footballer_id: "mun-gk-1".to_string(),
            name: "S. Lammens".to_string(),
            shirt_number: 1,
            role: PlayerRole::Goalkeeper,
            attributes: PlayerAttributes::keeper(82, 81, 80),
        },
        PlayerSnapshot {
            footballer_id: "mun-def-1".to_string(),
            name: "L. Martinez".to_string(),
            shirt_number: 6,
            role: PlayerRole::Defender,
            attributes: PlayerAttributes::defender(77, 86, 83, 82),
        },
        PlayerSnapshot {
            footballer_id: "mun-mid-1".to_string(),
            name: "K. Mainoo".to_string(),
            shirt_number: 37,
            role: PlayerRole::Midfielder,
            attributes: PlayerAttributes::midfielder(80, 86, 84, 75, 78),
        },
        PlayerSnapshot {
            footballer_id: "mun-mid-2".to_string(),
            name: "B. Fernandes".to_string(),
            shirt_number: 8,
            role: PlayerRole::Midfielder,
            attributes: PlayerAttributes::midfielder(76, 85, 90, 86, 70),
        },
        PlayerSnapshot {
            footballer_id: "mun-fwd-1".to_string(),
            name: "M. Cunha".to_string(),
            shirt_number: 10,
            role: PlayerRole::Forward,
            attributes: PlayerAttributes::forward(86, 87, 82, 84, 79),
        },
        PlayerSnapshot {
            footballer_id: "mun-def-2".to_string(),
            name: "D. Dalot".to_string(),
            shirt_number: 2,
            role: PlayerRole::Defender,
            attributes: PlayerAttributes::defender(85, 80, 79, 77),
        },
        PlayerSnapshot {
            footballer_id: "mun-def-3".to_string(),
            name: "M. de Ligt".to_string(),
            shirt_number: 4,
            role: PlayerRole::Defender,
            attributes: PlayerAttributes::defender(74, 85, 75, 88),
        },
        PlayerSnapshot {
            footballer_id: "mun-def-4".to_string(),
            name: "L. Yoro".to_string(),
            shirt_number: 15,
            role: PlayerRole::Defender,
            attributes: PlayerAttributes::defender(83, 84, 78, 82),
        },
        PlayerSnapshot {
            footballer_id: "mun-mid-3".to_string(),
            name: "Y. Tielemans".to_string(),
            shirt_number: 18,
            role: PlayerRole::Midfielder,
            attributes: PlayerAttributes::midfielder(71, 82, 86, 80, 75),
        },
        PlayerSnapshot {
            footballer_id: "mun-fwd-2".to_string(),
            name: "B. Mbeumo".to_string(),
            shirt_number: 19,
            role: PlayerRole::Forward,
            attributes: PlayerAttributes::forward(89, 85, 80, 85, 77),
        },
        PlayerSnapshot {
            footballer_id: "mun-fwd-3".to_string(),
            name: "B. Sesko".to_string(),
            shirt_number: 30,
            role: PlayerRole::Forward,
            attributes: PlayerAttributes::forward(88, 80, 73, 84, 86),
        },
    ];
    let count = players_per_side.clamp(2, full.len());
    let starters = full[..count].to_vec();
    let substitutes = full[count..].to_vec();
    TeamSnapshot {
        club_id: "man-utd".to_string(),
        club_name: "Manchester United".to_string(),
        short_name: "MUN".to_string(),
        primary_color: "#DA291C".to_string(),
        secondary_color: "#FFFFFF".to_string(),
        formation: if count == 5 { "1-2-1" } else { "4-3-3" }.to_string(),
        starters,
        substitutes,
    }
}

pub fn sample_arsenal(players_per_side: usize) -> TeamSnapshot {
    let full = vec![
        PlayerSnapshot {
            footballer_id: "ars-gk-1".to_string(),
            name: "D. Raya".to_string(),
            shirt_number: 1,
            role: PlayerRole::Goalkeeper,
            attributes: PlayerAttributes::keeper(88, 87, 85),
        },
        PlayerSnapshot {
            footballer_id: "ars-def-1".to_string(),
            name: "W. Saliba".to_string(),
            shirt_number: 2,
            role: PlayerRole::Defender,
            attributes: PlayerAttributes::defender(84, 90, 82, 87),
        },
        PlayerSnapshot {
            footballer_id: "ars-mid-1".to_string(),
            name: "D. Rice".to_string(),
            shirt_number: 41,
            role: PlayerRole::Midfielder,
            attributes: PlayerAttributes::midfielder(80, 83, 86, 78, 89),
        },
        PlayerSnapshot {
            footballer_id: "ars-mid-2".to_string(),
            name: "M. Odegaard".to_string(),
            shirt_number: 8,
            role: PlayerRole::Midfielder,
            attributes: PlayerAttributes::midfielder(79, 90, 91, 84, 68),
        },
        PlayerSnapshot {
            footballer_id: "ars-fwd-1".to_string(),
            name: "B. Saka".to_string(),
            shirt_number: 7,
            role: PlayerRole::Forward,
            attributes: PlayerAttributes::forward(89, 90, 86, 87, 76),
        },
        PlayerSnapshot {
            footballer_id: "ars-def-2".to_string(),
            name: "Gabriel".to_string(),
            shirt_number: 6,
            role: PlayerRole::Defender,
            attributes: PlayerAttributes::defender(78, 88, 76, 89),
        },
        PlayerSnapshot {
            footballer_id: "ars-def-3".to_string(),
            name: "R. Calafiori".to_string(),
            shirt_number: 33,
            role: PlayerRole::Defender,
            attributes: PlayerAttributes::defender(81, 85, 83, 83),
        },
        PlayerSnapshot {
            footballer_id: "ars-def-4".to_string(),
            name: "J. Timber".to_string(),
            shirt_number: 12,
            role: PlayerRole::Defender,
            attributes: PlayerAttributes::defender(84, 84, 82, 80),
        },
        PlayerSnapshot {
            footballer_id: "ars-mid-3".to_string(),
            name: "B. Guimaraes".to_string(),
            shirt_number: 39,
            role: PlayerRole::Midfielder,
            attributes: PlayerAttributes::midfielder(77, 85, 87, 79, 84),
        },
        PlayerSnapshot {
            footballer_id: "ars-fwd-2".to_string(),
            name: "E. Eze".to_string(),
            shirt_number: 10,
            role: PlayerRole::Forward,
            attributes: PlayerAttributes::forward(85, 90, 84, 83, 74),
        },
        PlayerSnapshot {
            footballer_id: "ars-fwd-3".to_string(),
            name: "V. Gyokeres".to_string(),
            shirt_number: 14,
            role: PlayerRole::Forward,
            attributes: PlayerAttributes::forward(87, 83, 77, 88, 88),
        },
    ];
    let count = players_per_side.clamp(2, full.len());
    let starters = full[..count].to_vec();
    let substitutes = full[count..].to_vec();
    TeamSnapshot {
        club_id: "arsenal".to_string(),
        club_name: "Arsenal".to_string(),
        short_name: "ARS".to_string(),
        primary_color: "#EF0107".to_string(),
        secondary_color: "#FFFFFF".to_string(),
        formation: if count == 5 { "1-2-1" } else { "4-3-3" }.to_string(),
        starters,
        substitutes,
    }
}
