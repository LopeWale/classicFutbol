#![forbid(unsafe_code)]

//! `psfc-replay`: Headless match replay verifier and deterministic benchmark CLI.
//!
//! Subcommands:
//! - `psfc-replay verify --snapshot <snapshot.json> --inputs <inputs.json> [--out <result.json>]`
//! - `psfc-replay simulate-demo [--format 5v5|11v11] [--seed <u64>] [--half-ticks <u32>] [--out-snapshot <f>] [--out-inputs <f>] [--out-result <f>]`

use psfc_sim::{
    CpuDifficulty, FixtureSnapshot, InputFrame, MatchEvent, MatchSimulation, PlayerMatchStats,
    SideController,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::process;
use std::time::Instant;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HashCheckpoint {
    pub tick: u32,
    pub hash_hex: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TickInputPair {
    pub home: InputFrame,
    pub away: InputFrame,
}

/// Canonical session input log consumed by `psfc-replay verify`.
/// Supports either `frames_hex` (compact 16-byte-per-tick hex: 8B home + 8B away)
/// or structured `frames` (`Vec<TickInputPair>`), plus `tick_count` for pure CPU replays.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SessionInputLog {
    #[serde(default)]
    pub session_id: String,
    #[serde(default)]
    pub claimed_final_hash_hex: Option<String>,
    #[serde(default)]
    pub claimed_score: Option<[u16; 2]>,
    #[serde(default)]
    pub hash_checkpoints: Vec<HashCheckpoint>,
    #[serde(default)]
    pub frames_hex: Option<String>,
    #[serde(default)]
    pub frames: Vec<TickInputPair>,
    #[serde(default)]
    pub tick_count: Option<u32>,
}

impl SessionInputLog {
    pub fn decode_frames(&self) -> Result<Vec<[InputFrame; 2]>, String> {
        if let Some(ref hex) = self.frames_hex {
            let trimmed: String = hex.chars().filter(|c| !c.is_whitespace()).collect();
            if !trimmed.len().is_multiple_of(32) {
                return Err(format!(
                    "frames_hex length ({}) must be a multiple of 32 hex chars (16 bytes/tick)",
                    trimmed.len()
                ));
            }
            let total_ticks = trimmed.len() / 32;
            let mut out = Vec::with_capacity(total_ticks);
            for t in 0..total_ticks {
                let slice = &trimmed[t * 32..(t + 1) * 32];
                let mut raw = [0_u8; 16];
                for (i, slot) in raw.iter_mut().enumerate() {
                    *slot = u8::from_str_radix(&slice[i * 2..i * 2 + 2], 16)
                        .map_err(|e| format!("invalid hex at tick {t}: {e}"))?;
                }
                let mut h = [0_u8; 8];
                let mut a = [0_u8; 8];
                h.copy_from_slice(&raw[0..8]);
                a.copy_from_slice(&raw[8..16]);
                out.push([InputFrame::from_bytes(h), InputFrame::from_bytes(a)]);
            }
            return Ok(out);
        }
        if !self.frames.is_empty() {
            return Ok(self.frames.iter().map(|p| [p.home, p.away]).collect());
        }
        let n = self.tick_count.unwrap_or(0) as usize;
        Ok(vec![[InputFrame::IDLE, InputFrame::IDLE]; n])
    }

    pub fn encode_frames_hex(pairs: &[[InputFrame; 2]]) -> String {
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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplayVerificationResult {
    pub fixture_id: String,
    pub session_id: String,
    pub gameplay_version: String,
    pub format_name: String,
    pub players_per_side: usize,
    pub ticks_simulated: u32,
    pub finished: bool,
    pub final_hash_hex: String,
    pub hash_verified: bool,
    pub hash_mismatch_reason: Option<String>,
    pub home_goals: u16,
    pub away_goals: u16,
    pub possession_home_pct: u8,
    pub elapsed_ms: u128,
    pub hash_checkpoints: Vec<HashCheckpoint>,
    pub events: Vec<MatchEvent>,
    pub home_player_stats: Vec<PlayerMatchStats>,
    pub away_player_stats: Vec<PlayerMatchStats>,
}

fn format_hash_hex(hash: u64) -> String {
    format!("{hash:016x}")
}

fn normalize_hash_str(s: &str) -> String {
    s.trim()
        .trim_start_matches("0x")
        .trim_start_matches("0X")
        .to_ascii_lowercase()
}

fn run_verify(snapshot_path: &str, inputs_path: &str, out_path: Option<&str>) -> Result<(), String> {
    let snap_raw = fs::read_to_string(snapshot_path)
        .map_err(|e| format!("failed to read snapshot '{snapshot_path}': {e}"))?;
    let snapshot: FixtureSnapshot = serde_json::from_str(&snap_raw)
        .map_err(|e| format!("invalid snapshot JSON '{snapshot_path}': {e}"))?;

    let inputs_raw = fs::read_to_string(inputs_path)
        .map_err(|e| format!("failed to read inputs '{inputs_path}': {e}"))?;
    let input_log: SessionInputLog = serde_json::from_str(&inputs_raw)
        .map_err(|e| format!("invalid input log JSON '{inputs_path}': {e}"))?;

    let frames = input_log.decode_frames()?;
    let started = Instant::now();
    let mut sim = MatchSimulation::new(&snapshot).map_err(|e| e.to_string())?;

    let mut checkpoints = Vec::new();
    let mut mismatch_reason: Option<String> = None;

    for pair in &frames {
        if sim.is_finished() {
            break;
        }
        sim.tick(pair);
        let t = sim.state().tick;
        if t % 60 == 0 {
            let hx = format_hash_hex(sim.state_hash());
            if let Some(claimed_cp) = input_log.hash_checkpoints.iter().find(|c| c.tick == t) {
                if normalize_hash_str(&claimed_cp.hash_hex) != hx && mismatch_reason.is_none() {
                    mismatch_reason = Some(format!(
                        "checkpoint hash mismatch at tick {t}: client={}, replay={hx}",
                        claimed_cp.hash_hex
                    ));
                }
            }
            checkpoints.push(HashCheckpoint {
                tick: t,
                hash_hex: hx,
            });
        }
    }

    let final_hash_hex = format_hash_hex(sim.state_hash());
    if let Some(ref claimed_final) = input_log.claimed_final_hash_hex {
        if normalize_hash_str(claimed_final) != final_hash_hex && mismatch_reason.is_none() {
            mismatch_reason = Some(format!(
                "final hash mismatch: client={claimed_final}, replay={final_hash_hex}"
            ));
        }
    }

    let st = sim.state();
    let total_poss = (st.possession_ticks[0] + st.possession_ticks[1]).max(1);
    let poss_home = ((st.possession_ticks[0] as u64 * 100) / (total_poss as u64)) as u8;

    let result = ReplayVerificationResult {
        fixture_id: snapshot.fixture_id,
        session_id: input_log.session_id,
        gameplay_version: snapshot.gameplay_version,
        format_name: st.format.name.clone(),
        players_per_side: st.format.players_per_side,
        ticks_simulated: st.tick,
        finished: sim.is_finished(),
        final_hash_hex,
        hash_verified: mismatch_reason.is_none(),
        hash_mismatch_reason: mismatch_reason.clone(),
        home_goals: st.score[0],
        away_goals: st.score[1],
        possession_home_pct: poss_home,
        elapsed_ms: started.elapsed().as_millis(),
        hash_checkpoints: checkpoints,
        events: st.recorded_events.clone(),
        home_player_stats: st.player_stats[0].clone(),
        away_player_stats: st.player_stats[1].clone(),
    };

    let json_out = serde_json::to_string_pretty(&result).map_err(|e| e.to_string())?;
    if let Some(path) = out_path {
        fs::write(path, &json_out).map_err(|e| format!("failed to write '{path}': {e}"))?;
    } else {
        println!("{json_out}");
    }

    if let Some(reason) = mismatch_reason {
        return Err(reason);
    }
    Ok(())
}

fn run_simulate_demo(args: &[String]) -> Result<(), String> {
    let mut format = "5v5".to_string();
    let mut seed: u64 = 20_261_118;
    let mut half_ticks: u32 = 1_800; // 30s halves by default for quick demo
    let mut out_snapshot: Option<String> = None;
    let mut out_inputs: Option<String> = None;
    let mut out_result: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--format" => {
                i += 1;
                format = args.get(i).cloned().ok_or("missing --format value")?;
            }
            "--seed" => {
                i += 1;
                seed = args
                    .get(i)
                    .ok_or("missing --seed value")?
                    .parse()
                    .map_err(|_| "invalid --seed integer")?;
            }
            "--half-ticks" => {
                i += 1;
                half_ticks = args
                    .get(i)
                    .ok_or("missing --half-ticks value")?
                    .parse()
                    .map_err(|_| "invalid --half-ticks integer")?;
            }
            "--out-snapshot" => {
                i += 1;
                out_snapshot = Some(args.get(i).cloned().ok_or("missing --out-snapshot path")?);
            }
            "--out-inputs" => {
                i += 1;
                out_inputs = Some(args.get(i).cloned().ok_or("missing --out-inputs path")?);
            }
            "--out-result" => {
                i += 1;
                out_result = Some(args.get(i).cloned().ok_or("missing --out-result path")?);
            }
            other => return Err(format!("unknown argument '{other}'")),
        }
        i += 1;
    }

    let mut snapshot = if format == "11v11" {
        FixtureSnapshot::sample_eleven_a_side(seed)
    } else {
        FixtureSnapshot::sample_five_a_side(seed)
    };
    snapshot.rules.half_duration_ticks = half_ticks;
    snapshot.rules.home_controller = SideController::Cpu(CpuDifficulty::Hard);
    snapshot.rules.away_controller = SideController::Cpu(CpuDifficulty::Normal);

    let started = Instant::now();
    let mut sim = MatchSimulation::new(&snapshot).map_err(|e| e.to_string())?;
    let mut pairs: Vec<[InputFrame; 2]> = Vec::new();
    let mut checkpoints = Vec::new();

    let safety_cap = half_ticks.saturating_mul(3).saturating_add(2_000);
    while !sim.is_finished() && sim.state().tick < safety_cap {
        let pair = [InputFrame::IDLE, InputFrame::IDLE];
        sim.tick(&pair);
        pairs.push(pair);
        let t = sim.state().tick;
        if t % 60 == 0 {
            checkpoints.push(HashCheckpoint {
                tick: t,
                hash_hex: format_hash_hex(sim.state_hash()),
            });
        }
    }

    let final_hash_hex = format_hash_hex(sim.state_hash());
    let st = sim.state();
    let total_poss = (st.possession_ticks[0] + st.possession_ticks[1]).max(1);
    let poss_home = ((st.possession_ticks[0] as u64 * 100) / (total_poss as u64)) as u8;

    if let Some(ref p) = out_snapshot {
        let s = serde_json::to_string_pretty(&snapshot).map_err(|e| e.to_string())?;
        fs::write(p, s).map_err(|e| e.to_string())?;
    }
    if let Some(ref p) = out_inputs {
        let log = SessionInputLog {
            session_id: format!("demo-{seed}"),
            claimed_final_hash_hex: Some(final_hash_hex.clone()),
            claimed_score: Some(st.score),
            hash_checkpoints: checkpoints.clone(),
            frames_hex: Some(SessionInputLog::encode_frames_hex(&pairs)),
            frames: Vec::new(),
            tick_count: Some(st.tick),
        };
        let s = serde_json::to_string_pretty(&log).map_err(|e| e.to_string())?;
        fs::write(p, s).map_err(|e| e.to_string())?;
    }

    let result = ReplayVerificationResult {
        fixture_id: snapshot.fixture_id,
        session_id: format!("demo-{seed}"),
        gameplay_version: snapshot.gameplay_version,
        format_name: st.format.name.clone(),
        players_per_side: st.format.players_per_side,
        ticks_simulated: st.tick,
        finished: sim.is_finished(),
        final_hash_hex,
        hash_verified: true,
        hash_mismatch_reason: None,
        home_goals: st.score[0],
        away_goals: st.score[1],
        possession_home_pct: poss_home,
        elapsed_ms: started.elapsed().as_millis(),
        hash_checkpoints: checkpoints,
        events: st.recorded_events.clone(),
        home_player_stats: st.player_stats[0].clone(),
        away_player_stats: st.player_stats[1].clone(),
    };
    let json_out = serde_json::to_string_pretty(&result).map_err(|e| e.to_string())?;
    if let Some(ref p) = out_result {
        fs::write(p, json_out).map_err(|e| e.to_string())?;
    } else {
        println!("{json_out}");
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!(
            "Usage:\n  psfc-replay verify --snapshot <snapshot.json> --inputs <inputs.json> [--out <result.json>]\n  psfc-replay simulate-demo [--format 5v5|11v11] [--seed <u64>] [--half-ticks <u32>]"
        );
        process::exit(2);
    }

    // Support both `psfc-replay --snapshot ... --inputs ...` (direct shorthand from docs §7.3)
    // and `psfc-replay verify --snapshot ... --inputs ...`
    let (subcmd, rest) = if args[0].starts_with("--") {
        ("verify", &args[..])
    } else {
        (args[0].as_str(), &args[1..])
    };

    let res = match subcmd {
        "verify" => {
            let mut snapshot: Option<String> = None;
            let mut inputs: Option<String> = None;
            let mut out: Option<String> = None;
            let mut i = 0;
            while i < rest.len() {
                match rest[i].as_str() {
                    "--snapshot" => {
                        i += 1;
                        snapshot = rest.get(i).cloned();
                    }
                    "--inputs" => {
                        i += 1;
                        inputs = rest.get(i).cloned();
                    }
                    "--out" => {
                        i += 1;
                        out = rest.get(i).cloned();
                    }
                    other => {
                        eprintln!("error: unknown argument '{other}'");
                        process::exit(2);
                    }
                }
                i += 1;
            }
            match (snapshot, inputs) {
                (Some(s), Some(inp)) => run_verify(&s, &inp, out.as_deref()),
                _ => Err("--snapshot and --inputs are required".to_string()),
            }
        }
        "simulate-demo" => run_simulate_demo(rest),
        other => Err(format!("unknown subcommand '{other}'")),
    };

    if let Err(err) = res {
        eprintln!("psfc-replay error: {err}");
        process::exit(1);
    }
}
