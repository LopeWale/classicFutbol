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

### Defensive transition

| metric | home | away | teams mean |
|---|---:|---:|---:|
| count_per_10min | 22.67 | 23 | **22.83** |
| counterpress_pct | 32.83 | 28.66 | **30.74** |
| forward_recovery_speed_mps_0_5s | 6.63 | 6.44 | **6.54** |
| forward_sprint_share_0_5s | 65.24 | 60.60 | **62.92** |
| forwards_staying_high_mean | 0.92 | 0.81 | **0.87** |
| in_shape_at_turnover_pct | 60.97 | 59.13 | **60.05** |
| mid_recovery_speed_mps_0_5s | 6.59 | 6.52 | **6.56** |
| mid_sprint_share_0_5s | 64.31 | 63.30 | **63.81** |
| pct_outfield_behind_ball_at_3s | 72.45 | 65.90 | **69.17** |
| pct_outfield_behind_ball_at_6s | 61.43 | 65.10 | **63.27** |
| time_to_shape_capped_pct | 0 | 0 | **0** |
| time_to_shape_excluded_pct | 12.70 | 15.50 | **14.10** |
| time_to_shape_s | 0.65 | 1.08 | **0.86** |
| time_to_shape_static_capped_pct | 0 | 0 | **0** |
| time_to_shape_static_excluded_pct | 26.38 | 36.75 | **31.56** |
| time_to_shape_static_s | 0.61 | 0.64 | **0.62** |

### Attacking support

| metric | home | away | teams mean |
|---|---:|---:|---:|
| carrier_pct_of_open_play | 20.34 | 17.89 | **19.11** |
| cluster_pct | 7.13 | 4.70 | **5.92** |
| depth_m | 18.49 | 18.50 | **18.49** |
| forward_options_mean | 0.24 | 0.31 | **0.28** |
| nearest_option_dist_m | 12.45 | 11.67 | **12.06** |
| open_options_mean | 1.26 | 1.44 | **1.35** |
| open_options_p10 | 0 | 0 | **0** |
| pass_target_open_pct | 68.97 | 70.37 | **69.67** |
| passes_completed_tracked | 132.3 | 132.7 | **132.5** |
| runs_in_behind_per_10min | 4.67 | 3 | **3.83** |
| ticks_with_le1_option_pct | 57.44 | 48.60 | **53.02** |
| width_m | 19.00 | 19.81 | **19.40** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 10.92 | 11.05 | **10.98** |
| accel_gt10_pct | 49.43 | 49.73 | **49.58** |
| accel_mean_mps2 | 10.89 | 11.02 | **10.96** |
| class_time_pct.0 | 12.79 | 12.41 | **12.60** |
| class_time_pct.1 | 6.61 | 7.14 | **6.88** |
| class_time_pct.2 | 65.23 | 64.75 | **64.99** |
| class_time_pct.3 | 15.36 | 15.70 | **15.53** |
| dspeed_max_mps_per_tick | 3.63 | 3.70 | **3.67** |
| heading_change_p95_deg_per_tick_gt3mps | 3.43 | 3.52 | **3.47** |
| heading_change_pct.0_10 | 99.36 | 99.31 | **99.34** |
| heading_change_pct.10_45 | 0.62 | 0.67 | **0.64** |
| heading_change_pct.45_90 | 0.02 | 0.02 | **0.02** |
| heading_change_pct.gt90 | 0 | 0.00 | **0.00** |
| heading_sign_flips_per_player_min | 36.71 | 35.49 | **36.10** |
| lateral_accel_mean_mps2 | 7.96 | 7.95 | **7.96** |
| reversals_per_player_min | 0 | 0.01 | **0.00** |
| side_flips_by_class.0_per_player_min | 176.5 | 177.0 | **176.8** |
| side_flips_by_class.0_share_pct | 27.07 | 26.78 | **26.93** |
| side_flips_by_class.1_per_player_min | 97.65 | 97.14 | **97.39** |
| side_flips_by_class.1_share_pct | 7.73 | 8.46 | **8.09** |
| side_flips_by_class.2_per_player_min | 73.01 | 70.38 | **71.70** |
| side_flips_by_class.2_share_pct | 57.04 | 55.63 | **56.34** |
| side_flips_by_class.3_per_player_min | 44.26 | 47.70 | **45.98** |
| side_flips_by_class.3_share_pct | 8.16 | 9.13 | **8.65** |
| side_flips_per_player_min | 83.46 | 81.96 | **82.71** |
| side_flips_speed_ge3_per_player_min | 110.0 | 106.7 | **108.3** |
| side_flips_speed_lt3_per_player_min | 24.48 | 25.73 | **25.10** |
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
| move.class_time_pct.0 | 12.53 / 12.38 | 13.05 / 12.38 | 12.80 / 12.47 |
| move.class_time_pct.1 | 6.93 / 7.10 | 6.50 / 7.03 | 6.40 / 7.30 |
| move.class_time_pct.2 | 65.40 / 65.39 | 64.52 / 64.29 | 65.78 / 64.56 |
| move.class_time_pct.3 | 15.14 / 15.13 | 15.93 / 16.30 | 15.03 / 15.67 |
| move.dspeed_max_mps_per_tick | 3.81 / 3.65 | 3.67 / 3.58 | 3.41 / 3.88 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.40 / 3.52 | 3.47 / 3.53 | 3.41 / 3.52 |
| move.heading_change_pct.0_10 | 99.40 / 99.32 | 99.32 / 99.36 | 99.37 / 99.25 |
| move.heading_change_pct.10_45 | 0.58 / 0.67 | 0.67 / 0.62 | 0.61 / 0.72 |
| move.heading_change_pct.45_90 | 0.02 / 0.01 | 0.01 / 0.02 | 0.02 / 0.02 |
| move.heading_change_pct.gt90 | 0 / 0 | 0 / 0.00 | 0 / 0 |
| move.heading_sign_flips_per_player_min | 36.05 / 34.15 | 36.53 / 36.07 | 37.55 / 36.25 |
| move.lateral_accel_mean_mps2 | 8.01 / 7.95 | 8.03 / 8.12 | 7.83 / 7.78 |
| move.reversals_per_player_min | 0 / 0 | 0 / 0.02 | 0 / 0 |
| move.side_flips_by_class.0_per_player_min | 172.2 / 157.6 | 169.4 / 188.5 | 188.0 / 185.0 |
| move.side_flips_by_class.0_share_pct | 25.44 / 24.77 | 26.49 / 28.49 | 29.29 / 27.07 |
| move.side_flips_by_class.1_per_player_min | 92.72 / 89.13 | 113.0 / 106.3 | 87.26 / 96.01 |
| move.side_flips_by_class.1_share_pct | 7.58 / 8.03 | 8.81 / 9.12 | 6.80 / 8.22 |
| move.side_flips_by_class.2_per_player_min | 77.37 / 69.95 | 72.16 / 67.78 | 69.51 / 73.42 |
| move.side_flips_by_class.2_share_pct | 59.66 / 58.05 | 55.81 / 53.20 | 55.65 / 55.64 |
| move.side_flips_by_class.3_per_player_min | 40.99 / 47.62 | 46.62 / 46.17 | 45.16 / 49.30 |
| move.side_flips_by_class.3_share_pct | 7.32 / 9.15 | 8.90 / 9.19 | 8.26 / 9.07 |
| move.side_flips_per_player_min | 84.81 / 78.79 | 83.43 / 81.91 | 82.15 / 85.19 |
| move.side_flips_speed_ge3_per_player_min | 113.1 / 102.7 | 109.3 / 106.5 | 107.6 / 110.8 |
| move.side_flips_speed_lt3_per_player_min | 21.97 / 24.34 | 25.62 / 26.37 | 25.84 / 26.48 |
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
| support.carrier_pct_of_open_play | 20.34 / 18.67 | 19.89 / 17.99 | 20.77 / 16.99 |
| support.cluster_pct | 3.82 / 4.81 | 4.24 / 6.72 | 13.34 / 2.57 |
| support.depth_m | 19.16 / 18.19 | 18.20 / 19.16 | 18.11 / 18.16 |
| support.forward_options_mean | 0.26 / 0.29 | 0.26 / 0.32 | 0.19 / 0.33 |
| support.nearest_option_dist_m | 12.07 / 11.37 | 12.70 / 11.72 | 12.57 / 11.92 |
| support.open_options_mean | 1.24 / 1.42 | 1.34 / 1.48 | 1.21 / 1.41 |
| support.open_options_p10 | 0 / 0 | 0 / 0 | 0 / 0 |
| support.pass_target_open_pct | 74.38 / 69.66 | 67.61 / 71.11 | 64.93 / 70.34 |
| support.passes_completed_tracked | 121 / 145 | 142 / 135 | 134 / 118 |
| support.runs_in_behind_per_10min | 5 / 2 | 4 / 5 | 5 / 2 |
| support.ticks_with_le1_option_pct | 60.36 / 47.87 | 53.09 / 47.68 | 58.88 / 50.25 |
| support.width_m | 19.28 / 18.34 | 20.18 / 19.64 | 17.53 / 21.44 |
| trans.count_per_10min | 23 / 25 | 20 / 18 | 25 / 26 |
| trans.counterpress_pct | 43.48 / 24 | 15 / 38.89 | 40 / 23.08 |
| trans.forward_recovery_speed_mps_0_5s | 6.90 / 6.31 | 6.46 / 6.78 | 6.54 / 6.22 |
| trans.forward_sprint_share_0_5s | 68.76 / 58.59 | 62.51 / 67.22 | 64.44 / 56.00 |
| trans.forwards_staying_high_mean | 1 / 0.75 | 0.86 / 0.75 | 0.90 / 0.94 |
| trans.in_shape_at_turnover_pct | 58.82 / 70 | 65 / 43.75 | 59.09 / 63.64 |
| trans.mid_recovery_speed_mps_0_5s | 6.76 / 6.43 | 6.40 / 6.47 | 6.62 / 6.66 |
| trans.mid_sprint_share_0_5s | 66.79 / 62.61 | 60.53 / 62.59 | 65.62 / 64.70 |
| trans.pct_outfield_behind_ball_at_3s | 71.88 / 52.50 | 65.79 / 82.69 | 79.69 / 62.50 |
| trans.pct_outfield_behind_ball_at_6s | 50 / 75 | 64.29 / 62.50 | 70 / 57.81 |
| trans.time_to_shape_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_excluded_pct | 26.09 / 20 | 0 / 11.11 | 12 / 15.38 |
| trans.time_to_shape_s | 0.74 / 0.94 | 0.54 / 0.87 | 0.65 / 1.42 |
| trans.time_to_shape_static_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_static_excluded_pct | 39.13 / 44 | 20 / 27.78 | 20 / 38.46 |
| trans.time_to_shape_static_s | 0.51 / 0.60 | 0.70 / 0.33 | 0.61 / 0.99 |

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

### Defensive transition

| metric | home | away | teams mean |
|---|---:|---:|---:|
| count_per_10min | 27.67 | 23.33 | **25.50** |
| counterpress_pct | 40.22 | 48.12 | **44.17** |
| forward_recovery_speed_mps_0_5s | 6.18 | 6.58 | **6.38** |
| forward_sprint_share_0_5s | 56.80 | 63.45 | **60.12** |
| forwards_staying_high_mean | 0.80 | 0.81 | **0.80** |
| in_shape_at_turnover_pct | 40.86 | 45.03 | **42.95** |
| mid_recovery_speed_mps_0_5s | 6.45 | 6.70 | **6.57** |
| mid_sprint_share_0_5s | 62.31 | 66.56 | **64.43** |
| pct_outfield_behind_ball_at_3s | 62.32 | 66.90 | **64.61** |
| pct_outfield_behind_ball_at_6s | 62.40 | 67.36 | **64.88** |
| time_to_shape_capped_pct | 0 | 0 | **0** |
| time_to_shape_excluded_pct | 15.40 | 18.48 | **16.94** |
| time_to_shape_s | 1.40 | 1.33 | **1.37** |
| time_to_shape_static_capped_pct | 0 | 0 | **0** |
| time_to_shape_static_excluded_pct | 35.68 | 29.79 | **32.73** |
| time_to_shape_static_s | 0.82 | 1.26 | **1.04** |

### Attacking support

| metric | home | away | teams mean |
|---|---:|---:|---:|
| carrier_pct_of_open_play | 19.29 | 19.51 | **19.40** |
| cluster_pct | 7.72 | 6.22 | **6.97** |
| depth_m | 18.60 | 18.29 | **18.44** |
| forward_options_mean | 0.24 | 0.27 | **0.26** |
| nearest_option_dist_m | 12.56 | 13.13 | **12.85** |
| open_options_mean | 1.26 | 1.25 | **1.25** |
| open_options_p10 | 0 | 0 | **0** |
| pass_target_open_pct | 66.68 | 65.24 | **65.96** |
| passes_completed_tracked | 117.7 | 117.3 | **117.5** |
| runs_in_behind_per_10min | 2.33 | 4.33 | **3.33** |
| ticks_with_le1_option_pct | 58.56 | 58.54 | **58.55** |
| width_m | 18.32 | 19.57 | **18.95** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 10.80 | 10.74 | **10.77** |
| accel_gt10_pct | 48.83 | 48.40 | **48.62** |
| accel_mean_mps2 | 10.78 | 10.73 | **10.75** |
| class_time_pct.0 | 12.86 | 13.02 | **12.94** |
| class_time_pct.1 | 6.71 | 6.28 | **6.49** |
| class_time_pct.2 | 65.00 | 65.63 | **65.31** |
| class_time_pct.3 | 15.44 | 15.07 | **15.25** |
| dspeed_max_mps_per_tick | 3.57 | 3.80 | **3.68** |
| heading_change_p95_deg_per_tick_gt3mps | 3.46 | 3.47 | **3.47** |
| heading_change_pct.0_10 | 99.37 | 99.39 | **99.38** |
| heading_change_pct.10_45 | 0.61 | 0.60 | **0.60** |
| heading_change_pct.45_90 | 0.02 | 0.02 | **0.02** |
| heading_change_pct.gt90 | 0.00 | 0.00 | **0.00** |
| heading_sign_flips_per_player_min | 37.46 | 36.63 | **37.04** |
| lateral_accel_mean_mps2 | 7.81 | 7.81 | **7.81** |
| reversals_per_player_min | 0.02 | 0.01 | **0.01** |
| side_flips_by_class.0_per_player_min | 183.9 | 169.9 | **176.9** |
| side_flips_by_class.0_share_pct | 27.61 | 26.65 | **27.13** |
| side_flips_by_class.1_per_player_min | 99.06 | 95.28 | **97.17** |
| side_flips_by_class.1_share_pct | 7.79 | 7.20 | **7.49** |
| side_flips_by_class.2_per_player_min | 74.27 | 73.45 | **73.86** |
| side_flips_by_class.2_share_pct | 56.51 | 58.04 | **57.28** |
| side_flips_by_class.3_per_player_min | 44.91 | 44.62 | **44.76** |
| side_flips_by_class.3_share_pct | 8.09 | 8.10 | **8.10** |
| side_flips_per_player_min | 85.47 | 83.04 | **84.26** |
| side_flips_speed_ge3_per_player_min | 112.8 | 109.4 | **111.1** |
| side_flips_speed_lt3_per_player_min | 24.95 | 25.11 | **25.03** |
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
| move.class_time_pct.0 | 12.58 / 12.73 | 13.21 / 12.74 | 12.78 / 13.61 |
| move.class_time_pct.1 | 7.16 / 6.39 | 6.42 / 6.67 | 6.55 / 5.78 |
| move.class_time_pct.2 | 64.84 / 64.94 | 64.50 / 66.36 | 65.66 / 65.57 |
| move.class_time_pct.3 | 15.43 / 15.94 | 15.87 / 14.23 | 15.01 / 15.03 |
| move.dspeed_max_mps_per_tick | 3.52 / 3.69 | 3.62 / 3.82 | 3.58 / 3.88 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.42 / 3.49 | 3.50 / 3.48 | 3.47 / 3.43 |
| move.heading_change_pct.0_10 | 99.39 / 99.45 | 99.35 / 99.34 | 99.38 / 99.36 |
| move.heading_change_pct.10_45 | 0.60 / 0.53 | 0.63 / 0.64 | 0.60 / 0.62 |
| move.heading_change_pct.45_90 | 0.01 / 0.02 | 0.02 / 0.02 | 0.02 / 0.02 |
| move.heading_change_pct.gt90 | 0.00 / 0 | 0.00 / 0.00 | 0.00 / 0 |
| move.heading_sign_flips_per_player_min | 35.85 / 37.17 | 36.25 / 36.73 | 40.27 / 36.00 |
| move.lateral_accel_mean_mps2 | 7.67 / 7.80 | 7.89 / 7.95 | 7.88 / 7.68 |
| move.reversals_per_player_min | 0.02 / 0 | 0.02 / 0.02 | 0.02 / 0 |
| move.side_flips_by_class.0_per_player_min | 180.3 / 157.0 | 167.9 / 182.4 | 203.5 / 170.3 |
| move.side_flips_by_class.0_share_pct | 27.49 / 24.15 | 26.98 / 27.76 | 28.36 / 28.04 |
| move.side_flips_by_class.1_per_player_min | 98.12 / 96.48 | 94.15 / 93.38 | 104.9 / 95.97 |
| move.side_flips_by_class.1_share_pct | 8.52 / 7.45 | 7.36 / 7.44 | 7.49 / 6.71 |
| move.side_flips_by_class.2_per_player_min | 70.82 / 76.59 | 73.73 / 72.83 | 78.26 / 70.94 |
| move.side_flips_by_class.2_share_pct | 55.65 / 60.14 | 57.87 / 57.75 | 56.01 / 56.25 |
| move.side_flips_by_class.3_per_player_min | 44.64 / 42.82 | 40.34 / 41.48 | 49.75 / 49.55 |
| move.side_flips_by_class.3_share_pct | 8.35 / 8.25 | 7.79 / 7.05 | 8.14 / 9.01 |
| move.side_flips_per_player_min | 82.51 / 82.71 | 82.18 / 83.70 | 91.73 / 82.70 |
| move.side_flips_speed_ge3_per_player_min | 108.5 / 108.5 | 108.6 / 110.9 | 121.2 / 108.8 |
| move.side_flips_speed_lt3_per_player_min | 23.07 / 24.25 | 24.94 / 24.55 | 26.85 / 26.54 |
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
| support.carrier_pct_of_open_play | 19.88 / 21.29 | 20.51 / 17.96 | 17.48 / 19.29 |
| support.cluster_pct | 4.96 / 6.48 | 6.49 / 5.38 | 11.71 / 6.78 |
| support.depth_m | 18.46 / 17.89 | 19.32 / 18.83 | 18.01 / 18.16 |
| support.forward_options_mean | 0.22 / 0.18 | 0.32 / 0.31 | 0.19 / 0.32 |
| support.nearest_option_dist_m | 12.96 / 13.46 | 12.61 / 13.04 | 12.12 / 12.89 |
| support.open_options_mean | 1.29 / 1.15 | 1.22 / 1.33 | 1.26 / 1.26 |
| support.open_options_p10 | 0 / 0 | 0 / 0 | 0 / 0 |
| support.pass_target_open_pct | 64.62 / 66.39 | 67.27 / 66.13 | 68.14 / 63.21 |
| support.passes_completed_tracked | 130 / 122 | 110 / 124 | 113 / 106 |
| support.runs_in_behind_per_10min | 2 / 5 | 3 / 5 | 2 / 3 |
| support.ticks_with_le1_option_pct | 59.02 / 63.62 | 57.41 / 54.63 | 59.24 / 57.37 |
| support.width_m | 19.34 / 20.21 | 18.58 / 20.09 | 17.05 / 18.43 |
| trans.count_per_10min | 33 / 21 | 28 / 26 | 22 / 23 |
| trans.counterpress_pct | 33.33 / 61.90 | 46.43 / 34.62 | 40.91 / 47.83 |
| trans.forward_recovery_speed_mps_0_5s | 6.25 / 6.66 | 6.19 / 6.43 | 6.09 / 6.65 |
| trans.forward_sprint_share_0_5s | 58.21 / 63.96 | 55.86 / 60.96 | 56.34 / 65.43 |
| trans.forwards_staying_high_mean | 0.88 / 1 | 0.75 / 0.75 | 0.78 / 0.67 |
| trans.in_shape_at_turnover_pct | 34.48 / 50 | 38.10 / 56.52 | 50 / 28.57 |
| trans.mid_recovery_speed_mps_0_5s | 6.40 / 6.87 | 6.46 / 6.55 | 6.50 / 6.67 |
| trans.mid_sprint_share_0_5s | 61.87 / 68.61 | 62.32 / 64.89 | 62.73 / 66.18 |
| trans.pct_outfield_behind_ball_at_3s | 71 / 67.86 | 56.58 / 65 | 59.38 / 67.86 |
| trans.pct_outfield_behind_ball_at_6s | 57.35 / 66.67 | 60.42 / 68.75 | 69.44 / 66.67 |
| trans.time_to_shape_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_excluded_pct | 12.12 / 4.76 | 25 / 11.54 | 9.09 / 39.13 |
| trans.time_to_shape_s | 1.33 / 1.34 | 1.63 / 1.17 | 1.25 / 1.49 |
| trans.time_to_shape_static_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_static_excluded_pct | 24.24 / 23.81 | 46.43 / 30.77 | 36.36 / 34.78 |
| trans.time_to_shape_static_s | 1.23 / 1.00 | 0.63 / 1.49 | 0.60 / 1.29 |

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

### Defensive transition

| metric | home | away | teams mean |
|---|---:|---:|---:|
| count_per_10min | 17.33 | 12.67 | **15** |
| counterpress_pct | 46.06 | 44.78 | **45.42** |
| forward_recovery_speed_mps_0_5s | 6.23 | 6.50 | **6.36** |
| forward_sprint_share_0_5s | 65.70 | 70.39 | **68.04** |
| forwards_staying_high_mean | 2.18 | 1.93 | **2.05** |
| in_shape_at_turnover_pct | 52.22 | 52.46 | **52.34** |
| mid_recovery_speed_mps_0_5s | 5.55 | 5.87 | **5.71** |
| mid_sprint_share_0_5s | 50.28 | 56.49 | **53.38** |
| pct_outfield_behind_ball_at_3s | 68.11 | 65.28 | **66.69** |
| pct_outfield_behind_ball_at_6s | 69.92 | 67.48 | **68.70** |
| time_to_shape_capped_pct | 0 | 0 | **0** |
| time_to_shape_excluded_pct | 16.98 | 28.11 | **22.55** |
| time_to_shape_s | 1.46 | 1.37 | **1.41** |
| time_to_shape_static_capped_pct | 0 | 0 | **0** |
| time_to_shape_static_excluded_pct | 24.66 | 36.70 | **30.68** |
| time_to_shape_static_s | 0.93 | 0.80 | **0.86** |

### Attacking support

| metric | home | away | teams mean |
|---|---:|---:|---:|
| carrier_pct_of_open_play | 15.03 | 15.05 | **15.04** |
| cluster_pct | 3.90 | 5.37 | **4.63** |
| depth_m | 39.77 | 38.30 | **39.03** |
| forward_options_mean | 0.77 | 0.71 | **0.74** |
| nearest_option_dist_m | 12.79 | 12.62 | **12.70** |
| open_options_mean | 3.47 | 3.88 | **3.68** |
| open_options_p10 | 0 | 0 | **0** |
| pass_target_open_pct | 72.57 | 75.18 | **73.87** |
| passes_completed_tracked | 122 | 110.3 | **116.2** |
| runs_in_behind_per_10min | 1.67 | 0 | **0.83** |
| ticks_with_le1_option_pct | 22.07 | 17.88 | **19.98** |
| width_m | 44.57 | 46.17 | **45.37** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 7.86 | 7.85 | **7.86** |
| accel_gt10_pct | 35.86 | 36.07 | **35.96** |
| accel_mean_mps2 | 7.85 | 7.85 | **7.85** |
| class_time_pct.0 | 5.87 | 5.92 | **5.90** |
| class_time_pct.1 | 2.93 | 2.89 | **2.91** |
| class_time_pct.2 | 68.54 | 69.55 | **69.04** |
| class_time_pct.3 | 22.66 | 21.64 | **22.15** |
| dspeed_max_mps_per_tick | 3.51 | 3.56 | **3.53** |
| heading_change_p95_deg_per_tick_gt3mps | 3.19 | 3.20 | **3.19** |
| heading_change_pct.0_10 | 99.66 | 99.68 | **99.67** |
| heading_change_pct.10_45 | 0.33 | 0.32 | **0.32** |
| heading_change_pct.45_90 | 0.01 | 0.01 | **0.01** |
| heading_change_pct.gt90 | 0.00 | 0.00 | **0.00** |
| heading_sign_flips_per_player_min | 19.87 | 19.26 | **19.57** |
| lateral_accel_mean_mps2 | 5.58 | 5.58 | **5.58** |
| reversals_per_player_min | 0.01 | 0.01 | **0.01** |
| side_flips_by_class.0_per_player_min | 192.3 | 178.1 | **185.2** |
| side_flips_by_class.0_share_pct | 22.75 | 20.75 | **21.75** |
| side_flips_by_class.1_per_player_min | 89.85 | 92.18 | **91.02** |
| side_flips_by_class.1_share_pct | 5.29 | 5.22 | **5.26** |
| side_flips_by_class.2_per_player_min | 40.42 | 42.65 | **41.53** |
| side_flips_by_class.2_share_pct | 55.86 | 58.32 | **57.09** |
| side_flips_by_class.3_per_player_min | 35.22 | 36.94 | **36.08** |
| side_flips_by_class.3_share_pct | 16.10 | 15.71 | **15.91** |
| side_flips_per_player_min | 49.60 | 50.88 | **50.24** |
| side_flips_speed_ge3_per_player_min | 57.69 | 58.61 | **58.15** |
| side_flips_speed_lt3_per_player_min | 24.91 | 25.16 | **25.03** |
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
| move.class_time_pct.0 | 5.86 / 6.03 | 5.69 / 5.82 | 6.06 / 5.91 |
| move.class_time_pct.1 | 2.96 / 2.74 | 3.11 / 2.96 | 2.73 / 2.96 |
| move.class_time_pct.2 | 68.84 / 69.31 | 68.26 / 69.67 | 68.52 / 69.66 |
| move.class_time_pct.3 | 22.34 / 21.92 | 22.94 / 21.54 | 22.69 / 21.47 |
| move.dspeed_max_mps_per_tick | 3.62 / 3.68 | 3.59 / 3.52 | 3.32 / 3.48 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.19 / 3.21 | 3.18 / 3.16 | 3.20 / 3.21 |
| move.heading_change_pct.0_10 | 99.66 / 99.67 | 99.71 / 99.69 | 99.63 / 99.68 |
| move.heading_change_pct.10_45 | 0.34 / 0.33 | 0.28 / 0.31 | 0.37 / 0.31 |
| move.heading_change_pct.45_90 | 0.01 / 0.01 | 0.01 / 0.00 | 0.01 / 0.00 |
| move.heading_change_pct.gt90 | 0 / 0.00 | 0.00 / 0.00 | 0 / 0 |
| move.heading_sign_flips_per_player_min | 20.69 / 19.88 | 19.12 / 18.40 | 19.80 / 19.50 |
| move.lateral_accel_mean_mps2 | 5.55 / 5.57 | 5.48 / 5.41 | 5.71 / 5.76 |
| move.reversals_per_player_min | 0 / 0.01 | 0.02 / 0.02 | 0 / 0 |
| move.side_flips_by_class.0_per_player_min | 207.1 / 194.1 | 182.9 / 163.3 | 187.0 / 177.0 |
| move.side_flips_by_class.0_share_pct | 23.97 / 22.97 | 21.87 / 19.23 | 22.41 / 20.03 |
| move.side_flips_by_class.1_per_player_min | 90.93 / 98.02 | 81.04 / 88.68 | 97.59 / 89.85 |
| move.side_flips_by_class.1_share_pct | 5.32 / 5.26 | 5.29 / 5.32 | 5.27 / 5.08 |
| move.side_flips_by_class.2_per_player_min | 40.21 / 40.87 | 38.97 / 42.85 | 42.07 / 44.24 |
| move.side_flips_by_class.2_share_pct | 54.70 / 55.56 | 55.86 / 60.43 | 57.00 / 58.99 |
| move.side_flips_by_class.3_per_player_min | 36.27 / 37.71 | 35.24 / 34.44 | 34.14 / 38.67 |
| move.side_flips_by_class.3_share_pct | 16.01 / 16.21 | 16.98 / 15.02 | 15.32 / 15.89 |
| move.side_flips_per_player_min | 50.61 / 50.99 | 47.61 / 49.41 | 50.57 / 52.24 |
| move.side_flips_speed_ge3_per_player_min | 59.68 / 59.27 | 55.08 / 56.97 | 58.32 / 59.58 |
| move.side_flips_speed_lt3_per_player_min | 24.29 / 24.66 | 24.25 / 23.54 | 26.17 / 27.27 |
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
| support.carrier_pct_of_open_play | 13.14 / 14.58 | 17.58 / 14.83 | 14.37 / 15.72 |
| support.cluster_pct | 3.70 / 8.61 | 3.41 / 4.85 | 4.58 / 2.65 |
| support.depth_m | 41.43 / 38.64 | 37.71 / 39.23 | 40.16 / 37.02 |
| support.forward_options_mean | 0.78 / 0.65 | 0.50 / 0.87 | 1.02 / 0.62 |
| support.nearest_option_dist_m | 12.68 / 12.28 | 13.70 / 12.32 | 11.99 / 13.25 |
| support.open_options_mean | 3.45 / 3.74 | 3.58 / 4.06 | 3.39 / 3.84 |
| support.open_options_p10 | 0 / 0 | 0 / 0 | 0 / 0 |
| support.pass_target_open_pct | 68.91 / 73.50 | 73.04 / 80.18 | 75.76 / 71.84 |
| support.passes_completed_tracked | 119 / 117 | 115 / 111 | 132 / 103 |
| support.runs_in_behind_per_10min | 3 / 0 | 1 / 0 | 1 / 0 |
| support.ticks_with_le1_option_pct | 19.50 / 17.22 | 20.39 / 16.64 | 26.33 / 19.78 |
| support.width_m | 44.53 / 47.58 | 44.83 / 46.85 | 44.35 / 44.08 |
| trans.count_per_10min | 19 / 9 | 17 / 11 | 16 / 18 |
| trans.counterpress_pct | 47.37 / 44.44 | 47.06 / 45.45 | 43.75 / 44.44 |
| trans.forward_recovery_speed_mps_0_5s | 6.25 / 6.41 | 6.30 / 6.69 | 6.13 / 6.39 |
| trans.forward_sprint_share_0_5s | 66.97 / 68.99 | 66.35 / 74.28 | 63.78 / 67.89 |
| trans.forwards_staying_high_mean | 1.30 / 2.33 | 2.38 / 2 | 2.88 / 1.44 |
| trans.in_shape_at_turnover_pct | 57.14 / 62.50 | 68.75 / 33.33 | 30.77 / 61.54 |
| trans.mid_recovery_speed_mps_0_5s | 5.55 / 5.93 | 5.52 / 6.03 | 5.58 / 5.65 |
| trans.mid_sprint_share_0_5s | 50.95 / 58.97 | 50.99 / 58.30 | 48.89 / 52.20 |
| trans.pct_outfield_behind_ball_at_3s | 66.11 / 65 | 76.67 / 60 | 61.54 / 70.83 |
| trans.pct_outfield_behind_ball_at_6s | 81 / 60 | 71.25 / 68 | 57.50 / 74.44 |
| trans.time_to_shape_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_excluded_pct | 26.32 / 11.11 | 5.88 / 45.45 | 18.75 / 27.78 |
| trans.time_to_shape_s | 1.38 / 1.40 | 0.79 / 1.98 | 2.21 / 0.73 |
| trans.time_to_shape_static_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_static_excluded_pct | 36.84 / 22.22 | 5.88 / 54.55 | 31.25 / 33.33 |
| trans.time_to_shape_static_s | 0.59 / 0.88 | 0.40 / 0.99 | 1.80 / 0.53 |

GK scenario field: not present in the public state.

## 5v5 autopilot-hard vs hard (10 min x 3 seeds from 1)

Command: `psfc-replay analyze --format 5v5 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty hard --away-difficulty hard --home-controller autopilot`

Gameplay version: `0.1.0-5v5`

| seed | final state_hash | score (H-A) |
|---|---|---|
| 1 | `b403bcc1f2736f00` | 7-3 |
| 2 | `1b11ea469462e60c` | 3-3 |
| 3 | `a66646b5730690bd` | 3-4 |

Match meta (mean over seeds): goals_total = 7.67, open_play_minutes = 10.02, open_play_pct_of_ticks = 93.27, shots_total = 42.67, ticks_simulated = 38672.7

### Match

| metric | home | away | teams mean |
|---|---:|---:|---:|
| fouls | 2 | 0 | **1** |
| goals | 4.33 | 3.33 | **3.83** |
| offsides | 1.67 | 1 | **1.33** |
| passes_completed_pct | 76.24 | 79.34 | **77.79** |
| possession_pct | 61.18 | 38.82 | **50** |
| shots | 33.33 | 9.33 | **21.33** |
| shots_on_target | 20.67 | 7.67 | **14.17** |

### Goalkeeper

| metric | home | away | teams mean |
|---|---:|---:|---:|
| catches | 0.67 | 3 | **1.83** |
| claims_non_save | 21.67 | 2.33 | **12** |
| excursion_mean_s | 1.12 | 0.84 | **0.98** |
| excursions_gt1s_per_10min | 3.66 | 0.33 | **2.00** |
| gathers | 22.33 | 5.33 | **13.83** |
| goals_conceded | 3.33 | 4.33 | **3.83** |
| goals_conceded_gk_gt3m_off_line | 0.67 | 2.33 | **1.50** |
| goals_conceded_no_shot_found | 0.67 | 0 | **0.33** |
| line_dist_at_conceded_shot_mean_m | 2.62 | 2.95 | **2.78** |
| line_dist_max_m | 11.29 | 7.68 | **9.48** |
| line_dist_mean_m | 3.07 | 2.93 | **3.00** |
| parries | 4.67 | 16 | **10.33** |
| pct_outside_6yd | 1.31 | 0.14 | **0.72** |
| pct_outside_box | 0.06 | 0 | **0.03** |
| rush_count | 7.33 | 0.67 | **4** |
| rush_per_10min | 7.32 | 0.67 | **3.99** |
| saves | 5.33 | 19 | **12.17** |
| tips_over | 0 | 0 | **0** |

### Defence and shape

| metric | home | away | teams mean |
|---|---:|---:|---:|
| attacking.anchor_dist_m.defender | 10.17 | 9.88 | **10.02** |
| attacking.anchor_dist_m.forward | 10.52 | 16.91 | **13.72** |
| attacking.anchor_dist_m.goalkeeper | 1.34 | 1.36 | **1.35** |
| attacking.anchor_dist_m.midfielder | 13.78 | 14.59 | **14.19** |
| attacking.anchor_dist_shape_m.defender | 6.67 | 8.56 | **7.62** |
| attacking.anchor_dist_shape_m.forward | 9.61 | 11.92 | **10.77** |
| attacking.anchor_dist_shape_m.midfielder | 9.15 | 9.67 | **9.41** |
| attacking.bbox_area_m2 | 545.0 | 336.3 | **440.6** |
| attacking.bbox_depth_m | 21.15 | 17.07 | **19.11** |
| attacking.bbox_width_m | 24.66 | 18.21 | **21.44** |
| attacking.length_m | 21.15 | 17.07 | **19.11** |
| attacking.nearest_mate_mean_m | 13.34 | 10.25 | **11.80** |
| attacking.nearest_mate_p10_m | 4.64 | 3.52 | **4.08** |
| attacking.pct_of_open_play | 62.94 | 37.06 | **50** |
| attacking.stretch_m | 11.93 | 9.21 | **10.57** |
| attacking.width_m | 24.66 | 18.21 | **21.44** |
| defending.anchor_dist_m.defender | 11.47 | 9.42 | **10.45** |
| defending.anchor_dist_m.forward | 12.81 | 14.49 | **13.65** |
| defending.anchor_dist_m.goalkeeper | 1.35 | 1.49 | **1.42** |
| defending.anchor_dist_m.midfielder | 13.33 | 12.02 | **12.67** |
| defending.anchor_dist_shape_m.defender | 8.47 | 9.04 | **8.75** |
| defending.anchor_dist_shape_m.forward | 10.64 | 10.66 | **10.65** |
| defending.anchor_dist_shape_m.midfielder | 8.13 | 8.22 | **8.17** |
| defending.bbox_area_m2 | 413.3 | 296.5 | **354.9** |
| defending.bbox_depth_m | 19.16 | 15.32 | **17.24** |
| defending.bbox_width_m | 21.13 | 18.40 | **19.77** |
| defending.length_m | 19.16 | 15.32 | **17.24** |
| defending.line_x_std_m | 6.12 | 4.04 | **5.08** |
| defending.line_x_variance_m2 | 53.89 | 25.44 | **39.66** |
| defending.nearest_mate_mean_m | 11.79 | 10.05 | **10.92** |
| defending.nearest_mate_p10_m | 4.63 | 3.97 | **4.30** |
| defending.pct_of_open_play | 37.06 | 62.94 | **50** |
| defending.pct_outfield_behind_ball | 67.60 | 66.55 | **67.08** |
| defending.stretch_m | 10.51 | 8.72 | **9.62** |
| defending.swarm2_pct | 1.44 | 1.52 | **1.48** |
| defending.swarm3_pct | 0.03 | 0.00 | **0.02** |
| defending.width_m | 21.13 | 18.40 | **19.77** |

### Defensive transition

| metric | home | away | teams mean |
|---|---:|---:|---:|
| count_per_10min | 33.27 | 52.90 | **43.08** |
| counterpress_pct | 74.77 | 41.87 | **58.32** |
| forward_recovery_speed_mps_0_5s | 6.53 | 6.20 | **6.36** |
| forward_sprint_share_0_5s | 62.52 | 56.00 | **59.26** |
| forwards_staying_high_mean | 0.72 | 0.46 | **0.59** |
| in_shape_at_turnover_pct | 53.54 | 37.30 | **45.42** |
| mid_recovery_speed_mps_0_5s | 6.64 | 6.21 | **6.43** |
| mid_sprint_share_0_5s | 64.95 | 57.22 | **61.08** |
| pct_outfield_behind_ball_at_3s | 71.30 | 48.36 | **59.83** |
| pct_outfield_behind_ball_at_6s | 58.33 | 73.28 | **65.81** |
| time_to_shape_capped_pct | 0 | 0 | **0** |
| time_to_shape_excluded_pct | 34.86 | 33.75 | **34.31** |
| time_to_shape_s | 0.72 | 1.04 | **0.88** |
| time_to_shape_static_capped_pct | 0 | 0 | **0** |
| time_to_shape_static_excluded_pct | 39.92 | 46.17 | **43.04** |
| time_to_shape_static_s | 0.56 | 0.53 | **0.54** |

### Attacking support

| metric | home | away | teams mean |
|---|---:|---:|---:|
| carrier_pct_of_open_play | 18.67 | 12.51 | **15.59** |
| cluster_pct | 4.96 | 11.05 | **8.00** |
| depth_m | 20.57 | 17.43 | **19.00** |
| forward_options_mean | 0.38 | 0.28 | **0.33** |
| nearest_option_dist_m | 14.47 | 12.36 | **13.41** |
| open_options_mean | 1.18 | 0.95 | **1.07** |
| open_options_p10 | 0 | 0 | **0** |
| pass_target_open_pct | 67.37 | 48.67 | **58.02** |
| passes_completed_tracked | 125.3 | 86 | **105.7** |
| runs_in_behind_per_10min | 6.33 | 5.32 | **5.82** |
| ticks_with_le1_option_pct | 57.99 | 68.26 | **63.12** |
| width_m | 22.51 | 17.22 | **19.87** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 12.31 | 11.89 | **12.10** |
| accel_gt10_pct | 51.17 | 53.29 | **52.23** |
| accel_mean_mps2 | 12.25 | 11.78 | **12.02** |
| class_time_pct.0 | 17.17 | 11.39 | **14.28** |
| class_time_pct.1 | 6.79 | 8.51 | **7.65** |
| class_time_pct.2 | 64.83 | 65.45 | **65.14** |
| class_time_pct.3 | 11.21 | 14.65 | **12.93** |
| dspeed_max_mps_per_tick | 4.23 | 6.23 | **5.23** |
| heading_change_p95_deg_per_tick_gt3mps | 3.78 | 3.53 | **3.66** |
| heading_change_pct.0_10 | 98.99 | 99.22 | **99.10** |
| heading_change_pct.10_45 | 0.99 | 0.76 | **0.87** |
| heading_change_pct.45_90 | 0.03 | 0.02 | **0.02** |
| heading_change_pct.gt90 | 0.00 | 0.00 | **0.00** |
| heading_sign_flips_per_player_min | 44.59 | 38.24 | **41.41** |
| lateral_accel_mean_mps2 | 8.45 | 8.23 | **8.34** |
| reversals_per_player_min | 0.05 | 0.03 | **0.04** |
| side_flips_by_class.0_per_player_min | 330.3 | 202.0 | **266.2** |
| side_flips_by_class.0_share_pct | 50.58 | 26.73 | **38.66** |
| side_flips_by_class.1_per_player_min | 152.3 | 90.83 | **121.6** |
| side_flips_by_class.1_share_pct | 9.15 | 8.94 | **9.04** |
| side_flips_by_class.2_per_player_min | 61.22 | 70.98 | **66.10** |
| side_flips_by_class.2_share_pct | 35.40 | 53.94 | **44.67** |
| side_flips_by_class.3_per_player_min | 48.64 | 61.13 | **54.89** |
| side_flips_by_class.3_share_pct | 4.86 | 10.39 | **7.63** |
| side_flips_per_player_min | 112.1 | 86.14 | **99.12** |
| side_flips_speed_ge3_per_player_min | 149.4 | 111.8 | **130.6** |
| side_flips_speed_lt3_per_player_min | 28.77 | 29.81 | **29.29** |
| speed_oscillation_pct | 4.36 | 4.28 | **4.32** |
| turn_rate_mean_deg_per_s | 106.3 | 107.0 | **106.7** |

### Collisions and push-out

| metric | home | away | teams mean |
|---|---:|---:|---:|
| any.depth_max_m | 0.57 | 0.57 | **0.57** |
| any.depth_mean_m | 0.16 | 0.16 | **0.16** |
| any.events_per_min | 1.86 | 1.86 | **1.86** |
| any.stuck_mean_ticks | 5 | 5 | **5** |
| any.stuck_pairs | 0.33 | 0.33 | **0.33** |
| any.ticks_gt1cm_per_min | 3.16 | 3.16 | **3.16** |
| any.ticks_per_min | 3.73 | 3.73 | **3.73** |
| correction_vel_corrected_p50_mps | 0.38 | 0.43 | **0.40** |
| correction_vel_corrected_p99_mps | 13.03 | 13.47 | **13.25** |
| correction_vel_corrected_pct | 0.10 | 0.10 | **0.10** |
| correction_vel_max_mps | 13.58 | 13.87 | **13.72** |
| correction_vel_p50_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p95_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p99_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_pct_gt_0_3 | 0.06 | 0.06 | **0.06** |
| opponent.depth_max_m | 0.13 | 0.13 | **0.13** |
| opponent.depth_mean_m | 0.04 | 0.04 | **0.04** |
| opponent.events_per_min | 1.53 | 1.53 | **1.53** |
| opponent.stuck_mean_ticks | 0 | 0 | **0** |
| opponent.stuck_pairs | 0 | 0 | **0** |
| opponent.ticks_gt1cm_per_min | 1.76 | 1.76 | **1.76** |
| opponent.ticks_per_min | 2.30 | 2.30 | **2.30** |
| teammate.depth_max_m | 0.52 | 0.53 | **0.53** |
| teammate.depth_mean_m | 0.25 | 0.27 | **0.26** |
| teammate.events_per_min | 0.17 | 0.17 | **0.17** |
| teammate.stuck_mean_ticks | 0 | 5 | **2.50** |
| teammate.stuck_pairs | 0 | 0.33 | **0.17** |
| teammate.ticks_gt1cm_per_min | 0.70 | 1.26 | **0.98** |
| teammate.ticks_per_min | 0.70 | 1.30 | **1.00** |

### Coordination

| metric | home | away | teams mean |
|---|---:|---:|---:|
| cruise_over72_pct | 35.16 | 37.60 | **36.38** |
| dual_chaser_pct_of_loose | 13.47 | 17.77 | **15.62** |
| dual_chaser_ticks_per_min | 321.2 | 424.0 | **372.6** |
| engagers3_pct_of_carrier_ticks | 0.25 | 0.33 | **0.29** |
| engagers3_ticks_per_min | 1.10 | 2.40 | **1.75** |
| max_simultaneous_chasers | 4 | 4.67 | **4.33** |

### Aerial

| metric | home | away | teams mean |
|---|---:|---:|---:|
| headers | 20.33 | 10.33 | **15.33** |
| headers_clear | 0.67 | 1 | **0.83** |
| headers_pass | 16.33 | 8.67 | **12.50** |
| headers_shot | 3.33 | 0.67 | **2** |
| jumps_gk | 14 | 25.33 | **19.67** |
| jumps_outfield | 57.33 | 33.33 | **45.33** |
| max_airborne_outfield | 1 | 1 | **1** |

### Team shape vs envelope (teams mean)

| metric | in possession | out of possession | 11v11 envelope (m) |
|---|---:|---:|---|
| length_m | 19.11 | 17.24 | 31-46 |
| width_m | 21.44 | 19.77 | 35-48 |
| stretch_m | 10.57 | 9.62 | 7-16 |
| nearest_mate_mean_m | 11.80 | 10.92 | none published (in-possession mean near 9-10 m, estimate) |
| nearest_mate_p10_m | 4.08 | 4.30 | none published (in-possession mean near 9-10 m, estimate) |

Note: 5v5 has no published team-shape envelope; the 11v11 ranges are printed for reference only and do not apply to a 4-outfielder team on a small pitch.

### Per seed (home / away)

| metric | seed 1 | seed 2 | seed 3 |
|---|---:|---:|---:|
| aerial.headers | 23 / 10 | 18 / 12 | 20 / 9 |
| aerial.headers_clear | 1 / 2 | 1 / 1 | 0 / 0 |
| aerial.headers_pass | 18 / 8 | 13 / 10 | 18 / 8 |
| aerial.headers_shot | 4 / 0 | 4 / 1 | 2 / 1 |
| aerial.jumps_gk | 13 / 27 | 10 / 36 | 19 / 13 |
| aerial.jumps_outfield | 59 / 34 | 51 / 40 | 62 / 26 |
| aerial.max_airborne_outfield | 1 / 1 | 1 / 1 | 1 / 1 |
| collision.any.depth_max_m | 0.79 / 0.79 | 0.79 / 0.79 | 0.14 / 0.14 |
| collision.any.depth_mean_m | 0.20 / 0.20 | 0.23 / 0.23 | 0.04 / 0.04 |
| collision.any.events_per_min | 1.39 / 1.39 | 2.40 / 2.40 | 1.80 / 1.80 |
| collision.any.stuck_mean_ticks | 0 / 0 | 15 / 15 | 0 / 0 |
| collision.any.stuck_pairs | 0 / 0 | 1 / 1 | 0 / 0 |
| collision.any.ticks_gt1cm_per_min | 2.69 / 2.69 | 4.39 / 4.39 | 2.40 / 2.40 |
| collision.any.ticks_per_min | 3.19 / 3.19 | 5.39 / 5.39 | 2.60 / 2.60 |
| collision.correction_vel_corrected_p50_mps | 0.39 / 0.51 | 0.35 / 0.39 | 0.40 / 0.38 |
| collision.correction_vel_corrected_p99_mps | 13.23 / 13.32 | 12.58 / 13.57 | 13.27 / 13.52 |
| collision.correction_vel_corrected_pct | 0.08 / 0.09 | 0.10 / 0.11 | 0.12 / 0.10 |
| collision.correction_vel_max_mps | 13.86 / 13.74 | 13.51 / 13.95 | 13.37 / 13.92 |
| collision.correction_vel_p50_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p95_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p99_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_pct_gt_0_3 | 0.05 / 0.06 | 0.05 / 0.07 | 0.07 / 0.06 |
| collision.opponent.depth_max_m | 0.12 / 0.12 | 0.11 / 0.11 | 0.14 / 0.14 |
| collision.opponent.depth_mean_m | 0.05 / 0.05 | 0.03 / 0.03 | 0.05 / 0.05 |
| collision.opponent.events_per_min | 1.20 / 1.20 | 1.80 / 1.80 | 1.60 / 1.60 |
| collision.opponent.stuck_mean_ticks | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.stuck_pairs | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.ticks_gt1cm_per_min | 1.69 / 1.69 | 1.40 / 1.40 | 2.20 / 2.20 |
| collision.opponent.ticks_per_min | 2.19 / 2.19 | 2.40 / 2.40 | 2.30 / 2.30 |
| collision.teammate.depth_max_m | 0.75 / 0.79 | 0.79 / 0.77 | 0.03 / 0.04 |
| collision.teammate.depth_mean_m | 0.37 / 0.44 | 0.35 / 0.35 | 0.03 / 0.02 |
| collision.teammate.events_per_min | 0.10 / 0.10 | 0.30 / 0.30 | 0.10 / 0.10 |
| collision.teammate.stuck_mean_ticks | 0 / 0 | 0 / 15 | 0 / 0 |
| collision.teammate.stuck_pairs | 0 / 0 | 0 / 1 | 0 / 0 |
| collision.teammate.ticks_gt1cm_per_min | 0.60 / 1.00 | 1.40 / 2.70 | 0.10 / 0.10 |
| collision.teammate.ticks_per_min | 0.60 / 1.00 | 1.40 / 2.70 | 0.10 / 0.20 |
| coord.cruise_over72_pct | 32.81 / 37.66 | 38.17 / 37.24 | 34.51 / 37.90 |
| coord.dual_chaser_pct_of_loose | 12.10 / 17.35 | 13.44 / 21.10 | 14.86 / 14.86 |
| coord.dual_chaser_ticks_per_min | 285.5 / 409.5 | 324.1 / 508.7 | 353.9 / 353.8 |
| coord.engagers3_pct_of_carrier_ticks | 0 / 0.38 | 0.75 / 0.13 | 0 / 0.46 |
| coord.engagers3_ticks_per_min | 0 / 3.09 | 3.29 / 1.00 | 0 / 3.10 |
| coord.max_simultaneous_chasers | 4 / 5 | 4 / 4 | 4 / 5 |
| gk.catches | 1 / 2 | 0 / 2 | 1 / 5 |
| gk.claims_non_save | 21 / 2 | 18 / 3 | 26 / 2 |
| gk.excursion_mean_s | 1.26 / 0 | 1.25 / 1.68 | 0.85 / 0.85 |
| gk.excursions_gt1s_per_10min | 4.98 / 0 | 2.00 / 1.00 | 4 / 0 |
| gk.gathers | 22 / 4 | 18 / 5 | 27 / 7 |
| gk.goals_conceded | 3 / 7 | 3 / 3 | 4 / 3 |
| gk.goals_conceded_gk_gt3m_off_line | 0 / 4 | 0 / 1 | 2 / 2 |
| gk.goals_conceded_no_shot_found | 1 / 0 | 0 / 0 | 1 / 0 |
| gk.line_dist_at_conceded_shot_mean_m | 2.45 / 2.82 | 2.37 / 2.69 | 3.04 / 3.32 |
| gk.line_dist_max_m | 12.46 / 5.30 | 11.63 / 10.73 | 9.77 / 7.00 |
| gk.line_dist_mean_m | 3.12 / 2.92 | 3.08 / 2.86 | 3.01 / 3.01 |
| gk.parries | 2 / 16 | 3 / 22 | 9 / 10 |
| gk.pct_outside_6yd | 1.68 / 0 | 0.83 / 0.28 | 1.42 / 0.14 |
| gk.pct_outside_box | 0.15 / 0 | 0.03 / 0 | 0 / 0 |
| gk.rush_count | 8 / 0 | 4 / 1 | 10 / 1 |
| gk.rush_per_10min | 7.97 / 0 | 3.99 / 1.00 | 10 / 1 |
| gk.saves | 3 / 18 | 3 / 24 | 10 / 15 |
| gk.tips_over | 0 / 0 | 0 / 0 | 0 / 0 |
| match.fouls | 0 / 0 | 2 / 0 | 4 / 0 |
| match.goals | 7 / 3 | 3 / 3 | 3 / 4 |
| match.offsides | 1 / 0 | 2 / 1 | 2 / 2 |
| match.passes_completed_pct | 78.05 / 81.21 | 74.88 / 77.71 | 75.79 / 79.11 |
| match.possession_pct | 65.45 / 34.55 | 63.11 / 36.89 | 54.98 / 45.02 |
| match.shots | 33 / 8 | 47 / 8 | 20 / 12 |
| match.shots_on_target | 22 / 5 | 26 / 6 | 14 / 12 |
| move.accel_from_pos_mean_mps2 | 12.40 / 11.80 | 12.02 / 11.99 | 12.51 / 11.87 |
| move.accel_gt10_pct | 51.87 / 53.04 | 49.42 / 53.74 | 52.22 / 53.09 |
| move.accel_mean_mps2 | 12.35 / 11.69 | 11.97 / 11.88 | 12.43 / 11.78 |
| move.class_time_pct.0 | 17.41 / 10.95 | 17.36 / 11.71 | 16.74 / 11.50 |
| move.class_time_pct.1 | 6.29 / 9.11 | 6.12 / 8.03 | 7.97 / 8.40 |
| move.class_time_pct.2 | 64.81 / 64.88 | 65.27 / 65.82 | 64.42 / 65.64 |
| move.class_time_pct.3 | 11.50 / 15.06 | 11.25 / 14.44 | 10.87 / 14.46 |
| move.dspeed_max_mps_per_tick | 4.06 / 6.73 | 3.84 / 5.90 | 4.79 / 6.07 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.78 / 3.52 | 3.77 / 3.59 | 3.78 / 3.50 |
| move.heading_change_pct.0_10 | 98.97 / 99.28 | 99.01 / 99.15 | 98.97 / 99.23 |
| move.heading_change_pct.10_45 | 1.00 / 0.70 | 0.96 / 0.83 | 1.00 / 0.75 |
| move.heading_change_pct.45_90 | 0.02 / 0.02 | 0.03 / 0.02 | 0.03 / 0.03 |
| move.heading_change_pct.gt90 | 0.00 / 0 | 0.00 / 0.00 | 0.00 / 0.00 |
| move.heading_sign_flips_per_player_min | 42.86 / 35.82 | 46.14 / 42.38 | 44.76 / 36.53 |
| move.lateral_accel_mean_mps2 | 8.56 / 8.26 | 8.29 / 8.27 | 8.49 / 8.16 |
| move.reversals_per_player_min | 0.04 / 0 | 0.02 / 0.04 | 0.08 / 0.06 |
| move.side_flips_by_class.0_per_player_min | 331.8 / 189.6 | 317.7 / 208.4 | 341.6 / 208.0 |
| move.side_flips_by_class.0_share_pct | 51.98 / 24.98 | 49.85 / 26.70 | 49.91 / 28.51 |
| move.side_flips_by_class.1_per_player_min | 163.7 / 86.05 | 153.9 / 104.7 | 139.3 / 81.79 |
| move.side_flips_by_class.1_share_pct | 9.26 / 9.43 | 8.52 / 9.19 | 9.68 / 8.18 |
| move.side_flips_by_class.2_per_player_min | 57.73 / 70.66 | 62.51 / 74.87 | 63.42 / 67.42 |
| move.side_flips_by_class.2_share_pct | 33.67 / 55.17 | 36.88 / 53.93 | 35.65 / 52.72 |
| move.side_flips_by_class.3_per_player_min | 49.09 / 57.49 | 46.74 / 64.39 | 50.10 / 61.51 |
| move.side_flips_by_class.3_share_pct | 5.08 / 10.42 | 4.76 / 10.18 | 4.75 / 10.59 |
| move.side_flips_per_player_min | 111.1 / 83.10 | 110.6 / 91.38 | 114.6 / 83.95 |
| move.side_flips_speed_ge3_per_player_min | 149.8 / 108.5 | 146.6 / 116.5 | 151.8 / 110.3 |
| move.side_flips_speed_lt3_per_player_min | 26.75 / 27.90 | 29.21 / 34.55 | 30.35 / 26.97 |
| move.speed_oscillation_pct | 4.32 / 4.29 | 4.44 / 4.34 | 4.32 / 4.21 |
| move.turn_rate_mean_deg_per_s | 108.6 / 106.8 | 102.2 / 108.3 | 108.3 / 105.9 |
| shape.attacking.anchor_dist_m.defender | 9.57 / 9.44 | 11.84 / 9.24 | 9.09 / 10.95 |
| shape.attacking.anchor_dist_m.forward | 9.99 / 18.15 | 11.32 / 16.17 | 10.26 / 16.43 |
| shape.attacking.anchor_dist_m.goalkeeper | 1.35 / 1.38 | 1.25 / 1.43 | 1.43 / 1.28 |
| shape.attacking.anchor_dist_m.midfielder | 12.45 / 13.36 | 17.13 / 15.75 | 11.75 / 14.68 |
| shape.attacking.anchor_dist_shape_m.defender | 6.52 / 8.35 | 7.05 / 8.22 | 6.45 / 9.11 |
| shape.attacking.anchor_dist_shape_m.forward | 9.43 / 12.43 | 10.59 / 10.84 | 8.83 / 12.49 |
| shape.attacking.anchor_dist_shape_m.midfielder | 8.17 / 8.72 | 11.43 / 10.55 | 7.86 / 9.74 |
| shape.attacking.bbox_area_m2 | 553.1 / 324.1 | 499.6 / 324.6 | 582.2 / 360.2 |
| shape.attacking.bbox_depth_m | 21.02 / 16.64 | 19.92 / 17.61 | 22.51 / 16.97 |
| shape.attacking.bbox_width_m | 25.19 / 17.97 | 23.71 / 17.63 | 25.08 / 19.04 |
| shape.attacking.length_m | 21.02 / 16.64 | 19.92 / 17.61 | 22.51 / 16.97 |
| shape.attacking.nearest_mate_mean_m | 13.55 / 10.01 | 12.64 / 10.24 | 13.83 / 10.51 |
| shape.attacking.nearest_mate_p10_m | 4.81 / 3.51 | 4.03 / 3.56 | 5.09 / 3.49 |
| shape.attacking.pct_of_open_play | 64.90 / 35.10 | 59.92 / 40.08 | 64.01 / 35.99 |
| shape.attacking.stretch_m | 11.98 / 9.00 | 11.40 / 9.28 | 12.40 / 9.35 |
| shape.attacking.width_m | 25.19 / 17.97 | 23.71 / 17.63 | 25.08 / 19.04 |
| shape.defending.anchor_dist_m.defender | 11.03 / 9.62 | 12.19 / 8.76 | 11.19 / 9.90 |
| shape.defending.anchor_dist_m.forward | 12.20 / 13.99 | 12.30 / 16.20 | 13.93 / 13.30 |
| shape.defending.anchor_dist_m.goalkeeper | 1.32 / 1.46 | 1.30 / 1.61 | 1.42 / 1.38 |
| shape.defending.anchor_dist_m.midfielder | 12.03 / 10.39 | 15.66 / 13.14 | 12.30 / 12.52 |
| shape.defending.anchor_dist_shape_m.defender | 7.96 / 9.13 | 8.50 / 9.23 | 8.96 / 8.74 |
| shape.defending.anchor_dist_shape_m.forward | 9.83 / 10.73 | 10.52 / 10.77 | 11.57 / 10.48 |
| shape.defending.anchor_dist_shape_m.midfielder | 6.68 / 7.03 | 10.04 / 9.05 | 7.65 / 8.59 |
| shape.defending.bbox_area_m2 | 437.2 / 288.7 | 408.9 / 280.4 | 393.7 / 320.5 |
| shape.defending.bbox_depth_m | 19.92 / 14.87 | 19.04 / 14.72 | 18.53 / 16.38 |
| shape.defending.bbox_width_m | 21.59 / 18.54 | 20.52 / 18.20 | 21.29 / 18.45 |
| shape.defending.length_m | 19.92 / 14.87 | 19.04 / 14.72 | 18.53 / 16.38 |
| shape.defending.line_x_std_m | 6.28 / 3.89 | 6.31 / 3.68 | 5.78 / 4.56 |
| shape.defending.line_x_variance_m2 | 55.32 / 22.37 | 58.02 / 22.28 | 48.32 / 31.66 |
| shape.defending.nearest_mate_mean_m | 12.29 / 9.92 | 11.53 / 9.82 | 11.55 / 10.42 |
| shape.defending.nearest_mate_p10_m | 5.21 / 3.83 | 4.15 / 3.61 | 4.52 / 4.47 |
| shape.defending.pct_of_open_play | 35.10 / 64.90 | 40.08 / 59.92 | 35.99 / 64.01 |
| shape.defending.pct_outfield_behind_ball | 68.48 / 65.42 | 67.79 / 61.48 | 66.53 / 72.76 |
| shape.defending.stretch_m | 10.80 / 8.63 | 10.30 / 8.58 | 10.42 / 8.97 |
| shape.defending.swarm2_pct | 1.07 / 1.87 | 2.19 / 1.25 | 1.06 / 1.43 |
| shape.defending.swarm3_pct | 0 / 0 | 0.09 / 0.01 | 0 / 0 |
| shape.defending.width_m | 21.59 / 18.54 | 20.52 / 18.20 | 21.29 / 18.45 |
| support.carrier_pct_of_open_play | 20.54 / 11.49 | 19.41 / 11.55 | 16.05 / 14.47 |
| support.cluster_pct | 5.16 / 7.73 | 7.57 / 10.01 | 2.15 / 15.41 |
| support.depth_m | 20.23 / 16.94 | 19.77 / 18.62 | 21.71 / 16.75 |
| support.forward_options_mean | 0.41 / 0.28 | 0.32 / 0.36 | 0.42 / 0.21 |
| support.nearest_option_dist_m | 14.64 / 12.28 | 14.06 / 12.56 | 14.70 / 12.22 |
| support.open_options_mean | 1.22 / 1.04 | 1.11 / 1.02 | 1.23 / 0.80 |
| support.open_options_p10 | 0 / 0 | 0 / 0 | 0 / 0 |
| support.pass_target_open_pct | 73.68 / 53.19 | 63.41 / 44.71 | 65 / 48.10 |
| support.passes_completed_tracked | 133 / 94 | 123 / 85 | 120 / 79 |
| support.runs_in_behind_per_10min | 2.99 / 6.97 | 5.99 / 3.99 | 10 / 5 |
| support.ticks_with_le1_option_pct | 57.44 / 66.64 | 60.66 / 65.44 | 55.87 / 72.69 |
| support.width_m | 23.18 / 16.94 | 21.64 / 16.02 | 22.72 / 18.70 |
| trans.count_per_10min | 33.86 / 56.76 | 34.95 / 47.93 | 31 / 54 |
| trans.counterpress_pct | 79.41 / 47.37 | 77.14 / 37.50 | 67.74 / 40.74 |
| trans.forward_recovery_speed_mps_0_5s | 6.76 / 6.46 | 6.45 / 5.76 | 6.37 / 6.38 |
| trans.forward_sprint_share_0_5s | 66.29 / 61.63 | 61.43 / 47.22 | 59.82 / 59.14 |
| trans.forwards_staying_high_mean | 1 / 0.44 | 0.67 / 0.57 | 0.50 / 0.36 |
| trans.in_shape_at_turnover_pct | 47.37 / 38.46 | 54.17 / 25.93 | 59.09 / 47.50 |
| trans.mid_recovery_speed_mps_0_5s | 6.74 / 6.42 | 6.72 / 6.02 | 6.47 / 6.19 |
| trans.mid_sprint_share_0_5s | 65.61 / 61.63 | 67.15 / 53.08 | 62.08 / 56.94 |
| trans.pct_outfield_behind_ball_at_3s | 68.75 / 35.83 | 72.92 / 41.67 | 72.22 / 67.59 |
| trans.pct_outfield_behind_ball_at_6s | 37.50 / 80.56 | 75 / 57.14 | 62.50 / 82.14 |
| trans.time_to_shape_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_excluded_pct | 44.12 / 31.58 | 31.43 / 43.75 | 29.03 / 25.93 |
| trans.time_to_shape_s | 1.08 / 1.13 | 0.48 / 1.27 | 0.60 / 0.71 |
| trans.time_to_shape_static_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_static_excluded_pct | 50 / 40.35 | 34.29 / 66.67 | 35.48 / 31.48 |
| trans.time_to_shape_static_s | 0.47 / 0.60 | 0.58 / 0.40 | 0.63 / 0.58 |

GK scenario field: not present in the public state.

## 11v11 autopilot-normal vs normal (10 min x 3 seeds from 1)

Command: `psfc-replay analyze --format 11v11 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty normal --away-difficulty normal --home-controller autopilot`

Gameplay version: `0.1.0-5v5`

| seed | final state_hash | score (H-A) |
|---|---|---|
| 1 | `c6a2d8b919094925` | 1-1 |
| 2 | `d246d7e3bef1c8ab` | 5-0 |
| 3 | `18743e40f6674a8b` | 3-0 |

Match meta (mean over seeds): goals_total = 3.33, open_play_minutes = 10, open_play_pct_of_ticks = 96.09, shots_total = 22.67, ticks_simulated = 37469.7

### Match

| metric | home | away | teams mean |
|---|---:|---:|---:|
| fouls | 0 | 0 | **0** |
| goals | 3 | 0.33 | **1.67** |
| offsides | 1.33 | 0 | **0.67** |
| passes_completed_pct | 80.79 | 79.32 | **80.06** |
| possession_pct | 58.57 | 41.43 | **50** |
| shots | 20.33 | 2.33 | **11.33** |
| shots_on_target | 11.67 | 1 | **6.33** |

### Goalkeeper

| metric | home | away | teams mean |
|---|---:|---:|---:|
| catches | 0.33 | 2.33 | **1.33** |
| claims_non_save | 9 | 1.67 | **5.33** |
| excursion_mean_s | 1.30 | 0.97 | **1.13** |
| excursions_gt1s_per_10min | 3.67 | 4.33 | **4** |
| gathers | 9.33 | 4 | **6.67** |
| goals_conceded | 0.33 | 3 | **1.67** |
| goals_conceded_gk_gt3m_off_line | 0.33 | 2.33 | **1.33** |
| goals_conceded_no_shot_found | 0 | 0 | **0** |
| line_dist_at_conceded_shot_mean_m | 1.23 | 5.45 | **3.34** |
| line_dist_max_m | 11.22 | 9.33 | **10.28** |
| line_dist_mean_m | 4.33 | 4.49 | **4.41** |
| parries | 0.33 | 8.67 | **4.50** |
| pct_outside_6yd | 1.25 | 1.26 | **1.25** |
| pct_outside_box | 0 | 0 | **0** |
| rush_count | 5.67 | 8 | **6.83** |
| rush_per_10min | 5.67 | 8 | **6.83** |
| saves | 0.67 | 11 | **5.83** |
| tips_over | 0 | 0 | **0** |

### Defence and shape

| metric | home | away | teams mean |
|---|---:|---:|---:|
| attacking.anchor_dist_m.defender | 23.20 | 19.96 | **21.58** |
| attacking.anchor_dist_m.forward | 21.38 | 22.87 | **22.12** |
| attacking.anchor_dist_m.goalkeeper | 2.43 | 2.24 | **2.34** |
| attacking.anchor_dist_m.midfielder | 18.56 | 17.93 | **18.25** |
| attacking.anchor_dist_shape_m.defender | 19.06 | 14.70 | **16.88** |
| attacking.anchor_dist_shape_m.forward | 16.66 | 18.09 | **17.37** |
| attacking.anchor_dist_shape_m.midfielder | 12.76 | 10.18 | **11.47** |
| attacking.bbox_area_m2 | 2266.3 | 1869.1 | **2067.7** |
| attacking.bbox_depth_m | 45.81 | 40.09 | **42.95** |
| attacking.bbox_width_m | 48.19 | 45.11 | **46.65** |
| attacking.length_m | 45.81 | 40.09 | **42.95** |
| attacking.nearest_mate_mean_m | 13.54 | 12.03 | **12.79** |
| attacking.nearest_mate_p10_m | 6.59 | 5.37 | **5.98** |
| attacking.pct_of_open_play | 61.62 | 38.38 | **50** |
| attacking.stretch_m | 21.32 | 19.51 | **20.42** |
| attacking.width_m | 48.19 | 45.11 | **46.65** |
| defending.anchor_dist_m.defender | 21.73 | 20.76 | **21.24** |
| defending.anchor_dist_m.forward | 23.21 | 23.24 | **23.22** |
| defending.anchor_dist_m.goalkeeper | 2.59 | 2.51 | **2.55** |
| defending.anchor_dist_m.midfielder | 18.08 | 16.74 | **17.41** |
| defending.anchor_dist_shape_m.defender | 17.53 | 15.99 | **16.76** |
| defending.anchor_dist_shape_m.forward | 18.26 | 20.24 | **19.25** |
| defending.anchor_dist_shape_m.midfielder | 11.53 | 10.42 | **10.97** |
| defending.bbox_area_m2 | 1741.9 | 1376.7 | **1559.3** |
| defending.bbox_depth_m | 40.77 | 35.24 | **38.00** |
| defending.bbox_width_m | 41.37 | 37.25 | **39.31** |
| defending.length_m | 40.77 | 35.24 | **38.00** |
| defending.line_x_std_m | 7.12 | 4.71 | **5.92** |
| defending.line_x_variance_m2 | 78.80 | 42.92 | **60.86** |
| defending.nearest_mate_mean_m | 11.44 | 10.36 | **10.90** |
| defending.nearest_mate_p10_m | 6.24 | 5.30 | **5.77** |
| defending.pct_of_open_play | 38.38 | 61.62 | **50** |
| defending.pct_outfield_behind_ball | 69.57 | 73.67 | **71.62** |
| defending.stretch_m | 18.36 | 16.13 | **17.25** |
| defending.swarm2_pct | 1.02 | 0.34 | **0.68** |
| defending.swarm3_pct | 0.01 | 0 | **0.00** |
| defending.width_m | 41.37 | 37.25 | **39.31** |

### Defensive transition

| metric | home | away | teams mean |
|---|---:|---:|---:|
| count_per_10min | 21.67 | 35.67 | **28.67** |
| counterpress_pct | 71.70 | 36.24 | **53.97** |
| forward_recovery_speed_mps_0_5s | 6.51 | 6.07 | **6.29** |
| forward_sprint_share_0_5s | 66.60 | 59.08 | **62.84** |
| forwards_staying_high_mean | 2.08 | 1.25 | **1.67** |
| in_shape_at_turnover_pct | 66.99 | 62.87 | **64.93** |
| mid_recovery_speed_mps_0_5s | 5.79 | 5.57 | **5.68** |
| mid_sprint_share_0_5s | 53.38 | 49.31 | **51.35** |
| pct_outfield_behind_ball_at_3s | 73.21 | 71.04 | **72.12** |
| pct_outfield_behind_ball_at_6s | 68.89 | 78.99 | **73.94** |
| time_to_shape_capped_pct | 0 | 0 | **0** |
| time_to_shape_excluded_pct | 16.36 | 21.53 | **18.94** |
| time_to_shape_s | 0.92 | 0.88 | **0.90** |
| time_to_shape_static_capped_pct | 0 | 0 | **0** |
| time_to_shape_static_excluded_pct | 28.63 | 33.07 | **30.85** |
| time_to_shape_static_s | 0.59 | 0.52 | **0.55** |

### Attacking support

| metric | home | away | teams mean |
|---|---:|---:|---:|
| carrier_pct_of_open_play | 16.28 | 11.37 | **13.82** |
| cluster_pct | 4.78 | 10.01 | **7.39** |
| depth_m | 42.94 | 38.59 | **40.77** |
| forward_options_mean | 0.87 | 0.62 | **0.74** |
| nearest_option_dist_m | 13.44 | 11.97 | **12.70** |
| open_options_mean | 3.14 | 2.58 | **2.86** |
| open_options_p10 | 0 | 0 | **0** |
| pass_target_open_pct | 73.75 | 50.02 | **61.88** |
| passes_completed_tracked | 134.3 | 87.67 | **111** |
| runs_in_behind_per_10min | 1.33 | 0 | **0.67** |
| ticks_with_le1_option_pct | 28.07 | 45.71 | **36.89** |
| width_m | 45.27 | 44.93 | **45.10** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 8.71 | 8.52 | **8.61** |
| accel_gt10_pct | 38.55 | 39.69 | **39.12** |
| accel_mean_mps2 | 8.70 | 8.49 | **8.60** |
| class_time_pct.0 | 7.90 | 5.22 | **6.56** |
| class_time_pct.1 | 2.86 | 3.77 | **3.32** |
| class_time_pct.2 | 66.17 | 72.56 | **69.37** |
| class_time_pct.3 | 23.07 | 18.45 | **20.76** |
| dspeed_max_mps_per_tick | 3.89 | 5.26 | **4.57** |
| heading_change_p95_deg_per_tick_gt3mps | 3.31 | 3.29 | **3.30** |
| heading_change_pct.0_10 | 99.53 | 99.60 | **99.57** |
| heading_change_pct.10_45 | 0.46 | 0.39 | **0.42** |
| heading_change_pct.45_90 | 0.01 | 0.01 | **0.01** |
| heading_change_pct.gt90 | 0.00 | 0.00 | **0.00** |
| heading_sign_flips_per_player_min | 24.04 | 20.67 | **22.36** |
| lateral_accel_mean_mps2 | 5.94 | 6.00 | **5.97** |
| reversals_per_player_min | 0.00 | 0.01 | **0.00** |
| side_flips_by_class.0_per_player_min | 290.1 | 169.3 | **229.7** |
| side_flips_by_class.0_share_pct | 38.33 | 17.73 | **28.03** |
| side_flips_by_class.1_per_player_min | 142.4 | 80.71 | **111.6** |
| side_flips_by_class.1_share_pct | 6.79 | 6.07 | **6.43** |
| side_flips_by_class.2_per_player_min | 36.68 | 42.06 | **39.37** |
| side_flips_by_class.2_share_pct | 40.52 | 61.33 | **50.92** |
| side_flips_by_class.3_per_player_min | 37.42 | 40.14 | **38.78** |
| side_flips_by_class.3_share_pct | 14.37 | 14.87 | **14.62** |
| side_flips_per_player_min | 59.84 | 49.78 | **54.81** |
| side_flips_speed_ge3_per_player_min | 71.41 | 56.94 | **64.18** |
| side_flips_speed_lt3_per_player_min | 27.15 | 28.22 | **27.68** |
| speed_oscillation_pct | 3.15 | 3.06 | **3.11** |
| turn_rate_mean_deg_per_s | 82.35 | 85.04 | **83.70** |

### Collisions and push-out

| metric | home | away | teams mean |
|---|---:|---:|---:|
| any.depth_max_m | 0.80 | 0.80 | **0.80** |
| any.depth_mean_m | 0.31 | 0.31 | **0.31** |
| any.events_per_min | 2.57 | 2.57 | **2.57** |
| any.stuck_mean_ticks | 10.33 | 10.33 | **10.33** |
| any.stuck_pairs | 0.67 | 0.67 | **0.67** |
| any.ticks_gt1cm_per_min | 8.57 | 8.57 | **8.57** |
| any.ticks_per_min | 9.20 | 9.20 | **9.20** |
| correction_vel_corrected_p50_mps | 1.06 | 1.80 | **1.43** |
| correction_vel_corrected_p99_mps | 9.88 | 12.38 | **11.13** |
| correction_vel_corrected_pct | 0.04 | 0.04 | **0.04** |
| correction_vel_max_mps | 13.21 | 12.90 | **13.05** |
| correction_vel_p50_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p95_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p99_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_pct_gt_0_3 | 0.04 | 0.04 | **0.04** |
| opponent.depth_max_m | 0.30 | 0.30 | **0.30** |
| opponent.depth_mean_m | 0.06 | 0.06 | **0.06** |
| opponent.events_per_min | 1.53 | 1.53 | **1.53** |
| opponent.stuck_mean_ticks | 0 | 0 | **0** |
| opponent.stuck_pairs | 0 | 0 | **0** |
| opponent.ticks_gt1cm_per_min | 2.10 | 2.10 | **2.10** |
| opponent.ticks_per_min | 2.63 | 2.63 | **2.63** |
| teammate.depth_max_m | 0.78 | 0.80 | **0.79** |
| teammate.depth_mean_m | 0.39 | 0.39 | **0.39** |
| teammate.events_per_min | 0.53 | 0.50 | **0.52** |
| teammate.stuck_mean_ticks | 5.33 | 5 | **5.17** |
| teammate.stuck_pairs | 0.33 | 0.33 | **0.33** |
| teammate.ticks_gt1cm_per_min | 3.87 | 4.03 | **3.95** |
| teammate.ticks_per_min | 3.93 | 4.07 | **4** |

### Coordination

| metric | home | away | teams mean |
|---|---:|---:|---:|
| cruise_over72_pct | 34.90 | 35.25 | **35.07** |
| dual_chaser_pct_of_loose | 11.39 | 13.09 | **12.24** |
| dual_chaser_ticks_per_min | 293.4 | 336.7 | **315.1** |
| engagers3_pct_of_carrier_ticks | 0.08 | 0.05 | **0.06** |
| engagers3_ticks_per_min | 0.33 | 0.27 | **0.30** |
| max_simultaneous_chasers | 3 | 4.33 | **3.67** |

### Aerial

| metric | home | away | teams mean |
|---|---:|---:|---:|
| headers | 20.67 | 18.67 | **19.67** |
| headers_clear | 0.67 | 0 | **0.33** |
| headers_pass | 17.33 | 17.33 | **17.33** |
| headers_shot | 2.67 | 1.33 | **2** |
| jumps_gk | 4.67 | 19 | **11.83** |
| jumps_outfield | 55.33 | 39.33 | **47.33** |
| max_airborne_outfield | 1 | 1 | **1** |

### Team shape vs envelope (teams mean)

| metric | in possession | out of possession | 11v11 envelope (m) |
|---|---:|---:|---|
| length_m | 42.95 | 38.00 | 31-46 |
| width_m | 46.65 | 39.31 | 35-48 |
| stretch_m | 20.42 (outside) | 17.25 (outside) | 7-16 |
| nearest_mate_mean_m | 12.79 | 10.90 | none published (in-possession mean near 9-10 m, estimate) |
| nearest_mate_p10_m | 5.98 | 5.77 | none published (in-possession mean near 9-10 m, estimate) |

Envelope: peer-reviewed 11v11 ranges (length 31-46 m, width 35-48 m, stretch index 7-16 m) quoted in the research report; the report's tighter targets are 31-36 / 35-40 / 7-10 m out of possession and 40-46 / 42-48 / 12-16 m in possession (estimates).

### Per seed (home / away)

| metric | seed 1 | seed 2 | seed 3 |
|---|---:|---:|---:|
| aerial.headers | 15 / 15 | 19 / 17 | 28 / 24 |
| aerial.headers_clear | 0 / 0 | 2 / 0 | 0 / 0 |
| aerial.headers_pass | 14 / 13 | 14 / 15 | 24 / 24 |
| aerial.headers_shot | 1 / 2 | 3 / 2 | 4 / 0 |
| aerial.jumps_gk | 9 / 10 | 4 / 26 | 1 / 21 |
| aerial.jumps_outfield | 48 / 32 | 52 / 36 | 66 / 50 |
| aerial.max_airborne_outfield | 1 / 1 | 1 / 1 | 1 / 1 |
| collision.any.depth_max_m | 0.80 / 0.80 | 0.80 / 0.80 | 0.80 / 0.80 |
| collision.any.depth_mean_m | 0.33 / 0.33 | 0.32 / 0.32 | 0.27 / 0.27 |
| collision.any.events_per_min | 2.30 / 2.30 | 2.50 / 2.50 | 2.90 / 2.90 |
| collision.any.stuck_mean_ticks | 16 / 16 | 0 / 0 | 15 / 15 |
| collision.any.stuck_pairs | 1 / 1 | 0 / 0 | 1 / 1 |
| collision.any.ticks_gt1cm_per_min | 10.50 / 10.50 | 8.40 / 8.40 | 6.80 / 6.80 |
| collision.any.ticks_per_min | 10.90 / 10.90 | 9 / 9 | 7.70 / 7.70 |
| collision.correction_vel_corrected_p50_mps | 1.80 / 1.80 | 0.89 / 1.80 | 0.47 / 1.80 |
| collision.correction_vel_corrected_p99_mps | 12.52 / 12.58 | 3.60 / 12.30 | 13.53 / 12.26 |
| collision.correction_vel_corrected_pct | 0.05 / 0.04 | 0.04 / 0.04 | 0.04 / 0.05 |
| collision.correction_vel_max_mps | 12.73 / 12.72 | 13.32 / 13.24 | 13.58 / 12.74 |
| collision.correction_vel_p50_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p95_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p99_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_pct_gt_0_3 | 0.04 / 0.03 | 0.04 / 0.04 | 0.03 / 0.04 |
| collision.opponent.depth_max_m | 0.36 / 0.36 | 0.36 / 0.36 | 0.17 / 0.17 |
| collision.opponent.depth_mean_m | 0.08 / 0.08 | 0.06 / 0.06 | 0.04 / 0.04 |
| collision.opponent.events_per_min | 1.10 / 1.10 | 1.40 / 1.40 | 2.10 / 2.10 |
| collision.opponent.stuck_mean_ticks | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.stuck_pairs | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.ticks_gt1cm_per_min | 2.50 / 2.50 | 1.80 / 1.80 | 2 / 2 |
| collision.opponent.ticks_per_min | 2.80 / 2.80 | 2.20 / 2.20 | 2.90 / 2.90 |
| collision.teammate.depth_max_m | 0.80 / 0.80 | 0.80 / 0.80 | 0.74 / 0.80 |
| collision.teammate.depth_mean_m | 0.40 / 0.39 | 0.39 / 0.40 | 0.38 / 0.38 |
| collision.teammate.events_per_min | 0.70 / 0.50 | 0.60 / 0.50 | 0.30 / 0.50 |
| collision.teammate.stuck_mean_ticks | 16 / 0 | 0 / 0 | 0 / 15 |
| collision.teammate.stuck_pairs | 1 / 0 | 0 / 0 | 0 / 1 |
| collision.teammate.ticks_gt1cm_per_min | 6.40 / 3.50 | 3.70 / 4.10 | 1.50 / 4.50 |
| collision.teammate.ticks_per_min | 6.50 / 3.50 | 3.80 / 4.20 | 1.50 / 4.50 |
| coord.cruise_over72_pct | 33.07 / 34.33 | 37.41 / 36.20 | 34.22 / 35.21 |
| coord.dual_chaser_pct_of_loose | 12.49 / 10.89 | 11.85 / 13.98 | 9.83 / 14.40 |
| coord.dual_chaser_ticks_per_min | 327.1 / 285.3 | 298.8 / 352.5 | 254.2 / 372.4 |
| coord.engagers3_pct_of_carrier_ticks | 0 / 0 | 0 / 0.14 | 0.24 / 0 |
| coord.engagers3_ticks_per_min | 0 / 0 | 0 / 0.80 | 1 / 0 |
| coord.max_simultaneous_chasers | 3 / 4 | 3 / 4 | 3 / 5 |
| gk.catches | 1 / 0 | 0 / 1 | 0 / 6 |
| gk.claims_non_save | 16 / 0 | 7 / 3 | 4 / 2 |
| gk.excursion_mean_s | 1.39 / 1.02 | 1.46 / 1.02 | 1.04 / 0.87 |
| gk.excursions_gt1s_per_10min | 6 / 1 | 3 / 6 | 2 / 6 |
| gk.gathers | 17 / 0 | 7 / 4 | 4 / 8 |
| gk.goals_conceded | 1 / 1 | 0 / 5 | 0 / 3 |
| gk.goals_conceded_gk_gt3m_off_line | 1 / 1 | 0 / 5 | 0 / 1 |
| gk.goals_conceded_no_shot_found | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.line_dist_at_conceded_shot_mean_m | 3.70 / 9.65 | 0 / 4.07 | 0 / 2.63 |
| gk.line_dist_max_m | 16.42 / 10.89 | 5.44 / 8.12 | 11.80 / 8.99 |
| gk.line_dist_mean_m | 4.23 / 4.63 | 4.17 / 4.44 | 4.58 / 4.40 |
| gk.parries | 1 / 5 | 0 / 10 | 0 / 11 |
| gk.pct_outside_6yd | 2.09 / 0.51 | 0.97 / 1.53 | 0.69 / 1.73 |
| gk.pct_outside_box | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_count | 9 / 3 | 4 / 9 | 4 / 12 |
| gk.rush_per_10min | 9 / 3 | 4 / 9 | 4 / 12 |
| gk.saves | 2 / 5 | 0 / 11 | 0 / 17 |
| gk.tips_over | 0 / 0 | 0 / 0 | 0 / 0 |
| match.fouls | 0 / 0 | 0 / 0 | 0 / 0 |
| match.goals | 1 / 1 | 5 / 0 | 3 / 0 |
| match.offsides | 1 / 0 | 1 / 0 | 2 / 0 |
| match.passes_completed_pct | 82.55 / 80.31 | 79.49 / 77.27 | 80.35 / 80.37 |
| match.possession_pct | 63.01 / 36.99 | 54.47 / 45.53 | 58.24 / 41.76 |
| match.shots | 13 / 4 | 23 / 2 | 25 / 1 |
| match.shots_on_target | 5 / 3 | 15 / 0 | 15 / 0 |
| move.accel_from_pos_mean_mps2 | 8.74 / 8.19 | 8.79 / 8.52 | 8.62 / 8.84 |
| move.accel_gt10_pct | 38.54 / 38.12 | 38.77 / 39.48 | 38.34 / 41.47 |
| move.accel_mean_mps2 | 8.73 / 8.17 | 8.78 / 8.50 | 8.60 / 8.81 |
| move.class_time_pct.0 | 8.07 / 4.56 | 7.73 / 5.41 | 7.90 / 5.68 |
| move.class_time_pct.1 | 2.91 / 4.43 | 3.07 / 3.59 | 2.59 / 3.30 |
| move.class_time_pct.2 | 64.18 / 73.18 | 68.12 / 72.31 | 66.20 / 72.19 |
| move.class_time_pct.3 | 24.83 / 17.82 | 21.08 / 18.68 | 23.31 / 18.83 |
| move.dspeed_max_mps_per_tick | 3.88 / 4.89 | 3.73 / 5.68 | 4.05 / 5.21 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.32 / 3.27 | 3.30 / 3.27 | 3.31 / 3.33 |
| move.heading_change_pct.0_10 | 99.48 / 99.66 | 99.59 / 99.61 | 99.53 / 99.53 |
| move.heading_change_pct.10_45 | 0.51 / 0.33 | 0.41 / 0.38 | 0.46 / 0.46 |
| move.heading_change_pct.45_90 | 0.01 / 0.01 | 0.01 / 0.01 | 0.01 / 0.01 |
| move.heading_change_pct.gt90 | 0 / 0.00 | 0.00 / 0 | 0 / 0.00 |
| move.heading_sign_flips_per_player_min | 24.58 / 20.50 | 23.90 / 19.64 | 23.64 / 21.88 |
| move.lateral_accel_mean_mps2 | 5.85 / 5.84 | 6.00 / 6.02 | 5.98 / 6.14 |
| move.reversals_per_player_min | 0 / 0.01 | 0.01 / 0 | 0 / 0.01 |
| move.side_flips_by_class.0_per_player_min | 283.9 / 168.4 | 301.8 / 161.4 | 284.8 / 178.2 |
| move.side_flips_by_class.0_share_pct | 37.71 / 15.82 | 37.54 / 17.74 | 39.73 / 19.64 |
| move.side_flips_by_class.1_per_player_min | 143.6 / 74.26 | 142.8 / 76.55 | 140.8 / 91.33 |
| move.side_flips_by_class.1_share_pct | 6.89 / 6.78 | 7.04 / 5.58 | 6.44 / 5.84 |
| move.side_flips_by_class.2_per_player_min | 38.64 / 41.27 | 37.74 / 41.94 | 33.66 / 42.98 |
| move.side_flips_by_class.2_share_pct | 40.83 / 62.22 | 41.36 / 61.56 | 39.36 / 60.20 |
| move.side_flips_by_class.3_per_player_min | 35.65 / 41.35 | 41.45 / 39.90 | 35.16 / 39.17 |
| move.side_flips_by_class.3_share_pct | 14.57 / 15.18 | 14.06 / 15.13 | 14.47 / 14.31 |
| move.side_flips_per_player_min | 60.75 / 48.55 | 62.16 / 49.27 | 56.62 / 51.54 |
| move.side_flips_speed_ge3_per_player_min | 73.51 / 55.68 | 72.58 / 56.65 | 68.15 / 58.50 |
| move.side_flips_speed_lt3_per_player_min | 28.79 / 28.08 | 28.36 / 25.52 | 24.29 / 31.06 |
| move.speed_oscillation_pct | 3.29 / 3.11 | 3.10 / 2.95 | 3.06 / 3.13 |
| move.turn_rate_mean_deg_per_s | 83.34 / 82.26 | 80.55 / 83.81 | 83.17 / 89.05 |
| shape.attacking.anchor_dist_m.defender | 22.97 / 20.16 | 20.80 / 20.82 | 25.83 / 18.91 |
| shape.attacking.anchor_dist_m.forward | 19.65 / 24.34 | 23.04 / 21.40 | 21.45 / 22.86 |
| shape.attacking.anchor_dist_m.goalkeeper | 2.42 / 2.17 | 2.53 / 2.26 | 2.35 / 2.30 |
| shape.attacking.anchor_dist_m.midfielder | 18.57 / 18.53 | 18.55 / 18.09 | 18.56 / 17.19 |
| shape.attacking.anchor_dist_shape_m.defender | 21.04 / 14.28 | 16.61 / 14.03 | 19.53 / 15.79 |
| shape.attacking.anchor_dist_shape_m.forward | 15.35 / 20.38 | 17.72 / 16.91 | 16.91 / 16.98 |
| shape.attacking.anchor_dist_shape_m.midfielder | 13.03 / 10.83 | 12.30 / 9.67 | 12.96 / 10.05 |
| shape.attacking.bbox_area_m2 | 2378.4 / 1826.9 | 2153.9 / 1860.4 | 2266.7 / 1920.0 |
| shape.attacking.bbox_depth_m | 47.20 / 39.40 | 44.67 / 39.78 | 45.56 / 41.09 |
| shape.attacking.bbox_width_m | 49.22 / 45.10 | 46.93 / 45.42 | 48.41 / 44.82 |
| shape.attacking.length_m | 47.20 / 39.40 | 44.67 / 39.78 | 45.56 / 41.09 |
| shape.attacking.nearest_mate_mean_m | 13.98 / 11.72 | 13.05 / 11.91 | 13.59 / 12.48 |
| shape.attacking.nearest_mate_p10_m | 6.49 / 4.82 | 6.26 / 5.20 | 7.02 / 6.10 |
| shape.attacking.pct_of_open_play | 68.26 / 31.74 | 58.48 / 41.52 | 58.12 / 41.88 |
| shape.attacking.stretch_m | 21.90 / 19.19 | 20.76 / 19.68 | 21.31 / 19.64 |
| shape.attacking.width_m | 49.22 / 45.10 | 46.93 / 45.42 | 48.41 / 44.82 |
| shape.defending.anchor_dist_m.defender | 22.38 / 22.70 | 19.75 / 21.30 | 23.05 / 18.27 |
| shape.defending.anchor_dist_m.forward | 23.24 / 23.63 | 24.59 / 21.16 | 21.79 / 24.93 |
| shape.defending.anchor_dist_m.goalkeeper | 2.58 / 2.46 | 2.65 / 2.53 | 2.53 / 2.54 |
| shape.defending.anchor_dist_m.midfielder | 18.55 / 15.71 | 17.89 / 16.75 | 17.81 / 17.77 |
| shape.defending.anchor_dist_shape_m.defender | 18.68 / 16.13 | 16.60 / 15.68 | 17.31 / 16.16 |
| shape.defending.anchor_dist_shape_m.forward | 19.23 / 23.36 | 18.16 / 18.46 | 17.41 / 18.91 |
| shape.defending.anchor_dist_shape_m.midfielder | 12.45 / 10.00 | 10.49 / 10.09 | 11.65 / 11.17 |
| shape.defending.bbox_area_m2 | 1749.5 / 1284.0 | 1704.0 / 1451.8 | 1772.2 / 1394.2 |
| shape.defending.bbox_depth_m | 40.56 / 34.52 | 40.35 / 35.93 | 41.40 / 35.26 |
| shape.defending.bbox_width_m | 41.68 / 35.78 | 40.95 / 38.52 | 41.48 / 37.45 |
| shape.defending.length_m | 40.56 / 34.52 | 40.35 / 35.93 | 41.40 / 35.26 |
| shape.defending.line_x_std_m | 7.81 / 4.27 | 6.85 / 5.08 | 6.69 / 4.80 |
| shape.defending.line_x_variance_m2 | 84.56 / 38.41 | 77.81 / 46.75 | 74.03 / 43.61 |
| shape.defending.nearest_mate_mean_m | 11.33 / 10.06 | 11.27 / 10.69 | 11.72 / 10.33 |
| shape.defending.nearest_mate_p10_m | 5.86 / 5.23 | 5.90 / 5.53 | 6.97 / 5.14 |
| shape.defending.pct_of_open_play | 31.74 / 68.26 | 41.52 / 58.48 | 41.88 / 58.12 |
| shape.defending.pct_outfield_behind_ball | 64.66 / 81.71 | 66.88 / 72.19 | 77.17 / 67.12 |
| shape.defending.stretch_m | 18.42 / 15.53 | 18.19 / 16.68 | 18.48 / 16.19 |
| shape.defending.swarm2_pct | 1.87 / 0.39 | 0.98 / 0 | 0.21 / 0.64 |
| shape.defending.swarm3_pct | 0 / 0 | 0 / 0 | 0.02 / 0 |
| shape.defending.width_m | 41.68 / 35.78 | 40.95 / 38.52 | 41.48 / 37.45 |
| support.carrier_pct_of_open_play | 16.51 / 10.08 | 16.06 / 13.10 | 16.27 / 10.92 |
| support.cluster_pct | 3.64 / 11.49 | 5.81 / 10.82 | 4.88 / 7.73 |
| support.depth_m | 44.06 / 36.87 | 42.15 / 38.29 | 42.62 / 40.61 |
| support.forward_options_mean | 0.95 / 0.52 | 0.89 / 0.42 | 0.77 / 0.91 |
| support.nearest_option_dist_m | 12.63 / 11.49 | 13.84 / 12.17 | 13.84 / 12.25 |
| support.open_options_mean | 3.20 / 2.59 | 3.14 / 2.06 | 3.07 / 3.10 |
| support.open_options_p10 | 0 / 0 | 0 / 0 | 0 / 0 |
| support.pass_target_open_pct | 76.22 / 48.61 | 75.44 / 48.91 | 69.60 / 52.53 |
| support.passes_completed_tracked | 164 / 72 | 114 / 92 | 125 / 99 |
| support.runs_in_behind_per_10min | 3 / 0 | 0 / 0 | 1 / 0 |
| support.ticks_with_le1_option_pct | 26.98 / 47.40 | 28.61 / 54.87 | 28.62 / 34.87 |
| support.width_m | 45.86 / 43.64 | 44.61 / 45.74 | 45.36 / 45.41 |
| trans.count_per_10min | 19 / 28 | 21 / 42 | 25 / 37 |
| trans.counterpress_pct | 68.42 / 39.29 | 66.67 / 26.19 | 80 / 43.24 |
| trans.forward_recovery_speed_mps_0_5s | 6.69 / 6.05 | 6.38 / 5.94 | 6.47 / 6.22 |
| trans.forward_sprint_share_0_5s | 70.60 / 57.04 | 63.92 / 59.13 | 65.28 / 61.07 |
| trans.forwards_staying_high_mean | 1.83 / 0.85 | 2.40 / 1.18 | 2 / 1.73 |
| trans.in_shape_at_turnover_pct | 72.22 / 72 | 68.75 / 60.61 | 60 / 56 |
| trans.mid_recovery_speed_mps_0_5s | 5.86 / 5.65 | 5.68 / 5.46 | 5.84 / 5.60 |
| trans.mid_sprint_share_0_5s | 53.51 / 51.15 | 51.61 / 47.86 | 55.02 / 48.93 |
| trans.pct_outfield_behind_ball_at_3s | 73.33 / 83.33 | 73.64 / 62 | 72.67 / 67.78 |
| trans.pct_outfield_behind_ball_at_6s | 76.67 / 90.00 | 60 / 73.64 | 70 / 73.33 |
| trans.time_to_shape_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_excluded_pct | 5.26 / 10.71 | 23.81 / 21.43 | 20 / 32.43 |
| trans.time_to_shape_s | 0.87 / 0.54 | 0.88 / 1.09 | 1 / 1.00 |
| trans.time_to_shape_static_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_static_excluded_pct | 15.79 / 17.86 | 38.10 / 38.10 | 32 / 43.24 |
| trans.time_to_shape_static_s | 0.71 / 0.27 | 0.19 / 0.41 | 0.87 / 0.87 |

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
- **general.autopilot**: Round 4. --home-controller / --away-controller autopilot sets that side to SideController::Human and, before every tick, feeds it psfc_sim::ai::suggest_cpu_input(sim.state(), side, difficulty) (the side-effect-free CPU input, computed from the state after the previous tick, i.e. one tick of input latency; the other side's frame is idle and ignored). This drives the human-side paths (auto-switch, controlled player, teammate contain) that CPU-vs-CPU never runs. Config ids/labels name such a side 'autopilot-<difficulty>'; CPU-only ids are unchanged. Final hashes are deterministic: the unit test autopilot_match_is_deterministic runs the same autopilot config twice and asserts identical state_hash.
- **general.means**: mean_home / mean_away are means over seeds; mean_teams is the mean of mean_home and mean_away (for *_max keys this is a mean of per-match maxima).
- **general.phase**: Per side and open-play tick: 'attacking' if ball.owner is that side, else (ball loose) if ball.last_touch_team is that side; 'defending' if the owner / last toucher is the opponent; ticks with no owner and no last touch are excluded from phase metrics.
- **general.sampling**: The sim is ticked at 60 Hz with both controllers set to SideController::Cpu(difficulty) and idle external input; MatchState is sampled after every tick. 'Open play' = MatchPhase::OpenPlay. Per-player deltas only use pairs of consecutive open-play ticks with the same footballer in the slot; sent-off players are skipped. Coordinates are metres, x along the pitch, own goal line at x = -attack_sign(second_half_swapped) * pitch_length/2.
- **general.support**: Round 4. Support metrics are sampled on open-play ticks where an outfielder (slot != 0) of the side is ball.owner (carrier ticks). Open lane to a teammate: no on-pitch opponent (GK included) within 1.5 m of the segment carrier -> teammate. Offside: mirrors the sim's is_player_offside_at_pass: offsides_enabled, teammate (not the GK) past the offside-zone line (halfway in 11v11, +half_length/3 in 5v5 Rush), more than 0.15 m ahead of the ball and more than 0.2 m ahead of the second-last on-pitch opponent (GK included), all along the attacking axis (this is the task's 'ahead of the second-last opponent' rule with the sim's own zone, ball and tolerance conditions, so it matches what the referee would flag). Open option: an outfield teammate 6-30 m from the carrier with an open lane and not offside.
- **general.turnover**: Round 4. A turnover against side L happens on the open-play tick t0 on which the opponent O becomes ball.owner while the previous open-play owner was a player of L whose last owned tick is at most 120 ticks (2 s) earlier, with no stoppage in between (any non-open-play tick clears the last owner). So the ball goes from L to O directly (tackle, interception of a pass, keeper claim of a shot or cross) or after a loose spell of at most 2 s; a loose ball contested for longer, and restarts (throw-ins, goal kicks, free kicks), are not turnovers. A transition episode for L starts at t0 and ends at the first of: a stoppage, L owning the ball again, or t0 + 15 s. All trans.* metrics are in the losing side's map.
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
- **move.class_time_pct.<c>**: % of consecutive open-play player-tick pairs whose current tick has move_priority c (the exposure behind the per-class rates).
- **move.dspeed_max_mps_per_tick**: Max ||vel_t| - |vel_t-1|| over the match (m/s per tick).
- **move.heading_change_p95_deg_per_tick_gt3mps**: 95th percentile (nearest rank) of the per-tick heading change |angle(vel_t-1, vel_t)| in degrees, over player-tick pairs with speed > 3 m/s on both ticks.
- **move.heading_change_pct.<bucket>**: Per player per consecutive open-play tick pair with speed > 1 m/s on both ticks: |angle between vel_t-1 and vel_t| bucketed 0-10, 10-45, 45-90, >90 degrees; % of samples. All on-pitch players incl. GK.
- **move.heading_sign_flips_per_player_min**: Over the heading-histogram samples (speed > 1 m/s on both ticks): signed heading change theta_t = atan2(cross(vel_t-1, vel_t), dot(vel_t-1, vel_t)) in degrees. A heading sign flip is a sample where |theta_t| > 1 deg and the previous consecutive sample's |theta_t-1| > 1 deg with opposite signs (left-right-left steering wobble, independent of the 0.02 m/s lateral dead band of side flips). Count / player-minutes.
- **move.lateral_accel_mean_mps2**: Mean |cross(vel_t-1, vel_t)| / |vel_t-1| * 60 over the same samples: velocity change perpendicular to the previous heading (m/s^2).
- **move.reversals_per_player_min**: Count of those samples with dot(vel_t, vel_t-1) < 0, divided by player-minutes (all consecutive open-play player-ticks / 3600).
- **move.side_flips_by_class.<c>_per_player_min / _share_pct**: Round 4. Side flips (move.side_flips_per_player_min) split by psfc_sim::ai::move_priority(state, side, idx) on the tick of the flip (0 carrier / elected chaser / committed keeper, 1 engager or human-controlled, 2 tactical mover, 3 idle / cruising / yielding; outside a fresh plan everyone but the carrier reads 2). per_player_min = flips in class c / player-minutes spent in class c (all consecutive open-play player-tick pairs whose current tick is in class c); share_pct = % of all the side's flips that occurred in class c.
- **move.side_flips_per_player_min**: Per player, over the heading-histogram samples (speed > 1 m/s on both ticks): lateral component l_t = cross(vel_t-1, vel_t) / |vel_t-1| (m/s, sign = turning left/right of the previous heading). A side flip is a sample whose l_t and the previous tick's l_t-1 are both above 0.02 m/s in magnitude with opposite signs (avoidance oscillation). Count / player-minutes.
- **move.side_flips_speed_lt3_per_player_min / _ge3_per_player_min**: Side flips split by the current tick's speed |vel_t| < 3 m/s or >= 3 m/s, each divided by the player-minutes spent in that speed band.
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
- **support.carrier_pct_of_open_play**: Carrier ticks as % of the side's open-play ticks (sample size of the support metrics).
- **support.cluster_pct**: % of carrier ticks where two outfield teammates other than the carrier are within 4 m of each other and both within 15 m of the carrier.
- **support.forward_options_mean**: Mean number of open options at least 3 m ahead of the carrier along the attacking axis ((x_mate - x_carrier) * attack_sign >= 3).
- **support.nearest_option_dist_m**: Mean distance carrier -> nearest open option, over carrier ticks with at least one open option.
- **support.open_options_mean / open_options_p10 / ticks_with_le1_option_pct**: Number of open options per carrier tick: mean, 10th percentile (nearest rank) across carrier ticks, and % of carrier ticks with 0 or 1 open option.
- **support.pass_target_open_pct / passes_completed_tracked**: From the event stream plus owner transitions (the sim's Pass event carries no receiver): a MatchEventType::Pass of the side within 2 ticks after its last carrier tick starts a pass by that carrier, remembering which teammates (GK included, no distance band) had an open, non-offside lane on that last carrier tick. The pass is completed when the next open-play owner is a teammate other than the passer; any other next owner, or a stoppage, ends it. pass_target_open_pct = % of completed passes whose receiver had an open lane at the moment of the pass; passes_completed_tracked = their count per match (open-play passes only; Pass events also cover clearances, which count when a teammate collects them; set-piece passes are not tracked).
- **support.runs_in_behind_per_10min**: Per outfielder other than the owner, on open-play ticks while his side owns the ball (any owner incl. GK), against the line L = second-last opponent's x along the attacking axis on that tick: a run counts when the player was at or behind L on an earlier such tick (armed) and is now ahead of L with |vel| > 5 m/s. After crossing (counted or not) he must drop more than 1 m behind L to be armed again; losing possession or a stoppage disarms everyone. Count per 10 open-play minutes.
- **support.width_m / support.depth_m**: y-extent / x-extent of the side's on-pitch outfielders (carrier included) on carrier ticks, mean.
- **trans.count_per_10min**: Turnovers suffered by the side (general.turnover) per 10 open-play minutes.
- **trans.counterpress_pct**: % of turnovers after which the side owns the ball again within 6 s (t0 + 360 ticks) with no stoppage in between.
- **trans.forward_recovery_speed_mps_0_5s / forward_sprint_share_0_5s**: Over episode ticks t0 .. t0 + 5 s (exclusive) and the side's on-pitch players with PlayerRole::Forward: mean |vel| (m/s), and % of those player-ticks with |vel| > 6 m/s.
- **trans.forwards_staying_high_mean**: Mean number of the side's on-pitch Forwards ahead of the ball ((x - ball.x) * attack_sign > 0) at t0 + 6 s, over episodes still running then (rest offence kept high).
- **trans.in_shape_at_turnover_pct**: % of scored time_to_shape episodes (defensive_shape_anchor variant) already in shape on the turnover tick t0 (time 0 s). A high value means the criterion is met by position at the moment of the loss (players already goal-side of the ball), not by a recovery run.
- **trans.mid_recovery_speed_mps_0_5s / mid_sprint_share_0_5s**: Same for PlayerRole::Midfielder.
- **trans.pct_outfield_behind_ball_at_3s / _at_6s**: % of the side's on-pitch outfielders strictly goal-side of the ball at exactly t0 + 3 s (180 ticks) and t0 + 6 s (360 ticks), over episodes still running at that tick (no stoppage, not regained).
- **trans.time_to_shape_s / time_to_shape_capped_pct / time_to_shape_excluded_pct**: Seconds from t0 to the first episode tick on which at least 80% of the side's on-pitch outfielders are either within 5 m of psfc_sim::ai::defensive_shape_anchor(state, side, idx) (the defensive-block slot the AI is assigned, ball-relative via the team memory's ref_ball; falls back to the static slot for unassigned players) or strictly goal-side of the ball ((x - ball.x) * attack_sign < 0). Episodes reaching 15 s without it score 15 s and count as capped (capped_pct = % of scored episodes). Episodes that end earlier by a stoppage or by the side regaining the ball, before reaching shape, are not scored (excluded_pct = % of all turnovers). The tick t0 itself is checked (0 s = already in shape).
- **trans.time_to_shape_static_s /_static_capped_pct / _static_excluded_pct**: Same with the static formation slot world_anchor_for_player(state, side, idx, 0) instead of defensive_shape_anchor: a reference that does not depend on the AI's own block logic, for before/after comparisons across AI changes.

