# Match analytics baseline (CPU vs CPU)

The baseline was taken on simulation code at commit **`10e07a3`** (`build(wasm): rebuild psfc_wasm.wasm from the merged simulation`, branch `feat/keen-pascal-bvdins`), before the goalkeeper, team-shape/movement and renderer work. `psfc-sim` was not modified. Machine-readable data is in `docs/match-analytics-baseline.json`; it has the same schema as `psfc-replay analyze --json`.

## How to rerun / compare

```sh
cd sim
cargo build --release -p psfc-replay
# all three baseline configurations in one go (same JSON/MD layout as this file):
./target/release/psfc-replay analyze --preset baseline --seeds 3 --seed-base 1 --minutes 10 --per-seed \
    --json /tmp/after.json --md /tmp/after.md --compare ../docs/match-analytics-baseline.json
# or individually:
./target/release/psfc-replay analyze --format 5v5   --seeds 3 --seed-base 1 --minutes 10 --home-difficulty hard   --away-difficulty hard   --per-seed
./target/release/psfc-replay analyze --format 5v5   --seeds 3 --seed-base 1 --minutes 10 --home-difficulty normal --away-difficulty normal --per-seed
./target/release/psfc-replay analyze --format 11v11 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty normal --away-difficulty normal --per-seed
```

`--compare <baseline.json>` prints a before/after/delta table (teams mean) for every configuration id present in both files, plus whether the final `state_hash`es are identical. On unchanged sim code they are identical (determinism check); any sim change alters them. The full preset takes about 40 s on 4 cores (seeds and configurations run in parallel threads; each match is independent and deterministic).

## Headline (teams mean = mean of home and away, mean over 3 seeds; per team per 10-minute match)

| metric | 5v5 Hard v Hard | 5v5 Normal v Normal | 11v11 Normal v Normal |
|---|---:|---:|---:|
| goals per team (match total) | 7.50 (15.0) | 8.17 (16.3) | 3.17 (6.3) |
| shots / on target per team | 36.3 / 24.2 | 40.0 / 26.0 | 19.3 / 10.2 |
| offsides / fouls per team | 0.17 / 0.50 | 0.83 / 0.17 | 0.50 / 0.17 |
| GK % open play outside 6-yd box | 2.92 | 3.59 | 4.26 |
| GK % open play outside penalty box | 0.00 | 0.00 | 0.00 |
| GK rush events per 10 min | 19.3 | 22.8 | 13.0 |
| GK mean excursion outside 6-yd box (s) | 0.89 | 0.95 | 1.88 |
| GK excursions > 1 s per 10 min | 6.7 | 8.7 | 9.3 |
| GK mean / max distance from line (m) | 3.51 / 9.11 | 3.42 / 9.74 | 4.17 / 13.03 |
| GK saves (catches / parries) | 17.5 (3.0 / 14.5) | 18.7 (2.5 / 16.2) | 6.7 (0.5 / 6.2) |
| goals conceded with GK > 3 m off line at the shot | 6.17 of 7.50 | 5.83 of 8.17 | 1.83 of 3.17 |
| swarming: >= 2 defenders within 1.6 m of carrier (% of defending ticks) | 0.26 | 0.93 | 0.38 |
| back-line x variance while defending (m^2) / std (m) | 88.5 / 7.76 | 54.7 / 6.58 | 148.1 / 11.43 |
| % outfielders behind the ball while defending | 61.3 | 56.5 | 65.8 |
| defender distance from formation anchor, defending, shape-corrected (m) | 7.57 | 7.32 | 12.49 |
| outfield bounding box defending / attacking (m^2) | 366 / 483 | 336 / 331 | 1885 / 2136 |
| heading change per tick at > 1 m/s: 0-10 / 10-45 / 45-90 / >90 deg (%) | 97.29 / 2.65 / 0.04 / 0.02 | 97.27 / 2.68 / 0.04 / 0.02 | 97.65 / 2.33 / 0.01 / 0.01 |
| mean turn rate while moving (deg/s) | 132 | 135 | 119 |
| velocity reversals per player-minute | 0.52 | 0.48 | 0.20 |
| mean abs acceleration (m/s^2) | 18.4 | 19.7 | 14.0 |
| speed sawtooth (% of speed-change pairs flipping sign) | 11.7 | 11.4 | 7.3 |
| outfield jumps / max simultaneous airborne / headers per team | 30.7 / 1 / 11.8 | 30.3 / 1 / 10.8 | 52.0 / 1 / 22.3 |

### Reading the baseline

- **Goalkeepers do not stay off their line for long, but they leave the goal area very often.** They are outside the 6-yard box only about 3-4 % of open play and never outside the penalty box. Still, they "rush" (leave the goal area moving toward the ball) 13-23 times per 10 minutes, and the mean excursion lasts under 1 s in 5v5. Many rushes are short in-and-out steps across the 5.5 m line. The resting position is already deep in the goal area (mean 3.4-4.2 m from the line), and in 5v5 most conceded goals (6.2 of 7.5 Hard, 5.8 of 8.2 Normal) came with the GK more than 3 m off his line when the shot was struck. Saves are mostly parries; catches are rare.
- **Scoring is very high in 5v5** (15-16 goals per 10-minute match, about 2/3 of shots on target). 11v11 is lower (6.3 per match).
- **The back line is not flat.** The x standard deviation of the back line while defending is 6.6-7.8 m in 5v5 and 11.4 m in 11v11. Caveat for 5v5: the 1-2-1 formation has a single Defender, so its "line" is the CB plus the deeper wide midfielder (see the definitions). Swarming (two or more defenders within 1.6 m of the carrier) is rare, below 1 % of defending ticks.
- **Movement is jittery at the micro level.** 97 % of per-tick heading changes are under 10 deg, but the mean turn rate while moving is 120-135 deg/s and the mean absolute acceleration is 14-20 m/s^2. Position second differences give the same number, so this is what the renderer sees, not an artefact of the vel field. That is 2-4 times a sprinter's peak acceleration. Players constantly weave by a few degrees per tick, and about 11 % of consecutive speed changes in 5v5 flip sign.
- **Aerial play:** never more than one outfielder per team is airborne at once, which is consistent with the "one contestant per team" aerial rule.


## 5v5 hard vs hard (10 min x 3 seeds from 1)

Command: `psfc-replay analyze --format 5v5 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty hard --away-difficulty hard`

Gameplay version: `0.1.0-5v5`

| seed | final state_hash | score (H-A) |
|---|---|---|
| 1 | `d5e4f10e0094e51c` | 5-7 |
| 2 | `ac04072e7876caf0` | 4-13 |
| 3 | `2dfb67cf7c01eaf0` | 7-9 |

Match meta (mean over seeds): goals_total = 15, open_play_minutes = 10.01, open_play_pct_of_ticks = 93.03, shots_total = 72.67, ticks_simulated = 38734.7

### Match

| metric | home | away | teams mean |
|---|---:|---:|---:|
| fouls | 0.33 | 0.67 | **0.50** |
| goals | 5.33 | 9.67 | **7.50** |
| offsides | 0 | 0.33 | **0.17** |
| passes_completed_pct | 82.19 | 80.67 | **81.43** |
| possession_pct | 39.86 | 60.14 | **50** |
| shots | 33 | 39.67 | **36.33** |
| shots_on_target | 21.67 | 26.67 | **24.17** |

### Goalkeeper

| metric | home | away | teams mean |
|---|---:|---:|---:|
| catches | 3.33 | 2.67 | **3** |
| claims_non_save | 1.33 | 4 | **2.67** |
| excursion_mean_s | 0.89 | 0.89 | **0.89** |
| excursions_gt1s_per_10min | 6.33 | 6.99 | **6.66** |
| gathers | 4.67 | 6.67 | **5.67** |
| goals_conceded | 9.67 | 5.33 | **7.50** |
| goals_conceded_gk_gt3m_off_line | 7.67 | 4.67 | **6.17** |
| goals_conceded_no_shot_found | 0 | 0 | **0** |
| line_dist_at_conceded_shot_mean_m | 4.54 | 4.52 | **4.53** |
| line_dist_max_m | 9.52 | 8.70 | **9.11** |
| line_dist_mean_m | 3.53 | 3.49 | **3.51** |
| parries | 13.67 | 15.33 | **14.50** |
| pct_outside_6yd | 3.01 | 2.83 | **2.92** |
| pct_outside_box | 0 | 0 | **0** |
| rush_count | 20 | 18.67 | **19.33** |
| rush_per_10min | 19.98 | 18.65 | **19.32** |
| saves | 17 | 18 | **17.50** |
| tips_over | 0 | 0 | **0** |

### Defence and shape

| metric | home | away | teams mean |
|---|---:|---:|---:|
| attacking.anchor_dist_m.defender | 13.35 | 21.52 | **17.43** |
| attacking.anchor_dist_m.forward | 12.28 | 14.25 | **13.27** |
| attacking.anchor_dist_m.goalkeeper | 2.04 | 2.03 | **2.03** |
| attacking.anchor_dist_m.midfielder | 14.91 | 15.88 | **15.39** |
| attacking.anchor_dist_shape_m.defender | 7.92 | 16.98 | **12.45** |
| attacking.anchor_dist_shape_m.forward | 8.85 | 12.02 | **10.43** |
| attacking.anchor_dist_shape_m.midfielder | 9.78 | 12.00 | **10.89** |
| attacking.bbox_area_m2 | 329.2 | 636.8 | **483.0** |
| attacking.bbox_depth_m | 22.51 | 27.78 | **25.14** |
| attacking.bbox_width_m | 14.88 | 19.12 | **17.00** |
| attacking.pct_of_open_play | 42.78 | 57.22 | **50** |
| defending.anchor_dist_m.defender | 10.43 | 11.68 | **11.06** |
| defending.anchor_dist_m.forward | 13.14 | 15.11 | **14.12** |
| defending.anchor_dist_m.goalkeeper | 1.77 | 1.71 | **1.74** |
| defending.anchor_dist_m.midfielder | 15.12 | 14.18 | **14.65** |
| defending.anchor_dist_shape_m.defender | 7.19 | 7.96 | **7.57** |
| defending.anchor_dist_shape_m.forward | 10.74 | 10.47 | **10.60** |
| defending.anchor_dist_shape_m.midfielder | 10.42 | 9.70 | **10.06** |
| defending.bbox_area_m2 | 400.6 | 331.1 | **365.8** |
| defending.bbox_depth_m | 25.38 | 20.77 | **23.08** |
| defending.bbox_width_m | 15.86 | 16.17 | **16.02** |
| defending.line_x_std_m | 8.86 | 6.66 | **7.76** |
| defending.line_x_variance_m2 | 118.0 | 58.97 | **88.46** |
| defending.pct_of_open_play | 57.22 | 42.78 | **50** |
| defending.pct_outfield_behind_ball | 64.02 | 58.54 | **61.28** |
| defending.swarm2_pct | 0.41 | 0.11 | **0.26** |
| defending.swarm3_pct | 0 | 0 | **0** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 18.48 | 18.20 | **18.34** |
| accel_gt10_pct | 61.01 | 59.64 | **60.33** |
| accel_mean_mps2 | 18.51 | 18.21 | **18.36** |
| dspeed_max_mps_per_tick | 7.87 | 8.35 | **8.11** |
| heading_change_pct.0_10 | 97.25 | 97.33 | **97.29** |
| heading_change_pct.10_45 | 2.69 | 2.61 | **2.65** |
| heading_change_pct.45_90 | 0.03 | 0.04 | **0.04** |
| heading_change_pct.gt90 | 0.02 | 0.02 | **0.02** |
| lateral_accel_mean_mps2 | 11.89 | 11.54 | **11.72** |
| reversals_per_player_min | 0.59 | 0.46 | **0.52** |
| speed_oscillation_pct | 12.69 | 10.67 | **11.68** |
| turn_rate_mean_deg_per_s | 134.8 | 129.5 | **132.1** |

### Aerial

| metric | home | away | teams mean |
|---|---:|---:|---:|
| headers | 12 | 11.67 | **11.83** |
| headers_clear | 2 | 1 | **1.50** |
| headers_pass | 7.67 | 8 | **7.83** |
| headers_shot | 2.33 | 2.67 | **2.50** |
| jumps_gk | 34.33 | 34.67 | **34.50** |
| jumps_outfield | 31 | 30.33 | **30.67** |
| max_airborne_outfield | 1 | 1 | **1** |

### Per seed (home / away)

| metric | seed 1 | seed 2 | seed 3 |
|---|---:|---:|---:|
| aerial.headers | 11 / 15 | 13 / 8 | 12 / 12 |
| aerial.headers_clear | 3 / 2 | 2 / 0 | 1 / 1 |
| aerial.headers_pass | 7 / 12 | 7 / 4 | 9 / 8 |
| aerial.headers_shot | 1 / 1 | 4 / 4 | 2 / 3 |
| aerial.jumps_gk | 20 / 27 | 47 / 29 | 36 / 48 |
| aerial.jumps_outfield | 27 / 29 | 35 / 33 | 31 / 29 |
| aerial.max_airborne_outfield | 1 / 1 | 1 / 1 | 1 / 1 |
| gk.catches | 2 / 2 | 6 / 2 | 2 / 4 |
| gk.claims_non_save | 2 / 3 | 1 / 1 | 1 / 8 |
| gk.excursion_mean_s | 0.92 / 0.94 | 0.91 / 0.86 | 0.84 / 0.88 |
| gk.excursions_gt1s_per_10min | 5 / 8 | 9 / 6 | 4.99 / 6.98 |
| gk.gathers | 4 / 5 | 7 / 3 | 3 / 12 |
| gk.goals_conceded | 7 / 5 | 13 / 4 | 9 / 7 |
| gk.goals_conceded_gk_gt3m_off_line | 6 / 5 | 10 / 2 | 7 / 7 |
| gk.goals_conceded_no_shot_found | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.line_dist_at_conceded_shot_mean_m | 4.99 / 5.22 | 4.43 / 3.71 | 4.22 / 4.64 |
| gk.line_dist_max_m | 9.58 / 8.35 | 9.80 / 8.65 | 9.16 / 9.10 |
| gk.line_dist_mean_m | 3.60 / 3.59 | 3.51 / 3.44 | 3.50 / 3.44 |
| gk.parries | 13 / 12 | 15 / 14 | 13 / 20 |
| gk.pct_outside_6yd | 1.84 / 2.67 | 4.26 / 2.30 | 2.92 / 3.52 |
| gk.pct_outside_box | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_count | 12 / 17 | 28 / 16 | 20 / 23 |
| gk.rush_per_10min | 12 / 17 | 28 / 16 | 19.95 / 22.95 |
| gk.saves | 15 / 14 | 21 / 16 | 15 / 24 |
| gk.tips_over | 0 / 0 | 0 / 0 | 0 / 0 |
| match.fouls | 0 / 0 | 0 / 1 | 1 / 1 |
| match.goals | 5 / 7 | 4 / 13 | 7 / 9 |
| match.offsides | 0 / 0 | 0 / 0 | 0 / 1 |
| match.passes_completed_pct | 78.48 / 84.85 | 85.11 / 77.03 | 82.99 / 80.14 |
| match.possession_pct | 19.85 / 80.15 | 46.67 / 53.33 | 53.06 / 46.94 |
| match.shots | 25 / 27 | 31 / 53 | 43 / 39 |
| match.shots_on_target | 17 / 22 | 19 / 34 | 29 / 24 |
| move.accel_from_pos_mean_mps2 | 16.63 / 14.81 | 19.40 / 19.66 | 19.42 / 20.13 |
| move.accel_gt10_pct | 54.51 / 48.22 | 64.24 / 64.86 | 64.29 / 65.84 |
| move.accel_mean_mps2 | 16.67 / 14.79 | 19.41 / 19.71 | 19.45 / 20.14 |
| move.dspeed_max_mps_per_tick | 7.42 / 8.46 | 8.02 / 8.52 | 8.19 / 8.07 |
| move.heading_change_pct.0_10 | 97.34 / 97.78 | 97.23 / 97.23 | 97.19 / 96.99 |
| move.heading_change_pct.10_45 | 2.61 / 2.17 | 2.72 / 2.72 | 2.75 / 2.94 |
| move.heading_change_pct.45_90 | 0.03 / 0.03 | 0.03 / 0.04 | 0.04 / 0.05 |
| move.heading_change_pct.gt90 | 0.02 / 0.02 | 0.02 / 0.01 | 0.02 / 0.02 |
| move.lateral_accel_mean_mps2 | 11.37 / 9.86 | 11.94 / 12.39 | 12.36 / 12.37 |
| move.reversals_per_player_min | 0.50 / 0.42 | 0.58 / 0.40 | 0.68 / 0.56 |
| move.speed_oscillation_pct | 16.66 / 10.25 | 10.56 / 11.02 | 10.84 / 10.73 |
| move.turn_rate_mean_deg_per_s | 130.1 / 109.8 | 135.5 / 137.3 | 138.9 / 141.2 |
| shape.attacking.anchor_dist_m.defender | 12.28 / 36.39 | 12.59 / 14.89 | 15.18 / 13.27 |
| shape.attacking.anchor_dist_m.forward | 11.73 / 16.35 | 12.69 / 13.42 | 12.43 / 12.99 |
| shape.attacking.anchor_dist_m.goalkeeper | 2.09 / 1.83 | 2.03 / 2.08 | 1.99 / 2.17 |
| shape.attacking.anchor_dist_m.midfielder | 13.95 / 15.78 | 15.16 / 16.16 | 15.60 / 15.69 |
| shape.attacking.anchor_dist_shape_m.defender | 6.42 / 33.01 | 7.51 / 9.38 | 9.82 / 8.55 |
| shape.attacking.anchor_dist_shape_m.forward | 7.91 / 15.98 | 9.47 / 10.38 | 9.16 / 9.69 |
| shape.attacking.anchor_dist_shape_m.midfielder | 8.21 / 14.51 | 10.64 / 10.73 | 10.48 / 10.74 |
| shape.attacking.bbox_area_m2 | 348.5 / 1260.4 | 320.6 / 317.5 | 318.6 / 332.4 |
| shape.attacking.bbox_depth_m | 23.02 / 39.69 | 22.66 / 20.90 | 21.87 / 22.74 |
| shape.attacking.bbox_width_m | 15.40 / 27.09 | 14.53 / 15.49 | 14.71 / 14.77 |
| shape.attacking.pct_of_open_play | 30.67 / 69.33 | 47.40 / 52.60 | 50.26 / 49.74 |
| shape.defending.anchor_dist_m.defender | 6.70 / 12.07 | 11.04 / 11.24 | 13.54 / 11.74 |
| shape.defending.anchor_dist_m.forward | 11.76 / 15.27 | 14.43 / 15.02 | 13.23 / 15.03 |
| shape.defending.anchor_dist_m.goalkeeper | 1.68 / 1.83 | 1.83 / 1.58 | 1.82 / 1.74 |
| shape.defending.anchor_dist_m.midfielder | 16.06 / 14.46 | 14.71 / 14.11 | 14.60 / 13.97 |
| shape.defending.anchor_dist_shape_m.defender | 4.55 / 7.96 | 7.78 / 7.40 | 9.23 / 8.52 |
| shape.defending.anchor_dist_shape_m.forward | 13.23 / 10.90 | 10.17 / 10.28 | 8.82 / 10.22 |
| shape.defending.anchor_dist_shape_m.midfielder | 11.77 / 9.94 | 9.56 / 9.42 | 9.94 / 9.73 |
| shape.defending.bbox_area_m2 | 506.2 / 323.6 | 344.1 / 335.5 | 351.4 / 334.1 |
| shape.defending.bbox_depth_m | 32.14 / 20.64 | 21.73 / 20.51 | 22.27 / 21.18 |
| shape.defending.bbox_width_m | 15.80 / 15.81 | 15.95 / 16.68 | 15.85 / 16.02 |
| shape.defending.line_x_std_m | 13.96 / 7.10 | 6.90 / 6.37 | 5.71 / 6.49 |
| shape.defending.line_x_variance_m2 | 244.9 / 67.17 | 64.75 / 52.35 | 44.28 / 57.39 |
| shape.defending.pct_of_open_play | 69.33 / 30.67 | 52.60 / 47.40 | 49.74 / 50.26 |
| shape.defending.pct_outfield_behind_ball | 80.49 / 56.09 | 52.74 / 61.33 | 58.83 / 58.20 |
| shape.defending.swarm2_pct | 0.13 / 0.20 | 0.38 / 0.03 | 0.73 / 0.11 |
| shape.defending.swarm3_pct | 0 / 0 | 0 / 0 | 0 / 0 |

GK scenario field: not present in the public state.

## 5v5 normal vs normal (10 min x 3 seeds from 1)

Command: `psfc-replay analyze --format 5v5 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty normal --away-difficulty normal`

Gameplay version: `0.1.0-5v5`

| seed | final state_hash | score (H-A) |
|---|---|---|
| 1 | `b5afe3999eea9054` | 10-9 |
| 2 | `80375dbc0f620401` | 8-8 |
| 3 | `3e9595d495247d8c` | 7-7 |

Match meta (mean over seeds): goals_total = 16.33, open_play_minutes = 10.01, open_play_pct_of_ticks = 91.85, shots_total = 80, ticks_simulated = 39221.3

### Match

| metric | home | away | teams mean |
|---|---:|---:|---:|
| fouls | 0.33 | 0 | **0.17** |
| goals | 8.33 | 8 | **8.17** |
| offsides | 1.33 | 0.33 | **0.83** |
| passes_completed_pct | 82.16 | 77.39 | **79.77** |
| possession_pct | 48.51 | 51.49 | **50** |
| shots | 42 | 38 | **40** |
| shots_on_target | 25.67 | 26.33 | **26** |

### Goalkeeper

| metric | home | away | teams mean |
|---|---:|---:|---:|
| catches | 1.33 | 3.67 | **2.50** |
| claims_non_save | 1.67 | 2.33 | **2** |
| excursion_mean_s | 0.99 | 0.91 | **0.95** |
| excursions_gt1s_per_10min | 9.00 | 8.33 | **8.66** |
| gathers | 3 | 6 | **4.50** |
| goals_conceded | 8 | 8.33 | **8.17** |
| goals_conceded_gk_gt3m_off_line | 6 | 5.67 | **5.83** |
| goals_conceded_no_shot_found | 0 | 0 | **0** |
| line_dist_at_conceded_shot_mean_m | 4.04 | 4.19 | **4.11** |
| line_dist_max_m | 9.67 | 9.82 | **9.74** |
| line_dist_mean_m | 3.43 | 3.42 | **3.42** |
| parries | 18 | 14.33 | **16.17** |
| pct_outside_6yd | 3.68 | 3.50 | **3.59** |
| pct_outside_box | 0 | 0 | **0** |
| rush_count | 22 | 23.67 | **22.83** |
| rush_per_10min | 21.99 | 23.65 | **22.82** |
| saves | 19.33 | 18 | **18.67** |
| tips_over | 0 | 0 | **0** |

### Defence and shape

| metric | home | away | teams mean |
|---|---:|---:|---:|
| attacking.anchor_dist_m.defender | 14.57 | 14.47 | **14.52** |
| attacking.anchor_dist_m.forward | 12.55 | 12.42 | **12.48** |
| attacking.anchor_dist_m.goalkeeper | 2.15 | 2.07 | **2.11** |
| attacking.anchor_dist_m.midfielder | 15.53 | 15.84 | **15.69** |
| attacking.anchor_dist_shape_m.defender | 8.15 | 8.28 | **8.21** |
| attacking.anchor_dist_shape_m.forward | 9.33 | 9.15 | **9.24** |
| attacking.anchor_dist_shape_m.midfielder | 9.86 | 10.49 | **10.18** |
| attacking.bbox_area_m2 | 327.0 | 335.9 | **331.4** |
| attacking.bbox_depth_m | 21.74 | 22.37 | **22.05** |
| attacking.bbox_width_m | 15.08 | 15.03 | **15.05** |
| attacking.pct_of_open_play | 49.01 | 50.99 | **50** |
| defending.anchor_dist_m.defender | 11.94 | 11.92 | **11.93** |
| defending.anchor_dist_m.forward | 13.12 | 12.78 | **12.95** |
| defending.anchor_dist_m.goalkeeper | 1.77 | 1.71 | **1.74** |
| defending.anchor_dist_m.midfielder | 14.37 | 14.10 | **14.24** |
| defending.anchor_dist_shape_m.defender | 7.46 | 7.17 | **7.32** |
| defending.anchor_dist_shape_m.forward | 8.86 | 8.48 | **8.67** |
| defending.anchor_dist_shape_m.midfielder | 9.59 | 9.27 | **9.43** |
| defending.bbox_area_m2 | 335.2 | 337.7 | **336.5** |
| defending.bbox_depth_m | 22.16 | 22.38 | **22.27** |
| defending.bbox_width_m | 15.29 | 15.31 | **15.30** |
| defending.line_x_std_m | 6.72 | 6.45 | **6.58** |
| defending.line_x_variance_m2 | 56.40 | 53.06 | **54.73** |
| defending.pct_of_open_play | 50.99 | 49.01 | **50** |
| defending.pct_outfield_behind_ball | 57.32 | 55.58 | **56.45** |
| defending.swarm2_pct | 0.92 | 0.94 | **0.93** |
| defending.swarm3_pct | 0 | 0 | **0** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 19.39 | 19.84 | **19.62** |
| accel_gt10_pct | 64.39 | 65.32 | **64.85** |
| accel_mean_mps2 | 19.43 | 19.89 | **19.66** |
| dspeed_max_mps_per_tick | 8.57 | 8.28 | **8.43** |
| heading_change_pct.0_10 | 97.31 | 97.23 | **97.27** |
| heading_change_pct.10_45 | 2.64 | 2.72 | **2.68** |
| heading_change_pct.45_90 | 0.03 | 0.04 | **0.04** |
| heading_change_pct.gt90 | 0.02 | 0.02 | **0.02** |
| lateral_accel_mean_mps2 | 11.89 | 12.28 | **12.08** |
| reversals_per_player_min | 0.46 | 0.50 | **0.48** |
| speed_oscillation_pct | 11.46 | 11.42 | **11.44** |
| turn_rate_mean_deg_per_s | 133.5 | 136.6 | **135.0** |

### Aerial

| metric | home | away | teams mean |
|---|---:|---:|---:|
| headers | 13.33 | 8.33 | **10.83** |
| headers_clear | 2 | 1 | **1.50** |
| headers_pass | 7.33 | 6.33 | **6.83** |
| headers_shot | 4 | 1 | **2.50** |
| jumps_gk | 32.67 | 35.33 | **34** |
| jumps_outfield | 30.33 | 30.33 | **30.33** |
| max_airborne_outfield | 1 | 1 | **1** |

### Per seed (home / away)

| metric | seed 1 | seed 2 | seed 3 |
|---|---:|---:|---:|
| aerial.headers | 13 / 9 | 7 / 9 | 20 / 7 |
| aerial.headers_clear | 1 / 1 | 3 / 1 | 2 / 1 |
| aerial.headers_pass | 6 / 7 | 2 / 6 | 14 / 6 |
| aerial.headers_shot | 6 / 1 | 2 / 2 | 4 / 0 |
| aerial.jumps_gk | 33 / 41 | 37 / 31 | 28 / 34 |
| aerial.jumps_outfield | 27 / 32 | 27 / 24 | 37 / 35 |
| aerial.max_airborne_outfield | 1 / 1 | 1 / 1 | 1 / 1 |
| gk.catches | 1 / 6 | 1 / 1 | 2 / 4 |
| gk.claims_non_save | 4 / 2 | 0 / 3 | 1 / 2 |
| gk.excursion_mean_s | 0.77 / 0.71 | 1.09 / 1.05 | 1.12 / 0.95 |
| gk.excursions_gt1s_per_10min | 5.99 / 5.99 | 10.99 / 8.99 | 10 / 10 |
| gk.gathers | 5 / 8 | 1 / 4 | 3 / 6 |
| gk.goals_conceded | 9 / 10 | 8 / 8 | 7 / 7 |
| gk.goals_conceded_gk_gt3m_off_line | 8 / 8 | 6 / 5 | 4 / 4 |
| gk.goals_conceded_no_shot_found | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.line_dist_at_conceded_shot_mean_m | 4.39 / 4.40 | 4.01 / 4.10 | 3.71 / 4.05 |
| gk.line_dist_max_m | 9.72 / 9.02 | 9.60 / 9.91 | 9.68 / 10.52 |
| gk.line_dist_mean_m | 3.39 / 3.43 | 3.42 / 3.40 | 3.48 / 3.42 |
| gk.parries | 14 / 21 | 21 / 10 | 19 / 12 |
| gk.pct_outside_6yd | 2.56 / 3.56 | 4.18 / 2.81 | 4.30 / 4.12 |
| gk.pct_outside_box | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_count | 20 / 30 | 23 / 16 | 23 / 25 |
| gk.rush_per_10min | 19.98 / 29.96 | 22.99 / 15.99 | 23 / 25 |
| gk.saves | 15 / 27 | 22 / 11 | 21 / 16 |
| gk.tips_over | 0 / 0 | 0 / 0 | 0 / 0 |
| match.fouls | 0 / 0 | 0 / 0 | 1 / 0 |
| match.goals | 10 / 9 | 8 / 8 | 7 / 7 |
| match.offsides | 1 / 0 | 1 / 1 | 2 / 0 |
| match.passes_completed_pct | 87.60 / 79.31 | 76.30 / 75.82 | 82.58 / 77.04 |
| match.possession_pct | 47.81 / 52.19 | 47.16 / 52.84 | 50.55 / 49.45 |
| match.shots | 51 / 39 | 37 / 42 | 38 / 33 |
| match.shots_on_target | 36 / 23 | 19 / 30 | 22 / 26 |
| move.accel_from_pos_mean_mps2 | 19.44 / 19.79 | 19.57 / 19.85 | 19.17 / 19.88 |
| move.accel_gt10_pct | 64.60 / 65.15 | 65.08 / 65.44 | 63.47 / 65.38 |
| move.accel_mean_mps2 | 19.48 / 19.82 | 19.64 / 19.92 | 19.18 / 19.92 |
| move.dspeed_max_mps_per_tick | 8.42 / 8.12 | 8.69 / 8.15 | 8.61 / 8.57 |
| move.heading_change_pct.0_10 | 97.38 / 97.32 | 97.31 / 97.37 | 97.24 / 97.00 |
| move.heading_change_pct.10_45 | 2.56 / 2.62 | 2.64 / 2.58 | 2.72 / 2.94 |
| move.heading_change_pct.45_90 | 0.04 / 0.04 | 0.03 / 0.04 | 0.03 / 0.04 |
| move.heading_change_pct.gt90 | 0.02 / 0.02 | 0.02 / 0.01 | 0.01 / 0.02 |
| move.lateral_accel_mean_mps2 | 11.90 / 12.29 | 12.08 / 12.47 | 11.69 / 12.07 |
| move.reversals_per_player_min | 0.50 / 0.56 | 0.54 / 0.44 | 0.34 / 0.50 |
| move.speed_oscillation_pct | 11.72 / 11.26 | 11.32 / 11.59 | 11.35 / 11.42 |
| move.turn_rate_mean_deg_per_s | 133.2 / 135.8 | 135.2 / 136.8 | 132.1 / 137.3 |
| shape.attacking.anchor_dist_m.defender | 14.40 / 15.13 | 14.59 / 14.98 | 14.71 / 13.29 |
| shape.attacking.anchor_dist_m.forward | 13.29 / 13.58 | 12.82 / 11.97 | 11.53 / 11.71 |
| shape.attacking.anchor_dist_m.goalkeeper | 2.07 / 2.14 | 2.26 / 2.02 | 2.11 / 2.04 |
| shape.attacking.anchor_dist_m.midfielder | 16.22 / 16.27 | 15.36 / 15.90 | 15.02 / 15.34 |
| shape.attacking.anchor_dist_shape_m.defender | 7.36 / 8.91 | 8.55 / 8.74 | 8.53 / 7.19 |
| shape.attacking.anchor_dist_shape_m.forward | 10.13 / 10.25 | 9.59 / 8.74 | 8.25 / 8.46 |
| shape.attacking.anchor_dist_shape_m.midfielder | 10.28 / 11.08 | 9.85 / 10.41 | 9.46 / 9.99 |
| shape.attacking.bbox_area_m2 | 321.3 / 327.3 | 314.0 / 329.5 | 345.7 / 350.8 |
| shape.attacking.bbox_depth_m | 21.61 / 21.79 | 21.24 / 21.83 | 22.38 / 23.49 |
| shape.attacking.bbox_width_m | 14.95 / 15.15 | 14.82 / 15.00 | 15.47 / 14.94 |
| shape.attacking.pct_of_open_play | 48.28 / 51.72 | 48.45 / 51.55 | 50.31 / 49.69 |
| shape.defending.anchor_dist_m.defender | 11.69 / 13.70 | 12.23 / 11.14 | 11.91 / 10.93 |
| shape.defending.anchor_dist_m.forward | 14.09 / 13.99 | 13.45 / 12.94 | 11.83 / 11.43 |
| shape.defending.anchor_dist_m.goalkeeper | 1.73 / 1.79 | 1.75 / 1.64 | 1.83 / 1.71 |
| shape.defending.anchor_dist_m.midfielder | 14.01 / 15.29 | 14.87 / 14.01 | 14.22 / 13.02 |
| shape.defending.anchor_dist_shape_m.defender | 6.99 / 9.26 | 8.17 / 5.85 | 7.24 / 6.41 |
| shape.defending.anchor_dist_shape_m.forward | 9.96 / 9.72 | 9.36 / 8.38 | 7.27 / 7.33 |
| shape.defending.anchor_dist_shape_m.midfielder | 9.07 / 10.53 | 10.30 / 8.87 | 9.40 / 8.39 |
| shape.defending.bbox_area_m2 | 331.1 / 324.1 | 318.3 / 337.6 | 356.2 / 351.4 |
| shape.defending.bbox_depth_m | 22.18 / 21.43 | 21.38 / 22.51 | 22.92 / 23.20 |
| shape.defending.bbox_width_m | 15.15 / 15.36 | 15.12 / 15.23 | 15.60 / 15.33 |
| shape.defending.line_x_std_m | 6.99 / 6.23 | 6.78 / 6.78 | 6.37 / 6.32 |
| shape.defending.line_x_variance_m2 | 61.98 / 48.94 | 57.81 / 58.65 | 49.42 / 51.58 |
| shape.defending.pct_of_open_play | 51.72 / 48.28 | 51.55 / 48.45 | 49.69 / 50.31 |
| shape.defending.pct_outfield_behind_ball | 58.80 / 54.91 | 55.12 / 56.78 | 58.04 / 55.06 |
| shape.defending.swarm2_pct | 0.41 / 0.74 | 1.50 / 1.69 | 0.86 / 0.38 |
| shape.defending.swarm3_pct | 0 / 0 | 0 / 0 | 0 / 0 |

GK scenario field: not present in the public state.

## 11v11 normal vs normal (10 min x 3 seeds from 1)

Command: `psfc-replay analyze --format 11v11 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty normal --away-difficulty normal`

Gameplay version: `0.1.0-5v5`

| seed | final state_hash | score (H-A) |
|---|---|---|
| 1 | `975188305fbd65fc` | 1-2 |
| 2 | `01bdf252ab4ec2b7` | 4-2 |
| 3 | `351670cf9ab3155e` | 6-4 |

Match meta (mean over seeds): goals_total = 6.33, open_play_minutes = 10, open_play_pct_of_ticks = 96.54, shots_total = 38.67, ticks_simulated = 37293.7

### Match

| metric | home | away | teams mean |
|---|---:|---:|---:|
| fouls | 0 | 0.33 | **0.17** |
| goals | 3.67 | 2.67 | **3.17** |
| offsides | 0 | 1 | **0.50** |
| passes_completed_pct | 81.70 | 77.53 | **79.62** |
| possession_pct | 55.62 | 44.38 | **50** |
| shots | 20 | 18.67 | **19.33** |
| shots_on_target | 11 | 9.33 | **10.17** |

### Goalkeeper

| metric | home | away | teams mean |
|---|---:|---:|---:|
| catches | 0.33 | 0.67 | **0.50** |
| claims_non_save | 2.67 | 1.67 | **2.17** |
| excursion_mean_s | 1.87 | 1.89 | **1.88** |
| excursions_gt1s_per_10min | 11 | 7.67 | **9.33** |
| gathers | 3 | 2.33 | **2.67** |
| goals_conceded | 2.67 | 3.67 | **3.17** |
| goals_conceded_gk_gt3m_off_line | 2 | 1.67 | **1.83** |
| goals_conceded_no_shot_found | 0 | 0 | **0** |
| line_dist_at_conceded_shot_mean_m | 5.34 | 3.63 | **4.49** |
| line_dist_max_m | 13.33 | 12.72 | **13.03** |
| line_dist_mean_m | 4.20 | 4.15 | **4.17** |
| parries | 6 | 6.33 | **6.17** |
| pct_outside_6yd | 4.97 | 3.55 | **4.26** |
| pct_outside_box | 0 | 0 | **0** |
| rush_count | 14.67 | 11.33 | **13** |
| rush_per_10min | 14.67 | 11.33 | **13** |
| saves | 6.33 | 7 | **6.67** |
| tips_over | 0 | 0 | **0** |

### Defence and shape

| metric | home | away | teams mean |
|---|---:|---:|---:|
| attacking.anchor_dist_m.defender | 19.72 | 17.58 | **18.65** |
| attacking.anchor_dist_m.forward | 17.70 | 20.26 | **18.98** |
| attacking.anchor_dist_m.goalkeeper | 2.33 | 2.11 | **2.22** |
| attacking.anchor_dist_m.midfielder | 18.14 | 17.82 | **17.98** |
| attacking.anchor_dist_shape_m.defender | 15.06 | 11.99 | **13.53** |
| attacking.anchor_dist_shape_m.forward | 12.99 | 14.47 | **13.73** |
| attacking.anchor_dist_shape_m.midfielder | 13.30 | 11.85 | **12.57** |
| attacking.bbox_area_m2 | 2285.8 | 1986.5 | **2136.1** |
| attacking.bbox_depth_m | 57.63 | 54.19 | **55.91** |
| attacking.bbox_width_m | 38.58 | 36.65 | **37.61** |
| attacking.pct_of_open_play | 53.56 | 46.44 | **50** |
| defending.anchor_dist_m.defender | 17.62 | 17.09 | **17.35** |
| defending.anchor_dist_m.forward | 18.17 | 18.84 | **18.50** |
| defending.anchor_dist_m.goalkeeper | 1.88 | 1.76 | **1.82** |
| defending.anchor_dist_m.midfielder | 17.38 | 15.89 | **16.64** |
| defending.anchor_dist_shape_m.defender | 12.78 | 12.21 | **12.49** |
| defending.anchor_dist_shape_m.forward | 12.45 | 13.33 | **12.89** |
| defending.anchor_dist_shape_m.midfielder | 12.21 | 10.54 | **11.38** |
| defending.bbox_area_m2 | 1848.3 | 1922.7 | **1885.5** |
| defending.bbox_depth_m | 53.25 | 55.77 | **54.51** |
| defending.bbox_width_m | 34.69 | 34.49 | **34.59** |
| defending.line_x_std_m | 11.68 | 11.17 | **11.43** |
| defending.line_x_variance_m2 | 152.7 | 143.6 | **148.1** |
| defending.pct_of_open_play | 46.44 | 53.56 | **50** |
| defending.pct_outfield_behind_ball | 63.46 | 68.21 | **65.83** |
| defending.swarm2_pct | 0.51 | 0.24 | **0.38** |
| defending.swarm3_pct | 0 | 0 | **0** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 13.92 | 14.11 | **14.02** |
| accel_gt10_pct | 53.20 | 53.60 | **53.40** |
| accel_mean_mps2 | 13.92 | 14.12 | **14.02** |
| dspeed_max_mps_per_tick | 7.11 | 7.43 | **7.27** |
| heading_change_pct.0_10 | 97.71 | 97.58 | **97.65** |
| heading_change_pct.10_45 | 2.27 | 2.40 | **2.33** |
| heading_change_pct.45_90 | 0.01 | 0.01 | **0.01** |
| heading_change_pct.gt90 | 0.01 | 0.01 | **0.01** |
| lateral_accel_mean_mps2 | 9.20 | 9.30 | **9.25** |
| reversals_per_player_min | 0.21 | 0.20 | **0.20** |
| speed_oscillation_pct | 7.04 | 7.63 | **7.33** |
| turn_rate_mean_deg_per_s | 118.0 | 119.4 | **118.7** |

### Aerial

| metric | home | away | teams mean |
|---|---:|---:|---:|
| headers | 25 | 19.67 | **22.33** |
| headers_clear | 0.33 | 0 | **0.17** |
| headers_pass | 22.67 | 18.33 | **20.50** |
| headers_shot | 2 | 1.33 | **1.67** |
| jumps_gk | 15.33 | 13.33 | **14.33** |
| jumps_outfield | 54.33 | 49.67 | **52** |
| max_airborne_outfield | 1 | 1 | **1** |

### Per seed (home / away)

| metric | seed 1 | seed 2 | seed 3 |
|---|---:|---:|---:|
| aerial.headers | 26 / 14 | 24 / 27 | 25 / 18 |
| aerial.headers_clear | 0 / 0 | 0 / 0 | 1 / 0 |
| aerial.headers_pass | 24 / 13 | 23 / 26 | 21 / 16 |
| aerial.headers_shot | 2 / 1 | 1 / 1 | 3 / 2 |
| aerial.jumps_gk | 16 / 9 | 17 / 11 | 13 / 20 |
| aerial.jumps_outfield | 54 / 49 | 55 / 55 | 54 / 45 |
| aerial.max_airborne_outfield | 1 / 1 | 1 / 1 | 1 / 1 |
| gk.catches | 0 / 1 | 1 / 0 | 0 / 1 |
| gk.claims_non_save | 1 / 3 | 6 / 1 | 1 / 1 |
| gk.excursion_mean_s | 1.88 / 1.81 | 2.00 / 1.97 | 1.73 / 1.88 |
| gk.excursions_gt1s_per_10min | 8 / 5 | 18 / 4 | 7 / 14 |
| gk.gathers | 1 / 4 | 7 / 1 | 1 / 2 |
| gk.goals_conceded | 2 / 1 | 2 / 4 | 4 / 6 |
| gk.goals_conceded_gk_gt3m_off_line | 2 / 1 | 1 / 3 | 3 / 1 |
| gk.goals_conceded_no_shot_found | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.line_dist_at_conceded_shot_mean_m | 4.66 / 3.57 | 4.42 / 4.41 | 6.94 / 2.92 |
| gk.line_dist_max_m | 12.97 / 12.76 | 13.21 / 12.87 | 13.82 / 12.52 |
| gk.line_dist_mean_m | 4.22 / 4.16 | 4.26 / 4.16 | 4.11 / 4.11 |
| gk.parries | 8 / 5 | 9 / 4 | 1 / 10 |
| gk.pct_outside_6yd | 4.07 / 3.02 | 7.66 / 2.29 | 3.17 / 5.33 |
| gk.pct_outside_box | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_count | 13 / 10 | 21 / 7 | 10 / 17 |
| gk.rush_per_10min | 13 / 10 | 21 / 7 | 10 / 17 |
| gk.saves | 8 / 6 | 10 / 4 | 1 / 11 |
| gk.tips_over | 0 / 0 | 0 / 0 | 0 / 0 |
| match.fouls | 0 / 1 | 0 / 0 | 0 / 0 |
| match.goals | 1 / 2 | 4 / 2 | 6 / 4 |
| match.offsides | 0 / 1 | 0 / 1 | 0 / 1 |
| match.passes_completed_pct | 84.29 / 73.91 | 76.16 / 79.89 | 84.66 / 78.80 |
| match.possession_pct | 66.96 / 33.04 | 50.92 / 49.08 | 48.97 / 51.03 |
| match.shots | 15 / 19 | 15 / 23 | 30 / 14 |
| match.shots_on_target | 7 / 10 | 8 / 12 | 18 / 6 |
| move.accel_from_pos_mean_mps2 | 12.18 / 12.63 | 14.68 / 14.61 | 14.90 / 15.09 |
| move.accel_gt10_pct | 46.46 / 47.87 | 56.08 / 55.48 | 57.06 / 57.45 |
| move.accel_mean_mps2 | 12.16 / 12.63 | 14.69 / 14.63 | 14.91 / 15.11 |
| move.dspeed_max_mps_per_tick | 7.47 / 7.30 | 6.90 / 6.67 | 6.95 / 8.31 |
| move.heading_change_pct.0_10 | 97.74 / 97.53 | 97.64 / 97.67 | 97.74 / 97.54 |
| move.heading_change_pct.10_45 | 2.23 / 2.45 | 2.34 / 2.31 | 2.24 / 2.44 |
| move.heading_change_pct.45_90 | 0.02 / 0.01 | 0.01 / 0.01 | 0.01 / 0.02 |
| move.heading_change_pct.gt90 | 0.01 / 0.01 | 0.01 / 0.01 | 0.01 / 0.00 |
| move.lateral_accel_mean_mps2 | 9.16 / 9.28 | 9.05 / 9.28 | 9.38 / 9.36 |
| move.reversals_per_player_min | 0.21 / 0.21 | 0.20 / 0.24 | 0.21 / 0.15 |
| move.speed_oscillation_pct | 7.06 / 8.99 | 6.88 / 6.98 | 7.17 / 6.90 |
| move.turn_rate_mean_deg_per_s | 116.4 / 119.2 | 117.5 / 118.3 | 120.1 / 120.8 |
| shape.attacking.anchor_dist_m.defender | 27.10 / 20.04 | 14.76 / 15.37 | 17.29 / 17.32 |
| shape.attacking.anchor_dist_m.forward | 19.57 / 24.73 | 14.96 / 17.73 | 18.57 / 18.33 |
| shape.attacking.anchor_dist_m.goalkeeper | 2.43 / 2.15 | 2.54 / 1.91 | 2.01 / 2.28 |
| shape.attacking.anchor_dist_m.midfielder | 22.00 / 23.53 | 13.82 / 14.34 | 18.60 / 15.59 |
| shape.attacking.anchor_dist_shape_m.defender | 23.96 / 16.34 | 9.39 / 8.72 | 11.84 / 10.91 |
| shape.attacking.anchor_dist_shape_m.forward | 16.95 / 18.42 | 9.50 / 12.69 | 12.53 / 12.29 |
| shape.attacking.anchor_dist_shape_m.midfielder | 19.02 / 17.35 | 8.68 / 8.54 | 12.18 / 9.66 |
| shape.attacking.bbox_area_m2 | 2893.4 / 2006.2 | 2010.6 / 1966.7 | 1953.3 / 1986.4 |
| shape.attacking.bbox_depth_m | 64.96 / 54.75 | 54.70 / 53.98 | 53.22 / 53.83 |
| shape.attacking.bbox_width_m | 42.34 / 36.64 | 36.71 / 36.41 | 36.69 / 36.90 |
| shape.attacking.pct_of_open_play | 60.34 / 39.66 | 50.86 / 49.14 | 49.48 / 50.52 |
| shape.defending.anchor_dist_m.defender | 21.92 / 19.79 | 14.37 / 14.51 | 16.56 / 16.98 |
| shape.defending.anchor_dist_m.forward | 21.65 / 20.95 | 14.19 / 17.45 | 18.67 / 18.12 |
| shape.defending.anchor_dist_m.goalkeeper | 1.90 / 1.76 | 1.90 / 1.74 | 1.83 / 1.79 |
| shape.defending.anchor_dist_m.midfielder | 21.50 / 18.71 | 13.13 / 13.50 | 17.50 / 15.47 |
| shape.defending.anchor_dist_shape_m.defender | 17.45 / 16.53 | 9.21 / 8.68 | 11.67 / 11.43 |
| shape.defending.anchor_dist_shape_m.forward | 16.38 / 15.66 | 8.50 / 12.17 | 12.48 / 12.14 |
| shape.defending.anchor_dist_shape_m.midfielder | 17.41 / 13.77 | 7.67 / 7.97 | 11.57 / 9.87 |
| shape.defending.bbox_area_m2 | 1856.2 / 2031.9 | 1850.7 / 1893.7 | 1837.9 / 1842.4 |
| shape.defending.bbox_depth_m | 53.54 / 59.48 | 53.36 / 54.50 | 52.84 / 53.35 |
| shape.defending.bbox_width_m | 34.61 / 34.23 | 34.67 / 34.75 | 34.79 / 34.47 |
| shape.defending.line_x_std_m | 12.05 / 9.24 | 10.90 / 11.88 | 12.09 / 12.40 |
| shape.defending.line_x_variance_m2 | 161.2 / 103.4 | 128.2 / 157.6 | 168.7 / 169.9 |
| shape.defending.pct_of_open_play | 39.66 / 60.34 | 49.14 / 50.86 | 50.52 / 49.48 |
| shape.defending.pct_outfield_behind_ball | 62.81 / 76.43 | 60.90 / 68.44 | 66.66 / 59.76 |
| shape.defending.swarm2_pct | 0.45 / 0.21 | 0.36 / 0.11 | 0.71 / 0.42 |
| shape.defending.swarm3_pct | 0 / 0 | 0 / 0 | 0 / 0 |

GK scenario field: not present in the public state.

## Metric definitions

- **aerial.headers (+ _shot/_clear/_pass)**: Outfielder with z > 0.14 m whose recent_kick_ticks rose versus the previous tick (a new contact); split by recent_kick_kind: 4/5 shot, 3 clearance/lob, else pass/flick.
- **aerial.jumps_outfield / aerial.jumps_gk**: Count of z going from <= 0 to > 0 between consecutive open-play ticks (outfielders / GK; GK dives that leave the ground count).
- **aerial.max_airborne_outfield**: Max number of the side's outfielders with z > 0 on one open-play tick.
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
- **move.heading_change_pct.<bucket>**: Per player per consecutive open-play tick pair with speed > 1 m/s on both ticks: |angle between vel_t-1 and vel_t| bucketed 0-10, 10-45, 45-90, >90 degrees; % of samples. All on-pitch players incl. GK.
- **move.lateral_accel_mean_mps2**: Mean |cross(vel_t-1, vel_t)| / |vel_t-1| * 60 over the same samples: velocity change perpendicular to the previous heading (m/s^2).
- **move.reversals_per_player_min**: Count of those samples with dot(vel_t, vel_t-1) < 0, divided by player-minutes (all consecutive open-play player-ticks / 3600).
- **move.speed_oscillation_pct**: Over three consecutive open-play ticks where both speed changes exceed 0.05 m/s: % where the speed change flips sign (speed up / slow down / speed up sawtooth). A smooth accelerate-cruise-decelerate profile scores near 0.
- **move.turn_rate_mean_deg_per_s**: Mean of the per-tick heading change (degrees) * 60 over the heading-histogram samples (speed > 1 m/s on both ticks).
- **shape.<phase>.anchor_dist_m.<role>**: Mean distance in metres between each player and psfc_sim::ai::world_anchor_for_player(state, side, idx, 0) (static formation slot, no shift) by PlayerRole.
- **shape.<phase>.anchor_dist_shape_m.<role>**: Same for outfielders after subtracting the outfield block's mean x-offset from anchors on that tick: measures loss of shape independent of the whole team pushing up / dropping as a unit.
- **shape.<phase>.bbox_area_m2 / bbox_depth_m / bbox_width_m**: Axis-aligned bounding box of on-pitch outfielders (GK excluded): depth = x-extent, width = y-extent, area = depth * width.
- **shape.<phase>.pct_of_open_play**: % of the side's open-play ticks classified in that phase.
- **shape.defending.line_x_variance_m2 / line_x_std_m**: Population variance (and std dev) of x of the back line while defending. Back line = outfielders with role Defender; if fewer than 2 (5v5 1-2-1 has one), the 2 outfielders with the deepest base_anchor_permille.x.
- **shape.defending.pct_outfield_behind_ball**: Mean % of the side's outfielders strictly goal-side of the ball ((x - ball.x) * attack_sign < 0) while defending.
- **shape.defending.swarm2_pct / swarm3_pct**: % of defending ticks where the opponent owns the ball (a carrier exists) with >= 2 (>= 3) of this side's outfielders within 1.6 m of the carrier.

