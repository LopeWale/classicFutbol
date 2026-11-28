# Match analytics

## 5v5 hard vs hard (10 min x 3 seeds from 1)

Command: `psfc-replay analyze --format 5v5 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty hard --away-difficulty hard`

Gameplay version: `0.1.0-5v5`

| seed | final state_hash | score (H-A) |
|---|---|---|
| 1 | `ba3c7971f9a815b6` | 7-5 |
| 2 | `ac5073b006b9e549` | 7-7 |
| 3 | `037b4b0f1cd65145` | 4-4 |

Match meta (mean over seeds): goals_total = 11.33, open_play_minutes = 10, open_play_pct_of_ticks = 93.66, shots_total = 55, ticks_simulated = 38436

### Match

| metric | home | away | teams mean |
|---|---:|---:|---:|
| fouls | 0 | 0 | **0** |
| goals | 6 | 5.33 | **5.67** |
| offsides | 1.67 | 1.67 | **1.67** |
| passes_completed_pct | 88.76 | 89.21 | **88.98** |
| possession_pct | 49.38 | 50.62 | **50** |
| shots | 25.33 | 29.67 | **27.50** |
| shots_on_target | 21 | 27 | **24** |

### Goalkeeper

| metric | home | away | teams mean |
|---|---:|---:|---:|
| catches | 2.33 | 1.33 | **1.83** |
| claims_non_save | 1.67 | 0.67 | **1.17** |
| excursion_mean_s | 0.77 | 0.14 | **0.46** |
| excursions_gt1s_per_10min | 1 | 0 | **0.50** |
| gathers | 4 | 2 | **3** |
| goals_conceded | 5.33 | 6 | **5.67** |
| goals_conceded_gk_gt3m_off_line | 3.33 | 3.67 | **3.50** |
| goals_conceded_no_shot_found | 0 | 0 | **0** |
| line_dist_at_conceded_shot_mean_m | 2.70 | 2.63 | **2.67** |
| line_dist_max_m | 8.81 | 5.44 | **7.13** |
| line_dist_mean_m | 2.97 | 3.03 | **3.00** |
| parries | 20.67 | 14 | **17.33** |
| pct_outside_6yd | 0.32 | 0.02 | **0.17** |
| pct_outside_box | 0.01 | 0 | **0.00** |
| rush_count | 1.67 | 0.67 | **1.17** |
| rush_per_10min | 1.67 | 0.67 | **1.17** |
| saves | 23 | 15.33 | **19.17** |
| tips_over | 0 | 0 | **0** |

### Defence and shape

| metric | home | away | teams mean |
|---|---:|---:|---:|
| attacking.anchor_dist_m.defender | 7.74 | 8.51 | **8.12** |
| attacking.anchor_dist_m.forward | 16.33 | 14.90 | **15.62** |
| attacking.anchor_dist_m.goalkeeper | 1.29 | 1.21 | **1.25** |
| attacking.anchor_dist_m.midfielder | 15.90 | 16.36 | **16.13** |
| attacking.anchor_dist_shape_m.defender | 7.25 | 7.75 | **7.50** |
| attacking.anchor_dist_shape_m.forward | 10.75 | 10.48 | **10.61** |
| attacking.anchor_dist_shape_m.midfielder | 10.26 | 11.06 | **10.66** |
| attacking.bbox_area_m2 | 310.6 | 325.3 | **317.9** |
| attacking.bbox_depth_m | 17.85 | 18.70 | **18.27** |
| attacking.bbox_width_m | 16.42 | 16.24 | **16.33** |
| attacking.length_m | 17.85 | 18.70 | **18.27** |
| attacking.nearest_mate_mean_m | 9.96 | 10.16 | **10.06** |
| attacking.nearest_mate_p10_m | 3.69 | 3.50 | **3.59** |
| attacking.pct_of_open_play | 51.15 | 48.85 | **50** |
| attacking.stretch_m | 9.00 | 9.21 | **9.10** |
| attacking.width_m | 16.42 | 16.24 | **16.33** |
| defending.anchor_dist_m.defender | 7.46 | 7.74 | **7.60** |
| defending.anchor_dist_m.forward | 16.26 | 14.99 | **15.63** |
| defending.anchor_dist_m.goalkeeper | 1.41 | 1.34 | **1.37** |
| defending.anchor_dist_m.midfielder | 14.32 | 14.57 | **14.44** |
| defending.anchor_dist_shape_m.defender | 8.77 | 8.81 | **8.79** |
| defending.anchor_dist_shape_m.forward | 10.20 | 9.18 | **9.69** |
| defending.anchor_dist_shape_m.midfielder | 9.61 | 9.72 | **9.67** |
| defending.bbox_area_m2 | 310.9 | 353.7 | **332.3** |
| defending.bbox_depth_m | 17.35 | 18.77 | **18.06** |
| defending.bbox_width_m | 17.08 | 17.59 | **17.34** |
| defending.length_m | 17.35 | 18.77 | **18.06** |
| defending.line_x_std_m | 5.34 | 5.84 | **5.59** |
| defending.line_x_variance_m2 | 46.63 | 55.34 | **50.98** |
| defending.nearest_mate_mean_m | 10.14 | 10.78 | **10.46** |
| defending.nearest_mate_p10_m | 4.14 | 4.32 | **4.23** |
| defending.pct_of_open_play | 48.85 | 51.15 | **50** |
| defending.pct_outfield_behind_ball | 65.50 | 69.98 | **67.74** |
| defending.stretch_m | 8.97 | 9.51 | **9.24** |
| defending.swarm2_pct | 0.27 | 0.77 | **0.52** |
| defending.swarm3_pct | 0 | 0 | **0** |
| defending.width_m | 17.08 | 17.59 | **17.34** |

### Defensive transition

| metric | home | away | teams mean |
|---|---:|---:|---:|
| count_per_10min | 20.33 | 19.67 | **20** |
| counterpress_pct | 31.37 | 35.70 | **33.54** |
| forward_recovery_speed_mps_0_5s | 6.41 | 6.62 | **6.51** |
| forward_sprint_share_0_5s | 60.27 | 63.46 | **61.86** |
| forward_sprint_share_0_5s_7mps | 46.05 | 48.19 | **47.12** |
| forwards_staying_high_mean | 0.92 | 0.74 | **0.83** |
| in_shape_at_turnover_pct | 64.38 | 63.13 | **63.75** |
| mid_recovery_speed_mps_0_5s | 6.32 | 6.43 | **6.37** |
| mid_sprint_share_0_5s | 61.11 | 62.23 | **61.67** |
| mid_sprint_share_0_5s_7mps | 49.32 | 51.65 | **50.49** |
| outfield_stamina_end_permille | 833.3 | 919.1 | **876.2** |
| pct_outfield_behind_ball_at_3s | 66.55 | 70.75 | **68.65** |
| pct_outfield_behind_ball_at_6s | 54.73 | 74.70 | **64.71** |
| time_to_shape_capped_pct | 0 | 0 | **0** |
| time_to_shape_excluded_pct | 12.66 | 13.60 | **13.13** |
| time_to_shape_s | 0.88 | 1.01 | **0.94** |
| time_to_shape_static_capped_pct | 0 | 0 | **0** |
| time_to_shape_static_excluded_pct | 32.65 | 21.75 | **27.20** |
| time_to_shape_static_s | 0.53 | 0.78 | **0.65** |

### Attacking support

| metric | home | away | teams mean |
|---|---:|---:|---:|
| carrier_pct_of_open_play | 19.20 | 19.83 | **19.51** |
| cluster_pct | 6.37 | 7.56 | **6.96** |
| depth_m | 18.99 | 19.30 | **19.15** |
| forward_options_mean | 0.24 | 0.20 | **0.22** |
| nearest_option_dist_m | 12.19 | 11.71 | **11.95** |
| open_options_mean | 1.41 | 1.42 | **1.41** |
| open_options_p10 | 0 | 0 | **0** |
| pass_target_open_pct | 75.36 | 76.19 | **75.78** |
| passes_completed_tracked | 142.7 | 127.3 | **135** |
| release_pass_completed_pct | 85.38 | 67.59 | **76.49** |
| releases_per_10min | 13.33 | 11 | **12.17** |
| run_abandon_per_10min | 0 | 0 | **0** |
| runs_in_behind_per_10min | 2.33 | 1.33 | **1.83** |
| ticks_with_le1_option_pct | 52.32 | 49.86 | **51.09** |
| width_m | 15.73 | 14.84 | **15.29** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 11.23 | 11.29 | **11.26** |
| accel_gt10_pct | 52.18 | 52.63 | **52.40** |
| accel_mean_mps2 | 11.20 | 11.27 | **11.24** |
| class_time_pct.0 | 12.44 | 12.07 | **12.26** |
| class_time_pct.1 | 7.09 | 7.30 | **7.19** |
| class_time_pct.2 | 64.82 | 64.59 | **64.70** |
| class_time_pct.3 | 15.65 | 16.04 | **15.85** |
| dspeed_max_mps_per_tick | 3.52 | 4.06 | **3.79** |
| heading_change_p95_deg_per_tick_gt3mps | 3.45 | 3.45 | **3.45** |
| heading_change_pct.0_10 | 99.42 | 99.43 | **99.43** |
| heading_change_pct.10_45 | 0.56 | 0.55 | **0.55** |
| heading_change_pct.45_90 | 0.02 | 0.02 | **0.02** |
| heading_change_pct.gt90 | 0.00 | 0.00 | **0.00** |
| heading_sign_flips_per_player_min | 17.07 | 16.07 | **16.57** |
| lateral_accel_mean_mps2 | 8.26 | 8.46 | **8.36** |
| reversals_per_player_min | 0.01 | 0.03 | **0.02** |
| side_flips_by_class.0_per_player_min | 116.2 | 106.2 | **111.2** |
| side_flips_by_class.0_share_pct | 39.10 | 36.34 | **37.72** |
| side_flips_by_class.1_per_player_min | 45.19 | 44.44 | **44.82** |
| side_flips_by_class.1_share_pct | 8.67 | 9.24 | **8.95** |
| side_flips_by_class.2_per_player_min | 24.27 | 23.87 | **24.07** |
| side_flips_by_class.2_share_pct | 42.62 | 43.86 | **43.24** |
| side_flips_by_class.3_per_player_min | 22.68 | 23.16 | **22.92** |
| side_flips_by_class.3_share_pct | 9.62 | 10.56 | **10.09** |
| side_flips_cause.desired_per_player_min | 15.85 | 13.84 | **14.85** |
| side_flips_cause.filter_per_player_min | 17.51 | 17.91 | **17.71** |
| side_flips_cause.limiter_per_player_min | 1.62 | 1.67 | **1.64** |
| side_flips_cause.sampler_per_player_min | 1.97 | 1.76 | **1.86** |
| side_flips_per_player_min | 36.95 | 35.18 | **36.06** |
| side_flips_schmitt_per_player_min | 40.55 | 40.23 | **40.39** |
| side_flips_speed_ge3_per_player_min | 47.54 | 45.43 | **46.49** |
| side_flips_speed_lt3_per_player_min | 12.40 | 12.47 | **12.44** |
| speed_oscillation_pct | 4.47 | 4.43 | **4.45** |
| turn_rate_mean_deg_per_s | 102.5 | 103.4 | **102.9** |

### Collisions and push-out

| metric | home | away | teams mean |
|---|---:|---:|---:|
| any.depth_max_m | 0.79 | 0.79 | **0.79** |
| any.depth_mean_m | 0.25 | 0.25 | **0.25** |
| any.events_per_min | 1.73 | 1.73 | **1.73** |
| any.stuck_mean_ticks | 3.67 | 3.67 | **3.67** |
| any.stuck_pairs | 0.33 | 0.33 | **0.33** |
| any.ticks_gt1cm_per_min | 3.83 | 3.83 | **3.83** |
| any.ticks_per_min | 4.57 | 4.57 | **4.57** |
| correction_vel_corrected_p50_mps | 0.84 | 0.82 | **0.83** |
| correction_vel_corrected_p99_mps | 11.49 | 11.62 | **11.55** |
| correction_vel_corrected_pct | 0.06 | 0.06 | **0.06** |
| correction_vel_max_mps | 11.98 | 12.13 | **12.06** |
| correction_vel_p50_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p95_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p99_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_pct_gt_0_3 | 0.05 | 0.05 | **0.05** |
| opponent.depth_max_m | 0.09 | 0.09 | **0.09** |
| opponent.depth_mean_m | 0.03 | 0.03 | **0.03** |
| opponent.events_per_min | 1.13 | 1.13 | **1.13** |
| opponent.stuck_mean_ticks | 0 | 0 | **0** |
| opponent.stuck_pairs | 0 | 0 | **0** |
| opponent.ticks_gt1cm_per_min | 1.37 | 1.37 | **1.37** |
| opponent.ticks_per_min | 2 | 2 | **2** |
| teammate.depth_max_m | 0.77 | 0.78 | **0.77** |
| teammate.depth_mean_m | 0.38 | 0.37 | **0.37** |
| teammate.events_per_min | 0.30 | 0.30 | **0.30** |
| teammate.stuck_mean_ticks | 0 | 3.67 | **1.83** |
| teammate.stuck_pairs | 0 | 0.33 | **0.17** |
| teammate.ticks_gt1cm_per_min | 2.13 | 1.70 | **1.92** |
| teammate.ticks_per_min | 2.17 | 1.80 | **1.98** |

### Coordination

| metric | home | away | teams mean |
|---|---:|---:|---:|
| cruise_over72_pct | 36.16 | 34.36 | **35.26** |
| dual_chaser_pct_of_loose | 19.06 | 16.32 | **17.69** |
| dual_chaser_ticks_per_min | 415.1 | 355.7 | **385.4** |
| engagers3_pct_of_carrier_ticks | 0.01 | 0 | **0.00** |
| engagers3_ticks_per_min | 0.03 | 0 | **0.02** |
| mark_steal_per_defender_min | 0.34 | 0.29 | **0.31** |
| max_simultaneous_chasers | 4.33 | 4 | **4.17** |

### Aerial

| metric | home | away | teams mean |
|---|---:|---:|---:|
| headers | 10 | 11.67 | **10.83** |
| headers_clear | 1.67 | 1.33 | **1.50** |
| headers_pass | 5.67 | 8 | **6.83** |
| headers_shot | 2.67 | 2.33 | **2.50** |
| jumps_gk | 27.33 | 22 | **24.67** |
| jumps_outfield | 21.67 | 25 | **23.33** |
| max_airborne_outfield | 1 | 1 | **1** |

### Team shape vs envelope (teams mean)

| metric | in possession | out of possession | 11v11 envelope (m) |
|---|---:|---:|---|
| length_m | 18.27 | 18.06 | 31-46 |
| width_m | 16.33 | 17.34 | 35-48 |
| stretch_m | 9.10 | 9.24 | 7-16 |
| nearest_mate_mean_m | 10.06 | 10.46 | none published (in-possession mean near 9-10 m, estimate) |
| nearest_mate_p10_m | 3.59 | 4.23 | none published (in-possession mean near 9-10 m, estimate) |

Note: 5v5 has no published team-shape envelope; the 11v11 ranges are printed for reference only and do not apply to a 4-outfielder team on a small pitch.

### Per seed (home / away)

| metric | seed 1 | seed 2 | seed 3 |
|---|---:|---:|---:|
| aerial.headers | 12 / 13 | 10 / 10 | 8 / 12 |
| aerial.headers_clear | 2 / 1 | 1 / 2 | 2 / 1 |
| aerial.headers_pass | 6 / 11 | 7 / 6 | 4 / 7 |
| aerial.headers_shot | 4 / 1 | 2 / 2 | 2 / 4 |
| aerial.jumps_gk | 19 / 26 | 29 / 21 | 34 / 19 |
| aerial.jumps_outfield | 24 / 23 | 21 / 23 | 20 / 29 |
| aerial.max_airborne_outfield | 1 / 1 | 1 / 1 | 1 / 1 |
| collision.any.depth_max_m | 0.80 / 0.80 | 0.79 / 0.79 | 0.77 / 0.77 |
| collision.any.depth_mean_m | 0.25 / 0.25 | 0.33 / 0.33 | 0.19 / 0.19 |
| collision.any.events_per_min | 2.10 / 2.10 | 1.70 / 1.70 | 1.40 / 1.40 |
| collision.any.stuck_mean_ticks | 11 / 11 | 0 / 0 | 0 / 0 |
| collision.any.stuck_pairs | 1 / 1 | 0 / 0 | 0 / 0 |
| collision.any.ticks_gt1cm_per_min | 4.70 / 4.70 | 3.80 / 3.80 | 3 / 3 |
| collision.any.ticks_per_min | 5.90 / 5.90 | 4.30 / 4.30 | 3.50 / 3.50 |
| collision.correction_vel_corrected_p50_mps | 1.07 / 0.95 | 0.75 / 1.03 | 0.70 / 0.49 |
| collision.correction_vel_corrected_p99_mps | 12.45 / 11.43 | 10.68 / 12.60 | 11.33 / 10.83 |
| collision.correction_vel_corrected_pct | 0.07 / 0.08 | 0.07 / 0.06 | 0.05 / 0.05 |
| collision.correction_vel_max_mps | 12.69 / 12.73 | 11.92 / 12.84 | 11.33 / 10.83 |
| collision.correction_vel_p50_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p95_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p99_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_pct_gt_0_3 | 0.05 / 0.06 | 0.05 / 0.04 | 0.04 / 0.03 |
| collision.opponent.depth_max_m | 0.14 / 0.14 | 0.05 / 0.05 | 0.07 / 0.07 |
| collision.opponent.depth_mean_m | 0.04 / 0.04 | 0.02 / 0.02 | 0.03 / 0.03 |
| collision.opponent.events_per_min | 1.40 / 1.40 | 1 / 1 | 1 / 1 |
| collision.opponent.stuck_mean_ticks | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.stuck_pairs | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.ticks_gt1cm_per_min | 1.60 / 1.60 | 0.80 / 0.80 | 1.70 / 1.70 |
| collision.opponent.ticks_per_min | 2.70 / 2.70 | 1.20 / 1.20 | 2.10 / 2.10 |
| collision.teammate.depth_max_m | 0.80 / 0.77 | 0.77 / 0.79 | 0.75 / 0.77 |
| collision.teammate.depth_mean_m | 0.38 / 0.35 | 0.41 / 0.40 | 0.35 / 0.34 |
| collision.teammate.events_per_min | 0.30 / 0.40 | 0.40 / 0.30 | 0.20 / 0.20 |
| collision.teammate.stuck_mean_ticks | 0 / 11 | 0 / 0 | 0 / 0 |
| collision.teammate.stuck_pairs | 0 / 1 | 0 / 0 | 0 / 0 |
| collision.teammate.ticks_gt1cm_per_min | 2.10 / 2.70 | 3 / 1.80 | 1.30 / 0.60 |
| collision.teammate.ticks_per_min | 2.10 / 2.90 | 3.10 / 1.80 | 1.30 / 0.70 |
| coord.cruise_over72_pct | 36.40 / 34.14 | 35.81 / 33.50 | 36.27 / 35.43 |
| coord.dual_chaser_pct_of_loose | 19.13 / 17.14 | 18.95 / 16.59 | 19.09 / 15.24 |
| coord.dual_chaser_ticks_per_min | 425.2 / 380.9 | 411.8 / 360.5 | 408.2 / 325.8 |
| coord.engagers3_pct_of_carrier_ticks | 0.02 / 0 | 0 / 0 | 0 / 0 |
| coord.engagers3_ticks_per_min | 0.10 / 0 | 0 / 0 | 0 / 0 |
| coord.mark_steal_per_defender_min | 0.43 / 0.48 | 0.12 / 0.13 | 0.46 / 0.26 |
| coord.max_simultaneous_chasers | 4 / 4 | 4 / 4 | 5 / 4 |
| gk.catches | 2 / 2 | 1 / 1 | 4 / 1 |
| gk.claims_non_save | 3 / 0 | 2 / 1 | 0 / 1 |
| gk.excursion_mean_s | 1.16 / 0.03 | 1.16 / 0.38 | 0 / 0 |
| gk.excursions_gt1s_per_10min | 2 / 0 | 1 / 0 | 0 / 0 |
| gk.gathers | 5 / 2 | 3 / 2 | 4 / 2 |
| gk.goals_conceded | 5 / 7 | 7 / 7 | 4 / 4 |
| gk.goals_conceded_gk_gt3m_off_line | 5 / 4 | 3 / 5 | 2 / 2 |
| gk.goals_conceded_no_shot_found | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.line_dist_at_conceded_shot_mean_m | 3.22 / 2.68 | 2.23 / 2.65 | 2.66 / 2.56 |
| gk.line_dist_max_m | 11.56 / 5.50 | 9.63 / 5.69 | 5.25 / 5.13 |
| gk.line_dist_mean_m | 3.03 / 3.02 | 2.97 / 3.07 | 2.91 / 3.00 |
| gk.parries | 13 / 18 | 25 / 12 | 24 / 12 |
| gk.pct_outside_6yd | 0.58 / 0.01 | 0.39 / 0.06 | 0 / 0 |
| gk.pct_outside_box | 0.02 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_count | 3 / 1 | 2 / 1 | 0 / 0 |
| gk.rush_per_10min | 3 / 1 | 2 / 1 | 0 / 0 |
| gk.saves | 15 / 20 | 26 / 13 | 28 / 13 |
| gk.tips_over | 0 / 0 | 0 / 0 | 0 / 0 |
| match.fouls | 0 / 0 | 0 / 0 | 0 / 0 |
| match.goals | 7 / 5 | 7 / 7 | 4 / 4 |
| match.offsides | 1 / 2 | 2 / 0 | 2 / 3 |
| match.passes_completed_pct | 90.10 / 89.53 | 89.39 / 91.91 | 86.78 / 86.18 |
| match.possession_pct | 53.90 / 46.10 | 47.59 / 52.41 | 46.65 / 53.35 |
| match.shots | 32 / 19 | 23 / 33 | 21 / 37 |
| match.shots_on_target | 27 / 18 | 19 / 32 | 17 / 31 |
| move.accel_from_pos_mean_mps2 | 11.25 / 11.55 | 11.43 / 11.36 | 11.01 / 10.96 |
| move.accel_gt10_pct | 52.34 / 53.88 | 53.59 / 53.22 | 50.61 / 50.79 |
| move.accel_mean_mps2 | 11.20 / 11.52 | 11.41 / 11.34 | 10.99 / 10.96 |
| move.class_time_pct.0 | 13.09 / 11.93 | 12.26 / 11.90 | 11.98 / 12.38 |
| move.class_time_pct.1 | 6.26 / 7.57 | 7.20 / 7.34 | 7.80 / 6.99 |
| move.class_time_pct.2 | 65.57 / 65.23 | 64.19 / 64.08 | 64.68 / 64.46 |
| move.class_time_pct.3 | 15.08 / 15.26 | 16.34 / 16.69 | 15.54 / 16.17 |
| move.dspeed_max_mps_per_tick | 3.47 / 4.57 | 3.61 / 3.82 | 3.49 / 3.80 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.43 / 3.50 | 3.49 / 3.42 | 3.43 / 3.44 |
| move.heading_change_pct.0_10 | 99.48 / 99.34 | 99.41 / 99.50 | 99.38 / 99.44 |
| move.heading_change_pct.10_45 | 0.50 / 0.64 | 0.56 / 0.48 | 0.60 / 0.54 |
| move.heading_change_pct.45_90 | 0.02 / 0.03 | 0.02 / 0.02 | 0.02 / 0.02 |
| move.heading_change_pct.gt90 | 0 / 0.00 | 0.00 / 0.00 | 0 / 0.00 |
| move.heading_sign_flips_per_player_min | 17.72 / 17.04 | 16.98 / 16.62 | 16.50 / 14.56 |
| move.lateral_accel_mean_mps2 | 8.28 / 8.67 | 8.53 / 8.51 | 7.97 / 8.21 |
| move.reversals_per_player_min | 0 / 0.02 | 0.04 / 0.04 | 0 / 0.02 |
| move.side_flips_by_class.0_per_player_min | 120.4 / 107.7 | 122.9 / 118.3 | 105.1 / 92.68 |
| move.side_flips_by_class.0_share_pct | 41.57 / 35.37 | 40.16 / 38.50 | 35.58 / 35.15 |
| move.side_flips_by_class.1_per_player_min | 50.20 / 45.21 | 38.92 / 42.58 | 46.45 / 45.54 |
| move.side_flips_by_class.1_share_pct | 8.29 / 9.42 | 7.47 / 8.54 | 10.24 / 9.75 |
| move.side_flips_by_class.2_per_player_min | 23.79 / 25.05 | 24.73 / 24.18 | 24.30 / 22.40 |
| move.side_flips_by_class.2_share_pct | 41.15 / 44.96 | 42.29 / 42.39 | 44.40 / 44.23 |
| move.side_flips_by_class.3_per_player_min | 22.58 / 24.40 | 23.16 / 23.16 | 22.30 / 21.91 |
| move.side_flips_by_class.3_share_pct | 8.98 / 10.25 | 10.08 / 10.57 | 9.79 / 10.86 |
| move.side_flips_cause.desired_per_player_min | 16.60 / 13.88 | 16.56 / 15.40 | 14.40 / 12.25 |
| move.side_flips_cause.filter_per_player_min | 17.84 / 18.84 | 17.40 / 17.60 | 17.30 / 17.30 |
| move.side_flips_cause.limiter_per_player_min | 1.50 / 1.42 | 1.76 / 2.00 | 1.58 / 1.58 |
| move.side_flips_cause.sampler_per_player_min | 1.96 / 2.20 | 1.82 / 1.56 | 2.12 / 1.50 |
| move.side_flips_per_player_min | 37.90 / 36.34 | 37.54 / 36.56 | 35.40 / 32.64 |
| move.side_flips_schmitt_per_player_min | 41.30 / 40.92 | 38.88 / 40.00 | 41.46 / 39.76 |
| move.side_flips_speed_ge3_per_player_min | 48.65 / 46.38 | 48.25 / 47.27 | 45.73 / 42.64 |
| move.side_flips_speed_lt3_per_player_min | 13.06 / 13.87 | 12.45 / 12.68 | 11.68 / 10.87 |
| move.speed_oscillation_pct | 4.59 / 4.48 | 4.39 / 4.40 | 4.42 / 4.41 |
| move.turn_rate_mean_deg_per_s | 102.5 / 107.1 | 105.1 / 103.5 | 100.0 / 99.52 |
| shape.attacking.anchor_dist_m.defender | 7.67 / 8.48 | 7.24 / 8.16 | 8.31 / 8.88 |
| shape.attacking.anchor_dist_m.forward | 15.39 / 14.90 | 15.92 / 14.90 | 17.69 / 14.91 |
| shape.attacking.anchor_dist_m.goalkeeper | 1.22 / 1.21 | 1.26 / 1.18 | 1.37 / 1.25 |
| shape.attacking.anchor_dist_m.midfielder | 17.02 / 15.70 | 13.93 / 15.04 | 16.76 / 18.34 |
| shape.attacking.anchor_dist_shape_m.defender | 7.30 / 7.90 | 6.55 / 7.30 | 7.91 / 8.05 |
| shape.attacking.anchor_dist_shape_m.forward | 10.73 / 10.78 | 10.43 / 10.62 | 11.09 / 10.04 |
| shape.attacking.anchor_dist_shape_m.midfielder | 11.90 / 10.70 | 8.15 / 9.50 | 10.73 / 13.00 |
| shape.attacking.bbox_area_m2 | 315.2 / 317.5 | 319.0 / 331.8 | 297.6 / 326.6 |
| shape.attacking.bbox_depth_m | 18.50 / 17.81 | 17.98 / 19.15 | 17.06 / 19.15 |
| shape.attacking.bbox_width_m | 16.03 / 16.80 | 16.93 / 16.21 | 16.29 / 15.72 |
| shape.attacking.length_m | 18.50 / 17.81 | 17.98 / 19.15 | 17.06 / 19.15 |
| shape.attacking.nearest_mate_mean_m | 10.06 / 10.12 | 10.22 / 10.26 | 9.61 / 10.11 |
| shape.attacking.nearest_mate_p10_m | 3.56 / 3.52 | 4.01 / 3.32 | 3.50 / 3.66 |
| shape.attacking.pct_of_open_play | 50.92 / 49.08 | 51.50 / 48.50 | 51.01 / 48.99 |
| shape.attacking.stretch_m | 9.13 / 9.06 | 9.12 / 9.35 | 8.73 / 9.22 |
| shape.attacking.width_m | 16.03 / 16.80 | 16.93 / 16.21 | 16.29 / 15.72 |
| shape.defending.anchor_dist_m.defender | 7.02 / 7.87 | 7.31 / 7.15 | 8.06 / 8.19 |
| shape.defending.anchor_dist_m.forward | 15.62 / 15.28 | 16.30 / 14.51 | 16.86 / 15.18 |
| shape.defending.anchor_dist_m.goalkeeper | 1.40 / 1.38 | 1.42 / 1.26 | 1.39 / 1.37 |
| shape.defending.anchor_dist_m.midfielder | 14.82 / 14.02 | 12.98 / 12.89 | 15.17 / 16.79 |
| shape.defending.anchor_dist_shape_m.defender | 8.41 / 8.56 | 8.58 / 8.52 | 9.32 / 9.34 |
| shape.defending.anchor_dist_shape_m.forward | 10.31 / 9.73 | 10.19 / 8.88 | 10.09 / 8.94 |
| shape.defending.anchor_dist_shape_m.midfielder | 10.64 / 9.16 | 8.23 / 7.86 | 9.98 / 12.14 |
| shape.defending.bbox_area_m2 | 325.8 / 323.7 | 305.2 / 351.2 | 301.6 / 386.3 |
| shape.defending.bbox_depth_m | 18.06 / 17.43 | 17.08 / 18.59 | 16.91 / 20.27 |
| shape.defending.bbox_width_m | 17.27 / 17.51 | 17.12 / 17.50 | 16.85 / 17.77 |
| shape.defending.length_m | 18.06 / 17.43 | 17.08 / 18.59 | 16.91 / 20.27 |
| shape.defending.line_x_std_m | 5.75 / 5.51 | 5.26 / 5.63 | 5.00 / 6.38 |
| shape.defending.line_x_variance_m2 | 52.66 / 49.28 | 44.08 / 51.08 | 43.14 / 65.65 |
| shape.defending.nearest_mate_mean_m | 10.45 / 10.30 | 9.98 / 10.85 | 10.01 / 11.20 |
| shape.defending.nearest_mate_p10_m | 4.37 / 3.95 | 3.95 / 4.56 | 4.10 / 4.44 |
| shape.defending.pct_of_open_play | 49.08 / 50.92 | 48.50 / 51.50 | 48.99 / 51.01 |
| shape.defending.pct_outfield_behind_ball | 67.48 / 68.20 | 63.15 / 73.09 | 65.89 / 68.63 |
| shape.defending.stretch_m | 9.19 / 9.12 | 8.94 / 9.42 | 8.78 / 9.99 |
| shape.defending.swarm2_pct | 0.57 / 1.94 | 0.05 / 0.21 | 0.18 / 0.18 |
| shape.defending.swarm3_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| shape.defending.width_m | 17.27 / 17.51 | 17.12 / 17.50 | 16.85 / 17.77 |
| support.carrier_pct_of_open_play | 20.29 / 17.48 | 18.68 / 20.45 | 18.64 / 21.56 |
| support.cluster_pct | 8.38 / 8.15 | 6.05 / 7.85 | 4.68 / 6.66 |
| support.depth_m | 19.52 / 18.61 | 18.90 / 19.20 | 18.56 / 20.09 |
| support.forward_options_mean | 0.20 / 0.24 | 0.27 / 0.17 | 0.24 / 0.19 |
| support.nearest_option_dist_m | 12.11 / 11.77 | 12.14 / 11.68 | 12.31 / 11.68 |
| support.open_options_mean | 1.32 / 1.43 | 1.47 / 1.45 | 1.44 / 1.37 |
| support.open_options_p10 | 0 / 0 | 0 / 0 | 0 / 0 |
| support.pass_target_open_pct | 74.68 / 76.06 | 74.17 / 80.30 | 77.24 / 72.22 |
| support.passes_completed_tracked | 154 / 142 | 151 / 132 | 123 / 108 |
| support.release_pass_completed_pct | 77.78 / 66.67 | 93.75 / 77.78 | 84.62 / 58.33 |
| support.releases_per_10min | 10 / 12 | 17 / 9 | 13 / 12 |
| support.run_abandon_per_10min | 0 / 0 | 0 / 0 | 0 / 0 |
| support.runs_in_behind_per_10min | 1 / 1 | 2 / 1 | 4 / 2 |
| support.ticks_with_le1_option_pct | 54.68 / 50.46 | 51.18 / 48.35 | 51.10 / 50.76 |
| support.width_m | 15.18 / 15.31 | 16.05 / 14.98 | 15.96 / 14.24 |
| trans.count_per_10min | 19 / 20 | 22 / 19 | 20 / 20 |
| trans.counterpress_pct | 36.84 / 40 | 27.27 / 42.11 | 30 / 25 |
| trans.forward_recovery_speed_mps_0_5s | 6.28 / 6.51 | 6.43 / 6.56 | 6.50 / 6.81 |
| trans.forward_sprint_share_0_5s | 59.37 / 60.48 | 61.43 / 65.02 | 60.00 / 64.88 |
| trans.forward_sprint_share_0_5s_7mps | 42.93 / 45.51 | 46.69 / 48.27 | 48.54 / 50.81 |
| trans.forwards_staying_high_mean | 1 / 0.75 | 1 / 0.70 | 0.75 / 0.78 |
| trans.in_shape_at_turnover_pct | 61.11 / 55.56 | 76.47 / 75 | 55.56 / 58.82 |
| trans.mid_recovery_speed_mps_0_5s | 6.17 / 6.33 | 6.38 / 6.44 | 6.42 / 6.51 |
| trans.mid_sprint_share_0_5s | 60.19 / 60.24 | 60.63 / 62.08 | 62.52 / 64.38 |
| trans.mid_sprint_share_0_5s_7mps | 47.10 / 49.25 | 49.75 / 51.62 | 51.12 / 54.09 |
| trans.outfield_stamina_end_permille | 722.5 / 883.5 | 890.8 / 958 | 886.8 / 915.8 |
| trans.pct_outfield_behind_ball_at_3s | 67.31 / 57.35 | 66.18 / 78.12 | 66.18 / 76.79 |
| trans.pct_outfield_behind_ball_at_6s | 53.57 / 71.88 | 52.27 / 80 | 58.33 / 72.22 |
| trans.time_to_shape_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_excluded_pct | 5.26 / 10 | 22.73 / 15.79 | 10 / 15 |
| trans.time_to_shape_s | 0.73 / 1.30 | 0.46 / 0.62 | 1.44 / 1.11 |
| trans.time_to_shape_static_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_static_excluded_pct | 31.58 / 35 | 36.36 / 5.26 | 30 / 25 |
| trans.time_to_shape_static_s | 0.38 / 0.76 | 0.97 / 0.94 | 0.23 / 0.63 |

GK scenario field: not present in the public state.

## 5v5 normal vs normal (10 min x 3 seeds from 1)

Command: `psfc-replay analyze --format 5v5 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty normal --away-difficulty normal`

Gameplay version: `0.1.0-5v5`

| seed | final state_hash | score (H-A) |
|---|---|---|
| 1 | `5686657451b610c6` | 4-2 |
| 2 | `f0b0695d8f270699` | 1-1 |
| 3 | `51e9fda5045e39e9` | 3-6 |

Match meta (mean over seeds): goals_total = 5.67, open_play_minutes = 10.00, open_play_pct_of_ticks = 95.01, shots_total = 57, ticks_simulated = 37913

### Match

| metric | home | away | teams mean |
|---|---:|---:|---:|
| fouls | 0 | 0.67 | **0.33** |
| goals | 2.67 | 3 | **2.83** |
| offsides | 2 | 2 | **2** |
| passes_completed_pct | 88.06 | 87.08 | **87.57** |
| possession_pct | 52.26 | 47.74 | **50** |
| shots | 33.33 | 23.67 | **28.50** |
| shots_on_target | 26.67 | 19.33 | **23** |

### Goalkeeper

| metric | home | away | teams mean |
|---|---:|---:|---:|
| catches | 1 | 1.33 | **1.17** |
| claims_non_save | 0.67 | 1 | **0.83** |
| excursion_mean_s | 0.11 | 0 | **0.05** |
| excursions_gt1s_per_10min | 0 | 0 | **0** |
| gathers | 1.67 | 2.33 | **2** |
| goals_conceded | 3 | 2.67 | **2.83** |
| goals_conceded_gk_gt3m_off_line | 1.67 | 0.67 | **1.17** |
| goals_conceded_no_shot_found | 0 | 0 | **0** |
| line_dist_at_conceded_shot_mean_m | 2.65 | 2.48 | **2.56** |
| line_dist_max_m | 5.12 | 5.32 | **5.22** |
| line_dist_mean_m | 2.96 | 2.95 | **2.96** |
| parries | 15.33 | 23 | **19.17** |
| pct_outside_6yd | 0.02 | 0 | **0.01** |
| pct_outside_box | 0 | 0 | **0** |
| rush_count | 0.33 | 0 | **0.17** |
| rush_per_10min | 0.33 | 0 | **0.17** |
| saves | 16.33 | 24.33 | **20.33** |
| tips_over | 0 | 0 | **0** |

### Defence and shape

| metric | home | away | teams mean |
|---|---:|---:|---:|
| attacking.anchor_dist_m.defender | 9.43 | 7.92 | **8.68** |
| attacking.anchor_dist_m.forward | 15.57 | 16.87 | **16.22** |
| attacking.anchor_dist_m.goalkeeper | 1.26 | 1.32 | **1.29** |
| attacking.anchor_dist_m.midfielder | 20.21 | 16.28 | **18.24** |
| attacking.anchor_dist_shape_m.defender | 7.99 | 7.33 | **7.66** |
| attacking.anchor_dist_shape_m.forward | 10.57 | 10.40 | **10.49** |
| attacking.anchor_dist_shape_m.midfielder | 14.86 | 10.00 | **12.43** |
| attacking.bbox_area_m2 | 331.6 | 322.2 | **326.9** |
| attacking.bbox_depth_m | 18.55 | 18.23 | **18.39** |
| attacking.bbox_width_m | 16.46 | 16.57 | **16.52** |
| attacking.length_m | 18.55 | 18.23 | **18.39** |
| attacking.nearest_mate_mean_m | 10.13 | 10.09 | **10.11** |
| attacking.nearest_mate_p10_m | 3.51 | 3.64 | **3.57** |
| attacking.pct_of_open_play | 50.30 | 49.70 | **50** |
| attacking.stretch_m | 9.22 | 9.16 | **9.19** |
| attacking.width_m | 16.46 | 16.57 | **16.52** |
| defending.anchor_dist_m.defender | 7.77 | 8.29 | **8.03** |
| defending.anchor_dist_m.forward | 15.02 | 16.52 | **15.77** |
| defending.anchor_dist_m.goalkeeper | 1.35 | 1.46 | **1.40** |
| defending.anchor_dist_m.midfielder | 18.80 | 14.75 | **16.78** |
| defending.anchor_dist_shape_m.defender | 8.72 | 9.50 | **9.11** |
| defending.anchor_dist_shape_m.forward | 9.12 | 10.14 | **9.63** |
| defending.anchor_dist_shape_m.midfielder | 14.15 | 9.27 | **11.71** |
| defending.bbox_area_m2 | 370.7 | 339.6 | **355.2** |
| defending.bbox_depth_m | 20.06 | 18.69 | **19.38** |
| defending.bbox_width_m | 17.18 | 17.27 | **17.22** |
| defending.length_m | 20.06 | 18.69 | **19.38** |
| defending.line_x_std_m | 6.38 | 5.79 | **6.08** |
| defending.line_x_variance_m2 | 64.98 | 56.33 | **60.66** |
| defending.nearest_mate_mean_m | 10.95 | 10.38 | **10.67** |
| defending.nearest_mate_p10_m | 4.15 | 3.76 | **3.96** |
| defending.pct_of_open_play | 49.70 | 50.30 | **50** |
| defending.pct_outfield_behind_ball | 67.18 | 63.79 | **65.48** |
| defending.stretch_m | 9.79 | 9.46 | **9.63** |
| defending.swarm2_pct | 0.08 | 0.29 | **0.18** |
| defending.swarm3_pct | 0 | 0 | **0** |
| defending.width_m | 17.18 | 17.27 | **17.22** |

### Defensive transition

| metric | home | away | teams mean |
|---|---:|---:|---:|
| count_per_10min | 22.99 | 21.99 | **22.49** |
| counterpress_pct | 34.97 | 42.22 | **38.59** |
| forward_recovery_speed_mps_0_5s | 6.60 | 6.66 | **6.63** |
| forward_sprint_share_0_5s | 64.90 | 65.64 | **65.27** |
| forward_sprint_share_0_5s_7mps | 48.81 | 49.40 | **49.11** |
| forwards_staying_high_mean | 0.72 | 0.79 | **0.75** |
| in_shape_at_turnover_pct | 70.00 | 62.77 | **66.39** |
| mid_recovery_speed_mps_0_5s | 6.30 | 6.29 | **6.29** |
| mid_sprint_share_0_5s | 60.26 | 59.27 | **59.76** |
| mid_sprint_share_0_5s_7mps | 48.70 | 47.17 | **47.94** |
| outfield_stamina_end_permille | 886.8 | 880.3 | **883.5** |
| pct_outfield_behind_ball_at_3s | 70.62 | 58.22 | **64.42** |
| pct_outfield_behind_ball_at_6s | 71.62 | 65.25 | **68.44** |
| time_to_shape_capped_pct | 0 | 0 | **0** |
| time_to_shape_excluded_pct | 10.11 | 15.28 | **12.69** |
| time_to_shape_s | 0.56 | 0.90 | **0.73** |
| time_to_shape_static_capped_pct | 0 | 0 | **0** |
| time_to_shape_static_excluded_pct | 18.25 | 29.58 | **23.92** |
| time_to_shape_static_s | 0.47 | 0.82 | **0.65** |

### Attacking support

| metric | home | away | teams mean |
|---|---:|---:|---:|
| carrier_pct_of_open_play | 21.44 | 19.37 | **20.40** |
| cluster_pct | 8.46 | 6.91 | **7.69** |
| depth_m | 19.95 | 20.14 | **20.05** |
| forward_options_mean | 0.22 | 0.26 | **0.24** |
| nearest_option_dist_m | 11.95 | 12.24 | **12.10** |
| open_options_mean | 1.30 | 1.35 | **1.33** |
| open_options_p10 | 0 | 0 | **0** |
| pass_target_open_pct | 73.77 | 72.08 | **72.93** |
| passes_completed_tracked | 124.7 | 122.3 | **123.5** |
| release_pass_completed_pct | 74.19 | 70.95 | **72.57** |
| releases_per_10min | 12.33 | 13.00 | **12.66** |
| run_abandon_per_10min | 0 | 0 | **0** |
| runs_in_behind_per_10min | 4.67 | 1.67 | **3.17** |
| ticks_with_le1_option_pct | 55.20 | 52.64 | **53.92** |
| width_m | 16.20 | 16.26 | **16.23** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 10.85 | 11.03 | **10.94** |
| accel_gt10_pct | 50.07 | 51.13 | **50.60** |
| accel_mean_mps2 | 10.83 | 11.01 | **10.92** |
| class_time_pct.0 | 12.37 | 12.28 | **12.32** |
| class_time_pct.1 | 6.92 | 7.41 | **7.17** |
| class_time_pct.2 | 66.05 | 65.04 | **65.55** |
| class_time_pct.3 | 14.66 | 15.27 | **14.96** |
| dspeed_max_mps_per_tick | 3.63 | 3.70 | **3.66** |
| heading_change_p95_deg_per_tick_gt3mps | 3.38 | 3.48 | **3.43** |
| heading_change_pct.0_10 | 99.49 | 99.39 | **99.44** |
| heading_change_pct.10_45 | 0.49 | 0.59 | **0.54** |
| heading_change_pct.45_90 | 0.02 | 0.02 | **0.02** |
| heading_change_pct.gt90 | 0.00 | 0.00 | **0.00** |
| heading_sign_flips_per_player_min | 16.28 | 16.83 | **16.55** |
| lateral_accel_mean_mps2 | 8.08 | 8.27 | **8.18** |
| reversals_per_player_min | 0.03 | 0.02 | **0.02** |
| side_flips_by_class.0_per_player_min | 102.5 | 105.6 | **104.0** |
| side_flips_by_class.0_share_pct | 36.07 | 36.16 | **36.12** |
| side_flips_by_class.1_per_player_min | 44.94 | 44.27 | **44.60** |
| side_flips_by_class.1_share_pct | 8.88 | 9.20 | **9.04** |
| side_flips_by_class.2_per_player_min | 24.11 | 24.82 | **24.47** |
| side_flips_by_class.2_share_pct | 45.32 | 45.05 | **45.18** |
| side_flips_by_class.3_per_player_min | 23.33 | 22.50 | **22.92** |
| side_flips_by_class.3_share_pct | 9.74 | 9.58 | **9.66** |
| side_flips_cause.desired_per_player_min | 14.19 | 14.05 | **14.12** |
| side_flips_cause.filter_per_player_min | 17.84 | 18.41 | **18.12** |
| side_flips_cause.limiter_per_player_min | 1.61 | 1.45 | **1.53** |
| side_flips_cause.sampler_per_player_min | 1.49 | 1.94 | **1.72** |
| side_flips_per_player_min | 35.13 | 35.85 | **35.49** |
| side_flips_schmitt_per_player_min | 39.92 | 40.15 | **40.03** |
| side_flips_speed_ge3_per_player_min | 46.37 | 46.28 | **46.32** |
| side_flips_speed_lt3_per_player_min | 10.16 | 12.21 | **11.19** |
| speed_oscillation_pct | 4.30 | 4.45 | **4.38** |
| turn_rate_mean_deg_per_s | 97.86 | 101.9 | **99.86** |

### Collisions and push-out

| metric | home | away | teams mean |
|---|---:|---:|---:|
| any.depth_max_m | 0.54 | 0.54 | **0.54** |
| any.depth_mean_m | 0.22 | 0.22 | **0.22** |
| any.events_per_min | 1.40 | 1.40 | **1.40** |
| any.stuck_mean_ticks | 7.78 | 7.78 | **7.78** |
| any.stuck_pairs | 1.33 | 1.33 | **1.33** |
| any.ticks_gt1cm_per_min | 3.60 | 3.60 | **3.60** |
| any.ticks_per_min | 3.90 | 3.90 | **3.90** |
| correction_vel_corrected_p50_mps | 0.99 | 0.75 | **0.87** |
| correction_vel_corrected_p99_mps | 9.12 | 12.16 | **10.64** |
| correction_vel_corrected_pct | 0.06 | 0.06 | **0.06** |
| correction_vel_max_mps | 12.48 | 12.74 | **12.61** |
| correction_vel_p50_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p95_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p99_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_pct_gt_0_3 | 0.04 | 0.04 | **0.04** |
| opponent.depth_max_m | 0.09 | 0.09 | **0.09** |
| opponent.depth_mean_m | 0.03 | 0.03 | **0.03** |
| opponent.events_per_min | 0.90 | 0.90 | **0.90** |
| opponent.stuck_mean_ticks | 0 | 0 | **0** |
| opponent.stuck_pairs | 0 | 0 | **0** |
| opponent.ticks_gt1cm_per_min | 1.00 | 1.00 | **1.00** |
| opponent.ticks_per_min | 1.23 | 1.23 | **1.23** |
| teammate.depth_max_m | 0.52 | 0.53 | **0.52** |
| teammate.depth_mean_m | 0.25 | 0.26 | **0.26** |
| teammate.events_per_min | 0.23 | 0.27 | **0.25** |
| teammate.stuck_mean_ticks | 5 | 7.33 | **6.17** |
| teammate.stuck_pairs | 0.33 | 1 | **0.67** |
| teammate.ticks_gt1cm_per_min | 1.83 | 2.16 | **2.00** |
| teammate.ticks_per_min | 1.83 | 2.23 | **2.03** |

### Coordination

| metric | home | away | teams mean |
|---|---:|---:|---:|
| cruise_over72_pct | 36.71 | 35.21 | **35.96** |
| dual_chaser_pct_of_loose | 18.21 | 20.18 | **19.20** |
| dual_chaser_ticks_per_min | 385.9 | 427.3 | **406.6** |
| engagers3_pct_of_carrier_ticks | 0.01 | 0.09 | **0.05** |
| engagers3_ticks_per_min | 0.07 | 0.77 | **0.42** |
| mark_steal_per_defender_min | 0.43 | 0.83 | **0.63** |
| max_simultaneous_chasers | 5 | 4.33 | **4.67** |

### Aerial

| metric | home | away | teams mean |
|---|---:|---:|---:|
| headers | 8.67 | 8.33 | **8.50** |
| headers_clear | 1.33 | 2.67 | **2** |
| headers_pass | 5 | 3.67 | **4.33** |
| headers_shot | 2.33 | 2 | **2.17** |
| jumps_gk | 18.67 | 29 | **23.83** |
| jumps_outfield | 21 | 19.67 | **20.33** |
| max_airborne_outfield | 1 | 1 | **1** |

### Team shape vs envelope (teams mean)

| metric | in possession | out of possession | 11v11 envelope (m) |
|---|---:|---:|---|
| length_m | 18.39 | 19.38 | 31-46 |
| width_m | 16.52 | 17.22 | 35-48 |
| stretch_m | 9.19 | 9.63 | 7-16 |
| nearest_mate_mean_m | 10.11 | 10.67 | none published (in-possession mean near 9-10 m, estimate) |
| nearest_mate_p10_m | 3.57 | 3.96 | none published (in-possession mean near 9-10 m, estimate) |

Note: 5v5 has no published team-shape envelope; the 11v11 ranges are printed for reference only and do not apply to a 4-outfielder team on a small pitch.

### Per seed (home / away)

| metric | seed 1 | seed 2 | seed 3 |
|---|---:|---:|---:|
| aerial.headers | 7 / 7 | 9 / 9 | 10 / 9 |
| aerial.headers_clear | 2 / 3 | 0 / 3 | 2 / 2 |
| aerial.headers_pass | 3 / 4 | 7 / 3 | 5 / 4 |
| aerial.headers_shot | 2 / 0 | 2 / 3 | 3 / 3 |
| aerial.jumps_gk | 16 / 29 | 22 / 26 | 18 / 32 |
| aerial.jumps_outfield | 22 / 18 | 16 / 16 | 25 / 25 |
| aerial.max_airborne_outfield | 1 / 1 | 1 / 1 | 1 / 1 |
| collision.any.depth_max_m | 0.80 / 0.80 | 0.04 / 0.04 | 0.79 / 0.79 |
| collision.any.depth_mean_m | 0.35 / 0.35 | 0.02 / 0.02 | 0.30 / 0.30 |
| collision.any.events_per_min | 1.80 / 1.80 | 1.20 / 1.20 | 1.20 / 1.20 |
| collision.any.stuck_mean_ticks | 12.33 / 12.33 | 0 / 0 | 11 / 11 |
| collision.any.stuck_pairs | 3 / 3 | 0 / 0 | 1 / 1 |
| collision.any.ticks_gt1cm_per_min | 6.59 / 6.59 | 1 / 1 | 3.20 / 3.20 |
| collision.any.ticks_per_min | 6.99 / 6.99 | 1.30 / 1.30 | 3.40 / 3.40 |
| collision.correction_vel_corrected_p50_mps | 1.80 / 1.02 | 0.32 / 0.37 | 0.85 / 0.85 |
| collision.correction_vel_corrected_p99_mps | 3.60 / 12.65 | 12.05 / 10.94 | 11.71 / 12.89 |
| collision.correction_vel_corrected_pct | 0.07 / 0.07 | 0.06 / 0.05 | 0.06 / 0.06 |
| collision.correction_vel_max_mps | 12.96 / 14.13 | 12.20 / 10.94 | 12.27 / 13.14 |
| collision.correction_vel_p50_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p95_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p99_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_pct_gt_0_3 | 0.05 / 0.05 | 0.03 / 0.03 | 0.04 / 0.04 |
| collision.opponent.depth_max_m | 0.15 / 0.15 | 0.04 / 0.04 | 0.08 / 0.08 |
| collision.opponent.depth_mean_m | 0.04 / 0.04 | 0.02 / 0.02 | 0.04 / 0.04 |
| collision.opponent.events_per_min | 0.80 / 0.80 | 1.20 / 1.20 | 0.70 / 0.70 |
| collision.opponent.stuck_mean_ticks | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.stuck_pairs | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.ticks_gt1cm_per_min | 1.00 / 1.00 | 1 / 1 | 1 / 1 |
| collision.opponent.ticks_per_min | 1.30 / 1.30 | 1.30 / 1.30 | 1.10 / 1.10 |
| collision.teammate.depth_max_m | 0.78 / 0.80 | 0 / 0 | 0.77 / 0.79 |
| collision.teammate.depth_mean_m | 0.37 / 0.41 | 0 / 0 | 0.39 / 0.37 |
| collision.teammate.events_per_min | 0.50 / 0.50 | 0 / 0 | 0.20 / 0.30 |
| collision.teammate.stuck_mean_ticks | 15 / 11 | 0 / 0 | 0 / 11 |
| collision.teammate.stuck_pairs | 1 / 2 | 0 / 0 | 0 / 1 |
| collision.teammate.ticks_gt1cm_per_min | 4.29 / 4.29 | 0 / 0 | 1.20 / 2.20 |
| collision.teammate.ticks_per_min | 4.29 / 4.39 | 0 / 0 | 1.20 / 2.30 |
| coord.cruise_over72_pct | 37.40 / 34.12 | 37.82 / 34.96 | 34.93 / 36.55 |
| coord.dual_chaser_pct_of_loose | 17.06 / 20.17 | 17.38 / 20.04 | 20.20 / 20.34 |
| coord.dual_chaser_ticks_per_min | 358.9 / 424.3 | 361.6 / 417 | 437.3 / 440.5 |
| coord.engagers3_pct_of_carrier_ticks | 0.03 / 0.27 | 0 / 0 | 0 / 0 |
| coord.engagers3_ticks_per_min | 0.20 / 2.30 | 0 / 0 | 0 / 0 |
| coord.mark_steal_per_defender_min | 0.68 / 0.75 | 0.25 / 0.45 | 0.37 / 1.29 |
| coord.max_simultaneous_chasers | 5 / 4 | 5 / 4 | 5 / 5 |
| gk.catches | 0 / 0 | 3 / 2 | 0 / 2 |
| gk.claims_non_save | 1 / 0 | 0 / 1 | 1 / 2 |
| gk.excursion_mean_s | 0.32 / 0 | 0 / 0 | 0 / 0 |
| gk.excursions_gt1s_per_10min | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.gathers | 1 / 0 | 3 / 3 | 1 / 4 |
| gk.goals_conceded | 2 / 4 | 1 / 1 | 6 / 3 |
| gk.goals_conceded_gk_gt3m_off_line | 1 / 0 | 1 / 1 | 3 / 1 |
| gk.goals_conceded_no_shot_found | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.line_dist_at_conceded_shot_mean_m | 2.47 / 1.94 | 3.23 / 3.03 | 2.23 / 2.47 |
| gk.line_dist_max_m | 5.74 / 5.49 | 5.23 / 5.11 | 4.38 / 5.36 |
| gk.line_dist_mean_m | 2.92 / 2.92 | 2.98 / 2.98 | 2.99 / 2.96 |
| gk.parries | 15 / 27 | 21 / 19 | 10 / 23 |
| gk.pct_outside_6yd | 0.05 / 0 | 0 / 0 | 0 / 0 |
| gk.pct_outside_box | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_count | 1 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_per_10min | 1.00 / 0 | 0 / 0 | 0 / 0 |
| gk.saves | 15 / 27 | 24 / 21 | 10 / 25 |
| gk.tips_over | 0 / 0 | 0 / 0 | 0 / 0 |
| match.fouls | 0 / 1 | 0 / 1 | 0 / 0 |
| match.goals | 4 / 2 | 1 / 1 | 3 / 6 |
| match.offsides | 0 / 2 | 2 / 1 | 4 / 3 |
| match.passes_completed_pct | 85.96 / 84.67 | 88.52 / 89.87 | 89.68 / 86.70 |
| match.possession_pct | 55.92 / 44.08 | 52.25 / 47.75 | 48.60 / 51.40 |
| match.shots | 35 / 22 | 26 / 27 | 39 / 22 |
| match.shots_on_target | 31 / 17 | 22 / 25 | 27 / 16 |
| move.accel_from_pos_mean_mps2 | 10.87 / 10.97 | 10.62 / 10.87 | 11.05 / 11.25 |
| move.accel_gt10_pct | 49.74 / 50.43 | 49.00 / 50.82 | 51.46 / 52.13 |
| move.accel_mean_mps2 | 10.86 / 10.94 | 10.60 / 10.86 | 11.03 / 11.23 |
| move.class_time_pct.0 | 13.19 / 11.79 | 12.19 / 11.91 | 11.72 / 13.13 |
| move.class_time_pct.1 | 6.35 / 7.95 | 6.87 / 7.65 | 7.55 / 6.63 |
| move.class_time_pct.2 | 65.62 / 65.40 | 67.02 / 65.04 | 65.52 / 64.68 |
| move.class_time_pct.3 | 14.85 / 14.86 | 13.91 / 15.40 | 15.21 / 15.56 |
| move.dspeed_max_mps_per_tick | 3.64 / 3.67 | 3.56 / 3.62 | 3.69 / 3.80 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.39 / 3.44 | 3.41 / 3.47 | 3.35 / 3.54 |
| move.heading_change_pct.0_10 | 99.49 / 99.39 | 99.47 / 99.47 | 99.52 / 99.33 |
| move.heading_change_pct.10_45 | 0.50 / 0.60 | 0.51 / 0.51 | 0.46 / 0.66 |
| move.heading_change_pct.45_90 | 0.01 / 0.01 | 0.02 / 0.02 | 0.02 / 0.02 |
| move.heading_change_pct.gt90 | 0.00 / 0.00 | 0.00 / 0 | 0.00 / 0.00 |
| move.heading_sign_flips_per_player_min | 16.22 / 16.56 | 16.21 / 16.11 | 16.40 / 17.82 |
| move.lateral_accel_mean_mps2 | 7.95 / 8.04 | 7.99 / 8.34 | 8.31 / 8.44 |
| move.reversals_per_player_min | 0.02 / 0.04 | 0.04 / 0 | 0.02 / 0.02 |
| move.side_flips_by_class.0_per_player_min | 99.78 / 100.4 | 108.5 / 106.4 | 99.27 / 109.9 |
| move.side_flips_by_class.0_share_pct | 36.86 / 33.71 | 37.94 / 35.72 | 33.39 / 39.06 |
| move.side_flips_by_class.1_per_player_min | 43.18 / 47.06 | 46.00 / 43.14 | 45.63 / 42.61 |
| move.side_flips_by_class.1_share_pct | 7.68 / 10.65 | 9.07 / 9.31 | 9.89 / 7.64 |
| move.side_flips_by_class.2_per_player_min | 24.84 / 24.65 | 22.84 / 24.80 | 24.66 / 25.01 |
| move.side_flips_by_class.2_share_pct | 45.66 / 45.90 | 43.92 / 45.49 | 46.38 / 43.77 |
| move.side_flips_by_class.3_per_player_min | 23.57 / 23.01 | 22.74 / 21.84 | 23.69 / 22.65 |
| move.side_flips_by_class.3_share_pct | 9.80 / 9.74 | 9.07 / 9.48 | 10.34 / 9.53 |
| move.side_flips_cause.desired_per_player_min | 14.38 / 12.28 | 15.07 / 13.83 | 13.11 / 16.04 |
| move.side_flips_cause.filter_per_player_min | 18.22 / 18.76 | 17.01 / 18.65 | 18.28 / 17.82 |
| move.side_flips_cause.limiter_per_player_min | 1.46 / 1.70 | 1.40 / 1.38 | 1.98 / 1.26 |
| move.side_flips_cause.sampler_per_player_min | 1.64 / 2.38 | 1.38 / 1.60 | 1.46 / 1.84 |
| move.side_flips_per_player_min | 35.70 / 35.12 | 34.86 / 35.46 | 34.84 / 36.96 |
| move.side_flips_schmitt_per_player_min | 39.40 / 39.16 | 40.53 / 40.41 | 39.82 / 40.89 |
| move.side_flips_speed_ge3_per_player_min | 47.08 / 45.81 | 45.83 / 46.11 | 46.20 / 46.92 |
| move.side_flips_speed_lt3_per_player_min | 10.93 / 11.40 | 9.75 / 11.17 | 9.81 / 14.07 |
| move.speed_oscillation_pct | 4.21 / 4.47 | 4.39 / 4.42 | 4.32 / 4.46 |
| move.turn_rate_mean_deg_per_s | 96.38 / 99.72 | 97.30 / 101.5 | 99.90 / 104.3 |
| shape.attacking.anchor_dist_m.defender | 10.28 / 8.31 | 7.81 / 7.55 | 10.22 / 7.91 |
| shape.attacking.anchor_dist_m.forward | 15.91 / 16.93 | 15.74 / 16.86 | 15.05 / 16.81 |
| shape.attacking.anchor_dist_m.goalkeeper | 1.28 / 1.39 | 1.26 / 1.26 | 1.24 / 1.31 |
| shape.attacking.anchor_dist_m.midfielder | 17.25 / 16.48 | 23.34 / 14.55 | 20.02 / 17.81 |
| shape.attacking.anchor_dist_shape_m.defender | 8.54 / 7.85 | 7.38 / 6.91 | 8.07 / 7.24 |
| shape.attacking.anchor_dist_shape_m.forward | 10.90 / 10.90 | 10.09 / 10.31 | 10.72 / 10.01 |
| shape.attacking.anchor_dist_shape_m.midfielder | 10.95 / 9.82 | 19.67 / 8.47 | 13.95 / 11.72 |
| shape.attacking.bbox_area_m2 | 343.2 / 339.1 | 331.7 / 299.2 | 319.9 / 328.4 |
| shape.attacking.bbox_depth_m | 19.27 / 18.50 | 18.03 / 18.03 | 18.36 / 18.18 |
| shape.attacking.bbox_width_m | 16.16 / 17.24 | 17.13 / 15.69 | 16.09 / 16.79 |
| shape.attacking.length_m | 19.27 / 18.50 | 18.03 / 18.03 | 18.36 / 18.18 |
| shape.attacking.nearest_mate_mean_m | 10.23 / 10.39 | 10.23 / 9.69 | 9.93 / 10.19 |
| shape.attacking.nearest_mate_p10_m | 3.41 / 3.56 | 3.86 / 3.39 | 3.24 / 3.99 |
| shape.attacking.pct_of_open_play | 51.99 / 48.01 | 52.45 / 47.55 | 46.48 / 53.52 |
| shape.attacking.stretch_m | 9.32 / 9.43 | 9.21 / 8.90 | 9.12 / 9.16 |
| shape.attacking.width_m | 16.16 / 17.24 | 17.13 / 15.69 | 16.09 / 16.79 |
| shape.defending.anchor_dist_m.defender | 7.79 / 9.41 | 7.48 / 7.25 | 8.05 / 8.20 |
| shape.defending.anchor_dist_m.forward | 15.74 / 17.15 | 14.51 / 14.89 | 14.83 / 17.52 |
| shape.defending.anchor_dist_m.goalkeeper | 1.43 / 1.57 | 1.32 / 1.34 | 1.30 / 1.47 |
| shape.defending.anchor_dist_m.midfielder | 14.53 / 15.51 | 22.73 / 12.76 | 19.14 / 15.98 |
| shape.defending.anchor_dist_shape_m.defender | 9.31 / 10.23 | 8.39 / 8.71 | 8.48 / 9.55 |
| shape.defending.anchor_dist_shape_m.forward | 8.91 / 10.55 | 9.01 / 9.32 | 9.44 / 10.54 |
| shape.defending.anchor_dist_shape_m.midfielder | 8.52 / 9.58 | 19.49 / 7.53 | 14.44 / 10.71 |
| shape.defending.bbox_area_m2 | 390.2 / 339.4 | 387.4 / 359.3 | 334.5 / 320.2 |
| shape.defending.bbox_depth_m | 20.09 / 18.54 | 20.54 / 19.74 | 19.55 / 17.79 |
| shape.defending.bbox_width_m | 18.13 / 17.35 | 17.40 / 17.43 | 15.99 / 17.02 |
| shape.defending.length_m | 20.09 / 18.54 | 20.54 / 19.74 | 19.55 / 17.79 |
| shape.defending.line_x_std_m | 6.42 / 5.65 | 6.64 / 6.01 | 6.07 / 5.71 |
| shape.defending.line_x_variance_m2 | 67.05 / 56.14 | 67.03 / 57.32 | 60.88 / 55.53 |
| shape.defending.nearest_mate_mean_m | 11.22 / 10.23 | 11.20 / 10.84 | 10.44 / 10.07 |
| shape.defending.nearest_mate_p10_m | 4.13 / 3.38 | 4.25 / 4.14 | 4.06 / 3.76 |
| shape.defending.pct_of_open_play | 48.01 / 51.99 | 47.55 / 52.45 | 53.52 / 46.48 |
| shape.defending.pct_outfield_behind_ball | 66.30 / 61.31 | 62.99 / 67.35 | 72.24 / 62.70 |
| shape.defending.stretch_m | 10.05 / 9.47 | 9.94 / 9.76 | 9.39 / 9.16 |
| shape.defending.swarm2_pct | 0.03 / 0.80 | 0.21 / 0 | 0 / 0.06 |
| shape.defending.swarm3_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| shape.defending.width_m | 18.13 / 17.35 | 17.40 / 17.43 | 15.99 / 17.02 |
| support.carrier_pct_of_open_play | 23.17 / 18.32 | 21.81 / 19.94 | 19.33 / 19.86 |
| support.cluster_pct | 12.61 / 6.94 | 4.56 / 7.29 | 8.21 / 6.52 |
| support.depth_m | 20.65 / 20.50 | 19.03 / 19.97 | 20.17 / 19.97 |
| support.forward_options_mean | 0.21 / 0.22 | 0.25 / 0.24 | 0.19 / 0.31 |
| support.nearest_option_dist_m | 12.10 / 12.46 | 11.69 / 12.53 | 12.06 / 11.74 |
| support.open_options_mean | 1.19 / 1.39 | 1.47 / 1.30 | 1.26 / 1.34 |
| support.open_options_p10 | 0 / 0 | 0 / 0 | 0 / 0 |
| support.pass_target_open_pct | 67.77 / 71.84 | 79.43 / 75.61 | 74.11 / 68.79 |
| support.passes_completed_tracked | 121 / 103 | 141 / 123 | 112 / 141 |
| support.release_pass_completed_pct | 66.67 / 72.73 | 86.67 / 61.54 | 69.23 / 78.57 |
| support.releases_per_10min | 8.99 / 10.99 | 15 / 13 | 13 / 15 |
| support.run_abandon_per_10min | 0 / 0 | 0 / 0 | 0 / 0 |
| support.runs_in_behind_per_10min | 4.00 / 2.00 | 4 / 2 | 6 / 1 |
| support.ticks_with_le1_option_pct | 60.51 / 51.12 | 48.98 / 53.71 | 56.10 / 53.10 |
| support.width_m | 15.60 / 17.05 | 16.92 / 15.60 | 16.09 / 16.13 |
| trans.count_per_10min | 18.98 / 15.98 | 27 / 30 | 23 / 20 |
| trans.counterpress_pct | 26.32 / 50 | 48.15 / 36.67 | 30.43 / 40 |
| trans.forward_recovery_speed_mps_0_5s | 6.67 / 6.57 | 6.70 / 6.82 | 6.42 / 6.59 |
| trans.forward_sprint_share_0_5s | 65.74 / 64.04 | 68.43 / 69.59 | 60.53 / 63.31 |
| trans.forward_sprint_share_0_5s_7mps | 50.31 / 45.67 | 52.72 / 54.84 | 43.41 / 47.68 |
| trans.forwards_staying_high_mean | 0.67 / 0.57 | 0.77 / 0.94 | 0.71 / 0.86 |
| trans.in_shape_at_turnover_pct | 58.82 / 64.29 | 75 / 61.54 | 76.19 / 62.50 |
| trans.mid_recovery_speed_mps_0_5s | 6.45 / 5.96 | 6.36 / 6.44 | 6.09 / 6.47 |
| trans.mid_sprint_share_0_5s | 63.13 / 54.01 | 61.96 / 61.14 | 55.69 / 62.65 |
| trans.mid_sprint_share_0_5s_7mps | 50.96 / 41.72 | 50.40 / 50.71 | 44.76 / 49.06 |
| trans.outfield_stamina_end_permille | 963.5 / 899.5 | 733 / 916 | 963.8 / 825.5 |
| trans.pct_outfield_behind_ball_at_3s | 73.21 / 62.50 | 61.25 / 66 | 77.38 / 46.15 |
| trans.pct_outfield_behind_ball_at_6s | 77.78 / 67.86 | 69.23 / 67.19 | 67.86 / 60.71 |
| trans.time_to_shape_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_excluded_pct | 10.53 / 12.50 | 11.11 / 13.33 | 8.70 / 20 |
| trans.time_to_shape_s | 0.67 / 0.56 | 0.70 / 1.01 | 0.31 / 1.12 |
| trans.time_to_shape_static_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_static_excluded_pct | 15.79 / 18.75 | 25.93 / 20 | 13.04 / 50 |
| trans.time_to_shape_static_s | 0.50 / 0.40 | 0.63 / 0.99 | 0.29 / 1.08 |

GK scenario field: not present in the public state.

## 11v11 normal vs normal (10 min x 3 seeds from 1)

Command: `psfc-replay analyze --format 11v11 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty normal --away-difficulty normal`

Gameplay version: `0.1.0-5v5`

| seed | final state_hash | score (H-A) |
|---|---|---|
| 1 | `c8e1cb4f3d538784` | 2-6 |
| 2 | `1ac77891aef54da4` | 4-0 |
| 3 | `9fd4c89e795fabe7` | 1-0 |

Match meta (mean over seeds): goals_total = 4.33, open_play_minutes = 10, open_play_pct_of_ticks = 96.68, shots_total = 28.33, ticks_simulated = 37241

### Match

| metric | home | away | teams mean |
|---|---:|---:|---:|
| fouls | 0 | 0 | **0** |
| goals | 2.33 | 2 | **2.17** |
| offsides | 0 | 0.67 | **0.33** |
| passes_completed_pct | 82.93 | 81.84 | **82.38** |
| possession_pct | 47.10 | 52.90 | **50** |
| shots | 9.33 | 19 | **14.17** |
| shots_on_target | 7 | 13 | **10** |

### Goalkeeper

| metric | home | away | teams mean |
|---|---:|---:|---:|
| catches | 2.33 | 1 | **1.67** |
| claims_non_save | 0.67 | 0.67 | **0.67** |
| excursion_mean_s | 0.89 | 0.52 | **0.70** |
| excursions_gt1s_per_10min | 2 | 1 | **1.50** |
| gathers | 3 | 1.67 | **2.33** |
| goals_conceded | 2 | 2.33 | **2.17** |
| goals_conceded_gk_gt3m_off_line | 1.67 | 2 | **1.83** |
| goals_conceded_no_shot_found | 0 | 0 | **0** |
| line_dist_at_conceded_shot_mean_m | 1.36 | 4.11 | **2.74** |
| line_dist_max_m | 8.00 | 6.28 | **7.14** |
| line_dist_mean_m | 4.36 | 4.56 | **4.46** |
| parries | 8.67 | 4 | **6.33** |
| pct_outside_6yd | 0.75 | 0.38 | **0.56** |
| pct_outside_box | 0 | 0 | **0** |
| rush_count | 5 | 2.33 | **3.67** |
| rush_per_10min | 5 | 2.33 | **3.67** |
| saves | 11 | 5 | **8** |
| tips_over | 0 | 0 | **0** |

### Defence and shape

| metric | home | away | teams mean |
|---|---:|---:|---:|
| attacking.anchor_dist_m.defender | 18.60 | 23.14 | **20.87** |
| attacking.anchor_dist_m.forward | 27.16 | 25.62 | **26.39** |
| attacking.anchor_dist_m.goalkeeper | 2.28 | 2.25 | **2.26** |
| attacking.anchor_dist_m.midfielder | 18.91 | 18.75 | **18.83** |
| attacking.anchor_dist_shape_m.defender | 15.47 | 16.96 | **16.21** |
| attacking.anchor_dist_shape_m.forward | 20.37 | 20.62 | **20.50** |
| attacking.anchor_dist_shape_m.midfielder | 11.66 | 11.65 | **11.65** |
| attacking.bbox_area_m2 | 1795.4 | 1941.4 | **1868.4** |
| attacking.bbox_depth_m | 40.64 | 41.45 | **41.04** |
| attacking.bbox_width_m | 43.13 | 45.71 | **44.42** |
| attacking.length_m | 40.64 | 41.45 | **41.04** |
| attacking.nearest_mate_mean_m | 11.37 | 11.90 | **11.64** |
| attacking.nearest_mate_p10_m | 5.17 | 5.51 | **5.34** |
| attacking.pct_of_open_play | 50.07 | 49.93 | **50** |
| attacking.stretch_m | 18.59 | 19.40 | **19.00** |
| attacking.width_m | 43.13 | 45.71 | **44.42** |
| defending.anchor_dist_m.defender | 17.56 | 22.00 | **19.78** |
| defending.anchor_dist_m.forward | 29.89 | 28.15 | **29.02** |
| defending.anchor_dist_m.goalkeeper | 2.58 | 2.53 | **2.55** |
| defending.anchor_dist_m.midfielder | 19.20 | 19.59 | **19.39** |
| defending.anchor_dist_shape_m.defender | 16.66 | 18.25 | **17.46** |
| defending.anchor_dist_shape_m.forward | 21.17 | 21.65 | **21.41** |
| defending.anchor_dist_shape_m.midfielder | 11.44 | 12.42 | **11.93** |
| defending.bbox_area_m2 | 1422.4 | 1509.5 | **1466.0** |
| defending.bbox_depth_m | 36.68 | 37.23 | **36.95** |
| defending.bbox_width_m | 37.24 | 38.46 | **37.85** |
| defending.length_m | 36.68 | 37.23 | **36.95** |
| defending.line_x_std_m | 5.32 | 6.05 | **5.69** |
| defending.line_x_variance_m2 | 53.66 | 64.81 | **59.23** |
| defending.nearest_mate_mean_m | 10.08 | 10.37 | **10.23** |
| defending.nearest_mate_p10_m | 4.98 | 5.11 | **5.04** |
| defending.pct_of_open_play | 49.93 | 50.07 | **50** |
| defending.pct_outfield_behind_ball | 68.04 | 72.56 | **70.30** |
| defending.stretch_m | 16.42 | 16.86 | **16.64** |
| defending.swarm2_pct | 0 | 0.06 | **0.03** |
| defending.swarm3_pct | 0 | 0 | **0** |
| defending.width_m | 37.24 | 38.46 | **37.85** |

### Defensive transition

| metric | home | away | teams mean |
|---|---:|---:|---:|
| count_per_10min | 17.33 | 20.33 | **18.83** |
| counterpress_pct | 36.81 | 49.21 | **43.01** |
| forward_recovery_speed_mps_0_5s | 7.27 | 7.28 | **7.27** |
| forward_sprint_share_0_5s | 71.81 | 72.41 | **72.11** |
| forward_sprint_share_0_5s_7mps | 60.96 | 61.15 | **61.06** |
| forwards_staying_high_mean | 1.86 | 1.99 | **1.92** |
| in_shape_at_turnover_pct | 63.39 | 72.46 | **67.93** |
| mid_recovery_speed_mps_0_5s | 6.66 | 6.73 | **6.70** |
| mid_sprint_share_0_5s | 63.96 | 64.59 | **64.27** |
| mid_sprint_share_0_5s_7mps | 52.31 | 52.77 | **52.54** |
| outfield_stamina_end_permille | 883.5 | 893.1 | **888.3** |
| pct_outfield_behind_ball_at_3s | 66.34 | 64.44 | **65.39** |
| pct_outfield_behind_ball_at_6s | 65.72 | 73.96 | **69.84** |
| time_to_shape_capped_pct | 0 | 0 | **0** |
| time_to_shape_excluded_pct | 11.11 | 18.10 | **14.60** |
| time_to_shape_s | 1.20 | 0.88 | **1.04** |
| time_to_shape_static_capped_pct | 0 | 0 | **0** |
| time_to_shape_static_excluded_pct | 26.85 | 24.68 | **25.77** |
| time_to_shape_static_s | 0.72 | 0.73 | **0.73** |

### Attacking support

| metric | home | away | teams mean |
|---|---:|---:|---:|
| carrier_pct_of_open_play | 15.50 | 17.66 | **16.58** |
| cluster_pct | 7.20 | 8.62 | **7.91** |
| depth_m | 40.32 | 39.48 | **39.90** |
| forward_options_mean | 0.71 | 0.61 | **0.66** |
| nearest_option_dist_m | 12.28 | 12.88 | **12.58** |
| open_options_mean | 3.57 | 3.43 | **3.50** |
| open_options_p10 | 0.33 | 0 | **0.17** |
| pass_target_open_pct | 75.14 | 72.45 | **73.79** |
| passes_completed_tracked | 107 | 108.7 | **107.8** |
| release_pass_completed_pct | 88.89 | 91.67 | **90.28** |
| releases_per_10min | 3.33 | 4.33 | **3.83** |
| run_abandon_per_10min | 0 | 0 | **0** |
| runs_in_behind_per_10min | 3.33 | 3 | **3.17** |
| ticks_with_le1_option_pct | 21.25 | 24.26 | **22.76** |
| width_m | 42.82 | 44.96 | **43.89** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 9.69 | 9.50 | **9.60** |
| accel_gt10_pct | 46.48 | 45.52 | **46.00** |
| accel_mean_mps2 | 9.67 | 9.49 | **9.58** |
| class_time_pct.0 | 6.02 | 6.11 | **6.06** |
| class_time_pct.1 | 2.96 | 2.80 | **2.88** |
| class_time_pct.2 | 72.28 | 71.66 | **71.97** |
| class_time_pct.3 | 18.74 | 19.43 | **19.09** |
| dspeed_max_mps_per_tick | 4.41 | 4.62 | **4.52** |
| heading_change_p95_deg_per_tick_gt3mps | 3.08 | 3.11 | **3.10** |
| heading_change_pct.0_10 | 99.72 | 99.72 | **99.72** |
| heading_change_pct.10_45 | 0.27 | 0.27 | **0.27** |
| heading_change_pct.45_90 | 0.01 | 0.01 | **0.01** |
| heading_change_pct.gt90 | 0.00 | 0.00 | **0.00** |
| heading_sign_flips_per_player_min | 10.17 | 9.32 | **9.75** |
| lateral_accel_mean_mps2 | 7.10 | 7.00 | **7.05** |
| reversals_per_player_min | 0.01 | 0.01 | **0.01** |
| side_flips_by_class.0_per_player_min | 101.4 | 96.89 | **99.14** |
| side_flips_by_class.0_share_pct | 26.81 | 27.59 | **27.20** |
| side_flips_by_class.1_per_player_min | 47.97 | 48.09 | **48.03** |
| side_flips_by_class.1_share_pct | 6.21 | 6.28 | **6.24** |
| side_flips_by_class.2_per_player_min | 16.57 | 15.30 | **15.93** |
| side_flips_by_class.2_share_pct | 52.60 | 51.09 | **51.85** |
| side_flips_by_class.3_per_player_min | 17.49 | 16.63 | **17.06** |
| side_flips_by_class.3_share_pct | 14.38 | 15.04 | **14.71** |
| side_flips_cause.desired_per_player_min | 7.00 | 6.68 | **6.84** |
| side_flips_cause.filter_per_player_min | 13.96 | 13.18 | **13.57** |
| side_flips_cause.limiter_per_player_min | 0.88 | 0.84 | **0.86** |
| side_flips_cause.sampler_per_player_min | 0.92 | 0.75 | **0.83** |
| side_flips_per_player_min | 22.76 | 21.45 | **22.10** |
| side_flips_schmitt_per_player_min | 34.87 | 34.23 | **34.55** |
| side_flips_speed_ge3_per_player_min | 25.38 | 24.45 | **24.91** |
| side_flips_speed_lt3_per_player_min | 12.37 | 9.94 | **11.15** |
| speed_oscillation_pct | 3.20 | 3.14 | **3.17** |
| turn_rate_mean_deg_per_s | 82.17 | 81.13 | **81.65** |

### Collisions and push-out

| metric | home | away | teams mean |
|---|---:|---:|---:|
| any.depth_max_m | 0.80 | 0.80 | **0.80** |
| any.depth_mean_m | 0.24 | 0.24 | **0.24** |
| any.events_per_min | 2.43 | 2.43 | **2.43** |
| any.stuck_mean_ticks | 3.67 | 3.67 | **3.67** |
| any.stuck_pairs | 0.33 | 0.33 | **0.33** |
| any.ticks_gt1cm_per_min | 6.43 | 6.43 | **6.43** |
| any.ticks_per_min | 7.37 | 7.37 | **7.37** |
| correction_vel_corrected_p50_mps | 1.45 | 1.21 | **1.33** |
| correction_vel_corrected_p99_mps | 9.67 | 9.05 | **9.36** |
| correction_vel_corrected_pct | 0.04 | 0.04 | **0.04** |
| correction_vel_max_mps | 12.97 | 12.91 | **12.94** |
| correction_vel_p50_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p95_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p99_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_pct_gt_0_3 | 0.03 | 0.03 | **0.03** |
| opponent.depth_max_m | 0.28 | 0.28 | **0.28** |
| opponent.depth_mean_m | 0.06 | 0.06 | **0.06** |
| opponent.events_per_min | 1.70 | 1.70 | **1.70** |
| opponent.stuck_mean_ticks | 0 | 0 | **0** |
| opponent.stuck_pairs | 0 | 0 | **0** |
| opponent.ticks_gt1cm_per_min | 2.50 | 2.50 | **2.50** |
| opponent.ticks_per_min | 3.17 | 3.17 | **3.17** |
| teammate.depth_max_m | 0.80 | 0.51 | **0.66** |
| teammate.depth_mean_m | 0.37 | 0.24 | **0.30** |
| teammate.events_per_min | 0.50 | 0.23 | **0.37** |
| teammate.stuck_mean_ticks | 3.67 | 0 | **1.83** |
| teammate.stuck_pairs | 0.33 | 0 | **0.17** |
| teammate.ticks_gt1cm_per_min | 3.13 | 1.60 | **2.37** |
| teammate.ticks_per_min | 3.37 | 1.63 | **2.50** |

### Coordination

| metric | home | away | teams mean |
|---|---:|---:|---:|
| cruise_over72_pct | 43.02 | 43.70 | **43.36** |
| dual_chaser_pct_of_loose | 16.02 | 13.28 | **14.65** |
| dual_chaser_ticks_per_min | 383.1 | 317.5 | **350.3** |
| engagers3_pct_of_carrier_ticks | 0 | 0 | **0** |
| engagers3_ticks_per_min | 0 | 0 | **0** |
| mark_steal_per_defender_min | 0.14 | 0.18 | **0.16** |
| max_simultaneous_chasers | 4.67 | 4.33 | **4.50** |

### Aerial

| metric | home | away | teams mean |
|---|---:|---:|---:|
| headers | 15.67 | 13.33 | **14.50** |
| headers_clear | 0.67 | 0 | **0.33** |
| headers_pass | 13.67 | 9 | **11.33** |
| headers_shot | 1.33 | 4.33 | **2.83** |
| jumps_gk | 12.67 | 9.33 | **11** |
| jumps_outfield | 33.33 | 31.33 | **32.33** |
| max_airborne_outfield | 1 | 1 | **1** |

### Team shape vs envelope (teams mean)

| metric | in possession | out of possession | 11v11 envelope (m) |
|---|---:|---:|---|
| length_m | 41.04 | 36.95 | 31-46 |
| width_m | 44.42 | 37.85 | 35-48 |
| stretch_m | 19.00 (outside) | 16.64 (outside) | 7-16 |
| nearest_mate_mean_m | 11.64 | 10.23 | none published (in-possession mean near 9-10 m, estimate) |
| nearest_mate_p10_m | 5.34 | 5.04 | none published (in-possession mean near 9-10 m, estimate) |

Envelope: peer-reviewed 11v11 ranges (length 31-46 m, width 35-48 m, stretch index 7-16 m) quoted in the research report; the report's tighter targets are 31-36 / 35-40 / 7-10 m out of possession and 40-46 / 42-48 / 12-16 m in possession (estimates).

### Per seed (home / away)

| metric | seed 1 | seed 2 | seed 3 |
|---|---:|---:|---:|
| aerial.headers | 15 / 13 | 15 / 12 | 17 / 15 |
| aerial.headers_clear | 2 / 0 | 0 / 0 | 0 / 0 |
| aerial.headers_pass | 12 / 11 | 14 / 5 | 15 / 11 |
| aerial.headers_shot | 1 / 2 | 1 / 7 | 2 / 4 |
| aerial.jumps_gk | 18 / 6 | 10 / 16 | 10 / 6 |
| aerial.jumps_outfield | 32 / 31 | 34 / 27 | 34 / 36 |
| aerial.max_airborne_outfield | 1 / 1 | 1 / 1 | 1 / 1 |
| collision.any.depth_max_m | 0.80 / 0.80 | 0.80 / 0.80 | 0.80 / 0.80 |
| collision.any.depth_mean_m | 0.25 / 0.25 | 0.19 / 0.19 | 0.28 / 0.28 |
| collision.any.events_per_min | 2.30 / 2.30 | 2.20 / 2.20 | 2.80 / 2.80 |
| collision.any.stuck_mean_ticks | 11 / 11 | 0 / 0 | 0 / 0 |
| collision.any.stuck_pairs | 1 / 1 | 0 / 0 | 0 / 0 |
| collision.any.ticks_gt1cm_per_min | 6.30 / 6.30 | 4.10 / 4.10 | 8.90 / 8.90 |
| collision.any.ticks_per_min | 7.40 / 7.40 | 5.30 / 5.30 | 9.40 / 9.40 |
| collision.correction_vel_corrected_p50_mps | 1.63 / 1.48 | 0.91 / 0.35 | 1.80 / 1.80 |
| collision.correction_vel_corrected_p99_mps | 13.93 / 12.36 | 11.50 / 2.31 | 3.60 / 12.48 |
| collision.correction_vel_corrected_pct | 0.04 / 0.04 | 0.04 / 0.03 | 0.04 / 0.04 |
| collision.correction_vel_max_mps | 14.35 / 12.78 | 13.03 / 12.52 | 11.54 / 13.42 |
| collision.correction_vel_p50_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p95_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p99_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_pct_gt_0_3 | 0.03 / 0.03 | 0.03 / 0.02 | 0.03 / 0.04 |
| collision.opponent.depth_max_m | 0.18 / 0.18 | 0.14 / 0.14 | 0.51 / 0.51 |
| collision.opponent.depth_mean_m | 0.06 / 0.06 | 0.03 / 0.03 | 0.10 / 0.10 |
| collision.opponent.events_per_min | 1.40 / 1.40 | 1.70 / 1.70 | 2 / 2 |
| collision.opponent.stuck_mean_ticks | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.stuck_pairs | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.ticks_gt1cm_per_min | 2.10 / 2.10 | 1.70 / 1.70 | 3.70 / 3.70 |
| collision.opponent.ticks_per_min | 2.80 / 2.80 | 2.60 / 2.60 | 4.10 / 4.10 |
| collision.teammate.depth_max_m | 0.80 / 0.74 | 0.80 / 0 | 0.80 / 0.80 |
| collision.teammate.depth_mean_m | 0.35 / 0.31 | 0.35 / 0 | 0.39 / 0.40 |
| collision.teammate.events_per_min | 0.60 / 0.30 | 0.50 / 0 | 0.40 / 0.40 |
| collision.teammate.stuck_mean_ticks | 11 / 0 | 0 / 0 | 0 / 0 |
| collision.teammate.stuck_pairs | 1 / 0 | 0 / 0 | 0 / 0 |
| collision.teammate.ticks_gt1cm_per_min | 4.20 / 1.20 | 2.40 / 0 | 2.80 / 3.60 |
| collision.teammate.ticks_per_min | 4.50 / 1.30 | 2.70 / 0 | 2.90 / 3.60 |
| coord.cruise_over72_pct | 43.14 / 43.03 | 44.36 / 43.11 | 41.57 / 44.95 |
| coord.dual_chaser_pct_of_loose | 16.43 / 12.36 | 15.43 / 15.23 | 16.19 / 12.25 |
| coord.dual_chaser_ticks_per_min | 393.3 / 295.9 | 365.8 / 361.2 | 390.2 / 295.3 |
| coord.engagers3_pct_of_carrier_ticks | 0 / 0 | 0 / 0 | 0 / 0 |
| coord.engagers3_ticks_per_min | 0 / 0 | 0 / 0 | 0 / 0 |
| coord.mark_steal_per_defender_min | 0.24 / 0.08 | 0.12 / 0.23 | 0.06 / 0.23 |
| coord.max_simultaneous_chasers | 5 / 4 | 4 / 5 | 5 / 4 |
| gk.catches | 2 / 1 | 4 / 1 | 1 / 1 |
| gk.claims_non_save | 0 / 0 | 1 / 2 | 1 / 0 |
| gk.excursion_mean_s | 0.96 / 0 | 0.82 / 1.10 | 0.88 / 0.45 |
| gk.excursions_gt1s_per_10min | 4 / 0 | 2 / 3 | 0 / 0 |
| gk.gathers | 2 / 1 | 5 / 3 | 2 / 1 |
| gk.goals_conceded | 6 / 2 | 0 / 4 | 0 / 1 |
| gk.goals_conceded_gk_gt3m_off_line | 5 / 2 | 0 / 3 | 0 / 1 |
| gk.goals_conceded_no_shot_found | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.line_dist_at_conceded_shot_mean_m | 4.09 / 3.97 | 0 / 3.65 | 0 / 4.70 |
| gk.line_dist_max_m | 8.66 / 5.27 | 9.04 / 7.38 | 6.30 / 6.19 |
| gk.line_dist_mean_m | 4.28 / 4.66 | 4.32 / 4.57 | 4.50 / 4.44 |
| gk.parries | 10 / 0 | 9 / 7 | 7 / 5 |
| gk.pct_outside_6yd | 1.28 / 0 | 0.82 / 0.92 | 0.15 / 0.23 |
| gk.pct_outside_box | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_count | 8 / 0 | 6 / 4 | 1 / 3 |
| gk.rush_per_10min | 8 / 0 | 6 / 4 | 1 / 3 |
| gk.saves | 12 / 1 | 13 / 8 | 8 / 6 |
| gk.tips_over | 0 / 0 | 0 / 0 | 0 / 0 |
| match.fouls | 0 / 0 | 0 / 0 | 0 / 0 |
| match.goals | 2 / 6 | 4 / 0 | 1 / 0 |
| match.offsides | 0 / 0 | 0 / 2 | 0 / 0 |
| match.passes_completed_pct | 82.88 / 81.25 | 84.51 / 82.88 | 81.41 / 81.38 |
| match.possession_pct | 38.75 / 61.25 | 50.33 / 49.67 | 52.21 / 47.79 |
| match.shots | 4 / 21 | 14 / 24 | 10 / 12 |
| match.shots_on_target | 3 / 18 | 11 / 12 | 7 / 9 |
| move.accel_from_pos_mean_mps2 | 9.82 / 9.55 | 9.68 / 9.40 | 9.58 / 9.56 |
| move.accel_gt10_pct | 47.23 / 45.80 | 46.22 / 44.86 | 45.98 / 45.90 |
| move.accel_mean_mps2 | 9.79 / 9.53 | 9.66 / 9.40 | 9.57 / 9.54 |
| move.class_time_pct.0 | 5.81 / 6.36 | 6.12 / 5.94 | 6.12 / 6.01 |
| move.class_time_pct.1 | 3.40 / 2.38 | 2.77 / 3.04 | 2.72 / 2.98 |
| move.class_time_pct.2 | 72.73 / 69.90 | 72.77 / 71.36 | 71.33 / 73.71 |
| move.class_time_pct.3 | 18.05 / 21.36 | 18.35 / 19.66 | 19.82 / 17.29 |
| move.dspeed_max_mps_per_tick | 3.94 / 5.56 | 5.53 / 3.69 | 3.76 / 4.60 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.10 / 3.11 | 3.07 / 3.10 | 3.08 / 3.13 |
| move.heading_change_pct.0_10 | 99.72 / 99.72 | 99.71 / 99.73 | 99.72 / 99.71 |
| move.heading_change_pct.10_45 | 0.27 / 0.27 | 0.28 / 0.26 | 0.26 / 0.28 |
| move.heading_change_pct.45_90 | 0.01 / 0.01 | 0.01 / 0.01 | 0.01 / 0.01 |
| move.heading_change_pct.gt90 | 0.00 / 0 | 0 / 0.00 | 0.00 / 0.00 |
| move.heading_sign_flips_per_player_min | 10.41 / 9.65 | 10.11 / 9.29 | 10.00 / 9.01 |
| move.lateral_accel_mean_mps2 | 7.17 / 7.13 | 7.12 / 6.88 | 7.00 / 7.00 |
| move.reversals_per_player_min | 0.01 / 0 | 0 / 0.01 | 0.01 / 0.02 |
| move.side_flips_by_class.0_per_player_min | 108.3 / 96.71 | 97.70 / 98.69 | 98.19 / 95.27 |
| move.side_flips_by_class.0_share_pct | 25.98 / 28.62 | 26.85 / 26.80 | 27.60 / 27.34 |
| move.side_flips_by_class.1_per_player_min | 43.83 / 48.54 | 52.92 / 47.55 | 47.15 / 48.17 |
| move.side_flips_by_class.1_share_pct | 6.16 / 5.38 | 6.58 / 6.61 | 5.89 / 6.86 |
| move.side_flips_by_class.2_per_player_min | 17.81 / 15.23 | 16.24 / 16.05 | 15.66 / 14.62 |
| move.side_flips_by_class.2_share_pct | 53.45 / 49.53 | 53.09 / 52.31 | 51.27 / 51.43 |
| move.side_flips_by_class.3_per_player_min | 19.36 / 16.57 | 16.36 / 15.92 | 16.75 / 17.41 |
| move.side_flips_by_class.3_share_pct | 14.41 / 16.47 | 13.49 / 14.29 | 15.24 / 14.37 |
| move.side_flips_cause.desired_per_player_min | 7.17 / 6.54 | 7.19 / 7.03 | 6.63 / 6.46 |
| move.side_flips_cause.filter_per_player_min | 14.98 / 13.59 | 13.43 / 13.05 | 13.49 / 12.90 |
| move.side_flips_cause.limiter_per_player_min | 0.96 / 0.81 | 0.80 / 0.93 | 0.88 / 0.77 |
| move.side_flips_cause.sampler_per_player_min | 1.13 / 0.55 | 0.84 / 0.88 | 0.78 / 0.83 |
| move.side_flips_per_player_min | 24.24 / 21.49 | 22.26 / 21.89 | 21.78 / 20.96 |
| move.side_flips_schmitt_per_player_min | 35.36 / 33.04 | 35.10 / 34.81 | 34.15 / 34.83 |
| move.side_flips_speed_ge3_per_player_min | 26.86 / 25.00 | 24.87 / 24.82 | 24.42 / 23.52 |
| move.side_flips_speed_lt3_per_player_min | 13.55 / 8.87 | 11.83 / 10.82 | 11.72 / 10.12 |
| move.speed_oscillation_pct | 3.28 / 3.15 | 3.16 / 3.14 | 3.17 / 3.14 |
| move.turn_rate_mean_deg_per_s | 82.89 / 82.37 | 81.68 / 79.84 | 81.93 / 81.16 |
| shape.attacking.anchor_dist_m.defender | 16.75 / 22.40 | 19.57 / 24.12 | 19.47 / 22.89 |
| shape.attacking.anchor_dist_m.forward | 28.96 / 23.14 | 25.86 / 27.24 | 26.65 / 26.49 |
| shape.attacking.anchor_dist_m.goalkeeper | 2.33 / 2.21 | 2.27 / 2.27 | 2.23 / 2.26 |
| shape.attacking.anchor_dist_m.midfielder | 17.61 / 17.96 | 19.73 / 20.31 | 19.41 / 17.98 |
| shape.attacking.anchor_dist_shape_m.defender | 15.30 / 14.14 | 16.28 / 17.64 | 14.82 / 19.09 |
| shape.attacking.anchor_dist_shape_m.forward | 21.50 / 19.57 | 18.48 / 21.59 | 21.14 / 20.69 |
| shape.attacking.anchor_dist_shape_m.midfielder | 9.84 / 11.37 | 12.23 / 12.83 | 12.90 / 10.73 |
| shape.attacking.bbox_area_m2 | 1723.6 / 2033.4 | 1802.5 / 1948.6 | 1860.0 / 1842.3 |
| shape.attacking.bbox_depth_m | 39.48 / 42.42 | 40.91 / 42.13 | 41.52 / 39.79 |
| shape.attacking.bbox_width_m | 42.50 / 47.07 | 42.96 / 45.07 | 43.93 / 44.99 |
| shape.attacking.length_m | 39.48 / 42.42 | 40.91 / 42.13 | 41.52 / 39.79 |
| shape.attacking.nearest_mate_mean_m | 10.85 / 12.25 | 11.63 / 11.86 | 11.64 / 11.58 |
| shape.attacking.nearest_mate_p10_m | 4.85 / 5.66 | 5.28 / 5.67 | 5.37 / 5.20 |
| shape.attacking.pct_of_open_play | 47.20 / 52.80 | 51.01 / 48.99 | 52.02 / 47.98 |
| shape.attacking.stretch_m | 18.02 / 19.98 | 18.59 / 19.49 | 19.16 / 18.74 |
| shape.attacking.width_m | 42.50 / 47.07 | 42.96 / 45.07 | 43.93 / 44.99 |
| shape.defending.anchor_dist_m.defender | 16.64 / 20.85 | 18.77 / 23.10 | 17.28 / 22.06 |
| shape.defending.anchor_dist_m.forward | 32.27 / 25.69 | 29.20 / 29.19 | 28.18 / 29.56 |
| shape.defending.anchor_dist_m.goalkeeper | 2.71 / 2.47 | 2.57 / 2.51 | 2.46 / 2.59 |
| shape.defending.anchor_dist_m.midfielder | 18.81 / 19.07 | 20.03 / 20.65 | 18.75 / 19.06 |
| shape.defending.anchor_dist_shape_m.defender | 17.37 / 16.11 | 17.67 / 18.67 | 14.95 / 19.98 |
| shape.defending.anchor_dist_shape_m.forward | 22.97 / 19.15 | 19.51 / 23.37 | 21.03 / 22.43 |
| shape.defending.anchor_dist_shape_m.midfielder | 10.49 / 12.03 | 11.73 / 13.52 | 12.10 / 11.72 |
| shape.defending.bbox_area_m2 | 1334.0 / 1567.1 | 1442.1 / 1491.9 | 1491.1 / 1469.6 |
| shape.defending.bbox_depth_m | 34.87 / 38.17 | 36.90 / 37.02 | 38.28 / 36.49 |
| shape.defending.bbox_width_m | 36.75 / 38.97 | 37.47 / 38.14 | 37.51 / 38.27 |
| shape.defending.length_m | 34.87 / 38.17 | 36.90 / 37.02 | 38.28 / 36.49 |
| shape.defending.line_x_std_m | 4.96 / 6.68 | 5.36 / 5.66 | 5.64 / 5.81 |
| shape.defending.line_x_variance_m2 | 49.34 / 79.78 | 53.41 / 56.18 | 58.23 / 58.46 |
| shape.defending.nearest_mate_mean_m | 9.44 / 10.63 | 10.34 / 10.21 | 10.47 / 10.28 |
| shape.defending.nearest_mate_p10_m | 4.44 / 5.31 | 5.02 / 5.12 | 5.47 / 4.90 |
| shape.defending.pct_of_open_play | 52.80 / 47.20 | 48.99 / 51.01 | 47.98 / 52.02 |
| shape.defending.pct_outfield_behind_ball | 64.45 / 76.32 | 67.87 / 72.67 | 71.79 / 68.67 |
| shape.defending.stretch_m | 15.65 / 17.28 | 16.48 / 16.76 | 17.12 / 16.54 |
| shape.defending.swarm2_pct | 0 / 0.11 | 0 / 0.06 | 0 / 0 |
| shape.defending.swarm3_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| shape.defending.width_m | 36.75 / 38.97 | 37.47 / 38.14 | 37.51 / 38.27 |
| support.carrier_pct_of_open_play | 12.61 / 20.44 | 16.77 / 16.82 | 17.11 / 15.72 |
| support.cluster_pct | 8.46 / 8.64 | 6.25 / 6.95 | 6.89 / 10.27 |
| support.depth_m | 39.96 / 40.45 | 40.56 / 40.29 | 40.43 / 37.71 |
| support.forward_options_mean | 0.81 / 0.43 | 0.68 / 0.70 | 0.65 / 0.70 |
| support.nearest_option_dist_m | 11.85 / 13.46 | 12.38 / 12.97 | 12.63 / 12.19 |
| support.open_options_mean | 4.17 / 2.76 | 3.15 / 3.68 | 3.39 / 3.85 |
| support.open_options_p10 | 1 / 0 | 0 / 0 | 0 / 0 |
| support.pass_target_open_pct | 79.41 / 70.18 | 77.48 / 74.53 | 68.52 / 72.64 |
| support.passes_completed_tracked | 102 / 114 | 111 / 106 | 108 / 106 |
| support.release_pass_completed_pct | 100 / 75 | 100 / 100 | 66.67 / 100 |
| support.releases_per_10min | 1 / 9 | 2 / 3 | 7 / 1 |
| support.run_abandon_per_10min | 0 / 0 | 0 / 0 | 0 / 0 |
| support.runs_in_behind_per_10min | 2 / 4 | 4 / 4 | 4 / 1 |
| support.ticks_with_le1_option_pct | 13.26 / 35.30 | 28.21 / 19.77 | 22.28 / 17.71 |
| support.width_m | 42.72 / 45.57 | 41.87 / 44.72 | 43.88 / 44.58 |
| trans.count_per_10min | 18 / 20 | 18 / 21 | 16 / 20 |
| trans.counterpress_pct | 27.78 / 35 | 38.89 / 47.62 | 43.75 / 65 |
| trans.forward_recovery_speed_mps_0_5s | 7.12 / 7.12 | 7.53 / 7.32 | 7.15 / 7.39 |
| trans.forward_sprint_share_0_5s | 69.14 / 69.22 | 77.27 / 73.19 | 69.01 / 74.81 |
| trans.forward_sprint_share_0_5s_7mps | 57.11 / 58.69 | 67.53 / 62.61 | 58.25 / 62.13 |
| trans.forwards_staying_high_mean | 2.17 / 1.56 | 1.55 / 1.60 | 1.88 / 2.80 |
| trans.in_shape_at_turnover_pct | 71.43 / 86.67 | 68.75 / 77.78 | 50 / 52.94 |
| trans.mid_recovery_speed_mps_0_5s | 6.78 / 6.83 | 6.73 / 6.66 | 6.48 / 6.70 |
| trans.mid_sprint_share_0_5s | 66.43 / 64.95 | 64.65 / 63.90 | 60.79 / 64.91 |
| trans.mid_sprint_share_0_5s_7mps | 54.01 / 55.95 | 54.74 / 51.72 | 48.18 / 50.63 |
| trans.outfield_stamina_end_permille | 879.2 / 899.1 | 887.3 / 897.9 | 884 / 882.2 |
| trans.pct_outfield_behind_ball_at_3s | 58.75 / 68.33 | 76.92 / 65 | 63.33 / 60.00 |
| trans.pct_outfield_behind_ball_at_6s | 62.50 / 78.89 | 70.91 / 79.00 | 63.75 / 64.00 |
| trans.time_to_shape_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_excluded_pct | 22.22 / 25 | 11.11 / 14.29 | 0 / 15 |
| trans.time_to_shape_s | 1.12 / 0.40 | 1.06 / 0.50 | 1.43 / 1.72 |
| trans.time_to_shape_static_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_static_excluded_pct | 38.89 / 25 | 16.67 / 19.05 | 25 / 30 |
| trans.time_to_shape_static_s | 0.49 / 0.43 | 0.41 / 0.53 | 1.26 / 1.23 |

GK scenario field: not present in the public state.

## 5v5 autopilot-hard vs hard (10 min x 3 seeds from 1)

Command: `psfc-replay analyze --format 5v5 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty hard --away-difficulty hard --home-controller autopilot`

Gameplay version: `0.1.0-5v5`

| seed | final state_hash | score (H-A) |
|---|---|---|
| 1 | `387fbe5fc48fcf2a` | 7-1 |
| 2 | `a24742aa3e960901` | 10-3 |
| 3 | `cfa33756fff490ca` | 5-0 |

Match meta (mean over seeds): goals_total = 8.67, open_play_minutes = 10.00, open_play_pct_of_ticks = 93.04, shots_total = 47.67, ticks_simulated = 38702.7

### Match

| metric | home | away | teams mean |
|---|---:|---:|---:|
| fouls | 1 | 0 | **0.50** |
| goals | 7.33 | 1.33 | **4.33** |
| offsides | 2 | 0 | **1** |
| passes_completed_pct | 75.78 | 78.19 | **76.99** |
| possession_pct | 64.98 | 35.02 | **50** |
| shots | 41.67 | 6 | **23.83** |
| shots_on_target | 32 | 4 | **18** |

### Goalkeeper

| metric | home | away | teams mean |
|---|---:|---:|---:|
| catches | 0.67 | 5.33 | **3** |
| claims_non_save | 14 | 2.67 | **8.33** |
| excursion_mean_s | 0.99 | 0.74 | **0.87** |
| excursions_gt1s_per_10min | 1.67 | 1.00 | **1.33** |
| gathers | 14.67 | 8 | **11.33** |
| goals_conceded | 1.33 | 7.33 | **4.33** |
| goals_conceded_gk_gt3m_off_line | 0.33 | 3.67 | **2** |
| goals_conceded_no_shot_found | 0 | 0 | **0** |
| line_dist_at_conceded_shot_mean_m | 1.78 | 2.95 | **2.36** |
| line_dist_max_m | 9.64 | 8.47 | **9.06** |
| line_dist_mean_m | 3.07 | 2.85 | **2.96** |
| parries | 2.33 | 26.33 | **14.33** |
| pct_outside_6yd | 0.68 | 0.42 | **0.55** |
| pct_outside_box | 0 | 0 | **0** |
| rush_count | 4.67 | 3.33 | **4** |
| rush_per_10min | 4.67 | 3.33 | **4.00** |
| saves | 3 | 31.67 | **17.33** |
| tips_over | 0 | 0 | **0** |

### Defence and shape

| metric | home | away | teams mean |
|---|---:|---:|---:|
| attacking.anchor_dist_m.defender | 9.77 | 8.96 | **9.37** |
| attacking.anchor_dist_m.forward | 11.85 | 18.37 | **15.11** |
| attacking.anchor_dist_m.goalkeeper | 1.22 | 1.56 | **1.39** |
| attacking.anchor_dist_m.midfielder | 15.52 | 15.46 | **15.49** |
| attacking.anchor_dist_shape_m.defender | 7.42 | 8.67 | **8.04** |
| attacking.anchor_dist_shape_m.forward | 10.20 | 10.89 | **10.54** |
| attacking.anchor_dist_shape_m.midfielder | 10.61 | 10.26 | **10.43** |
| attacking.bbox_area_m2 | 441.3 | 307.7 | **374.5** |
| attacking.bbox_depth_m | 22.35 | 16.81 | **19.58** |
| attacking.bbox_width_m | 19.43 | 17.17 | **18.30** |
| attacking.length_m | 22.35 | 16.81 | **19.58** |
| attacking.nearest_mate_mean_m | 11.88 | 9.87 | **10.88** |
| attacking.nearest_mate_p10_m | 3.90 | 3.54 | **3.72** |
| attacking.pct_of_open_play | 59.54 | 40.46 | **50** |
| attacking.stretch_m | 11.06 | 8.97 | **10.02** |
| attacking.width_m | 19.43 | 17.17 | **18.30** |
| defending.anchor_dist_m.defender | 7.54 | 9.40 | **8.47** |
| defending.anchor_dist_m.forward | 12.08 | 16.95 | **14.51** |
| defending.anchor_dist_m.goalkeeper | 1.24 | 1.65 | **1.45** |
| defending.anchor_dist_m.midfielder | 14.44 | 14.18 | **14.31** |
| defending.anchor_dist_shape_m.defender | 7.79 | 9.67 | **8.73** |
| defending.anchor_dist_shape_m.forward | 9.61 | 11.05 | **10.33** |
| defending.anchor_dist_shape_m.midfielder | 9.15 | 10.13 | **9.64** |
| defending.bbox_area_m2 | 463.2 | 266.1 | **364.7** |
| defending.bbox_depth_m | 24.50 | 15.25 | **19.87** |
| defending.bbox_width_m | 18.82 | 16.69 | **17.75** |
| defending.length_m | 24.50 | 15.25 | **19.87** |
| defending.line_x_std_m | 8.04 | 4.05 | **6.04** |
| defending.line_x_variance_m2 | 90.96 | 26.78 | **58.87** |
| defending.nearest_mate_mean_m | 12.66 | 9.27 | **10.96** |
| defending.nearest_mate_p10_m | 5.29 | 3.50 | **4.40** |
| defending.pct_of_open_play | 40.46 | 59.54 | **50** |
| defending.pct_outfield_behind_ball | 74.48 | 62.43 | **68.46** |
| defending.stretch_m | 11.39 | 8.39 | **9.89** |
| defending.swarm2_pct | 0.66 | 0.90 | **0.78** |
| defending.swarm3_pct | 0 | 0 | **0** |
| defending.width_m | 18.82 | 16.69 | **17.75** |

### Defensive transition

| metric | home | away | teams mean |
|---|---:|---:|---:|
| count_per_10min | 29.66 | 52.99 | **41.32** |
| counterpress_pct | 72.15 | 33.95 | **53.05** |
| forward_recovery_speed_mps_0_5s | 6.86 | 6.41 | **6.64** |
| forward_sprint_share_0_5s | 68.62 | 59.96 | **64.29** |
| forward_sprint_share_0_5s_7mps | 55.69 | 46.37 | **51.03** |
| forwards_staying_high_mean | 0.40 | 0.59 | **0.50** |
| in_shape_at_turnover_pct | 68.85 | 44.45 | **56.65** |
| mid_recovery_speed_mps_0_5s | 6.64 | 6.38 | **6.51** |
| mid_sprint_share_0_5s | 65.50 | 60.93 | **63.22** |
| mid_sprint_share_0_5s_7mps | 53.67 | 47.13 | **50.40** |
| outfield_stamina_end_permille | 794.3 | 784.3 | **789.3** |
| pct_outfield_behind_ball_at_3s | 61.48 | 56.26 | **58.87** |
| pct_outfield_behind_ball_at_6s | 78.33 | 66.19 | **72.26** |
| time_to_shape_capped_pct | 0 | 0 | **0** |
| time_to_shape_excluded_pct | 26.93 | 23.11 | **25.02** |
| time_to_shape_s | 0.44 | 1.20 | **0.82** |
| time_to_shape_static_capped_pct | 0 | 0 | **0** |
| time_to_shape_static_excluded_pct | 24.68 | 42.05 | **33.37** |
| time_to_shape_static_s | 0.33 | 0.52 | **0.43** |

### Attacking support

| metric | home | away | teams mean |
|---|---:|---:|---:|
| carrier_pct_of_open_play | 22.37 | 11.88 | **17.12** |
| cluster_pct | 6.45 | 7.57 | **7.01** |
| depth_m | 22.20 | 17.71 | **19.96** |
| forward_options_mean | 0.29 | 0.34 | **0.32** |
| nearest_option_dist_m | 13.42 | 12.91 | **13.17** |
| open_options_mean | 1.27 | 0.99 | **1.13** |
| open_options_p10 | 0 | 0 | **0** |
| pass_target_open_pct | 73.40 | 62.26 | **67.83** |
| passes_completed_tracked | 123 | 89.33 | **106.2** |
| release_pass_completed_pct | 69.44 | 50 | **59.72** |
| releases_per_10min | 6.67 | 1.67 | **4.17** |
| run_abandon_per_10min | 0 | 0 | **0** |
| runs_in_behind_per_10min | 9.33 | 3.67 | **6.50** |
| ticks_with_le1_option_pct | 55.49 | 66.68 | **61.08** |
| width_m | 17.89 | 15.88 | **16.88** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 11.97 | 12.09 | **12.03** |
| accel_gt10_pct | 51.10 | 55.88 | **53.49** |
| accel_mean_mps2 | 11.94 | 11.97 | **11.95** |
| class_time_pct.0 | 17.22 | 11.35 | **14.29** |
| class_time_pct.1 | 6.46 | 8.97 | **7.71** |
| class_time_pct.2 | 62.53 | 67.44 | **64.98** |
| class_time_pct.3 | 13.80 | 12.24 | **13.02** |
| dspeed_max_mps_per_tick | 4.24 | 5.94 | **5.09** |
| heading_change_p95_deg_per_tick_gt3mps | 3.74 | 3.48 | **3.61** |
| heading_change_pct.0_10 | 99.13 | 99.31 | **99.22** |
| heading_change_pct.10_45 | 0.84 | 0.66 | **0.75** |
| heading_change_pct.45_90 | 0.03 | 0.03 | **0.03** |
| heading_change_pct.gt90 | 0.00 | 0.00 | **0.00** |
| heading_sign_flips_per_player_min | 28.90 | 18.17 | **23.53** |
| lateral_accel_mean_mps2 | 8.67 | 8.65 | **8.66** |
| reversals_per_player_min | 0.02 | 0.03 | **0.03** |
| side_flips_by_class.0_per_player_min | 304.0 | 119.6 | **211.8** |
| side_flips_by_class.0_share_pct | 69.44 | 35.65 | **52.55** |
| side_flips_by_class.1_per_player_min | 127.9 | 47.25 | **87.56** |
| side_flips_by_class.1_share_pct | 10.95 | 11.13 | **11.04** |
| side_flips_by_class.2_per_player_min | 18.85 | 24.82 | **21.83** |
| side_flips_by_class.2_share_pct | 15.64 | 43.97 | **29.80** |
| side_flips_by_class.3_per_player_min | 21.62 | 28.84 | **25.23** |
| side_flips_by_class.3_share_pct | 3.97 | 9.25 | **6.61** |
| side_flips_cause.desired_per_player_min | 61.32 | 14.76 | **38.04** |
| side_flips_cause.filter_per_player_min | 10.51 | 18.84 | **14.67** |
| side_flips_cause.limiter_per_player_min | 2.26 | 2.30 | **2.28** |
| side_flips_cause.sampler_per_player_min | 1.27 | 2.18 | **1.73** |
| side_flips_per_player_min | 75.36 | 38.09 | **56.72** |
| side_flips_schmitt_per_player_min | 37.96 | 39.57 | **38.77** |
| side_flips_speed_ge3_per_player_min | 104.0 | 46.73 | **75.35** |
| side_flips_speed_lt3_per_player_min | 11.71 | 15.26 | **13.48** |
| speed_oscillation_pct | 4.36 | 4.56 | **4.46** |
| turn_rate_mean_deg_per_s | 102.8 | 104.5 | **103.6** |

### Collisions and push-out

| metric | home | away | teams mean |
|---|---:|---:|---:|
| any.depth_max_m | 0.78 | 0.78 | **0.78** |
| any.depth_mean_m | 0.27 | 0.27 | **0.27** |
| any.events_per_min | 1.97 | 1.97 | **1.97** |
| any.stuck_mean_ticks | 12.83 | 12.83 | **12.83** |
| any.stuck_pairs | 2.67 | 2.67 | **2.67** |
| any.ticks_gt1cm_per_min | 5.00 | 5.00 | **5.00** |
| any.ticks_per_min | 5.50 | 5.50 | **5.50** |
| correction_vel_corrected_p50_mps | 0.58 | 0.64 | **0.61** |
| correction_vel_corrected_p99_mps | 12.22 | 12.83 | **12.53** |
| correction_vel_corrected_pct | 0.11 | 0.12 | **0.12** |
| correction_vel_max_mps | 14.01 | 14.22 | **14.11** |
| correction_vel_p50_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p95_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p99_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_pct_gt_0_3 | 0.08 | 0.09 | **0.08** |
| opponent.depth_max_m | 0.08 | 0.08 | **0.08** |
| opponent.depth_mean_m | 0.03 | 0.03 | **0.03** |
| opponent.events_per_min | 1.10 | 1.10 | **1.10** |
| opponent.stuck_mean_ticks | 0 | 0 | **0** |
| opponent.stuck_pairs | 0 | 0 | **0** |
| opponent.ticks_gt1cm_per_min | 1.00 | 1.00 | **1.00** |
| opponent.ticks_per_min | 1.37 | 1.37 | **1.37** |
| teammate.depth_max_m | 0.73 | 0.78 | **0.75** |
| teammate.depth_mean_m | 0.32 | 0.34 | **0.33** |
| teammate.events_per_min | 0.37 | 0.50 | **0.43** |
| teammate.stuck_mean_ticks | 10.67 | 11 | **10.83** |
| teammate.stuck_pairs | 1.33 | 1.33 | **1.33** |
| teammate.ticks_gt1cm_per_min | 3.17 | 2.63 | **2.90** |
| teammate.ticks_per_min | 3.27 | 2.73 | **3.00** |

### Coordination

| metric | home | away | teams mean |
|---|---:|---:|---:|
| cruise_over72_pct | 38.14 | 41.21 | **39.68** |
| dual_chaser_pct_of_loose | 14.74 | 18.89 | **16.81** |
| dual_chaser_ticks_per_min | 336.8 | 431.5 | **384.2** |
| engagers3_pct_of_carrier_ticks | 0.05 | 0.12 | **0.08** |
| engagers3_ticks_per_min | 0.23 | 1.03 | **0.63** |
| mark_steal_per_defender_min | 0.25 | 0.21 | **0.23** |
| max_simultaneous_chasers | 4 | 4.33 | **4.17** |

### Aerial

| metric | home | away | teams mean |
|---|---:|---:|---:|
| headers | 11.67 | 5 | **8.33** |
| headers_clear | 0.33 | 2 | **1.17** |
| headers_pass | 10 | 2.67 | **6.33** |
| headers_shot | 1.33 | 0.33 | **0.83** |
| jumps_gk | 6.67 | 35.67 | **21.17** |
| jumps_outfield | 31 | 21.67 | **26.33** |
| max_airborne_outfield | 1 | 1 | **1** |

### Team shape vs envelope (teams mean)

| metric | in possession | out of possession | 11v11 envelope (m) |
|---|---:|---:|---|
| length_m | 19.58 | 19.87 | 31-46 |
| width_m | 18.30 | 17.75 | 35-48 |
| stretch_m | 10.02 | 9.89 | 7-16 |
| nearest_mate_mean_m | 10.88 | 10.96 | none published (in-possession mean near 9-10 m, estimate) |
| nearest_mate_p10_m | 3.72 | 4.40 | none published (in-possession mean near 9-10 m, estimate) |

Note: 5v5 has no published team-shape envelope; the 11v11 ranges are printed for reference only and do not apply to a 4-outfielder team on a small pitch.

### Per seed (home / away)

| metric | seed 1 | seed 2 | seed 3 |
|---|---:|---:|---:|
| aerial.headers | 12 / 4 | 9 / 4 | 14 / 7 |
| aerial.headers_clear | 0 / 1 | 0 / 1 | 1 / 4 |
| aerial.headers_pass | 10 / 3 | 8 / 2 | 12 / 3 |
| aerial.headers_shot | 2 / 0 | 1 / 1 | 1 / 0 |
| aerial.jumps_gk | 6 / 35 | 8 / 38 | 6 / 34 |
| aerial.jumps_outfield | 34 / 23 | 26 / 16 | 33 / 26 |
| aerial.max_airborne_outfield | 1 / 1 | 1 / 1 | 1 / 1 |
| collision.any.depth_max_m | 0.79 / 0.79 | 0.79 / 0.79 | 0.77 / 0.77 |
| collision.any.depth_mean_m | 0.30 / 0.30 | 0.30 / 0.30 | 0.20 / 0.20 |
| collision.any.events_per_min | 2.70 / 2.70 | 1.60 / 1.60 | 1.60 / 1.60 |
| collision.any.stuck_mean_ticks | 14 / 14 | 13.50 / 13.50 | 11 / 11 |
| collision.any.stuck_pairs | 5 / 5 | 2 / 2 | 1 / 1 |
| collision.any.ticks_gt1cm_per_min | 8.20 / 8.20 | 4.40 / 4.40 | 2.40 / 2.40 |
| collision.any.ticks_per_min | 8.90 / 8.90 | 4.90 / 4.90 | 2.70 / 2.70 |
| collision.correction_vel_corrected_p50_mps | 0.83 / 0.86 | 0.44 / 0.57 | 0.46 / 0.48 |
| collision.correction_vel_corrected_p99_mps | 12.30 / 12.46 | 12.33 / 12.85 | 12.04 / 13.18 |
| collision.correction_vel_corrected_pct | 0.12 / 0.12 | 0.10 / 0.12 | 0.11 / 0.12 |
| collision.correction_vel_max_mps | 14.36 / 14.48 | 13.43 / 13.96 | 14.23 / 14.22 |
| collision.correction_vel_p50_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p95_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p99_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_pct_gt_0_3 | 0.10 / 0.09 | 0.07 / 0.09 | 0.08 / 0.08 |
| collision.opponent.depth_max_m | 0.09 / 0.09 | 0.08 / 0.08 | 0.06 / 0.06 |
| collision.opponent.depth_mean_m | 0.03 / 0.03 | 0.03 / 0.03 | 0.02 / 0.02 |
| collision.opponent.events_per_min | 1.30 / 1.30 | 0.90 / 0.90 | 1.10 / 1.10 |
| collision.opponent.stuck_mean_ticks | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.stuck_pairs | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.ticks_gt1cm_per_min | 1 / 1 | 0.90 / 0.90 | 1.10 / 1.10 |
| collision.opponent.ticks_per_min | 1.50 / 1.50 | 1.30 / 1.30 | 1.30 / 1.30 |
| collision.teammate.depth_max_m | 0.79 / 0.79 | 0.79 / 0.79 | 0.59 / 0.77 |
| collision.teammate.depth_mean_m | 0.34 / 0.34 | 0.37 / 0.35 | 0.26 / 0.34 |
| collision.teammate.events_per_min | 0.60 / 0.80 | 0.30 / 0.40 | 0.20 / 0.30 |
| collision.teammate.stuck_mean_ticks | 16 / 11 | 16 / 11 | 0 / 11 |
| collision.teammate.stuck_pairs | 3 / 2 | 1 / 1 | 0 / 1 |
| collision.teammate.ticks_gt1cm_per_min | 5.80 / 4.40 | 3 / 2.30 | 0.70 / 1.20 |
| collision.teammate.ticks_per_min | 6.10 / 4.40 | 3 / 2.50 | 0.70 / 1.30 |
| coord.cruise_over72_pct | 36.17 / 41.13 | 37.52 / 41.18 | 40.74 / 41.34 |
| coord.dual_chaser_pct_of_loose | 15.19 / 18.79 | 14.74 / 19.68 | 14.29 / 18.19 |
| coord.dual_chaser_ticks_per_min | 346.7 / 428.9 | 330.7 / 441.7 | 333.1 / 423.9 |
| coord.engagers3_pct_of_carrier_ticks | 0.04 / 0.22 | 0.10 / 0.14 | 0 / 0 |
| coord.engagers3_ticks_per_min | 0.20 / 1.90 | 0.50 / 1.20 | 0 / 0 |
| coord.mark_steal_per_defender_min | 0.20 / 0.21 | 0.56 / 0.21 | 0 / 0.22 |
| coord.max_simultaneous_chasers | 4 / 4 | 4 / 4 | 4 / 5 |
| gk.catches | 0 / 7 | 1 / 3 | 1 / 6 |
| gk.claims_non_save | 10 / 3 | 18 / 2 | 14 / 3 |
| gk.excursion_mean_s | 1.42 / 0.99 | 0.93 / 0.41 | 0.62 / 0.82 |
| gk.excursions_gt1s_per_10min | 1 / 1 | 3 / 1 | 1.00 / 1.00 |
| gk.gathers | 10 / 10 | 19 / 5 | 15 / 9 |
| gk.goals_conceded | 1 / 7 | 3 / 10 | 0 / 5 |
| gk.goals_conceded_gk_gt3m_off_line | 0 / 3 | 1 / 5 | 0 / 3 |
| gk.goals_conceded_no_shot_found | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.line_dist_at_conceded_shot_mean_m | 2.80 / 2.77 | 2.54 / 3.20 | 0 / 2.87 |
| gk.line_dist_max_m | 10.03 / 10.99 | 10.96 / 7.50 | 7.93 / 6.92 |
| gk.line_dist_mean_m | 3.06 / 2.88 | 3.08 / 2.87 | 3.08 / 2.82 |
| gk.parries | 2 / 20 | 3 / 31 | 2 / 28 |
| gk.pct_outside_6yd | 0.24 / 0.50 | 1.40 / 0.21 | 0.41 / 0.54 |
| gk.pct_outside_box | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_count | 1 / 3 | 9 / 3 | 4 / 4 |
| gk.rush_per_10min | 1 / 3 | 9 / 3 | 4.00 / 4.00 |
| gk.saves | 2 / 27 | 4 / 34 | 3 / 34 |
| gk.tips_over | 0 / 0 | 0 / 0 | 0 / 0 |
| match.fouls | 0 / 0 | 2 / 0 | 1 / 0 |
| match.goals | 7 / 1 | 10 / 3 | 5 / 0 |
| match.offsides | 3 / 0 | 1 / 0 | 2 / 0 |
| match.passes_completed_pct | 77.27 / 76.82 | 76.05 / 80.12 | 74.03 / 77.64 |
| match.possession_pct | 66.21 / 33.79 | 64.49 / 35.51 | 64.24 / 35.76 |
| match.shots | 35 / 4 | 47 / 10 | 43 / 4 |
| match.shots_on_target | 28 / 3 | 37 / 7 | 31 / 2 |
| move.accel_from_pos_mean_mps2 | 12.03 / 12.32 | 11.81 / 11.77 | 12.06 / 12.17 |
| move.accel_gt10_pct | 51.59 / 58.18 | 50.07 / 53.91 | 51.64 / 55.55 |
| move.accel_mean_mps2 | 12.00 / 12.22 | 11.77 / 11.66 | 12.04 / 12.01 |
| move.class_time_pct.0 | 17.28 / 11.05 | 17.11 / 11.35 | 17.26 / 11.66 |
| move.class_time_pct.1 | 6.21 / 9.26 | 6.62 / 8.84 | 6.54 / 8.82 |
| move.class_time_pct.2 | 62.54 / 67.83 | 61.62 / 66.80 | 63.43 / 67.68 |
| move.class_time_pct.3 | 13.97 / 11.86 | 14.65 / 13.01 | 12.77 / 11.84 |
| move.dspeed_max_mps_per_tick | 4.28 / 5.56 | 4.09 / 5.80 | 4.35 / 6.45 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.77 / 3.48 | 3.70 / 3.47 | 3.75 / 3.48 |
| move.heading_change_pct.0_10 | 99.06 / 99.33 | 99.16 / 99.33 | 99.17 / 99.27 |
| move.heading_change_pct.10_45 | 0.92 / 0.64 | 0.81 / 0.65 | 0.81 / 0.70 |
| move.heading_change_pct.45_90 | 0.03 / 0.03 | 0.03 / 0.02 | 0.02 / 0.03 |
| move.heading_change_pct.gt90 | 0.00 / 0.00 | 0 / 0.00 | 0.00 / 0.00 |
| move.heading_sign_flips_per_player_min | 30.50 / 17.58 | 28.42 / 17.85 | 27.78 / 19.08 |
| move.lateral_accel_mean_mps2 | 8.72 / 9.04 | 8.46 / 8.37 | 8.84 / 8.53 |
| move.reversals_per_player_min | 0.02 / 0.06 | 0 / 0.02 | 0.04 / 0.02 |
| move.side_flips_by_class.0_per_player_min | 308.9 / 115.4 | 301.2 / 121.1 | 301.8 / 122.2 |
| move.side_flips_by_class.0_share_pct | 69.33 / 34.12 | 68.93 / 36.45 | 70.08 / 36.38 |
| move.side_flips_by_class.1_per_player_min | 131.6 / 46.51 | 121.3 / 45.54 | 130.7 / 49.71 |
| move.side_flips_by_class.1_share_pct | 10.61 / 11.52 | 10.74 / 10.68 | 11.50 / 11.19 |
| move.side_flips_by_class.2_per_player_min | 18.86 / 24.98 | 19.66 / 24.47 | 18.02 / 25.02 |
| move.side_flips_by_class.2_share_pct | 15.32 / 45.31 | 16.21 / 43.36 | 15.38 / 43.23 |
| move.side_flips_by_class.3_per_player_min | 26.09 / 28.54 | 21.05 / 27.56 | 17.71 / 30.43 |
| move.side_flips_by_class.3_share_pct | 4.73 / 9.05 | 4.13 / 9.51 | 3.04 / 9.20 |
| move.side_flips_cause.desired_per_player_min | 61.82 / 14.36 | 61.23 / 14.48 | 60.91 / 15.43 |
| move.side_flips_cause.filter_per_player_min | 11.46 / 18.91 | 10.53 / 18.53 | 9.53 / 19.10 |
| move.side_flips_cause.limiter_per_player_min | 2.16 / 2.10 | 1.94 / 2.46 | 2.66 / 2.34 |
| move.side_flips_cause.sampler_per_player_min | 1.54 / 2.02 | 1.06 / 2.22 | 1.22 / 2.30 |
| move.side_flips_per_player_min | 76.98 / 37.39 | 74.77 / 37.69 | 74.32 / 39.17 |
| move.side_flips_schmitt_per_player_min | 39.67 / 38.93 | 36.63 / 39.06 | 37.57 / 40.73 |
| move.side_flips_speed_ge3_per_player_min | 107.4 / 45.58 | 102.6 / 46.40 | 101.9 / 48.21 |
| move.side_flips_speed_lt3_per_player_min | 13.10 / 15.42 | 11.59 / 14.82 | 10.43 / 15.55 |
| move.speed_oscillation_pct | 4.51 / 4.56 | 4.33 / 4.58 | 4.23 / 4.53 |
| move.turn_rate_mean_deg_per_s | 106.2 / 108.4 | 99.54 / 101.2 | 102.7 / 103.8 |
| shape.attacking.anchor_dist_m.defender | 9.45 / 7.88 | 9.46 / 9.33 | 10.42 / 9.66 |
| shape.attacking.anchor_dist_m.forward | 11.26 / 17.86 | 11.70 / 18.93 | 12.59 / 18.32 |
| shape.attacking.anchor_dist_m.goalkeeper | 1.16 / 1.53 | 1.30 / 1.51 | 1.20 / 1.64 |
| shape.attacking.anchor_dist_m.midfielder | 15.55 / 12.69 | 16.10 / 16.69 | 14.90 / 17.02 |
| shape.attacking.anchor_dist_shape_m.defender | 7.36 / 7.76 | 7.35 / 9.06 | 7.54 / 9.18 |
| shape.attacking.anchor_dist_shape_m.forward | 10.42 / 10.54 | 9.52 / 11.37 | 10.65 / 10.76 |
| shape.attacking.anchor_dist_shape_m.midfielder | 10.45 / 7.98 | 11.17 / 10.77 | 10.21 / 12.01 |
| shape.attacking.bbox_area_m2 | 433.4 / 321.9 | 438.6 / 302.3 | 451.8 / 298.9 |
| shape.attacking.bbox_depth_m | 22.54 / 17.43 | 22.54 / 16.43 | 21.96 / 16.57 |
| shape.attacking.bbox_width_m | 19.08 / 17.32 | 18.79 / 17.25 | 20.43 / 16.93 |
| shape.attacking.length_m | 22.54 / 17.43 | 22.54 / 16.43 | 21.96 / 16.57 |
| shape.attacking.nearest_mate_mean_m | 11.81 / 10.09 | 11.71 / 9.72 | 12.13 / 9.80 |
| shape.attacking.nearest_mate_p10_m | 3.79 / 3.60 | 3.76 / 3.50 | 4.16 / 3.51 |
| shape.attacking.pct_of_open_play | 61.46 / 38.54 | 58.67 / 41.33 | 58.50 / 41.50 |
| shape.attacking.stretch_m | 11.01 / 9.22 | 10.97 / 8.84 | 11.21 / 8.86 |
| shape.attacking.width_m | 19.08 / 17.32 | 18.79 / 17.25 | 20.43 / 16.93 |
| shape.defending.anchor_dist_m.defender | 6.91 / 9.10 | 7.41 / 9.41 | 8.30 / 9.69 |
| shape.defending.anchor_dist_m.forward | 11.09 / 16.76 | 12.24 / 17.41 | 12.89 / 16.68 |
| shape.defending.anchor_dist_m.goalkeeper | 1.19 / 1.67 | 1.26 / 1.61 | 1.28 / 1.67 |
| shape.defending.anchor_dist_m.midfielder | 13.83 / 12.31 | 15.23 / 14.14 | 14.26 / 16.09 |
| shape.defending.anchor_dist_shape_m.defender | 6.58 / 9.58 | 8.67 / 9.59 | 8.11 / 9.83 |
| shape.defending.anchor_dist_shape_m.forward | 9.40 / 10.94 | 9.62 / 11.48 | 9.81 / 10.72 |
| shape.defending.anchor_dist_shape_m.midfielder | 8.71 / 8.14 | 9.40 / 10.17 | 9.33 / 12.09 |
| shape.defending.bbox_area_m2 | 454.5 / 253.3 | 469.4 / 275.6 | 465.8 / 269.5 |
| shape.defending.bbox_depth_m | 24.92 / 14.80 | 24.13 / 15.44 | 24.45 / 15.50 |
| shape.defending.bbox_width_m | 18.19 / 16.64 | 19.08 / 17.11 | 19.18 / 16.32 |
| shape.defending.length_m | 24.92 / 14.80 | 24.13 / 15.44 | 24.45 / 15.50 |
| shape.defending.line_x_std_m | 8.16 / 3.93 | 7.90 / 4.20 | 8.07 / 4.00 |
| shape.defending.line_x_variance_m2 | 89.01 / 24.37 | 95.47 / 29.52 | 88.39 / 26.45 |
| shape.defending.nearest_mate_mean_m | 12.55 / 9.13 | 12.70 / 9.43 | 12.72 / 9.24 |
| shape.defending.nearest_mate_p10_m | 5.09 / 3.49 | 5.59 / 3.65 | 5.21 / 3.37 |
| shape.defending.pct_of_open_play | 38.54 / 61.46 | 41.33 / 58.67 | 41.50 / 58.50 |
| shape.defending.pct_outfield_behind_ball | 76.12 / 63.15 | 73.89 / 63.14 | 73.42 / 61.01 |
| shape.defending.stretch_m | 11.33 / 8.30 | 11.37 / 8.50 | 11.47 / 8.36 |
| shape.defending.swarm2_pct | 0.85 / 1.40 | 0.19 / 0.31 | 0.95 / 0.99 |
| shape.defending.swarm3_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| shape.defending.width_m | 18.19 / 16.64 | 19.08 / 17.11 | 19.18 / 16.32 |
| support.carrier_pct_of_open_play | 23.36 / 10.90 | 22.72 / 12.96 | 21.02 / 11.77 |
| support.cluster_pct | 6.29 / 6.22 | 6.31 / 7.18 | 6.75 / 9.32 |
| support.depth_m | 22.42 / 17.88 | 22.15 / 18.03 | 22.03 / 17.23 |
| support.forward_options_mean | 0.37 / 0.34 | 0.22 / 0.28 | 0.28 / 0.40 |
| support.nearest_option_dist_m | 13.60 / 13.11 | 13.25 / 13.13 | 13.41 / 12.50 |
| support.open_options_mean | 1.28 / 0.96 | 1.29 / 0.88 | 1.26 / 1.12 |
| support.open_options_p10 | 0 / 0 | 0 / 0 | 0 / 0 |
| support.pass_target_open_pct | 76 / 68.35 | 70.75 / 59.80 | 73.45 / 58.62 |
| support.passes_completed_tracked | 150 / 79 | 106 / 102 | 113 / 87 |
| support.release_pass_completed_pct | 83.33 / 0 | 50 / 50 | 75 / 100 |
| support.releases_per_10min | 12 / 1 | 4 / 2 | 4.00 / 2.00 |
| support.run_abandon_per_10min | 0 / 0 | 0 / 0 | 0 / 0 |
| support.runs_in_behind_per_10min | 7 / 2 | 10 / 7 | 10.99 / 2.00 |
| support.ticks_with_le1_option_pct | 55.99 / 67.57 | 53.93 / 71.92 | 56.56 / 60.54 |
| support.width_m | 17.46 / 15.23 | 17.37 / 16.39 | 18.82 / 16.01 |
| trans.count_per_10min | 29 / 49 | 33 / 50 | 26.98 / 59.96 |
| trans.counterpress_pct | 68.97 / 26.53 | 69.70 / 42 | 77.78 / 33.33 |
| trans.forward_recovery_speed_mps_0_5s | 6.85 / 6.36 | 6.88 / 6.54 | 6.84 / 6.33 |
| trans.forward_sprint_share_0_5s | 67.94 / 58.66 | 68.09 / 61.32 | 69.85 / 59.90 |
| trans.forward_sprint_share_0_5s_7mps | 53.41 / 45.47 | 59.10 / 48.69 | 54.57 / 44.96 |
| trans.forwards_staying_high_mean | 0.40 / 0.79 | 0.80 / 0.27 | 0 / 0.72 |
| trans.in_shape_at_turnover_pct | 52.38 / 43.90 | 79.17 / 47.22 | 75 / 42.22 |
| trans.mid_recovery_speed_mps_0_5s | 6.29 / 6.32 | 6.71 / 6.51 | 6.92 / 6.31 |
| trans.mid_sprint_share_0_5s | 59.23 / 61.27 | 66.65 / 61.99 | 70.62 / 59.53 |
| trans.mid_sprint_share_0_5s_7mps | 48.17 / 47.41 | 53.99 / 49.74 | 58.85 / 44.23 |
| trans.outfield_stamina_end_permille | 816.2 / 808.8 | 792.2 / 796.5 | 774.5 / 747.8 |
| trans.pct_outfield_behind_ball_at_3s | 55.77 / 60.16 | 67.31 / 48.21 | 61.36 / 60.42 |
| trans.pct_outfield_behind_ball_at_6s | 75 / 64.47 | 60 / 84.09 | 100 / 50 |
| trans.time_to_shape_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_excluded_pct | 27.59 / 16.33 | 27.27 / 28 | 25.93 / 25 |
| trans.time_to_shape_s | 0.54 / 1.01 | 0.35 / 1.40 | 0.44 / 1.20 |
| trans.time_to_shape_static_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_static_excluded_pct | 27.59 / 40.82 | 24.24 / 42 | 22.22 / 43.33 |
| trans.time_to_shape_static_s | 0.31 / 0.44 | 0.40 / 0.54 | 0.29 / 0.58 |

GK scenario field: not present in the public state.

## 11v11 autopilot-normal vs normal (10 min x 3 seeds from 1)

Command: `psfc-replay analyze --format 11v11 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty normal --away-difficulty normal --home-controller autopilot`

Gameplay version: `0.1.0-5v5`

| seed | final state_hash | score (H-A) |
|---|---|---|
| 1 | `b3ef5e592c62f714` | 13-1 |
| 2 | `7546b85582cf8251` | 2-1 |
| 3 | `e011fc00f8befc1c` | 6-1 |

Match meta (mean over seeds): goals_total = 8, open_play_minutes = 10, open_play_pct_of_ticks = 94.78, shots_total = 34.67, ticks_simulated = 37990.7

### Match

| metric | home | away | teams mean |
|---|---:|---:|---:|
| fouls | 0 | 0 | **0** |
| goals | 7 | 1 | **4** |
| offsides | 1.67 | 0.67 | **1.17** |
| passes_completed_pct | 74.12 | 75.23 | **74.68** |
| possession_pct | 64.24 | 35.76 | **50** |
| shots | 29.67 | 5 | **17.33** |
| shots_on_target | 20.33 | 3.33 | **11.83** |

### Goalkeeper

| metric | home | away | teams mean |
|---|---:|---:|---:|
| catches | 0 | 1.33 | **0.67** |
| claims_non_save | 4.33 | 2 | **3.17** |
| excursion_mean_s | 1.14 | 0.83 | **0.98** |
| excursions_gt1s_per_10min | 2.67 | 3.67 | **3.17** |
| gathers | 4.33 | 3.33 | **3.83** |
| goals_conceded | 1 | 7 | **4** |
| goals_conceded_gk_gt3m_off_line | 0.33 | 5.33 | **2.83** |
| goals_conceded_no_shot_found | 0 | 0 | **0** |
| line_dist_at_conceded_shot_mean_m | 2.38 | 3.66 | **3.02** |
| line_dist_max_m | 8.68 | 8.23 | **8.45** |
| line_dist_mean_m | 4.60 | 4.30 | **4.45** |
| parries | 2 | 13 | **7.50** |
| pct_outside_6yd | 0.73 | 1.29 | **1.01** |
| pct_outside_box | 0 | 0 | **0** |
| rush_count | 3.67 | 9 | **6.33** |
| rush_per_10min | 3.67 | 9 | **6.33** |
| saves | 2 | 14.33 | **8.17** |
| tips_over | 0 | 0 | **0** |

### Defence and shape

| metric | home | away | teams mean |
|---|---:|---:|---:|
| attacking.anchor_dist_m.defender | 23.51 | 16.60 | **20.05** |
| attacking.anchor_dist_m.forward | 22.89 | 24.90 | **23.90** |
| attacking.anchor_dist_m.goalkeeper | 2.32 | 2.34 | **2.33** |
| attacking.anchor_dist_m.midfielder | 17.88 | 17.80 | **17.84** |
| attacking.anchor_dist_shape_m.defender | 17.55 | 15.27 | **16.41** |
| attacking.anchor_dist_shape_m.forward | 19.03 | 17.16 | **18.10** |
| attacking.anchor_dist_shape_m.midfielder | 11.45 | 10.09 | **10.77** |
| attacking.bbox_area_m2 | 2120.7 | 1724.2 | **1922.5** |
| attacking.bbox_depth_m | 45.81 | 39.33 | **42.57** |
| attacking.bbox_width_m | 45.42 | 42.42 | **43.92** |
| attacking.length_m | 45.81 | 39.33 | **42.57** |
| attacking.nearest_mate_mean_m | 12.46 | 11.03 | **11.75** |
| attacking.nearest_mate_p10_m | 6.27 | 4.71 | **5.49** |
| attacking.pct_of_open_play | 61.20 | 38.80 | **50** |
| attacking.stretch_m | 20.55 | 18.18 | **19.37** |
| attacking.width_m | 45.42 | 42.42 | **43.92** |
| defending.anchor_dist_m.defender | 21.65 | 17.24 | **19.44** |
| defending.anchor_dist_m.forward | 24.17 | 26.87 | **25.52** |
| defending.anchor_dist_m.goalkeeper | 2.46 | 2.63 | **2.54** |
| defending.anchor_dist_m.midfielder | 17.44 | 18.04 | **17.74** |
| defending.anchor_dist_shape_m.defender | 17.11 | 16.35 | **16.73** |
| defending.anchor_dist_shape_m.forward | 19.32 | 19.54 | **19.43** |
| defending.anchor_dist_shape_m.midfielder | 10.65 | 10.69 | **10.67** |
| defending.bbox_area_m2 | 1741.1 | 1387.8 | **1564.5** |
| defending.bbox_depth_m | 42.13 | 36.10 | **39.11** |
| defending.bbox_width_m | 39.92 | 37.03 | **38.47** |
| defending.length_m | 42.13 | 36.10 | **39.11** |
| defending.line_x_std_m | 7.54 | 4.55 | **6.04** |
| defending.line_x_variance_m2 | 91.05 | 38.12 | **64.59** |
| defending.nearest_mate_mean_m | 11.13 | 9.82 | **10.48** |
| defending.nearest_mate_p10_m | 5.84 | 4.50 | **5.17** |
| defending.pct_of_open_play | 38.80 | 61.20 | **50** |
| defending.pct_outfield_behind_ball | 77.86 | 68.06 | **72.96** |
| defending.stretch_m | 18.64 | 15.77 | **17.21** |
| defending.swarm2_pct | 0.57 | 0.36 | **0.46** |
| defending.swarm3_pct | 0 | 0 | **0** |
| defending.width_m | 39.92 | 37.03 | **38.47** |

### Defensive transition

| metric | home | away | teams mean |
|---|---:|---:|---:|
| count_per_10min | 20.33 | 42 | **31.17** |
| counterpress_pct | 67.35 | 42.73 | **55.04** |
| forward_recovery_speed_mps_0_5s | 7.25 | 7.01 | **7.13** |
| forward_sprint_share_0_5s | 71.92 | 67.63 | **69.78** |
| forward_sprint_share_0_5s_7mps | 60.65 | 56.30 | **58.48** |
| forwards_staying_high_mean | 1.89 | 1.61 | **1.75** |
| in_shape_at_turnover_pct | 67.52 | 54.45 | **60.98** |
| mid_recovery_speed_mps_0_5s | 6.68 | 6.50 | **6.59** |
| mid_sprint_share_0_5s | 64.69 | 60.13 | **62.41** |
| mid_sprint_share_0_5s_7mps | 52.06 | 47.27 | **49.67** |
| outfield_stamina_end_permille | 909.8 | 882.3 | **896.1** |
| pct_outfield_behind_ball_at_3s | 72.74 | 57.58 | **65.16** |
| pct_outfield_behind_ball_at_6s | 63.06 | 67.89 | **65.47** |
| time_to_shape_capped_pct | 0 | 0 | **0** |
| time_to_shape_excluded_pct | 25.98 | 31.10 | **28.54** |
| time_to_shape_s | 0.97 | 1.21 | **1.09** |
| time_to_shape_static_capped_pct | 0 | 0 | **0** |
| time_to_shape_static_excluded_pct | 30.38 | 44.40 | **37.39** |
| time_to_shape_static_s | 0.78 | 0.71 | **0.75** |

### Attacking support

| metric | home | away | teams mean |
|---|---:|---:|---:|
| carrier_pct_of_open_play | 19.02 | 11.05 | **15.04** |
| cluster_pct | 3.28 | 9.17 | **6.23** |
| depth_m | 43.75 | 37.68 | **40.72** |
| forward_options_mean | 0.63 | 0.61 | **0.62** |
| nearest_option_dist_m | 13.67 | 11.81 | **12.74** |
| open_options_mean | 2.96 | 3.00 | **2.98** |
| open_options_p10 | 0 | 0 | **0** |
| pass_target_open_pct | 68.27 | 53.31 | **60.79** |
| passes_completed_tracked | 115.3 | 88.33 | **101.8** |
| release_pass_completed_pct | 37.50 | 16.67 | **27.08** |
| releases_per_10min | 1.67 | 1.33 | **1.50** |
| run_abandon_per_10min | 0 | 0 | **0** |
| runs_in_behind_per_10min | 6 | 1.33 | **3.67** |
| ticks_with_le1_option_pct | 30.61 | 36.71 | **33.66** |
| width_m | 43.95 | 42.56 | **43.25** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 10.24 | 10.52 | **10.38** |
| accel_gt10_pct | 48.18 | 51.57 | **49.88** |
| accel_mean_mps2 | 10.24 | 10.47 | **10.35** |
| class_time_pct.0 | 7.95 | 5.62 | **6.79** |
| class_time_pct.1 | 2.83 | 3.54 | **3.19** |
| class_time_pct.2 | 66.91 | 76.26 | **71.58** |
| class_time_pct.3 | 22.31 | 14.58 | **18.44** |
| dspeed_max_mps_per_tick | 4.01 | 6.13 | **5.07** |
| heading_change_p95_deg_per_tick_gt3mps | 3.24 | 3.19 | **3.21** |
| heading_change_pct.0_10 | 99.63 | 99.67 | **99.65** |
| heading_change_pct.10_45 | 0.36 | 0.32 | **0.34** |
| heading_change_pct.45_90 | 0.01 | 0.01 | **0.01** |
| heading_change_pct.gt90 | 0.00 | 0.00 | **0.00** |
| heading_sign_flips_per_player_min | 15.95 | 10.50 | **13.22** |
| lateral_accel_mean_mps2 | 7.50 | 7.76 | **7.63** |
| reversals_per_player_min | 0.01 | 0.02 | **0.01** |
| side_flips_by_class.0_per_player_min | 292.9 | 101.4 | **197.1** |
| side_flips_by_class.0_share_pct | 57.39 | 24.08 | **40.74** |
| side_flips_by_class.1_per_player_min | 118.2 | 47.56 | **82.89** |
| side_flips_by_class.1_share_pct | 8.22 | 7.08 | **7.65** |
| side_flips_by_class.2_per_player_min | 15.13 | 17.53 | **16.33** |
| side_flips_by_class.2_share_pct | 24.94 | 56.20 | **40.57** |
| side_flips_by_class.3_per_player_min | 17.24 | 20.72 | **18.98** |
| side_flips_by_class.3_share_pct | 9.44 | 12.64 | **11.04** |
| side_flips_cause.desired_per_player_min | 27.74 | 6.59 | **17.16** |
| side_flips_cause.filter_per_player_min | 10.86 | 14.77 | **12.82** |
| side_flips_cause.limiter_per_player_min | 1.20 | 1.29 | **1.24** |
| side_flips_cause.sampler_per_player_min | 0.78 | 1.13 | **0.95** |
| side_flips_per_player_min | 40.57 | 23.77 | **32.17** |
| side_flips_schmitt_per_player_min | 33.10 | 34.75 | **33.93** |
| side_flips_speed_ge3_per_player_min | 48.74 | 26.41 | **37.58** |
| side_flips_speed_lt3_per_player_min | 11.95 | 13.20 | **12.58** |
| speed_oscillation_pct | 3.18 | 3.37 | **3.27** |
| turn_rate_mean_deg_per_s | 86.39 | 90.17 | **88.28** |

### Collisions and push-out

| metric | home | away | teams mean |
|---|---:|---:|---:|
| any.depth_max_m | 0.80 | 0.80 | **0.80** |
| any.depth_mean_m | 0.26 | 0.26 | **0.26** |
| any.events_per_min | 2.80 | 2.80 | **2.80** |
| any.stuck_mean_ticks | 8.67 | 8.67 | **8.67** |
| any.stuck_pairs | 1 | 1 | **1** |
| any.ticks_gt1cm_per_min | 7.63 | 7.63 | **7.63** |
| any.ticks_per_min | 8.70 | 8.70 | **8.70** |
| correction_vel_corrected_p50_mps | 0.63 | 1.01 | **0.82** |
| correction_vel_corrected_p99_mps | 6.21 | 12.46 | **9.33** |
| correction_vel_corrected_pct | 0.05 | 0.07 | **0.06** |
| correction_vel_max_mps | 9.47 | 13.61 | **11.54** |
| correction_vel_p50_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p95_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p99_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_pct_gt_0_3 | 0.04 | 0.05 | **0.04** |
| opponent.depth_max_m | 0.34 | 0.34 | **0.34** |
| opponent.depth_mean_m | 0.07 | 0.07 | **0.07** |
| opponent.events_per_min | 1.83 | 1.83 | **1.83** |
| opponent.stuck_mean_ticks | 0 | 0 | **0** |
| opponent.stuck_pairs | 0 | 0 | **0** |
| opponent.ticks_gt1cm_per_min | 2.17 | 2.17 | **2.17** |
| opponent.ticks_per_min | 2.90 | 2.90 | **2.90** |
| teammate.depth_max_m | 0.78 | 0.80 | **0.79** |
| teammate.depth_mean_m | 0.36 | 0.36 | **0.36** |
| teammate.events_per_min | 0.23 | 0.73 | **0.48** |
| teammate.stuck_mean_ticks | 5 | 8 | **6.50** |
| teammate.stuck_pairs | 0.33 | 0.67 | **0.50** |
| teammate.ticks_gt1cm_per_min | 1.47 | 4.63 | **3.05** |
| teammate.ticks_per_min | 1.53 | 4.93 | **3.23** |

### Coordination

| metric | home | away | teams mean |
|---|---:|---:|---:|
| cruise_over72_pct | 42.65 | 42.79 | **42.72** |
| dual_chaser_pct_of_loose | 13.63 | 17.11 | **15.37** |
| dual_chaser_ticks_per_min | 335.2 | 420.9 | **378.1** |
| engagers3_pct_of_carrier_ticks | 0.28 | 0.09 | **0.18** |
| engagers3_ticks_per_min | 1.30 | 0.70 | **1** |
| mark_steal_per_defender_min | 0.15 | 0.33 | **0.24** |
| max_simultaneous_chasers | 4.33 | 5 | **4.67** |

### Aerial

| metric | home | away | teams mean |
|---|---:|---:|---:|
| headers | 21.67 | 15 | **18.33** |
| headers_clear | 1 | 2.33 | **1.67** |
| headers_pass | 17.33 | 11.67 | **14.50** |
| headers_shot | 3.33 | 1 | **2.17** |
| jumps_gk | 4 | 20 | **12** |
| jumps_outfield | 48.33 | 31.67 | **40** |
| max_airborne_outfield | 1 | 1 | **1** |

### Team shape vs envelope (teams mean)

| metric | in possession | out of possession | 11v11 envelope (m) |
|---|---:|---:|---|
| length_m | 42.57 | 39.11 | 31-46 |
| width_m | 43.92 | 38.47 | 35-48 |
| stretch_m | 19.37 (outside) | 17.21 (outside) | 7-16 |
| nearest_mate_mean_m | 11.75 | 10.48 | none published (in-possession mean near 9-10 m, estimate) |
| nearest_mate_p10_m | 5.49 | 5.17 | none published (in-possession mean near 9-10 m, estimate) |

Envelope: peer-reviewed 11v11 ranges (length 31-46 m, width 35-48 m, stretch index 7-16 m) quoted in the research report; the report's tighter targets are 31-36 / 35-40 / 7-10 m out of possession and 40-46 / 42-48 / 12-16 m in possession (estimates).

### Per seed (home / away)

| metric | seed 1 | seed 2 | seed 3 |
|---|---:|---:|---:|
| aerial.headers | 25 / 14 | 22 / 19 | 18 / 12 |
| aerial.headers_clear | 1 / 2 | 0 / 5 | 2 / 0 |
| aerial.headers_pass | 23 / 11 | 17 / 13 | 12 / 11 |
| aerial.headers_shot | 1 / 1 | 5 / 1 | 4 / 1 |
| aerial.jumps_gk | 4 / 27 | 5 / 12 | 3 / 21 |
| aerial.jumps_outfield | 49 / 28 | 53 / 38 | 43 / 29 |
| aerial.max_airborne_outfield | 1 / 1 | 1 / 1 | 1 / 1 |
| collision.any.depth_max_m | 0.80 / 0.80 | 0.80 / 0.80 | 0.80 / 0.80 |
| collision.any.depth_mean_m | 0.25 / 0.25 | 0.24 / 0.24 | 0.29 / 0.29 |
| collision.any.events_per_min | 2.30 / 2.30 | 3.30 / 3.30 | 2.80 / 2.80 |
| collision.any.stuck_mean_ticks | 0 / 0 | 13 / 13 | 13 / 13 |
| collision.any.stuck_pairs | 0 / 0 | 1 / 1 | 2 / 2 |
| collision.any.ticks_gt1cm_per_min | 6.20 / 6.20 | 7.50 / 7.50 | 9.20 / 9.20 |
| collision.any.ticks_per_min | 7.30 / 7.30 | 8.40 / 8.40 | 10.40 / 10.40 |
| collision.correction_vel_corrected_p50_mps | 0.87 / 0.45 | 0.57 / 0.77 | 0.45 / 1.80 |
| collision.correction_vel_corrected_p99_mps | 10.95 / 12.16 | 3.60 / 12.51 | 4.07 / 12.71 |
| collision.correction_vel_corrected_pct | 0.06 / 0.07 | 0.05 / 0.07 | 0.04 / 0.07 |
| collision.correction_vel_max_mps | 13.18 / 14.30 | 3.60 / 13.28 | 11.64 / 13.25 |
| collision.correction_vel_p50_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p95_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p99_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_pct_gt_0_3 | 0.04 / 0.04 | 0.03 / 0.05 | 0.03 / 0.05 |
| collision.opponent.depth_max_m | 0.51 / 0.51 | 0.39 / 0.39 | 0.12 / 0.12 |
| collision.opponent.depth_mean_m | 0.09 / 0.09 | 0.08 / 0.08 | 0.04 / 0.04 |
| collision.opponent.events_per_min | 1.70 / 1.70 | 2.30 / 2.30 | 1.50 / 1.50 |
| collision.opponent.stuck_mean_ticks | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.stuck_pairs | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.ticks_gt1cm_per_min | 2.10 / 2.10 | 2.80 / 2.80 | 1.60 / 1.60 |
| collision.opponent.ticks_per_min | 2.90 / 2.90 | 3.50 / 3.50 | 2.30 / 2.30 |
| collision.teammate.depth_max_m | 0.80 / 0.80 | 0.80 / 0.80 | 0.74 / 0.80 |
| collision.teammate.depth_mean_m | 0.39 / 0.36 | 0.38 / 0.35 | 0.31 / 0.37 |
| collision.teammate.events_per_min | 0.10 / 0.50 | 0.20 / 0.80 | 0.40 / 0.90 |
| collision.teammate.stuck_mean_ticks | 0 / 0 | 0 / 13 | 15 / 11 |
| collision.teammate.stuck_pairs | 0 / 0 | 0 / 1 | 1 / 1 |
| collision.teammate.ticks_gt1cm_per_min | 0.80 / 3.30 | 1.40 / 3.90 | 2.20 / 6.70 |
| collision.teammate.ticks_per_min | 0.80 / 3.60 | 1.40 / 4.20 | 2.40 / 7 |
| coord.cruise_over72_pct | 40.71 / 42.67 | 41.63 / 40.90 | 45.60 / 44.81 |
| coord.dual_chaser_pct_of_loose | 13.74 / 16.56 | 12.91 / 16.14 | 14.24 / 18.63 |
| coord.dual_chaser_ticks_per_min | 322.9 / 389.1 | 333.9 / 417.3 | 348.8 / 456.3 |
| coord.engagers3_pct_of_carrier_ticks | 0 / 0.23 | 0 / 0 | 0.85 / 0.03 |
| coord.engagers3_ticks_per_min | 0 / 1.90 | 0 / 0 | 3.90 / 0.20 |
| coord.mark_steal_per_defender_min | 0.17 / 0.09 | 0.22 / 0.74 | 0.08 / 0.16 |
| coord.max_simultaneous_chasers | 4 / 5 | 4 / 5 | 5 / 5 |
| gk.catches | 0 / 0 | 0 / 2 | 0 / 2 |
| gk.claims_non_save | 3 / 0 | 7 / 3 | 3 / 3 |
| gk.excursion_mean_s | 0.61 / 0.69 | 1.47 / 0.87 | 1.34 / 0.94 |
| gk.excursions_gt1s_per_10min | 1 / 2 | 4 / 6 | 3 / 3 |
| gk.gathers | 3 / 0 | 7 / 5 | 3 / 5 |
| gk.goals_conceded | 1 / 13 | 1 / 2 | 1 / 6 |
| gk.goals_conceded_gk_gt3m_off_line | 1 / 11 | 0 / 1 | 0 / 4 |
| gk.goals_conceded_no_shot_found | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.line_dist_at_conceded_shot_mean_m | 3.72 / 4.26 | 1.56 / 3.20 | 1.87 / 3.51 |
| gk.line_dist_max_m | 6.89 / 7.41 | 11.00 / 8.91 | 8.14 / 8.36 |
| gk.line_dist_mean_m | 4.61 / 4.48 | 4.60 / 4.29 | 4.57 / 4.13 |
| gk.parries | 2 / 12 | 2 / 11 | 2 / 16 |
| gk.pct_outside_6yd | 0.30 / 0.91 | 1.23 / 1.88 | 0.67 / 1.09 |
| gk.pct_outside_box | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_count | 3 / 7 | 5 / 13 | 3 / 7 |
| gk.rush_per_10min | 3 / 7 | 5 / 13 | 3 / 7 |
| gk.saves | 2 / 12 | 2 / 13 | 2 / 18 |
| gk.tips_over | 0 / 0 | 0 / 0 | 0 / 0 |
| match.fouls | 0 / 0 | 0 / 0 | 0 / 0 |
| match.goals | 13 / 1 | 2 / 1 | 6 / 1 |
| match.offsides | 2 / 2 | 1 / 0 | 2 / 0 |
| match.passes_completed_pct | 74.82 / 75.84 | 74.76 / 72.41 | 72.79 / 77.44 |
| match.possession_pct | 65.41 / 34.59 | 67.32 / 32.68 | 59.99 / 40.01 |
| match.shots | 30 / 5 | 24 / 5 | 35 / 5 |
| match.shots_on_target | 24 / 3 | 13 / 3 | 24 / 4 |
| move.accel_from_pos_mean_mps2 | 10.34 / 10.28 | 10.41 / 10.86 | 9.99 / 10.41 |
| move.accel_gt10_pct | 48.96 / 50.18 | 49.22 / 54.09 | 46.38 / 50.44 |
| move.accel_mean_mps2 | 10.32 / 10.24 | 10.40 / 10.80 | 9.98 / 10.36 |
| move.class_time_pct.0 | 7.90 / 5.68 | 8.13 / 5.46 | 7.83 / 5.73 |
| move.class_time_pct.1 | 3.05 / 3.53 | 2.51 / 3.69 | 2.94 / 3.41 |
| move.class_time_pct.2 | 66.62 / 74.56 | 65.29 / 77.15 | 68.80 / 77.07 |
| move.class_time_pct.3 | 22.43 / 16.23 | 24.07 / 13.70 | 20.43 / 13.79 |
| move.dspeed_max_mps_per_tick | 3.95 / 6.71 | 4.02 / 6.72 | 4.07 / 4.95 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.26 / 3.19 | 3.28 / 3.24 | 3.16 / 3.16 |
| move.heading_change_pct.0_10 | 99.65 / 99.68 | 99.59 / 99.66 | 99.65 / 99.66 |
| move.heading_change_pct.10_45 | 0.34 / 0.31 | 0.39 / 0.33 | 0.34 / 0.32 |
| move.heading_change_pct.45_90 | 0.01 / 0.01 | 0.02 / 0.02 | 0.01 / 0.01 |
| move.heading_change_pct.gt90 | 0.00 / 0.00 | 0.00 / 0.00 | 0 / 0.00 |
| move.heading_sign_flips_per_player_min | 16.40 / 10.58 | 16.32 / 10.91 | 15.12 / 10.01 |
| move.lateral_accel_mean_mps2 | 7.62 / 7.66 | 7.56 / 8.10 | 7.32 / 7.53 |
| move.reversals_per_player_min | 0.01 / 0.01 | 0.01 / 0.04 | 0 / 0.02 |
| move.side_flips_by_class.0_per_player_min | 302.2 / 106.7 | 288.9 / 95.16 | 287.5 / 102.3 |
| move.side_flips_by_class.0_share_pct | 58.32 / 26.12 | 57.73 / 20.82 | 56.13 / 25.29 |
| move.side_flips_by_class.1_per_player_min | 105.9 / 41.23 | 124.6 / 55.00 | 124.2 / 46.46 |
| move.side_flips_by_class.1_share_pct | 7.90 / 6.27 | 7.67 / 8.13 | 9.10 / 6.83 |
| move.side_flips_by_class.2_per_player_min | 14.86 / 16.91 | 15.73 / 18.88 | 14.80 / 16.80 |
| move.side_flips_by_class.2_share_pct | 24.20 / 54.31 | 25.23 / 58.39 | 25.40 / 55.89 |
| move.side_flips_by_class.3_per_player_min | 17.49 / 19.00 | 15.84 / 23.03 | 18.40 / 20.13 |
| move.side_flips_by_class.3_share_pct | 9.59 / 13.29 | 9.37 / 12.65 | 9.37 / 11.98 |
| move.side_flips_cause.desired_per_player_min | 28.20 / 6.66 | 27.54 / 6.22 | 27.49 / 6.87 |
| move.side_flips_cause.filter_per_player_min | 10.97 / 14.41 | 10.97 / 16.02 | 10.63 / 13.90 |
| move.side_flips_cause.limiter_per_player_min | 1.14 / 1.13 | 1.30 / 1.56 | 1.15 / 1.17 |
| move.side_flips_cause.sampler_per_player_min | 0.61 / 1.01 | 0.89 / 1.14 | 0.84 / 1.23 |
| move.side_flips_per_player_min | 40.92 / 23.21 | 40.70 / 24.94 | 40.10 / 23.17 |
| move.side_flips_schmitt_per_player_min | 33.19 / 33.47 | 33.80 / 36.04 | 32.31 / 34.74 |
| move.side_flips_speed_ge3_per_player_min | 49.27 / 26.48 | 49.12 / 27.26 | 47.82 / 25.51 |
| move.side_flips_speed_lt3_per_player_min | 12.17 / 11.24 | 12.47 / 15.16 | 11.22 / 13.19 |
| move.speed_oscillation_pct | 3.22 / 3.34 | 3.21 / 3.43 | 3.09 / 3.33 |
| move.turn_rate_mean_deg_per_s | 87.71 / 88.69 | 89.60 / 95.05 | 81.87 / 86.76 |
| shape.attacking.anchor_dist_m.defender | 19.06 / 15.80 | 27.05 / 16.41 | 24.41 / 17.60 |
| shape.attacking.anchor_dist_m.forward | 21.80 / 23.05 | 24.37 / 26.67 | 22.50 / 24.99 |
| shape.attacking.anchor_dist_m.goalkeeper | 2.33 / 2.30 | 2.30 / 2.47 | 2.33 / 2.24 |
| shape.attacking.anchor_dist_m.midfielder | 17.33 / 14.54 | 17.91 / 18.61 | 18.39 / 20.24 |
| shape.attacking.anchor_dist_shape_m.defender | 14.37 / 14.18 | 19.76 / 15.28 | 18.50 / 16.33 |
| shape.attacking.anchor_dist_shape_m.forward | 17.72 / 17.44 | 20.33 / 17.28 | 19.05 / 16.76 |
| shape.attacking.anchor_dist_shape_m.midfielder | 11.23 / 7.98 | 11.37 / 9.89 | 11.76 / 12.40 |
| shape.attacking.bbox_area_m2 | 2082.4 / 1745.3 | 2228.3 / 1651.7 | 2051.5 / 1775.7 |
| shape.attacking.bbox_depth_m | 44.48 / 39.10 | 47.30 / 38.54 | 45.64 / 40.34 |
| shape.attacking.bbox_width_m | 45.60 / 43.26 | 46.37 / 41.35 | 44.31 / 42.64 |
| shape.attacking.length_m | 44.48 / 39.10 | 47.30 / 38.54 | 45.64 / 40.34 |
| shape.attacking.nearest_mate_mean_m | 12.58 / 11.26 | 12.75 / 10.64 | 12.07 / 11.19 |
| shape.attacking.nearest_mate_p10_m | 6.44 / 4.79 | 6.32 / 4.72 | 6.04 / 4.61 |
| shape.attacking.pct_of_open_play | 58.18 / 41.82 | 67.37 / 32.63 | 58.05 / 41.95 |
| shape.attacking.stretch_m | 20.21 / 18.38 | 21.19 / 17.69 | 20.25 / 18.49 |
| shape.attacking.width_m | 45.60 / 43.26 | 46.37 / 41.35 | 44.31 / 42.64 |
| shape.defending.anchor_dist_m.defender | 16.72 / 17.70 | 24.61 / 16.37 | 23.62 / 17.63 |
| shape.defending.anchor_dist_m.forward | 24.11 / 25.66 | 23.61 / 28.26 | 24.80 / 26.70 |
| shape.defending.anchor_dist_m.goalkeeper | 2.45 / 2.52 | 2.44 / 2.70 | 2.48 / 2.67 |
| shape.defending.anchor_dist_m.midfielder | 16.43 / 16.48 | 17.44 / 17.79 | 18.45 / 19.85 |
| shape.defending.anchor_dist_shape_m.defender | 15.11 / 15.57 | 17.50 / 16.56 | 18.72 / 16.92 |
| shape.defending.anchor_dist_shape_m.forward | 19.01 / 19.60 | 18.47 / 19.63 | 20.47 / 19.38 |
| shape.defending.anchor_dist_shape_m.midfielder | 10.56 / 9.31 | 10.63 / 10.41 | 10.75 / 12.36 |
| shape.defending.bbox_area_m2 | 1488.5 / 1440.9 | 1932.5 / 1292.3 | 1802.3 / 1430.3 |
| shape.defending.bbox_depth_m | 37.81 / 37.20 | 44.76 / 34.35 | 43.81 / 36.75 |
| shape.defending.bbox_width_m | 37.85 / 37.37 | 42.06 / 36.28 | 39.85 / 37.43 |
| shape.defending.length_m | 37.81 / 37.20 | 44.76 / 34.35 | 43.81 / 36.75 |
| shape.defending.line_x_std_m | 6.11 / 4.82 | 8.79 / 4.14 | 7.72 / 4.68 |
| shape.defending.line_x_variance_m2 | 71.13 / 41.06 | 105.7 / 34.40 | 96.30 / 38.90 |
| shape.defending.nearest_mate_mean_m | 10.40 / 10.19 | 11.65 / 9.38 | 11.32 / 9.91 |
| shape.defending.nearest_mate_p10_m | 5.64 / 4.69 | 5.94 / 4.49 | 5.93 / 4.31 |
| shape.defending.pct_of_open_play | 41.82 / 58.18 | 32.63 / 67.37 | 41.95 / 58.05 |
| shape.defending.pct_outfield_behind_ball | 79.77 / 67.24 | 75.12 / 72.90 | 78.67 / 64.05 |
| shape.defending.stretch_m | 17.16 / 16.04 | 19.75 / 15.21 | 19.03 / 16.08 |
| shape.defending.swarm2_pct | 0 / 0.21 | 0.03 / 0.79 | 1.67 / 0.07 |
| shape.defending.swarm3_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| shape.defending.width_m | 37.85 / 37.37 | 42.06 / 36.28 | 39.85 / 37.43 |
| support.carrier_pct_of_open_play | 19.38 / 12.01 | 18.70 / 8.78 | 18.99 / 12.37 |
| support.cluster_pct | 3.00 / 8.65 | 4.59 / 6.27 | 2.25 / 12.60 |
| support.depth_m | 41.33 / 37.80 | 46.05 / 37.09 | 43.87 / 38.16 |
| support.forward_options_mean | 0.46 / 0.52 | 0.82 / 0.66 | 0.61 / 0.67 |
| support.nearest_option_dist_m | 14.21 / 12.47 | 12.85 / 11.54 | 13.97 / 11.43 |
| support.open_options_mean | 3.16 / 3.09 | 3.06 / 2.94 | 2.67 / 2.96 |
| support.open_options_p10 | 0 / 0 | 0 / 0 | 0 / 0 |
| support.pass_target_open_pct | 63.54 / 52.94 | 69.54 / 50.75 | 71.72 / 56.25 |
| support.passes_completed_tracked | 96 / 102 | 151 / 67 | 99 / 96 |
| support.release_pass_completed_pct | 0 / 0 | 75 / 0 | NaN / 50 |
| support.releases_per_10min | 1 / 1 | 4 / 1 | 0 / 2 |
| support.run_abandon_per_10min | 0 / 0 | 0 / 0 | 0 / 0 |
| support.runs_in_behind_per_10min | 4 / 2 | 8 / 1 | 6 / 1 |
| support.ticks_with_le1_option_pct | 24.89 / 36.83 | 30.35 / 34.85 | 36.58 / 38.45 |
| support.width_m | 43.69 / 42.98 | 45.03 / 41.72 | 43.12 / 42.97 |
| trans.count_per_10min | 15 / 40 | 26 / 43 | 20 / 43 |
| trans.counterpress_pct | 66.67 / 37.50 | 65.38 / 39.53 | 70 / 51.16 |
| trans.forward_recovery_speed_mps_0_5s | 7.03 / 7.05 | 7.26 / 6.85 | 7.45 / 7.14 |
| trans.forward_sprint_share_0_5s | 67.72 / 68.97 | 73.03 / 64.23 | 75.03 / 69.70 |
| trans.forward_sprint_share_0_5s_7mps | 56.21 / 56.26 | 60.03 / 52.91 | 65.72 / 59.73 |
| trans.forwards_staying_high_mean | 1.67 / 1.69 | 2 / 1.15 | 2 / 2 |
| trans.in_shape_at_turnover_pct | 76.92 / 52 | 47.06 / 51.35 | 78.57 / 60 |
| trans.mid_recovery_speed_mps_0_5s | 6.55 / 6.54 | 6.64 / 6.45 | 6.85 / 6.51 |
| trans.mid_sprint_share_0_5s | 62.33 / 61.70 | 63.73 / 57.98 | 68.00 / 60.71 |
| trans.mid_sprint_share_0_5s_7mps | 47.37 / 47.76 | 52.42 / 46.52 | 56.40 / 47.54 |
| trans.outfield_stamina_end_permille | 931.8 / 900 | 904.3 / 876 | 893.3 / 871 |
| trans.pct_outfield_behind_ball_at_3s | 76 / 54.78 | 70 / 64.52 | 72.22 / 53.44 |
| trans.pct_outfield_behind_ball_at_6s | 73.33 / 65.38 | 58.33 / 80.50 | 57.50 / 57.78 |
| trans.time_to_shape_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_excluded_pct | 13.33 / 37.50 | 34.62 / 13.95 | 30 / 41.86 |
| trans.time_to_shape_s | 0.80 / 1.26 | 1.42 / 1.28 | 0.69 / 1.10 |
| trans.time_to_shape_static_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_static_excluded_pct | 20 / 42.50 | 46.15 / 37.21 | 25 / 53.49 |
| trans.time_to_shape_static_s | 0.54 / 0.96 | 0.90 / 0.80 | 0.92 / 0.38 |

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
- **coord.mark_steal_per_defender_min**: Round 4 (area T). Changes of a defender's mark target (TeamAiMemory::mark_of goes from opponent A to a different opponent B, both not AI_NONE) while A is still within 1.35 x the marking zone (9 m in 11v11, 7 m in 5v5) of the defender's position, per defender-minute; defender-minutes count on-pitch outfielder ticks of open play while the opponent owns the ball. The FIFA 12 'steal' failure mode.
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
- **move.side_flips_cause.<cause>_per_player_min**: Round 5 (WA1). Raw side flips (move.side_flips_per_player_min) split by the earliest steering stage whose own lateral component (versus the same previous-tick velocity) flipped sign with the same 0.02 m/s dead band on the same tick pair: desired = the desired velocity before local avoidance flipped (decision side: intent, target, input); sampler = the avoidance-selected velocity flipped while the desired one did not; filter = only the filtered command flipped; limiter = none of the three flipped (motion limits, turn limiter, contact constraint; also flips without fresh psfc_sim PlayerState::steer_debug diagnostics, e.g. scripted actions). Each divided by all player-minutes, so the four sum to the raw rate.
- **move.side_flips_per_player_min**: Per player, over the heading-histogram samples (speed > 1 m/s on both ticks): lateral component l_t = cross(vel_t-1, vel_t) / |vel_t-1| (m/s, sign = turning left/right of the previous heading). A side flip is a sample whose l_t and the previous tick's l_t-1 are both above 0.02 m/s in magnitude with opposite signs (avoidance oscillation). Count / player-minutes.
- **move.side_flips_schmitt_per_player_min**: Round 5 (WA1). Schmitt-band variant of move.side_flips_per_player_min over the same samples: a per-player debounced side (left/right of the previous heading) changes only after the lateral component l_t has stayed beyond the opposite +-0.05 m/s band for 4 consecutive samples; each change from one side to the other counts one flip (the first side taken after a gap does not count; the state resets when a tick has no heading sample). Ignores sub-quantum tick-to-tick noise and counts weaving whose half-period is >= 4 ticks. Count / player-minutes.
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
- **support.release_pass_completed_pct**: % of passes launched toward a released / committed runner (as counted for releases_per_10min) whose next open-play owner is that runner; an opponent owning it, another teammate collecting it, or a stoppage (offside, out of play) before any owner is a failure.
- **support.releases_per_10min**: Round 4 (work area 3). Releases of committed runners per 10 open-play minutes: a CPU pass decision selecting a committed runner (TeamAiMemory::run_release with a new run_release_tick), or a ground or through pass of the side (the passer's recent_kick_kind 1 or 2; lofted switches excluded) launched toward a penetrate or called runner (run_role 1 or 5) that no CPU release of the same runner preceded within 60 ticks.
- **support.run_abandon_per_10min**: Round 4 (work area 3). Committed runs abandoned per 10 open-play minutes: a run role in the sim's TeamAiMemory (run_role / run_since: penetrate, support-from-behind, box, decoy, called) that ends or is replaced fewer than 60 ticks after it began, on an open-play tick on which the opponent does not own the ball, the ball is not loose (no owner and no pass of the side in flight), and the runner neither owns it nor is the intended receiver of the side's pass in flight. Runs ended by a stoppage (non-open-play tick), by possession loss, or by receiving the ball are not abandons (FIFA 12's 'change their minds three or four times' failure).
- **support.runs_in_behind_per_10min**: Per outfielder other than the owner, on open-play ticks while his side owns the ball (any owner incl. GK), against the line L = second-last opponent's x along the attacking axis on that tick: a run counts when the player was at or behind L on an earlier such tick (armed) and is now ahead of L with |vel| > 5 m/s. Since the work-area-3 second pass, the ticks of the side's own pass in flight also count for the pass's intended receiver (the other players' states are held), and so does the reception tick on which that receiver becomes the owner. After crossing (counted or not) he must drop more than 1 m behind L to be armed again; losing possession or a stoppage disarms everyone. Count per 10 open-play minutes.
- **support.width_m / support.depth_m**: y-extent / x-extent of the side's on-pitch outfielders (carrier included) on carrier ticks, mean.
- **trans.count_per_10min**: Turnovers suffered by the side (general.turnover) per 10 open-play minutes.
- **trans.counterpress_pct**: % of turnovers after which the side owns the ball again within 6 s (t0 + 360 ticks) with no stoppage in between.
- **trans.forward_recovery_speed_mps_0_5s / forward_sprint_share_0_5s**: Over episode ticks t0 .. t0 + 5 s (exclusive) and the side's on-pitch players with PlayerRole::Forward: mean |vel| (m/s), and % of those player-ticks with |vel| > 6 m/s.
- **trans.forward_sprint_share_0_5s_7mps / mid_sprint_share_0_5s_7mps**: Round 4 (area T). Same as forward_sprint_share_0_5s / mid_sprint_share_0_5s with the threshold at 7.0 m/s (25.2 km/h, the literature's sprint band) instead of 6 m/s, which coincides with the sim's 70% class-3 cruise cap and so counted capped jogging as sprinting.
- **trans.forwards_staying_high_mean**: Mean number of the side's on-pitch Forwards ahead of the ball ((x - ball.x) * attack_sign > 0) at t0 + 6 s, over episodes still running then (rest offence kept high).
- **trans.in_shape_at_turnover_pct**: % of scored time_to_shape episodes (defensive_shape_anchor variant) already in shape on the turnover tick t0 (time 0 s). A high value means the criterion is met by position at the moment of the loss (players already goal-side of the ball), not by a recovery run.
- **trans.mid_recovery_speed_mps_0_5s / mid_sprint_share_0_5s**: Same for PlayerRole::Midfielder.
- **trans.outfield_stamina_end_permille**: Round 4 (area T). Mean stamina_permille of the side's on-pitch outfielders on the last open-play tick of the match (the cost guard for pressing / recovery-sprint urgency).
- **trans.pct_outfield_behind_ball_at_3s / _at_6s**: % of the side's on-pitch outfielders strictly goal-side of the ball at exactly t0 + 3 s (180 ticks) and t0 + 6 s (360 ticks), over episodes still running at that tick (no stoppage, not regained).
- **trans.time_to_shape_s / time_to_shape_capped_pct / time_to_shape_excluded_pct**: Seconds from t0 to the first episode tick on which at least 80% of the side's on-pitch outfielders are either within 5 m of psfc_sim::ai::defensive_shape_anchor(state, side, idx) (the defensive-block slot the AI is assigned, ball-relative via the team memory's ref_ball; falls back to the static slot for unassigned players) or strictly goal-side of the ball ((x - ball.x) * attack_sign < 0). Episodes reaching 15 s without it score 15 s and count as capped (capped_pct = % of scored episodes). Episodes that end earlier by a stoppage or by the side regaining the ball, before reaching shape, are not scored (excluded_pct = % of all turnovers). The tick t0 itself is checked (0 s = already in shape).
- **trans.time_to_shape_static_s /_static_capped_pct / _static_excluded_pct**: Same with the static formation slot world_anchor_for_player(state, side, idx, 0) instead of defensive_shape_anchor: a reference that does not depend on the AI's own block logic, for before/after comparisons across AI changes.

