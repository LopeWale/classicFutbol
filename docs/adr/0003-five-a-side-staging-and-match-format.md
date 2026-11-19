# ADR-0003: Five-a-side staging (4 outfield + GK) and parameterized match format

Status: accepted · Date: 2026-11-18 · Updates gameplay staging in `01-requirements-review.md` §4 and `02-system-architecture.md` §12

## Context

The full product vision targets 11-a-side real-time football with classic FIFA-inspired feel alongside a persistent shared-world season ledger, custom controllers, and profile cartridges. Building full 22-player tactical spacing, offside traps, and full-pitch AI simultaneously with core ball physics, goalkeeper mechanics, rollback netcode, and ledger integration slows down gameplay iteration and obscures bugs in individual interactions.

Getting a tight, responsive small-sided game right first lets us validate movement, dribbling, passing, shooting, tackling, player switching, goalkeeper AI, deterministic replay verification, and season record-keeping end to end before scaling tactical density to 22 players.

## Decision

1. **Initial gameplay milestone is 5v5 (4 outfield players + 1 goalkeeper = 5 players per team, 10 on pitch).**
   - Default 5v5 pitch dimensions: `56 m × 36 m`, penalty box `18 m × 9 m`, goal frame `4.0 m × 2.1 m`.
   - Standard outdoor touchlines and goal lines apply (throw-ins, corners, goal kicks, fouls, penalties) so rules carry forward directly to 11v11.
   - Offsides are disabled by default in 5v5 (`RuleSet.offsides_enabled = false`) and enabled in 11v11 (`RuleSet.offsides_enabled = true`).
2. **Team size, pitch geometry, formations, and rules are parameterized via `MatchFormat` and `RuleSet` in `psfc-sim`.**
   - Nothing in `psfc-sim`, `psfc-rollback`, `psfc-replay`, `psfc-godot`, `psfc-wasm`, or `psfc_host` hardcodes `5` or `11`.
   - `MatchFormat::five_a_side()` (`players_per_side = 5`) is the Phase 1–2 default; `MatchFormat::eleven_a_side()` (`players_per_side = 11`, `105 m × 68 m` pitch, `7.32 m × 2.44 m` goals) is built into the same data model for expansion once 5v5 gameplay is signed off.
   - Frozen `fixture_snapshot` records the `MatchFormat` and `RuleSet` used for every official match so mixed-format archives remain replayable forever.
3. **Include a lightweight `psfc-wasm` browser preview harness alongside the Godot 4 + `psfc-godot` client.**
   - Godot 4 with `psfc-godot` (GDExtension) remains the primary game client architecture (ADR-0001, ADR-0002).
   - Compiling `psfc-sim` into `psfc-wasm` (`wasm32-unknown-unknown`) and serving a zero-install 3D broadcast preview page from `psfc_host` enables immediate playtesting with gamepads or keyboard on any machine without requiring a local Godot editor build, while simultaneously proving cross-architecture determinism (`wasm32` client vs `x86_64`/`aarch64` `psfc-replay` host validator).

## Consequences

- Every Parallel Season FC invariant (one fixture one result, takeover precedence, inherited vs earned statistics, frozen squad snapshots, host replay validation) works identically for 5v5 fixtures now and 11v11 fixtures later.
- Squad snapshots in 5v5 select 1 goalkeeper + 4 starting outfielders (+ substitutes) from the club's full squad catalogue; unpicked squad members remain on the bench or reserve list without altering catalogue schemas.
- Tier-B render budgets (Raspberry Pi 5) are even easier to hit with 10 skinned footballers in 5v5, while the 22-player render gate remains required before promoting 11v11 on Tier B.
