# Match analytics

## 5v5 hard vs hard (10 min x 3 seeds from 1)

Command: `psfc-replay analyze --format 5v5 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty hard --away-difficulty hard`

Gameplay version: `0.1.0-5v5`

| seed | final state_hash | score (H-A) |
|---|---|---|
| 1 | `62ca7f737de6fc40` | 3-6 |
| 2 | `597e5854429a0fe8` | 3-5 |
| 3 | `124b71f1cbce41fe` | 8-5 |

Match meta (mean over seeds): goals_total = 10, open_play_minutes = 10, open_play_pct_of_ticks = 93.60, shots_total = 54.33, ticks_simulated = 38466

### Match

| metric | home | away | teams mean |
|---|---:|---:|---:|
| fouls | 0.33 | 0 | **0.17** |
| goals | 4.67 | 5.33 | **5** |
| offsides | 3.67 | 0.67 | **2.17** |
| passes_completed_pct | 87.05 | 87.55 | **87.30** |
| possession_pct | 50.00 | 50.00 | **50** |
| shots | 27 | 27.33 | **27.17** |
| shots_on_target | 22 | 22.67 | **22.33** |

### Goalkeeper

| metric | home | away | teams mean |
|---|---:|---:|---:|
| catches | 2.33 | 2.33 | **2.33** |
| claims_non_save | 1 | 2 | **1.50** |
| excursion_mean_s | 0.67 | 0.57 | **0.62** |
| excursions_gt1s_per_10min | 0.33 | 0 | **0.17** |
| gathers | 3.33 | 4.33 | **3.83** |
| goals_conceded | 5.33 | 4.67 | **5** |
| goals_conceded_gk_gt3m_off_line | 3 | 1.67 | **2.33** |
| goals_conceded_no_shot_found | 0 | 0 | **0** |
| line_dist_at_conceded_shot_mean_m | 2.78 | 2.42 | **2.60** |
| line_dist_max_m | 7.24 | 7.67 | **7.46** |
| line_dist_mean_m | 2.99 | 2.97 | **2.98** |
| parries | 16 | 15.67 | **15.83** |
| pct_outside_6yd | 0.11 | 0.16 | **0.14** |
| pct_outside_box | 0 | 0 | **0** |
| rush_count | 0.67 | 1.33 | **1** |
| rush_per_10min | 0.67 | 1.33 | **1** |
| saves | 18.33 | 18 | **18.17** |
| tips_over | 0 | 0 | **0** |

### Defence and shape

| metric | home | away | teams mean |
|---|---:|---:|---:|
| attacking.anchor_dist_m.defender | 8.00 | 8.84 | **8.42** |
| attacking.anchor_dist_m.forward | 15.57 | 16.60 | **16.08** |
| attacking.anchor_dist_m.goalkeeper | 1.26 | 1.29 | **1.28** |
| attacking.anchor_dist_m.midfielder | 15.84 | 15.97 | **15.90** |
| attacking.anchor_dist_shape_m.defender | 7.63 | 7.85 | **7.74** |
| attacking.anchor_dist_shape_m.forward | 10.65 | 11.08 | **10.87** |
| attacking.anchor_dist_shape_m.midfielder | 10.47 | 10.19 | **10.33** |
| attacking.bbox_area_m2 | 325.7 | 300.3 | **313.0** |
| attacking.bbox_depth_m | 18.91 | 17.68 | **18.30** |
| attacking.bbox_width_m | 16.03 | 16.02 | **16.02** |
| attacking.length_m | 18.91 | 17.68 | **18.30** |
| attacking.nearest_mate_mean_m | 10.19 | 9.78 | **9.99** |
| attacking.nearest_mate_p10_m | 3.71 | 3.54 | **3.63** |
| attacking.pct_of_open_play | 51.09 | 48.91 | **50** |
| attacking.stretch_m | 9.15 | 8.87 | **9.01** |
| attacking.width_m | 16.03 | 16.02 | **16.02** |
| defending.anchor_dist_m.defender | 7.89 | 7.93 | **7.91** |
| defending.anchor_dist_m.forward | 15.03 | 15.55 | **15.29** |
| defending.anchor_dist_m.goalkeeper | 1.38 | 1.43 | **1.41** |
| defending.anchor_dist_m.midfielder | 14.09 | 13.65 | **13.87** |
| defending.anchor_dist_shape_m.defender | 8.95 | 9.16 | **9.05** |
| defending.anchor_dist_shape_m.forward | 9.83 | 9.74 | **9.78** |
| defending.anchor_dist_shape_m.midfielder | 9.17 | 8.71 | **8.94** |
| defending.bbox_area_m2 | 350.2 | 343.2 | **346.7** |
| defending.bbox_depth_m | 19.25 | 18.20 | **18.73** |
| defending.bbox_width_m | 17.35 | 17.76 | **17.56** |
| defending.length_m | 19.25 | 18.20 | **18.73** |
| defending.line_x_std_m | 6.03 | 5.61 | **5.82** |
| defending.line_x_variance_m2 | 58.18 | 51.96 | **55.07** |
| defending.nearest_mate_mean_m | 10.74 | 10.62 | **10.68** |
| defending.nearest_mate_p10_m | 4.17 | 4.18 | **4.18** |
| defending.pct_of_open_play | 48.91 | 51.09 | **50** |
| defending.pct_outfield_behind_ball | 68.47 | 67.38 | **67.92** |
| defending.stretch_m | 9.59 | 9.40 | **9.50** |
| defending.swarm2_pct | 0.28 | 0.25 | **0.27** |
| defending.swarm3_pct | 0 | 0 | **0** |
| defending.width_m | 17.35 | 17.76 | **17.56** |

### Defensive transition

| metric | home | away | teams mean |
|---|---:|---:|---:|
| count_per_10min | 21 | 21 | **21** |
| counterpress_pct | 42.77 | 25.19 | **33.98** |
| forward_recovery_speed_mps_0_5s | 6.63 | 6.43 | **6.53** |
| forward_sprint_share_0_5s | 63.72 | 61.82 | **62.77** |
| forward_sprint_share_0_5s_7mps | 49.71 | 47.18 | **48.44** |
| forwards_staying_high_mean | 0.80 | 0.84 | **0.82** |
| in_shape_at_turnover_pct | 62.20 | 60.35 | **61.28** |
| mid_recovery_speed_mps_0_5s | 6.45 | 6.39 | **6.42** |
| mid_sprint_share_0_5s | 63.29 | 62.09 | **62.69** |
| mid_sprint_share_0_5s_7mps | 51.72 | 50.32 | **51.02** |
| outfield_stamina_end_permille | 841.2 | 837.4 | **839.3** |
| pct_outfield_behind_ball_at_3s | 62.19 | 70.70 | **66.45** |
| pct_outfield_behind_ball_at_6s | 70.54 | 65.05 | **67.79** |
| time_to_shape_capped_pct | 0 | 0 | **0** |
| time_to_shape_excluded_pct | 20.74 | 12.06 | **16.40** |
| time_to_shape_s | 0.90 | 0.87 | **0.89** |
| time_to_shape_static_capped_pct | 0 | 0 | **0** |
| time_to_shape_static_excluded_pct | 33.54 | 22.45 | **27.99** |
| time_to_shape_static_s | 0.23 | 0.60 | **0.41** |

### Attacking support

| metric | home | away | teams mean |
|---|---:|---:|---:|
| carrier_pct_of_open_play | 19.19 | 19.14 | **19.16** |
| cluster_pct | 6.95 | 8.36 | **7.65** |
| depth_m | 19.79 | 18.94 | **19.37** |
| forward_options_mean | 0.21 | 0.24 | **0.23** |
| nearest_option_dist_m | 12.10 | 11.88 | **11.99** |
| open_options_mean | 1.34 | 1.43 | **1.38** |
| open_options_p10 | 0 | 0 | **0** |
| pass_target_open_pct | 73.07 | 73.91 | **73.49** |
| passes_completed_tracked | 132.3 | 125.7 | **129** |
| release_pass_completed_pct | 72.40 | 68.82 | **70.61** |
| releases_per_10min | 17 | 10.67 | **13.83** |
| run_abandon_per_10min | 0.33 | 0 | **0.17** |
| runs_in_behind_per_10min | 6.67 | 4.33 | **5.50** |
| ticks_with_le1_option_pct | 53.56 | 48.82 | **51.19** |
| width_m | 14.95 | 15.60 | **15.28** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 10.97 | 11.26 | **11.12** |
| accel_gt10_pct | 50.90 | 51.76 | **51.33** |
| accel_mean_mps2 | 10.95 | 11.23 | **11.09** |
| class_time_pct.0 | 12.46 | 12.25 | **12.35** |
| class_time_pct.1 | 6.98 | 7.18 | **7.08** |
| class_time_pct.2 | 65.04 | 65.79 | **65.42** |
| class_time_pct.3 | 15.52 | 14.78 | **15.15** |
| dspeed_max_mps_per_tick | 3.87 | 3.95 | **3.91** |
| heading_change_p95_deg_per_tick_gt3mps | 3.44 | 3.45 | **3.45** |
| heading_change_pct.0_10 | 99.44 | 99.40 | **99.42** |
| heading_change_pct.10_45 | 0.55 | 0.58 | **0.56** |
| heading_change_pct.45_90 | 0.02 | 0.02 | **0.02** |
| heading_change_pct.gt90 | 0.00 | 0.00 | **0.00** |
| heading_sign_flips_per_player_min | 16.73 | 16.77 | **16.75** |
| lateral_accel_mean_mps2 | 8.13 | 8.23 | **8.18** |
| reversals_per_player_min | 0.01 | 0.03 | **0.02** |
| side_flips_by_class.0_per_player_min | 116.3 | 114.0 | **115.2** |
| side_flips_by_class.0_share_pct | 39.93 | 37.58 | **38.76** |
| side_flips_by_class.1_per_player_min | 42.51 | 47.21 | **44.86** |
| side_flips_by_class.1_share_pct | 8.18 | 9.10 | **8.64** |
| side_flips_by_class.2_per_player_min | 23.87 | 24.76 | **24.31** |
| side_flips_by_class.2_share_pct | 42.72 | 43.92 | **43.32** |
| side_flips_by_class.3_per_player_min | 21.40 | 23.54 | **22.47** |
| side_flips_by_class.3_share_pct | 9.17 | 9.39 | **9.28** |
| side_flips_cause.desired_per_player_min | 15.76 | 15.52 | **15.64** |
| side_flips_cause.filter_per_player_min | 17.65 | 17.82 | **17.74** |
| side_flips_cause.limiter_per_player_min | 1.31 | 2.14 | **1.73** |
| side_flips_cause.sampler_per_player_min | 1.60 | 1.64 | **1.62** |
| side_flips_per_player_min | 36.33 | 37.12 | **36.73** |
| side_flips_schmitt_per_player_min | 39.73 | 39.74 | **39.74** |
| side_flips_speed_ge3_per_player_min | 47.83 | 47.67 | **47.75** |
| side_flips_speed_lt3_per_player_min | 10.93 | 12.59 | **11.76** |
| speed_oscillation_pct | 4.26 | 4.47 | **4.36** |
| turn_rate_mean_deg_per_s | 100.6 | 100.6 | **100.6** |

### Collisions and push-out

| metric | home | away | teams mean |
|---|---:|---:|---:|
| any.depth_max_m | 0.68 | 0.68 | **0.68** |
| any.depth_mean_m | 0.24 | 0.24 | **0.24** |
| any.events_per_min | 1.77 | 1.77 | **1.77** |
| any.stuck_mean_ticks | 8.33 | 8.33 | **8.33** |
| any.stuck_pairs | 1 | 1 | **1** |
| any.ticks_gt1cm_per_min | 4.47 | 4.47 | **4.47** |
| any.ticks_per_min | 4.90 | 4.90 | **4.90** |
| correction_vel_corrected_p50_mps | 0.76 | 0.70 | **0.73** |
| correction_vel_corrected_p99_mps | 11.17 | 9.80 | **10.48** |
| correction_vel_corrected_pct | 0.07 | 0.08 | **0.07** |
| correction_vel_max_mps | 11.43 | 13.14 | **12.28** |
| correction_vel_p50_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p95_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p99_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_pct_gt_0_3 | 0.05 | 0.06 | **0.05** |
| opponent.depth_max_m | 0.10 | 0.10 | **0.10** |
| opponent.depth_mean_m | 0.03 | 0.03 | **0.03** |
| opponent.events_per_min | 1.03 | 1.03 | **1.03** |
| opponent.stuck_mean_ticks | 0 | 0 | **0** |
| opponent.stuck_pairs | 0 | 0 | **0** |
| opponent.ticks_gt1cm_per_min | 1.03 | 1.03 | **1.03** |
| opponent.ticks_per_min | 1.37 | 1.37 | **1.37** |
| teammate.depth_max_m | 0.61 | 0.68 | **0.65** |
| teammate.depth_mean_m | 0.29 | 0.31 | **0.30** |
| teammate.events_per_min | 0.33 | 0.40 | **0.37** |
| teammate.stuck_mean_ticks | 7.67 | 5.33 | **6.50** |
| teammate.stuck_pairs | 0.67 | 0.33 | **0.50** |
| teammate.ticks_gt1cm_per_min | 2.07 | 2.80 | **2.43** |
| teammate.ticks_per_min | 2.10 | 2.87 | **2.48** |

### Coordination

| metric | home | away | teams mean |
|---|---:|---:|---:|
| cruise_over72_pct | 36.11 | 36.56 | **36.34** |
| dual_chaser_pct_of_loose | 18.05 | 18.14 | **18.10** |
| dual_chaser_ticks_per_min | 396.6 | 399.3 | **397.9** |
| engagers3_pct_of_carrier_ticks | 0.09 | 0.08 | **0.08** |
| engagers3_ticks_per_min | 0.63 | 0.53 | **0.58** |
| mark_steal_per_defender_min | 0.55 | 0.21 | **0.38** |
| max_simultaneous_chasers | 4.33 | 4 | **4.17** |

### Aerial

| metric | home | away | teams mean |
|---|---:|---:|---:|
| headers | 8.33 | 8.67 | **8.50** |
| headers_clear | 0.33 | 0.67 | **0.50** |
| headers_pass | 5.33 | 5 | **5.17** |
| headers_shot | 2.67 | 3 | **2.83** |
| jumps_gk | 22.33 | 23.33 | **22.83** |
| jumps_outfield | 19.67 | 22.67 | **21.17** |
| max_airborne_outfield | 1 | 1 | **1** |

### Team shape vs envelope (teams mean)

| metric | in possession | out of possession | 11v11 envelope (m) |
|---|---:|---:|---|
| length_m | 18.30 | 18.73 | 31-46 |
| width_m | 16.02 | 17.56 | 35-48 |
| stretch_m | 9.01 | 9.50 | 7-16 |
| nearest_mate_mean_m | 9.99 | 10.68 | none published (in-possession mean near 9-10 m, estimate) |
| nearest_mate_p10_m | 3.63 | 4.18 | none published (in-possession mean near 9-10 m, estimate) |

Note: 5v5 has no published team-shape envelope; the 11v11 ranges are printed for reference only and do not apply to a 4-outfielder team on a small pitch.

### Per seed (home / away)

| metric | seed 1 | seed 2 | seed 3 |
|---|---:|---:|---:|
| aerial.headers | 10 / 13 | 8 / 7 | 7 / 6 |
| aerial.headers_clear | 0 / 0 | 0 / 1 | 1 / 1 |
| aerial.headers_pass | 6 / 7 | 7 / 3 | 3 / 5 |
| aerial.headers_shot | 4 / 6 | 1 / 3 | 3 / 0 |
| aerial.jumps_gk | 21 / 25 | 24 / 20 | 22 / 25 |
| aerial.jumps_outfield | 19 / 27 | 19 / 19 | 21 / 22 |
| aerial.max_airborne_outfield | 1 / 1 | 1 / 1 | 1 / 1 |
| collision.any.depth_max_m | 0.46 / 0.46 | 0.79 / 0.79 | 0.79 / 0.79 |
| collision.any.depth_mean_m | 0.11 / 0.11 | 0.29 / 0.29 | 0.32 / 0.32 |
| collision.any.events_per_min | 1.40 / 1.40 | 1.80 / 1.80 | 2.10 / 2.10 |
| collision.any.stuck_mean_ticks | 0 / 0 | 11 / 11 | 14 / 14 |
| collision.any.stuck_pairs | 0 / 0 | 1 / 1 | 2 / 2 |
| collision.any.ticks_gt1cm_per_min | 2.80 / 2.80 | 5 / 5 | 5.60 / 5.60 |
| collision.any.ticks_per_min | 3.10 / 3.10 | 5.30 / 5.30 | 6.30 / 6.30 |
| collision.correction_vel_corrected_p50_mps | 0.65 / 0.65 | 0.55 / 0.81 | 1.07 / 0.64 |
| collision.correction_vel_corrected_p99_mps | 10.20 / 12.67 | 10.55 / 3.60 | 12.76 / 13.13 |
| collision.correction_vel_corrected_pct | 0.05 / 0.06 | 0.07 / 0.09 | 0.08 / 0.09 |
| collision.correction_vel_max_mps | 10.20 / 12.73 | 11.10 / 13.27 | 12.98 / 13.41 |
| collision.correction_vel_p50_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p95_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p99_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_pct_gt_0_3 | 0.03 / 0.04 | 0.05 / 0.07 | 0.05 / 0.06 |
| collision.opponent.depth_max_m | 0.10 / 0.10 | 0.10 / 0.10 | 0.08 / 0.08 |
| collision.opponent.depth_mean_m | 0.03 / 0.03 | 0.03 / 0.03 | 0.03 / 0.03 |
| collision.opponent.events_per_min | 1 / 1 | 1 / 1 | 1.10 / 1.10 |
| collision.opponent.stuck_mean_ticks | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.stuck_pairs | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.ticks_gt1cm_per_min | 1.20 / 1.20 | 1.10 / 1.10 | 0.80 / 0.80 |
| collision.opponent.ticks_per_min | 1.40 / 1.40 | 1.40 / 1.40 | 1.30 / 1.30 |
| collision.teammate.depth_max_m | 0.30 / 0.46 | 0.75 / 0.79 | 0.79 / 0.79 |
| collision.teammate.depth_mean_m | 0.16 / 0.19 | 0.36 / 0.36 | 0.36 / 0.39 |
| collision.teammate.events_per_min | 0.10 / 0.30 | 0.30 / 0.50 | 0.60 / 0.40 |
| collision.teammate.stuck_mean_ticks | 0 / 0 | 11 / 0 | 12 / 16 |
| collision.teammate.stuck_pairs | 0 / 0 | 1 / 0 | 1 / 1 |
| collision.teammate.ticks_gt1cm_per_min | 0.70 / 1 | 2.30 / 3.30 | 3.20 / 4.10 |
| collision.teammate.ticks_per_min | 0.70 / 1.10 | 2.30 / 3.30 | 3.30 / 4.20 |
| coord.cruise_over72_pct | 34.57 / 36.33 | 37.13 / 36.16 | 36.63 / 37.20 |
| coord.dual_chaser_pct_of_loose | 16.77 / 19.23 | 18.44 / 18.39 | 18.93 / 16.81 |
| coord.dual_chaser_ticks_per_min | 372.1 / 426.5 | 414.3 / 413.2 | 403.3 / 358.2 |
| coord.engagers3_pct_of_carrier_ticks | 0 / 0 | 0.03 / 0.20 | 0.23 / 0.04 |
| coord.engagers3_ticks_per_min | 0 / 0 | 0.20 / 1.30 | 1.70 / 0.30 |
| coord.mark_steal_per_defender_min | 0.41 / 0.25 | 0.64 / 0.14 | 0.62 / 0.24 |
| coord.max_simultaneous_chasers | 4 / 4 | 5 / 4 | 4 / 4 |
| gk.catches | 5 / 3 | 1 / 1 | 1 / 3 |
| gk.claims_non_save | 0 / 4 | 1 / 1 | 2 / 1 |
| gk.excursion_mean_s | 0 / 0.81 | 0.27 / 0.48 | 1.75 / 0.43 |
| gk.excursions_gt1s_per_10min | 0 / 0 | 0 / 0 | 1 / 0 |
| gk.gathers | 5 / 7 | 2 / 2 | 3 / 4 |
| gk.goals_conceded | 6 / 3 | 5 / 3 | 5 / 8 |
| gk.goals_conceded_gk_gt3m_off_line | 3 / 1 | 3 / 2 | 3 / 2 |
| gk.goals_conceded_no_shot_found | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.line_dist_at_conceded_shot_mean_m | 2.89 / 1.90 | 2.42 / 2.74 | 3.03 / 2.62 |
| gk.line_dist_max_m | 5.20 / 10.52 | 5.71 / 6.30 | 10.81 / 6.19 |
| gk.line_dist_mean_m | 2.99 / 2.99 | 2.96 / 2.96 | 3.01 / 2.95 |
| gk.parries | 14 / 15 | 16 / 17 | 18 / 15 |
| gk.pct_outside_6yd | 0 / 0.27 | 0.04 / 0.08 | 0.29 / 0.14 |
| gk.pct_outside_box | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_count | 0 / 1 | 1 / 1 | 1 / 2 |
| gk.rush_per_10min | 0 / 1 | 1 / 1 | 1 / 2 |
| gk.saves | 19 / 18 | 17 / 18 | 19 / 18 |
| gk.tips_over | 0 / 0 | 0 / 0 | 0 / 0 |
| match.fouls | 1 / 0 | 0 / 0 | 0 / 0 |
| match.goals | 3 / 6 | 3 / 5 | 8 / 5 |
| match.offsides | 5 / 0 | 1 / 2 | 5 / 0 |
| match.passes_completed_pct | 88.82 / 91.53 | 88.38 / 87.57 | 83.96 / 83.54 |
| match.possession_pct | 51.80 / 48.20 | 47.72 / 52.28 | 50.47 / 49.53 |
| match.shots | 27 / 28 | 25 / 29 | 29 / 25 |
| match.shots_on_target | 20 / 23 | 21 / 22 | 25 / 23 |
| move.accel_from_pos_mean_mps2 | 11.05 / 11.37 | 11.17 / 11.36 | 10.69 / 11.05 |
| move.accel_gt10_pct | 51.96 / 52.38 | 51.78 / 52.15 | 48.97 / 50.76 |
| move.accel_mean_mps2 | 11.05 / 11.33 | 11.14 / 11.35 | 10.67 / 11.01 |
| move.class_time_pct.0 | 12.28 / 12.31 | 12.57 / 12.53 | 12.53 / 11.91 |
| move.class_time_pct.1 | 7.17 / 7.23 | 7.12 / 6.80 | 6.65 / 7.50 |
| move.class_time_pct.2 | 64.02 / 65.12 | 65.37 / 66.33 | 65.73 / 65.92 |
| move.class_time_pct.3 | 16.53 / 15.34 | 14.94 / 14.33 | 15.09 / 14.67 |
| move.dspeed_max_mps_per_tick | 3.49 / 3.85 | 4.48 / 3.72 | 3.63 / 4.28 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.44 / 3.47 | 3.48 / 3.47 | 3.39 / 3.42 |
| move.heading_change_pct.0_10 | 99.45 / 99.34 | 99.44 / 99.40 | 99.43 / 99.45 |
| move.heading_change_pct.10_45 | 0.54 / 0.64 | 0.54 / 0.57 | 0.55 / 0.53 |
| move.heading_change_pct.45_90 | 0.01 / 0.02 | 0.02 / 0.02 | 0.02 / 0.03 |
| move.heading_change_pct.gt90 | 0 / 0 | 0 / 0.00 | 0.00 / 0.00 |
| move.heading_sign_flips_per_player_min | 16.14 / 17.90 | 18.02 / 17.22 | 16.02 / 15.20 |
| move.lateral_accel_mean_mps2 | 8.33 / 8.33 | 8.22 / 8.30 | 7.83 / 8.07 |
| move.reversals_per_player_min | 0 / 0 | 0 / 0.08 | 0.02 / 0.02 |
| move.side_flips_by_class.0_per_player_min | 113.5 / 124.7 | 116.6 / 117.0 | 118.9 / 100.4 |
| move.side_flips_by_class.0_share_pct | 38.33 / 40.18 | 39.27 / 37.91 | 42.20 / 34.67 |
| move.side_flips_by_class.1_per_player_min | 47.75 / 37.67 | 43.31 / 58.58 | 36.46 / 45.38 |
| move.side_flips_by_class.1_share_pct | 9.42 / 7.12 | 8.26 / 10.31 | 6.86 / 9.87 |
| move.side_flips_by_class.2_per_player_min | 23.67 / 25.58 | 25.24 / 24.94 | 22.70 / 23.76 |
| move.side_flips_by_class.2_share_pct | 41.69 / 43.58 | 44.21 / 42.78 | 42.26 / 45.41 |
| move.side_flips_by_class.3_per_player_min | 23.25 / 22.71 | 20.65 / 24.31 | 20.30 / 23.62 |
| move.side_flips_by_class.3_share_pct | 10.57 / 9.11 | 8.26 / 9.01 | 8.68 / 10.05 |
| move.side_flips_cause.desired_per_player_min | 14.99 / 17.00 | 16.66 / 15.60 | 15.62 / 13.96 |
| move.side_flips_cause.filter_per_player_min | 18.06 / 17.72 | 17.56 / 18.30 | 17.34 / 17.44 |
| move.side_flips_cause.limiter_per_player_min | 1.38 / 1.70 | 1.62 / 3.14 | 0.94 / 1.58 |
| move.side_flips_cause.sampler_per_player_min | 1.92 / 1.80 | 1.48 / 1.62 | 1.40 / 1.50 |
| move.side_flips_per_player_min | 36.35 / 38.22 | 37.33 / 38.67 | 35.31 / 34.49 |
| move.side_flips_schmitt_per_player_min | 39.96 / 39.78 | 39.67 / 40.43 | 39.57 / 39.01 |
| move.side_flips_speed_ge3_per_player_min | 48.04 / 49.07 | 48.85 / 50.14 | 46.60 / 43.81 |
| move.side_flips_speed_lt3_per_player_min | 10.84 / 12.98 | 11.25 / 12.39 | 10.70 / 12.41 |
| move.speed_oscillation_pct | 4.29 / 4.51 | 4.35 / 4.44 | 4.15 / 4.45 |
| move.turn_rate_mean_deg_per_s | 102.5 / 102.7 | 102.0 / 100.9 | 97.41 / 98.29 |
| shape.attacking.anchor_dist_m.defender | 6.91 / 7.77 | 8.77 / 10.31 | 8.30 / 8.45 |
| shape.attacking.anchor_dist_m.forward | 14.51 / 16.68 | 15.84 / 16.13 | 16.35 / 16.98 |
| shape.attacking.anchor_dist_m.goalkeeper | 1.25 / 1.29 | 1.25 / 1.28 | 1.29 / 1.29 |
| shape.attacking.anchor_dist_m.midfielder | 16.45 / 15.14 | 14.62 / 16.43 | 16.45 / 16.35 |
| shape.attacking.anchor_dist_shape_m.defender | 7.16 / 7.40 | 7.71 / 8.48 | 8.01 / 7.67 |
| shape.attacking.anchor_dist_shape_m.forward | 9.58 / 10.57 | 11.14 / 11.25 | 11.23 / 11.42 |
| shape.attacking.anchor_dist_shape_m.midfielder | 11.34 / 9.38 | 8.90 / 10.20 | 11.17 / 11.00 |
| shape.attacking.bbox_area_m2 | 353.9 / 329.9 | 296.4 / 291.5 | 326.8 / 279.4 |
| shape.attacking.bbox_depth_m | 20.29 / 18.08 | 17.93 / 18.00 | 18.53 / 16.96 |
| shape.attacking.bbox_width_m | 16.23 / 17.18 | 15.64 / 15.00 | 16.23 / 15.87 |
| shape.attacking.length_m | 20.29 / 18.08 | 17.93 / 18.00 | 18.53 / 16.96 |
| shape.attacking.nearest_mate_mean_m | 10.68 / 10.19 | 9.75 / 9.51 | 10.15 / 9.64 |
| shape.attacking.nearest_mate_p10_m | 4.02 / 3.39 | 3.52 / 3.47 | 3.59 / 3.77 |
| shape.attacking.pct_of_open_play | 49.79 / 50.21 | 51.71 / 48.29 | 51.76 / 48.24 |
| shape.attacking.stretch_m | 9.57 / 9.31 | 8.78 / 8.72 | 9.10 / 8.58 |
| shape.attacking.width_m | 16.23 / 17.18 | 15.64 / 15.00 | 16.23 / 15.87 |
| shape.defending.anchor_dist_m.defender | 7.32 / 7.31 | 8.70 / 7.28 | 7.64 / 9.21 |
| shape.defending.anchor_dist_m.forward | 14.09 / 15.88 | 16.38 / 15.00 | 14.61 / 15.77 |
| shape.defending.anchor_dist_m.goalkeeper | 1.35 / 1.41 | 1.47 / 1.41 | 1.33 / 1.46 |
| shape.defending.anchor_dist_m.midfielder | 14.88 / 13.37 | 12.86 / 12.56 | 14.54 / 15.03 |
| shape.defending.anchor_dist_shape_m.defender | 8.37 / 8.87 | 9.45 / 8.55 | 9.04 / 10.05 |
| shape.defending.anchor_dist_shape_m.forward | 9.01 / 9.61 | 10.56 / 9.58 | 9.91 / 10.01 |
| shape.defending.anchor_dist_shape_m.midfielder | 10.15 / 8.18 | 7.51 / 7.89 | 9.86 / 10.05 |
| shape.defending.bbox_area_m2 | 367.1 / 335.6 | 354.1 / 350.0 | 329.2 / 343.9 |
| shape.defending.bbox_depth_m | 20.35 / 18.13 | 18.48 / 18.84 | 18.92 / 17.63 |
| shape.defending.bbox_width_m | 17.05 / 17.60 | 18.15 / 17.63 | 16.87 / 18.04 |
| shape.defending.length_m | 20.35 / 18.13 | 18.48 / 18.84 | 18.92 / 17.63 |
| shape.defending.line_x_std_m | 6.29 / 5.37 | 5.85 / 5.79 | 5.96 / 5.66 |
| shape.defending.line_x_variance_m2 | 60.49 / 48.99 | 55.82 / 54.26 | 58.22 / 52.61 |
| shape.defending.nearest_mate_mean_m | 11.02 / 10.63 | 10.65 / 10.77 | 10.54 / 10.47 |
| shape.defending.nearest_mate_p10_m | 4.42 / 4.41 | 3.99 / 4.23 | 4.11 / 3.89 |
| shape.defending.pct_of_open_play | 50.21 / 49.79 | 48.29 / 51.71 | 48.24 / 51.76 |
| shape.defending.pct_outfield_behind_ball | 69.77 / 67.97 | 67.12 / 68.92 | 68.51 / 65.25 |
| shape.defending.stretch_m | 9.83 / 9.34 | 9.57 / 9.54 | 9.38 / 9.31 |
| shape.defending.swarm2_pct | 0.33 / 0 | 0.31 / 0.54 | 0.21 / 0.22 |
| shape.defending.swarm3_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| shape.defending.width_m | 17.05 / 17.60 | 18.15 / 17.63 | 16.87 / 18.04 |
| support.carrier_pct_of_open_play | 19.47 / 17.92 | 17.91 / 19.55 | 20.19 / 19.95 |
| support.cluster_pct | 6.24 / 10.43 | 6.95 / 7.90 | 7.65 / 6.75 |
| support.depth_m | 20.84 / 19.17 | 19.22 / 19.24 | 19.32 / 18.41 |
| support.forward_options_mean | 0.23 / 0.29 | 0.23 / 0.22 | 0.19 / 0.21 |
| support.nearest_option_dist_m | 12.23 / 12.50 | 11.99 / 11.47 | 12.06 / 11.65 |
| support.open_options_mean | 1.32 / 1.50 | 1.31 / 1.32 | 1.38 / 1.46 |
| support.open_options_p10 | 0 / 0 | 0 / 0 | 0 / 0 |
| support.pass_target_open_pct | 73.28 / 75.35 | 72.79 / 70.37 | 73.13 / 76 |
| support.passes_completed_tracked | 116 / 142 | 147 / 135 | 134 / 100 |
| support.release_pass_completed_pct | 68.18 / 76.47 | 82.35 / 70 | 66.67 / 60 |
| support.releases_per_10min | 22 / 17 | 17 / 10 | 12 / 5 |
| support.run_abandon_per_10min | 0 / 0 | 1 / 0 | 0 / 0 |
| support.runs_in_behind_per_10min | 8 / 6 | 3 / 5 | 9 / 2 |
| support.ticks_with_le1_option_pct | 53.60 / 46.34 | 52.41 / 53.70 | 54.67 / 46.43 |
| support.width_m | 14.98 / 16.95 | 14.72 / 14.85 | 15.14 / 15.01 |
| trans.count_per_10min | 22 / 17 | 20 / 21 | 21 / 25 |
| trans.counterpress_pct | 45.45 / 11.76 | 40 / 23.81 | 42.86 / 40 |
| trans.forward_recovery_speed_mps_0_5s | 6.75 / 6.62 | 6.82 / 6.25 | 6.32 / 6.41 |
| trans.forward_sprint_share_0_5s | 66.19 / 65.08 | 65.97 / 56.85 | 59.01 / 63.54 |
| trans.forward_sprint_share_0_5s_7mps | 51.99 / 52.41 | 53.37 / 40.72 | 43.77 / 48.41 |
| trans.forwards_staying_high_mean | 0.67 / 0.75 | 0.86 / 0.78 | 0.88 / 1 |
| trans.in_shape_at_turnover_pct | 61.11 / 43.75 | 66.67 / 61.11 | 58.82 / 76.19 |
| trans.mid_recovery_speed_mps_0_5s | 6.36 / 6.46 | 6.51 / 6.35 | 6.49 / 6.36 |
| trans.mid_sprint_share_0_5s | 60.72 / 63.10 | 65.36 / 62.16 | 63.80 / 61.01 |
| trans.mid_sprint_share_0_5s_7mps | 51.24 / 49.29 | 53.30 / 51.12 | 50.63 / 50.55 |
| trans.outfield_stamina_end_permille | 934.8 / 875 | 809.2 / 810 | 779.5 / 827.2 |
| trans.pct_outfield_behind_ball_at_3s | 67.11 / 61.67 | 61.67 / 71.88 | 57.81 / 78.57 |
| trans.pct_outfield_behind_ball_at_6s | 75 / 64.58 | 67.86 / 72.22 | 68.75 / 58.33 |
| trans.time_to_shape_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_excluded_pct | 18.18 / 5.88 | 25 / 14.29 | 19.05 / 16 |
| trans.time_to_shape_s | 1.08 / 1.26 | 0.79 / 0.85 | 0.84 / 0.50 |
| trans.time_to_shape_static_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_static_excluded_pct | 27.27 / 23.53 | 40 / 23.81 | 33.33 / 20 |
| trans.time_to_shape_static_s | 0.32 / 1.05 | 0.08 / 0.56 | 0.29 / 0.19 |

GK scenario field: not present in the public state.

## 5v5 normal vs normal (10 min x 3 seeds from 1)

Command: `psfc-replay analyze --format 5v5 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty normal --away-difficulty normal`

Gameplay version: `0.1.0-5v5`

| seed | final state_hash | score (H-A) |
|---|---|---|
| 1 | `8326feeb563d7001` | 1-7 |
| 2 | `a945aff3d8bd31c4` | 5-2 |
| 3 | `565aa6553a121ec0` | 2-6 |

Match meta (mean over seeds): goals_total = 7.67, open_play_minutes = 10.02, open_play_pct_of_ticks = 94.45, shots_total = 59, ticks_simulated = 38196.7

### Match

| metric | home | away | teams mean |
|---|---:|---:|---:|
| fouls | 0 | 0 | **0** |
| goals | 2.67 | 5 | **3.83** |
| offsides | 0.67 | 1.33 | **1** |
| passes_completed_pct | 85.48 | 85.11 | **85.29** |
| possession_pct | 49.16 | 50.84 | **50** |
| shots | 29.33 | 29.67 | **29.50** |
| shots_on_target | 24.67 | 25 | **24.83** |

### Goalkeeper

| metric | home | away | teams mean |
|---|---:|---:|---:|
| catches | 1.67 | 2.33 | **2** |
| claims_non_save | 1 | 0 | **0.50** |
| excursion_mean_s | 0.37 | 0.39 | **0.38** |
| excursions_gt1s_per_10min | 0.33 | 0.33 | **0.33** |
| gathers | 2.67 | 2.33 | **2.50** |
| goals_conceded | 5 | 2.67 | **3.83** |
| goals_conceded_gk_gt3m_off_line | 3 | 1.33 | **2.17** |
| goals_conceded_no_shot_found | 0 | 0 | **0** |
| line_dist_at_conceded_shot_mean_m | 2.70 | 3.07 | **2.89** |
| line_dist_max_m | 5.86 | 6.54 | **6.20** |
| line_dist_mean_m | 2.93 | 2.94 | **2.93** |
| parries | 18.67 | 20.33 | **19.50** |
| pct_outside_6yd | 0.06 | 0.06 | **0.06** |
| pct_outside_box | 0 | 0 | **0** |
| rush_count | 0.33 | 0.33 | **0.33** |
| rush_per_10min | 0.33 | 0.33 | **0.33** |
| saves | 20.33 | 22.67 | **21.50** |
| tips_over | 0 | 0 | **0** |

### Defence and shape

| metric | home | away | teams mean |
|---|---:|---:|---:|
| attacking.anchor_dist_m.defender | 8.95 | 9.27 | **9.11** |
| attacking.anchor_dist_m.forward | 16.43 | 17.23 | **16.83** |
| attacking.anchor_dist_m.goalkeeper | 1.30 | 1.31 | **1.30** |
| attacking.anchor_dist_m.midfielder | 17.51 | 16.24 | **16.88** |
| attacking.anchor_dist_shape_m.defender | 8.02 | 8.29 | **8.16** |
| attacking.anchor_dist_shape_m.forward | 10.74 | 11.38 | **11.06** |
| attacking.anchor_dist_shape_m.midfielder | 11.41 | 10.12 | **10.76** |
| attacking.bbox_area_m2 | 339.4 | 309.2 | **324.3** |
| attacking.bbox_depth_m | 18.74 | 17.59 | **18.16** |
| attacking.bbox_width_m | 16.93 | 16.00 | **16.47** |
| attacking.length_m | 18.74 | 17.59 | **18.16** |
| attacking.nearest_mate_mean_m | 10.27 | 9.77 | **10.02** |
| attacking.nearest_mate_p10_m | 3.43 | 3.35 | **3.39** |
| attacking.pct_of_open_play | 49.48 | 50.52 | **50** |
| attacking.stretch_m | 9.36 | 8.81 | **9.08** |
| attacking.width_m | 16.93 | 16.00 | **16.47** |
| defending.anchor_dist_m.defender | 8.68 | 8.03 | **8.36** |
| defending.anchor_dist_m.forward | 16.26 | 16.26 | **16.26** |
| defending.anchor_dist_m.goalkeeper | 1.46 | 1.45 | **1.46** |
| defending.anchor_dist_m.midfielder | 16.02 | 14.50 | **15.26** |
| defending.anchor_dist_shape_m.defender | 9.49 | 8.99 | **9.24** |
| defending.anchor_dist_shape_m.forward | 10.39 | 9.74 | **10.06** |
| defending.anchor_dist_shape_m.midfielder | 10.75 | 8.94 | **9.85** |
| defending.bbox_area_m2 | 353.5 | 356.1 | **354.8** |
| defending.bbox_depth_m | 18.90 | 19.17 | **19.03** |
| defending.bbox_width_m | 17.47 | 17.30 | **17.38** |
| defending.length_m | 18.90 | 19.17 | **19.03** |
| defending.line_x_std_m | 6.11 | 5.95 | **6.03** |
| defending.line_x_variance_m2 | 61.06 | 57.23 | **59.15** |
| defending.nearest_mate_mean_m | 10.47 | 10.66 | **10.56** |
| defending.nearest_mate_p10_m | 3.57 | 3.83 | **3.70** |
| defending.pct_of_open_play | 50.52 | 49.48 | **50** |
| defending.pct_outfield_behind_ball | 65.43 | 65.82 | **65.62** |
| defending.stretch_m | 9.54 | 9.57 | **9.56** |
| defending.swarm2_pct | 0.69 | 0.39 | **0.54** |
| defending.swarm3_pct | 0 | 0 | **0** |
| defending.width_m | 17.47 | 17.30 | **17.38** |

### Defensive transition

| metric | home | away | teams mean |
|---|---:|---:|---:|
| count_per_10min | 23.61 | 25.61 | **24.61** |
| counterpress_pct | 45.24 | 39.23 | **42.24** |
| forward_recovery_speed_mps_0_5s | 6.53 | 6.67 | **6.60** |
| forward_sprint_share_0_5s | 62.58 | 64.63 | **63.61** |
| forward_sprint_share_0_5s_7mps | 46.22 | 50.05 | **48.14** |
| forwards_staying_high_mean | 0.81 | 0.85 | **0.83** |
| in_shape_at_turnover_pct | 68.69 | 55.38 | **62.03** |
| mid_recovery_speed_mps_0_5s | 6.37 | 6.47 | **6.42** |
| mid_sprint_share_0_5s | 62.37 | 63.54 | **62.96** |
| mid_sprint_share_0_5s_7mps | 50.51 | 50.91 | **50.71** |
| outfield_stamina_end_permille | 856.2 | 814.6 | **835.4** |
| pct_outfield_behind_ball_at_3s | 61.94 | 59.89 | **60.92** |
| pct_outfield_behind_ball_at_6s | 65.18 | 59.72 | **62.45** |
| time_to_shape_capped_pct | 0 | 0 | **0** |
| time_to_shape_excluded_pct | 10.75 | 11.93 | **11.34** |
| time_to_shape_s | 0.75 | 0.89 | **0.82** |
| time_to_shape_static_capped_pct | 0 | 0 | **0** |
| time_to_shape_static_excluded_pct | 28.39 | 34.27 | **31.33** |
| time_to_shape_static_s | 0.26 | 0.27 | **0.26** |

### Attacking support

| metric | home | away | teams mean |
|---|---:|---:|---:|
| carrier_pct_of_open_play | 20.03 | 20.83 | **20.43** |
| cluster_pct | 5.49 | 9.19 | **7.34** |
| depth_m | 20.27 | 18.95 | **19.61** |
| forward_options_mean | 0.25 | 0.19 | **0.22** |
| nearest_option_dist_m | 12.36 | 11.78 | **12.07** |
| open_options_mean | 1.32 | 1.27 | **1.30** |
| open_options_p10 | 0 | 0 | **0** |
| pass_target_open_pct | 67.85 | 72.94 | **70.40** |
| passes_completed_tracked | 110 | 121.3 | **115.7** |
| release_pass_completed_pct | 90.05 | 85.35 | **87.70** |
| releases_per_10min | 11.97 | 9.32 | **10.64** |
| run_abandon_per_10min | 0.33 | 0 | **0.17** |
| runs_in_behind_per_10min | 2.00 | 4.33 | **3.16** |
| ticks_with_le1_option_pct | 54.55 | 57.04 | **55.79** |
| width_m | 16.70 | 15.56 | **16.13** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 10.73 | 10.94 | **10.84** |
| accel_gt10_pct | 49.59 | 50.27 | **49.93** |
| accel_mean_mps2 | 10.71 | 10.92 | **10.81** |
| class_time_pct.0 | 12.30 | 12.48 | **12.39** |
| class_time_pct.1 | 7.34 | 6.79 | **7.07** |
| class_time_pct.2 | 65.96 | 66.09 | **66.02** |
| class_time_pct.3 | 14.41 | 14.64 | **14.52** |
| dspeed_max_mps_per_tick | 4.39 | 4.10 | **4.24** |
| heading_change_p95_deg_per_tick_gt3mps | 3.38 | 3.46 | **3.42** |
| heading_change_pct.0_10 | 99.49 | 99.42 | **99.45** |
| heading_change_pct.10_45 | 0.49 | 0.56 | **0.53** |
| heading_change_pct.45_90 | 0.02 | 0.02 | **0.02** |
| heading_change_pct.gt90 | 0.00 | 0.00 | **0.00** |
| heading_sign_flips_per_player_min | 16.08 | 16.22 | **16.15** |
| lateral_accel_mean_mps2 | 7.99 | 8.13 | **8.06** |
| reversals_per_player_min | 0.03 | 0.01 | **0.02** |
| side_flips_by_class.0_per_player_min | 108.5 | 102.0 | **105.2** |
| side_flips_by_class.0_share_pct | 37.38 | 36.49 | **36.94** |
| side_flips_by_class.1_per_player_min | 42.91 | 40.82 | **41.87** |
| side_flips_by_class.1_share_pct | 8.80 | 8.05 | **8.43** |
| side_flips_by_class.2_per_player_min | 24.32 | 24.52 | **24.42** |
| side_flips_by_class.2_share_pct | 44.98 | 46.48 | **45.73** |
| side_flips_by_class.3_per_player_min | 21.89 | 21.41 | **21.65** |
| side_flips_by_class.3_share_pct | 8.84 | 8.98 | **8.91** |
| side_flips_cause.desired_per_player_min | 14.87 | 13.72 | **14.30** |
| side_flips_cause.filter_per_player_min | 17.60 | 17.90 | **17.75** |
| side_flips_cause.limiter_per_player_min | 1.53 | 1.74 | **1.63** |
| side_flips_cause.sampler_per_player_min | 1.65 | 1.53 | **1.59** |
| side_flips_per_player_min | 35.66 | 34.88 | **35.27** |
| side_flips_schmitt_per_player_min | 39.67 | 39.97 | **39.82** |
| side_flips_speed_ge3_per_player_min | 46.70 | 44.93 | **45.82** |
| side_flips_speed_lt3_per_player_min | 10.63 | 11.78 | **11.20** |
| speed_oscillation_pct | 4.41 | 4.37 | **4.39** |
| turn_rate_mean_deg_per_s | 97.22 | 99.21 | **98.22** |

### Collisions and push-out

| metric | home | away | teams mean |
|---|---:|---:|---:|
| any.depth_max_m | 0.79 | 0.79 | **0.79** |
| any.depth_mean_m | 0.31 | 0.31 | **0.31** |
| any.events_per_min | 1.90 | 1.90 | **1.90** |
| any.stuck_mean_ticks | 13.11 | 13.11 | **13.11** |
| any.stuck_pairs | 2.67 | 2.67 | **2.67** |
| any.ticks_gt1cm_per_min | 5.85 | 5.85 | **5.85** |
| any.ticks_per_min | 6.29 | 6.29 | **6.29** |
| correction_vel_corrected_p50_mps | 1.21 | 1.17 | **1.19** |
| correction_vel_corrected_p99_mps | 8.69 | 12.59 | **10.64** |
| correction_vel_corrected_pct | 0.07 | 0.07 | **0.07** |
| correction_vel_max_mps | 12.75 | 13.24 | **12.99** |
| correction_vel_p50_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p95_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p99_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_pct_gt_0_3 | 0.05 | 0.05 | **0.05** |
| opponent.depth_max_m | 0.11 | 0.11 | **0.11** |
| opponent.depth_mean_m | 0.03 | 0.03 | **0.03** |
| opponent.events_per_min | 0.90 | 0.90 | **0.90** |
| opponent.stuck_mean_ticks | 0 | 0 | **0** |
| opponent.stuck_pairs | 0 | 0 | **0** |
| opponent.ticks_gt1cm_per_min | 1.03 | 1.03 | **1.03** |
| opponent.ticks_per_min | 1.43 | 1.43 | **1.43** |
| teammate.depth_max_m | 0.79 | 0.78 | **0.79** |
| teammate.depth_mean_m | 0.35 | 0.38 | **0.37** |
| teammate.events_per_min | 0.57 | 0.43 | **0.50** |
| teammate.stuck_mean_ticks | 13.50 | 7.89 | **10.69** |
| teammate.stuck_pairs | 1.33 | 1.33 | **1.33** |
| teammate.ticks_gt1cm_per_min | 3.66 | 3.46 | **3.56** |
| teammate.ticks_per_min | 3.73 | 3.46 | **3.59** |

### Coordination

| metric | home | away | teams mean |
|---|---:|---:|---:|
| cruise_over72_pct | 37.62 | 35.65 | **36.64** |
| dual_chaser_pct_of_loose | 20.61 | 20.22 | **20.41** |
| dual_chaser_ticks_per_min | 435.2 | 426.8 | **431.0** |
| engagers3_pct_of_carrier_ticks | 0.04 | 0.09 | **0.07** |
| engagers3_ticks_per_min | 0.33 | 0.63 | **0.48** |
| mark_steal_per_defender_min | 0.42 | 0.50 | **0.46** |
| max_simultaneous_chasers | 4.67 | 4.33 | **4.50** |

### Aerial

| metric | home | away | teams mean |
|---|---:|---:|---:|
| headers | 7.67 | 7.67 | **7.67** |
| headers_clear | 0.67 | 1.33 | **1** |
| headers_pass | 4 | 2.67 | **3.33** |
| headers_shot | 3 | 3.67 | **3.33** |
| jumps_gk | 25.67 | 23.67 | **24.67** |
| jumps_outfield | 18 | 21 | **19.50** |
| max_airborne_outfield | 1 | 1 | **1** |

### Team shape vs envelope (teams mean)

| metric | in possession | out of possession | 11v11 envelope (m) |
|---|---:|---:|---|
| length_m | 18.16 | 19.03 | 31-46 |
| width_m | 16.47 | 17.38 | 35-48 |
| stretch_m | 9.08 | 9.56 | 7-16 |
| nearest_mate_mean_m | 10.02 | 10.56 | none published (in-possession mean near 9-10 m, estimate) |
| nearest_mate_p10_m | 3.39 | 3.70 | none published (in-possession mean near 9-10 m, estimate) |

Note: 5v5 has no published team-shape envelope; the 11v11 ranges are printed for reference only and do not apply to a 4-outfielder team on a small pitch.

### Per seed (home / away)

| metric | seed 1 | seed 2 | seed 3 |
|---|---:|---:|---:|
| aerial.headers | 6 / 8 | 10 / 7 | 7 / 8 |
| aerial.headers_clear | 2 / 1 | 0 / 2 | 0 / 1 |
| aerial.headers_pass | 2 / 4 | 6 / 1 | 4 / 3 |
| aerial.headers_shot | 2 / 3 | 4 / 4 | 3 / 4 |
| aerial.jumps_gk | 27 / 26 | 23 / 30 | 27 / 15 |
| aerial.jumps_outfield | 14 / 22 | 26 / 23 | 14 / 18 |
| aerial.max_airborne_outfield | 1 / 1 | 1 / 1 | 1 / 1 |
| collision.any.depth_max_m | 0.80 / 0.80 | 0.79 / 0.79 | 0.79 / 0.79 |
| collision.any.depth_mean_m | 0.32 / 0.32 | 0.32 / 0.32 | 0.29 / 0.29 |
| collision.any.events_per_min | 1.30 / 1.30 | 2.19 / 2.19 | 2.20 / 2.20 |
| collision.any.stuck_mean_ticks | 15 / 15 | 13 / 13 | 11.33 / 11.33 |
| collision.any.stuck_pairs | 1 / 1 | 4 / 4 | 3 / 3 |
| collision.any.ticks_gt1cm_per_min | 4.69 / 4.69 | 7.37 / 7.37 | 5.50 / 5.50 |
| collision.any.ticks_per_min | 4.79 / 4.79 | 8.17 / 8.17 | 5.90 / 5.90 |
| collision.correction_vel_corrected_p50_mps | 0.68 / 1.57 | 1.68 / 1.29 | 1.29 / 0.66 |
| collision.correction_vel_corrected_p99_mps | 3.60 / 13.05 | 12.48 / 12.25 | 10.00 / 12.47 |
| collision.correction_vel_corrected_pct | 0.06 / 0.05 | 0.09 / 0.09 | 0.07 / 0.07 |
| collision.correction_vel_max_mps | 13.00 / 13.05 | 12.86 / 13.55 | 12.37 / 13.11 |
| collision.correction_vel_p50_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p95_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p99_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_pct_gt_0_3 | 0.04 / 0.03 | 0.07 / 0.06 | 0.05 / 0.05 |
| collision.opponent.depth_max_m | 0.09 / 0.09 | 0.19 / 0.19 | 0.05 / 0.05 |
| collision.opponent.depth_mean_m | 0.04 / 0.04 | 0.04 / 0.04 | 0.02 / 0.02 |
| collision.opponent.events_per_min | 0.70 / 0.70 | 1.00 / 1.00 | 1 / 1 |
| collision.opponent.stuck_mean_ticks | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.stuck_pairs | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.ticks_gt1cm_per_min | 1.20 / 1.20 | 1.00 / 1.00 | 0.90 / 0.90 |
| collision.opponent.ticks_per_min | 1.30 / 1.30 | 1.79 / 1.79 | 1.20 / 1.20 |
| collision.teammate.depth_max_m | 0.80 / 0.77 | 0.79 / 0.79 | 0.79 / 0.79 |
| collision.teammate.depth_mean_m | 0.38 / 0.39 | 0.36 / 0.38 | 0.33 / 0.36 |
| collision.teammate.events_per_min | 0.30 / 0.30 | 0.60 / 0.60 | 0.80 / 0.40 |
| collision.teammate.stuck_mean_ticks | 15 / 0 | 14 / 12.67 | 11.50 / 11 |
| collision.teammate.stuck_pairs | 1 / 0 | 1 / 3 | 2 / 1 |
| collision.teammate.ticks_gt1cm_per_min | 3.09 / 2.20 | 3.78 / 5.48 | 4.10 / 2.70 |
| collision.teammate.ticks_per_min | 3.09 / 2.20 | 3.88 / 5.48 | 4.20 / 2.70 |
| coord.cruise_over72_pct | 33.27 / 36.43 | 41.11 / 33.81 | 38.47 / 36.71 |
| coord.dual_chaser_pct_of_loose | 20.27 / 21.44 | 20.12 / 20.11 | 21.42 / 19.11 |
| coord.dual_chaser_ticks_per_min | 422.9 / 447.2 | 427.9 / 427.7 | 454.7 / 405.6 |
| coord.engagers3_pct_of_carrier_ticks | 0 / 0.16 | 0 / 0 | 0.12 / 0.12 |
| coord.engagers3_ticks_per_min | 0 / 1.10 | 0 / 0 | 1 / 0.80 |
| coord.mark_steal_per_defender_min | 0.54 / 0.53 | 0.29 / 0.42 | 0.43 / 0.56 |
| coord.max_simultaneous_chasers | 4 / 5 | 5 / 4 | 5 / 4 |
| gk.catches | 0 / 3 | 4 / 2 | 1 / 2 |
| gk.claims_non_save | 0 / 0 | 2 / 0 | 1 / 0 |
| gk.excursion_mean_s | 0 / 0 | 1.12 / 0 | 0 / 1.17 |
| gk.excursions_gt1s_per_10min | 0 / 0 | 1.00 / 0 | 0 / 1 |
| gk.gathers | 0 / 3 | 6 / 2 | 2 / 2 |
| gk.goals_conceded | 7 / 1 | 2 / 5 | 6 / 2 |
| gk.goals_conceded_gk_gt3m_off_line | 3 / 1 | 2 / 2 | 4 / 1 |
| gk.goals_conceded_no_shot_found | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.line_dist_at_conceded_shot_mean_m | 2.42 / 3.24 | 3.12 / 2.91 | 2.56 / 3.06 |
| gk.line_dist_max_m | 5.39 / 5.31 | 7.30 / 5.45 | 4.89 / 8.85 |
| gk.line_dist_mean_m | 2.93 / 2.97 | 2.99 / 2.86 | 2.86 / 2.98 |
| gk.parries | 18 / 27 | 16 / 23 | 22 / 11 |
| gk.pct_outside_6yd | 0 / 0 | 0.19 / 0 | 0 / 0.19 |
| gk.pct_outside_box | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_count | 0 / 0 | 1 / 0 | 0 / 1 |
| gk.rush_per_10min | 0 / 0 | 1.00 / 0 | 0 / 1 |
| gk.saves | 18 / 30 | 20 / 25 | 23 / 13 |
| gk.tips_over | 0 / 0 | 0 / 0 | 0 / 0 |
| match.fouls | 0 / 0 | 0 / 0 | 0 / 0 |
| match.goals | 1 / 7 | 5 / 2 | 2 / 6 |
| match.offsides | 1 / 2 | 1 / 1 | 0 / 1 |
| match.passes_completed_pct | 85.61 / 86.70 | 84.92 / 84.72 | 85.91 / 83.91 |
| match.possession_pct | 45.19 / 54.81 | 58.53 / 41.47 | 43.75 / 56.25 |
| match.shots | 33 / 30 | 34 / 24 | 21 / 35 |
| match.shots_on_target | 30 / 24 | 30 / 22 | 14 / 29 |
| move.accel_from_pos_mean_mps2 | 10.54 / 10.87 | 10.94 / 11.14 | 10.72 / 10.82 |
| move.accel_gt10_pct | 49.66 / 49.57 | 50.04 / 51.51 | 49.07 / 49.74 |
| move.accel_mean_mps2 | 10.53 / 10.87 | 10.89 / 11.10 | 10.69 / 10.80 |
| move.class_time_pct.0 | 11.31 / 13.32 | 13.57 / 11.04 | 12.01 / 13.09 |
| move.class_time_pct.1 | 8.39 / 5.84 | 5.83 / 8.34 | 7.80 / 6.20 |
| move.class_time_pct.2 | 65.34 / 65.73 | 66.30 / 66.53 | 66.24 / 66.00 |
| move.class_time_pct.3 | 14.97 / 15.10 | 14.30 / 14.10 | 13.95 / 14.72 |
| move.dspeed_max_mps_per_tick | 3.42 / 3.68 | 5.12 / 4.61 | 4.62 / 4.01 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.36 / 3.49 | 3.42 / 3.46 | 3.37 / 3.42 |
| move.heading_change_pct.0_10 | 99.56 / 99.39 | 99.44 / 99.45 | 99.46 / 99.41 |
| move.heading_change_pct.10_45 | 0.43 / 0.59 | 0.54 / 0.53 | 0.51 / 0.57 |
| move.heading_change_pct.45_90 | 0.01 / 0.02 | 0.02 / 0.02 | 0.03 / 0.02 |
| move.heading_change_pct.gt90 | 0 / 0 | 0.00 / 0 | 0.00 / 0.00 |
| move.heading_sign_flips_per_player_min | 16.42 / 16.30 | 16.49 / 16.03 | 15.34 / 16.32 |
| move.lateral_accel_mean_mps2 | 8.08 / 8.04 | 8.08 / 8.23 | 7.83 / 8.12 |
| move.reversals_per_player_min | 0 / 0 | 0.08 / 0 | 0.02 / 0.02 |
| move.side_flips_by_class.0_per_player_min | 110.9 / 111.7 | 108.9 / 100.1 | 105.6 / 94.09 |
| move.side_flips_by_class.0_share_pct | 34.68 / 41.32 | 39.69 / 31.66 | 37.77 / 36.50 |
| move.side_flips_by_class.1_per_player_min | 43.60 / 37.63 | 45.87 / 44.47 | 39.26 / 40.38 |
| move.side_flips_by_class.1_share_pct | 10.10 / 6.10 | 7.18 / 10.63 | 9.13 / 7.42 |
| move.side_flips_by_class.2_per_player_min | 25.41 / 24.07 | 24.87 / 25.39 | 22.67 / 24.09 |
| move.side_flips_by_class.2_share_pct | 45.89 / 43.93 | 44.30 / 48.40 | 44.75 / 47.12 |
| move.side_flips_by_class.3_per_player_min | 22.56 / 20.63 | 23.00 / 23.06 | 20.09 / 20.54 |
| move.side_flips_by_class.3_share_pct | 9.33 / 8.65 | 8.84 / 9.31 | 8.35 / 8.96 |
| move.side_flips_cause.desired_per_player_min | 14.17 / 15.96 | 16.37 / 12.30 | 14.07 / 12.89 |
| move.side_flips_cause.filter_per_player_min | 19.02 / 16.96 | 17.67 / 19.42 | 16.12 / 17.32 |
| move.side_flips_cause.limiter_per_player_min | 1.42 / 1.54 | 1.75 / 1.77 | 1.42 / 1.90 |
| move.side_flips_cause.sampler_per_player_min | 1.58 / 1.56 | 1.44 / 1.40 | 1.94 / 1.62 |
| move.side_flips_per_player_min | 36.18 / 36.02 | 37.23 / 34.90 | 33.55 / 33.73 |
| move.side_flips_schmitt_per_player_min | 40.66 / 39.88 | 40.50 / 41.46 | 37.86 / 38.58 |
| move.side_flips_speed_ge3_per_player_min | 48.10 / 46.72 | 49.01 / 44.51 | 42.99 / 43.57 |
| move.side_flips_speed_lt3_per_player_min | 10.18 / 11.06 | 10.13 / 12.58 | 11.59 / 11.68 |
| move.speed_oscillation_pct | 4.39 / 4.26 | 4.32 / 4.43 | 4.52 / 4.42 |
| move.turn_rate_mean_deg_per_s | 98.24 / 98.48 | 97.91 / 101.2 | 95.52 / 97.91 |
| shape.attacking.anchor_dist_m.defender | 8.30 / 8.65 | 10.70 / 8.48 | 7.85 / 10.68 |
| shape.attacking.anchor_dist_m.forward | 16.28 / 17.08 | 15.20 / 18.07 | 17.81 / 16.56 |
| shape.attacking.anchor_dist_m.goalkeeper | 1.32 / 1.27 | 1.23 / 1.41 | 1.34 / 1.24 |
| shape.attacking.anchor_dist_m.midfielder | 17.05 / 17.34 | 16.90 / 14.90 | 18.59 / 16.48 |
| shape.attacking.anchor_dist_shape_m.defender | 8.14 / 7.64 | 8.10 / 8.15 | 7.83 / 9.09 |
| shape.attacking.anchor_dist_shape_m.forward | 10.76 / 10.89 | 10.45 / 10.65 | 11.01 / 12.60 |
| shape.attacking.anchor_dist_shape_m.midfielder | 11.21 / 10.99 | 10.17 / 8.93 | 12.86 / 10.42 |
| shape.attacking.bbox_area_m2 | 318.0 / 336.7 | 342.3 / 286.1 | 357.9 / 304.8 |
| shape.attacking.bbox_depth_m | 18.57 / 18.61 | 19.24 / 16.27 | 18.40 / 17.89 |
| shape.attacking.bbox_width_m | 16.06 / 16.47 | 16.53 / 16.07 | 18.20 / 15.48 |
| shape.attacking.length_m | 18.57 / 18.61 | 19.24 / 16.27 | 18.40 / 17.89 |
| shape.attacking.nearest_mate_mean_m | 9.97 / 10.16 | 10.22 / 9.47 | 10.63 / 9.68 |
| shape.attacking.nearest_mate_p10_m | 3.33 / 3.54 | 3.37 / 3.40 | 3.60 / 3.10 |
| shape.attacking.pct_of_open_play | 43.82 / 56.18 | 55.19 / 44.81 | 49.44 / 50.56 |
| shape.attacking.stretch_m | 9.12 / 9.19 | 9.42 / 8.46 | 9.53 / 8.77 |
| shape.attacking.width_m | 16.06 / 16.47 | 16.53 / 16.07 | 18.20 / 15.48 |
| shape.defending.anchor_dist_m.defender | 7.94 / 8.02 | 9.04 / 8.09 | 9.07 / 7.97 |
| shape.defending.anchor_dist_m.forward | 15.88 / 16.47 | 14.72 / 17.09 | 18.17 / 15.21 |
| shape.defending.anchor_dist_m.goalkeeper | 1.44 / 1.40 | 1.36 / 1.56 | 1.60 / 1.39 |
| shape.defending.anchor_dist_m.midfielder | 16.12 / 15.68 | 15.05 / 13.36 | 16.89 / 14.45 |
| shape.defending.anchor_dist_shape_m.defender | 8.87 / 9.24 | 9.33 / 9.09 | 10.26 / 8.64 |
| shape.defending.anchor_dist_shape_m.forward | 9.37 / 10.17 | 9.71 / 9.89 | 12.08 / 9.16 |
| shape.defending.anchor_dist_shape_m.midfielder | 10.88 / 10.12 | 9.13 / 8.27 | 12.26 / 8.44 |
| shape.defending.bbox_area_m2 | 359.2 / 355.4 | 380.2 / 351.2 | 321.0 / 361.7 |
| shape.defending.bbox_depth_m | 19.89 / 18.62 | 19.63 / 18.76 | 17.16 / 20.13 |
| shape.defending.bbox_width_m | 16.74 / 17.54 | 18.07 / 17.44 | 17.59 / 16.91 |
| shape.defending.length_m | 19.89 / 18.62 | 19.63 / 18.76 | 17.16 / 20.13 |
| shape.defending.line_x_std_m | 6.32 / 5.80 | 6.54 / 5.64 | 5.48 / 6.41 |
| shape.defending.line_x_variance_m2 | 64.24 / 55.10 | 68.16 / 51.24 | 50.79 / 65.35 |
| shape.defending.nearest_mate_mean_m | 10.55 / 10.65 | 10.85 / 10.54 | 10.01 / 10.78 |
| shape.defending.nearest_mate_p10_m | 3.73 / 3.93 | 3.49 / 3.57 | 3.50 / 3.99 |
| shape.defending.pct_of_open_play | 56.18 / 43.82 | 44.81 / 55.19 | 50.56 / 49.44 |
| shape.defending.pct_outfield_behind_ball | 68.73 / 62.28 | 65.45 / 64.31 | 62.11 / 70.86 |
| shape.defending.stretch_m | 9.67 / 9.42 | 9.90 / 9.51 | 9.06 / 9.77 |
| shape.defending.swarm2_pct | 1.25 / 0.41 | 0.24 / 0.29 | 0.57 / 0.48 |
| shape.defending.swarm3_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| shape.defending.width_m | 16.74 / 17.54 | 18.07 / 17.44 | 17.59 / 16.91 |
| support.carrier_pct_of_open_play | 19.00 / 22.77 | 23.24 / 16.79 | 17.84 / 22.93 |
| support.cluster_pct | 4.14 / 7.91 | 6.00 / 5.90 | 6.32 / 13.77 |
| support.depth_m | 20.41 / 19.95 | 20.90 / 18.26 | 19.49 / 18.62 |
| support.forward_options_mean | 0.13 / 0.27 | 0.34 / 0.14 | 0.28 / 0.15 |
| support.nearest_option_dist_m | 12.34 / 11.91 | 12.48 / 11.76 | 12.25 / 11.68 |
| support.open_options_mean | 1.24 / 1.25 | 1.36 / 1.39 | 1.36 / 1.17 |
| support.open_options_p10 | 0 / 0 | 0 / 0 | 0 / 0 |
| support.pass_target_open_pct | 64.65 / 74.65 | 73.77 / 77.23 | 65.14 / 66.94 |
| support.passes_completed_tracked | 99 / 142 | 122 / 101 | 109 / 121 |
| support.release_pass_completed_pct | 88.89 / 100 | 81.25 / 71.43 | 100 / 84.62 |
| support.releases_per_10min | 8.98 / 7.98 | 15.93 / 6.97 | 11 / 13 |
| support.run_abandon_per_10min | 0 / 0 | 0 / 0 | 1 / 0 |
| support.runs_in_behind_per_10min | 2.00 / 2.00 | 1.99 / 3.98 | 2 / 7 |
| support.ticks_with_le1_option_pct | 57.14 / 58.61 | 53.84 / 51.03 | 52.65 / 61.47 |
| support.width_m | 16.08 / 16.14 | 16.47 / 15.73 | 17.54 / 14.80 |
| trans.count_per_10min | 27.94 / 17.96 | 24.90 / 29.88 | 18 / 29 |
| trans.counterpress_pct | 39.29 / 50 | 52 / 36.67 | 44.44 / 31.03 |
| trans.forward_recovery_speed_mps_0_5s | 6.57 / 6.70 | 6.49 / 6.49 | 6.53 / 6.82 |
| trans.forward_sprint_share_0_5s | 63.25 / 66.27 | 60.57 / 61.07 | 63.93 / 66.53 |
| trans.forward_sprint_share_0_5s_7mps | 46.46 / 52.05 | 45.28 / 46.75 | 46.92 / 51.36 |
| trans.forwards_staying_high_mean | 0.93 / 0.83 | 0.75 / 0.73 | 0.75 / 1 |
| trans.in_shape_at_turnover_pct | 66.67 / 50 | 72.73 / 64.29 | 66.67 / 51.85 |
| trans.mid_recovery_speed_mps_0_5s | 6.12 / 6.50 | 6.53 / 6.21 | 6.45 / 6.70 |
| trans.mid_sprint_share_0_5s | 57.20 / 63.57 | 66.19 / 59.95 | 63.72 / 67.11 |
| trans.mid_sprint_share_0_5s_7mps | 46.96 / 51.60 | 53.39 / 45.68 | 51.17 / 55.46 |
| trans.outfield_stamina_end_permille | 913.2 / 786.8 | 852 / 851.8 | 803.5 / 805.2 |
| trans.pct_outfield_behind_ball_at_3s | 65.91 / 47.73 | 60.29 / 61.96 | 59.62 / 70 |
| trans.pct_outfield_behind_ball_at_6s | 64.29 / 54.17 | 59.38 / 72.73 | 71.88 / 52.27 |
| trans.time_to_shape_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_excluded_pct | 3.57 / 22.22 | 12 / 6.67 | 16.67 / 6.90 |
| trans.time_to_shape_s | 0.51 / 0.95 | 0.79 / 0.99 | 0.96 / 0.73 |
| trans.time_to_shape_static_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_static_excluded_pct | 14.29 / 55.56 | 32 / 30 | 38.89 / 17.24 |
| trans.time_to_shape_static_s | 0.42 / 0.04 | 0.14 / 0.30 | 0.23 / 0.45 |

GK scenario field: not present in the public state.

## 11v11 normal vs normal (10 min x 3 seeds from 1)

Command: `psfc-replay analyze --format 11v11 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty normal --away-difficulty normal`

Gameplay version: `0.1.0-5v5`

| seed | final state_hash | score (H-A) |
|---|---|---|
| 1 | `dc717cf343a3c18c` | 2-2 |
| 2 | `09ae3df82c05fc9b` | 1-3 |
| 3 | `21579bbc457c298b` | 2-1 |

Match meta (mean over seeds): goals_total = 3.67, open_play_minutes = 10, open_play_pct_of_ticks = 97.43, shots_total = 21.33, ticks_simulated = 36948.7

### Match

| metric | home | away | teams mean |
|---|---:|---:|---:|
| fouls | 0 | 0 | **0** |
| goals | 1.67 | 2 | **1.83** |
| offsides | 0 | 0 | **0** |
| passes_completed_pct | 81.95 | 79.29 | **80.62** |
| possession_pct | 49.71 | 50.29 | **50** |
| shots | 9.33 | 12 | **10.67** |
| shots_on_target | 7.67 | 10.33 | **9** |

### Goalkeeper

| metric | home | away | teams mean |
|---|---:|---:|---:|
| catches | 1.67 | 0.33 | **1** |
| claims_non_save | 0 | 0.67 | **0.33** |
| excursion_mean_s | 0.84 | 0.73 | **0.78** |
| excursions_gt1s_per_10min | 1 | 1.33 | **1.17** |
| gathers | 1.67 | 1 | **1.33** |
| goals_conceded | 2 | 1.67 | **1.83** |
| goals_conceded_gk_gt3m_off_line | 2 | 1 | **1.50** |
| goals_conceded_no_shot_found | 0 | 0 | **0** |
| line_dist_at_conceded_shot_mean_m | 4.43 | 3.53 | **3.98** |
| line_dist_max_m | 7.48 | 9.18 | **8.33** |
| line_dist_mean_m | 4.44 | 4.59 | **4.51** |
| parries | 7 | 5.67 | **6.33** |
| pct_outside_6yd | 0.52 | 0.49 | **0.51** |
| pct_outside_box | 0 | 0 | **0** |
| rush_count | 4 | 2.67 | **3.33** |
| rush_per_10min | 4 | 2.67 | **3.33** |
| saves | 8.67 | 6 | **7.33** |
| tips_over | 0 | 0 | **0** |

### Defence and shape

| metric | home | away | teams mean |
|---|---:|---:|---:|
| attacking.anchor_dist_m.defender | 19.51 | 23.00 | **21.26** |
| attacking.anchor_dist_m.forward | 26.29 | 24.35 | **25.32** |
| attacking.anchor_dist_m.goalkeeper | 2.22 | 2.27 | **2.24** |
| attacking.anchor_dist_m.midfielder | 18.91 | 18.35 | **18.63** |
| attacking.anchor_dist_shape_m.defender | 16.96 | 18.19 | **17.57** |
| attacking.anchor_dist_shape_m.forward | 19.59 | 19.76 | **19.67** |
| attacking.anchor_dist_shape_m.midfielder | 12.44 | 12.19 | **12.31** |
| attacking.bbox_area_m2 | 1845.5 | 1865.1 | **1855.3** |
| attacking.bbox_depth_m | 40.89 | 40.56 | **40.72** |
| attacking.bbox_width_m | 43.97 | 44.76 | **44.37** |
| attacking.length_m | 40.89 | 40.56 | **40.72** |
| attacking.nearest_mate_mean_m | 11.61 | 11.69 | **11.65** |
| attacking.nearest_mate_p10_m | 5.40 | 5.38 | **5.39** |
| attacking.pct_of_open_play | 51.96 | 48.04 | **50** |
| attacking.stretch_m | 18.82 | 19.03 | **18.92** |
| attacking.width_m | 43.97 | 44.76 | **44.37** |
| defending.anchor_dist_m.defender | 19.16 | 22.71 | **20.93** |
| defending.anchor_dist_m.forward | 27.89 | 25.93 | **26.91** |
| defending.anchor_dist_m.goalkeeper | 2.54 | 2.50 | **2.52** |
| defending.anchor_dist_m.midfielder | 18.95 | 18.36 | **18.65** |
| defending.anchor_dist_shape_m.defender | 17.91 | 18.87 | **18.39** |
| defending.anchor_dist_shape_m.forward | 20.24 | 20.20 | **20.22** |
| defending.anchor_dist_shape_m.midfielder | 12.29 | 11.86 | **12.08** |
| defending.bbox_area_m2 | 1463.4 | 1488.5 | **1476.0** |
| defending.bbox_depth_m | 36.96 | 36.93 | **36.95** |
| defending.bbox_width_m | 38.00 | 38.36 | **38.18** |
| defending.length_m | 36.96 | 36.93 | **36.95** |
| defending.line_x_std_m | 5.84 | 5.75 | **5.80** |
| defending.line_x_variance_m2 | 61.56 | 60.24 | **60.90** |
| defending.nearest_mate_mean_m | 10.29 | 10.39 | **10.34** |
| defending.nearest_mate_p10_m | 5.34 | 5.17 | **5.26** |
| defending.pct_of_open_play | 48.04 | 51.96 | **50** |
| defending.pct_outfield_behind_ball | 69.42 | 74.03 | **71.73** |
| defending.stretch_m | 16.67 | 16.72 | **16.70** |
| defending.swarm2_pct | 0.21 | 0.89 | **0.55** |
| defending.swarm3_pct | 0 | 0 | **0** |
| defending.width_m | 38.00 | 38.36 | **38.18** |

### Defensive transition

| metric | home | away | teams mean |
|---|---:|---:|---:|
| count_per_10min | 16.67 | 17.33 | **17** |
| counterpress_pct | 48.15 | 41.72 | **44.93** |
| forward_recovery_speed_mps_0_5s | 7.22 | 7.36 | **7.29** |
| forward_sprint_share_0_5s | 71.73 | 74.31 | **73.02** |
| forward_sprint_share_0_5s_7mps | 60.33 | 64.23 | **62.28** |
| forwards_staying_high_mean | 1.58 | 1.67 | **1.63** |
| in_shape_at_turnover_pct | 72.08 | 60.69 | **66.39** |
| mid_recovery_speed_mps_0_5s | 6.60 | 6.63 | **6.61** |
| mid_sprint_share_0_5s | 63.72 | 62.78 | **63.25** |
| mid_sprint_share_0_5s_7mps | 51.59 | 50.71 | **51.15** |
| outfield_stamina_end_permille | 887.6 | 887.8 | **887.7** |
| pct_outfield_behind_ball_at_3s | 64.49 | 73.49 | **68.99** |
| pct_outfield_behind_ball_at_6s | 78.33 | 75.77 | **77.05** |
| time_to_shape_capped_pct | 0 | 0 | **0** |
| time_to_shape_excluded_pct | 24.54 | 10.82 | **17.68** |
| time_to_shape_s | 0.65 | 1.40 | **1.03** |
| time_to_shape_static_capped_pct | 0 | 0 | **0** |
| time_to_shape_static_excluded_pct | 34.72 | 18.13 | **26.43** |
| time_to_shape_static_s | 0.30 | 1.20 | **0.75** |

### Attacking support

| metric | home | away | teams mean |
|---|---:|---:|---:|
| carrier_pct_of_open_play | 15.76 | 16.09 | **15.92** |
| cluster_pct | 9.12 | 6.60 | **7.86** |
| depth_m | 39.75 | 38.84 | **39.29** |
| forward_options_mean | 0.75 | 0.53 | **0.64** |
| nearest_option_dist_m | 12.26 | 13.56 | **12.91** |
| open_options_mean | 3.54 | 3.32 | **3.43** |
| open_options_p10 | 0.33 | 0 | **0.17** |
| pass_target_open_pct | 72.84 | 71.15 | **71.99** |
| passes_completed_tracked | 112.7 | 99 | **105.8** |
| release_pass_completed_pct | 66.67 | 59.52 | **63.10** |
| releases_per_10min | 3.33 | 5 | **4.17** |
| run_abandon_per_10min | 0 | 0 | **0** |
| runs_in_behind_per_10min | 0.67 | 1.33 | **1** |
| ticks_with_le1_option_pct | 22.89 | 24.80 | **23.85** |
| width_m | 43.32 | 43.90 | **43.61** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 9.58 | 9.42 | **9.50** |
| accel_gt10_pct | 45.79 | 45.06 | **45.43** |
| accel_mean_mps2 | 9.56 | 9.40 | **9.48** |
| class_time_pct.0 | 6.20 | 5.89 | **6.05** |
| class_time_pct.1 | 2.73 | 2.99 | **2.86** |
| class_time_pct.2 | 71.55 | 72.17 | **71.86** |
| class_time_pct.3 | 19.52 | 18.95 | **19.23** |
| dspeed_max_mps_per_tick | 4.33 | 4.02 | **4.17** |
| heading_change_p95_deg_per_tick_gt3mps | 3.08 | 3.09 | **3.09** |
| heading_change_pct.0_10 | 99.75 | 99.75 | **99.75** |
| heading_change_pct.10_45 | 0.24 | 0.24 | **0.24** |
| heading_change_pct.45_90 | 0.01 | 0.01 | **0.01** |
| heading_change_pct.gt90 | 0.00 | 0.00 | **0.00** |
| heading_sign_flips_per_player_min | 9.33 | 8.96 | **9.15** |
| lateral_accel_mean_mps2 | 7.02 | 7.04 | **7.03** |
| reversals_per_player_min | 0.02 | 0.02 | **0.02** |
| side_flips_by_class.0_per_player_min | 95.17 | 95.35 | **95.26** |
| side_flips_by_class.0_share_pct | 26.74 | 25.80 | **26.27** |
| side_flips_by_class.1_per_player_min | 43.98 | 50.44 | **47.21** |
| side_flips_by_class.1_share_pct | 5.45 | 6.91 | **6.18** |
| side_flips_by_class.2_per_player_min | 16.15 | 15.79 | **15.97** |
| side_flips_by_class.2_share_pct | 52.39 | 52.35 | **52.37** |
| side_flips_by_class.3_per_player_min | 17.42 | 17.17 | **17.29** |
| side_flips_by_class.3_share_pct | 15.42 | 14.93 | **15.18** |
| side_flips_cause.desired_per_player_min | 6.60 | 6.15 | **6.38** |
| side_flips_cause.filter_per_player_min | 13.75 | 13.84 | **13.79** |
| side_flips_cause.limiter_per_player_min | 0.89 | 0.87 | **0.88** |
| side_flips_cause.sampler_per_player_min | 0.81 | 0.90 | **0.86** |
| side_flips_per_player_min | 22.06 | 21.76 | **21.91** |
| side_flips_schmitt_per_player_min | 34.79 | 34.02 | **34.40** |
| side_flips_speed_ge3_per_player_min | 25.07 | 24.75 | **24.91** |
| side_flips_speed_lt3_per_player_min | 10.36 | 10.07 | **10.21** |
| speed_oscillation_pct | 3.18 | 3.22 | **3.20** |
| turn_rate_mean_deg_per_s | 80.88 | 80.41 | **80.65** |

### Collisions and push-out

| metric | home | away | teams mean |
|---|---:|---:|---:|
| any.depth_max_m | 0.79 | 0.79 | **0.79** |
| any.depth_mean_m | 0.20 | 0.20 | **0.20** |
| any.events_per_min | 2.43 | 2.43 | **2.43** |
| any.stuck_mean_ticks | 8 | 8 | **8** |
| any.stuck_pairs | 0.67 | 0.67 | **0.67** |
| any.ticks_gt1cm_per_min | 4.93 | 4.93 | **4.93** |
| any.ticks_per_min | 5.77 | 5.77 | **5.77** |
| correction_vel_corrected_p50_mps | 0.99 | 1.11 | **1.05** |
| correction_vel_corrected_p99_mps | 9.91 | 12.87 | **11.39** |
| correction_vel_corrected_pct | 0.04 | 0.04 | **0.04** |
| correction_vel_max_mps | 13.40 | 13.21 | **13.31** |
| correction_vel_p50_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p95_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p99_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_pct_gt_0_3 | 0.03 | 0.03 | **0.03** |
| opponent.depth_max_m | 0.22 | 0.22 | **0.22** |
| opponent.depth_mean_m | 0.06 | 0.06 | **0.06** |
| opponent.events_per_min | 1.93 | 1.93 | **1.93** |
| opponent.stuck_mean_ticks | 0 | 0 | **0** |
| opponent.stuck_pairs | 0 | 0 | **0** |
| opponent.ticks_gt1cm_per_min | 2.70 | 2.70 | **2.70** |
| opponent.ticks_per_min | 3.27 | 3.27 | **3.27** |
| teammate.depth_max_m | 0.76 | 0.79 | **0.78** |
| teammate.depth_mean_m | 0.33 | 0.37 | **0.35** |
| teammate.events_per_min | 0.30 | 0.20 | **0.25** |
| teammate.stuck_mean_ticks | 4 | 4 | **4** |
| teammate.stuck_pairs | 0.33 | 0.33 | **0.33** |
| teammate.ticks_gt1cm_per_min | 1.40 | 1.47 | **1.43** |
| teammate.ticks_per_min | 1.53 | 1.60 | **1.57** |

### Coordination

| metric | home | away | teams mean |
|---|---:|---:|---:|
| cruise_over72_pct | 43.24 | 43.66 | **43.45** |
| dual_chaser_pct_of_loose | 14.66 | 12.97 | **13.82** |
| dual_chaser_ticks_per_min | 357.8 | 316.3 | **337.1** |
| engagers3_pct_of_carrier_ticks | 0 | 0.11 | **0.05** |
| engagers3_ticks_per_min | 0 | 0.73 | **0.37** |
| mark_steal_per_defender_min | 0.19 | 0.21 | **0.20** |
| max_simultaneous_chasers | 4 | 4 | **4** |

### Aerial

| metric | home | away | teams mean |
|---|---:|---:|---:|
| headers | 17 | 13 | **15** |
| headers_clear | 2 | 0 | **1** |
| headers_pass | 14.33 | 11.67 | **13** |
| headers_shot | 0.67 | 1.33 | **1** |
| jumps_gk | 10.33 | 7.67 | **9** |
| jumps_outfield | 36.33 | 31.33 | **33.83** |
| max_airborne_outfield | 1 | 1 | **1** |

### Team shape vs envelope (teams mean)

| metric | in possession | out of possession | 11v11 envelope (m) |
|---|---:|---:|---|
| length_m | 40.72 | 36.95 | 31-46 |
| width_m | 44.37 | 38.18 | 35-48 |
| stretch_m | 18.92 (outside) | 16.70 (outside) | 7-16 |
| nearest_mate_mean_m | 11.65 | 10.34 | none published (in-possession mean near 9-10 m, estimate) |
| nearest_mate_p10_m | 5.39 | 5.26 | none published (in-possession mean near 9-10 m, estimate) |

Envelope: peer-reviewed 11v11 ranges (length 31-46 m, width 35-48 m, stretch index 7-16 m) quoted in the research report; the report's tighter targets are 31-36 / 35-40 / 7-10 m out of possession and 40-46 / 42-48 / 12-16 m in possession (estimates).

### Per seed (home / away)

| metric | seed 1 | seed 2 | seed 3 |
|---|---:|---:|---:|
| aerial.headers | 18 / 11 | 16 / 13 | 17 / 15 |
| aerial.headers_clear | 1 / 0 | 4 / 0 | 1 / 0 |
| aerial.headers_pass | 16 / 10 | 12 / 11 | 15 / 14 |
| aerial.headers_shot | 1 / 1 | 0 / 2 | 1 / 1 |
| aerial.jumps_gk | 11 / 6 | 15 / 5 | 5 / 12 |
| aerial.jumps_outfield | 38 / 31 | 34 / 30 | 37 / 33 |
| aerial.max_airborne_outfield | 1 / 1 | 1 / 1 | 1 / 1 |
| collision.any.depth_max_m | 0.80 / 0.80 | 0.77 / 0.77 | 0.80 / 0.80 |
| collision.any.depth_mean_m | 0.20 / 0.20 | 0.20 / 0.20 | 0.21 / 0.21 |
| collision.any.events_per_min | 2.20 / 2.20 | 1.80 / 1.80 | 3.30 / 3.30 |
| collision.any.stuck_mean_ticks | 0 / 0 | 12 / 12 | 12 / 12 |
| collision.any.stuck_pairs | 0 / 0 | 1 / 1 | 1 / 1 |
| collision.any.ticks_gt1cm_per_min | 4.80 / 4.80 | 2.40 / 2.40 | 7.60 / 7.60 |
| collision.any.ticks_per_min | 5.50 / 5.50 | 3.10 / 3.10 | 8.70 / 8.70 |
| collision.correction_vel_corrected_p50_mps | 1.12 / 0.65 | 0.64 / 0.89 | 1.20 / 1.80 |
| collision.correction_vel_corrected_p99_mps | 12.43 / 13.12 | 13.70 / 12.22 | 3.60 / 13.27 |
| collision.correction_vel_corrected_pct | 0.03 / 0.04 | 0.04 / 0.04 | 0.04 / 0.05 |
| collision.correction_vel_max_mps | 12.88 / 13.72 | 14.47 / 12.47 | 12.86 / 13.44 |
| collision.correction_vel_p50_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p95_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p99_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_pct_gt_0_3 | 0.02 / 0.03 | 0.03 / 0.03 | 0.03 / 0.04 |
| collision.opponent.depth_max_m | 0.12 / 0.12 | 0.10 / 0.10 | 0.45 / 0.45 |
| collision.opponent.depth_mean_m | 0.04 / 0.04 | 0.03 / 0.03 | 0.10 / 0.10 |
| collision.opponent.events_per_min | 1.80 / 1.80 | 1.40 / 1.40 | 2.60 / 2.60 |
| collision.opponent.stuck_mean_ticks | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.stuck_pairs | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.ticks_gt1cm_per_min | 2.30 / 2.30 | 1.10 / 1.10 | 4.70 / 4.70 |
| collision.opponent.ticks_per_min | 2.70 / 2.70 | 1.70 / 1.70 | 5.40 / 5.40 |
| collision.teammate.depth_max_m | 0.80 / 0.80 | 0.73 / 0.77 | 0.75 / 0.80 |
| collision.teammate.depth_mean_m | 0.34 / 0.36 | 0.31 / 0.39 | 0.35 / 0.36 |
| collision.teammate.events_per_min | 0.30 / 0.10 | 0.30 / 0.10 | 0.30 / 0.40 |
| collision.teammate.stuck_mean_ticks | 0 / 0 | 12 / 0 | 0 / 12 |
| collision.teammate.stuck_pairs | 0 / 0 | 1 / 0 | 0 / 1 |
| collision.teammate.ticks_gt1cm_per_min | 1.70 / 0.80 | 1.30 / 0.70 | 1.20 / 2.90 |
| collision.teammate.ticks_per_min | 1.90 / 0.90 | 1.40 / 0.70 | 1.30 / 3.20 |
| coord.cruise_over72_pct | 42.99 / 41.85 | 43.41 / 43.51 | 43.31 / 45.60 |
| coord.dual_chaser_pct_of_loose | 15.53 / 10.53 | 14.91 / 12.00 | 13.54 / 16.39 |
| coord.dual_chaser_ticks_per_min | 376.6 / 255.4 | 369 / 296.8 | 327.8 / 396.7 |
| coord.engagers3_pct_of_carrier_ticks | 0 / 0 | 0 / 0 | 0 / 0.32 |
| coord.engagers3_ticks_per_min | 0 / 0 | 0 / 0 | 0 / 2.20 |
| coord.mark_steal_per_defender_min | 0.44 / 0.14 | 0.12 / 0.28 | 0 / 0.21 |
| coord.max_simultaneous_chasers | 5 / 4 | 4 / 4 | 3 / 4 |
| gk.catches | 1 / 1 | 3 / 0 | 1 / 0 |
| gk.claims_non_save | 0 / 0 | 0 / 2 | 0 / 0 |
| gk.excursion_mean_s | 0.81 / 0 | 0.72 / 1.18 | 0.98 / 1 |
| gk.excursions_gt1s_per_10min | 2 / 0 | 1 / 2 | 0 / 2 |
| gk.gathers | 1 / 1 | 3 / 2 | 1 / 0 |
| gk.goals_conceded | 2 / 2 | 3 / 1 | 1 / 2 |
| gk.goals_conceded_gk_gt3m_off_line | 2 / 1 | 3 / 0 | 1 / 2 |
| gk.goals_conceded_no_shot_found | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.line_dist_at_conceded_shot_mean_m | 5.28 / 3.35 | 3.92 / 2.58 | 4.10 / 4.66 |
| gk.line_dist_max_m | 7.69 / 5.27 | 7.54 / 14.61 | 7.20 / 7.66 |
| gk.line_dist_mean_m | 4.44 / 4.68 | 4.31 / 4.63 | 4.56 / 4.46 |
| gk.parries | 8 / 4 | 12 / 4 | 1 / 9 |
| gk.pct_outside_6yd | 0.81 / 0 | 0.60 / 0.98 | 0.16 / 0.50 |
| gk.pct_outside_box | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_count | 6 / 0 | 5 / 5 | 1 / 3 |
| gk.rush_per_10min | 6 / 0 | 5 / 5 | 1 / 3 |
| gk.saves | 9 / 5 | 15 / 4 | 2 / 9 |
| gk.tips_over | 0 / 0 | 0 / 0 | 0 / 0 |
| match.fouls | 0 / 0 | 0 / 0 | 0 / 0 |
| match.goals | 2 / 2 | 1 / 3 | 2 / 1 |
| match.offsides | 0 / 0 | 0 / 0 | 0 / 0 |
| match.passes_completed_pct | 84.91 / 78.38 | 80.95 / 80.18 | 80 / 79.31 |
| match.possession_pct | 43.86 / 56.14 | 46.33 / 53.67 | 58.95 / 41.05 |
| match.shots | 8 / 15 | 6 / 18 | 14 / 3 |
| match.shots_on_target | 7 / 11 | 5 / 17 | 11 / 3 |
| move.accel_from_pos_mean_mps2 | 9.75 / 9.50 | 9.65 / 9.36 | 9.33 / 9.41 |
| move.accel_gt10_pct | 47.24 / 45.90 | 45.99 / 44.94 | 44.13 / 44.35 |
| move.accel_mean_mps2 | 9.74 / 9.48 | 9.63 / 9.35 | 9.32 / 9.39 |
| move.class_time_pct.0 | 5.96 / 5.79 | 6.40 / 5.94 | 6.25 / 5.94 |
| move.class_time_pct.1 | 3.03 / 2.97 | 2.56 / 2.74 | 2.60 / 3.25 |
| move.class_time_pct.2 | 71.99 / 70.66 | 71.89 / 72.44 | 70.77 / 73.42 |
| move.class_time_pct.3 | 19.01 / 20.58 | 19.15 / 18.87 | 20.38 / 17.39 |
| move.dspeed_max_mps_per_tick | 4.52 / 4.59 | 4.68 / 3.74 | 3.78 / 3.73 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.11 / 3.15 | 3.10 / 3.09 | 3.04 / 3.04 |
| move.heading_change_pct.0_10 | 99.77 / 99.76 | 99.70 / 99.76 | 99.77 / 99.73 |
| move.heading_change_pct.10_45 | 0.22 / 0.23 | 0.28 / 0.23 | 0.22 / 0.26 |
| move.heading_change_pct.45_90 | 0.01 / 0.01 | 0.01 / 0.01 | 0.01 / 0.01 |
| move.heading_change_pct.gt90 | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| move.heading_sign_flips_per_player_min | 9.46 / 9.57 | 9.90 / 8.50 | 8.64 / 8.81 |
| move.lateral_accel_mean_mps2 | 7.20 / 7.18 | 6.99 / 7.06 | 6.87 / 6.88 |
| move.reversals_per_player_min | 0.03 / 0.02 | 0.03 / 0.01 | 0.01 / 0.02 |
| move.side_flips_by_class.0_per_player_min | 98.97 / 95.11 | 101.9 / 95.67 | 84.59 / 95.27 |
| move.side_flips_by_class.0_share_pct | 25.90 / 24.99 | 29.51 / 26.43 | 24.83 / 25.99 |
| move.side_flips_by_class.1_per_player_min | 42.87 / 46.55 | 39.06 / 54.74 | 50.02 / 50.03 |
| move.side_flips_by_class.1_share_pct | 5.71 / 6.28 | 4.53 / 6.98 | 6.11 / 7.48 |
| move.side_flips_by_class.2_per_player_min | 16.75 / 16.20 | 15.91 / 15.74 | 15.78 / 15.43 |
| move.side_flips_by_class.2_share_pct | 52.91 / 52.00 | 51.77 / 53.02 | 52.48 / 52.03 |
| move.side_flips_by_class.3_per_player_min | 18.56 / 17.89 | 16.38 / 15.47 | 17.31 / 18.15 |
| move.side_flips_by_class.3_share_pct | 15.48 / 16.73 | 14.20 / 13.57 | 16.58 / 14.50 |
| move.side_flips_cause.desired_per_player_min | 6.78 / 6.09 | 6.89 / 6.18 | 6.14 / 6.18 |
| move.side_flips_cause.filter_per_player_min | 14.40 / 14.15 | 13.41 / 13.69 | 13.44 / 13.67 |
| move.side_flips_cause.limiter_per_player_min | 0.85 / 0.88 | 1.03 / 0.75 | 0.81 / 0.99 |
| move.side_flips_cause.sampler_per_player_min | 0.77 / 0.89 | 0.77 / 0.89 | 0.89 / 0.93 |
| move.side_flips_per_player_min | 22.79 / 22.02 | 22.10 / 21.51 | 21.28 / 21.77 |
| move.side_flips_schmitt_per_player_min | 35.74 / 35.04 | 33.89 / 33.50 | 34.74 / 33.51 |
| move.side_flips_speed_ge3_per_player_min | 25.78 / 24.87 | 25.25 / 24.75 | 24.17 / 24.62 |
| move.side_flips_speed_lt3_per_player_min | 11.03 / 11.28 | 10.41 / 9.14 | 9.63 / 9.79 |
| move.speed_oscillation_pct | 3.21 / 3.24 | 3.15 / 3.12 | 3.17 / 3.30 |
| move.turn_rate_mean_deg_per_s | 82.91 / 83.45 | 81.66 / 80.27 | 78.06 / 77.52 |
| shape.attacking.anchor_dist_m.defender | 21.95 / 23.90 | 16.40 / 21.87 | 20.18 / 23.23 |
| shape.attacking.anchor_dist_m.forward | 28.87 / 20.21 | 26.76 / 22.75 | 23.24 / 30.08 |
| shape.attacking.anchor_dist_m.goalkeeper | 2.20 / 2.24 | 2.25 / 2.31 | 2.20 / 2.26 |
| shape.attacking.anchor_dist_m.midfielder | 19.34 / 19.78 | 20.12 / 17.64 | 17.27 / 17.64 |
| shape.attacking.anchor_dist_shape_m.defender | 20.81 / 17.19 | 13.96 / 16.75 | 16.10 / 20.64 |
| shape.attacking.anchor_dist_shape_m.forward | 21.18 / 16.21 | 18.89 / 18.25 | 18.70 / 24.81 |
| shape.attacking.anchor_dist_shape_m.midfielder | 12.64 / 13.81 | 12.91 / 11.54 | 11.75 / 11.23 |
| shape.attacking.bbox_area_m2 | 1760.3 / 1917.0 | 1869.3 / 1863.7 | 1906.9 / 1814.6 |
| shape.attacking.bbox_depth_m | 39.58 / 41.31 | 41.39 / 41.17 | 41.70 / 39.19 |
| shape.attacking.bbox_width_m | 43.14 / 45.22 | 43.97 / 44.12 | 44.82 / 44.95 |
| shape.attacking.length_m | 39.58 / 41.31 | 41.39 / 41.17 | 41.70 / 39.19 |
| shape.attacking.nearest_mate_mean_m | 11.45 / 11.89 | 11.63 / 11.75 | 11.77 / 11.44 |
| shape.attacking.nearest_mate_p10_m | 5.39 / 5.78 | 5.37 / 5.51 | 5.43 / 4.84 |
| shape.attacking.pct_of_open_play | 51.48 / 48.52 | 53.33 / 46.67 | 51.05 / 48.95 |
| shape.attacking.stretch_m | 18.22 / 19.38 | 18.83 / 18.96 | 19.39 / 18.75 |
| shape.attacking.width_m | 43.14 / 45.22 | 43.97 / 44.12 | 44.82 / 44.95 |
| shape.defending.anchor_dist_m.defender | 20.92 / 23.13 | 16.18 / 22.51 | 20.37 / 22.48 |
| shape.defending.anchor_dist_m.forward | 30.28 / 21.46 | 27.61 / 24.92 | 25.78 / 31.40 |
| shape.defending.anchor_dist_m.goalkeeper | 2.54 / 2.42 | 2.56 / 2.51 | 2.52 / 2.57 |
| shape.defending.anchor_dist_m.midfielder | 19.89 / 19.12 | 18.84 / 18.70 | 18.10 / 17.27 |
| shape.defending.anchor_dist_shape_m.defender | 20.82 / 18.20 | 15.12 / 17.60 | 17.80 / 20.81 |
| shape.defending.anchor_dist_shape_m.forward | 21.15 / 15.91 | 19.71 / 18.78 | 19.87 / 25.93 |
| shape.defending.anchor_dist_shape_m.midfielder | 12.72 / 13.19 | 12.03 / 11.65 | 12.13 / 10.74 |
| shape.defending.bbox_area_m2 | 1405.1 / 1517.0 | 1521.7 / 1544.4 | 1463.5 / 1404.2 |
| shape.defending.bbox_depth_m | 35.89 / 37.74 | 38.06 / 37.69 | 36.92 / 35.37 |
| shape.defending.bbox_width_m | 37.64 / 38.27 | 38.22 / 38.84 | 38.13 / 37.97 |
| shape.defending.length_m | 35.89 / 37.74 | 38.06 / 37.69 | 36.92 / 35.37 |
| shape.defending.line_x_std_m | 5.35 / 5.02 | 6.32 / 6.43 | 5.84 / 5.82 |
| shape.defending.line_x_variance_m2 | 52.95 / 46.67 | 69.42 / 73.09 | 62.30 / 60.95 |
| shape.defending.nearest_mate_mean_m | 10.05 / 10.89 | 10.59 / 10.40 | 10.24 / 9.88 |
| shape.defending.nearest_mate_p10_m | 4.74 / 5.63 | 5.48 / 5.25 | 5.81 / 4.63 |
| shape.defending.pct_of_open_play | 48.52 / 51.48 | 46.67 / 53.33 | 48.95 / 51.05 |
| shape.defending.pct_outfield_behind_ball | 67.93 / 76.95 | 64.96 / 77.08 | 75.39 / 68.08 |
| shape.defending.stretch_m | 16.16 / 17.05 | 17.01 / 17.05 | 16.85 / 16.06 |
| shape.defending.swarm2_pct | 0.11 / 0 | 0.17 / 0 | 0.37 / 2.67 |
| shape.defending.swarm3_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| shape.defending.width_m | 37.64 / 38.27 | 38.22 / 38.84 | 38.13 / 37.97 |
| support.carrier_pct_of_open_play | 14.21 / 18.21 | 13.82 / 16.61 | 19.24 / 13.45 |
| support.cluster_pct | 6.65 / 5.04 | 10.95 / 4.57 | 9.74 / 10.20 |
| support.depth_m | 38.85 / 40.21 | 40.43 / 39.53 | 39.96 / 36.77 |
| support.forward_options_mean | 0.93 / 0.52 | 0.86 / 0.39 | 0.46 / 0.67 |
| support.nearest_option_dist_m | 11.73 / 13.51 | 11.86 / 14.60 | 13.19 / 12.57 |
| support.open_options_mean | 4.15 / 3.30 | 3.71 / 2.97 | 2.75 / 3.70 |
| support.open_options_p10 | 1 / 0 | 0 / 0 | 0 / 0 |
| support.pass_target_open_pct | 81.89 / 70.27 | 72.41 / 72.15 | 64.21 / 71.03 |
| support.passes_completed_tracked | 127 / 111 | 116 / 79 | 95 / 107 |
| support.release_pass_completed_pct | 75 / 28.57 | 100 / 100 | 25 / 50 |
| support.releases_per_10min | 4 / 8 | 1 / 1 | 5 / 6 |
| support.run_abandon_per_10min | 0 / 0 | 0 / 0 | 0 / 0 |
| support.runs_in_behind_per_10min | 0 / 3 | 1 / 1 | 1 / 0 |
| support.ticks_with_le1_option_pct | 14.25 / 21.74 | 21.07 / 29.23 | 33.35 / 23.44 |
| support.width_m | 43.09 / 43.91 | 43.45 / 43.44 | 43.42 / 44.35 |
| trans.count_per_10min | 16 / 19 | 16 / 15 | 18 / 18 |
| trans.counterpress_pct | 68.75 / 47.37 | 31.25 / 33.33 | 44.44 / 44.44 |
| trans.forward_recovery_speed_mps_0_5s | 7.01 / 7.47 | 7.23 / 7.19 | 7.42 / 7.42 |
| trans.forward_sprint_share_0_5s | 68.13 / 77.13 | 72.17 / 70.95 | 74.90 / 74.85 |
| trans.forward_sprint_share_0_5s_7mps | 56.94 / 66.97 | 60.45 / 59.97 | 63.60 / 65.74 |
| trans.forwards_staying_high_mean | 1.25 / 1.43 | 2 / 1.70 | 1.50 / 1.89 |
| trans.in_shape_at_turnover_pct | 60 / 68.75 | 75 / 60 | 81.25 / 53.33 |
| trans.mid_recovery_speed_mps_0_5s | 6.42 / 6.79 | 6.77 / 6.38 | 6.60 / 6.71 |
| trans.mid_sprint_share_0_5s | 60.23 / 65.38 | 67.38 / 57.83 | 63.55 / 65.14 |
| trans.mid_sprint_share_0_5s_7mps | 47.31 / 54.06 | 54.83 / 46.86 | 52.63 / 51.22 |
| trans.outfield_stamina_end_permille | 890.5 / 897.6 | 890.4 / 896.3 | 881.8 / 869.4 |
| trans.pct_outfield_behind_ball_at_3s | 51.82 / 78.46 | 57.50 / 78.67 | 84.17 / 63.33 |
| trans.pct_outfield_behind_ball_at_6s | 87.50 / 81.43 | 67.50 / 77 | 80 / 68.89 |
| trans.time_to_shape_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_excluded_pct | 37.50 / 15.79 | 25 / 0 | 11.11 / 16.67 |
| trans.time_to_shape_s | 0.93 / 1.10 | 0.71 / 1.33 | 0.32 / 1.78 |
| trans.time_to_shape_static_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_static_excluded_pct | 56.25 / 21.05 | 31.25 / 0 | 16.67 / 33.33 |
| trans.time_to_shape_static_s | 0.26 / 0.71 | 0.42 / 1.15 | 0.21 / 1.74 |

GK scenario field: not present in the public state.

## 5v5 autopilot-hard vs hard (10 min x 3 seeds from 1)

Command: `psfc-replay analyze --format 5v5 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty hard --away-difficulty hard --home-controller autopilot`

Gameplay version: `0.1.0-5v5`

| seed | final state_hash | score (H-A) |
|---|---|---|
| 1 | `c092078a0e6392ee` | 6-5 |
| 2 | `105eab3c9cb0ef71` | 5-4 |
| 3 | `54e45e3c4c753e80` | 5-4 |

Match meta (mean over seeds): goals_total = 9.67, open_play_minutes = 10.00, open_play_pct_of_ticks = 92.20, shots_total = 42.67, ticks_simulated = 39055

### Match

| metric | home | away | teams mean |
|---|---:|---:|---:|
| fouls | 0.33 | 0 | **0.17** |
| goals | 5.33 | 4.33 | **4.83** |
| offsides | 2 | 0.67 | **1.33** |
| passes_completed_pct | 73.81 | 79.24 | **76.53** |
| possession_pct | 60.01 | 39.99 | **50** |
| shots | 33 | 9.67 | **21.33** |
| shots_on_target | 25.67 | 5.67 | **15.67** |

### Goalkeeper

| metric | home | away | teams mean |
|---|---:|---:|---:|
| catches | 0.67 | 5 | **2.83** |
| claims_non_save | 15.67 | 0.67 | **8.17** |
| excursion_mean_s | 1.06 | 0.88 | **0.97** |
| excursions_gt1s_per_10min | 4.00 | 0.67 | **2.33** |
| gathers | 16.33 | 5.67 | **11** |
| goals_conceded | 4.33 | 5.33 | **4.83** |
| goals_conceded_gk_gt3m_off_line | 1.33 | 2.67 | **2** |
| goals_conceded_no_shot_found | 1.33 | 0 | **0.67** |
| line_dist_at_conceded_shot_mean_m | 2.73 | 2.85 | **2.79** |
| line_dist_max_m | 9.38 | 8.22 | **8.80** |
| line_dist_mean_m | 3.03 | 2.86 | **2.94** |
| parries | 2.67 | 21.33 | **12** |
| pct_outside_6yd | 1.30 | 0.22 | **0.76** |
| pct_outside_box | 0 | 0 | **0** |
| rush_count | 7.67 | 1.33 | **4.50** |
| rush_per_10min | 7.66 | 1.33 | **4.50** |
| saves | 3.33 | 26.33 | **14.83** |
| tips_over | 0 | 0 | **0** |

### Defence and shape

| metric | home | away | teams mean |
|---|---:|---:|---:|
| attacking.anchor_dist_m.defender | 10.00 | 9.94 | **9.97** |
| attacking.anchor_dist_m.forward | 13.10 | 18.43 | **15.77** |
| attacking.anchor_dist_m.goalkeeper | 1.30 | 1.46 | **1.38** |
| attacking.anchor_dist_m.midfielder | 14.92 | 15.01 | **14.97** |
| attacking.anchor_dist_shape_m.defender | 8.07 | 9.07 | **8.57** |
| attacking.anchor_dist_shape_m.forward | 10.54 | 11.99 | **11.27** |
| attacking.anchor_dist_shape_m.midfielder | 10.46 | 9.84 | **10.15** |
| attacking.bbox_area_m2 | 424.4 | 279.1 | **351.7** |
| attacking.bbox_depth_m | 20.88 | 16.63 | **18.76** |
| attacking.bbox_width_m | 19.66 | 15.86 | **17.76** |
| attacking.length_m | 20.88 | 16.63 | **18.76** |
| attacking.nearest_mate_mean_m | 11.65 | 9.46 | **10.56** |
| attacking.nearest_mate_p10_m | 3.91 | 3.25 | **3.58** |
| attacking.pct_of_open_play | 57.69 | 42.31 | **50** |
| attacking.stretch_m | 10.70 | 8.56 | **9.63** |
| attacking.width_m | 19.66 | 15.86 | **17.76** |
| defending.anchor_dist_m.defender | 8.83 | 9.91 | **9.37** |
| defending.anchor_dist_m.forward | 13.54 | 16.18 | **14.86** |
| defending.anchor_dist_m.goalkeeper | 1.37 | 1.60 | **1.48** |
| defending.anchor_dist_m.midfielder | 13.70 | 14.04 | **13.87** |
| defending.anchor_dist_shape_m.defender | 8.67 | 9.89 | **9.28** |
| defending.anchor_dist_shape_m.forward | 10.72 | 11.28 | **11.00** |
| defending.anchor_dist_shape_m.midfielder | 9.11 | 9.65 | **9.38** |
| defending.bbox_area_m2 | 422.5 | 258.4 | **340.4** |
| defending.bbox_depth_m | 22.03 | 15.39 | **18.71** |
| defending.bbox_width_m | 18.85 | 16.54 | **17.69** |
| defending.length_m | 22.03 | 15.39 | **18.71** |
| defending.line_x_std_m | 7.13 | 4.45 | **5.79** |
| defending.line_x_variance_m2 | 73.82 | 32.04 | **52.93** |
| defending.nearest_mate_mean_m | 11.92 | 9.27 | **10.59** |
| defending.nearest_mate_p10_m | 4.85 | 3.37 | **4.11** |
| defending.pct_of_open_play | 42.31 | 57.69 | **50** |
| defending.pct_outfield_behind_ball | 72.71 | 65.58 | **69.14** |
| defending.stretch_m | 10.71 | 8.34 | **9.53** |
| defending.swarm2_pct | 1.08 | 0.59 | **0.84** |
| defending.swarm3_pct | 0 | 0.01 | **0.00** |
| defending.width_m | 18.85 | 16.54 | **17.69** |

### Defensive transition

| metric | home | away | teams mean |
|---|---:|---:|---:|
| count_per_10min | 27.99 | 52.32 | **40.16** |
| counterpress_pct | 72.84 | 38.03 | **55.44** |
| forward_recovery_speed_mps_0_5s | 6.75 | 6.37 | **6.56** |
| forward_sprint_share_0_5s | 65.29 | 58.94 | **62.11** |
| forward_sprint_share_0_5s_7mps | 52.30 | 45.30 | **48.80** |
| forwards_staying_high_mean | 0.78 | 0.64 | **0.71** |
| in_shape_at_turnover_pct | 63.75 | 49.50 | **56.62** |
| mid_recovery_speed_mps_0_5s | 6.67 | 6.25 | **6.46** |
| mid_sprint_share_0_5s | 66.28 | 59.87 | **63.08** |
| mid_sprint_share_0_5s_7mps | 54.11 | 46.38 | **50.25** |
| outfield_stamina_end_permille | 738.7 | 818 | **778.3** |
| pct_outfield_behind_ball_at_3s | 68.67 | 56.73 | **62.70** |
| pct_outfield_behind_ball_at_6s | 41.67 | 65.57 | **53.62** |
| time_to_shape_capped_pct | 0 | 0 | **0** |
| time_to_shape_excluded_pct | 19.83 | 30.52 | **25.17** |
| time_to_shape_s | 0.56 | 0.89 | **0.73** |
| time_to_shape_static_capped_pct | 0 | 0 | **0** |
| time_to_shape_static_excluded_pct | 26.82 | 49.00 | **37.91** |
| time_to_shape_static_s | 0.29 | 0.37 | **0.33** |

### Attacking support

| metric | home | away | teams mean |
|---|---:|---:|---:|
| carrier_pct_of_open_play | 19.97 | 13.56 | **16.76** |
| cluster_pct | 7.88 | 11.38 | **9.63** |
| depth_m | 20.64 | 17.94 | **19.29** |
| forward_options_mean | 0.32 | 0.27 | **0.30** |
| nearest_option_dist_m | 12.83 | 12.21 | **12.52** |
| open_options_mean | 1.30 | 0.93 | **1.12** |
| open_options_p10 | 0 | 0 | **0** |
| pass_target_open_pct | 73.92 | 56.43 | **65.18** |
| passes_completed_tracked | 122 | 102 | **112** |
| release_pass_completed_pct | 80.95 | 52.78 | **66.87** |
| releases_per_10min | 6.33 | 4.33 | **5.33** |
| run_abandon_per_10min | 0 | 0 | **0** |
| runs_in_behind_per_10min | 10.00 | 3.00 | **6.50** |
| ticks_with_le1_option_pct | 54.47 | 67.37 | **60.92** |
| width_m | 17.95 | 14.55 | **16.25** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 12.20 | 11.75 | **11.98** |
| accel_gt10_pct | 52.00 | 53.78 | **52.89** |
| accel_mean_mps2 | 12.16 | 11.65 | **11.91** |
| class_time_pct.0 | 16.92 | 11.73 | **14.33** |
| class_time_pct.1 | 7.21 | 8.27 | **7.74** |
| class_time_pct.2 | 62.94 | 67.44 | **65.19** |
| class_time_pct.3 | 12.92 | 12.56 | **12.74** |
| dspeed_max_mps_per_tick | 4.22 | 5.94 | **5.08** |
| heading_change_p95_deg_per_tick_gt3mps | 3.73 | 3.49 | **3.61** |
| heading_change_pct.0_10 | 99.10 | 99.31 | **99.21** |
| heading_change_pct.10_45 | 0.88 | 0.66 | **0.77** |
| heading_change_pct.45_90 | 0.03 | 0.02 | **0.02** |
| heading_change_pct.gt90 | 0.00 | 0.00 | **0.00** |
| heading_sign_flips_per_player_min | 28.99 | 18.21 | **23.60** |
| lateral_accel_mean_mps2 | 8.57 | 8.39 | **8.48** |
| reversals_per_player_min | 0.01 | 0.03 | **0.02** |
| side_flips_by_class.0_per_player_min | 295.3 | 117.4 | **206.3** |
| side_flips_by_class.0_share_pct | 66.18 | 35.86 | **51.02** |
| side_flips_by_class.1_per_player_min | 132.4 | 47.57 | **89.99** |
| side_flips_by_class.1_share_pct | 12.69 | 10.18 | **11.43** |
| side_flips_by_class.2_per_player_min | 20.24 | 25.48 | **22.86** |
| side_flips_by_class.2_share_pct | 16.88 | 44.73 | **30.81** |
| side_flips_by_class.3_per_player_min | 24.92 | 27.91 | **26.41** |
| side_flips_by_class.3_share_pct | 4.25 | 9.23 | **6.74** |
| side_flips_cause.desired_per_player_min | 60.99 | 15.22 | **38.11** |
| side_flips_cause.filter_per_player_min | 11.24 | 18.86 | **15.05** |
| side_flips_cause.limiter_per_player_min | 1.92 | 1.94 | **1.93** |
| side_flips_cause.sampler_per_player_min | 1.37 | 2.36 | **1.86** |
| side_flips_per_player_min | 75.51 | 38.38 | **56.94** |
| side_flips_schmitt_per_player_min | 38.49 | 39.83 | **39.16** |
| side_flips_speed_ge3_per_player_min | 101.4 | 48.16 | **74.78** |
| side_flips_speed_lt3_per_player_min | 13.97 | 14.60 | **14.28** |
| speed_oscillation_pct | 4.48 | 4.47 | **4.47** |
| turn_rate_mean_deg_per_s | 102.0 | 103.3 | **102.7** |

### Collisions and push-out

| metric | home | away | teams mean |
|---|---:|---:|---:|
| any.depth_max_m | 0.79 | 0.79 | **0.79** |
| any.depth_mean_m | 0.25 | 0.25 | **0.25** |
| any.events_per_min | 2.00 | 2.00 | **2.00** |
| any.stuck_mean_ticks | 11.58 | 11.58 | **11.58** |
| any.stuck_pairs | 2 | 2 | **2** |
| any.ticks_gt1cm_per_min | 5.17 | 5.17 | **5.17** |
| any.ticks_per_min | 5.90 | 5.90 | **5.90** |
| correction_vel_corrected_p50_mps | 0.64 | 1.08 | **0.86** |
| correction_vel_corrected_p99_mps | 11.68 | 12.72 | **12.20** |
| correction_vel_corrected_pct | 0.09 | 0.10 | **0.10** |
| correction_vel_max_mps | 12.70 | 14.66 | **13.68** |
| correction_vel_p50_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p95_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p99_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_pct_gt_0_3 | 0.07 | 0.08 | **0.08** |
| opponent.depth_max_m | 0.13 | 0.13 | **0.13** |
| opponent.depth_mean_m | 0.04 | 0.04 | **0.04** |
| opponent.events_per_min | 1.23 | 1.23 | **1.23** |
| opponent.stuck_mean_ticks | 3.67 | 3.67 | **3.67** |
| opponent.stuck_pairs | 0.33 | 0.33 | **0.33** |
| opponent.ticks_gt1cm_per_min | 1.97 | 1.97 | **1.97** |
| opponent.ticks_per_min | 2.57 | 2.57 | **2.57** |
| teammate.depth_max_m | 0.78 | 0.78 | **0.78** |
| teammate.depth_mean_m | 0.38 | 0.34 | **0.36** |
| teammate.events_per_min | 0.33 | 0.43 | **0.38** |
| teammate.stuck_mean_ticks | 8.67 | 7.33 | **8** |
| teammate.stuck_pairs | 0.67 | 1 | **0.83** |
| teammate.ticks_gt1cm_per_min | 2.47 | 2.43 | **2.45** |
| teammate.ticks_per_min | 2.50 | 2.53 | **2.52** |

### Coordination

| metric | home | away | teams mean |
|---|---:|---:|---:|
| cruise_over72_pct | 39.52 | 40.30 | **39.91** |
| dual_chaser_pct_of_loose | 14.43 | 17.62 | **16.02** |
| dual_chaser_ticks_per_min | 335.8 | 410.0 | **372.9** |
| engagers3_pct_of_carrier_ticks | 0.01 | 0.18 | **0.10** |
| engagers3_ticks_per_min | 0.03 | 1.40 | **0.72** |
| mark_steal_per_defender_min | 0.07 | 0.12 | **0.09** |
| max_simultaneous_chasers | 4 | 5 | **4.50** |

### Aerial

| metric | home | away | teams mean |
|---|---:|---:|---:|
| headers | 7.33 | 8.33 | **7.83** |
| headers_clear | 1.33 | 2.67 | **2** |
| headers_pass | 3.67 | 4.33 | **4** |
| headers_shot | 2.33 | 1.33 | **1.83** |
| jumps_gk | 12.67 | 24 | **18.33** |
| jumps_outfield | 29.33 | 23 | **26.17** |
| max_airborne_outfield | 1 | 1 | **1** |

### Team shape vs envelope (teams mean)

| metric | in possession | out of possession | 11v11 envelope (m) |
|---|---:|---:|---|
| length_m | 18.76 | 18.71 | 31-46 |
| width_m | 17.76 | 17.69 | 35-48 |
| stretch_m | 9.63 | 9.53 | 7-16 |
| nearest_mate_mean_m | 10.56 | 10.59 | none published (in-possession mean near 9-10 m, estimate) |
| nearest_mate_p10_m | 3.58 | 4.11 | none published (in-possession mean near 9-10 m, estimate) |

Note: 5v5 has no published team-shape envelope; the 11v11 ranges are printed for reference only and do not apply to a 4-outfielder team on a small pitch.

### Per seed (home / away)

| metric | seed 1 | seed 2 | seed 3 |
|---|---:|---:|---:|
| aerial.headers | 7 / 7 | 9 / 11 | 6 / 7 |
| aerial.headers_clear | 1 / 4 | 1 / 4 | 2 / 0 |
| aerial.headers_pass | 3 / 2 | 6 / 6 | 2 / 5 |
| aerial.headers_shot | 3 / 1 | 2 / 1 | 2 / 2 |
| aerial.jumps_gk | 9 / 31 | 15 / 23 | 14 / 18 |
| aerial.jumps_outfield | 31 / 19 | 24 / 34 | 33 / 16 |
| aerial.max_airborne_outfield | 1 / 1 | 1 / 1 | 1 / 1 |
| collision.any.depth_max_m | 0.76 / 0.76 | 0.80 / 0.80 | 0.80 / 0.80 |
| collision.any.depth_mean_m | 0.25 / 0.25 | 0.24 / 0.24 | 0.26 / 0.26 |
| collision.any.events_per_min | 2.00 / 2.00 | 2.20 / 2.20 | 1.80 / 1.80 |
| collision.any.stuck_mean_ticks | 11 / 11 | 12 / 12 | 11.75 / 11.75 |
| collision.any.stuck_pairs | 1 / 1 | 1 / 1 | 4 / 4 |
| collision.any.ticks_gt1cm_per_min | 4.50 / 4.50 | 4 / 4 | 7.00 / 7.00 |
| collision.any.ticks_per_min | 5.40 / 5.40 | 4.90 / 4.90 | 7.40 / 7.40 |
| collision.correction_vel_corrected_p50_mps | 0.48 / 0.90 | 0.51 / 0.97 | 0.94 / 1.38 |
| collision.correction_vel_corrected_p99_mps | 10.84 / 12.68 | 12.55 / 12.65 | 11.67 / 12.83 |
| collision.correction_vel_corrected_pct | 0.08 / 0.08 | 0.11 / 0.12 | 0.09 / 0.09 |
| collision.correction_vel_max_mps | 11.28 / 13.82 | 13.27 / 16.93 | 13.55 / 13.22 |
| collision.correction_vel_p50_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p95_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p99_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_pct_gt_0_3 | 0.06 / 0.06 | 0.09 / 0.10 | 0.08 / 0.08 |
| collision.opponent.depth_max_m | 0.11 / 0.11 | 0.11 / 0.11 | 0.17 / 0.17 |
| collision.opponent.depth_mean_m | 0.03 / 0.03 | 0.03 / 0.03 | 0.07 / 0.07 |
| collision.opponent.events_per_min | 1.10 / 1.10 | 1.60 / 1.60 | 1.00 / 1.00 |
| collision.opponent.stuck_mean_ticks | 0 / 0 | 0 / 0 | 11 / 11 |
| collision.opponent.stuck_pairs | 0 / 0 | 0 / 0 | 1 / 1 |
| collision.opponent.ticks_gt1cm_per_min | 1.00 / 1.00 | 2 / 2 | 2.90 / 2.90 |
| collision.opponent.ticks_per_min | 1.70 / 1.70 | 2.70 / 2.70 | 3.30 / 3.30 |
| collision.teammate.depth_max_m | 0.76 / 0.76 | 0.80 / 0.78 | 0.78 / 0.80 |
| collision.teammate.depth_mean_m | 0.35 / 0.28 | 0.44 / 0.35 | 0.35 / 0.39 |
| collision.teammate.events_per_min | 0.40 / 0.50 | 0.20 / 0.40 | 0.40 / 0.40 |
| collision.teammate.stuck_mean_ticks | 0 / 11 | 12 / 0 | 14 / 11 |
| collision.teammate.stuck_pairs | 0 / 1 | 1 / 0 | 1 / 2 |
| collision.teammate.ticks_gt1cm_per_min | 2.50 / 3.00 | 1.90 / 1.40 | 3.00 / 2.90 |
| collision.teammate.ticks_per_min | 2.50 / 3.20 | 2 / 1.50 | 3.00 / 2.90 |
| coord.cruise_over72_pct | 39.73 / 39.92 | 40.26 / 38.99 | 38.57 / 41.98 |
| coord.dual_chaser_pct_of_loose | 12.79 / 17.23 | 15.52 / 19.10 | 14.98 / 16.51 |
| coord.dual_chaser_ticks_per_min | 298.3 / 401.9 | 361.7 / 445.3 | 347.5 / 383.0 |
| coord.engagers3_pct_of_carrier_ticks | 0 / 0.27 | 0.02 / 0.29 | 0 / 0 |
| coord.engagers3_ticks_per_min | 0 / 2.20 | 0.10 / 2 | 0 / 0 |
| coord.mark_steal_per_defender_min | 0.20 / 0 | 0 / 0.13 | 0 / 0.23 |
| coord.max_simultaneous_chasers | 4 / 5 | 4 / 5 | 4 / 5 |
| gk.catches | 0 / 7 | 1 / 4 | 1 / 4 |
| gk.claims_non_save | 17 / 0 | 13 / 0 | 17 / 2 |
| gk.excursion_mean_s | 1.01 / 0.12 | 1.23 / 1.90 | 0.93 / 0.62 |
| gk.excursions_gt1s_per_10min | 3.00 / 0 | 4 / 1 | 5.00 / 1.00 |
| gk.gathers | 17 / 7 | 14 / 4 | 18 / 6 |
| gk.goals_conceded | 5 / 6 | 4 / 5 | 4 / 5 |
| gk.goals_conceded_gk_gt3m_off_line | 2 / 3 | 1 / 4 | 1 / 1 |
| gk.goals_conceded_no_shot_found | 2 / 0 | 1 / 0 | 1 / 0 |
| gk.line_dist_at_conceded_shot_mean_m | 3.08 / 2.37 | 3.08 / 4.00 | 2.03 / 2.17 |
| gk.line_dist_max_m | 10.10 / 4.84 | 8.76 / 10.51 | 9.27 / 9.29 |
| gk.line_dist_mean_m | 3.05 / 2.78 | 3.03 / 2.83 | 3.01 / 2.96 |
| gk.parries | 0 / 24 | 5 / 19 | 3 / 21 |
| gk.pct_outside_6yd | 1.18 / 0.02 | 1.02 / 0.32 | 1.71 / 0.31 |
| gk.pct_outside_box | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_count | 7 / 0 | 5 / 1 | 11 / 3 |
| gk.rush_per_10min | 7.00 / 0 | 5 / 1 | 10.99 / 3.00 |
| gk.saves | 0 / 31 | 6 / 23 | 4 / 25 |
| gk.tips_over | 0 / 0 | 0 / 0 | 0 / 0 |
| match.fouls | 0 / 0 | 1 / 0 | 0 / 0 |
| match.goals | 6 / 5 | 5 / 4 | 5 / 4 |
| match.offsides | 1 / 2 | 1 / 0 | 4 / 0 |
| match.passes_completed_pct | 77.31 / 79.25 | 67.38 / 77.05 | 76.74 / 81.42 |
| match.possession_pct | 65.22 / 34.78 | 54.89 / 45.11 | 59.94 / 40.06 |
| match.shots | 40 / 7 | 28 / 10 | 31 / 12 |
| match.shots_on_target | 31 / 2 | 21 / 8 | 25 / 7 |
| move.accel_from_pos_mean_mps2 | 11.96 / 11.64 | 12.10 / 11.78 | 12.56 / 11.83 |
| move.accel_gt10_pct | 50.46 / 53.40 | 51.68 / 53.79 | 53.87 / 54.16 |
| move.accel_mean_mps2 | 11.92 / 11.53 | 12.04 / 11.69 | 12.53 / 11.73 |
| move.class_time_pct.0 | 17.31 / 10.81 | 16.57 / 12.83 | 16.88 / 11.55 |
| move.class_time_pct.1 | 6.25 / 8.99 | 8.07 / 7.33 | 7.32 / 8.48 |
| move.class_time_pct.2 | 62.70 / 67.44 | 63.50 / 67.13 | 62.62 / 67.75 |
| move.class_time_pct.3 | 13.73 / 12.76 | 11.87 / 12.71 | 13.18 / 12.21 |
| move.dspeed_max_mps_per_tick | 4.10 / 5.80 | 3.83 / 5.65 | 4.72 / 6.39 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.70 / 3.47 | 3.72 / 3.50 | 3.76 / 3.49 |
| move.heading_change_pct.0_10 | 99.16 / 99.34 | 99.10 / 99.32 | 99.04 / 99.28 |
| move.heading_change_pct.10_45 | 0.82 / 0.65 | 0.87 / 0.65 | 0.94 / 0.69 |
| move.heading_change_pct.45_90 | 0.02 / 0.02 | 0.03 / 0.03 | 0.02 / 0.02 |
| move.heading_change_pct.gt90 | 0 / 0.00 | 0.00 / 0.00 | 0 / 0.00 |
| move.heading_sign_flips_per_player_min | 26.28 / 16.93 | 30.61 / 18.67 | 30.07 / 19.04 |
| move.lateral_accel_mean_mps2 | 8.24 / 8.33 | 8.68 / 8.41 | 8.80 / 8.44 |
| move.reversals_per_player_min | 0 / 0.04 | 0.04 / 0.04 | 0 / 0.02 |
| move.side_flips_by_class.0_per_player_min | 301.3 / 112.9 | 301.5 / 119.9 | 283.0 / 119.4 |
| move.side_flips_by_class.0_share_pct | 69.38 / 34.10 | 64.93 / 37.61 | 64.22 / 35.87 |
| move.side_flips_by_class.1_per_player_min | 126.8 / 42.56 | 140.8 / 54.11 | 129.6 / 46.03 |
| move.side_flips_by_class.1_share_pct | 10.55 / 10.69 | 14.76 / 9.70 | 12.75 / 10.15 |
| move.side_flips_by_class.2_per_player_min | 18.79 / 23.23 | 19.62 / 27.84 | 22.31 / 25.38 |
| move.side_flips_by_class.2_share_pct | 15.67 / 43.78 | 16.19 / 45.69 | 18.78 / 44.72 |
| move.side_flips_by_class.3_per_player_min | 24.07 / 32.02 | 26.67 / 22.53 | 24.00 / 29.17 |
| move.side_flips_by_class.3_share_pct | 4.40 / 11.42 | 4.11 / 7.00 | 4.25 / 9.27 |
| move.side_flips_cause.desired_per_player_min | 61.20 / 13.48 | 62.17 / 16.50 | 59.59 / 15.67 |
| move.side_flips_cause.filter_per_player_min | 10.98 / 18.63 | 10.88 / 19.59 | 11.85 / 18.36 |
| move.side_flips_cause.limiter_per_player_min | 1.74 / 1.54 | 2.42 / 2.32 | 1.58 / 1.96 |
| move.side_flips_cause.sampler_per_player_min | 1.26 / 2.12 | 1.46 / 2.48 | 1.38 / 2.46 |
| move.side_flips_per_player_min | 75.19 / 35.78 | 76.94 / 40.90 | 74.41 / 38.46 |
| move.side_flips_schmitt_per_player_min | 37.72 / 38.68 | 38.96 / 41.06 | 38.80 / 39.76 |
| move.side_flips_speed_ge3_per_player_min | 101.7 / 44.50 | 102.1 / 51.58 | 100.4 / 48.41 |
| move.side_flips_speed_lt3_per_player_min | 13.30 / 14.89 | 15.96 / 14.58 | 12.65 / 14.33 |
| move.speed_oscillation_pct | 4.45 / 4.36 | 4.53 / 4.62 | 4.45 / 4.45 |
| move.turn_rate_mean_deg_per_s | 98.15 / 102.3 | 102.2 / 103.6 | 105.7 / 104.1 |
| shape.attacking.anchor_dist_m.defender | 9.71 / 10.15 | 11.00 / 9.54 | 9.30 / 10.14 |
| shape.attacking.anchor_dist_m.forward | 12.76 / 19.53 | 13.79 / 17.08 | 12.75 / 18.69 |
| shape.attacking.anchor_dist_m.goalkeeper | 1.27 / 1.58 | 1.29 / 1.40 | 1.33 / 1.39 |
| shape.attacking.anchor_dist_m.midfielder | 14.45 / 16.18 | 16.05 / 14.24 | 14.26 / 14.61 |
| shape.attacking.anchor_dist_shape_m.defender | 7.98 / 9.68 | 8.73 / 7.98 | 7.50 / 9.55 |
| shape.attacking.anchor_dist_shape_m.forward | 10.53 / 12.08 | 10.86 / 11.68 | 10.24 / 12.22 |
| shape.attacking.anchor_dist_shape_m.midfielder | 10.32 / 10.60 | 11.04 / 9.71 | 10.04 / 9.22 |
| shape.attacking.bbox_area_m2 | 447.8 / 259.1 | 410.1 / 302.2 | 415.3 / 275.9 |
| shape.attacking.bbox_depth_m | 21.76 / 16.05 | 20.46 / 17.77 | 20.43 / 16.07 |
| shape.attacking.bbox_width_m | 20.15 / 15.06 | 19.54 / 16.31 | 19.28 / 16.21 |
| shape.attacking.length_m | 21.76 / 16.05 | 20.46 / 17.77 | 20.43 / 16.07 |
| shape.attacking.nearest_mate_mean_m | 12.04 / 9.08 | 11.37 / 9.97 | 11.55 / 9.33 |
| shape.attacking.nearest_mate_p10_m | 4.13 / 3.00 | 3.83 / 3.49 | 3.77 / 3.28 |
| shape.attacking.pct_of_open_play | 60.30 / 39.70 | 55.07 / 44.93 | 57.69 / 42.31 |
| shape.attacking.stretch_m | 11.05 / 8.32 | 10.61 / 8.92 | 10.44 / 8.43 |
| shape.attacking.width_m | 20.15 / 15.06 | 19.54 / 16.31 | 19.28 / 16.21 |
| shape.defending.anchor_dist_m.defender | 9.55 / 9.46 | 8.35 / 10.42 | 8.61 / 9.86 |
| shape.defending.anchor_dist_m.forward | 13.02 / 16.06 | 14.17 / 17.25 | 13.41 / 15.23 |
| shape.defending.anchor_dist_m.goalkeeper | 1.32 / 1.58 | 1.35 / 1.67 | 1.43 / 1.55 |
| shape.defending.anchor_dist_m.midfielder | 14.06 / 14.66 | 13.63 / 14.23 | 13.42 / 13.23 |
| shape.defending.anchor_dist_shape_m.defender | 9.17 / 9.65 | 8.39 / 10.18 | 8.46 / 9.85 |
| shape.defending.anchor_dist_shape_m.forward | 10.42 / 10.90 | 11.12 / 11.75 | 10.63 / 11.18 |
| shape.defending.anchor_dist_shape_m.midfielder | 9.28 / 10.61 | 9.38 / 9.03 | 8.67 / 9.31 |
| shape.defending.bbox_area_m2 | 446.5 / 250.7 | 409.8 / 260.5 | 411.2 / 263.9 |
| shape.defending.bbox_depth_m | 23.48 / 15.06 | 21.42 / 15.53 | 21.20 / 15.57 |
| shape.defending.bbox_width_m | 18.66 / 16.39 | 18.65 / 16.42 | 19.23 / 16.80 |
| shape.defending.length_m | 23.48 / 15.06 | 21.42 / 15.53 | 21.20 / 15.57 |
| shape.defending.line_x_std_m | 7.47 / 4.27 | 6.81 / 4.61 | 7.12 / 4.45 |
| shape.defending.line_x_variance_m2 | 80.40 / 29.35 | 69.33 / 34.52 | 71.73 / 32.26 |
| shape.defending.nearest_mate_mean_m | 12.30 / 9.28 | 11.81 / 9.23 | 11.64 / 9.31 |
| shape.defending.nearest_mate_p10_m | 5.00 / 3.61 | 4.89 / 3.12 | 4.66 / 3.39 |
| shape.defending.pct_of_open_play | 39.70 / 60.30 | 44.93 / 55.07 | 42.31 / 57.69 |
| shape.defending.pct_outfield_behind_ball | 74.97 / 67.60 | 72.24 / 61.97 | 70.90 / 67.19 |
| shape.defending.stretch_m | 11.06 / 8.24 | 10.47 / 8.39 | 10.61 / 8.40 |
| shape.defending.swarm2_pct | 1.18 / 0.51 | 1.26 / 0.88 | 0.82 / 0.38 |
| shape.defending.swarm3_pct | 0 / 0.02 | 0 / 0 | 0 / 0 |
| shape.defending.width_m | 18.66 / 16.39 | 18.65 / 16.42 | 19.23 / 16.80 |
| support.carrier_pct_of_open_play | 22.15 / 11.55 | 17.38 / 15.31 | 20.38 / 13.81 |
| support.cluster_pct | 5.53 / 14.89 | 6.34 / 8.27 | 11.77 / 10.97 |
| support.depth_m | 21.81 / 17.34 | 20.50 / 18.56 | 19.62 / 17.93 |
| support.forward_options_mean | 0.33 / 0.27 | 0.28 / 0.31 | 0.34 / 0.23 |
| support.nearest_option_dist_m | 12.88 / 12.45 | 12.72 / 12.25 | 12.87 / 11.95 |
| support.open_options_mean | 1.33 / 0.89 | 1.23 / 0.95 | 1.35 / 0.95 |
| support.open_options_p10 | 0 / 0 | 0 / 0 | 0 / 0 |
| support.pass_target_open_pct | 75.57 / 55.17 | 75.73 / 60 | 70.45 / 54.13 |
| support.passes_completed_tracked | 131 / 87 | 103 / 110 | 132 / 109 |
| support.release_pass_completed_pct | 71.43 / 50 | 71.43 / 83.33 | 100 / 25 |
| support.releases_per_10min | 7.00 / 2.00 | 7 / 7 | 5.00 / 4.00 |
| support.run_abandon_per_10min | 0 / 0 | 0 / 0 | 0 / 0 |
| support.runs_in_behind_per_10min | 10.00 / 3.00 | 12 / 3 | 7.99 / 3.00 |
| support.ticks_with_le1_option_pct | 52.03 / 70.11 | 59.23 / 65.16 | 52.17 / 66.83 |
| support.width_m | 18.13 / 13.52 | 18.47 / 15.17 | 17.26 / 14.96 |
| trans.count_per_10min | 28.00 / 51.00 | 23 / 52 | 32.98 / 53.97 |
| trans.counterpress_pct | 89.29 / 25.49 | 56.52 / 42.31 | 72.73 / 46.30 |
| trans.forward_recovery_speed_mps_0_5s | 6.79 / 6.74 | 6.43 / 6.11 | 7.02 / 6.25 |
| trans.forward_sprint_share_0_5s | 66.46 / 64.64 | 59.39 / 54.56 | 70.00 / 57.63 |
| trans.forward_sprint_share_0_5s_7mps | 53.41 / 52.00 | 45.39 / 41.13 | 58.10 / 42.76 |
| trans.forwards_staying_high_mean | 1 / 0.58 | 0.67 / 0.80 | 0.67 / 0.55 |
| trans.in_shape_at_turnover_pct | 82.61 / 54.05 | 52.63 / 55.56 | 56 / 38.89 |
| trans.mid_recovery_speed_mps_0_5s | 6.64 / 6.18 | 6.77 / 6.24 | 6.60 / 6.32 |
| trans.mid_sprint_share_0_5s | 65.82 / 58.10 | 68.29 / 60.36 | 64.73 / 61.15 |
| trans.mid_sprint_share_0_5s_7mps | 54.62 / 44.65 | 55.70 / 47.12 | 52.02 / 47.38 |
| trans.outfield_stamina_end_permille | 787 / 910.8 | 724.2 / 797.5 | 704.8 / 745.8 |
| trans.pct_outfield_behind_ball_at_3s | 68.18 / 58.33 | 66.67 / 59.26 | 71.15 / 52.59 |
| trans.pct_outfield_behind_ball_at_6s | 0 / 68.75 | 58.33 / 57.50 | 66.67 / 70.45 |
| trans.time_to_shape_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_excluded_pct | 17.86 / 27.45 | 17.39 / 30.77 | 24.24 / 33.33 |
| trans.time_to_shape_s | 0.15 / 0.87 | 1.01 / 0.84 | 0.53 / 0.98 |
| trans.time_to_shape_static_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_static_excluded_pct | 21.43 / 47.06 | 34.78 / 48.08 | 24.24 / 51.85 |
| trans.time_to_shape_static_s | 0.13 / 0.40 | 0.24 / 0.37 | 0.51 / 0.33 |

GK scenario field: not present in the public state.

## 11v11 autopilot-normal vs normal (10 min x 3 seeds from 1)

Command: `psfc-replay analyze --format 11v11 --seeds 3 --seed-base 1 --minutes 10 --home-difficulty normal --away-difficulty normal --home-controller autopilot`

Gameplay version: `0.1.0-5v5`

| seed | final state_hash | score (H-A) |
|---|---|---|
| 1 | `920c008be2952151` | 4-0 |
| 2 | `9c91b31d8231838d` | 6-1 |
| 3 | `7fc0f9241bd5a6e1` | 9-0 |

Match meta (mean over seeds): goals_total = 6.67, open_play_minutes = 10.00, open_play_pct_of_ticks = 95.19, shots_total = 30.33, ticks_simulated = 37832.3

### Match

| metric | home | away | teams mean |
|---|---:|---:|---:|
| fouls | 0.67 | 0 | **0.33** |
| goals | 6.33 | 0.33 | **3.33** |
| offsides | 1.67 | 0.33 | **1** |
| passes_completed_pct | 77.14 | 75.32 | **76.23** |
| possession_pct | 66.03 | 33.97 | **50** |
| shots | 27.67 | 2.67 | **15.17** |
| shots_on_target | 18.67 | 2 | **10.33** |

### Goalkeeper

| metric | home | away | teams mean |
|---|---:|---:|---:|
| catches | 1 | 1.33 | **1.17** |
| claims_non_save | 7 | 2 | **4.50** |
| excursion_mean_s | 1.04 | 0.98 | **1.01** |
| excursions_gt1s_per_10min | 2.67 | 3.66 | **3.17** |
| gathers | 8 | 3.33 | **5.67** |
| goals_conceded | 0.33 | 6.33 | **3.33** |
| goals_conceded_gk_gt3m_off_line | 0 | 4.67 | **2.33** |
| goals_conceded_no_shot_found | 0 | 0 | **0** |
| line_dist_at_conceded_shot_mean_m | 0.52 | 3.63 | **2.07** |
| line_dist_max_m | 10.06 | 7.38 | **8.72** |
| line_dist_mean_m | 4.43 | 4.36 | **4.39** |
| parries | 0.67 | 11.67 | **6.17** |
| pct_outside_6yd | 0.88 | 1.27 | **1.07** |
| pct_outside_box | 0 | 0 | **0** |
| rush_count | 5 | 7.33 | **6.17** |
| rush_per_10min | 5.00 | 7.33 | **6.16** |
| saves | 1.67 | 13 | **7.33** |
| tips_over | 0 | 0 | **0** |

### Defence and shape

| metric | home | away | teams mean |
|---|---:|---:|---:|
| attacking.anchor_dist_m.defender | 21.74 | 16.65 | **19.19** |
| attacking.anchor_dist_m.forward | 20.33 | 27.22 | **23.78** |
| attacking.anchor_dist_m.goalkeeper | 2.32 | 2.32 | **2.32** |
| attacking.anchor_dist_m.midfielder | 18.65 | 18.80 | **18.72** |
| attacking.anchor_dist_shape_m.defender | 16.92 | 14.91 | **15.91** |
| attacking.anchor_dist_shape_m.forward | 16.54 | 20.16 | **18.35** |
| attacking.anchor_dist_shape_m.midfielder | 12.97 | 10.85 | **11.91** |
| attacking.bbox_area_m2 | 2236.6 | 1697.8 | **1967.2** |
| attacking.bbox_depth_m | 47.60 | 39.36 | **43.48** |
| attacking.bbox_width_m | 46.32 | 41.86 | **44.09** |
| attacking.length_m | 47.60 | 39.36 | **43.48** |
| attacking.nearest_mate_mean_m | 12.97 | 10.87 | **11.92** |
| attacking.nearest_mate_p10_m | 6.59 | 4.44 | **5.52** |
| attacking.pct_of_open_play | 64.41 | 35.59 | **50** |
| attacking.stretch_m | 21.18 | 17.95 | **19.57** |
| attacking.width_m | 46.32 | 41.86 | **44.09** |
| defending.anchor_dist_m.defender | 20.81 | 18.33 | **19.57** |
| defending.anchor_dist_m.forward | 22.99 | 26.56 | **24.78** |
| defending.anchor_dist_m.goalkeeper | 2.47 | 2.59 | **2.53** |
| defending.anchor_dist_m.midfielder | 17.35 | 18.16 | **17.75** |
| defending.anchor_dist_shape_m.defender | 16.62 | 16.23 | **16.43** |
| defending.anchor_dist_shape_m.forward | 18.05 | 21.58 | **19.81** |
| defending.anchor_dist_shape_m.midfielder | 10.80 | 11.66 | **11.23** |
| defending.bbox_area_m2 | 1765.7 | 1365.9 | **1565.8** |
| defending.bbox_depth_m | 42.60 | 36.13 | **39.36** |
| defending.bbox_width_m | 40.13 | 36.46 | **38.30** |
| defending.length_m | 42.60 | 36.13 | **39.36** |
| defending.line_x_std_m | 7.03 | 4.49 | **5.76** |
| defending.line_x_variance_m2 | 82.80 | 36.75 | **59.77** |
| defending.nearest_mate_mean_m | 11.28 | 9.70 | **10.49** |
| defending.nearest_mate_p10_m | 5.93 | 4.36 | **5.14** |
| defending.pct_of_open_play | 35.59 | 64.41 | **50** |
| defending.pct_outfield_behind_ball | 76.11 | 71.72 | **73.91** |
| defending.stretch_m | 18.67 | 15.63 | **17.15** |
| defending.swarm2_pct | 1.10 | 0.93 | **1.01** |
| defending.swarm3_pct | 0 | 0 | **0** |
| defending.width_m | 40.13 | 36.46 | **38.30** |

### Defensive transition

| metric | home | away | teams mean |
|---|---:|---:|---:|
| count_per_10min | 20.66 | 36.99 | **28.82** |
| counterpress_pct | 88.94 | 33.33 | **61.14** |
| forward_recovery_speed_mps_0_5s | 7.40 | 6.89 | **7.14** |
| forward_sprint_share_0_5s | 74.12 | 66.28 | **70.20** |
| forward_sprint_share_0_5s_7mps | 63.24 | 54.08 | **58.66** |
| forwards_staying_high_mean | 2.25 | 1.04 | **1.64** |
| in_shape_at_turnover_pct | 74.31 | 59.13 | **66.72** |
| mid_recovery_speed_mps_0_5s | 6.77 | 6.46 | **6.62** |
| mid_sprint_share_0_5s | 66.63 | 59.98 | **63.30** |
| mid_sprint_share_0_5s_7mps | 54.05 | 47.02 | **50.54** |
| outfield_stamina_end_permille | 910 | 870.2 | **890.1** |
| pct_outfield_behind_ball_at_3s | 70.59 | 68.86 | **69.72** |
| pct_outfield_behind_ball_at_6s | 77.50 | 77.62 | **77.56** |
| time_to_shape_capped_pct | 0 | 0 | **0** |
| time_to_shape_excluded_pct | 16.36 | 24.32 | **20.34** |
| time_to_shape_s | 0.70 | 0.84 | **0.77** |
| time_to_shape_static_capped_pct | 0 | 0 | **0** |
| time_to_shape_static_excluded_pct | 16.36 | 36.94 | **26.65** |
| time_to_shape_static_s | 0.47 | 0.60 | **0.54** |

### Attacking support

| metric | home | away | teams mean |
|---|---:|---:|---:|
| carrier_pct_of_open_play | 20.06 | 10.16 | **15.11** |
| cluster_pct | 3.91 | 15.35 | **9.63** |
| depth_m | 45.30 | 38.68 | **41.99** |
| forward_options_mean | 0.72 | 0.59 | **0.65** |
| nearest_option_dist_m | 13.83 | 11.39 | **12.61** |
| open_options_mean | 2.77 | 2.47 | **2.62** |
| open_options_p10 | 0 | 0 | **0** |
| pass_target_open_pct | 71.14 | 52.86 | **62.00** |
| passes_completed_tracked | 123.7 | 74 | **98.83** |
| release_pass_completed_pct | 0 | 0 | **0** |
| releases_per_10min | 0.33 | 0.67 | **0.50** |
| run_abandon_per_10min | 0 | 0 | **0** |
| runs_in_behind_per_10min | 7.67 | 2.33 | **5.00** |
| ticks_with_le1_option_pct | 31.24 | 47.68 | **39.46** |
| width_m | 44.56 | 42.49 | **43.52** |

### Movement smoothness

| metric | home | away | teams mean |
|---|---:|---:|---:|
| accel_from_pos_mean_mps2 | 10.40 | 10.38 | **10.39** |
| accel_gt10_pct | 48.90 | 50.80 | **49.85** |
| accel_mean_mps2 | 10.39 | 10.34 | **10.37** |
| class_time_pct.0 | 8.03 | 4.99 | **6.51** |
| class_time_pct.1 | 2.58 | 4.19 | **3.38** |
| class_time_pct.2 | 66.76 | 76.58 | **71.67** |
| class_time_pct.3 | 22.63 | 14.24 | **18.43** |
| dspeed_max_mps_per_tick | 4.07 | 5.18 | **4.63** |
| heading_change_p95_deg_per_tick_gt3mps | 3.25 | 3.17 | **3.21** |
| heading_change_pct.0_10 | 99.62 | 99.69 | **99.66** |
| heading_change_pct.10_45 | 0.37 | 0.29 | **0.33** |
| heading_change_pct.45_90 | 0.01 | 0.01 | **0.01** |
| heading_change_pct.gt90 | 0.00 | 0.00 | **0.00** |
| heading_sign_flips_per_player_min | 16.48 | 10.36 | **13.42** |
| lateral_accel_mean_mps2 | 7.59 | 7.69 | **7.64** |
| reversals_per_player_min | 0.01 | 0.01 | **0.01** |
| side_flips_by_class.0_per_player_min | 305.7 | 100.6 | **203.1** |
| side_flips_by_class.0_share_pct | 58.75 | 21.39 | **40.07** |
| side_flips_by_class.1_per_player_min | 130.2 | 44.93 | **87.55** |
| side_flips_by_class.1_share_pct | 7.99 | 8.04 | **8.01** |
| side_flips_by_class.2_per_player_min | 14.97 | 17.46 | **16.22** |
| side_flips_by_class.2_share_pct | 23.93 | 57.00 | **40.46** |
| side_flips_by_class.3_per_player_min | 17.30 | 22.42 | **19.86** |
| side_flips_by_class.3_share_pct | 9.34 | 13.57 | **11.45** |
| side_flips_cause.desired_per_player_min | 29.26 | 6.18 | **17.72** |
| side_flips_cause.filter_per_player_min | 10.68 | 14.92 | **12.80** |
| side_flips_cause.limiter_per_player_min | 1.14 | 1.26 | **1.20** |
| side_flips_cause.sampler_per_player_min | 0.69 | 1.09 | **0.89** |
| side_flips_per_player_min | 41.77 | 23.46 | **32.62** |
| side_flips_schmitt_per_player_min | 32.59 | 34.96 | **33.78** |
| side_flips_speed_ge3_per_player_min | 50.35 | 25.75 | **38.05** |
| side_flips_speed_lt3_per_player_min | 12.15 | 14.06 | **13.11** |
| speed_oscillation_pct | 3.11 | 3.32 | **3.22** |
| turn_rate_mean_deg_per_s | 87.77 | 89.06 | **88.41** |

### Collisions and push-out

| metric | home | away | teams mean |
|---|---:|---:|---:|
| any.depth_max_m | 0.80 | 0.80 | **0.80** |
| any.depth_mean_m | 0.24 | 0.24 | **0.24** |
| any.events_per_min | 3.00 | 3.00 | **3.00** |
| any.stuck_mean_ticks | 4 | 4 | **4** |
| any.stuck_pairs | 0.33 | 0.33 | **0.33** |
| any.ticks_gt1cm_per_min | 7.90 | 7.90 | **7.90** |
| any.ticks_per_min | 8.86 | 8.86 | **8.86** |
| correction_vel_corrected_p50_mps | 0.83 | 1.09 | **0.96** |
| correction_vel_corrected_p99_mps | 10.59 | 12.05 | **11.32** |
| correction_vel_corrected_pct | 0.04 | 0.06 | **0.05** |
| correction_vel_max_mps | 12.57 | 13.39 | **12.98** |
| correction_vel_p50_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p95_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_p99_mps | 0.00 | 0.00 | **0.00** |
| correction_vel_pct_gt_0_3 | 0.03 | 0.05 | **0.04** |
| opponent.depth_max_m | 0.40 | 0.40 | **0.40** |
| opponent.depth_mean_m | 0.09 | 0.09 | **0.09** |
| opponent.events_per_min | 2.13 | 2.13 | **2.13** |
| opponent.stuck_mean_ticks | 0 | 0 | **0** |
| opponent.stuck_pairs | 0 | 0 | **0** |
| opponent.ticks_gt1cm_per_min | 3.13 | 3.13 | **3.13** |
| opponent.ticks_per_min | 3.87 | 3.87 | **3.87** |
| teammate.depth_max_m | 0.80 | 0.79 | **0.80** |
| teammate.depth_mean_m | 0.37 | 0.34 | **0.36** |
| teammate.events_per_min | 0.33 | 0.53 | **0.43** |
| teammate.stuck_mean_ticks | 0 | 4 | **2** |
| teammate.stuck_pairs | 0 | 0.33 | **0.17** |
| teammate.ticks_gt1cm_per_min | 2.27 | 3.10 | **2.68** |
| teammate.ticks_per_min | 2.33 | 3.33 | **2.83** |

### Coordination

| metric | home | away | teams mean |
|---|---:|---:|---:|
| cruise_over72_pct | 42.67 | 42.93 | **42.80** |
| dual_chaser_pct_of_loose | 12.16 | 15.45 | **13.80** |
| dual_chaser_ticks_per_min | 301.7 | 383.7 | **342.7** |
| engagers3_pct_of_carrier_ticks | 0.19 | 0.66 | **0.42** |
| engagers3_ticks_per_min | 0.73 | 4.80 | **2.77** |
| mark_steal_per_defender_min | 0.09 | 0.23 | **0.16** |
| max_simultaneous_chasers | 3 | 4.33 | **3.67** |

### Aerial

| metric | home | away | teams mean |
|---|---:|---:|---:|
| headers | 20.33 | 12.67 | **16.50** |
| headers_clear | 0.67 | 0.33 | **0.50** |
| headers_pass | 16 | 10.67 | **13.33** |
| headers_shot | 3.67 | 1.67 | **2.67** |
| jumps_gk | 5.33 | 23 | **14.17** |
| jumps_outfield | 54.33 | 30.33 | **42.33** |
| max_airborne_outfield | 1 | 1 | **1** |

### Team shape vs envelope (teams mean)

| metric | in possession | out of possession | 11v11 envelope (m) |
|---|---:|---:|---|
| length_m | 43.48 | 39.36 | 31-46 |
| width_m | 44.09 | 38.30 | 35-48 |
| stretch_m | 19.57 (outside) | 17.15 (outside) | 7-16 |
| nearest_mate_mean_m | 11.92 | 10.49 | none published (in-possession mean near 9-10 m, estimate) |
| nearest_mate_p10_m | 5.52 | 5.14 | none published (in-possession mean near 9-10 m, estimate) |

Envelope: peer-reviewed 11v11 ranges (length 31-46 m, width 35-48 m, stretch index 7-16 m) quoted in the research report; the report's tighter targets are 31-36 / 35-40 / 7-10 m out of possession and 40-46 / 42-48 / 12-16 m in possession (estimates).

### Per seed (home / away)

| metric | seed 1 | seed 2 | seed 3 |
|---|---:|---:|---:|
| aerial.headers | 21 / 11 | 25 / 12 | 15 / 15 |
| aerial.headers_clear | 2 / 0 | 0 / 1 | 0 / 0 |
| aerial.headers_pass | 18 / 9 | 18 / 9 | 12 / 14 |
| aerial.headers_shot | 1 / 2 | 7 / 2 | 3 / 1 |
| aerial.jumps_gk | 5 / 15 | 9 / 24 | 2 / 30 |
| aerial.jumps_outfield | 58 / 28 | 59 / 30 | 46 / 33 |
| aerial.max_airborne_outfield | 1 / 1 | 1 / 1 | 1 / 1 |
| collision.any.depth_max_m | 0.80 / 0.80 | 0.80 / 0.80 | 0.80 / 0.80 |
| collision.any.depth_mean_m | 0.25 / 0.25 | 0.23 / 0.23 | 0.26 / 0.26 |
| collision.any.events_per_min | 3.40 / 3.40 | 2.70 / 2.70 | 2.90 / 2.90 |
| collision.any.stuck_mean_ticks | 12 / 12 | 0 / 0 | 0 / 0 |
| collision.any.stuck_pairs | 1 / 1 | 0 / 0 | 0 / 0 |
| collision.any.ticks_gt1cm_per_min | 8.69 / 8.69 | 6.50 / 6.50 | 8.50 / 8.50 |
| collision.any.ticks_per_min | 9.59 / 9.59 | 7.50 / 7.50 | 9.50 / 9.50 |
| collision.correction_vel_corrected_p50_mps | 1.47 / 1.40 | 0.65 / 0.75 | 0.37 / 1.10 |
| collision.correction_vel_corrected_p99_mps | 10.51 / 11.03 | 9.19 / 12.85 | 12.07 / 12.28 |
| collision.correction_vel_corrected_pct | 0.05 / 0.05 | 0.04 / 0.05 | 0.05 / 0.08 |
| collision.correction_vel_max_mps | 12.28 / 12.94 | 12.86 / 13.38 | 12.57 / 13.85 |
| collision.correction_vel_p50_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p95_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_p99_mps | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| collision.correction_vel_pct_gt_0_3 | 0.04 / 0.04 | 0.03 / 0.04 | 0.02 / 0.06 |
| collision.opponent.depth_max_m | 0.51 / 0.51 | 0.13 / 0.13 | 0.55 / 0.55 |
| collision.opponent.depth_mean_m | 0.13 / 0.13 | 0.03 / 0.03 | 0.10 / 0.10 |
| collision.opponent.events_per_min | 2.20 / 2.20 | 2.20 / 2.20 | 2 / 2 |
| collision.opponent.stuck_mean_ticks | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.stuck_pairs | 0 / 0 | 0 / 0 | 0 / 0 |
| collision.opponent.ticks_gt1cm_per_min | 4.00 / 4.00 | 2.50 / 2.50 | 2.90 / 2.90 |
| collision.opponent.ticks_per_min | 4.60 / 4.60 | 3.20 / 3.20 | 3.80 / 3.80 |
| collision.teammate.depth_max_m | 0.80 / 0.77 | 0.80 / 0.80 | 0.80 / 0.80 |
| collision.teammate.depth_mean_m | 0.36 / 0.29 | 0.39 / 0.36 | 0.36 / 0.38 |
| collision.teammate.events_per_min | 0.50 / 0.70 | 0.20 / 0.30 | 0.30 / 0.60 |
| collision.teammate.stuck_mean_ticks | 0 / 12 | 0 / 0 | 0 / 0 |
| collision.teammate.stuck_pairs | 0 / 1 | 0 / 0 | 0 / 0 |
| collision.teammate.ticks_gt1cm_per_min | 3.70 / 2.80 | 1.60 / 2.40 | 1.50 / 4.10 |
| collision.teammate.ticks_per_min | 3.70 / 3.10 | 1.60 / 2.70 | 1.70 / 4.20 |
| coord.cruise_over72_pct | 43.45 / 42.66 | 43.04 / 41.74 | 41.51 / 44.39 |
| coord.dual_chaser_pct_of_loose | 10.92 / 14.83 | 13.58 / 13.76 | 11.96 / 17.75 |
| coord.dual_chaser_ticks_per_min | 280.1 / 380.2 | 332.3 / 336.7 | 292.7 / 434.2 |
| coord.engagers3_pct_of_carrier_ticks | 0 / 0.10 | 0.10 / 1.87 | 0.47 / 0 |
| coord.engagers3_ticks_per_min | 0 / 0.70 | 0.40 / 13.70 | 1.80 / 0 |
| coord.mark_steal_per_defender_min | 0 / 0.21 | 0.17 / 0.15 | 0.09 / 0.33 |
| coord.max_simultaneous_chasers | 3 / 4 | 3 / 5 | 3 / 4 |
| gk.catches | 1 / 0 | 1 / 2 | 1 / 2 |
| gk.claims_non_save | 9 / 3 | 7 / 2 | 5 / 1 |
| gk.excursion_mean_s | 1.08 / 1.15 | 1.02 / 1 | 1.02 / 0.79 |
| gk.excursions_gt1s_per_10min | 4.00 / 5.99 | 2 / 5 | 2 / 0 |
| gk.gathers | 10 / 3 | 8 / 4 | 6 / 3 |
| gk.goals_conceded | 0 / 4 | 1 / 6 | 0 / 9 |
| gk.goals_conceded_gk_gt3m_off_line | 0 / 3 | 0 / 4 | 0 / 7 |
| gk.goals_conceded_no_shot_found | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.line_dist_at_conceded_shot_mean_m | 0 / 3.33 | 1.56 / 3.50 | 0 / 4.05 |
| gk.line_dist_max_m | 13.29 / 7.82 | 8.52 / 7.18 | 8.36 / 7.13 |
| gk.line_dist_mean_m | 4.32 / 4.47 | 4.38 / 4.35 | 4.59 / 4.25 |
| gk.parries | 1 / 11 | 1 / 13 | 0 / 11 |
| gk.pct_outside_6yd | 1.43 / 1.73 | 0.68 / 1.17 | 0.51 / 0.93 |
| gk.pct_outside_box | 0 / 0 | 0 / 0 | 0 / 0 |
| gk.rush_count | 8 / 8 | 4 / 7 | 3 / 7 |
| gk.rush_per_10min | 7.99 / 7.99 | 4 / 7 | 3 / 7 |
| gk.saves | 2 / 11 | 2 / 15 | 1 / 13 |
| gk.tips_over | 0 / 0 | 0 / 0 | 0 / 0 |
| match.fouls | 1 / 0 | 1 / 0 | 0 / 0 |
| match.goals | 4 / 0 | 6 / 1 | 9 / 0 |
| match.offsides | 0 / 1 | 3 / 0 | 2 / 0 |
| match.passes_completed_pct | 76.72 / 73.55 | 80.81 / 74.64 | 73.89 / 77.78 |
| match.possession_pct | 67.72 / 32.28 | 63.60 / 36.40 | 66.76 / 33.24 |
| match.shots | 22 / 3 | 30 / 4 | 31 / 1 |
| match.shots_on_target | 16 / 2 | 23 / 3 | 17 / 1 |
| move.accel_from_pos_mean_mps2 | 10.33 / 10.30 | 10.52 / 10.40 | 10.36 / 10.44 |
| move.accel_gt10_pct | 48.13 / 50.25 | 49.77 / 51.02 | 48.80 / 51.11 |
| move.accel_mean_mps2 | 10.32 / 10.25 | 10.51 / 10.38 | 10.34 / 10.40 |
| move.class_time_pct.0 | 8.15 / 4.71 | 7.93 / 4.98 | 8.01 / 5.28 |
| move.class_time_pct.1 | 2.39 / 4.43 | 2.72 / 4.22 | 2.63 / 3.92 |
| move.class_time_pct.2 | 65.89 / 77.03 | 68.05 / 75.48 | 66.35 / 77.24 |
| move.class_time_pct.3 | 23.57 / 13.83 | 21.31 / 15.32 | 23.01 / 13.56 |
| move.dspeed_max_mps_per_tick | 3.80 / 6.37 | 3.79 / 4.95 | 4.63 / 4.21 |
| move.heading_change_p95_deg_per_tick_gt3mps | 3.24 / 3.16 | 3.27 / 3.19 | 3.23 / 3.16 |
| move.heading_change_pct.0_10 | 99.63 / 99.68 | 99.60 / 99.68 | 99.62 / 99.72 |
| move.heading_change_pct.10_45 | 0.35 / 0.31 | 0.39 / 0.30 | 0.36 / 0.26 |
| move.heading_change_pct.45_90 | 0.01 / 0.01 | 0.01 / 0.02 | 0.02 / 0.01 |
| move.heading_change_pct.gt90 | 0 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| move.heading_sign_flips_per_player_min | 16.41 / 10.38 | 16.91 / 9.82 | 16.12 / 10.88 |
| move.lateral_accel_mean_mps2 | 7.43 / 7.68 | 7.65 / 7.73 | 7.69 / 7.67 |
| move.reversals_per_player_min | 0 / 0.01 | 0.01 / 0.01 | 0.01 / 0.02 |
| move.side_flips_by_class.0_per_player_min | 295.7 / 96.50 | 315.6 / 102.4 | 305.8 / 102.7 |
| move.side_flips_by_class.0_share_pct | 57.81 / 19.91 | 58.83 / 21.69 | 59.61 / 22.58 |
| move.side_flips_by_class.1_per_player_min | 155.3 / 44.15 | 114.2 / 47.20 | 120.9 / 43.45 |
| move.side_flips_by_class.1_share_pct | 8.92 / 8.56 | 7.29 / 8.47 | 7.75 / 7.08 |
| move.side_flips_by_class.2_per_player_min | 15.24 / 16.88 | 15.15 / 17.70 | 14.52 / 17.81 |
| move.side_flips_by_class.2_share_pct | 24.10 / 56.99 | 24.23 / 56.75 | 23.45 / 57.27 |
| move.side_flips_by_class.3_per_player_min | 16.23 / 23.99 | 19.26 / 20.14 | 16.41 / 23.15 |
| move.side_flips_by_class.3_share_pct | 9.18 / 14.54 | 9.64 / 13.10 | 9.19 / 13.07 |
| move.side_flips_cause.desired_per_player_min | 29.34 / 5.72 | 29.71 / 6.32 | 28.73 / 6.51 |
| move.side_flips_cause.filter_per_player_min | 10.62 / 14.99 | 10.89 / 14.61 | 10.53 / 15.17 |
| move.side_flips_cause.limiter_per_player_min | 0.99 / 1.12 | 1.23 / 1.51 | 1.21 / 1.16 |
| move.side_flips_cause.sampler_per_player_min | 0.74 / 1.00 | 0.72 / 1.09 | 0.63 / 1.19 |
| move.side_flips_per_player_min | 41.68 / 22.82 | 42.55 / 23.54 | 41.09 / 24.02 |
| move.side_flips_schmitt_per_player_min | 31.97 / 34.67 | 33.25 / 34.93 | 32.54 / 35.29 |
| move.side_flips_speed_ge3_per_player_min | 50.53 / 24.79 | 50.86 / 25.91 | 49.66 / 26.54 |
| move.side_flips_speed_lt3_per_player_min | 11.85 / 14.83 | 12.98 / 14.11 | 11.63 / 13.25 |
| move.speed_oscillation_pct | 3.13 / 3.29 | 3.13 / 3.30 | 3.07 / 3.38 |
| move.turn_rate_mean_deg_per_s | 85.73 / 88.90 | 89.66 / 90.53 | 87.91 / 87.74 |
| shape.attacking.anchor_dist_m.defender | 21.45 / 17.08 | 22.39 / 18.28 | 21.37 / 14.58 |
| shape.attacking.anchor_dist_m.forward | 17.98 / 28.12 | 22.22 / 28.96 | 20.79 / 24.59 |
| shape.attacking.anchor_dist_m.goalkeeper | 2.41 / 2.33 | 2.27 / 2.34 | 2.30 / 2.29 |
| shape.attacking.anchor_dist_m.midfielder | 18.32 / 20.42 | 19.14 / 20.01 | 18.49 / 15.96 |
| shape.attacking.anchor_dist_shape_m.defender | 17.80 / 15.07 | 17.92 / 14.87 | 15.04 / 14.77 |
| shape.attacking.anchor_dist_shape_m.forward | 13.66 / 20.75 | 17.91 / 21.95 | 18.06 / 17.77 |
| shape.attacking.anchor_dist_shape_m.midfielder | 12.96 / 12.48 | 13.29 / 11.20 | 12.66 / 8.87 |
| shape.attacking.bbox_area_m2 | 2347.0 / 1640.1 | 2226.8 / 1731.1 | 2136.1 / 1722.3 |
| shape.attacking.bbox_depth_m | 48.26 / 39.20 | 47.84 / 39.71 | 46.70 / 39.17 |
| shape.attacking.bbox_width_m | 47.91 / 41.03 | 45.84 / 42.37 | 45.21 / 42.17 |
| shape.attacking.length_m | 48.26 / 39.20 | 47.84 / 39.71 | 46.70 / 39.17 |
| shape.attacking.nearest_mate_mean_m | 13.50 / 10.66 | 12.79 / 10.77 | 12.62 / 11.19 |
| shape.attacking.nearest_mate_p10_m | 6.84 / 4.33 | 6.39 / 4.47 | 6.53 / 4.53 |
| shape.attacking.pct_of_open_play | 67.72 / 32.28 | 63.57 / 36.43 | 61.92 / 38.08 |
| shape.attacking.stretch_m | 21.75 / 17.69 | 21.15 / 17.93 | 20.63 / 18.24 |
| shape.attacking.width_m | 47.91 / 41.03 | 45.84 / 42.37 | 45.21 / 42.17 |
| shape.defending.anchor_dist_m.defender | 23.41 / 19.28 | 21.06 / 18.13 | 17.95 / 17.59 |
| shape.defending.anchor_dist_m.forward | 21.20 / 24.58 | 26.32 / 27.73 | 21.45 / 27.38 |
| shape.defending.anchor_dist_m.goalkeeper | 2.54 / 2.52 | 2.49 / 2.63 | 2.40 / 2.62 |
| shape.defending.anchor_dist_m.midfielder | 19.11 / 17.56 | 18.47 / 18.24 | 14.46 / 18.67 |
| shape.defending.anchor_dist_shape_m.defender | 18.72 / 15.72 | 17.51 / 15.83 | 13.63 / 17.14 |
| shape.defending.anchor_dist_shape_m.forward | 16.80 / 21.52 | 19.98 / 22.34 | 17.38 / 20.86 |
| shape.defending.anchor_dist_shape_m.midfielder | 12.71 / 11.80 | 11.08 / 11.62 | 8.60 / 11.56 |
| shape.defending.bbox_area_m2 | 1897.9 / 1333.1 | 1724.9 / 1415.4 | 1674.2 / 1349.3 |
| shape.defending.bbox_depth_m | 44.39 / 36.17 | 41.49 / 36.84 | 41.92 / 35.38 |
| shape.defending.bbox_width_m | 41.60 / 35.77 | 39.95 / 36.92 | 38.85 / 36.68 |
| shape.defending.length_m | 44.39 / 36.17 | 41.49 / 36.84 | 41.92 / 35.38 |
| shape.defending.line_x_std_m | 8.32 / 4.21 | 7.09 / 4.80 | 5.69 / 4.44 |
| shape.defending.line_x_variance_m2 | 103.5 / 32.33 | 82.21 / 42.96 | 62.65 / 34.97 |
| shape.defending.nearest_mate_mean_m | 11.61 / 9.80 | 10.84 / 9.73 | 11.39 / 9.58 |
| shape.defending.nearest_mate_p10_m | 6.21 / 4.61 | 5.37 / 4.29 | 6.21 / 4.16 |
| shape.defending.pct_of_open_play | 32.28 / 67.72 | 36.43 / 63.57 | 38.08 / 61.92 |
| shape.defending.pct_outfield_behind_ball | 71.55 / 77.58 | 73.11 / 72.24 | 83.66 / 65.34 |
| shape.defending.stretch_m | 19.46 / 15.42 | 18.35 / 15.89 | 18.20 / 15.58 |
| shape.defending.swarm2_pct | 0.75 / 0.10 | 1.38 / 1.69 | 1.17 / 0.99 |
| shape.defending.swarm3_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| shape.defending.width_m | 41.60 / 35.77 | 39.95 / 36.92 | 38.85 / 36.68 |
| support.carrier_pct_of_open_play | 19.02 / 8.92 | 20.04 / 11.33 | 21.13 / 10.23 |
| support.cluster_pct | 4.99 / 18.57 | 4.09 / 16.99 | 2.66 / 10.48 |
| support.depth_m | 44.89 / 38.14 | 46.71 / 39.64 | 44.29 / 38.26 |
| support.forward_options_mean | 0.77 / 0.47 | 0.79 / 0.55 | 0.61 / 0.74 |
| support.nearest_option_dist_m | 13.27 / 11.08 | 13.95 / 11.56 | 14.28 / 11.52 |
| support.open_options_mean | 2.94 / 2.13 | 2.87 / 2.18 | 2.51 / 3.12 |
| support.open_options_p10 | 0 / 0 | 0 / 0 | 0 / 0 |
| support.pass_target_open_pct | 72.26 / 45.90 | 69.60 / 54.05 | 71.56 / 58.62 |
| support.passes_completed_tracked | 137 / 61 | 125 / 74 | 109 / 87 |
| support.release_pass_completed_pct | NaN / 0 | NaN / NaN | 0 / NaN |
| support.releases_per_10min | 0 / 1.00 | 0 / 0 | 1 / 1 |
| support.run_abandon_per_10min | 0 / 0 | 0 / 0 | 0 / 0 |
| support.runs_in_behind_per_10min | 5.00 / 1.00 | 11 / 5 | 7 / 1 |
| support.ticks_with_le1_option_pct | 28.88 / 51.70 | 29.70 / 51.99 | 35.13 / 39.36 |
| support.width_m | 45.38 / 41.74 | 44.74 / 43.32 | 43.56 / 42.41 |
| trans.count_per_10min | 19.98 / 36.96 | 22 / 37 | 20 / 37 |
| trans.counterpress_pct | 95 / 32.43 | 81.82 / 29.73 | 90 / 37.84 |
| trans.forward_recovery_speed_mps_0_5s | 7.61 / 6.66 | 7.37 / 7.10 | 7.22 / 6.90 |
| trans.forward_sprint_share_0_5s | 77.62 / 61.80 | 74.28 / 70.38 | 70.46 / 66.67 |
| trans.forward_sprint_share_0_5s_7mps | 67.37 / 50.18 | 62.84 / 57.78 | 59.50 / 54.28 |
| trans.forwards_staying_high_mean | NaN / 1.06 | 3 / 1.14 | 1.50 / 0.92 |
| trans.in_shape_at_turnover_pct | 85.71 / 60.71 | 65 / 62.50 | 72.22 / 54.17 |
| trans.mid_recovery_speed_mps_0_5s | 6.87 / 6.36 | 6.68 / 6.37 | 6.78 / 6.66 |
| trans.mid_sprint_share_0_5s | 68.00 / 57.67 | 65.56 / 57.93 | 66.32 / 64.34 |
| trans.mid_sprint_share_0_5s_7mps | 56.20 / 45.41 | 54.16 / 44.03 | 51.78 / 51.63 |
| trans.outfield_stamina_end_permille | 922.3 / 870.9 | 901.4 / 872.9 | 906.3 / 866.7 |
| trans.pct_outfield_behind_ball_at_3s | 67.27 / 73.45 | 72 / 69.05 | 72.50 / 64.07 |
| trans.pct_outfield_behind_ball_at_6s | NaN / 71.67 | 70 / 77.86 | 85 / 83.33 |
| trans.time_to_shape_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_excluded_pct | 30 / 24.32 | 9.09 / 13.51 | 10 / 35.14 |
| trans.time_to_shape_s | 0.61 / 0.65 | 0.76 / 0.92 | 0.71 / 0.96 |
| trans.time_to_shape_static_capped_pct | 0 / 0 | 0 / 0 | 0 / 0 |
| trans.time_to_shape_static_excluded_pct | 20 / 32.43 | 9.09 / 27.03 | 20 / 51.35 |
| trans.time_to_shape_static_s | 0.40 / 0.56 | 0.55 / 0.94 | 0.45 / 0.31 |

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

