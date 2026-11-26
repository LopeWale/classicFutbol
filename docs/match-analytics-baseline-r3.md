# Match analytics baseline, round 3 (movement coordination, CPU vs CPU)

This baseline was taken on simulation code at commit **`2dd0e3b`** (`docs: research report on FIFA/PES player movement coordination`), before the round-3 exclusivity-layer and avoidance/push-out work. `psfc-sim`, `psfc-wasm`, the browser client and the committed wasm were not modified; only `psfc-replay analyze` was extended (still read-only on `psfc-sim`: it ticks `MatchSimulation` and samples the public `MatchState` after every tick). Machine-readable data is in `docs/match-analytics-baseline-r3.json` (same schema as `psfc-replay analyze --json`).

## How to rerun / compare

```sh
cd sim
cargo build --release -p psfc-replay
./target/release/psfc-replay analyze --preset baseline --seeds 3 --seed-base 1 --minutes 10 --per-seed \
    --json ../docs/match-analytics-baseline-r3.json --md /tmp/r3.md
# after a sim change:
./target/release/psfc-replay analyze --preset baseline --seeds 3 --seed-base 1 --minutes 10 --per-seed \
    --json /tmp/after.json --md /tmp/after.md --compare ../docs/match-analytics-baseline-r3.json
```

`--compare` accepts any earlier baseline (`match-analytics-baseline.json`, `match-analytics-round2.json`): keys missing from the older file print `n/a` instead of failing.

**Old keys unchanged.** All pre-existing metric keys and values are unchanged: against `docs/match-analytics-round2.json` and against the JSON written by the unmodified tool at `2dd0e3b`, all 1,977 old values (teams, home, away, match and per-seed maps for all three configurations) are bit-identical, and `--compare ../docs/match-analytics-round2.json` reports `Final hashes identical to baseline: true` for all three configurations. The sim has not changed since round 2, so the round-2 numbers are also the round-3 "before" numbers for every old key. The new keys (47 per side) show as `n/a` in that comparison.

## New metrics (round 3)

All are computed over open-play ticks (`MatchPhase::OpenPlay`), deterministic, and stored per side under `collision.*`, `coord.*`, `shape.<phase>.*` and `move.*`. Exact definitions are in the "Metric definitions" section at the end. Summary:

- **Collisions** (`collision.{any,teammate,opponent}.*`): a pair is overlapping when its centres are closer than 0.8 m (the sim's `min_sep`). `ticks_per_min` (ticks with at least one overlapping pair), `ticks_gt1cm_per_min` (same, overlap deeper than 1 cm), `events_per_min` (overlap episodes), `depth_mean_m` / `depth_max_m`, `stuck_pairs` (episodes longer than 10 consecutive ticks) and `stuck_mean_ticks`. `any` and `opponent` are match-level (identical in the home and away maps); `teammate` is the side's own pairs.
- **Position-correction velocity** (`collision.correction_vel_*`): |Δpos/dt − vel| per player per tick. p50/p95/p99, `pct_gt_0_3`, plus `corrected_pct`, `corrected_p50_mps` / `corrected_p99_mps` (samples above 0.01 m/s) and `max_mps`.
- **Coordination** (`coord.*`): `dual_chaser_ticks_per_min`, `dual_chaser_pct_of_loose` and `max_simultaneous_chasers` (loose ball, within 8 m, closing faster than 1.5 m/s); `engagers3_ticks_per_min` and `engagers3_pct_of_carrier_ticks` (3 or more outfielders within 2.5 m of the opposing carrier); `cruise_over72_pct`.
- **Shape** (`shape.<phase>.*`, attacking = in possession, defending = out of possession): `length_m`, `width_m`, `stretch_m`, `nearest_mate_mean_m`, `nearest_mate_p10_m`.
- **Movement** (`move.*`): `side_flips_per_player_min`, `heading_change_p95_deg_per_tick_gt3mps`.

### Headline numbers (teams mean, 3 seeds x 10 min)

| metric | 5v5 hard | 5v5 normal | 11v11 normal |
|---|---:|---:|---:|
| collision.any.events_per_min | 56.7 | 47.8 | 47.5 |
| collision.any.ticks_per_min | 161 | 137 | 145 |
| collision.any.ticks_gt1cm_per_min | 0.50 | 0.17 | 0.27 |
| collision.teammate.events_per_min (per side) | 6.1 | 5.6 | 3.1 |
| collision.opponent.events_per_min | 44.5 | 36.6 | 41.2 |
| collision.any.depth_mean_m / depth_max_m | 0.0022 / 0.79 | 0.0008 / 0.52 | 0.0013 / 0.80 |
| collision.any.stuck_pairs (per match) | 32.0 | 27.7 | 31.0 |
| collision.any.stuck_mean_ticks | 22.3 | 22.2 | 21.4 |
| collision.teammate.stuck_pairs (per side) | 2.67 | 2.67 | 2.33 |
| collision.correction_vel_p50 / p95 / p99 (m/s) | 0 / 0 / 0.134 | 0 / 0 / 0.051 | 0 / 0 / 0.000 |
| collision.correction_vel_pct_gt_0_3 | 0.25 % | 0.21 % | 0.08 % |
| collision.correction_vel_corrected_pct | 1.21 % | 1.01 % | 0.49 % |
| collision.correction_vel_corrected_p50 / p99 (m/s) | 0.17 / 7.42 | 0.17 / 6.62 | 0.16 / 5.08 |
| collision.correction_vel_max_mps | 23.1 | 19.6 | 20.7 |
| coord.dual_chaser_ticks_per_min | 503 | 459 | 424 |
| coord.dual_chaser_pct_of_loose | 22.7 % | 21.7 % | 16.8 % |
| coord.max_simultaneous_chasers | 4.50 | 4.17 | 4.33 |
| coord.engagers3_ticks_per_min | 1.72 | 1.30 | 0.45 |
| coord.cruise_over72_pct | 38.9 % | 37.5 % | 38.4 % |
| move.side_flips_per_player_min | 71.7 | 70.1 | 44.8 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.50 | 3.48 | 3.18 |
| shape length in / out of possession (m) | 17.7 / 16.2 | 17.8 / 16.0 | 40.9 / 36.4 |
| shape width in / out (m) | 19.1 / 19.0 | 19.3 / 18.9 | 46.3 / 39.7 |
| shape stretch in / out (m) | 9.6 / 9.2 | 9.7 / 9.1 | **19.7 / 17.0** (envelope 7-16) |
| nearest mate mean in / out (m) | 10.6 / 10.3 | 10.7 / 10.3 | 12.4 / 10.6 |
| nearest mate p10 in / out (m) | 3.1 / 3.7 | 3.3 / 3.9 | 5.6 / 5.4 |

The peer-reviewed 11v11 envelope is length 31-46 m, width 35-48 m, stretch index 7-16 m. 11v11 length and width are inside it in both phases; the stretch index is above it in both phases (19.7 m in possession, 17.0 m out). 5v5 has no published envelope.

### Reading the numbers

- **Overlaps are resolved before sampling.** The public state is sampled after the sim's separation pass, which pushes every overlapping pair back to 0.8 m. Most overlap ticks are resting contact at 0.8 m minus fixed-point rounding (mean depth about 1-2 mm; only 0.2-0.5 ticks per minute have a pair deeper than 1 cm). So `events_per_min`, `ticks_per_min` and `stuck_pairs` count *contacts* (players pressed together, mostly opponents: shoulder-to-shoulder duels and the carrier), not visible interpenetration. The push-out itself shows up in the correction velocity.
- **Push-out is rare but violent.** About 0.5-1.2 % of player-ticks are corrected. The unconditional p99 is therefore near zero (11v11: 0.000 m/s, since under 1 % of samples are corrected), and the conditional p99 is 5-7 m/s, with maxima around 20-23 m/s (a 0.35-0.4 m jump in one tick when two players occupy nearly the same point). The research report's gate (p99 < 0.3 m/s, "current 1.25") should be read against `corrected_p99_mps` or `pct_gt_0_3`, because the per-tick p99 over all samples passes trivially while contact is under 1 % of ticks.
- **Exclusivity invariants are far from zero.** On about 17-23 % of loose-ball ticks two or more teammates are closing on the ball within 8 m, and up to 4-5 players of one side chase together. Note that loose-ball ticks include passes in flight, so an intended receiver plus one support runner already counts.
- **Cruise compliance:** about 38 % of eligible off-ball outfielder-ticks are faster than 72 % of sprint top speed, against Konami's "uninvolved players cruise at 70 %".
- **Side flips:** 45-72 lateral sign flips per player-minute (more than one per second) at speed > 1 m/s is a strong oscillation signature. The heading p95 at speed > 3 m/s is about 3.2-3.5 degrees per tick (190-210 deg/s).

### Not measurable from public state (and how it is approximated)

- **Pre-separation overlap and the true push-out displacement**: `resolve_player_separation` runs inside `tick()` before the state is visible, and it also changes `vel` (contact impulse, tangential "slip"). The correction velocity uses the published end-of-tick `vel_t`, so it equals push-out/dt minus the separation pass's velocity change. Both act roughly along the contact normal, so this is a lower bound on the pure push-out speed. Touchline clamping and GK own-half clamping also register as corrections (rare).
- **Chase mode / destination exclusivity**: the sim exposes no chaser flag, role assignment or movement target, so "two teammates in chase mode" and "two runners holding the same destination" cannot be counted. `coord.dual_chaser_*` is a kinematic proxy (closing on the loose ball), and pass receivers are not distinguished from chasers.
- **Top speed**: approximated as `(5.45 + 0.026·pace) m/s × 1.205 (5v5) / 1.220 (11v11)`, the sim's full-stamina, full-sprint-throttle cap. Stamina scaling, the facing-alignment penalty, jockey/pace-control multipliers and run boosts are not applied, so a tired player's real cap is lower than assumed and `cruise_over72_pct` is a lower bound.
- **Side flips per avoidance interaction**: the sim has no avoidance-interaction state, so flips are normalised per player-minute rather than per interaction.
- **Stuck-pair relative speed**: the report's variant (rel. speed < 0.5 m/s, about 15 ticks) is not used. The brief's definition (> 10 consecutive overlap ticks, no speed condition) is implemented.
- **Slot churn, ball-local compactness, keeper/defender double commits, attacker-keeper overlap and per-tick cross-target checksums** from the report's list are not part of this round's brief and were not added.

## 5v5 hard vs hard (10 min x 3 seeds from 1)

Command: `psfc-replay analyze --format 5v5 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty hard --away-difficulty hard`

Gameplay version: `0.1.0-5v5`

| seed | final state_hash | score (H-A) |
|---|---|---|
| 1 | `d11b03ffd918cd42` | 3-3 |
| 2 | `1b8d9eb6ff922d65` | 7-7 |
| 3 | `54697eba971cd616` | 4-6 |

Match meta (mean over seeds): goals_total = 10, open_play_minutes = 10, open_play_pct_of_ticks = 94.81, shots_total = 45.33, ticks_simulated = 37976

### Match

| metric | home | away | teams mean |
|---|---:|---:|---:|
| fouls | 0.33 | 0 | **0.17** |
| goals | 4.67 | 5.33 | **5** |
| offsides | 1 | 1.67 | **1.33** |
| passes_completed_pct | 85.16 | 87.11 | **86.14** |
| possession_pct | 47.31 | 52.69 | **50** |
| shots | 18 | 27.33 | **22.67** |
| shots_on_target | 14.33 | 21.67 | **18** |

### Goalkeeper

| metric | home | away | teams mean |
|---|---:|---:|---:|
| catches | 1.67 | 1 | **1.33** |
| claims_non_save | 2.33 | 0.33 | **1.33** |
| excursion_mean_s | 0.42 | 0 | **0.21** |
| excursions_gt1s_per_10min | 0.33 | 0 | **0.17** |
| gathers | 4 | 1.33 | **2.67** |
| goals_conceded | 5.33 | 4.67 | **5** |
| goals_conceded_gk_gt3m_off_line | 3.67 | 3.33 | **3.50** |
| goals_conceded_no_shot_found | 0 | 0 | **0** |
| line_dist_at_conceded_shot_mean_m | 2.59 | 2.83 | **2.71** |
| line_dist_max_m | 6.17 | 5.06 | **5.61** |
| line_dist_mean_m | 3.01 | 3.04 | **3.02** |
| parries | 15.33 | 8.67 | **12** |
| pct_outside_6yd | 0.07 | 0 | **0.03** |
| pct_outside_box | 0 | 0 | **0** |
| rush_count | 0.33 | 0 | **0.17** |
| rush_per_10min | 0.33 | 0 | **0.17** |
| saves | 17 | 9.67 | **13.33** |
| tips_over | 0 | 0 | **0** |

### Defence and shape

| metric | home | away | teams mean |
|---|---:|---:|---:|
| attacking.anchor_dist_m.defender | 8.56 | 9.23 | **8.90** |
| attacking.anchor_dist_m.forward | 15.17 | 13.95 | **14.56** |
| attacking.anchor_dist_m.goalkeeper | 1.24 | 1.18 | **1.21** |
| attacking.anchor_dist_m.midfielder | 14.27 | 14.27 | **14.27** |
| attacking.anchor_dist_shape_m.defender | 6.80 | 6.82 | **6.81** |
| attacking.anchor_dist_shape_m.forward | 10.81 | 10.96 | **10.88** |
| attacking.anchor_dist_shape_m.midfielder | 8.27 | 8.43 | **8.35** |
| attacking.bbox_area_m2 | 347.6 | 372.6 | **360.1** |
| attacking.bbox_depth_m | 17.30 | 18.06 | **17.68** |
| attacking.bbox_width_m | 18.95 | 19.34 | **19.15** |
| attacking.length_m | 17.30 | 18.06 | **17.68** |
| attacking.nearest_mate_mean_m | 10.44 | 10.81 | **10.62** |
| attacking.nearest_mate_p10_m | 3.10 | 3.14 | **3.12** |
| attacking.pct_of_open_play | 49.55 | 50.45 | **50** |
| attacking.stretch_m | 9.45 | 9.73 | **9.59** |
| attacking.width_m | 18.95 | 19.34 | **19.15** |
| defending.anchor_dist_m.defender | 9.68 | 10.42 | **10.05** |
| defending.anchor_dist_m.forward | 15.61 | 14.34 | **14.98** |
| defending.anchor_dist_m.goalkeeper | 1.37 | 1.33 | **1.35** |
| defending.anchor_dist_m.midfielder | 13.39 | 13.44 | **13.41** |
| defending.anchor_dist_shape_m.defender | 9.50 | 9.10 | **9.30** |
| defending.anchor_dist_shape_m.forward | 10.95 | 10.40 | **10.68** |
| defending.anchor_dist_shape_m.midfielder | 8.11 | 7.38 | **7.74** |
| defending.bbox_area_m2 | 317.2 | 338.2 | **327.7** |
| defending.bbox_depth_m | 15.50 | 16.83 | **16.17** |
| defending.bbox_width_m | 19.21 | 18.84 | **19.03** |
| defending.length_m | 15.50 | 16.83 | **16.17** |
| defending.line_x_std_m | 5.28 | 5.54 | **5.41** |
| defending.line_x_variance_m2 | 43.77 | 48.25 | **46.01** |
| defending.nearest_mate_mean_m | 10.16 | 10.49 | **10.32** |
| defending.nearest_mate_p10_m | 3.63 | 3.77 | **3.70** |
| defending.pct_of_open_play | 50.45 | 49.55 | **50** |
| defending.pct_outfield_behind_ball | 60.58 | 62.22 | **61.40** |
| defending.stretch_m | 9.04 | 9.34 | **9.19** |
| defending.swarm2_pct | 2.45 | 1.33 | **1.89** |
| defending.swarm3_pct | 0.03 | 0 | **0.02** |
| defending.width_m | 19.21 | 18.84 | **19.03** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 11.33 | 11.36 | **11.34** |
| accel_gt10_pct | 51.67 | 52.14 | **51.91** |
| accel_mean_mps2 | 11.28 | 11.31 | **11.29** |
| dspeed_max_mps_per_tick | 7.45 | 7.84 | **7.64** |
| heading_change_p95_deg_per_tick_gt3mps | 3.50 | 3.49 | **3.50** |
| heading_change_pct.0_10 | 99.18 | 99.28 | **99.23** |
| heading_change_pct.10_45 | 0.75 | 0.65 | **0.70** |
| heading_change_pct.45_90 | 0.04 | 0.04 | **0.04** |
| heading_change_pct.gt90 | 0.02 | 0.03 | **0.02** |
| lateral_accel_mean_mps2 | 7.96 | 8.10 | **8.03** |
| reversals_per_player_min | 0.70 | 0.76 | **0.73** |
| side_flips_per_player_min | 70.18 | 73.20 | **71.69** |
| speed_oscillation_pct | 4.22 | 4.26 | **4.24** |
| turn_rate_mean_deg_per_s | 102.2 | 101.3 | **101.8** |

### Collisions and push-out

| metric | home | away | teams mean |
|---|---:|---:|---:|
| any.depth_max_m | 0.79 | 0.79 | **0.79** |
| any.depth_mean_m | 0.00 | 0.00 | **0.00** |
| any.events_per_min | 56.67 | 56.67 | **56.67** |
| any.stuck_mean_ticks | 22.29 | 22.29 | **22.29** |
| any.stuck_pairs | 32 | 32 | **32** |
| any.ticks_gt1cm_per_min | 0.50 | 0.50 | **0.50** |
| any.ticks_per_min | 161.3 | 161.3 | **161.3** |
| correction_vel_corrected_p50_mps | 0.17 | 0.17 | **0.17** |
| correction_vel_corrected_p99_mps | 7.31 | 7.52 | **7.42** |
| correction_vel_corrected_pct | 1.22 | 1.20 | **1.21** |
| correction_vel_max_mps | 23.18 | 23.04 | **23.11** |
| correction_vel_p50_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p95_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p99_mps | 0.15 | 0.12 | **0.13** |
| correction_vel_pct_gt_0_3 | 0.25 | 0.25 | **0.25** |
| opponent.depth_max_m | 0.02 | 0.02 | **0.02** |
| opponent.depth_mean_m | 0.00 | 0.00 | **0.00** |
| opponent.events_per_min | 44.47 | 44.47 | **44.47** |
| opponent.stuck_mean_ticks | 21.90 | 21.90 | **21.90** |
| opponent.stuck_pairs | 26.67 | 26.67 | **26.67** |
| opponent.ticks_gt1cm_per_min | 0.17 | 0.17 | **0.17** |
| opponent.ticks_per_min | 134.3 | 134.3 | **134.3** |
| teammate.depth_max_m | 0.78 | 0.76 | **0.77** |
| teammate.depth_mean_m | 0.01 | 0.01 | **0.01** |
| teammate.events_per_min | 5.90 | 6.30 | **6.10** |
| teammate.stuck_mean_ticks | 25.67 | 15.43 | **20.55** |
| teammate.stuck_pairs | 3 | 2.33 | **2.67** |
| teammate.ticks_gt1cm_per_min | 0.33 | 0.23 | **0.28** |
| teammate.ticks_per_min | 16.47 | 15.07 | **15.77** |

### Coordination

| metric | home | away | teams mean |
|---|---:|---:|---:|
| cruise_over72_pct | 38.76 | 38.95 | **38.85** |
| dual_chaser_pct_of_loose | 23.74 | 21.61 | **22.67** |
| dual_chaser_ticks_per_min | 526.6 | 479.3 | **503.0** |
| engagers3_pct_of_carrier_ticks | 0.28 | 0.21 | **0.24** |
| engagers3_ticks_per_min | 2.10 | 1.33 | **1.72** |
| max_simultaneous_chasers | 4.67 | 4.33 | **4.50** |

### Aerial

| metric | home | away | teams mean |
|---|---:|---:|---:|
| headers | 17 | 16.67 | **16.83** |
| headers_clear | 0.33 | 0.67 | **0.50** |
| headers_pass | 14.67 | 13.33 | **14** |
| headers_shot | 2 | 2.67 | **2.33** |
| jumps_gk | 22 | 15 | **18.50** |
| jumps_outfield | 35 | 31.67 | **33.33** |
| max_airborne_outfield | 1 | 1 | **1** |

### Team shape vs envelope (teams mean)

| metric | in possession | out of possession | 11v11 envelope (m) |
|---|---:|---:|---|
| length_m | 17.68 | 16.17 | 31-46 |
| width_m | 19.15 | 19.03 | 35-48 |
| stretch_m | 9.59 | 9.19 | 7-16 |
| nearest_mate_mean_m | 10.62 | 10.32 | none published (in-possession mean near 9-10 m, estimate) |
| nearest_mate_p10_m | 3.12 | 3.70 | none published (in-possession mean near 9-10 m, estimate) |

Note: 5v5 has no published team-shape envelope; the 11v11 ranges are printed for reference only and do not apply to a 4-outfielder team on a small pitch.

### Per seed (home / away)

| metric | seed 1 | seed 2 | seed 3 |
|---|---:|---:|---:|
| aerial.headers | 20 / 15 | 17 / 14 | 14 / 21 |
| aerial.headers_clear | 1 / 0 | 0 / 1 | 0 / 1 |
| aerial.headers_pass | 16 / 12 | 14 / 11 | 14 / 17 |
| aerial.headers_shot | 3 / 3 | 3 / 2 | 0 / 3 |
| aerial.jumps_gk | 22 / 13 | 21 / 19 | 23 / 13 |
| aerial.jumps_outfield | 39 / 35 | 37 / 26 | 29 / 34 |
| aerial.max_airborne_outfield | 1 / 1 | 1 / 1 | 1 / 1 |
| collision.any.depth_max_m | 0.79 / 0.79 | 0.79 / 0.79 | 0.79 / 0.79 |
| collision.any.depth_mean_m | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.any.events_per_min | 54.90 / 54.90 | 57.20 / 57.20 | 57.90 / 57.90 |
| collision.any.stuck_mean_ticks | 24.52 / 24.52 | 21.13 / 21.13 | 21.21 / 21.21 |
| collision.any.stuck_pairs | 29 / 29 | 39 / 39 | 28 / 28 |
| collision.any.ticks_gt1cm_per_min | 0.20 / 0.20 | 0.50 / 0.50 | 0.80 / 0.80 |
| collision.any.ticks_per_min | 155.1 / 155.1 | 177.4 / 177.4 | 151.5 / 151.5 |
| collision.correction_vel_corrected_p50_mps | 0.16 / 0.17 | 0.17 / 0.16 | 0.16 / 0.17 |
| collision.correction_vel_corrected_p99_mps | 5.30 / 7.25 | 6.09 / 6.71 | 10.55 / 8.61 |
| collision.correction_vel_corrected_pct | 1.21 / 1.14 | 1.28 / 1.41 | 1.17 / 1.05 |
| collision.correction_vel_max_mps | 24.00 / 20.37 | 22.63 / 24.47 | 22.91 / 24.29 |
| collision.correction_vel_p50_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p95_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p99_mps | 0.15 / 0.15 | 0.15 / 0.15 | 0.15 / 0.05 |
| collision.correction_vel_pct_gt_0_3 | 0.22 / 0.23 | 0.24 / 0.26 | 0.28 / 0.25 |
| collision.opponent.depth_max_m | 0.01 / 0.01 | 0.01 / 0.01 | 0.04 / 0.04 |
| collision.opponent.depth_mean_m | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.opponent.events_per_min | 44.10 / 44.10 | 45.20 / 45.20 | 44.10 / 44.10 |
| collision.opponent.stuck_mean_ticks | 23.17 / 23.17 | 22.06 / 22.06 | 20.48 / 20.48 |
| collision.opponent.stuck_pairs | 24 / 24 | 31 / 31 | 25 / 25 |
| collision.opponent.ticks_gt1cm_per_min | 0 / 0 | 0 / 0 | 0.50 / 0.50 |
| collision.opponent.ticks_per_min | 127.1 / 127.1 | 151 / 151 | 124.8 / 124.8 |
| collision.teammate.depth_max_m | 0.79 / 0.71 | 0.77 / 0.79 | 0.77 / 0.79 |
| collision.teammate.depth_mean_m | 0.00 / 0.01 | 0.02 / 0.01 | 0.01 / 0.02 |
| collision.teammate.events_per_min | 4.90 / 5.90 | 4.80 / 7.20 | 8 / 5.80 |
| collision.teammate.stuck_mean_ticks | 32.67 / 28.50 | 17 / 17.80 | 27.33 / 0 |
| collision.teammate.stuck_pairs | 3 / 2 | 3 / 5 | 3 / 0 |
| collision.teammate.ticks_gt1cm_per_min | 0.20 / 0.10 | 0.50 / 0.30 | 0.30 / 0.30 |
| collision.teammate.ticks_per_min | 16.60 / 13.60 | 13.20 / 22.30 | 19.60 / 9.30 |
| coord.cruise_over72_pct | 37.79 / 38.51 | 39.48 / 38.87 | 39.00 / 39.46 |
| coord.dual_chaser_pct_of_loose | 23.94 / 22.45 | 21.71 / 21.28 | 25.56 / 21.09 |
| coord.dual_chaser_ticks_per_min | 538.4 / 504.9 | 476.7 / 467.2 | 564.8 / 465.9 |
| coord.engagers3_pct_of_carrier_ticks | 0 / 0.37 | 0.57 / 0 | 0.28 / 0.25 |
| coord.engagers3_ticks_per_min | 0 / 2.40 | 4.20 / 0 | 2.10 / 1.60 |
| coord.max_simultaneous_chasers | 5 / 5 | 4 / 4 | 5 / 4 |
| gk.catches | 3 / 2 | 0 / 1 | 2 / 0 |
| gk.claims_non_save | 3 / 1 | 1 / 0 | 3 / 0 |
| gk.excursion_mean_s | 0 / 0 | 0 / 0 | 1.25 / 0 |
| gk.excursions_gt1s_per_10min | 0 / 0 | 0 / 0 | 1 / 0 |
| gk.gathers | 6 / 3 | 1 / 1 | 5 / 0 |
| gk.goals_conceded | 3 / 3 | 7 / 7 | 6 / 4 |
| gk.goals_conceded_gk_gt3m_off_line | 2 / 2 | 5 / 5 | 4 / 3 |
| gk.goals_conceded_no_shot_found | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.line_dist_at_conceded_shot_mean_m | 2.35 / 3.02 | 2.62 / 3.00 | 2.79 / 2.47 |
| gk.line_dist_max_m | 5.40 / 4.61 | 4.74 / 5.35 | 8.35 / 5.22 |
| gk.line_dist_mean_m | 2.98 / 3.06 | 3.01 / 3.01 | 3.03 / 3.04 |
| gk.parries | 18 / 5 | 13 / 10 | 15 / 11 |
| gk.pct_outside_6yd | 0 / 0 | 0 / 0 | 0.21 / 0 |
| gk.pct_outside_box | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_count | 0 / 0 | 0 / 0 | 1 / 0 |
| gk.rush_per_10min | 0 / 0 | 0 / 0 | 1 / 0 |
| gk.saves | 21 / 7 | 13 / 11 | 17 / 11 |
| gk.tips_over | 0 / 0 | 0 / 0 | 0 / 0 |
| match.fouls | 0 / 0 | 0 / 0 | 1 / 0 |
| match.goals | 3 / 3 | 7 / 7 | 4 / 6 |
| match.offsides | 0 / 3 | 2 / 1 | 1 / 1 |
| match.passes_completed_pct | 85.26 / 87.17 | 84.66 / 87.86 | 85.56 / 86.32 |
| match.possession_pct | 48.18 / 51.82 | 47.28 / 52.72 | 46.46 / 53.54 |
| match.shots | 16 / 25 | 23 / 27 | 15 / 30 |
| match.shots_on_target | 10 / 23 | 18 / 20 | 15 / 22 |
| move.accel_from_pos_mean_mps2 | 11.16 / 11.18 | 11.11 / 11.34 | 11.71 / 11.55 |
| move.accel_gt10_pct | 51.38 / 51.86 | 50.56 / 51.51 | 53.08 / 53.05 |
| move.accel_mean_mps2 | 11.15 / 11.17 | 11.08 / 11.28 | 11.60 / 11.49 |
| move.dspeed_max_mps_per_tick | 8.09 / 7.43 | 6.64 / 7.95 | 7.62 / 8.12 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.52 / 3.46 | 3.46 / 3.50 | 3.52 / 3.52 |
| move.heading_change_pct.0_10 | 99.26 / 99.28 | 99.19 / 99.30 | 99.10 / 99.28 |
| move.heading_change_pct.10_45 | 0.69 / 0.66 | 0.74 / 0.63 | 0.82 / 0.66 |
| move.heading_change_pct.45_90 | 0.04 / 0.04 | 0.04 / 0.04 | 0.05 / 0.04 |
| move.heading_change_pct.gt90 | 0.01 / 0.02 | 0.02 / 0.03 | 0.03 / 0.02 |
| move.lateral_accel_mean_mps2 | 7.90 / 8.12 | 7.81 / 7.91 | 8.17 / 8.26 |
| move.reversals_per_player_min | 0.42 / 0.74 | 0.74 / 0.94 | 0.94 / 0.60 |
| move.side_flips_per_player_min | 71.53 / 72.51 | 66.98 / 72.55 | 72.02 / 74.55 |
| move.speed_oscillation_pct | 4.18 / 4.28 | 4.20 / 4.24 | 4.28 / 4.26 |
| move.turn_rate_mean_deg_per_s | 101.1 / 100.0 | 99.97 / 100.5 | 105.6 / 103.4 |
| shape.attacking.anchor_dist_m.defender | 8.02 / 9.77 | 9.39 / 8.18 | 8.27 / 9.75 |
| shape.attacking.anchor_dist_m.forward | 14.87 / 13.43 | 14.76 / 14.79 | 15.89 / 13.62 |
| shape.attacking.anchor_dist_m.goalkeeper | 1.26 / 1.12 | 1.20 / 1.22 | 1.24 / 1.19 |
| shape.attacking.anchor_dist_m.midfielder | 13.84 / 15.99 | 15.12 / 13.39 | 13.87 / 13.42 |
| shape.attacking.anchor_dist_shape_m.defender | 6.49 / 6.81 | 7.07 / 6.64 | 6.85 / 7.01 |
| shape.attacking.anchor_dist_shape_m.forward | 10.06 / 10.86 | 11.10 / 10.72 | 11.26 / 11.29 |
| shape.attacking.anchor_dist_shape_m.midfielder | 7.57 / 10.05 | 9.17 / 7.53 | 8.07 / 7.71 |
| shape.attacking.bbox_area_m2 | 372.6 / 379.5 | 337.3 / 363.6 | 332.8 / 374.9 |
| shape.attacking.bbox_depth_m | 18.24 / 18.29 | 16.86 / 17.85 | 16.79 / 18.05 |
| shape.attacking.bbox_width_m | 19.10 / 19.33 | 19.37 / 18.95 | 18.39 / 19.75 |
| shape.attacking.length_m | 18.24 / 18.29 | 16.86 / 17.85 | 16.79 / 18.05 |
| shape.attacking.nearest_mate_mean_m | 10.84 / 10.86 | 10.36 / 10.63 | 10.13 / 10.93 |
| shape.attacking.nearest_mate_p10_m | 3.40 / 3.01 | 2.92 / 3.05 | 2.97 / 3.37 |
| shape.attacking.pct_of_open_play | 50.39 / 49.61 | 50.19 / 49.81 | 48.07 / 51.93 |
| shape.attacking.stretch_m | 9.76 / 9.78 | 9.44 / 9.56 | 9.14 / 9.85 |
| shape.attacking.width_m | 19.10 / 19.33 | 19.37 / 18.95 | 18.39 / 19.75 |
| shape.defending.anchor_dist_m.defender | 9.29 / 10.99 | 9.78 / 9.90 | 9.97 / 10.35 |
| shape.defending.anchor_dist_m.forward | 16.10 / 13.94 | 15.24 / 14.84 | 15.50 / 14.26 |
| shape.defending.anchor_dist_m.goalkeeper | 1.39 / 1.29 | 1.32 / 1.36 | 1.39 / 1.33 |
| shape.defending.anchor_dist_m.midfielder | 12.55 / 14.92 | 14.91 / 12.12 | 12.71 / 13.26 |
| shape.defending.anchor_dist_shape_m.defender | 9.39 / 8.90 | 9.60 / 9.55 | 9.50 / 8.84 |
| shape.defending.anchor_dist_shape_m.forward | 10.89 / 10.59 | 10.69 / 10.12 | 11.28 / 10.50 |
| shape.defending.anchor_dist_shape_m.midfielder | 7.36 / 8.62 | 9.23 / 6.40 | 7.73 / 7.12 |
| shape.defending.bbox_area_m2 | 332.0 / 336.2 | 307.0 / 331.1 | 312.7 / 347.4 |
| shape.defending.bbox_depth_m | 15.50 / 17.38 | 15.99 / 16.11 | 15.02 / 17.01 |
| shape.defending.bbox_width_m | 19.91 / 18.30 | 18.20 / 19.19 | 19.53 / 19.03 |
| shape.defending.length_m | 15.50 / 17.38 | 15.99 / 16.11 | 15.02 / 17.01 |
| shape.defending.line_x_std_m | 5.12 / 5.84 | 5.58 / 4.96 | 5.15 / 5.83 |
| shape.defending.line_x_variance_m2 | 41.18 / 54.15 | 49.30 / 39.21 | 40.83 / 51.40 |
| shape.defending.nearest_mate_mean_m | 10.40 / 10.54 | 9.96 / 10.32 | 10.10 / 10.62 |
| shape.defending.nearest_mate_p10_m | 3.92 / 3.90 | 3.39 / 3.83 | 3.58 / 3.59 |
| shape.defending.pct_of_open_play | 49.61 / 50.39 | 49.81 / 50.19 | 51.93 / 48.07 |
| shape.defending.pct_outfield_behind_ball | 60.21 / 62.76 | 61.98 / 60.57 | 59.54 / 63.32 |
| shape.defending.stretch_m | 9.18 / 9.37 | 8.88 / 9.23 | 9.05 / 9.42 |
| shape.defending.swarm2_pct | 1.84 / 0.57 | 2.04 / 1.82 | 3.47 / 1.61 |
| shape.defending.swarm3_pct | 0 / 0 | 0.09 / 0 | 0 / 0 |
| shape.defending.width_m | 19.91 / 18.30 | 18.20 / 19.19 | 19.53 / 19.03 |

GK scenario field: not present in the public state.

## 5v5 normal vs normal (10 min x 3 seeds from 1)

Command: `psfc-replay analyze --format 5v5 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty normal --away-difficulty normal`

Gameplay version: `0.1.0-5v5`

| seed | final state_hash | score (H-A) |
|---|---|---|
| 1 | `70e848358d7a8477` | 5-5 |
| 2 | `68b5bb3b8a2cdebb` | 3-4 |
| 3 | `075dc1cbc22cffe8` | 7-4 |

Match meta (mean over seeds): goals_total = 9.33, open_play_minutes = 10.00, open_play_pct_of_ticks = 95.19, shots_total = 46.67, ticks_simulated = 37835.3

### Match

| metric | home | away | teams mean |
|---|---:|---:|---:|
| fouls | 0 | 0 | **0** |
| goals | 5 | 4.33 | **4.67** |
| offsides | 0.33 | 1 | **0.67** |
| passes_completed_pct | 85.10 | 84.63 | **84.86** |
| possession_pct | 51.68 | 48.32 | **50** |
| shots | 26 | 20.67 | **23.33** |
| shots_on_target | 19.33 | 16 | **17.67** |

### Goalkeeper

| metric | home | away | teams mean |
|---|---:|---:|---:|
| catches | 3.67 | 1 | **2.33** |
| claims_non_save | 0.67 | 2 | **1.33** |
| excursion_mean_s | 0.23 | 0.12 | **0.18** |
| excursions_gt1s_per_10min | 0.67 | 0 | **0.33** |
| gathers | 4.33 | 3 | **3.67** |
| goals_conceded | 4.33 | 5 | **4.67** |
| goals_conceded_gk_gt3m_off_line | 3.33 | 3 | **3.17** |
| goals_conceded_no_shot_found | 0 | 0 | **0** |
| line_dist_at_conceded_shot_mean_m | 3.08 | 2.48 | **2.78** |
| line_dist_max_m | 6.53 | 5.42 | **5.98** |
| line_dist_mean_m | 3.02 | 3.03 | **3.03** |
| parries | 9.67 | 13.33 | **11.50** |
| pct_outside_6yd | 0.15 | 0.02 | **0.09** |
| pct_outside_box | 0 | 0 | **0** |
| rush_count | 1.33 | 0 | **0.67** |
| rush_per_10min | 1.33 | 0 | **0.67** |
| saves | 13.33 | 14.33 | **13.83** |
| tips_over | 0 | 0 | **0** |

### Defence and shape

| metric | home | away | teams mean |
|---|---:|---:|---:|
| attacking.anchor_dist_m.defender | 8.87 | 8.75 | **8.81** |
| attacking.anchor_dist_m.forward | 14.47 | 13.94 | **14.21** |
| attacking.anchor_dist_m.goalkeeper | 1.20 | 1.17 | **1.18** |
| attacking.anchor_dist_m.midfielder | 14.65 | 15.74 | **15.20** |
| attacking.anchor_dist_shape_m.defender | 6.68 | 6.53 | **6.60** |
| attacking.anchor_dist_shape_m.forward | 10.61 | 10.33 | **10.47** |
| attacking.anchor_dist_shape_m.midfielder | 8.81 | 10.05 | **9.43** |
| attacking.bbox_area_m2 | 351.7 | 373.3 | **362.5** |
| attacking.bbox_depth_m | 17.77 | 17.76 | **17.77** |
| attacking.bbox_width_m | 18.75 | 19.78 | **19.26** |
| attacking.length_m | 17.77 | 17.76 | **17.77** |
| attacking.nearest_mate_mean_m | 10.52 | 10.88 | **10.70** |
| attacking.nearest_mate_p10_m | 3.18 | 3.48 | **3.33** |
| attacking.pct_of_open_play | 49.46 | 50.54 | **50** |
| attacking.stretch_m | 9.55 | 9.77 | **9.66** |
| attacking.width_m | 18.75 | 19.78 | **19.26** |
| defending.anchor_dist_m.defender | 9.32 | 9.75 | **9.53** |
| defending.anchor_dist_m.forward | 15.09 | 14.69 | **14.89** |
| defending.anchor_dist_m.goalkeeper | 1.36 | 1.34 | **1.35** |
| defending.anchor_dist_m.midfielder | 13.92 | 14.62 | **14.27** |
| defending.anchor_dist_shape_m.defender | 8.95 | 9.16 | **9.06** |
| defending.anchor_dist_shape_m.forward | 10.34 | 10.33 | **10.34** |
| defending.anchor_dist_shape_m.midfielder | 8.50 | 9.40 | **8.95** |
| defending.bbox_area_m2 | 325.8 | 325.6 | **325.7** |
| defending.bbox_depth_m | 16.01 | 15.97 | **15.99** |
| defending.bbox_width_m | 18.89 | 18.97 | **18.93** |
| defending.length_m | 16.01 | 15.97 | **15.99** |
| defending.line_x_std_m | 5.27 | 5.11 | **5.19** |
| defending.line_x_variance_m2 | 42.98 | 40.47 | **41.72** |
| defending.nearest_mate_mean_m | 10.34 | 10.33 | **10.33** |
| defending.nearest_mate_p10_m | 3.91 | 3.81 | **3.86** |
| defending.pct_of_open_play | 50.54 | 49.46 | **50** |
| defending.pct_outfield_behind_ball | 62.47 | 62.76 | **62.62** |
| defending.stretch_m | 9.12 | 9.13 | **9.12** |
| defending.swarm2_pct | 1.57 | 1.67 | **1.62** |
| defending.swarm3_pct | 0.02 | 0.03 | **0.03** |
| defending.width_m | 18.89 | 18.97 | **18.93** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 11.13 | 11.09 | **11.11** |
| accel_gt10_pct | 51.17 | 50.90 | **51.04** |
| accel_mean_mps2 | 11.11 | 11.08 | **11.09** |
| dspeed_max_mps_per_tick | 7.76 | 7.71 | **7.73** |
| heading_change_p95_deg_per_tick_gt3mps | 3.46 | 3.49 | **3.48** |
| heading_change_pct.0_10 | 99.26 | 99.25 | **99.25** |
| heading_change_pct.10_45 | 0.68 | 0.70 | **0.69** |
| heading_change_pct.45_90 | 0.04 | 0.03 | **0.04** |
| heading_change_pct.gt90 | 0.02 | 0.02 | **0.02** |
| lateral_accel_mean_mps2 | 7.88 | 7.96 | **7.92** |
| reversals_per_player_min | 0.63 | 0.65 | **0.64** |
| side_flips_per_player_min | 69.23 | 71.00 | **70.12** |
| speed_oscillation_pct | 4.10 | 4.11 | **4.10** |
| turn_rate_mean_deg_per_s | 99.65 | 100.2 | **99.90** |

### Collisions and push-out

| metric | home | away | teams mean |
|---|---:|---:|---:|
| any.depth_max_m | 0.52 | 0.52 | **0.52** |
| any.depth_mean_m | 0.00 | 0.00 | **0.00** |
| any.events_per_min | 47.78 | 47.78 | **47.78** |
| any.stuck_mean_ticks | 22.16 | 22.16 | **22.16** |
| any.stuck_pairs | 27.67 | 27.67 | **27.67** |
| any.ticks_gt1cm_per_min | 0.17 | 0.17 | **0.17** |
| any.ticks_per_min | 136.7 | 136.7 | **136.7** |
| correction_vel_corrected_p50_mps | 0.17 | 0.16 | **0.17** |
| correction_vel_corrected_p99_mps | 6.62 | 6.62 | **6.62** |
| correction_vel_corrected_pct | 1.01 | 1.01 | **1.01** |
| correction_vel_max_mps | 19.86 | 19.44 | **19.65** |
| correction_vel_p50_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p95_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p99_mps | 0.05 | 0.05 | **0.05** |
| correction_vel_pct_gt_0_3 | 0.22 | 0.21 | **0.21** |
| opponent.depth_max_m | 0.01 | 0.01 | **0.01** |
| opponent.depth_mean_m | 0.00 | 0.00 | **0.00** |
| opponent.events_per_min | 36.58 | 36.58 | **36.58** |
| opponent.stuck_mean_ticks | 22.19 | 22.19 | **22.19** |
| opponent.stuck_pairs | 22.33 | 22.33 | **22.33** |
| opponent.ticks_gt1cm_per_min | 0.10 | 0.10 | **0.10** |
| opponent.ticks_per_min | 110.6 | 110.6 | **110.6** |
| teammate.depth_max_m | 0.50 | 0.50 | **0.50** |
| teammate.depth_mean_m | 0.00 | 0.00 | **0.00** |
| teammate.events_per_min | 5.90 | 5.30 | **5.60** |
| teammate.stuck_mean_ticks | 19.92 | 19.56 | **19.74** |
| teammate.stuck_pairs | 2.33 | 3 | **2.67** |
| teammate.ticks_gt1cm_per_min | 0.07 | 0.07 | **0.07** |
| teammate.ticks_per_min | 13.23 | 13.43 | **13.33** |

### Coordination

| metric | home | away | teams mean |
|---|---:|---:|---:|
| cruise_over72_pct | 37.92 | 37.12 | **37.52** |
| dual_chaser_pct_of_loose | 22.17 | 21.24 | **21.70** |
| dual_chaser_ticks_per_min | 469.1 | 448.1 | **458.6** |
| engagers3_pct_of_carrier_ticks | 0.15 | 0.20 | **0.18** |
| engagers3_ticks_per_min | 1.03 | 1.57 | **1.30** |
| max_simultaneous_chasers | 4 | 4.33 | **4.17** |

### Aerial

| metric | home | away | teams mean |
|---|---:|---:|---:|
| headers | 16.67 | 19 | **17.83** |
| headers_clear | 1.67 | 1 | **1.33** |
| headers_pass | 12.67 | 15 | **13.83** |
| headers_shot | 2.33 | 3 | **2.67** |
| jumps_gk | 17 | 20.67 | **18.83** |
| jumps_outfield | 33.67 | 34.67 | **34.17** |
| max_airborne_outfield | 1 | 1 | **1** |

### Team shape vs envelope (teams mean)

| metric | in possession | out of possession | 11v11 envelope (m) |
|---|---:|---:|---|
| length_m | 17.77 | 15.99 | 31-46 |
| width_m | 19.26 | 18.93 | 35-48 |
| stretch_m | 9.66 | 9.12 | 7-16 |
| nearest_mate_mean_m | 10.70 | 10.33 | none published (in-possession mean near 9-10 m, estimate) |
| nearest_mate_p10_m | 3.33 | 3.86 | none published (in-possession mean near 9-10 m, estimate) |

Note: 5v5 has no published team-shape envelope; the 11v11 ranges are printed for reference only and do not apply to a 4-outfielder team on a small pitch.

### Per seed (home / away)

| metric | seed 1 | seed 2 | seed 3 |
|---|---:|---:|---:|
| aerial.headers | 15 / 16 | 20 / 20 | 15 / 21 |
| aerial.headers_clear | 2 / 0 | 1 / 1 | 2 / 2 |
| aerial.headers_pass | 10 / 13 | 17 / 14 | 11 / 18 |
| aerial.headers_shot | 3 / 3 | 2 / 5 | 2 / 1 |
| aerial.jumps_gk | 18 / 20 | 22 / 16 | 11 / 26 |
| aerial.jumps_outfield | 29 / 27 | 37 / 35 | 35 / 42 |
| aerial.max_airborne_outfield | 1 / 1 | 1 / 1 | 1 / 1 |
| collision.any.depth_max_m | 0.01 / 0.01 | 0.75 / 0.75 | 0.79 / 0.79 |
| collision.any.depth_mean_m | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.any.events_per_min | 48.74 / 48.74 | 46.80 / 46.80 | 47.80 / 47.80 |
| collision.any.stuck_mean_ticks | 30.37 / 30.37 | 17.35 / 17.35 | 18.77 / 18.77 |
| collision.any.stuck_pairs | 30 / 30 | 31 / 31 | 22 / 22 |
| collision.any.ticks_gt1cm_per_min | 0 / 0 | 0.10 / 0.10 | 0.40 / 0.40 |
| collision.any.ticks_per_min | 161.0 / 161.0 | 125.8 / 125.8 | 123.2 / 123.2 |
| collision.correction_vel_corrected_p50_mps | 0.17 / 0.16 | 0.16 / 0.17 | 0.17 / 0.16 |
| collision.correction_vel_corrected_p99_mps | 6.70 / 7.28 | 5.71 / 5.26 | 7.45 / 7.33 |
| collision.correction_vel_corrected_pct | 1.19 / 1.18 | 0.91 / 0.99 | 0.91 / 0.86 |
| collision.correction_vel_max_mps | 15.40 / 13.41 | 22.76 / 20.44 | 21.41 / 24.47 |
| collision.correction_vel_p50_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p95_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p99_mps | 0.15 / 0.15 | 0.00 / 0.01 | 0.00 / 0.00 |
| collision.correction_vel_pct_gt_0_3 | 0.23 / 0.21 | 0.22 / 0.19 | 0.22 / 0.22 |
| collision.opponent.depth_max_m | 0.01 / 0.01 | 0.01 / 0.01 | 0.03 / 0.03 |
| collision.opponent.depth_mean_m | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.opponent.events_per_min | 38.75 / 38.75 | 33.40 / 33.40 | 37.60 / 37.60 |
| collision.opponent.stuck_mean_ticks | 31.15 / 31.15 | 18.36 / 18.36 | 17.07 / 17.07 |
| collision.opponent.stuck_pairs | 27 / 27 | 25 / 25 | 15 / 15 |
| collision.opponent.ticks_gt1cm_per_min | 0 / 0 | 0 / 0 | 0.30 / 0.30 |
| collision.opponent.ticks_per_min | 141.2 / 141.2 | 96.80 / 96.80 | 93.90 / 93.90 |
| collision.teammate.depth_max_m | 0.01 / 0.00 | 0.75 / 0.71 | 0.74 / 0.79 |
| collision.teammate.depth_mean_m | 0.00 / 0.00 | 0.01 / 0.00 | 0.00 / 0.01 |
| collision.teammate.events_per_min | 5.99 / 3.99 | 6.20 / 7.20 | 5.50 / 4.70 |
| collision.teammate.stuck_mean_ticks | 24 / 23 | 13.50 / 13 | 22.25 / 22.67 |
| collision.teammate.stuck_pairs | 1 / 2 | 2 / 4 | 4 / 3 |
| collision.teammate.ticks_gt1cm_per_min | 0 / 0 | 0.10 / 0.10 | 0.10 / 0.10 |
| collision.teammate.ticks_per_min | 10.29 / 10.19 | 12.70 / 17 | 16.70 / 13.10 |
| coord.cruise_over72_pct | 38.26 / 37.42 | 41.07 / 37.96 | 34.44 / 35.97 |
| coord.dual_chaser_pct_of_loose | 21.03 / 22.73 | 26.40 / 18.21 | 19.07 / 22.78 |
| coord.dual_chaser_ticks_per_min | 433.8 / 468.9 | 568.9 / 392.4 | 404.5 / 483.1 |
| coord.engagers3_pct_of_carrier_ticks | 0 / 0.40 | 0 / 0.13 | 0.46 / 0.07 |
| coord.engagers3_ticks_per_min | 0 / 3.20 | 0 / 0.90 | 3.10 / 0.60 |
| coord.max_simultaneous_chasers | 4 / 4 | 4 / 4 | 4 / 5 |
| gk.catches | 4 / 3 | 5 / 0 | 2 / 0 |
| gk.claims_non_save | 2 / 2 | 0 / 1 | 0 / 3 |
| gk.excursion_mean_s | 0.70 / 0 | 0 / 0 | 0 / 0.37 |
| gk.excursions_gt1s_per_10min | 2.00 / 0 | 0 / 0 | 0 / 0 |
| gk.gathers | 6 / 5 | 5 / 1 | 2 / 3 |
| gk.goals_conceded | 5 / 5 | 4 / 3 | 4 / 7 |
| gk.goals_conceded_gk_gt3m_off_line | 4 / 4 | 3 / 1 | 3 / 4 |
| gk.goals_conceded_no_shot_found | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.line_dist_at_conceded_shot_mean_m | 3.07 / 2.76 | 3.06 / 2.04 | 3.10 / 2.64 |
| gk.line_dist_max_m | 9.09 / 5.24 | 5.30 / 5.17 | 5.22 / 5.86 |
| gk.line_dist_mean_m | 3.04 / 3.03 | 2.94 / 3.06 | 3.08 / 3.01 |
| gk.parries | 11 / 12 | 12 / 12 | 6 / 16 |
| gk.pct_outside_6yd | 0.46 / 0 | 0 / 0 | 0 / 0.06 |
| gk.pct_outside_box | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_count | 4 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_per_10min | 3.99 / 0 | 0 / 0 | 0 / 0 |
| gk.saves | 15 / 15 | 17 / 12 | 8 / 16 |
| gk.tips_over | 0 / 0 | 0 / 0 | 0 / 0 |
| match.fouls | 0 / 0 | 0 / 0 | 0 / 0 |
| match.goals | 5 / 5 | 3 / 4 | 7 / 4 |
| match.offsides | 0 / 1 | 0 / 0 | 1 / 2 |
| match.passes_completed_pct | 87.36 / 84.85 | 81.98 / 86.93 | 85.96 / 82.11 |
| match.possession_pct | 52.57 / 47.43 | 48.24 / 51.76 | 54.23 / 45.77 |
| match.shots | 26 / 21 | 20 / 28 | 32 / 13 |
| match.shots_on_target | 20 / 17 | 15 / 20 | 23 / 11 |
| move.accel_from_pos_mean_mps2 | 11.01 / 10.98 | 11.37 / 10.87 | 11.02 / 11.42 |
| move.accel_gt10_pct | 50.27 / 50.25 | 52.81 / 50.27 | 50.43 / 52.18 |
| move.accel_mean_mps2 | 11.00 / 10.96 | 11.36 / 10.85 | 10.99 / 11.42 |
| move.dspeed_max_mps_per_tick | 8.02 / 7.61 | 7.50 / 7.73 | 7.76 / 7.78 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.44 / 3.47 | 3.47 / 3.48 | 3.47 / 3.53 |
| move.heading_change_pct.0_10 | 99.28 / 99.26 | 99.28 / 99.33 | 99.22 / 99.15 |
| move.heading_change_pct.10_45 | 0.65 / 0.68 | 0.67 / 0.63 | 0.71 / 0.78 |
| move.heading_change_pct.45_90 | 0.04 / 0.04 | 0.03 / 0.03 | 0.05 / 0.04 |
| move.heading_change_pct.gt90 | 0.02 / 0.02 | 0.02 / 0.01 | 0.02 / 0.03 |
| move.lateral_accel_mean_mps2 | 7.75 / 7.85 | 7.97 / 7.97 | 7.93 / 8.06 |
| move.reversals_per_player_min | 0.64 / 0.66 | 0.54 / 0.44 | 0.70 / 0.84 |
| move.side_flips_per_player_min | 67.19 / 69.57 | 71.97 / 71.73 | 68.54 / 71.72 |
| move.speed_oscillation_pct | 4.15 / 4.16 | 4.15 / 4.08 | 3.99 / 4.08 |
| move.turn_rate_mean_deg_per_s | 97.70 / 99.46 | 99.29 / 97.70 | 102.0 / 103.3 |
| shape.attacking.anchor_dist_m.defender | 8.62 / 8.74 | 9.29 / 10.14 | 8.70 / 7.38 |
| shape.attacking.anchor_dist_m.forward | 14.52 / 14.31 | 16.20 / 12.81 | 12.70 / 14.71 |
| shape.attacking.anchor_dist_m.goalkeeper | 1.21 / 1.15 | 1.26 / 1.20 | 1.12 / 1.17 |
| shape.attacking.anchor_dist_m.midfielder | 14.10 / 16.88 | 15.49 / 16.46 | 14.37 / 13.88 |
| shape.attacking.anchor_dist_shape_m.defender | 6.45 / 6.51 | 7.50 / 6.84 | 6.09 / 6.24 |
| shape.attacking.anchor_dist_shape_m.forward | 10.69 / 10.49 | 10.85 / 10.37 | 10.28 / 10.12 |
| shape.attacking.anchor_dist_shape_m.midfielder | 8.01 / 11.10 | 9.76 / 10.30 | 8.67 / 8.73 |
| shape.attacking.bbox_area_m2 | 346.3 / 368.8 | 335.3 / 378.4 | 373.6 / 372.7 |
| shape.attacking.bbox_depth_m | 18.07 / 17.74 | 16.35 / 17.85 | 18.90 / 17.69 |
| shape.attacking.bbox_width_m | 18.27 / 19.63 | 19.09 / 20.30 | 18.90 / 19.41 |
| shape.attacking.length_m | 18.07 / 17.74 | 16.35 / 17.85 | 18.90 / 17.69 |
| shape.attacking.nearest_mate_mean_m | 10.45 / 10.83 | 10.17 / 11.17 | 10.94 / 10.63 |
| shape.attacking.nearest_mate_p10_m | 3.18 / 3.43 | 3.12 / 3.89 | 3.23 / 3.13 |
| shape.attacking.pct_of_open_play | 51.18 / 48.82 | 46.86 / 53.14 | 50.35 / 49.65 |
| shape.attacking.stretch_m | 9.51 / 9.74 | 9.29 / 9.90 | 9.87 / 9.69 |
| shape.attacking.width_m | 18.27 / 19.63 | 19.09 / 20.30 | 18.90 / 19.41 |
| shape.defending.anchor_dist_m.defender | 9.44 / 10.04 | 9.76 / 10.19 | 8.75 / 9.04 |
| shape.defending.anchor_dist_m.forward | 15.38 / 15.32 | 16.05 / 13.50 | 13.84 / 15.26 |
| shape.defending.anchor_dist_m.goalkeeper | 1.39 / 1.34 | 1.41 / 1.29 | 1.27 / 1.38 |
| shape.defending.anchor_dist_m.midfielder | 13.95 / 15.53 | 14.46 / 15.66 | 13.35 / 12.66 |
| shape.defending.anchor_dist_shape_m.defender | 9.05 / 9.58 | 9.60 / 8.47 | 8.20 / 9.44 |
| shape.defending.anchor_dist_shape_m.forward | 10.57 / 10.64 | 10.70 / 9.86 | 9.77 / 10.49 |
| shape.defending.anchor_dist_shape_m.midfielder | 8.13 / 10.52 | 9.37 / 9.83 | 8.00 / 7.84 |
| shape.defending.bbox_area_m2 | 305.5 / 295.5 | 307.1 / 380.6 | 364.8 / 300.6 |
| shape.defending.bbox_depth_m | 15.97 / 15.18 | 14.65 / 18.09 | 17.42 / 14.65 |
| shape.defending.bbox_width_m | 18.02 / 18.20 | 19.05 / 19.74 | 19.59 / 18.96 |
| shape.defending.length_m | 15.97 / 15.18 | 14.65 / 18.09 | 17.42 / 14.65 |
| shape.defending.line_x_std_m | 5.41 / 4.92 | 4.64 / 5.84 | 5.78 / 4.57 |
| shape.defending.line_x_variance_m2 | 45.77 / 39.38 | 34.01 / 49.65 | 49.16 / 32.38 |
| shape.defending.nearest_mate_mean_m | 9.96 / 9.76 | 9.95 / 11.24 | 11.10 / 9.98 |
| shape.defending.nearest_mate_p10_m | 3.78 / 3.42 | 3.90 / 4.06 | 4.06 / 3.95 |
| shape.defending.pct_of_open_play | 48.82 / 51.18 | 53.14 / 46.86 | 49.65 / 50.35 |
| shape.defending.pct_outfield_behind_ball | 60.93 / 63.15 | 59.45 / 63.32 | 67.04 / 61.82 |
| shape.defending.stretch_m | 8.84 / 8.74 | 8.83 / 9.86 | 9.68 / 8.78 |
| shape.defending.swarm2_pct | 1.27 / 2.30 | 1.15 / 0.92 | 2.29 / 1.78 |
| shape.defending.swarm3_pct | 0 / 0.10 | 0 / 0 | 0.07 / 0 |
| shape.defending.width_m | 18.02 / 18.20 | 19.05 / 19.74 | 19.59 / 18.96 |

GK scenario field: not present in the public state.

## 11v11 normal vs normal (10 min x 3 seeds from 1)

Command: `psfc-replay analyze --format 11v11 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty normal --away-difficulty normal`

Gameplay version: `0.1.0-5v5`

| seed | final state_hash | score (H-A) |
|---|---|---|
| 1 | `4d895153ae110891` | 0-1 |
| 2 | `4d78b76be0152ae0` | 1-2 |
| 3 | `0a9571e3cf001ba0` | 0-0 |

Match meta (mean over seeds): goals_total = 1.33, open_play_minutes = 10.00, open_play_pct_of_ticks = 98.23, shots_total = 14.33, ticks_simulated = 36656

### Match

| metric | home | away | teams mean |
|---|---:|---:|---:|
| fouls | 0 | 0 | **0** |
| goals | 0.33 | 1 | **0.67** |
| offsides | 0 | 0 | **0** |
| passes_completed_pct | 83.59 | 83.64 | **83.61** |
| possession_pct | 45.43 | 54.57 | **50** |
| shots | 4.33 | 10 | **7.17** |
| shots_on_target | 2 | 4.33 | **3.17** |

### Goalkeeper

| metric | home | away | teams mean |
|---|---:|---:|---:|
| catches | 1 | 0.67 | **0.83** |
| claims_non_save | 0.33 | 0.67 | **0.50** |
| excursion_mean_s | 0.75 | 0.44 | **0.59** |
| excursions_gt1s_per_10min | 1.00 | 0.33 | **0.67** |
| gathers | 1.33 | 1.33 | **1.33** |
| goals_conceded | 1 | 0.33 | **0.67** |
| goals_conceded_gk_gt3m_off_line | 1 | 0 | **0.50** |
| goals_conceded_no_shot_found | 0 | 0 | **0** |
| line_dist_at_conceded_shot_mean_m | 2.83 | 0.12 | **1.48** |
| line_dist_max_m | 6.86 | 6.50 | **6.68** |
| line_dist_mean_m | 4.40 | 4.52 | **4.46** |
| parries | 2.67 | 1.67 | **2.17** |
| pct_outside_6yd | 0.24 | 0.15 | **0.20** |
| pct_outside_box | 0 | 0 | **0** |
| rush_count | 1.33 | 1.33 | **1.33** |
| rush_per_10min | 1.33 | 1.33 | **1.33** |
| saves | 3.67 | 2.33 | **3** |
| tips_over | 0 | 0 | **0** |

### Defence and shape

| metric | home | away | teams mean |
|---|---:|---:|---:|
| attacking.anchor_dist_m.defender | 19.88 | 22.19 | **21.03** |
| attacking.anchor_dist_m.forward | 25.37 | 24.31 | **24.84** |
| attacking.anchor_dist_m.goalkeeper | 2.23 | 2.25 | **2.24** |
| attacking.anchor_dist_m.midfielder | 18.58 | 17.47 | **18.03** |
| attacking.anchor_dist_shape_m.defender | 16.84 | 15.45 | **16.15** |
| attacking.anchor_dist_shape_m.forward | 19.01 | 19.92 | **19.46** |
| attacking.anchor_dist_shape_m.midfielder | 12.10 | 10.58 | **11.34** |
| attacking.bbox_area_m2 | 1953.0 | 1975.7 | **1964.4** |
| attacking.bbox_depth_m | 41.28 | 40.56 | **40.92** |
| attacking.bbox_width_m | 45.59 | 47.05 | **46.32** |
| attacking.length_m | 41.28 | 40.56 | **40.92** |
| attacking.nearest_mate_mean_m | 12.46 | 12.34 | **12.40** |
| attacking.nearest_mate_p10_m | 5.71 | 5.40 | **5.56** |
| attacking.pct_of_open_play | 47.88 | 52.12 | **50** |
| attacking.stretch_m | 19.53 | 19.85 | **19.69** |
| attacking.width_m | 45.59 | 47.05 | **46.32** |
| defending.anchor_dist_m.defender | 18.71 | 21.27 | **19.99** |
| defending.anchor_dist_m.forward | 27.19 | 26.88 | **27.03** |
| defending.anchor_dist_m.goalkeeper | 2.61 | 2.53 | **2.57** |
| defending.anchor_dist_m.midfielder | 18.81 | 18.18 | **18.50** |
| defending.anchor_dist_shape_m.defender | 17.43 | 16.18 | **16.81** |
| defending.anchor_dist_shape_m.forward | 19.97 | 21.51 | **20.74** |
| defending.anchor_dist_shape_m.midfielder | 12.07 | 10.68 | **11.38** |
| defending.bbox_area_m2 | 1511.8 | 1506.5 | **1509.1** |
| defending.bbox_depth_m | 36.90 | 35.84 | **36.37** |
| defending.bbox_width_m | 39.16 | 40.14 | **39.65** |
| defending.length_m | 36.90 | 35.84 | **36.37** |
| defending.line_x_std_m | 6.62 | 6.71 | **6.67** |
| defending.line_x_variance_m2 | 70.88 | 66.58 | **68.73** |
| defending.nearest_mate_mean_m | 10.69 | 10.56 | **10.63** |
| defending.nearest_mate_p10_m | 5.36 | 5.38 | **5.37** |
| defending.pct_of_open_play | 52.12 | 47.88 | **50** |
| defending.pct_outfield_behind_ball | 65.95 | 69.18 | **67.57** |
| defending.stretch_m | 16.99 | 17.09 | **17.04** |
| defending.swarm2_pct | 0.91 | 1.23 | **1.07** |
| defending.swarm3_pct | 0 | 0 | **0** |
| defending.width_m | 39.16 | 40.14 | **39.65** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 7.89 | 7.88 | **7.88** |
| accel_gt10_pct | 36.31 | 36.26 | **36.29** |
| accel_mean_mps2 | 7.89 | 7.88 | **7.88** |
| dspeed_max_mps_per_tick | 7.24 | 8.33 | **7.78** |
| heading_change_p95_deg_per_tick_gt3mps | 3.17 | 3.20 | **3.18** |
| heading_change_pct.0_10 | 99.64 | 99.64 | **99.64** |
| heading_change_pct.10_45 | 0.35 | 0.34 | **0.34** |
| heading_change_pct.45_90 | 0.01 | 0.01 | **0.01** |
| heading_change_pct.gt90 | 0.01 | 0.01 | **0.01** |
| lateral_accel_mean_mps2 | 5.50 | 5.58 | **5.54** |
| reversals_per_player_min | 0.19 | 0.17 | **0.18** |
| side_flips_per_player_min | 43.75 | 45.76 | **44.76** |
| speed_oscillation_pct | 3.08 | 3.08 | **3.08** |
| turn_rate_mean_deg_per_s | 76.72 | 77.29 | **77.00** |

### Collisions and push-out

| metric | home | away | teams mean |
|---|---:|---:|---:|
| any.depth_max_m | 0.80 | 0.80 | **0.80** |
| any.depth_mean_m | 0.00 | 0.00 | **0.00** |
| any.events_per_min | 47.49 | 47.49 | **47.49** |
| any.stuck_mean_ticks | 21.40 | 21.40 | **21.40** |
| any.stuck_pairs | 31 | 31 | **31** |
| any.ticks_gt1cm_per_min | 0.27 | 0.27 | **0.27** |
| any.ticks_per_min | 145.5 | 145.5 | **145.5** |
| correction_vel_corrected_p50_mps | 0.16 | 0.16 | **0.16** |
| correction_vel_corrected_p99_mps | 5.22 | 4.94 | **5.08** |
| correction_vel_corrected_pct | 0.47 | 0.51 | **0.49** |
| correction_vel_max_mps | 20.45 | 21.03 | **20.74** |
| correction_vel_p50_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p95_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p99_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_pct_gt_0_3 | 0.08 | 0.08 | **0.08** |
| opponent.depth_max_m | 0.28 | 0.28 | **0.28** |
| opponent.depth_mean_m | 0.00 | 0.00 | **0.00** |
| opponent.events_per_min | 41.22 | 41.22 | **41.22** |
| opponent.stuck_mean_ticks | 19.74 | 19.74 | **19.74** |
| opponent.stuck_pairs | 26.33 | 26.33 | **26.33** |
| opponent.ticks_gt1cm_per_min | 0.10 | 0.10 | **0.10** |
| opponent.ticks_per_min | 123.9 | 123.9 | **123.9** |
| teammate.depth_max_m | 0.51 | 0.53 | **0.52** |
| teammate.depth_mean_m | 0.01 | 0.00 | **0.01** |
| teammate.events_per_min | 2.93 | 3.33 | **3.13** |
| teammate.stuck_mean_ticks | 21.44 | 34.56 | **28** |
| teammate.stuck_pairs | 2.33 | 2.33 | **2.33** |
| teammate.ticks_gt1cm_per_min | 0.10 | 0.10 | **0.10** |
| teammate.ticks_per_min | 8.66 | 13.80 | **11.23** |

### Coordination

| metric | home | away | teams mean |
|---|---:|---:|---:|
| cruise_over72_pct | 37.72 | 38.98 | **38.35** |
| dual_chaser_pct_of_loose | 15.98 | 17.59 | **16.78** |
| dual_chaser_ticks_per_min | 403.6 | 444.6 | **424.1** |
| engagers3_pct_of_carrier_ticks | 0.15 | 0 | **0.07** |
| engagers3_ticks_per_min | 0.90 | 0 | **0.45** |
| max_simultaneous_chasers | 4.33 | 4.33 | **4.33** |

### Aerial

| metric | home | away | teams mean |
|---|---:|---:|---:|
| headers | 27.33 | 29.33 | **28.33** |
| headers_clear | 1 | 0.33 | **0.67** |
| headers_pass | 25.33 | 26 | **25.67** |
| headers_shot | 1 | 3 | **2** |
| jumps_gk | 4.67 | 3.33 | **4** |
| jumps_outfield | 53 | 54.33 | **53.67** |
| max_airborne_outfield | 1 | 1 | **1** |

### Team shape vs envelope (teams mean)

| metric | in possession | out of possession | 11v11 envelope (m) |
|---|---:|---:|---|
| length_m | 40.92 | 36.37 | 31-46 |
| width_m | 46.32 | 39.65 | 35-48 |
| stretch_m | 19.69 (outside) | 17.04 (outside) | 7-16 |
| nearest_mate_mean_m | 12.40 | 10.63 | none published (in-possession mean near 9-10 m, estimate) |
| nearest_mate_p10_m | 5.56 | 5.37 | none published (in-possession mean near 9-10 m, estimate) |

Envelope: peer-reviewed 11v11 ranges (length 31-46 m, width 35-48 m, stretch index 7-16 m) quoted in the research report; the report's tighter targets are 31-36 / 35-40 / 7-10 m out of possession and 40-46 / 42-48 / 12-16 m in possession (estimates).

### Per seed (home / away)

| metric | seed 1 | seed 2 | seed 3 |
|---|---:|---:|---:|
| aerial.headers | 29 / 28 | 30 / 34 | 23 / 26 |
| aerial.headers_clear | 1 / 0 | 2 / 1 | 0 / 0 |
| aerial.headers_pass | 28 / 24 | 27 / 31 | 21 / 23 |
| aerial.headers_shot | 0 / 4 | 1 / 2 | 2 / 3 |
| aerial.jumps_gk | 3 / 3 | 4 / 2 | 7 / 5 |
| aerial.jumps_outfield | 55 / 55 | 54 / 56 | 50 / 52 |
| aerial.max_airborne_outfield | 1 / 1 | 1 / 1 | 1 / 1 |
| collision.any.depth_max_m | 0.80 / 0.80 | 0.80 / 0.80 | 0.80 / 0.80 |
| collision.any.depth_mean_m | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.any.events_per_min | 43.80 / 43.80 | 45.90 / 45.90 | 52.76 / 52.76 |
| collision.any.stuck_mean_ticks | 22.61 / 22.61 | 22.26 / 22.26 | 19.33 / 19.33 |
| collision.any.stuck_pairs | 28 / 28 | 23 / 23 | 42 / 42 |
| collision.any.ticks_gt1cm_per_min | 0.30 / 0.30 | 0.20 / 0.20 | 0.30 / 0.30 |
| collision.any.ticks_per_min | 134.4 / 134.4 | 127.8 / 127.8 | 174.2 / 174.2 |
| collision.correction_vel_corrected_p50_mps | 0.16 / 0.16 | 0.17 / 0.17 | 0.16 / 0.16 |
| collision.correction_vel_corrected_p99_mps | 5.37 / 4.28 | 5.51 / 5.47 | 4.78 / 5.05 |
| collision.correction_vel_corrected_pct | 0.43 / 0.48 | 0.45 / 0.43 | 0.54 / 0.61 |
| collision.correction_vel_max_mps | 14.78 / 24.50 | 24.23 / 14.10 | 22.34 / 24.51 |
| collision.correction_vel_p50_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p95_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p99_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_pct_gt_0_3 | 0.07 / 0.07 | 0.08 / 0.08 | 0.08 / 0.09 |
| collision.opponent.depth_max_m | 0.51 / 0.51 | 0.01 / 0.01 | 0.31 / 0.31 |
| collision.opponent.depth_mean_m | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.opponent.events_per_min | 38.20 / 38.20 | 41.80 / 41.80 | 43.67 / 43.67 |
| collision.opponent.stuck_mean_ticks | 19.29 / 19.29 | 20.79 / 20.79 | 19.14 / 19.14 |
| collision.opponent.stuck_pairs | 24 / 24 | 19 / 19 | 36 / 36 |
| collision.opponent.ticks_gt1cm_per_min | 0.20 / 0.20 | 0 / 0 | 0.10 / 0.10 |
| collision.opponent.ticks_per_min | 110.3 / 110.3 | 111.4 / 111.4 | 150.0 / 150.0 |
| collision.teammate.depth_max_m | 0.00 / 0.80 | 0.80 / 0.00 | 0.74 / 0.80 |
| collision.teammate.depth_mean_m | 0.00 / 0.00 | 0.01 / 0.00 | 0.01 / 0.01 |
| collision.teammate.events_per_min | 2.70 / 2.90 | 2.60 / 1.50 | 3.50 / 5.60 |
| collision.teammate.stuck_mean_ticks | 20 / 50 | 30 / 27 | 14.33 / 26.67 |
| collision.teammate.stuck_pairs | 1 / 3 | 3 / 1 | 3 / 3 |
| collision.teammate.ticks_gt1cm_per_min | 0 / 0.10 | 0.20 / 0 | 0.10 / 0.20 |
| collision.teammate.ticks_per_min | 5.70 / 19.20 | 12.40 / 4.10 | 7.89 / 18.09 |
| coord.cruise_over72_pct | 37.29 / 38.37 | 37.79 / 38.23 | 38.09 / 40.35 |
| coord.dual_chaser_pct_of_loose | 14.86 / 19.20 | 15.65 / 17.48 | 17.43 / 16.08 |
| coord.dual_chaser_ticks_per_min | 378.5 / 488.9 | 394.9 / 441.2 | 437.5 / 403.7 |
| coord.engagers3_pct_of_carrier_ticks | 0 / 0 | 0 / 0 | 0.44 / 0 |
| coord.engagers3_ticks_per_min | 0 / 0 | 0 / 0 | 2.70 / 0 |
| coord.max_simultaneous_chasers | 4 / 5 | 4 / 4 | 5 / 4 |
| gk.catches | 2 / 0 | 0 / 1 | 1 / 1 |
| gk.claims_non_save | 0 / 2 | 0 / 0 | 1 / 0 |
| gk.excursion_mean_s | 1.23 / 0.58 | 0 / 0 | 1.02 / 0.72 |
| gk.excursions_gt1s_per_10min | 1 / 0 | 0 / 0 | 2.00 / 1.00 |
| gk.gathers | 2 / 2 | 0 / 1 | 2 / 1 |
| gk.goals_conceded | 1 / 0 | 2 / 1 | 0 / 0 |
| gk.goals_conceded_gk_gt3m_off_line | 1 / 0 | 2 / 0 | 0 / 0 |
| gk.goals_conceded_no_shot_found | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.line_dist_at_conceded_shot_mean_m | 4.48 / 0 | 4.01 / 0.36 | 0 / 0 |
| gk.line_dist_max_m | 8.16 / 6.47 | 5.27 / 5.27 | 7.15 / 7.76 |
| gk.line_dist_mean_m | 4.45 / 4.42 | 4.38 / 4.57 | 4.36 / 4.57 |
| gk.parries | 0 / 3 | 3 / 0 | 5 / 2 |
| gk.pct_outside_6yd | 0.21 / 0.10 | 0 / 0 | 0.51 / 0.36 |
| gk.pct_outside_box | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_count | 1 / 1 | 0 / 0 | 3 / 3 |
| gk.rush_per_10min | 1 / 1 | 0 / 0 | 3.00 / 3.00 |
| gk.saves | 2 / 3 | 3 / 1 | 6 / 3 |
| gk.tips_over | 0 / 0 | 0 / 0 | 0 / 0 |
| match.fouls | 0 / 0 | 0 / 0 | 0 / 0 |
| match.goals | 0 / 1 | 1 / 2 | 0 / 0 |
| match.offsides | 0 / 0 | 0 / 0 | 0 / 0 |
| match.passes_completed_pct | 79.22 / 82.12 | 87.18 / 83.91 | 84.38 / 84.88 |
| match.possession_pct | 49.17 / 50.83 | 43.93 / 56.07 | 43.20 / 56.80 |
| match.shots | 4 / 5 | 3 / 10 | 6 / 15 |
| match.shots_on_target | 3 / 2 | 1 / 5 | 2 / 6 |
| move.accel_from_pos_mean_mps2 | 7.72 / 7.90 | 7.82 / 7.83 | 8.11 / 7.92 |
| move.accel_gt10_pct | 35.25 / 36.33 | 36.31 / 36.18 | 37.37 / 36.28 |
| move.accel_mean_mps2 | 7.73 / 7.90 | 7.83 / 7.84 | 8.11 / 7.91 |
| move.dspeed_max_mps_per_tick | 7.29 / 8.55 | 7.74 / 7.92 | 6.68 / 8.52 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.14 / 3.18 | 3.18 / 3.23 | 3.20 / 3.18 |
| move.heading_change_pct.0_10 | 99.67 / 99.65 | 99.65 / 99.64 | 99.59 / 99.64 |
| move.heading_change_pct.10_45 | 0.32 / 0.33 | 0.33 / 0.34 | 0.39 / 0.34 |
| move.heading_change_pct.45_90 | 0.01 / 0.01 | 0.01 / 0.01 | 0.01 / 0.01 |
| move.heading_change_pct.gt90 | 0.00 / 0.01 | 0.01 / 0.01 | 0.01 / 0.00 |
| move.lateral_accel_mean_mps2 | 5.38 / 5.51 | 5.51 / 5.61 | 5.59 / 5.61 |
| move.reversals_per_player_min | 0.16 / 0.19 | 0.23 / 0.17 | 0.18 / 0.15 |
| move.side_flips_per_player_min | 44.40 / 45.14 | 42.53 / 45.37 | 44.32 / 46.78 |
| move.speed_oscillation_pct | 3.06 / 3.04 | 3.02 / 3.12 | 3.15 / 3.08 |
| move.turn_rate_mean_deg_per_s | 74.80 / 76.40 | 76.19 / 78.24 | 79.17 / 77.23 |
| shape.attacking.anchor_dist_m.defender | 20.18 / 21.53 | 18.29 / 23.77 | 21.16 / 21.27 |
| shape.attacking.anchor_dist_m.forward | 25.45 / 23.81 | 26.18 / 22.71 | 24.46 / 26.42 |
| shape.attacking.anchor_dist_m.goalkeeper | 2.26 / 2.28 | 2.26 / 2.22 | 2.18 / 2.26 |
| shape.attacking.anchor_dist_m.midfielder | 18.06 / 17.45 | 18.39 / 19.04 | 19.29 / 15.92 |
| shape.attacking.anchor_dist_shape_m.defender | 16.42 / 15.93 | 15.04 / 16.80 | 19.07 / 13.63 |
| shape.attacking.anchor_dist_shape_m.forward | 20.16 / 18.49 | 19.55 / 17.98 | 17.31 / 23.27 |
| shape.attacking.anchor_dist_shape_m.midfielder | 12.02 / 10.39 | 11.60 / 12.18 | 12.69 / 9.17 |
| shape.attacking.bbox_area_m2 | 2024.3 / 2027.0 | 1898.5 / 1970.9 | 1936.3 / 1929.3 |
| shape.attacking.bbox_depth_m | 41.82 / 41.07 | 40.45 / 40.33 | 41.58 / 40.28 |
| shape.attacking.bbox_width_m | 46.71 / 47.63 | 45.14 / 47.14 | 44.91 / 46.39 |
| shape.attacking.length_m | 41.82 / 41.07 | 40.45 / 40.33 | 41.58 / 40.28 |
| shape.attacking.nearest_mate_mean_m | 12.85 / 12.57 | 12.20 / 12.24 | 12.32 / 12.21 |
| shape.attacking.nearest_mate_p10_m | 6.23 / 5.18 | 5.50 / 5.65 | 5.40 / 5.37 |
| shape.attacking.pct_of_open_play | 48.17 / 51.83 | 47.40 / 52.60 | 48.08 / 51.92 |
| shape.attacking.stretch_m | 19.90 / 20.22 | 19.13 / 19.78 | 19.58 / 19.55 |
| shape.attacking.width_m | 46.71 / 47.63 | 45.14 / 47.14 | 44.91 / 46.39 |
| shape.defending.anchor_dist_m.defender | 19.67 / 19.33 | 16.64 / 22.72 | 19.82 / 21.77 |
| shape.defending.anchor_dist_m.forward | 27.72 / 26.78 | 28.19 / 25.17 | 25.65 / 28.69 |
| shape.defending.anchor_dist_m.goalkeeper | 2.62 / 2.59 | 2.60 / 2.48 | 2.61 / 2.53 |
| shape.defending.anchor_dist_m.midfielder | 19.39 / 17.24 | 18.20 / 20.03 | 18.85 / 17.27 |
| shape.defending.anchor_dist_shape_m.defender | 17.92 / 16.27 | 15.41 / 17.87 | 18.97 / 14.40 |
| shape.defending.anchor_dist_shape_m.forward | 21.20 / 20.74 | 20.39 / 19.15 | 18.30 / 24.64 |
| shape.defending.anchor_dist_shape_m.midfielder | 12.77 / 10.24 | 11.22 / 12.45 | 12.22 / 9.36 |
| shape.defending.bbox_area_m2 | 1546.1 / 1488.3 | 1456.5 / 1486.0 | 1532.7 / 1545.2 |
| shape.defending.bbox_depth_m | 37.87 / 35.69 | 35.88 / 35.16 | 36.96 / 36.65 |
| shape.defending.bbox_width_m | 39.19 / 39.77 | 38.68 / 40.09 | 39.62 / 40.57 |
| shape.defending.length_m | 37.87 / 35.69 | 35.88 / 35.16 | 36.96 / 36.65 |
| shape.defending.line_x_std_m | 6.89 / 6.55 | 6.07 / 6.87 | 6.91 / 6.72 |
| shape.defending.line_x_variance_m2 | 79.02 / 65.69 | 58.92 / 68.55 | 74.69 / 65.50 |
| shape.defending.nearest_mate_mean_m | 10.77 / 10.50 | 10.62 / 10.43 | 10.68 / 10.77 |
| shape.defending.nearest_mate_p10_m | 5.56 / 5.42 | 5.32 / 5.42 | 5.21 / 5.31 |
| shape.defending.pct_of_open_play | 51.83 / 48.17 | 52.60 / 47.40 | 51.92 / 48.08 |
| shape.defending.pct_outfield_behind_ball | 68.51 / 67.22 | 65.50 / 69.53 | 63.83 / 70.80 |
| shape.defending.stretch_m | 17.02 / 16.73 | 16.74 / 16.94 | 17.20 / 17.61 |
| shape.defending.swarm2_pct | 1.06 / 0.52 | 0.91 / 0.51 | 0.74 / 2.65 |
| shape.defending.swarm3_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| shape.defending.width_m | 39.19 / 39.77 | 38.68 / 40.09 | 39.62 / 40.57 |

GK scenario field: not present in the public state.

## Metric definitions

- **aerial.headers (+ _shot/_clear/_pass)**: Outfielder with z > 0.14 m whose recent_kick_ticks rose versus the previous tick (a new contact); split by recent_kick_kind: 4/5 shot, 3 clearance/lob, else pass/flick.
- **aerial.jumps_outfield / aerial.jumps_gk**: Count of z going from <= 0 to > 0 between consecutive open-play ticks (outfielders / GK; GK dives that leave the ground count).
- **aerial.max_airborne_outfield**: Max number of the side's outfielders with z > 0 on one open-play tick.
- **collision.<class>.depth_mean_m / depth_max_m**: Overlap depth 0.8 m - centre distance over all overlapping pair-ticks of the class: mean, and maximum over the match.
- **collision.<class>.events_per_min**: Overlap episodes per open-play minute: a pair's maximal run of consecutive open-play ticks below 0.8 m counts once (runs are cut when open play stops).
- **collision.<class>.stuck_pairs / stuck_mean_ticks**: Stuck pair = overlap episode lasting more than 10 consecutive open-play ticks (no relative-speed condition). Count per match and mean episode length in ticks (60 ticks = 1 s; 0 when there are none).
- **collision.<class>.ticks_gt1cm_per_min**: Open-play ticks with at least one pair of the class overlapping by more than 1 cm (centre distance < 0.79 m), per open-play minute. The state is sampled after the sim's separation pass, which leaves resolved pairs at 0.8 m minus fixed-point rounding (resting contact); this variant counts only residual penetration the pass did not resolve.
- **collision.<class>.ticks_per_min**: Round 3. Pair classes: any (every on-pitch pair, both teams, GKs included), teammate (pairs of this side's players only), opponent (home-away pairs). An overlap is a pair whose centres are closer than 0.8 m (the sim's min_sep = 2 x 0.4 m body radius) on an open-play tick, sampled after the sim's separation pass. ticks_per_min = open-play ticks with at least one overlapping pair of the class / open-play minutes. 'any' and 'opponent' are match-level and identical in the home and away maps; 'teammate' is the side's own pairs, so match any = home teammate + away teammate + opponent for events.
- **collision.correction_vel_corrected_pct / corrected_p50_mps / corrected_p99_mps / max_mps**: Same samples restricted to corrected ones (> 0.01 m/s; fixed-point noise is ~1e-7 m/s): % of samples corrected, nearest-rank p50 / p99 of the corrected samples, and the match maximum over all samples. Contact is rare, so the unconditional p50/p95 are ~0 and the conditional ones characterise the push-out itself.
- **collision.correction_vel_p50_mps / p95 / p99 / pct_gt_0_3**: Position-correction velocity per player per consecutive open-play tick pair: |(pos_t - pos_t-1) * 60 - vel_t| in m/s, all on-pitch players incl. GK. vel_t is the published end-of-tick velocity, which the sim integrated (pos += vel * dt) before the separation pass, so with no contact the value is 0 apart from touchline clamps; with contact it is push-out displacement / dt minus the separation pass's own velocity impulse/slip (both act along the contact, so this is a lower bound of the pure push-out speed). Nearest-rank percentiles over the match; pct_gt_0_3 = % of samples above 0.3 m/s (the research report's p99 target).
- **coord.cruise_over72_pct**: Over outfielder-ticks of eligible players (not the ball carrier, more than 8 m from the ball, and not one of the side's two outfielders closest to the ball, ties by slot): % with speed > 0.72 * top speed. Top speed approximates the sim's step_all_players at full stamina with full sprint throttle and no carrier/jockey/direction penalty: (5.45 + 0.026 * pace) m/s * 1.205 (5v5) or * 1.220 (11v11); stamina, the facing-alignment scale and run boosts are ignored, so a tired player's real top speed is lower than assumed (the metric is a lower bound on violations).
- **coord.dual_chaser_ticks_per_min / dual_chaser_pct_of_loose**: Open-play ticks with ball.owner == None (includes passes and shots in flight) on which >= 2 of this side's on-pitch players (GK included) are within 8 m of the ball (xy) and closing on it at > 1.5 m/s (vel . unit(ball - pos)); per open-play minute, and as % of loose-ball ticks.
- **coord.engagers3_ticks_per_min / engagers3_pct_of_carrier_ticks**: Open-play ticks where the opponent owns the ball and >= 3 of this side's outfielders are within 2.5 m of the carrier; per open-play minute, and as % of ticks with an opposing carrier (same denominator as swarm3_pct).
- **coord.max_simultaneous_chasers**: Maximum number of this side's players meeting the chaser condition on one loose-ball tick.
- **final_hashes**: MatchSimulation::state_hash() after the final tick, one per seed, hex. Identical hashes on a rerun prove determinism; any sim change will change them.
- **general.means**: mean_home / mean_away are means over seeds; mean_teams is the mean of mean_home and mean_away (for *_max keys this is a mean of per-match maxima).
- **general.phase**: Per side and open-play tick: 'attacking' if ball.owner is that side, else (ball loose) if ball.last_touch_team is that side; 'defending' if the owner / last toucher is the opponent; ticks with no owner and no last touch are excluded from phase metrics.
- **general.sampling**: The sim is ticked at 60 Hz with both controllers set to SideController::Cpu(difficulty) and idle external input; MatchState is sampled after every tick. 'Open play' = MatchPhase::OpenPlay. Per-player deltas only use pairs of consecutive open-play ticks with the same footballer in the slot; sent-off players are skipped. Coordinates are metres, x along the pitch, own goal line at x = -attack_sign(second_half_swapped) * pitch_length/2.
- **gk.catches / gk.parries / gk.tips_over**: MatchEventType::Save events for this side classified by detail text: 'catches' -> catch, 'tips' -> tip over, anything else -> parry.
- **gk.claims_non_save**: gk.gathers - gk.catches (possession gains that were not a shot catch: crosses, loose balls, back passes; approximate).
- **gk.excursion_mean_s / gk.excursions_gt1s_per_10min**: Excursion = maximal run of consecutive open-play ticks with the GK outside the goal area (ended by re-entering or by open play stopping; one still open at full time is dropped). Mean duration in seconds, and the rate of excursions longer than 1 s per 10 open-play minutes (filters edge flicker out of rush counts).
- **gk.gathers**: Open-play ticks where ball.owner becomes (side, 0) after not being (side, 0) on the previous open-play tick (any GK gaining possession: catches, claims, picking up loose balls).
- **gk.goals_conceded**: Opponent score increments (includes own goals).
- **gk.goals_conceded_gk_gt3m_off_line**: Conceded goals where the GK's distance from his own goal line at the tick of the scoring side's last Shot event (within 10 s before the goal) was > 3 m.
- **gk.goals_conceded_no_shot_found**: Conceded goals with no Shot event by the scoring side in the preceding 10 s (own goals, deflections); excluded from the line-distance check.
- **gk.line_dist_at_conceded_shot_mean_m**: Mean GK line distance at the attributed shot tick over conceded goals with a shot found.
- **gk.line_dist_mean_m / gk.line_dist_max_m**: GK distance |gk.x - own_goal_x| in metres over open-play ticks (mean / max).
- **gk.pct_outside_6yd**: % of open-play ticks where the GK (slot 0) is outside the goal area: depth from own goal line > 5.5 m OR |y| > goal_width/2 + 5.5 m. Same FIFA goal-area geometry for 5v5 and 11v11 (MatchFormat has no goal-area field).
- **gk.pct_outside_box**: % of open-play ticks where the GK is outside the penalty area: depth > penalty_box_depth OR |y| > penalty_box_width/2 (from MatchFormat).
- **gk.rush_count / gk.rush_per_10min**: Rush event = open-play tick where the GK was inside the goal area on the previous open-play tick, is outside it now, and dot(gk.vel, ball.xy - gk.pos) > 0 (moving toward the ball). Per 10 min = count * 10 / open-play minutes.
- **gk.saves**: PlayerMatchStats.saves of the GK slot's footballer.
- **gk.scenario_switches_per_10min / gk.scenario_distinct_values**: Only present when a GK scenario field exists in the public state: the first serde key of the GK PlayerState (then of MatchState; per-side arrays indexed by side) containing 'scenario', 'gk_mode', 'keeper_mode' or equal to 'gk_state'. Switches = open-play ticks where the serialized value differs from the previous sample (MatchState fields are sampled every 30 ticks). seed reports also list ticks per value.
- **match(meta).goals_total / shots_total**: Both sides combined.
- **match(meta).open_play_minutes**: Open-play ticks / 3600.
- **match(meta).open_play_pct_of_ticks**: Open-play ticks as % of all simulated ticks (includes kick-offs, set pieces, celebrations, half time).
- **match.fouls**: Sum of PlayerMatchStats.fouls_committed.
- **match.goals**: Final score for the side.
- **match.offsides**: Count of MatchEventType::Offside events whose team is this side (the offending side).
- **match.passes_completed_pct**: 100 * sum(passes_completed) / sum(passes_attempted).
- **match.possession_pct**: 100 * possession_ticks[side] / (possession_ticks[home] + possession_ticks[away]) as kept by the sim.
- **match.shots**: Sum of PlayerMatchStats.shots over all players who appeared (starters + subs).
- **match.shots_on_target**: Sum of PlayerMatchStats.shots_on_target (the sim rolls this back on blocks/woodwork misses).
- **move.accel_from_pos_mean_mps2**: Same as accel_mean_mps2 but from the second difference of positions over three consecutive open-play ticks (|(p_t - p_t-1) - (p_t-1 - p_t-2)| * 3600): what the renderer actually sees, independent of the vel field.
- **move.accel_gt10_pct**: % of those player-tick pairs with |vel_t - vel_t-1| * 60 > 10 m/s^2 (roughly above a sprinter's peak acceleration).
- **move.accel_mean_mps2**: Mean |vel_t - vel_t-1| * 60 over all consecutive open-play player-tick pairs (m/s^2).
- **move.dspeed_max_mps_per_tick**: Max ||vel_t| - |vel_t-1|| over the match (m/s per tick).
- **move.heading_change_p95_deg_per_tick_gt3mps**: 95th percentile (nearest rank) of the per-tick heading change |angle(vel_t-1, vel_t)| in degrees, over player-tick pairs with speed > 3 m/s on both ticks.
- **move.heading_change_pct.<bucket>**: Per player per consecutive open-play tick pair with speed > 1 m/s on both ticks: |angle between vel_t-1 and vel_t| bucketed 0-10, 10-45, 45-90, >90 degrees; % of samples. All on-pitch players incl. GK.
- **move.lateral_accel_mean_mps2**: Mean |cross(vel_t-1, vel_t)| / |vel_t-1| * 60 over the same samples: velocity change perpendicular to the previous heading (m/s^2).
- **move.reversals_per_player_min**: Count of those samples with dot(vel_t, vel_t-1) < 0, divided by player-minutes (all consecutive open-play player-ticks / 3600).
- **move.side_flips_per_player_min**: Per player, over the heading-histogram samples (speed > 1 m/s on both ticks): lateral component l_t = cross(vel_t-1, vel_t) / |vel_t-1| (m/s, sign = turning left/right of the previous heading). A side flip is a sample whose l_t and the previous tick's l_t-1 are both above 0.02 m/s in magnitude with opposite signs (avoidance oscillation). Count / player-minutes.
- **move.speed_oscillation_pct**: Over three consecutive open-play ticks where both speed changes exceed 0.05 m/s: % where the speed change flips sign (speed up / slow down / speed up sawtooth). A smooth accelerate-cruise-decelerate profile scores near 0.
- **move.turn_rate_mean_deg_per_s**: Mean of the per-tick heading change (degrees) * 60 over the heading-histogram samples (speed > 1 m/s on both ticks).
- **shape.<phase>.anchor_dist_m.<role>**: Mean distance in metres between each player and psfc_sim::ai::world_anchor_for_player(state, side, idx, 0) (static formation slot, no shift) by PlayerRole.
- **shape.<phase>.anchor_dist_shape_m.<role>**: Same for outfielders after subtracting the outfield block's mean x-offset from anchors on that tick: measures loss of shape independent of the whole team pushing up / dropping as a unit.
- **shape.<phase>.bbox_area_m2 / bbox_depth_m / bbox_width_m**: Axis-aligned bounding box of on-pitch outfielders (GK excluded): depth = x-extent, width = y-extent, area = depth * width.
- **shape.<phase>.length_m / width_m / stretch_m**: Round 3 team shape over on-pitch outfielders (GK excluded): length = x extent, width = y extent (same numbers as bbox_depth_m / bbox_width_m, named after the literature), stretch index = mean distance of outfielders to their centroid. Phase attacking = in possession, defending = out of possession (general.phase). Peer-reviewed 11v11 envelope: length 31-46 m, width 35-48 m, stretch 7-16 m; there is no published 5v5 envelope.
- **shape.<phase>.nearest_mate_mean_m / nearest_mate_p10_m**: For every on-pitch outfielder and tick of the phase, distance to the nearest other outfielder of the same side; mean and 10th percentile (nearest rank).
- **shape.<phase>.pct_of_open_play**: % of the side's open-play ticks classified in that phase.
- **shape.defending.line_x_variance_m2 / line_x_std_m**: Population variance (and std dev) of x of the back line while defending. Back line = outfielders with role Defender; if fewer than 2 (5v5 1-2-1 has one), the 2 outfielders with the deepest base_anchor_permille.x.
- **shape.defending.pct_outfield_behind_ball**: Mean % of the side's outfielders strictly goal-side of the ball ((x - ball.x) * attack_sign < 0) while defending.
- **shape.defending.swarm2_pct / swarm3_pct**: % of defending ticks where the opponent owns the ball (a carrier exists) with >= 2 (>= 3) of this side's outfielders within 1.6 m of the carrier.

