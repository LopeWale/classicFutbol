//! Structured `MatchEvent` and per-player statistics produced deterministically by `psfc-sim`.

use crate::config::TeamSide;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchEventType {
    KickOff,
    Goal,
    Shot,
    Save,
    Pass,
    Interception,
    Tackle,
    Foul,
    YellowCard,
    RedCard,
    ThrowIn,
    CornerKick,
    GoalKick,
    FreeKick,
    PenaltyKick,
    Offside,
    SkillMove,
    Advantage,
    Substitution,
    Woodwork,
    ExtraTimeStart,
    PenaltyShootoutStart,
    ShootoutGoal,
    ShootoutMiss,
    HalfTime,
    FullTime,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchEvent {
    pub tick: u32,
    pub minute: u16,
    pub event_type: MatchEventType,
    pub team: TeamSide,
    pub footballer_id: Option<String>,
    pub assist_footballer_id: Option<String>,
    pub pos_mm: (i64, i64),
    pub detail: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerMatchStats {
    pub footballer_id: String,
    pub name: String,
    pub team: Option<TeamSide>,
    pub goals: u16,
    pub assists: u16,
    pub shots: u16,
    pub shots_on_target: u16,
    pub passes_attempted: u16,
    pub passes_completed: u16,
    pub tackles_won: u16,
    pub interceptions: u16,
    pub saves: u16,
    pub skill_moves_completed: u16,
    pub fouls_committed: u16,
    pub yellow_cards: u8,
    pub red_cards: u8,
    pub ticks_on_pitch: u32,
}
