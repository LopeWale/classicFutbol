# Match analytics

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
| attacking.pct_of_open_play | 49.55 | 50.45 | **50** |
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
| defending.line_x_std_m | 5.28 | 5.54 | **5.41** |
| defending.line_x_variance_m2 | 43.77 | 48.25 | **46.01** |
| defending.pct_of_open_play | 50.45 | 49.55 | **50** |
| defending.pct_outfield_behind_ball | 60.58 | 62.22 | **61.40** |
| defending.swarm2_pct | 2.45 | 1.33 | **1.89** |
| defending.swarm3_pct | 0.03 | 0 | **0.02** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 11.33 | 11.36 | **11.34** |
| accel_gt10_pct | 51.67 | 52.14 | **51.91** |
| accel_mean_mps2 | 11.28 | 11.31 | **11.29** |
| dspeed_max_mps_per_tick | 7.45 | 7.84 | **7.64** |
| heading_change_pct.0_10 | 99.18 | 99.28 | **99.23** |
| heading_change_pct.10_45 | 0.75 | 0.65 | **0.70** |
| heading_change_pct.45_90 | 0.04 | 0.04 | **0.04** |
| heading_change_pct.gt90 | 0.02 | 0.03 | **0.02** |
| lateral_accel_mean_mps2 | 7.96 | 8.10 | **8.03** |
| reversals_per_player_min | 0.70 | 0.76 | **0.73** |
| speed_oscillation_pct | 4.22 | 4.26 | **4.24** |
| turn_rate_mean_deg_per_s | 102.2 | 101.3 | **101.8** |

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
| move.heading_change_pct.0_10 | 99.26 / 99.28 | 99.19 / 99.30 | 99.10 / 99.28 |
| move.heading_change_pct.10_45 | 0.69 / 0.66 | 0.74 / 0.63 | 0.82 / 0.66 |
| move.heading_change_pct.45_90 | 0.04 / 0.04 | 0.04 / 0.04 | 0.05 / 0.04 |
| move.heading_change_pct.gt90 | 0.01 / 0.02 | 0.02 / 0.03 | 0.03 / 0.02 |
| move.lateral_accel_mean_mps2 | 7.90 / 8.12 | 7.81 / 7.91 | 8.17 / 8.26 |
| move.reversals_per_player_min | 0.42 / 0.74 | 0.74 / 0.94 | 0.94 / 0.60 |
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
| shape.attacking.pct_of_open_play | 50.39 / 49.61 | 50.19 / 49.81 | 48.07 / 51.93 |
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
| shape.defending.line_x_std_m | 5.12 / 5.84 | 5.58 / 4.96 | 5.15 / 5.83 |
| shape.defending.line_x_variance_m2 | 41.18 / 54.15 | 49.30 / 39.21 | 40.83 / 51.40 |
| shape.defending.pct_of_open_play | 49.61 / 50.39 | 49.81 / 50.19 | 51.93 / 48.07 |
| shape.defending.pct_outfield_behind_ball | 60.21 / 62.76 | 61.98 / 60.57 | 59.54 / 63.32 |
| shape.defending.swarm2_pct | 1.84 / 0.57 | 2.04 / 1.82 | 3.47 / 1.61 |
| shape.defending.swarm3_pct | 0 / 0 | 0.09 / 0 | 0 / 0 |

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
| attacking.pct_of_open_play | 49.46 | 50.54 | **50** |
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
| defending.line_x_std_m | 5.27 | 5.11 | **5.19** |
| defending.line_x_variance_m2 | 42.98 | 40.47 | **41.72** |
| defending.pct_of_open_play | 50.54 | 49.46 | **50** |
| defending.pct_outfield_behind_ball | 62.47 | 62.76 | **62.62** |
| defending.swarm2_pct | 1.57 | 1.67 | **1.62** |
| defending.swarm3_pct | 0.02 | 0.03 | **0.03** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 11.13 | 11.09 | **11.11** |
| accel_gt10_pct | 51.17 | 50.90 | **51.04** |
| accel_mean_mps2 | 11.11 | 11.08 | **11.09** |
| dspeed_max_mps_per_tick | 7.76 | 7.71 | **7.73** |
| heading_change_pct.0_10 | 99.26 | 99.25 | **99.25** |
| heading_change_pct.10_45 | 0.68 | 0.70 | **0.69** |
| heading_change_pct.45_90 | 0.04 | 0.03 | **0.04** |
| heading_change_pct.gt90 | 0.02 | 0.02 | **0.02** |
| lateral_accel_mean_mps2 | 7.88 | 7.96 | **7.92** |
| reversals_per_player_min | 0.63 | 0.65 | **0.64** |
| speed_oscillation_pct | 4.10 | 4.11 | **4.10** |
| turn_rate_mean_deg_per_s | 99.65 | 100.2 | **99.90** |

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
| move.heading_change_pct.0_10 | 99.28 / 99.26 | 99.28 / 99.33 | 99.22 / 99.15 |
| move.heading_change_pct.10_45 | 0.65 / 0.68 | 0.67 / 0.63 | 0.71 / 0.78 |
| move.heading_change_pct.45_90 | 0.04 / 0.04 | 0.03 / 0.03 | 0.05 / 0.04 |
| move.heading_change_pct.gt90 | 0.02 / 0.02 | 0.02 / 0.01 | 0.02 / 0.03 |
| move.lateral_accel_mean_mps2 | 7.75 / 7.85 | 7.97 / 7.97 | 7.93 / 8.06 |
| move.reversals_per_player_min | 0.64 / 0.66 | 0.54 / 0.44 | 0.70 / 0.84 |
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
| shape.attacking.pct_of_open_play | 51.18 / 48.82 | 46.86 / 53.14 | 50.35 / 49.65 |
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
| shape.defending.line_x_std_m | 5.41 / 4.92 | 4.64 / 5.84 | 5.78 / 4.57 |
| shape.defending.line_x_variance_m2 | 45.77 / 39.38 | 34.01 / 49.65 | 49.16 / 32.38 |
| shape.defending.pct_of_open_play | 48.82 / 51.18 | 53.14 / 46.86 | 49.65 / 50.35 |
| shape.defending.pct_outfield_behind_ball | 60.93 / 63.15 | 59.45 / 63.32 | 67.04 / 61.82 |
| shape.defending.swarm2_pct | 1.27 / 2.30 | 1.15 / 0.92 | 2.29 / 1.78 |
| shape.defending.swarm3_pct | 0 / 0.10 | 0 / 0 | 0.07 / 0 |

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
| attacking.pct_of_open_play | 47.88 | 52.12 | **50** |
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
| defending.line_x_std_m | 6.62 | 6.71 | **6.67** |
| defending.line_x_variance_m2 | 70.88 | 66.58 | **68.73** |
| defending.pct_of_open_play | 52.12 | 47.88 | **50** |
| defending.pct_outfield_behind_ball | 65.95 | 69.18 | **67.57** |
| defending.swarm2_pct | 0.91 | 1.23 | **1.07** |
| defending.swarm3_pct | 0 | 0 | **0** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 7.89 | 7.88 | **7.88** |
| accel_gt10_pct | 36.31 | 36.26 | **36.29** |
| accel_mean_mps2 | 7.89 | 7.88 | **7.88** |
| dspeed_max_mps_per_tick | 7.24 | 8.33 | **7.78** |
| heading_change_pct.0_10 | 99.64 | 99.64 | **99.64** |
| heading_change_pct.10_45 | 0.35 | 0.34 | **0.34** |
| heading_change_pct.45_90 | 0.01 | 0.01 | **0.01** |
| heading_change_pct.gt90 | 0.01 | 0.01 | **0.01** |
| lateral_accel_mean_mps2 | 5.50 | 5.58 | **5.54** |
| reversals_per_player_min | 0.19 | 0.17 | **0.18** |
| speed_oscillation_pct | 3.08 | 3.08 | **3.08** |
| turn_rate_mean_deg_per_s | 76.72 | 77.29 | **77.00** |

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
| move.heading_change_pct.0_10 | 99.67 / 99.65 | 99.65 / 99.64 | 99.59 / 99.64 |
| move.heading_change_pct.10_45 | 0.32 / 0.33 | 0.33 / 0.34 | 0.39 / 0.34 |
| move.heading_change_pct.45_90 | 0.01 / 0.01 | 0.01 / 0.01 | 0.01 / 0.01 |
| move.heading_change_pct.gt90 | 0.00 / 0.01 | 0.01 / 0.01 | 0.01 / 0.00 |
| move.lateral_accel_mean_mps2 | 5.38 / 5.51 | 5.51 / 5.61 | 5.59 / 5.61 |
| move.reversals_per_player_min | 0.16 / 0.19 | 0.23 / 0.17 | 0.18 / 0.15 |
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
| shape.attacking.pct_of_open_play | 48.17 / 51.83 | 47.40 / 52.60 | 48.08 / 51.92 |
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
| shape.defending.line_x_std_m | 6.89 / 6.55 | 6.07 / 6.87 | 6.91 / 6.72 |
| shape.defending.line_x_variance_m2 | 79.02 / 65.69 | 58.92 / 68.55 | 74.69 / 65.50 |
| shape.defending.pct_of_open_play | 51.83 / 48.17 | 52.60 / 47.40 | 51.92 / 48.08 |
| shape.defending.pct_outfield_behind_ball | 68.51 / 67.22 | 65.50 / 69.53 | 63.83 / 70.80 |
| shape.defending.swarm2_pct | 1.06 / 0.52 | 0.91 / 0.51 | 0.74 / 2.65 |
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

