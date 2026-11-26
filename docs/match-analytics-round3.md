# Match analytics

## 5v5 hard vs hard (10 min x 3 seeds from 1)

Command: `psfc-replay analyze --format 5v5 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty hard --away-difficulty hard`

Gameplay version: `0.1.0-5v5`

| seed | final state_hash | score (H-A) |
|---|---|---|
| 1 | `a8046151fcae0c40` | 7-6 |
| 2 | `28be474f9e4b1909` | 3-5 |
| 3 | `7ca786e27f16cbfd` | 4-4 |

Match meta (mean over seeds): goals_total = 9.67, open_play_minutes = 10, open_play_pct_of_ticks = 94.79, shots_total = 53.33, ticks_simulated = 37977.7

### Match

| metric | home | away | teams mean |
|---|---:|---:|---:|
| fouls | 0 | 0 | **0** |
| goals | 4.67 | 5 | **4.83** |
| offsides | 4 | 1 | **2.50** |
| passes_completed_pct | 91.19 | 87.23 | **89.21** |
| possession_pct | 53.25 | 46.75 | **50** |
| shots | 31.67 | 21.67 | **26.67** |
| shots_on_target | 21.33 | 18 | **19.67** |

### Goalkeeper

| metric | home | away | teams mean |
|---|---:|---:|---:|
| catches | 2.33 | 1.67 | **2** |
| claims_non_save | 1 | 1.67 | **1.33** |
| excursion_mean_s | 0.17 | 0.52 | **0.35** |
| excursions_gt1s_per_10min | 0 | 0.33 | **0.17** |
| gathers | 3.33 | 3.33 | **3.33** |
| goals_conceded | 5 | 4.67 | **4.83** |
| goals_conceded_gk_gt3m_off_line | 4 | 3.67 | **3.83** |
| goals_conceded_no_shot_found | 0 | 0 | **0** |
| line_dist_at_conceded_shot_mean_m | 2.74 | 2.85 | **2.80** |
| line_dist_max_m | 5.49 | 6.69 | **6.09** |
| line_dist_mean_m | 3.05 | 3.01 | **3.03** |
| parries | 11 | 16 | **13.50** |
| pct_outside_6yd | 0.03 | 0.17 | **0.10** |
| pct_outside_box | 0 | 0 | **0** |
| rush_count | 0.33 | 1.33 | **0.83** |
| rush_per_10min | 0.33 | 1.33 | **0.83** |
| saves | 13.33 | 17.67 | **15.50** |
| tips_over | 0 | 0 | **0** |

### Defence and shape

| metric | home | away | teams mean |
|---|---:|---:|---:|
| attacking.anchor_dist_m.defender | 8.53 | 7.39 | **7.96** |
| attacking.anchor_dist_m.forward | 14.09 | 15.04 | **14.57** |
| attacking.anchor_dist_m.goalkeeper | 1.16 | 1.22 | **1.19** |
| attacking.anchor_dist_m.midfielder | 14.19 | 12.88 | **13.53** |
| attacking.anchor_dist_shape_m.defender | 5.93 | 5.76 | **5.84** |
| attacking.anchor_dist_shape_m.forward | 10.44 | 10.28 | **10.36** |
| attacking.anchor_dist_shape_m.midfielder | 7.60 | 6.79 | **7.19** |
| attacking.bbox_area_m2 | 390.9 | 393.0 | **391.9** |
| attacking.bbox_depth_m | 18.40 | 18.11 | **18.25** |
| attacking.bbox_width_m | 19.93 | 20.21 | **20.07** |
| attacking.length_m | 18.40 | 18.11 | **18.25** |
| attacking.nearest_mate_mean_m | 11.17 | 11.19 | **11.18** |
| attacking.nearest_mate_p10_m | 4.02 | 4.23 | **4.12** |
| attacking.pct_of_open_play | 50.05 | 49.95 | **50** |
| attacking.stretch_m | 9.90 | 9.88 | **9.89** |
| attacking.width_m | 19.93 | 20.21 | **20.07** |
| defending.anchor_dist_m.defender | 8.89 | 8.40 | **8.64** |
| defending.anchor_dist_m.forward | 13.62 | 15.44 | **14.53** |
| defending.anchor_dist_m.goalkeeper | 1.29 | 1.37 | **1.33** |
| defending.anchor_dist_m.midfielder | 12.69 | 11.82 | **12.25** |
| defending.anchor_dist_shape_m.defender | 7.51 | 8.46 | **7.98** |
| defending.anchor_dist_shape_m.forward | 9.31 | 9.94 | **9.62** |
| defending.anchor_dist_shape_m.midfielder | 6.17 | 5.61 | **5.89** |
| defending.bbox_area_m2 | 364.6 | 343.3 | **354.0** |
| defending.bbox_depth_m | 17.25 | 15.98 | **16.62** |
| defending.bbox_width_m | 19.82 | 19.89 | **19.86** |
| defending.length_m | 17.25 | 15.98 | **16.62** |
| defending.line_x_std_m | 5.71 | 5.20 | **5.45** |
| defending.line_x_variance_m2 | 46.70 | 41.44 | **44.07** |
| defending.nearest_mate_mean_m | 11.28 | 10.83 | **11.05** |
| defending.nearest_mate_p10_m | 5.19 | 4.84 | **5.02** |
| defending.pct_of_open_play | 49.95 | 50.05 | **50** |
| defending.pct_outfield_behind_ball | 66.78 | 64.09 | **65.44** |
| defending.stretch_m | 9.59 | 9.30 | **9.44** |
| defending.swarm2_pct | 0.54 | 1.24 | **0.89** |
| defending.swarm3_pct | 0 | 0 | **0** |
| defending.width_m | 19.82 | 19.89 | **19.86** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 10.92 | 11.05 | **10.98** |
| accel_gt10_pct | 49.43 | 49.73 | **49.58** |
| accel_mean_mps2 | 10.89 | 11.02 | **10.96** |
| dspeed_max_mps_per_tick | 3.63 | 3.70 | **3.67** |
| heading_change_p95_deg_per_tick_gt3mps | 3.43 | 3.52 | **3.47** |
| heading_change_pct.0_10 | 99.36 | 99.31 | **99.34** |
| heading_change_pct.10_45 | 0.62 | 0.67 | **0.64** |
| heading_change_pct.45_90 | 0.02 | 0.02 | **0.02** |
| heading_change_pct.gt90 | 0 | 0.00 | **0.00** |
| lateral_accel_mean_mps2 | 7.96 | 7.95 | **7.96** |
| reversals_per_player_min | 0 | 0.01 | **0.00** |
| side_flips_per_player_min | 83.46 | 81.96 | **82.71** |
| speed_oscillation_pct | 4.23 | 4.21 | **4.22** |
| turn_rate_mean_deg_per_s | 101.3 | 103.0 | **102.1** |

### Collisions and push-out

| metric | home | away | teams mean |
|---|---:|---:|---:|
| any.depth_max_m | 0.56 | 0.56 | **0.56** |
| any.depth_mean_m | 0.21 | 0.21 | **0.21** |
| any.events_per_min | 1.13 | 1.13 | **1.13** |
| any.stuck_mean_ticks | 0 | 0 | **0** |
| any.stuck_pairs | 0 | 0 | **0** |
| any.ticks_gt1cm_per_min | 2.07 | 2.07 | **2.07** |
| any.ticks_per_min | 2.47 | 2.47 | **2.47** |
| correction_vel_corrected_p50_mps | 0.63 | 0.65 | **0.64** |
| correction_vel_corrected_p99_mps | 12.77 | 10.41 | **11.59** |
| correction_vel_corrected_pct | 0.06 | 0.06 | **0.06** |
| correction_vel_max_mps | 12.89 | 10.60 | **11.74** |
| correction_vel_p50_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p95_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p99_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_pct_gt_0_3 | 0.04 | 0.04 | **0.04** |
| opponent.depth_max_m | 0.08 | 0.08 | **0.08** |
| opponent.depth_mean_m | 0.03 | 0.03 | **0.03** |
| opponent.events_per_min | 0.83 | 0.83 | **0.83** |
| opponent.stuck_mean_ticks | 0 | 0 | **0** |
| opponent.stuck_pairs | 0 | 0 | **0** |
| opponent.ticks_gt1cm_per_min | 0.73 | 0.73 | **0.73** |
| opponent.ticks_per_min | 1.10 | 1.10 | **1.10** |
| teammate.depth_max_m | 0.51 | 0.53 | **0.52** |
| teammate.depth_mean_m | 0.22 | 0.27 | **0.25** |
| teammate.events_per_min | 0.13 | 0.17 | **0.15** |
| teammate.stuck_mean_ticks | 0 | 0 | **0** |
| teammate.stuck_pairs | 0 | 0 | **0** |
| teammate.ticks_gt1cm_per_min | 0.77 | 1.33 | **1.05** |
| teammate.ticks_per_min | 0.77 | 1.37 | **1.07** |

### Coordination

| metric | home | away | teams mean |
|---|---:|---:|---:|
| cruise_over72_pct | 35.35 | 34.13 | **34.74** |
| dual_chaser_pct_of_loose | 15.13 | 17.65 | **16.39** |
| dual_chaser_ticks_per_min | 333.4 | 388.9 | **361.1** |
| engagers3_pct_of_carrier_ticks | 0.11 | 0.04 | **0.08** |
| engagers3_ticks_per_min | 0.73 | 0.33 | **0.53** |
| max_simultaneous_chasers | 4 | 4.33 | **4.17** |

### Aerial

| metric | home | away | teams mean |
|---|---:|---:|---:|
| headers | 23 | 18.33 | **20.67** |
| headers_clear | 1 | 2.33 | **1.67** |
| headers_pass | 17 | 12.67 | **14.83** |
| headers_shot | 5 | 3.33 | **4.17** |
| jumps_gk | 20.67 | 21.33 | **21** |
| jumps_outfield | 38 | 41.33 | **39.67** |
| max_airborne_outfield | 1 | 1 | **1** |

### Team shape vs envelope (teams mean)

| metric | in possession | out of possession | 11v11 envelope (m) |
|---|---:|---:|---|
| length_m | 18.25 | 16.62 | 31-46 |
| width_m | 20.07 | 19.86 | 35-48 |
| stretch_m | 9.89 | 9.44 | 7-16 |
| nearest_mate_mean_m | 11.18 | 11.05 | none published (in-possession mean near 9-10 m, estimate) |
| nearest_mate_p10_m | 4.12 | 5.02 | none published (in-possession mean near 9-10 m, estimate) |

Note: 5v5 has no published team-shape envelope; the 11v11 ranges are printed for reference only and do not apply to a 4-outfielder team on a small pitch.

### Per seed (home / away)

| metric | seed 1 | seed 2 | seed 3 |
|---|---:|---:|---:|
| aerial.headers | 20 / 18 | 26 / 18 | 23 / 19 |
| aerial.headers_clear | 1 / 2 | 0 / 3 | 2 / 2 |
| aerial.headers_pass | 17 / 13 | 20 / 12 | 14 / 13 |
| aerial.headers_shot | 2 / 3 | 6 / 3 | 7 / 4 |
| aerial.jumps_gk | 20 / 24 | 22 / 16 | 20 / 24 |
| aerial.jumps_outfield | 27 / 37 | 41 / 42 | 46 / 45 |
| aerial.max_airborne_outfield | 1 / 1 | 1 / 1 | 1 / 1 |
| collision.any.depth_max_m | 0.10 / 0.10 | 0.79 / 0.79 | 0.79 / 0.79 |
| collision.any.depth_mean_m | 0.04 / 0.04 | 0.30 / 0.30 | 0.29 / 0.29 |
| collision.any.events_per_min | 0.80 / 0.80 | 1.30 / 1.30 | 1.30 / 1.30 |
| collision.any.stuck_mean_ticks | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.any.stuck_pairs | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.any.ticks_gt1cm_per_min | 1 / 1 | 2.40 / 2.40 | 2.80 / 2.80 |
| collision.any.ticks_per_min | 1.30 / 1.30 | 3.10 / 3.10 | 3 / 3 |
| collision.correction_vel_corrected_p50_mps | 0.39 / 0.40 | 0.43 / 0.42 | 1.07 / 1.14 |
| collision.correction_vel_corrected_p99_mps | 12.09 / 14.89 | 13.10 / 3.60 | 13.12 / 12.74 |
| collision.correction_vel_corrected_pct | 0.05 / 0.05 | 0.06 / 0.07 | 0.05 / 0.06 |
| collision.correction_vel_max_mps | 12.09 / 14.89 | 13.45 / 3.60 | 13.12 / 13.32 |
| collision.correction_vel_p50_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p95_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p99_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_pct_gt_0_3 | 0.03 / 0.03 | 0.04 / 0.05 | 0.04 / 0.05 |
| collision.opponent.depth_max_m | 0.10 / 0.10 | 0.08 / 0.08 | 0.07 / 0.07 |
| collision.opponent.depth_mean_m | 0.04 / 0.04 | 0.02 / 0.02 | 0.03 / 0.03 |
| collision.opponent.events_per_min | 0.80 / 0.80 | 0.80 / 0.80 | 0.90 / 0.90 |
| collision.opponent.stuck_mean_ticks | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.stuck_pairs | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.ticks_gt1cm_per_min | 1 / 1 | 0.40 / 0.40 | 0.80 / 0.80 |
| collision.opponent.ticks_per_min | 1.30 / 1.30 | 1 / 1 | 1 / 1 |
| collision.teammate.depth_max_m | 0 / 0 | 0.77 / 0.79 | 0.75 / 0.79 |
| collision.teammate.depth_mean_m | 0 / 0 | 0.35 / 0.41 | 0.33 / 0.40 |
| collision.teammate.events_per_min | 0 / 0 | 0.20 / 0.30 | 0.20 / 0.20 |
| collision.teammate.stuck_mean_ticks | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.teammate.stuck_pairs | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.teammate.ticks_gt1cm_per_min | 0 / 0 | 1.20 / 2 | 1.10 / 2 |
| collision.teammate.ticks_per_min | 0 / 0 | 1.20 / 2.10 | 1.10 / 2 |
| coord.cruise_over72_pct | 34.44 / 34.63 | 35.30 / 34.13 | 36.33 / 33.62 |
| coord.dual_chaser_pct_of_loose | 15.26 / 16.83 | 15.81 / 17.30 | 14.32 / 18.83 |
| coord.dual_chaser_ticks_per_min | 332.2 / 366.5 | 352.1 / 385.2 | 315.8 / 415.1 |
| coord.engagers3_pct_of_carrier_ticks | 0 / 0 | 0.17 / 0 | 0.18 / 0.13 |
| coord.engagers3_ticks_per_min | 0 / 0 | 1.10 / 0 | 1.10 / 1 |
| coord.max_simultaneous_chasers | 4 / 4 | 4 / 4 | 4 / 5 |
| gk.catches | 2 / 0 | 2 / 0 | 3 / 5 |
| gk.claims_non_save | 1 / 2 | 1 / 2 | 1 / 1 |
| gk.excursion_mean_s | 0.50 / 0 | 0 / 0.85 | 0 / 0.72 |
| gk.excursions_gt1s_per_10min | 0 / 0 | 0 / 0 | 0 / 1 |
| gk.gathers | 3 / 2 | 3 / 2 | 4 / 6 |
| gk.goals_conceded | 6 / 7 | 5 / 3 | 4 / 4 |
| gk.goals_conceded_gk_gt3m_off_line | 5 / 6 | 3 / 2 | 4 / 3 |
| gk.goals_conceded_no_shot_found | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.line_dist_at_conceded_shot_mean_m | 2.82 / 3.01 | 2.25 / 2.82 | 3.15 / 2.71 |
| gk.line_dist_max_m | 5.83 / 5.01 | 5.46 / 7.57 | 5.19 / 7.47 |
| gk.line_dist_mean_m | 3.06 / 3.05 | 3.02 / 3.02 | 3.07 / 2.96 |
| gk.parries | 9 / 14 | 13 / 15 | 11 / 19 |
| gk.pct_outside_6yd | 0.08 / 0 | 0 / 0.14 | 0 / 0.36 |
| gk.pct_outside_box | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_count | 1 / 0 | 0 / 1 | 0 / 3 |
| gk.rush_per_10min | 1 / 0 | 0 / 1 | 0 / 3 |
| gk.saves | 11 / 14 | 15 / 15 | 14 / 24 |
| gk.tips_over | 0 / 0 | 0 / 0 | 0 / 0 |
| match.fouls | 0 / 0 | 0 / 0 | 0 / 0 |
| match.goals | 7 / 6 | 3 / 5 | 4 / 4 |
| match.offsides | 5 / 1 | 4 / 2 | 3 / 0 |
| match.passes_completed_pct | 89.38 / 88.60 | 92.47 / 87.98 | 91.71 / 85.12 |
| match.possession_pct | 52.14 / 47.86 | 52.67 / 47.33 | 54.95 / 45.05 |
| match.shots | 29 / 20 | 29 / 24 | 37 / 21 |
| match.shots_on_target | 21 / 16 | 18 / 20 | 25 / 18 |
| move.accel_from_pos_mean_mps2 | 10.88 / 11.01 | 11.11 / 11.16 | 10.76 / 10.97 |
| move.accel_gt10_pct | 49.56 / 49.48 | 50.35 / 50.58 | 48.39 / 49.14 |
| move.accel_mean_mps2 | 10.86 / 10.97 | 11.08 / 11.15 | 10.72 / 10.95 |
| move.dspeed_max_mps_per_tick | 3.81 / 3.65 | 3.67 / 3.58 | 3.41 / 3.88 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.40 / 3.52 | 3.47 / 3.53 | 3.41 / 3.52 |
| move.heading_change_pct.0_10 | 99.40 / 99.32 | 99.32 / 99.36 | 99.37 / 99.25 |
| move.heading_change_pct.10_45 | 0.58 / 0.67 | 0.67 / 0.62 | 0.61 / 0.72 |
| move.heading_change_pct.45_90 | 0.02 / 0.01 | 0.01 / 0.02 | 0.02 / 0.02 |
| move.heading_change_pct.gt90 | 0 / 0 | 0 / 0.00 | 0 / 0 |
| move.lateral_accel_mean_mps2 | 8.01 / 7.95 | 8.03 / 8.12 | 7.83 / 7.78 |
| move.reversals_per_player_min | 0 / 0 | 0 / 0.02 | 0 / 0 |
| move.side_flips_per_player_min | 84.81 / 78.79 | 83.43 / 81.91 | 82.15 / 85.19 |
| move.speed_oscillation_pct | 4.24 / 4.13 | 4.27 / 4.29 | 4.18 / 4.21 |
| move.turn_rate_mean_deg_per_s | 101.3 / 102.9 | 102.9 / 103.4 | 99.54 / 102.6 |
| shape.attacking.anchor_dist_m.defender | 7.35 / 7.13 | 8.30 / 8.03 | 9.94 / 7.02 |
| shape.attacking.anchor_dist_m.forward | 13.44 / 14.68 | 15.41 / 14.04 | 13.43 / 16.40 |
| shape.attacking.anchor_dist_m.goalkeeper | 1.14 / 1.18 | 1.20 / 1.19 | 1.15 / 1.28 |
| shape.attacking.anchor_dist_m.midfielder | 13.02 / 12.84 | 14.57 / 13.36 | 14.97 / 12.45 |
| shape.attacking.anchor_dist_shape_m.defender | 5.27 / 5.84 | 5.89 / 5.63 | 6.62 / 5.82 |
| shape.attacking.anchor_dist_shape_m.forward | 9.99 / 10.11 | 10.96 / 10.18 | 10.36 / 10.55 |
| shape.attacking.anchor_dist_shape_m.midfielder | 6.93 / 6.71 | 7.88 / 7.07 | 7.98 / 6.59 |
| shape.attacking.bbox_area_m2 | 404.9 / 379.0 | 391.1 / 397.7 | 376.5 / 402.4 |
| shape.attacking.bbox_depth_m | 19.06 / 17.92 | 17.83 / 18.71 | 18.30 / 17.70 |
| shape.attacking.bbox_width_m | 20.12 / 19.63 | 20.64 / 19.93 | 19.02 / 21.08 |
| shape.attacking.length_m | 19.06 / 17.92 | 17.83 / 18.71 | 18.30 / 17.70 |
| shape.attacking.nearest_mate_mean_m | 11.46 / 10.99 | 11.20 / 11.21 | 10.84 / 11.37 |
| shape.attacking.nearest_mate_p10_m | 4.51 / 4.19 | 4.14 / 4.08 | 3.41 / 4.40 |
| shape.attacking.pct_of_open_play | 47.98 / 52.02 | 51.93 / 48.07 | 50.25 / 49.75 |
| shape.attacking.stretch_m | 10.08 / 9.70 | 9.93 / 9.98 | 9.69 / 9.96 |
| shape.attacking.width_m | 20.12 / 19.63 | 20.64 / 19.93 | 19.02 / 21.08 |
| shape.defending.anchor_dist_m.defender | 8.51 / 7.86 | 8.78 / 8.55 | 9.38 / 8.78 |
| shape.defending.anchor_dist_m.forward | 13.81 / 14.46 | 14.39 / 15.03 | 12.66 / 16.84 |
| shape.defending.anchor_dist_m.goalkeeper | 1.30 / 1.29 | 1.32 / 1.35 | 1.25 / 1.46 |
| shape.defending.anchor_dist_m.midfielder | 12.47 / 11.13 | 12.33 / 12.21 | 13.26 / 12.13 |
| shape.defending.anchor_dist_shape_m.defender | 7.31 / 7.92 | 8.00 / 8.15 | 7.22 / 9.30 |
| shape.defending.anchor_dist_shape_m.forward | 9.44 / 9.53 | 9.57 / 9.71 | 8.91 / 10.58 |
| shape.defending.anchor_dist_shape_m.midfielder | 5.93 / 5.32 | 6.00 / 5.79 | 6.58 / 5.72 |
| shape.defending.bbox_area_m2 | 352.1 / 354.3 | 350.0 / 358.1 | 391.8 / 317.6 |
| shape.defending.bbox_depth_m | 17.38 / 16.55 | 16.26 / 16.36 | 18.12 / 15.03 |
| shape.defending.bbox_width_m | 19.14 / 20.01 | 20.02 / 20.08 | 20.30 / 19.58 |
| shape.defending.length_m | 17.38 / 16.55 | 16.26 / 16.36 | 18.12 / 15.03 |
| shape.defending.line_x_std_m | 5.69 / 5.30 | 5.36 / 5.41 | 6.08 / 4.90 |
| shape.defending.line_x_variance_m2 | 47.14 / 41.22 | 42.54 / 44.52 | 50.41 / 38.59 |
| shape.defending.nearest_mate_mean_m | 11.06 / 11.12 | 11.03 / 11.06 | 11.74 / 10.30 |
| shape.defending.nearest_mate_p10_m | 5.34 / 5.41 | 4.93 / 4.91 | 5.31 / 4.21 |
| shape.defending.pct_of_open_play | 52.02 / 47.98 | 48.07 / 51.93 | 49.75 / 50.25 |
| shape.defending.pct_outfield_behind_ball | 67.44 / 62.75 | 64.22 / 66.75 | 68.68 / 62.79 |
| shape.defending.stretch_m | 9.48 / 9.43 | 9.37 / 9.44 | 9.91 / 9.02 |
| shape.defending.swarm2_pct | 0 / 0.97 | 0.51 / 0.88 | 1.10 / 1.85 |
| shape.defending.swarm3_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| shape.defending.width_m | 19.14 / 20.01 | 20.02 / 20.08 | 20.30 / 19.58 |

GK scenario field: not present in the public state.

## 5v5 normal vs normal (10 min x 3 seeds from 1)

Command: `psfc-replay analyze --format 5v5 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty normal --away-difficulty normal`

Gameplay version: `0.1.0-5v5`

| seed | final state_hash | score (H-A) |
|---|---|---|
| 1 | `bd5db4a18696394f` | 4-8 |
| 2 | `0c5aa98f243171cd` | 6-1 |
| 3 | `c1d8157032848063` | 3-4 |

Match meta (mean over seeds): goals_total = 8.67, open_play_minutes = 10, open_play_pct_of_ticks = 94.74, shots_total = 52.33, ticks_simulated = 37999.7

### Match

| metric | home | away | teams mean |
|---|---:|---:|---:|
| fouls | 0 | 0 | **0** |
| goals | 4.33 | 4.33 | **4.33** |
| offsides | 1.67 | 1.33 | **1.50** |
| passes_completed_pct | 84.53 | 84.60 | **84.56** |
| possession_pct | 49.80 | 50.20 | **50** |
| shots | 23.67 | 28.67 | **26.17** |
| shots_on_target | 18.33 | 23.67 | **21** |

### Goalkeeper

| metric | home | away | teams mean |
|---|---:|---:|---:|
| catches | 2.33 | 1.67 | **2** |
| claims_non_save | 0.67 | 0.33 | **0.50** |
| excursion_mean_s | 0.38 | 0.44 | **0.41** |
| excursions_gt1s_per_10min | 0.33 | 0.33 | **0.33** |
| gathers | 3 | 1.67 | **2.33** |
| goals_conceded | 4.33 | 4.33 | **4.33** |
| goals_conceded_gk_gt3m_off_line | 2.67 | 3 | **2.83** |
| goals_conceded_no_shot_found | 0 | 0 | **0** |
| line_dist_at_conceded_shot_mean_m | 3.08 | 2.79 | **2.93** |
| line_dist_max_m | 5.40 | 6.64 | **6.02** |
| line_dist_mean_m | 2.94 | 3.02 | **2.98** |
| parries | 17.33 | 12.33 | **14.83** |
| pct_outside_6yd | 0.06 | 0.07 | **0.07** |
| pct_outside_box | 0 | 0 | **0** |
| rush_count | 0.33 | 0.67 | **0.50** |
| rush_per_10min | 0.33 | 0.67 | **0.50** |
| saves | 19.67 | 14 | **16.83** |
| tips_over | 0 | 0 | **0** |

### Defence and shape

| metric | home | away | teams mean |
|---|---:|---:|---:|
| attacking.anchor_dist_m.defender | 8.63 | 9.44 | **9.03** |
| attacking.anchor_dist_m.forward | 15.38 | 14.94 | **15.16** |
| attacking.anchor_dist_m.goalkeeper | 1.25 | 1.21 | **1.23** |
| attacking.anchor_dist_m.midfielder | 14.99 | 16.51 | **15.75** |
| attacking.anchor_dist_shape_m.defender | 6.39 | 6.53 | **6.46** |
| attacking.anchor_dist_shape_m.forward | 10.78 | 10.86 | **10.82** |
| attacking.anchor_dist_shape_m.midfielder | 8.52 | 10.09 | **9.30** |
| attacking.bbox_area_m2 | 364.8 | 384.6 | **374.7** |
| attacking.bbox_depth_m | 17.78 | 17.80 | **17.79** |
| attacking.bbox_width_m | 19.20 | 20.19 | **19.70** |
| attacking.length_m | 17.78 | 17.80 | **17.79** |
| attacking.nearest_mate_mean_m | 10.73 | 11.03 | **10.88** |
| attacking.nearest_mate_p10_m | 3.74 | 3.85 | **3.80** |
| attacking.pct_of_open_play | 50.47 | 49.53 | **50** |
| attacking.stretch_m | 9.58 | 9.87 | **9.73** |
| attacking.width_m | 19.20 | 20.19 | **19.70** |
| defending.anchor_dist_m.defender | 9.60 | 9.72 | **9.66** |
| defending.anchor_dist_m.forward | 15.68 | 15.31 | **15.49** |
| defending.anchor_dist_m.goalkeeper | 1.43 | 1.39 | **1.41** |
| defending.anchor_dist_m.midfielder | 13.95 | 15.01 | **14.48** |
| defending.anchor_dist_shape_m.defender | 8.60 | 8.46 | **8.53** |
| defending.anchor_dist_shape_m.forward | 10.30 | 10.10 | **10.20** |
| defending.anchor_dist_shape_m.midfielder | 7.59 | 9.00 | **8.29** |
| defending.bbox_area_m2 | 336.1 | 370.2 | **353.2** |
| defending.bbox_depth_m | 16.03 | 16.85 | **16.44** |
| defending.bbox_width_m | 19.42 | 20.34 | **19.88** |
| defending.length_m | 16.03 | 16.85 | **16.44** |
| defending.line_x_std_m | 5.27 | 5.66 | **5.47** |
| defending.line_x_variance_m2 | 41.83 | 47.81 | **44.82** |
| defending.nearest_mate_mean_m | 10.55 | 11.09 | **10.82** |
| defending.nearest_mate_p10_m | 4.34 | 4.53 | **4.44** |
| defending.pct_of_open_play | 49.53 | 50.47 | **50** |
| defending.pct_outfield_behind_ball | 61.32 | 63.14 | **62.23** |
| defending.stretch_m | 9.21 | 9.64 | **9.42** |
| defending.swarm2_pct | 0.54 | 1.05 | **0.80** |
| defending.swarm3_pct | 0.01 | 0.03 | **0.02** |
| defending.width_m | 19.42 | 20.34 | **19.88** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 10.80 | 10.74 | **10.77** |
| accel_gt10_pct | 48.83 | 48.40 | **48.62** |
| accel_mean_mps2 | 10.78 | 10.73 | **10.75** |
| dspeed_max_mps_per_tick | 3.57 | 3.80 | **3.68** |
| heading_change_p95_deg_per_tick_gt3mps | 3.46 | 3.47 | **3.47** |
| heading_change_pct.0_10 | 99.37 | 99.39 | **99.38** |
| heading_change_pct.10_45 | 0.61 | 0.60 | **0.60** |
| heading_change_pct.45_90 | 0.02 | 0.02 | **0.02** |
| heading_change_pct.gt90 | 0.00 | 0.00 | **0.00** |
| lateral_accel_mean_mps2 | 7.81 | 7.81 | **7.81** |
| reversals_per_player_min | 0.02 | 0.01 | **0.01** |
| side_flips_per_player_min | 85.47 | 83.04 | **84.26** |
| speed_oscillation_pct | 4.27 | 4.27 | **4.27** |
| turn_rate_mean_deg_per_s | 101.1 | 99.44 | **100.3** |

### Collisions and push-out

| metric | home | away | teams mean |
|---|---:|---:|---:|
| any.depth_max_m | 0.79 | 0.79 | **0.79** |
| any.depth_mean_m | 0.29 | 0.29 | **0.29** |
| any.events_per_min | 1.50 | 1.50 | **1.50** |
| any.stuck_mean_ticks | 10 | 10 | **10** |
| any.stuck_pairs | 1.67 | 1.67 | **1.67** |
| any.ticks_gt1cm_per_min | 4.30 | 4.30 | **4.30** |
| any.ticks_per_min | 4.57 | 4.57 | **4.57** |
| correction_vel_corrected_p50_mps | 0.78 | 0.86 | **0.82** |
| correction_vel_corrected_p99_mps | 12.18 | 7.23 | **9.70** |
| correction_vel_corrected_pct | 0.07 | 0.07 | **0.07** |
| correction_vel_max_mps | 12.86 | 10.50 | **11.68** |
| correction_vel_p50_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p95_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p99_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_pct_gt_0_3 | 0.05 | 0.05 | **0.05** |
| opponent.depth_max_m | 0.07 | 0.07 | **0.07** |
| opponent.depth_mean_m | 0.03 | 0.03 | **0.03** |
| opponent.events_per_min | 1 | 1 | **1** |
| opponent.stuck_mean_ticks | 0 | 0 | **0** |
| opponent.stuck_pairs | 0 | 0 | **0** |
| opponent.ticks_gt1cm_per_min | 1.10 | 1.10 | **1.10** |
| opponent.ticks_per_min | 1.33 | 1.33 | **1.33** |
| teammate.depth_max_m | 0.79 | 0.76 | **0.77** |
| teammate.depth_mean_m | 0.40 | 0.37 | **0.38** |
| teammate.events_per_min | 0.23 | 0.27 | **0.25** |
| teammate.stuck_mean_ticks | 5 | 10 | **7.50** |
| teammate.stuck_pairs | 0.67 | 1 | **0.83** |
| teammate.ticks_gt1cm_per_min | 2.23 | 2.30 | **2.27** |
| teammate.ticks_per_min | 2.27 | 2.30 | **2.28** |

### Coordination

| metric | home | away | teams mean |
|---|---:|---:|---:|
| cruise_over72_pct | 34.93 | 34.78 | **34.85** |
| dual_chaser_pct_of_loose | 20.70 | 17.61 | **19.16** |
| dual_chaser_ticks_per_min | 451.6 | 384.9 | **418.3** |
| engagers3_pct_of_carrier_ticks | 0.16 | 0.19 | **0.17** |
| engagers3_ticks_per_min | 1.07 | 1.30 | **1.18** |
| max_simultaneous_chasers | 4.33 | 4 | **4.17** |

### Aerial

| metric | home | away | teams mean |
|---|---:|---:|---:|
| headers | 19 | 18.67 | **18.83** |
| headers_clear | 4 | 0.33 | **2.17** |
| headers_pass | 11.33 | 14.67 | **13** |
| headers_shot | 3.67 | 3.67 | **3.67** |
| jumps_gk | 23.33 | 19.67 | **21.50** |
| jumps_outfield | 38.33 | 36.33 | **37.33** |
| max_airborne_outfield | 1 | 1 | **1** |

### Team shape vs envelope (teams mean)

| metric | in possession | out of possession | 11v11 envelope (m) |
|---|---:|---:|---|
| length_m | 17.79 | 16.44 | 31-46 |
| width_m | 19.70 | 19.88 | 35-48 |
| stretch_m | 9.73 | 9.42 | 7-16 |
| nearest_mate_mean_m | 10.88 | 10.82 | none published (in-possession mean near 9-10 m, estimate) |
| nearest_mate_p10_m | 3.80 | 4.44 | none published (in-possession mean near 9-10 m, estimate) |

Note: 5v5 has no published team-shape envelope; the 11v11 ranges are printed for reference only and do not apply to a 4-outfielder team on a small pitch.

### Per seed (home / away)

| metric | seed 1 | seed 2 | seed 3 |
|---|---:|---:|---:|
| aerial.headers | 13 / 15 | 25 / 20 | 19 / 21 |
| aerial.headers_clear | 4 / 0 | 4 / 0 | 4 / 1 |
| aerial.headers_pass | 7 / 12 | 16 / 17 | 11 / 15 |
| aerial.headers_shot | 2 / 3 | 5 / 3 | 4 / 5 |
| aerial.jumps_gk | 25 / 16 | 19 / 22 | 26 / 21 |
| aerial.jumps_outfield | 32 / 30 | 43 / 34 | 40 / 45 |
| aerial.max_airborne_outfield | 1 / 1 | 1 / 1 | 1 / 1 |
| collision.any.depth_max_m | 0.78 / 0.78 | 0.79 / 0.79 | 0.79 / 0.79 |
| collision.any.depth_mean_m | 0.28 / 0.28 | 0.32 / 0.32 | 0.27 / 0.27 |
| collision.any.events_per_min | 0.90 / 0.90 | 1.60 / 1.60 | 2 / 2 |
| collision.any.stuck_mean_ticks | 0 / 0 | 15 / 15 | 15 / 15 |
| collision.any.stuck_pairs | 0 / 0 | 2 / 2 | 3 / 3 |
| collision.any.ticks_gt1cm_per_min | 1.80 / 1.80 | 5.20 / 5.20 | 5.90 / 5.90 |
| collision.any.ticks_per_min | 1.90 / 1.90 | 5.30 / 5.30 | 6.50 / 6.50 |
| collision.correction_vel_corrected_p50_mps | 0.90 / 0.49 | 0.72 / 1.02 | 0.71 / 1.07 |
| collision.correction_vel_corrected_p99_mps | 13.79 / 12.79 | 11.03 / 5.29 | 11.70 / 3.60 |
| collision.correction_vel_corrected_pct | 0.05 / 0.05 | 0.06 / 0.09 | 0.09 / 0.07 |
| collision.correction_vel_max_mps | 13.79 / 12.79 | 11.86 / 5.39 | 12.93 / 13.32 |
| collision.correction_vel_p50_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p95_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p99_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_pct_gt_0_3 | 0.04 / 0.04 | 0.05 / 0.06 | 0.07 / 0.05 |
| collision.opponent.depth_max_m | 0.05 / 0.05 | 0.07 / 0.07 | 0.10 / 0.10 |
| collision.opponent.depth_mean_m | 0.03 / 0.03 | 0.02 / 0.02 | 0.04 / 0.04 |
| collision.opponent.events_per_min | 0.70 / 0.70 | 0.90 / 0.90 | 1.40 / 1.40 |
| collision.opponent.stuck_mean_ticks | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.stuck_pairs | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.ticks_gt1cm_per_min | 0.80 / 0.80 | 1 / 1 | 1.50 / 1.50 |
| collision.opponent.ticks_per_min | 0.90 / 0.90 | 1.10 / 1.10 | 2 / 2 |
| collision.teammate.depth_max_m | 0.78 / 0.72 | 0.79 / 0.77 | 0.79 / 0.79 |
| collision.teammate.depth_mean_m | 0.44 / 0.41 | 0.42 / 0.34 | 0.35 / 0.35 |
| collision.teammate.events_per_min | 0.10 / 0.10 | 0.30 / 0.40 | 0.30 / 0.30 |
| collision.teammate.stuck_mean_ticks | 0 / 0 | 0 / 15 | 15 / 15 |
| collision.teammate.stuck_pairs | 0 / 0 | 0 / 2 | 2 / 1 |
| collision.teammate.ticks_gt1cm_per_min | 1 / 0.50 | 2.20 / 3.80 | 3.50 / 2.60 |
| collision.teammate.ticks_per_min | 1 / 0.50 | 2.20 / 3.80 | 3.60 / 2.60 |
| coord.cruise_over72_pct | 34.44 / 34.47 | 34.66 / 34.16 | 35.69 / 35.71 |
| coord.dual_chaser_pct_of_loose | 21.82 / 16.31 | 19.50 / 18.97 | 20.79 / 17.57 |
| coord.dual_chaser_ticks_per_min | 459.4 / 343.4 | 427.9 / 416.2 | 467.6 / 395.1 |
| coord.engagers3_pct_of_carrier_ticks | 0.05 / 0.15 | 0.38 / 0.16 | 0.04 / 0.25 |
| coord.engagers3_ticks_per_min | 0.40 / 1.10 | 2.50 / 1.20 | 0.30 / 1.60 |
| coord.max_simultaneous_chasers | 5 / 4 | 4 / 4 | 4 / 4 |
| gk.catches | 2 / 1 | 3 / 1 | 2 / 3 |
| gk.claims_non_save | 1 / 0 | 1 / 1 | 0 / 0 |
| gk.excursion_mean_s | 0 / 1.23 | 1.13 / 0.08 | 0 / 0 |
| gk.excursions_gt1s_per_10min | 0 / 1 | 1 / 0 | 0 / 0 |
| gk.gathers | 3 / 1 | 4 / 2 | 2 / 2 |
| gk.goals_conceded | 8 / 4 | 1 / 6 | 4 / 3 |
| gk.goals_conceded_gk_gt3m_off_line | 6 / 2 | 1 / 4 | 1 / 3 |
| gk.goals_conceded_no_shot_found | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.line_dist_at_conceded_shot_mean_m | 3.02 / 2.25 | 4.14 / 2.99 | 2.07 / 3.14 |
| gk.line_dist_max_m | 5.46 / 9.05 | 5.48 / 5.53 | 5.27 / 5.35 |
| gk.line_dist_mean_m | 2.95 / 3.07 | 2.99 / 3.02 | 2.89 / 2.96 |
| gk.parries | 14 / 9 | 17 / 14 | 21 / 14 |
| gk.pct_outside_6yd | 0 / 0.21 | 0.19 / 0.01 | 0 / 0 |
| gk.pct_outside_box | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_count | 0 / 1 | 1 / 1 | 0 / 0 |
| gk.rush_per_10min | 0 / 1 | 1 / 1 | 0 / 0 |
| gk.saves | 16 / 10 | 20 / 15 | 23 / 17 |
| gk.tips_over | 0 / 0 | 0 / 0 | 0 / 0 |
| match.fouls | 0 / 0 | 0 / 0 | 0 / 0 |
| match.goals | 4 / 8 | 6 / 1 | 3 / 4 |
| match.offsides | 1 / 2 | 1 / 1 | 3 / 1 |
| match.passes_completed_pct | 85.64 / 85.29 | 84.62 / 85.47 | 83.33 / 83.03 |
| match.possession_pct | 48.52 / 51.48 | 53.27 / 46.73 | 47.61 / 52.39 |
| match.shots | 19 / 28 | 29 / 25 | 23 / 33 |
| match.shots_on_target | 13 / 24 | 22 / 20 | 20 / 27 |
| move.accel_from_pos_mean_mps2 | 10.52 / 10.65 | 10.84 / 10.85 | 11.03 / 10.72 |
| move.accel_gt10_pct | 47.44 / 48.08 | 49.11 / 49.03 | 49.95 / 48.10 |
| move.accel_mean_mps2 | 10.49 / 10.64 | 10.82 / 10.85 | 11.01 / 10.71 |
| move.dspeed_max_mps_per_tick | 3.52 / 3.69 | 3.62 / 3.82 | 3.58 / 3.88 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.42 / 3.49 | 3.50 / 3.48 | 3.47 / 3.43 |
| move.heading_change_pct.0_10 | 99.39 / 99.45 | 99.35 / 99.34 | 99.38 / 99.36 |
| move.heading_change_pct.10_45 | 0.60 / 0.53 | 0.63 / 0.64 | 0.60 / 0.62 |
| move.heading_change_pct.45_90 | 0.01 / 0.02 | 0.02 / 0.02 | 0.02 / 0.02 |
| move.heading_change_pct.gt90 | 0.00 / 0 | 0.00 / 0.00 | 0.00 / 0 |
| move.lateral_accel_mean_mps2 | 7.67 / 7.80 | 7.89 / 7.95 | 7.88 / 7.68 |
| move.reversals_per_player_min | 0.02 / 0 | 0.02 / 0.02 | 0.02 / 0 |
| move.side_flips_per_player_min | 82.51 / 82.71 | 82.18 / 83.70 | 91.73 / 82.70 |
| move.speed_oscillation_pct | 4.19 / 4.31 | 4.28 / 4.32 | 4.33 / 4.19 |
| move.turn_rate_mean_deg_per_s | 98.55 / 99.37 | 102.7 / 101.4 | 102.0 / 97.55 |
| shape.attacking.anchor_dist_m.defender | 7.85 / 8.74 | 8.82 / 8.13 | 9.22 / 11.45 |
| shape.attacking.anchor_dist_m.forward | 15.26 / 14.68 | 15.05 / 15.20 | 15.85 / 14.94 |
| shape.attacking.anchor_dist_m.goalkeeper | 1.22 / 1.19 | 1.26 / 1.24 | 1.27 / 1.21 |
| shape.attacking.anchor_dist_m.midfielder | 13.57 / 14.19 | 15.03 / 17.29 | 16.38 / 18.06 |
| shape.attacking.anchor_dist_shape_m.defender | 5.95 / 5.95 | 6.23 / 6.27 | 6.99 / 7.39 |
| shape.attacking.anchor_dist_shape_m.forward | 10.66 / 11.07 | 10.32 / 10.53 | 11.36 / 10.97 |
| shape.attacking.anchor_dist_shape_m.midfielder | 7.07 / 7.64 | 8.65 / 11.72 | 9.85 / 10.90 |
| shape.attacking.bbox_area_m2 | 372.3 / 392.9 | 379.9 / 391.0 | 342.2 / 369.8 |
| shape.attacking.bbox_depth_m | 17.70 / 17.82 | 18.44 / 17.96 | 17.20 / 17.61 |
| shape.attacking.bbox_width_m | 19.64 / 20.56 | 19.48 / 20.35 | 18.49 / 19.67 |
| shape.attacking.length_m | 17.70 / 17.82 | 18.44 / 17.96 | 17.20 / 17.61 |
| shape.attacking.nearest_mate_mean_m | 10.82 / 11.14 | 11.07 / 11.14 | 10.28 / 10.79 |
| shape.attacking.nearest_mate_p10_m | 3.85 / 3.87 | 4.03 / 3.96 | 3.34 / 3.73 |
| shape.attacking.pct_of_open_play | 49.76 / 50.24 | 51.01 / 48.99 | 50.64 / 49.36 |
| shape.attacking.stretch_m | 9.67 / 9.91 | 9.79 / 9.93 | 9.29 / 9.78 |
| shape.attacking.width_m | 19.64 / 20.56 | 19.48 / 20.35 | 18.49 / 19.67 |
| shape.defending.anchor_dist_m.defender | 9.31 / 9.03 | 9.28 / 9.19 | 10.21 / 10.95 |
| shape.defending.anchor_dist_m.forward | 15.30 / 14.89 | 14.67 / 15.06 | 17.07 / 15.97 |
| shape.defending.anchor_dist_m.goalkeeper | 1.40 / 1.33 | 1.37 / 1.36 | 1.51 / 1.47 |
| shape.defending.anchor_dist_m.midfielder | 12.47 / 12.87 | 13.84 / 16.39 | 15.54 / 15.79 |
| shape.defending.anchor_dist_shape_m.defender | 8.43 / 8.05 | 8.19 / 8.25 | 9.19 / 9.07 |
| shape.defending.anchor_dist_shape_m.forward | 10.24 / 9.71 | 9.96 / 9.99 | 10.69 / 10.60 |
| shape.defending.anchor_dist_shape_m.midfielder | 6.11 / 6.51 | 7.74 / 10.75 | 8.90 / 9.73 |
| shape.defending.bbox_area_m2 | 333.6 / 378.4 | 338.0 / 383.2 | 336.8 / 349.1 |
| shape.defending.bbox_depth_m | 15.79 / 17.41 | 16.56 / 17.15 | 15.75 / 16.00 |
| shape.defending.bbox_width_m | 19.31 / 20.04 | 19.21 / 20.41 | 19.75 / 20.58 |
| shape.defending.length_m | 15.79 / 17.41 | 16.56 / 17.15 | 15.75 / 16.00 |
| shape.defending.line_x_std_m | 5.14 / 5.77 | 5.44 / 5.77 | 5.25 / 5.44 |
| shape.defending.line_x_variance_m2 | 38.79 / 49.35 | 43.77 / 49.56 | 42.93 / 44.51 |
| shape.defending.nearest_mate_mean_m | 10.45 / 11.29 | 10.73 / 11.14 | 10.47 / 10.84 |
| shape.defending.nearest_mate_p10_m | 4.35 / 5.19 | 4.77 / 4.10 | 3.90 / 4.31 |
| shape.defending.pct_of_open_play | 50.24 / 49.76 | 48.99 / 51.01 | 49.36 / 50.64 |
| shape.defending.pct_outfield_behind_ball | 61.76 / 64.50 | 64.58 / 62.41 | 57.61 / 62.52 |
| shape.defending.stretch_m | 9.10 / 9.64 | 9.30 / 9.74 | 9.22 / 9.55 |
| shape.defending.swarm2_pct | 0 / 0.84 | 0.33 / 2.02 | 1.29 / 0.30 |
| shape.defending.swarm3_pct | 0 / 0 | 0.03 / 0 | 0 / 0.08 |
| shape.defending.width_m | 19.31 / 20.04 | 19.21 / 20.41 | 19.75 / 20.58 |

GK scenario field: not present in the public state.

## 11v11 normal vs normal (10 min x 3 seeds from 1)

Command: `psfc-replay analyze --format 11v11 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty normal --away-difficulty normal`

Gameplay version: `0.1.0-5v5`

| seed | final state_hash | score (H-A) |
|---|---|---|
| 1 | `fb66a9cf17d5d6c4` | 0-0 |
| 2 | `f9ae9b5156ea05d2` | 3-0 |
| 3 | `b58ebd124475b008` | 2-0 |

Match meta (mean over seeds): goals_total = 1.67, open_play_minutes = 10, open_play_pct_of_ticks = 98.42, shots_total = 15.33, ticks_simulated = 36579

### Match

| metric | home | away | teams mean |
|---|---:|---:|---:|
| fouls | 0 | 0 | **0** |
| goals | 1.67 | 0 | **0.83** |
| offsides | 0.33 | 0.33 | **0.33** |
| passes_completed_pct | 84.48 | 83.83 | **84.15** |
| possession_pct | 49.77 | 50.23 | **50** |
| shots | 7.67 | 7.67 | **7.67** |
| shots_on_target | 4.67 | 4.67 | **4.67** |

### Goalkeeper

| metric | home | away | teams mean |
|---|---:|---:|---:|
| catches | 2 | 1.67 | **1.83** |
| claims_non_save | 0 | 0.33 | **0.17** |
| excursion_mean_s | 0.82 | 0.45 | **0.63** |
| excursions_gt1s_per_10min | 1 | 0.67 | **0.83** |
| gathers | 2 | 2 | **2** |
| goals_conceded | 0 | 1.67 | **0.83** |
| goals_conceded_gk_gt3m_off_line | 0 | 1.67 | **0.83** |
| goals_conceded_no_shot_found | 0 | 0 | **0** |
| line_dist_at_conceded_shot_mean_m | 0 | 2.61 | **1.31** |
| line_dist_max_m | 6.73 | 6.06 | **6.39** |
| line_dist_mean_m | 4.51 | 4.58 | **4.54** |
| parries | 3.33 | 1.33 | **2.33** |
| pct_outside_6yd | 0.32 | 0.15 | **0.23** |
| pct_outside_box | 0 | 0 | **0** |
| rush_count | 1.67 | 0.67 | **1.17** |
| rush_per_10min | 1.67 | 0.67 | **1.17** |
| saves | 5.33 | 3 | **4.17** |
| tips_over | 0 | 0 | **0** |

### Defence and shape

| metric | home | away | teams mean |
|---|---:|---:|---:|
| attacking.anchor_dist_m.defender | 18.86 | 21.49 | **20.17** |
| attacking.anchor_dist_m.forward | 21.81 | 23.19 | **22.50** |
| attacking.anchor_dist_m.goalkeeper | 2.26 | 2.25 | **2.26** |
| attacking.anchor_dist_m.midfielder | 14.49 | 15.48 | **14.98** |
| attacking.anchor_dist_shape_m.defender | 15.41 | 16.90 | **16.15** |
| attacking.anchor_dist_shape_m.forward | 16.20 | 18.86 | **17.53** |
| attacking.anchor_dist_shape_m.midfielder | 8.65 | 9.69 | **9.17** |
| attacking.bbox_area_m2 | 1964.8 | 1914.4 | **1939.6** |
| attacking.bbox_depth_m | 41.45 | 39.08 | **40.27** |
| attacking.bbox_width_m | 45.74 | 46.94 | **46.34** |
| attacking.length_m | 41.45 | 39.08 | **40.27** |
| attacking.nearest_mate_mean_m | 12.76 | 12.47 | **12.61** |
| attacking.nearest_mate_p10_m | 6.36 | 6.32 | **6.34** |
| attacking.pct_of_open_play | 50.06 | 49.94 | **50** |
| attacking.stretch_m | 19.70 | 19.51 | **19.61** |
| attacking.width_m | 45.74 | 46.94 | **46.34** |
| defending.anchor_dist_m.defender | 18.42 | 20.53 | **19.48** |
| defending.anchor_dist_m.forward | 23.95 | 24.67 | **24.31** |
| defending.anchor_dist_m.goalkeeper | 2.54 | 2.52 | **2.53** |
| defending.anchor_dist_m.midfielder | 15.10 | 16.30 | **15.70** |
| defending.anchor_dist_shape_m.defender | 16.59 | 16.99 | **16.79** |
| defending.anchor_dist_shape_m.forward | 17.52 | 19.24 | **18.38** |
| defending.anchor_dist_shape_m.midfielder | 9.09 | 9.68 | **9.39** |
| defending.bbox_area_m2 | 1499.8 | 1480.9 | **1490.4** |
| defending.bbox_depth_m | 36.35 | 35.42 | **35.89** |
| defending.bbox_width_m | 39.21 | 39.69 | **39.45** |
| defending.length_m | 36.35 | 35.42 | **35.89** |
| defending.line_x_std_m | 6.08 | 5.93 | **6.00** |
| defending.line_x_variance_m2 | 62.34 | 55.71 | **59.03** |
| defending.nearest_mate_mean_m | 11.11 | 10.92 | **11.02** |
| defending.nearest_mate_p10_m | 6.19 | 6.05 | **6.12** |
| defending.pct_of_open_play | 49.94 | 50.06 | **50** |
| defending.pct_outfield_behind_ball | 68.92 | 69.66 | **69.29** |
| defending.stretch_m | 17.06 | 16.87 | **16.97** |
| defending.swarm2_pct | 0.20 | 0.06 | **0.13** |
| defending.swarm3_pct | 0 | 0 | **0** |
| defending.width_m | 39.21 | 39.69 | **39.45** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 7.86 | 7.85 | **7.86** |
| accel_gt10_pct | 35.86 | 36.07 | **35.96** |
| accel_mean_mps2 | 7.85 | 7.85 | **7.85** |
| dspeed_max_mps_per_tick | 3.51 | 3.56 | **3.53** |
| heading_change_p95_deg_per_tick_gt3mps | 3.19 | 3.20 | **3.19** |
| heading_change_pct.0_10 | 99.66 | 99.68 | **99.67** |
| heading_change_pct.10_45 | 0.33 | 0.32 | **0.32** |
| heading_change_pct.45_90 | 0.01 | 0.01 | **0.01** |
| heading_change_pct.gt90 | 0.00 | 0.00 | **0.00** |
| lateral_accel_mean_mps2 | 5.58 | 5.58 | **5.58** |
| reversals_per_player_min | 0.01 | 0.01 | **0.01** |
| side_flips_per_player_min | 49.60 | 50.88 | **50.24** |
| speed_oscillation_pct | 3.17 | 3.19 | **3.18** |
| turn_rate_mean_deg_per_s | 78.77 | 77.70 | **78.23** |

### Collisions and push-out

| metric | home | away | teams mean |
|---|---:|---:|---:|
| any.depth_max_m | 0.58 | 0.58 | **0.58** |
| any.depth_mean_m | 0.19 | 0.19 | **0.19** |
| any.events_per_min | 1.13 | 1.13 | **1.13** |
| any.stuck_mean_ticks | 0 | 0 | **0** |
| any.stuck_pairs | 0 | 0 | **0** |
| any.ticks_gt1cm_per_min | 2.10 | 2.10 | **2.10** |
| any.ticks_per_min | 2.40 | 2.40 | **2.40** |
| correction_vel_corrected_p50_mps | 0.86 | 0.82 | **0.84** |
| correction_vel_corrected_p99_mps | 13.05 | 8.81 | **10.93** |
| correction_vel_corrected_pct | 0.02 | 0.02 | **0.02** |
| correction_vel_max_mps | 13.05 | 8.81 | **10.93** |
| correction_vel_p50_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p95_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p99_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_pct_gt_0_3 | 0.01 | 0.01 | **0.01** |
| opponent.depth_max_m | 0.11 | 0.11 | **0.11** |
| opponent.depth_mean_m | 0.04 | 0.04 | **0.04** |
| opponent.events_per_min | 0.97 | 0.97 | **0.97** |
| opponent.stuck_mean_ticks | 0 | 0 | **0** |
| opponent.stuck_pairs | 0 | 0 | **0** |
| opponent.ticks_gt1cm_per_min | 0.97 | 0.97 | **0.97** |
| opponent.ticks_per_min | 1.27 | 1.27 | **1.27** |
| teammate.depth_max_m | 0.51 | 0.27 | **0.39** |
| teammate.depth_mean_m | 0.27 | 0.13 | **0.20** |
| teammate.events_per_min | 0.10 | 0.07 | **0.08** |
| teammate.stuck_mean_ticks | 0 | 0 | **0** |
| teammate.stuck_pairs | 0 | 0 | **0** |
| teammate.ticks_gt1cm_per_min | 0.87 | 0.47 | **0.67** |
| teammate.ticks_per_min | 0.87 | 0.47 | **0.67** |

### Coordination

| metric | home | away | teams mean |
|---|---:|---:|---:|
| cruise_over72_pct | 35.08 | 37.62 | **36.35** |
| dual_chaser_pct_of_loose | 11.89 | 12.12 | **12.01** |
| dual_chaser_ticks_per_min | 298.2 | 303.2 | **300.7** |
| engagers3_pct_of_carrier_ticks | 0 | 0 | **0** |
| engagers3_ticks_per_min | 0 | 0 | **0** |
| max_simultaneous_chasers | 4 | 3.33 | **3.67** |

### Aerial

| metric | home | away | teams mean |
|---|---:|---:|---:|
| headers | 26 | 31.33 | **28.67** |
| headers_clear | 1.33 | 0.67 | **1** |
| headers_pass | 22.33 | 27.67 | **25** |
| headers_shot | 2.33 | 3 | **2.67** |
| jumps_gk | 4.67 | 6.67 | **5.67** |
| jumps_outfield | 52.33 | 54.67 | **53.50** |
| max_airborne_outfield | 1 | 1 | **1** |

### Team shape vs envelope (teams mean)

| metric | in possession | out of possession | 11v11 envelope (m) |
|---|---:|---:|---|
| length_m | 40.27 | 35.89 | 31-46 |
| width_m | 46.34 | 39.45 | 35-48 |
| stretch_m | 19.61 (outside) | 16.97 (outside) | 7-16 |
| nearest_mate_mean_m | 12.61 | 11.02 | none published (in-possession mean near 9-10 m, estimate) |
| nearest_mate_p10_m | 6.34 | 6.12 | none published (in-possession mean near 9-10 m, estimate) |

Envelope: peer-reviewed 11v11 ranges (length 31-46 m, width 35-48 m, stretch index 7-16 m) quoted in the research report; the report's tighter targets are 31-36 / 35-40 / 7-10 m out of possession and 40-46 / 42-48 / 12-16 m in possession (estimates).

### Per seed (home / away)

| metric | seed 1 | seed 2 | seed 3 |
|---|---:|---:|---:|
| aerial.headers | 23 / 36 | 28 / 28 | 27 / 30 |
| aerial.headers_clear | 1 / 1 | 1 / 0 | 2 / 1 |
| aerial.headers_pass | 19 / 31 | 24 / 26 | 24 / 26 |
| aerial.headers_shot | 3 / 4 | 3 / 2 | 1 / 3 |
| aerial.jumps_gk | 4 / 4 | 4 / 13 | 6 / 3 |
| aerial.jumps_outfield | 56 / 55 | 53 / 52 | 48 / 57 |
| aerial.max_airborne_outfield | 1 / 1 | 1 / 1 | 1 / 1 |
| collision.any.depth_max_m | 0.80 / 0.80 | 0.80 / 0.80 | 0.13 / 0.13 |
| collision.any.depth_mean_m | 0.27 / 0.27 | 0.24 / 0.24 | 0.04 / 0.04 |
| collision.any.events_per_min | 1.20 / 1.20 | 1.20 / 1.20 | 1 / 1 |
| collision.any.stuck_mean_ticks | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.any.stuck_pairs | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.any.ticks_gt1cm_per_min | 2.90 / 2.90 | 2.30 / 2.30 | 1.10 / 1.10 |
| collision.any.ticks_per_min | 3.20 / 3.20 | 2.70 / 2.70 | 1.30 / 1.30 |
| collision.correction_vel_corrected_p50_mps | 0.49 / 1.26 | 1.80 / 0.93 | 0.31 / 0.28 |
| collision.correction_vel_corrected_p99_mps | 13.25 / 3.60 | 12.65 / 12.63 | 13.24 / 10.20 |
| collision.correction_vel_corrected_pct | 0.02 / 0.02 | 0.02 / 0.01 | 0.02 / 0.02 |
| collision.correction_vel_max_mps | 13.25 / 3.60 | 12.65 / 12.63 | 13.24 / 10.20 |
| collision.correction_vel_p50_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p95_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p99_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_pct_gt_0_3 | 0.01 / 0.02 | 0.02 / 0.01 | 0.01 / 0.01 |
| collision.opponent.depth_max_m | 0.13 / 0.13 | 0.08 / 0.08 | 0.13 / 0.13 |
| collision.opponent.depth_mean_m | 0.05 / 0.05 | 0.02 / 0.02 | 0.04 / 0.04 |
| collision.opponent.events_per_min | 0.90 / 0.90 | 1 / 1 | 1 / 1 |
| collision.opponent.stuck_mean_ticks | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.stuck_pairs | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.ticks_gt1cm_per_min | 1.10 / 1.10 | 0.70 / 0.70 | 1.10 / 1.10 |
| collision.opponent.ticks_per_min | 1.40 / 1.40 | 1.10 / 1.10 | 1.30 / 1.30 |
| collision.teammate.depth_max_m | 0.74 / 0.80 | 0.80 / 0 | 0 / 0 |
| collision.teammate.depth_mean_m | 0.40 / 0.39 | 0.40 / 0 | 0 / 0 |
| collision.teammate.events_per_min | 0.10 / 0.20 | 0.20 / 0 | 0 / 0 |
| collision.teammate.stuck_mean_ticks | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.teammate.stuck_pairs | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.teammate.ticks_gt1cm_per_min | 1 / 1.40 | 1.60 / 0 | 0 / 0 |
| collision.teammate.ticks_per_min | 1 / 1.40 | 1.60 / 0 | 0 / 0 |
| coord.cruise_over72_pct | 35.14 / 36.90 | 34.47 / 38.13 | 35.64 / 37.83 |
| coord.dual_chaser_pct_of_loose | 11.45 / 12.12 | 10.13 / 13.44 | 14.10 / 10.81 |
| coord.dual_chaser_ticks_per_min | 295.4 / 312.7 | 245.7 / 325.9 | 353.5 / 271 |
| coord.engagers3_pct_of_carrier_ticks | 0 / 0 | 0 / 0 | 0 / 0 |
| coord.engagers3_ticks_per_min | 0 / 0 | 0 / 0 | 0 / 0 |
| coord.max_simultaneous_chasers | 4 / 4 | 4 / 3 | 4 / 3 |
| gk.catches | 4 / 1 | 0 / 3 | 2 / 1 |
| gk.claims_non_save | 0 / 1 | 0 / 0 | 0 / 0 |
| gk.excursion_mean_s | 1.37 / 1.34 | 0 / 0 | 1.09 / 0 |
| gk.excursions_gt1s_per_10min | 1 / 2 | 0 / 0 | 2 / 0 |
| gk.gathers | 4 / 2 | 0 / 3 | 2 / 1 |
| gk.goals_conceded | 0 / 0 | 0 / 3 | 0 / 2 |
| gk.goals_conceded_gk_gt3m_off_line | 0 / 0 | 0 / 3 | 0 / 2 |
| gk.goals_conceded_no_shot_found | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.line_dist_at_conceded_shot_mean_m | 0 / 0 | 0 / 4.11 | 0 / 3.73 |
| gk.line_dist_max_m | 7.24 / 7.64 | 5.27 / 5.27 | 7.68 / 5.27 |
| gk.line_dist_mean_m | 4.41 / 4.58 | 4.63 / 4.54 | 4.47 / 4.62 |
| gk.parries | 3 / 3 | 2 / 1 | 5 / 0 |
| gk.pct_outside_6yd | 0.23 / 0.45 | 0 / 0 | 0.73 / 0 |
| gk.pct_outside_box | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_count | 1 / 2 | 0 / 0 | 4 / 0 |
| gk.rush_per_10min | 1 / 2 | 0 / 0 | 4 / 0 |
| gk.saves | 7 / 4 | 2 / 4 | 7 / 1 |
| gk.tips_over | 0 / 0 | 0 / 0 | 0 / 0 |
| match.fouls | 0 / 0 | 0 / 0 | 0 / 0 |
| match.goals | 0 / 0 | 3 / 0 | 2 / 0 |
| match.offsides | 0 / 0 | 0 / 1 | 1 / 0 |
| match.passes_completed_pct | 83.33 / 86.14 | 86.67 / 82.47 | 83.43 / 82.88 |
| match.possession_pct | 47.50 / 52.50 | 53.86 / 46.14 | 47.94 / 52.06 |
| match.shots | 8 / 8 | 10 / 5 | 5 / 10 |
| match.shots_on_target | 4 / 6 | 7 / 2 | 3 / 6 |
| move.accel_from_pos_mean_mps2 | 7.86 / 7.88 | 7.60 / 7.68 | 8.12 / 8.00 |
| move.accel_gt10_pct | 35.76 / 36.08 | 34.61 / 34.92 | 37.22 / 37.20 |
| move.accel_mean_mps2 | 7.85 / 7.88 | 7.59 / 7.67 | 8.10 / 7.99 |
| move.dspeed_max_mps_per_tick | 3.62 / 3.68 | 3.59 / 3.52 | 3.32 / 3.48 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.19 / 3.21 | 3.18 / 3.16 | 3.20 / 3.21 |
| move.heading_change_pct.0_10 | 99.66 / 99.67 | 99.71 / 99.69 | 99.63 / 99.68 |
| move.heading_change_pct.10_45 | 0.34 / 0.33 | 0.28 / 0.31 | 0.37 / 0.31 |
| move.heading_change_pct.45_90 | 0.01 / 0.01 | 0.01 / 0.00 | 0.01 / 0.00 |
| move.heading_change_pct.gt90 | 0 / 0.00 | 0.00 / 0.00 | 0 / 0 |
| move.lateral_accel_mean_mps2 | 5.55 / 5.57 | 5.48 / 5.41 | 5.71 / 5.76 |
| move.reversals_per_player_min | 0 / 0.01 | 0.02 / 0.02 | 0 / 0 |
| move.side_flips_per_player_min | 50.61 / 50.99 | 47.61 / 49.41 | 50.57 / 52.24 |
| move.speed_oscillation_pct | 3.18 / 3.17 | 3.15 / 3.09 | 3.18 / 3.31 |
| move.turn_rate_mean_deg_per_s | 78.68 / 78.05 | 77.10 / 75.18 | 80.53 / 79.86 |
| shape.attacking.anchor_dist_m.defender | 17.72 / 22.35 | 21.32 / 21.51 | 17.53 / 20.61 |
| shape.attacking.anchor_dist_m.forward | 23.09 / 22.27 | 22.58 / 23.25 | 19.75 / 24.04 |
| shape.attacking.anchor_dist_m.goalkeeper | 2.27 / 2.25 | 2.29 / 2.24 | 2.21 / 2.27 |
| shape.attacking.anchor_dist_m.midfielder | 15.06 / 17.90 | 13.18 / 14.84 | 15.22 / 13.70 |
| shape.attacking.anchor_dist_shape_m.defender | 14.27 / 17.29 | 17.33 / 18.28 | 14.62 / 15.12 |
| shape.attacking.anchor_dist_shape_m.forward | 17.78 / 17.72 | 17.82 / 18.84 | 13.01 / 20.02 |
| shape.attacking.anchor_dist_shape_m.midfielder | 9.59 / 12.28 | 7.64 / 9.67 | 8.72 / 7.14 |
| shape.attacking.bbox_area_m2 | 1991.1 / 1961.7 | 1891.8 / 2015.9 | 2011.6 / 1765.7 |
| shape.attacking.bbox_depth_m | 42.47 / 39.42 | 39.90 / 40.48 | 41.99 / 37.34 |
| shape.attacking.bbox_width_m | 45.31 / 47.84 | 45.63 / 47.64 | 46.28 / 45.32 |
| shape.attacking.length_m | 42.47 / 39.42 | 39.90 / 40.48 | 41.99 / 37.34 |
| shape.attacking.nearest_mate_mean_m | 12.75 / 12.51 | 12.54 / 12.88 | 12.99 / 12.00 |
| shape.attacking.nearest_mate_p10_m | 6.36 / 5.92 | 6.48 / 6.63 | 6.24 / 6.40 |
| shape.attacking.pct_of_open_play | 48.64 / 51.36 | 49.78 / 50.22 | 51.76 / 48.24 |
| shape.attacking.stretch_m | 19.85 / 19.93 | 19.32 / 19.82 | 19.94 / 18.76 |
| shape.attacking.width_m | 45.31 / 47.84 | 45.63 / 47.64 | 46.28 / 45.32 |
| shape.defending.anchor_dist_m.defender | 18.74 / 21.33 | 19.51 / 20.02 | 17.01 / 20.26 |
| shape.defending.anchor_dist_m.forward | 25.71 / 23.38 | 23.94 / 25.65 | 22.19 / 24.99 |
| shape.defending.anchor_dist_m.goalkeeper | 2.58 / 2.56 | 2.49 / 2.50 | 2.56 / 2.50 |
| shape.defending.anchor_dist_m.midfielder | 15.66 / 17.13 | 13.75 / 16.50 | 15.90 / 15.28 |
| shape.defending.anchor_dist_shape_m.defender | 16.55 / 17.61 | 17.67 / 18.03 | 15.56 / 15.31 |
| shape.defending.anchor_dist_shape_m.forward | 19.94 / 18.27 | 18.29 / 18.99 | 14.35 / 20.44 |
| shape.defending.anchor_dist_shape_m.midfielder | 9.90 / 10.92 | 8.14 / 10.25 | 9.25 / 7.88 |
| shape.defending.bbox_area_m2 | 1534.1 / 1502.0 | 1434.9 / 1517.2 | 1530.5 / 1423.6 |
| shape.defending.bbox_depth_m | 37.22 / 35.47 | 35.85 / 35.60 | 35.98 / 35.20 |
| shape.defending.bbox_width_m | 39.31 / 40.16 | 38.15 / 40.24 | 40.18 / 38.67 |
| shape.defending.length_m | 37.22 / 35.47 | 35.85 / 35.60 | 35.98 / 35.20 |
| shape.defending.line_x_std_m | 6.45 / 6.02 | 5.22 / 6.03 | 6.56 / 5.74 |
| shape.defending.line_x_variance_m2 | 67.14 / 57.66 | 53.05 / 56.65 | 66.84 / 52.82 |
| shape.defending.nearest_mate_mean_m | 10.99 / 10.89 | 11.08 / 11.22 | 11.25 / 10.66 |
| shape.defending.nearest_mate_p10_m | 6.14 / 5.85 | 6.48 / 6.25 | 5.94 / 6.05 |
| shape.defending.pct_of_open_play | 51.36 / 48.64 | 50.22 / 49.78 | 48.24 / 51.76 |
| shape.defending.pct_outfield_behind_ball | 66.95 / 70.21 | 75.44 / 64.05 | 64.36 / 74.70 |
| shape.defending.stretch_m | 17.19 / 17.10 | 16.72 / 17.03 | 17.27 / 16.49 |
| shape.defending.swarm2_pct | 0.60 / 0 | 0 / 0.17 | 0 / 0 |
| shape.defending.swarm3_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| shape.defending.width_m | 39.31 / 40.16 | 38.15 / 40.24 | 40.18 / 38.67 |

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

