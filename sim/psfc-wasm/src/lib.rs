//! C-ABI WebAssembly FFI boundary crate for `psfc-sim` (which enforces `#![forbid(unsafe_code)]`).
//!
//! Zero `unsafe` blocks and zero external JS bundler requirements: host environments
//! (browsers and Node.js CI) read/write linear memory buffers via exported pointers.

use psfc_sim::ai::generate_cpu_input;
use psfc_sim::{
    CpuDifficulty, FixtureSnapshot, InputFrame, MatchPhase, MatchSimulation, PlayerActionState,
    SideController, TeamSide,
};
use serde::Serialize;
use std::cell::RefCell;

struct WasmSession {
    snapshot: FixtureSnapshot,
    sim: MatchSimulation,
    frames: Vec<[InputFrame; 2]>,
    checkpoints: Vec<HashCheckpointDto>,
}

#[derive(Clone, Serialize)]
struct HashCheckpointDto {
    tick: u32,
    hash_hex: String,
}

#[derive(Serialize)]
struct ExportedInputLog {
    session_id: String,
    claimed_final_hash_hex: String,
    claimed_score: [u16; 2],
    hash_checkpoints: Vec<HashCheckpointDto>,
    frames_hex: String,
    tick_count: u32,
}

#[derive(Serialize)]
struct ExportedSummary<'a> {
    fixture_id: &'a str,
    format_name: &'a str,
    players_per_side: usize,
    tick: u32,
    clock_ticks: u32,
    half: u8,
    finished: bool,
    final_hash_hex: String,
    score: [u16; 2],
    went_to_extra_time: bool,
    shootout_score: Option<[u8; 2]>,
    possession_ticks: [u32; 2],
    home_player_stats: Vec<psfc_sim::PlayerMatchStats>,
    away_player_stats: Vec<psfc_sim::PlayerMatchStats>,
    events: &'a [psfc_sim::MatchEvent],
}

#[derive(Serialize)]
struct ExportedTeams<'a> {
    home: &'a psfc_sim::TeamSnapshot,
    away: &'a psfc_sim::TeamSnapshot,
    subs_used: [u8; 2],
}

thread_local! {
    static INPUT_BUF: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    static OUTPUT_BUF: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    static PACKED_STATE: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) };
    static SESSION: RefCell<Option<WasmSession>> = const { RefCell::new(None) };
}

fn set_output_bytes(bytes: Vec<u8>) -> usize {
    let len = bytes.len();
    OUTPUT_BUF.with(|b| {
        *b.borrow_mut() = bytes;
    });
    len
}

fn decode_controller(mode: u32) -> SideController {
    match mode {
        0 => SideController::Human,
        1 => SideController::Cpu(CpuDifficulty::Easy),
        2 => SideController::Cpu(CpuDifficulty::Normal),
        _ => SideController::Cpu(CpuDifficulty::Hard),
    }
}

fn encode_frames_hex(pairs: &[[InputFrame; 2]]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(pairs.len() * 32);
    for [h, a] in pairs {
        for b in h.to_bytes().into_iter().chain(a.to_bytes()) {
            out.push(HEX[(b >> 4) as usize] as char);
            out.push(HEX[(b & 0x0F) as usize] as char);
        }
    }
    out
}

#[no_mangle]
pub extern "C" fn wasm_prepare_input_buf(len: usize) -> *mut u8 {
    INPUT_BUF.with(|buf| {
        let mut b = buf.borrow_mut();
        b.clear();
        b.resize(len, 0);
        b.as_mut_ptr()
    })
}

#[no_mangle]
pub extern "C" fn wasm_output_ptr() -> *const u8 {
    OUTPUT_BUF.with(|buf| buf.borrow().as_ptr())
}

#[no_mangle]
pub extern "C" fn wasm_output_len() -> usize {
    OUTPUT_BUF.with(|buf| buf.borrow().len())
}

#[no_mangle]
pub extern "C" fn wasm_packed_state_ptr() -> *const i32 {
    PACKED_STATE.with(|buf| buf.borrow().as_ptr())
}

#[no_mangle]
pub extern "C" fn wasm_packed_state_len() -> usize {
    PACKED_STATE.with(|buf| buf.borrow().len())
}

/// Initialize a preset 5v5 (`format_code = 0`) or 11v11 (`format_code = 1`) match.
/// Writes the resulting `FixtureSnapshot` JSON to `OUTPUT_BUF` and returns `1` on success.
#[no_mangle]
pub extern "C" fn sim_init_preset(
    format_code: u32,
    seed_lo: u32,
    seed_hi: u32,
    half_ticks: u32,
    home_mode: u32,
    away_mode: u32,
) -> i32 {
    let seed = ((seed_hi as u64) << 32) | (seed_lo as u64);
    let mut snapshot = if format_code == 1 {
        FixtureSnapshot::sample_eleven_a_side(seed)
    } else {
        FixtureSnapshot::sample_five_a_side(seed)
    };
    if half_ticks >= 60 {
        snapshot.rules.half_duration_ticks = half_ticks;
    }
    snapshot.rules.home_controller = decode_controller(home_mode);
    snapshot.rules.away_controller = decode_controller(away_mode);

    match MatchSimulation::new(&snapshot) {
        Ok(sim) => {
            let json = serde_json::to_vec(&snapshot).unwrap_or_default();
            set_output_bytes(json);
            SESSION.with(|s| {
                *s.borrow_mut() = Some(WasmSession {
                    snapshot,
                    sim,
                    frames: Vec::with_capacity((half_ticks as usize) * 2 + 512),
                    checkpoints: Vec::new(),
                });
            });
            1
        }
        Err(e) => {
            set_output_bytes(e.to_string().into_bytes());
            0
        }
    }
}

/// Initialize from a frozen `FixtureSnapshot` JSON written into `INPUT_BUF`.
#[no_mangle]
pub extern "C" fn sim_init_from_snapshot_json() -> i32 {
    let parsed: Result<FixtureSnapshot, String> = INPUT_BUF.with(|buf| {
        serde_json::from_slice(&buf.borrow()).map_err(|e| e.to_string())
    });
    let snapshot = match parsed {
        Ok(s) => s,
        Err(err) => {
            set_output_bytes(err.into_bytes());
            return 0;
        }
    };
    match MatchSimulation::new(&snapshot) {
        Ok(sim) => {
            let json = serde_json::to_vec(&snapshot).unwrap_or_default();
            set_output_bytes(json);
            SESSION.with(|s| {
                *s.borrow_mut() = Some(WasmSession {
                    snapshot,
                    sim,
                    frames: Vec::new(),
                    checkpoints: Vec::new(),
                });
            });
            1
        }
        Err(err) => {
            set_output_bytes(err.to_string().into_bytes());
            0
        }
    }
}

/// Advance one 60 Hz tick and record the exact `[home, away]` input frames.
/// Upper 16 bits of `h_btns` / `a_btns` carry `InputFrame.reserved` (e.g. deterministic substitution command).
#[no_mangle]
pub extern "C" fn sim_tick(
    h_mx: i32,
    h_my: i32,
    h_ax: i32,
    h_ay: i32,
    h_btns: u32,
    a_mx: i32,
    a_my: i32,
    a_ax: i32,
    a_ay: i32,
    a_btns: u32,
) -> u32 {
    let home = InputFrame {
        move_x: h_mx.clamp(-127, 127) as i8,
        move_y: h_my.clamp(-127, 127) as i8,
        aim_x: h_ax.clamp(-127, 127) as i8,
        aim_y: h_ay.clamp(-127, 127) as i8,
        buttons: (h_btns & 0xFFFF) as u16,
        reserved: ((h_btns >> 16) & 0xFFFF) as u16,
    };
    let away = InputFrame {
        move_x: a_mx.clamp(-127, 127) as i8,
        move_y: a_my.clamp(-127, 127) as i8,
        aim_x: a_ax.clamp(-127, 127) as i8,
        aim_y: a_ay.clamp(-127, 127) as i8,
        buttons: (a_btns & 0xFFFF) as u16,
        reserved: ((a_btns >> 16) & 0xFFFF) as u16,
    };

    SESSION.with(|cell| {
        let mut guard = cell.borrow_mut();
        let Some(sess) = guard.as_mut() else {
            return 0;
        };
        if !sess.sim.is_finished() {
            let pair = [home, away];
            sess.sim.tick(&pair);
            sess.frames.push(pair);
            let t = sess.sim.state().tick;
            if t % 60 == 0 {
                sess.checkpoints.push(HashCheckpointDto {
                    tick: t,
                    hash_hex: format!("{:016x}", sess.sim.state_hash()),
                });
            }
        }
        sess.sim.state().tick
    })
}

/// Fast-forward up to `steps` ticks using deterministic tactical auto-inputs for any Human side
/// while logging those exact `InputFrame`s so host replay validation still matches 100% bit-for-bit!
#[no_mangle]
pub extern "C" fn sim_auto_step(steps: u32) -> u32 {
    SESSION.with(|cell| {
        let mut guard = cell.borrow_mut();
        let Some(sess) = guard.as_mut() else {
            return 0;
        };
        for _ in 0..steps {
            if sess.sim.is_finished() {
                break;
            }
            let h_in = match sess.snapshot.rules.home_controller {
                SideController::Human => {
                    generate_cpu_input(sess.sim.state(), TeamSide::Home, CpuDifficulty::Hard)
                }
                SideController::Cpu(_) => InputFrame::IDLE,
            };
            let a_in = match sess.snapshot.rules.away_controller {
                SideController::Human => {
                    generate_cpu_input(sess.sim.state(), TeamSide::Away, CpuDifficulty::Hard)
                }
                SideController::Cpu(_) => InputFrame::IDLE,
            };
            let pair = [h_in, a_in];
            sess.sim.tick(&pair);
            sess.frames.push(pair);
            let t = sess.sim.state().tick;
            if t % 60 == 0 {
                sess.checkpoints.push(HashCheckpointDto {
                    tick: t,
                    hash_hex: format!("{:016x}", sess.sim.state_hash()),
                });
            }
        }
        sess.sim.state().tick
    })
}

/// Packs current render state into `PACKED_STATE` (`i32` array) for zero-GC 60 FPS JS rendering:
/// Header (18 `i32`s):
/// `[0]=tick`, `[1]=clock_ticks`, `[2]=half`, `[3]=phase_code`, `[4]=home_score`, `[5]=away_score`,
/// `[6]=players_per_side`, `[7]=home_ctrl_idx`, `[8]=away_ctrl_idx`,
/// `[9]=ball_x_mm`, `[10]=ball_y_mm`, `[11]=ball_z_mm`, `[12]=ball_vx_mm`, `[13]=ball_vy_mm`, `[14]=ball_vz_mm`,
/// `[15]=ball_owner_side (-1,0,1)`, `[16]=ball_owner_idx`, `[17]=pending_event_count`
/// Followed by `2 * players_per_side` blocks of 10 `i32`s:
/// `[side, idx, shirt_no, role_code, x_mm, y_mm, vx_mm, vy_mm, facing_bam, flags_stamina_charge]`
/// where `flags_stamina_charge`:
/// - bits `0..=3`: `action_code` (`0..7`)
/// - bits `4..=14`: `stamina_permille` (`0..=1000`)
/// - bits `15..=25`: `charge_permille` (`0..=1000`)
/// - bit `26`: `sent_off` (`0..1`)
/// - bit `27`: `has_yellow_card` (`0..1`)
/// - bits `28..=31`: `jump_z_steps` (`0..=15`, `60mm` per step -> `0..=900mm` aerial header elevation!)
#[no_mangle]
pub extern "C" fn sim_pack_render_state() -> usize {
    SESSION.with(|cell| {
        let guard = cell.borrow();
        let Some(sess) = guard.as_ref() else {
            return 0;
        };
        let st = sess.sim.state();
        let pps = st.format.players_per_side;
        let phase_code: i32 = match st.phase {
            MatchPhase::KickOff { .. } => 0,
            MatchPhase::OpenPlay => 1,
            MatchPhase::SetPiece { kind, .. } => 2 + (kind as i32),
            MatchPhase::GoalScored { .. } => 10,
            MatchPhase::HalfTime { .. } => 11,
            MatchPhase::FullTime => 12,
        };
        let (owner_side, owner_idx) = match st.phase {
            MatchPhase::GoalScored {
                scoring_team,
                scorer_idx,
                ..
            } => (scoring_team.index() as i32, scorer_idx as i32),
            _ => match st.ball.owner {
                Some((s, idx)) => (s.index() as i32, idx as i32),
                None => (-1, -1),
            },
        };

        PACKED_STATE.with(|buf| {
            let mut out = buf.borrow_mut();
            out.clear();
            out.reserve(20 + pps * 2 * 10);
            out.extend_from_slice(&[
                st.tick as i32,
                st.clock_ticks as i32,
                st.half as i32,
                phase_code,
                st.score[0] as i32,
                st.score[1] as i32,
                pps as i32,
                st.controlled_idx[0] as i32,
                st.controlled_idx[1] as i32,
                st.ball.pos.x.to_milli() as i32,
                st.ball.pos.y.to_milli() as i32,
                st.ball.pos.z.to_milli() as i32,
                st.ball.vel.x.to_milli() as i32,
                st.ball.vel.y.to_milli() as i32,
                st.ball.vel.z.to_milli() as i32,
                owner_side,
                owner_idx,
                st.pending_events.len() as i32,
            ]);

            for (s, side_players) in st.players.iter().enumerate() {
                for p in side_players {
                    let action_code: i32 = match p.action {
                        PlayerActionState::Normal => 0,
                        PlayerActionState::Charging { .. } => 1,
                        PlayerActionState::Tackling { .. } => 2,
                        PlayerActionState::Sliding { .. } => 3,
                        PlayerActionState::Diving { .. } => 4,
                        PlayerActionState::Stumbled { .. } => 5,
                        PlayerActionState::Cooldown { .. } => 6,
                        PlayerActionState::SkillMove { .. } => 7,
                    };
                    let stamina = p.stamina_permille.clamp(0, 1000);
                    let charge = (p.charge_ratio_permille() as i32).clamp(0, 1000);
                    let has_yellow = i32::from(p.yellow_cards > 0);
                    let jump_step = ((p.z.to_milli().clamp(0, 900)) / 60) as i32;
                    let packed = (action_code & 0xF)
                        | ((stamina & 0x7FF) << 4)
                        | ((charge & 0x7FF) << 15)
                        | ((i32::from(p.sent_off)) << 26)
                        | (has_yellow << 27)
                        | ((jump_step & 0xF) << 28);

                    out.extend_from_slice(&[
                        s as i32,
                        p.index as i32,
                        p.shirt_number as i32,
                        p.role as i32,
                        p.pos.x.to_milli() as i32,
                        p.pos.y.to_milli() as i32,
                        p.vel.x.to_milli() as i32,
                        p.vel.y.to_milli() as i32,
                        p.facing_bam as i32,
                        packed,
                    ]);
                }
            }
            // Trailing spin words `[spin_side_milli, spin_top_milli]` preserve all header & player offsets
            out.extend_from_slice(&[
                st.ball.spin_side.to_milli() as i32,
                st.ball.spin_top.to_milli() as i32,
            ]);
            out.len()
        })
    })
}

#[no_mangle]
pub extern "C" fn sim_drain_events_json() -> usize {
    SESSION.with(|cell| {
        let mut guard = cell.borrow_mut();
        let Some(sess) = guard.as_mut() else {
            return set_output_bytes(b"[]".to_vec());
        };
        let events = sess.sim.drain_events();
        let json = serde_json::to_vec(&events).unwrap_or_else(|_| b"[]".to_vec());
        set_output_bytes(json)
    })
}

#[no_mangle]
pub extern "C" fn sim_export_teams_json() -> usize {
    SESSION.with(|cell| {
        let guard = cell.borrow();
        let Some(sess) = guard.as_ref() else {
            return set_output_bytes(b"{}".to_vec());
        };
        let st = sess.sim.state();
        let dto = ExportedTeams {
            home: &st.teams[0],
            away: &st.teams[1],
            subs_used: st.subs_used,
        };
        let json = serde_json::to_vec(&dto).unwrap_or_else(|_| b"{}".to_vec());
        set_output_bytes(json)
    })
}

#[no_mangle]
pub extern "C" fn sim_state_hash_hex() -> usize {
    SESSION.with(|cell| {
        let guard = cell.borrow();
        let Some(sess) = guard.as_ref() else {
            return set_output_bytes(Vec::new());
        };
        let hx = format!("{:016x}", sess.sim.state_hash());
        set_output_bytes(hx.into_bytes())
    })
}

#[no_mangle]
pub extern "C" fn sim_export_input_log_json() -> usize {
    SESSION.with(|cell| {
        let guard = cell.borrow();
        let Some(sess) = guard.as_ref() else {
            return set_output_bytes(b"{}".to_vec());
        };
        let st = sess.sim.state();
        let log = ExportedInputLog {
            session_id: format!("wasm-{}", sess.snapshot.fixture_id),
            claimed_final_hash_hex: format!("{:016x}", sess.sim.state_hash()),
            claimed_score: st.score,
            hash_checkpoints: sess.checkpoints.clone(),
            frames_hex: encode_frames_hex(&sess.frames),
            tick_count: st.tick,
        };
        let json = serde_json::to_vec(&log).unwrap_or_else(|_| b"{}".to_vec());
        set_output_bytes(json)
    })
}

#[no_mangle]
pub extern "C" fn sim_export_summary_json() -> usize {
    SESSION.with(|cell| {
        let guard = cell.borrow();
        let Some(sess) = guard.as_ref() else {
            return set_output_bytes(b"{}".to_vec());
        };
        let st = sess.sim.state();
        let summary = ExportedSummary {
            fixture_id: &st.fixture_id,
            format_name: &st.format.name,
            players_per_side: st.format.players_per_side,
            tick: st.tick,
            clock_ticks: st.clock_ticks,
            half: st.half,
            finished: sess.sim.is_finished(),
            final_hash_hex: format!("{:016x}", sess.sim.state_hash()),
            score: st.score,
            went_to_extra_time: st.went_to_extra_time,
            shootout_score: st.shootout_score,
            possession_ticks: st.possession_ticks,
            home_player_stats: st.combined_player_stats(TeamSide::Home),
            away_player_stats: st.combined_player_stats(TeamSide::Away),
            events: &st.recorded_events,
        };
        let json = serde_json::to_vec(&summary).unwrap_or_else(|_| b"{}".to_vec());
        set_output_bytes(json)
    })
}
