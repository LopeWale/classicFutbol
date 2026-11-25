//! Godot 4 GDExtension bindings (`psfc-godot`) exposing `PsfcMatchSim` to GDScript presentation scenes.

use godot::prelude::*;
use psfc_sim::{
    CpuDifficulty, FixtureSnapshot, InputFrame, MatchSimulation, SideController,
};
use serde::Serialize;

struct PsfcExtension;

#[gdextension]
unsafe impl ExtensionLibrary for PsfcExtension {}

#[derive(Serialize)]
struct GodotSessionLog {
    session_id: String,
    claimed_final_hash_hex: String,
    claimed_score: [u16; 2],
    frames_hex: String,
    tick_count: u32,
}

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct PsfcMatchSim {
    base: Base<RefCounted>,
    sim: Option<MatchSimulation>,
    snapshot: Option<FixtureSnapshot>,
    recorded_frames: Vec<[InputFrame; 2]>,
}

#[godot_api]
impl IRefCounted for PsfcMatchSim {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            base,
            sim: None,
            snapshot: None,
            recorded_frames: Vec::new(),
        }
    }
}

#[godot_api]
impl PsfcMatchSim {
    /// Initialize a 5v5 (`is_eleven_a_side = false`) or 11v11 (`true`) match.
    #[func]
    pub fn setup_preset(
        &mut self,
        is_eleven_a_side: bool,
        seed: i64,
        half_seconds: i32,
        away_is_cpu: bool,
    ) -> bool {
        let mut snap = if is_eleven_a_side {
            FixtureSnapshot::sample_eleven_a_side(seed as u64)
        } else {
            FixtureSnapshot::sample_five_a_side(seed as u64)
        };
        if half_seconds > 5 {
            snap.rules.half_duration_ticks = (half_seconds as u32) * 60;
        }
        snap.rules.home_controller = SideController::Human;
        snap.rules.away_controller = if away_is_cpu {
            SideController::Cpu(CpuDifficulty::Normal)
        } else {
            SideController::Human
        };

        match MatchSimulation::new(&snap) {
            Ok(sim) => {
                self.sim = Some(sim);
                self.snapshot = Some(snap);
                self.recorded_frames.clear();
                true
            }
            Err(err) => {
                godot_error!("PsfcMatchSim::setup_preset failed: {err}");
                false
            }
        }
    }

    /// Initialize from a frozen `FixtureSnapshot` JSON string fetched from `psfc_host`.
    #[func]
    pub fn setup_from_snapshot_json(&mut self, snapshot_json: GString) -> bool {
        let raw = snapshot_json.to_string();
        let snap: FixtureSnapshot = match serde_json::from_str(&raw) {
            Ok(s) => s,
            Err(err) => {
                godot_error!("Invalid FixtureSnapshot JSON: {err}");
                return false;
            }
        };
        match MatchSimulation::new(&snap) {
            Ok(sim) => {
                self.sim = Some(sim);
                self.snapshot = Some(snap);
                self.recorded_frames.clear();
                true
            }
            Err(err) => {
                godot_error!("MatchSimulation init error: {err}");
                false
            }
        }
    }

    #[func]
    pub fn max_rollback_frames(&self) -> i32 {
        psfc_rollback::MAX_ROLLBACK_FRAMES as i32
    }

    /// Step one deterministic 60 Hz tick.
    #[func]
    #[allow(clippy::too_many_arguments)]
    pub fn tick(
        &mut self,
        home_move_x: i32,
        home_move_y: i32,
        home_aim_x: i32,
        home_aim_y: i32,
        home_buttons: i32,
        away_move_x: i32,
        away_move_y: i32,
        away_aim_x: i32,
        away_aim_y: i32,
        away_buttons: i32,
    ) -> i32 {
        let Some(ref mut sim) = self.sim else {
            return 0;
        };
        let home = InputFrame {
            move_x: home_move_x.clamp(-127, 127) as i8,
            move_y: home_move_y.clamp(-127, 127) as i8,
            aim_x: home_aim_x.clamp(-127, 127) as i8,
            aim_y: home_aim_y.clamp(-127, 127) as i8,
            buttons: (home_buttons & 0xFFFF) as u16,
            reserved: (((home_buttons as u32) >> 16) & 0xFFFF) as u16,
        };
        let away = InputFrame {
            move_x: away_move_x.clamp(-127, 127) as i8,
            move_y: away_move_y.clamp(-127, 127) as i8,
            aim_x: away_aim_x.clamp(-127, 127) as i8,
            aim_y: away_aim_y.clamp(-127, 127) as i8,
            buttons: (away_buttons & 0xFFFF) as u16,
            reserved: (((away_buttons as u32) >> 16) & 0xFFFF) as u16,
        };
        if !sim.is_finished() {
            let pair = [home, away];
            sim.tick(&pair);
            self.recorded_frames.push(pair);
        }
        sim.state().tick as i32
    }

    /// Returns Godot 3D coordinates `(x, z_height, y)` in meters for all players (`Home` then `Away`).
    #[func]
    pub fn get_player_positions(&self) -> PackedVector3Array {
        let mut arr = PackedVector3Array::new();
        let Some(ref sim) = self.sim else {
            return arr;
        };
        for side in &sim.state().players {
            for p in side {
                let x = (p.pos.x.to_milli() as f32) / 1000.0;
                let y_up = (p.z.to_milli() as f32) / 1000.0;
                let z = (p.pos.y.to_milli() as f32) / 1000.0;
                arr.push(Vector3::new(x, y_up, z));
            }
        }
        arr
    }

    /// Returns yaw angles in radians for all players (`Home` then `Away`).
    #[func]
    pub fn get_player_facings_rad(&self) -> PackedFloat32Array {
        let mut arr = PackedFloat32Array::new();
        let Some(ref sim) = self.sim else {
            return arr;
        };
        for side in &sim.state().players {
            for p in side {
                let rad = (p.facing_bam as f32) * core::f32::consts::TAU / 65536.0;
                arr.push(rad);
            }
        }
        arr
    }

    #[func]
    pub fn get_ball_position(&self) -> Vector3 {
        let Some(ref sim) = self.sim else {
            return Vector3::ZERO;
        };
        let b = &sim.state().ball.pos;
        Vector3::new(
            (b.x.to_milli() as f32) / 1000.0,
            (b.z.to_milli() as f32) / 1000.0,
            (b.y.to_milli() as f32) / 1000.0,
        )
    }

    #[func]
    pub fn get_pitch_dimensions(&self) -> Vector2 {
        let Some(ref sim) = self.sim else {
            return Vector2::new(56.0, 36.0);
        };
        let f = &sim.state().format;
        Vector2::new(
            (f.pitch_length_mm as f32) / 1000.0,
            (f.pitch_width_mm as f32) / 1000.0,
        )
    }

    #[func]
    pub fn get_score(&self) -> Vector2i {
        let Some(ref sim) = self.sim else {
            return Vector2i::ZERO;
        };
        let s = sim.state().score;
        Vector2i::new(s[0] as i32, s[1] as i32)
    }

    #[func]
    pub fn get_shootout_score(&self) -> Vector2i {
        let Some(ref sim) = self.sim else {
            return Vector2i::new(-1, -1);
        };
        match sim.state().shootout_score {
            Some([hp, ap]) => Vector2i::new(hp as i32, ap as i32),
            None => Vector2i::new(-1, -1),
        }
    }

    #[func]
    pub fn get_controlled_indices(&self) -> Vector2i {
        let Some(ref sim) = self.sim else {
            return Vector2i::new(1, 1);
        };
        let c = sim.state().controlled_idx;
        Vector2i::new(c[0] as i32, c[1] as i32)
    }

    /// Returns true when `side` (`0 = Home`, `1 = Away`) is in the authoritative Attacking control context.
    #[func]
    pub fn is_side_attacking(&self, side: i32) -> bool {
        let Some(ref sim) = self.sim else {
            return false;
        };
        let target = if side == 0 {
            psfc_sim::TeamSide::Home
        } else {
            psfc_sim::TeamSide::Away
        };
        sim.state().is_attacking_context(target)
    }

    #[func]
    pub fn get_phase_label(&self) -> GString {
        let Some(ref sim) = self.sim else {
            return GString::from("uninitialized");
        };
        GString::from(sim.state().phase.label())
    }

    #[func]
    pub fn get_display_minute(&self) -> i32 {
        self.sim
            .as_ref()
            .map(|s| s.state().display_minute() as i32)
            .unwrap_or(0)
    }

    #[func]
    pub fn is_finished(&self) -> bool {
        self.sim.as_ref().is_some_and(|s| s.is_finished())
    }

    #[func]
    pub fn get_state_hash_hex(&self) -> GString {
        let Some(ref sim) = self.sim else {
            return GString::new();
        };
        GString::from(&format!("{:016x}", sim.state_hash()))
    }

    #[func]
    pub fn drain_events_json(&mut self) -> GString {
        let Some(ref mut sim) = self.sim else {
            return GString::from("[]");
        };
        let evs = sim.drain_events();
        let json = serde_json::to_string(&evs).unwrap_or_else(|_| "[]".to_string());
        GString::from(&json)
    }

    #[func]
    pub fn export_input_log_json(&self) -> GString {
        let Some(ref sim) = self.sim else {
            return GString::from("{}");
        };
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut frames_hex = String::with_capacity(self.recorded_frames.len() * 32);
        for [h, a] in &self.recorded_frames {
            for b in h.to_bytes().into_iter().chain(a.to_bytes()) {
                frames_hex.push(HEX[(b >> 4) as usize] as char);
                frames_hex.push(HEX[(b & 0x0F) as usize] as char);
            }
        }
        let log = GodotSessionLog {
            session_id: format!("godot-{}", sim.state().fixture_id),
            claimed_final_hash_hex: format!("{:016x}", sim.state_hash()),
            claimed_score: sim.state().score,
            frames_hex,
            tick_count: sim.state().tick,
        };
        let s = serde_json::to_string(&log).unwrap_or_else(|_| "{}".to_string());
        GString::from(&s)
    }
}
