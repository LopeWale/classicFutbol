# Attacking support and runs (round 4, work area 3)

This note describes how the AI teammates of the side in possession choose where to support the
carrier and when to run in behind, after round 4. The design follows
[05-fifa-pes-off-ball-teammate-intelligence.md](05-fifa-pes-off-ball-teammate-intelligence.md),
work area 3 and the decision-side part of work area 1. Code: `sim/psfc-sim/src/space.rs` (the
space-ownership primitives) and the attacking branch of `outfield_shape_intent` in
`sim/psfc-sim/src/ai.rs`. Evidence labels follow the report: [official], [patent],
[peer-reviewed], [code], [community], [audit], [estimate].

## 1. Space primitives (`space.rs`)

| Function | What it computes | Constants |
|---|---|---|
| `time_to_intercept(m, target)` | Spearman's arrival time: keep the velocity for the reaction time, then run straight | 0.7 s, 5 m/s [peer-reviewed; code] |
| `control(target, ball_time, attackers, defenders)` | Pitch control of a point for the attacking side, discrete integration of Spearman (2018) as in LaurieOnTracking; offside attackers zeroed | sigma 0.45 s, lambda 4.3 /s attackers, x1.72 defenders, dt 1/15 s, 40 steps [peer-reviewed; code; step estimate] |
| `value_at(point, goal)` | `0.93 e^(-0.14 d) + 0.02`, d = metres to the goal centre | [peer-reviewed] |
| `lane_open(from, to, receiver, defenders)` | Fraction of 5 speeds (10-22 m/s) x 5 angles (-16..+16 deg) of passes the receiver reaches first | defender lane reaction 0.3 s, reach 0.75 m, receive slack 0.6 s [peer-reviewed window; counts and timings estimate] |
| `pressure_factor(point, defenders)` | 1 with nobody within 4.6 m, falling to 0.25 for a defender on the point | 4.6 m [peer-reviewed] |
| `opponent_voronoi_vertices` / `nearest_voronoi_vertex` | Voronoi vertices of the opponents (empty-circle circumcentres), nearest one within a radius | 25 m [code: Gliders2d] |
| `offside_line_point(line_x, near, half_wid)` | Point on the onside limit nearest a reference | |

`exp_neg` and `logistic` are 65-entry Q16 lookup tables with linear interpolation. The API is pure
(positions and velocities in, Q32.32 out), so the defensive marking arbiter can reuse
`control` x `value_at` directly.

## 2. Scored support candidates

Every 20 ticks (or at once when the carrier changes), each non-rest-defence supporter of the side
in possession scores up to 8 candidates:

- the ball-attracted formation slot (after the support-triangle pull and the 5v5 cutback run);
- the four of the eight PES ring points nearest his slot (ahead, left, right, behind the carrier at
  L = 14 m, diagonals at L sqrt 2 = 20 m; 9 m / 12.7 m in 5v5), plus the incumbent ring point
  (Konami US 9,180,371 [patent]; L estimate inside the 13-24 m band [peer-reviewed]);
- the nearest opponent Voronoi vertex within 25 m (15 m in 5v5) [code; patent];
- the point on the offside line nearest his slot.

A candidate other than the slot is rejected when it is farther than 18 m (10 m) from the slot (the
PES base-position leash), outside the 6-30 m pass band, within 10 m (6 m) of another teammate's
live reserved destination (or, for a teammate without one such as the human-controlled player,
of where he stands), or offside. The score is

`Control(supporter vs every opponent, ball at 15 m/s) x LaneOpen x (Value + 0.05) x pressure`,

with LaneOpen multiplied by 0.35 when an opponent stands within 1.6 m of the straight lane. The
`0.05` safe-option bias is the "Chance Creation passing" weight (`SUP_SAFE_BIAS`); `SUP_RING_L_MM`
is the "Support Range". The incumbent is kept for 60 ticks and then until a challenger scores
1.15x higher, or dropped at once when it is rejected. Between re-scores the committed kind's
geometry is followed (ring points move with the carrier; the Voronoi vertex is frozen so it cannot
jump with the triangulation). With no scored candidate the player keeps his slot with GRF spacing
(six mates, weight 0.4, 10.5 m) as before. After spacing, reservation and run push, the lane to the
FINAL target is re-checked on re-score ticks and reopened by a sticky lateral nudge of 2 or 4 m.

Positions are read as they stand when the player is processed in the stepping loop (players
stepped earlier in the tick have moved). The order is fixed, so the result is deterministic.

## 3. Runs

One arbiter per team (`update_run_arbiter`), computed lazily by the first supporter processed in a
tick and stamped like the team plan (`run_eval`), assigns roles stored in
`TeamAiMemory::run_role`:

| Role | Who | Target |
|---|---|---|
| `RUN_PENETRATE` (max 1) | highest-scoring trigger-eligible runner | end-point beyond the line, held at the onside limit until released |
| `RUN_SUPPORT` (max 1) | second-highest | 6 m (4 m) behind the onside limit, toward the end-point's y |
| `RUN_BOX` (max 3) | on a cross (carrier beyond 60% of the half, wide of the box), nearest runner or midfielder to near post, back post, centre | box spot, clamped onside |
| `RUN_DECOY` (max 1) | a forward whose marker (within 3 m, moving with him) would raise a teammate's Control by more than 0.1 if dragged | onside limit, never released |
| `RUN_CALLED` | `BTN_CALL_RUN` | end-point beyond the line, hold and release, 90 ticks |

Trigger (re-evaluated every 20 ticks): the Schmitt `passer_ready`, no defender within 4.6 m of the
carrier (1.85 m for a CPU carrier), the runner onside and Control > 0.5 at his 1 s look-ahead
(position + velocity when faster than 2 m/s). Candidates are runner roles (striker, pivo, winger,
ala, wide midfielder, or `PlayerRole::Forward`) outside rest defence, considered every round,
every second or every fourth round for attacking work rates High (>= 67), Medium, Low (< 34)
(`PlayerAttributes::work_rates(role).0`). The end-point is the Voronoi vertex beyond the line (or a
grid point 5 / 9 m beyond, +-6 m laterally) that the runner, forecast moving toward it at 5 m/s,
reaches before every opponent, maximising Control x Value (EA US 7,588,493 [patent]). A new run is
committed for 60 ticks, then kept unless a challenger beats it by 1.15x; it expires after 240 ticks,
and ends on possession loss, a stoppage or when the runner gets the ball.

Hold and release: until released, the penetrate / called / decoy runner's target x is the onside
limit (second-last opponent or ball, minus 0.35 m); within 2.5 m of it he stutters at about 3 m/s
and never steps back while onside (FIFA 13 [official]). He is released by our pass toward him
(the receive path then meets the ball; the existing `run_boost` 45 on through balls stays), or by
the CPU carrier's pass decision that selects him (`run_release`), in which case he goes when the
remaining pass charge is at most 3 ticks so he is still onside at the kick. The CPU pass scorer
evaluates committed runners at their post-release look-ahead (1 s at 7 m/s toward the end-point)
instead of the 5.2 m velocity lead.

`BTN_CALL_RUN` (`assign_called_run`): the teammate within 45 degrees of the carrier's facing with
the best look-ahead Control (onside players first; if nobody is in the cone, the one most in line)
gets a called run with an end-point and the same hold and release, plus the 90-tick burst.

## 4. Decision-side hysteresis

| Item | Rule | Label |
|---|---|---|
| Support triangle | incumbent kept 18 ticks, then until a challenger is 2 m closer to 9.5 m | estimate |
| `passer_ready` | on > -0.10, off < -0.30, 12-tick hold; new carrier starts at -0.15 | estimate |
| `reserve_destination` | the pair's last assignment holds unless the keeper of the destination is 1.5 m farther | estimate; Konami US 8,616,946 [patent] |
| `yield_target`, `keeper_corridor_yield` | side sign sticky while the yield episode lasts | estimate |
| Curved-run bend | sign held 30 ticks after it was last applied | estimate |
| Committed target | 0.4 m re-target deadband, alpha-beta filter (role alpha, beta 0.02, velocity cap 0.12 m/tick) | stability region [peer-reviewed]; gains estimate |
| Target memory | receive, chase, contain and non-open-play paths write `target` / `target_tick` | audit |

The arrive dead zone (160-510 mm by discipline) was left as it is: a hard 0.4 m floor left
yielders short of the yield radius (`tests/exclusivity.rs`).

## 5. State

`TeamAiMemory` (all `#[serde(default)]`, hashed, sized in `ensure_len`): per player `sup_kind`,
`sup_since`, `sup_eval`, `sup_pt`, `sup_carrier`, `sup_nudge`, `yield_sign`, `gk_yield_sign`,
`bend_sign`, `bend_tick`, `res_yield`, `tgt_vel`, `tgt_raw`, `run_role`, `run_since`, `run_end`;
per team `tri_idx`, `tri_since`, `tri_eval`, `pr_on`, `pr_since`, `pr_eval`, `pr_carrier`,
`run_eval`, `run_round`, `run_release`, `run_release_tick`.

## 6. Measurement

`support.run_abandon_per_10min` (new): committed runs that end within 60 ticks for a reason other
than possession loss, a stoppage or receiving the ball. Before/after:

Three seeds x 10 minutes, `--preset baseline`; each cell is r4 baseline / base e53e13d / this work (teams mean; match.* per match).

| metric | 5v5 hard vs hard | 5v5 normal vs normal | 11v11 normal vs normal | 5v5 autopilot-hard vs hard | 11v11 autopilot-normal vs normal |
|---|---|---|---|---|---|
| support.open_options_mean | 1.35 / 1.28 / 1.33 | 1.25 / 1.30 / 1.24 | 3.68 / 3.76 / 4.04 | 1.07 / 0.98 / 1.06 | 2.86 / 3.28 / 3.11 |
| support.forward_options_mean | 0.28 / 0.27 / 0.24 | 0.26 / 0.26 / 0.25 | 0.74 / 0.80 / 0.70 | 0.33 / 0.30 / 0.30 | 0.74 / 0.80 / 0.68 |
| support.ticks_with_le1_option_pct | 53.02 / 57.13 / 54.75 | 58.55 / 56.53 / 58.17 | 19.98 / 20.58 / 19.36 | 63.12 / 66.68 / 63.16 | 36.89 / 29.81 / 34.11 |
| support.runs_in_behind_per_10min | 3.83 / 5.16 / 3.17 | 3.33 / 3.67 / 3.00 | 0.83 / 0.50 / 0.50 | 5.82 / 8.00 / 7.50 | 0.67 / 1.33 / 1.83 |
| support.pass_target_open_pct | 69.67 / 70.78 / 67.83 | 65.96 / 68.63 / 63.73 | 73.87 / 74.87 / 73.64 | 58.02 / 53.24 / 58.86 | 61.88 / 66.18 / 66.55 |
| support.cluster_pct | 5.92 / 6.21 / 8.00 | 6.97 / 6.32 / 7.64 | 4.63 / 4.73 / 4.75 | 8.00 / 10.09 / 10.42 | 7.39 / 5.17 / 7.90 |
| support.nearest_option_dist_m | 12.06 / 12.61 / 11.98 | 12.85 / 12.19 / 12.14 | 12.70 / 12.57 / 12.10 | 13.41 / 13.21 / 12.16 | 12.70 / 12.48 / 12.49 |
| support.run_abandon_per_10min | - / - / 0.00 | - / - / 0.00 | - / - / 0.00 | - / - / 0.00 | - / - / 0.00 |
| support.releases_per_10min | - / - / 15.82 | - / - / 11.17 | - / - / 3.67 | - / - / 6.00 | - / - / 1.33 |
| support.release_pass_completed_pct | - / - / 80.07 | - / - / 82.55 | - / - / 52.78 | - / - / 65.08 | - / - / 58.33 |
| match.offsides | 2.50 / 2.00 / 1.17 | 1.50 / 0.83 / 1.50 | 0.33 / 0.50 / 0.00 | 1.33 / 1.50 / 0.00 | 0.67 / 0.00 / 0.00 |
| match.passes_completed_pct | 89.21 / 87.91 / 88.22 | 84.56 / 86.07 / 88.27 | 84.15 / 84.23 / 85.06 | 77.79 / 75.99 / 77.76 | 80.06 / 79.38 / 77.44 |
| match.goals | 4.83 / 5.50 / 5.17 | 4.33 / 5.83 / 5.50 | 0.83 / 1.50 / 1.50 | 3.83 / 3.67 / 4.33 | 1.67 / 1.67 / 2.33 |
| match.shots | 26.67 / 28.33 / 27.33 | 26.17 / 26.50 / 29.00 | 7.67 / 6.67 / 6.33 | 21.33 / 17.83 / 24.00 | 11.33 / 8.33 / 9.67 |
| move.side_flips_per_player_min | 82.71 / 85.12 / 79.99 | 84.26 / 83.17 / 79.58 | 50.24 / 49.71 / 48.13 | 99.12 / 92.42 / 91.67 | 54.81 / 52.84 / 53.27 |
| move.side_flips_by_class.2_per_player_min | 71.70 / 73.88 / 66.22 | 73.86 / 73.18 / 67.01 | 41.53 / 40.41 / 37.75 | 66.10 / 61.80 / 57.57 | 39.37 / 37.76 / 36.22 |
| move.side_flips_by_class.3_per_player_min | 45.98 / 51.52 / 55.42 | 44.76 / 42.62 / 52.17 | 36.08 / 37.50 / 38.34 | 54.89 / 53.63 / 66.20 | 38.78 / 37.52 / 42.40 |
| move.class_time_pct.3 | 15.53 / 15.02 / 17.54 | 15.25 / 15.25 / 17.88 | 22.15 / 21.78 / 21.58 | 12.93 / 13.44 / 14.87 | 20.76 / 21.38 / 21.20 |
| collision.any.ticks_per_min | 2.47 / 2.03 / 3.06 | 4.57 / 3.33 / 2.50 | 2.40 / 1.53 / 2.70 | 3.73 / 3.97 / 6.06 | 9.20 / 5.40 / 5.07 |
| collision.teammate.ticks_per_min | 1.07 / 0.70 / 1.17 | 2.28 / 1.38 / 0.70 | 0.67 / 0.27 / 0.77 | 1.00 / 1.78 / 3.36 | 4.00 / 2.52 / 1.73 |

### Second pass (coordinator review)

- **Metric semantics.** `support.runs_in_behind_per_10min` also counts the intended receiver of
  our pass in flight and his reception tick; new `support.releases_per_10min` (CPU pass decisions
  selecting a committed runner, plus ground / through passes launched toward a penetrate or called
  runner) and `support.release_pass_completed_pct` (the runner is the next owner).
- **Through balls to committed runners.** The CPU pass scorer's through-ball candidate for a
  penetrate / called runner is the post-release look-ahead (7 m/s for 1 s toward his end-point);
  it is safe when `space::control(landing, ball_time, [runner moving at 5 m/s], defenders) > 0.7`
  (`RUNNER_PASS_CONTROL_MIN`; 0.55 completed fewer passes) instead of the standing-receiver
  margin, and gets a 4.0 killer-ball bonus (`RUNNER_KILLER_BONUS`) when the carrier faces forward
  and is not tight-pressed. Runs now end on a loose ball, the CPU-controlled receiver meets a pass
  to him instead of running on, committed runners are not served by lofted switches, and run
  end-points stay 6 m (3 m) inside the touchline.
- **Near support.** When the carrier (or, while our pass is in flight, its receiver at the pass
  target) has fewer than 2 open options, the two outfielders nearest him (within 24 m / 15 m, no
  run role, not rest defence) are sent to points 9-17 m (6.5-12 m) away with LaneOpen >= 0.6 and a
  clear straight lane, at least 30 degrees apart, scored Control x LaneOpen^2 x (Value + bias) /
  (1 + d / 4 m)^2 with the midfielders' leash at 25 m (13 m); held 60 ticks, re-scored every 20,
  dropped at 3 open options. Diagnosis on seed 1 (11v11): the slot won 63% of re-scores (mean 27
  m from the carrier, LaneOpen 0.61), the final target had an open lane on 82% of re-scores, and
  41% of the ticks with at most one open option fall in the first 0.5 s after a reception, which is
  why near support also anticipates the receiver. The support gates did not move materially.
- **Class-3 flips.** Bisected with the scorer, the arbiter, the deadband, the beta and the
  final-lane nudge switched off in turn (3 seeds, 5v5): only switching the arbiter off brought the
  autopilot rate back to base (53.6); the rest are within the +-5 seed noise. Class-3 time rises
  from 15 to 17.5-18% in 5v5 because forwards on a scored candidate no longer get the run push's
  class 2; relabelling them class 2 restored the rate but pushed 5v5 swarming above the
  exclusivity bound, so it was reverted. Not fixed.

## 7. Not done

- The scripted PES patterns (overlap when the winger comes inside, wing rotation, false 9).
- Build Up Speed as a named urgency weight (Support Range, CC passing risk and Crossing count are
  named constants: `SUP_RING_L_MM`, `SUP_SAFE_BIAS`, `BOX_RUNNERS_MAX`).
- The support gates (open options 11v11 >= 4.5 / 5v5 >= 1.8, le1 <= 12% / 40%, pass target open
  >= 80%, forward options >= 1.2) and class-3 flips at or below base are not reached.
