//! A human side driven by `ai::suggest_cpu_input` from OUTSIDE the tick (the WASM auto-pilot, HUD
//! suggestions) must leave the simulation exactly as reproducible from the recorded inputs as a
//! real controller would: the recorded log replays to the same hash series, and the suggestion
//! itself never changes the state hash. (Regression: `generate_cpu_input` writes the team plan and
//! shape targets into the hashed `ai_memory`; called outside `tick` that made the psfc-replay
//! checkpoint verification of auto-piloted matches fail.)

use psfc_sim::{CpuDifficulty, FixtureSnapshot, InputFrame, MatchSimulation, SideController, TeamSide};

fn snapshot(seed: u64) -> FixtureSnapshot {
    let mut snap = FixtureSnapshot::sample_five_a_side(seed);
    snap.rules.home_controller = SideController::Human;
    snap.rules.away_controller = SideController::Cpu(CpuDifficulty::Hard);
    snap.rules.half_duration_ticks = 60 * 60;
    snap
}

#[test]
fn suggest_cpu_input_leaves_state_hash_unchanged() {
    let snap = snapshot(7);
    let mut sim = MatchSimulation::new(&snap).unwrap();
    for t in 0..600 {
        let before = sim.state_hash();
        let frame = psfc_sim::ai::suggest_cpu_input(sim.state(), TeamSide::Home, CpuDifficulty::Hard);
        assert_eq!(before, sim.state_hash(), "suggestion changed the state at tick {t}");
        sim.tick(&[frame, InputFrame::IDLE]);
    }
}

#[test]
fn auto_piloted_human_side_replays_to_identical_hashes() {
    for seed in [7_u64, 1_234] {
        let snap = snapshot(seed);
        let mut live = MatchSimulation::new(&snap).unwrap();
        let mut frames = Vec::new();
        let mut hashes = Vec::new();
        for _ in 0..1_500 {
            let home = psfc_sim::ai::suggest_cpu_input(live.state(), TeamSide::Home, CpuDifficulty::Hard);
            let pair = [home, InputFrame::IDLE];
            live.tick(&pair);
            frames.push(pair);
            hashes.push(live.state_hash());
        }
        assert!(frames.iter().any(|p| p[0] != InputFrame::IDLE), "auto-pilot produced no input");
        let mut replay = MatchSimulation::new(&snap).unwrap();
        for (t, pair) in frames.iter().enumerate() {
            replay.tick(pair);
            assert_eq!(replay.state_hash(), hashes[t], "seed {seed}: replay diverged at tick {}", t + 1);
        }
    }
}
