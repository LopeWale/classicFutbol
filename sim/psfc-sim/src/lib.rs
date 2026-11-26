#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic, clippy::float_cmp)]

//! `psfc-sim`: Pure deterministic fixed-point football simulation core for Parallel Season FC.
//!
//! Default Phase 1 match format is **5v5** (`4` outfield players + `1` goalkeeper per side) on a
//! `56m × 36m` outdoor pitch, parameterized via [`config::MatchFormat`] to expand cleanly to `11v11`.

pub mod ai;
pub mod config;
pub mod events;
pub mod input;
pub mod math;
pub mod rng;
pub mod sim;
pub mod snapshot;
pub mod state;

pub use config::{CpuDifficulty, MatchFormat, RuleSet, SideController, TeamSide, TICKS_PER_SECOND};
pub use events::{MatchEvent, MatchEventType, PlayerMatchStats};
pub use input::{
    AssistLevel, AutoSwitchMode, AutoSwitchMoveAssist, ControlSettings, DefendingMode, InputFrame,
    BTN_CONTAIN, BTN_FINESSE, BTN_GK_RUSH, BTN_PACE_CONTROL, BTN_SKILL, BTN_TEAMMATE_CONTAIN,
};
pub use math::{Fp, Vec2, Vec3};
pub use sim::{MatchSimulation, SimError};
pub use snapshot::{
    sample_arsenal, sample_manchester_united, FixtureSnapshot, PlayerAttributes, PlayerRole,
    PlayerSnapshot, TeamSnapshot, GAMEPLAY_VERSION,
};
pub use state::{
    BallState, GkScenario, GoalkeeperBrain, MatchPhase, MatchState, PlayerActionState, PlayerState,
    PossessionContext, RestartKind,
};
