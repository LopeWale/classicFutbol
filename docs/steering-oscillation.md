# Steering oscillation and side flips (round 5, WA1)

Implementation of Work area 1 of `docs/05-fifa-pes-off-ball-teammate-intelligence.md` on top of
e53e13d (avoidance frame built before the CPU input generators). All numbers: `psfc-replay
analyze --preset baseline --seeds 3 --seed-base 1 --minutes 10`, mean of home and away.

## What changed

| Item | Where | Value | Label |
|---|---|---|---|
| Schmitt side flips, cause split | `psfc-replay` `move.side_flips_schmitt_per_player_min`, `move.side_flips_cause.{desired,sampler,filter,limiter}_per_player_min` | +-0.05 m/s, 4-sample dwell | estimate |
| Steering diagnostics | `PlayerState::steer_debug` (`#[serde(skip)]`, unhashed, unread by the sim) | - | - |
| velBias / weightCurVel | `avoidance::VEL_BIAS_MILLI`, `W_CUR_MILLI` | 0.6 / 1.25 (Detour 0.4 / 0.75) | code + estimate |
| Lateral-sign flip penalty, dead band | `W_FLIP_MILLI`, `FLIP_EPS_MM_S` | 0.75 / 0.05 m/s | estimate on Detour scale |
| Tie-break | `solve` | (penalty, dist^2 to previous selection) | ClearPath |
| Per-pair passing side | `SteerMemory::pair`, `W_SIDE_MEM_MILLI` | weight **0** (0.75 measured worse), clear on separation / 60 idle ticks | HRVO det [code] |
| Lateral dwell | `DWELL_TICKS`, `W_DWELL_MILLI`, `DWELL_OVERRIDE_TTC_MS` | 10 ticks / 3.0 / TTC < 0.3 s or touching | estimate; Hespanha |
| Steering filter | `ai::filter_velocity_command` | critically damped direction filter T 0.15 s; 0.09 s for CPU carrier, pass receiver, commands < 2.5 m/s, or while avoidance changed the command; first-order speed low-pass k = 0.25 + 0.002 agility | Juckett [code]; T estimate |
| Chaser anti-dither | `ai::chase_intent` | keep previous point unless moved > min(0.4 m, 15% of distance); near mode 1.5 m in / 1.9 m out | UvA Trilearn [code]; estimate |
| Carrier evade side | `ai::evade_side` | 18-tick dwell, then 0.2 score margin | estimate |

## Per-step contribution

11v11 normal:

| step | flips | class 0 | sampler | heading flips | speed osc % | collision ticks/min | pass % |
|---|---|---|---|---|---|---|---|
| base e53e13d | 49.71 | 184.23 | 9.13 | 19.57 | 3.14 | 1.53 | 84.23 |
| knobs | 49.17 | 187.90 | 8.75 | 18.90 | 3.10 | 2.67 | 85.90 |
| flip penalty | 41.25 | 179.37 | 2.58 | 17.31 | 3.05 | 2.10 | 85.57 |
| side memory 0.75 | 43.36 | 188.17 | 3.11 | 18.58 | 3.25 | 2.57 | 86.09 |
| dwell (side memory 0) | 41.12 | 176.06 | 2.52 | 17.19 | 3.14 | 2.27 | 85.01 |
| filter, first cut (speed unfiltered, receiver T 0.15) | 13.41 | 44.08 | 0.39 | 4.87 | 4.31 | 3.43 | 81.73 |
| filter, final | 16.21 | 58.06 | 0.60 | 6.21 | 3.32 | 2.70 | 82.95 |
| decision side (= final) | 16.57 | 59.47 | 0.65 | 6.37 | 3.39 | 3.00 | 83.91 |

5v5 hard:

| step | flips | class 0 | sampler | heading flips | speed osc % | collision ticks/min | pass % |
|---|---|---|---|---|---|---|---|
| base e53e13d | 85.12 | 178.73 | 19.92 | 36.81 | 4.27 | 2.03 | 87.91 |
| knobs | 79.45 | 168.53 | 18.48 | 32.60 | 4.06 | 2.10 | 88.70 |
| flip penalty | 66.24 | 176.52 | 5.41 | 30.89 | 4.14 | 3.47 | 88.01 |
| side memory 0.75 | 69.06 | 188.57 | 6.19 | 32.72 | 4.31 | 4.10 | 88.35 |
| dwell (side memory 0) | 66.25 | 176.72 | 6.05 | 30.84 | 4.21 | 4.19 | 88.78 |
| filter, first cut | 26.16 | 56.09 | 1.15 | 10.47 | 5.60 | 3.82 | 79.38 |
| filter, final | 31.71 | 69.73 | 1.79 | 13.65 | 4.33 | 4.16 | 83.92 |
| decision side (= final) | 31.05 | 66.86 | 1.74 | 12.83 | 4.27 | 4.53 | 82.43 |

Readings: at the base 60% of raw flips are `desired` (decision side), 20% sampler, 18% the old
first-order filter. The flip penalty removes about 70% of sampler flips; the knobs, dwell and
decision-side items sit inside seed noise; the second-order filter does almost all of the rest
because it suppresses desired-velocity flips before they reach the motion limits. The first cut
of the filter (speed unfiltered) raised `speed_oscillation_pct` by a third and, with the receiver
on the long time constant, cost 8 points of 5v5 pass completion; the final version restores the
speed low-pass and uses the short constant for the receiver.

## Open costs

- Collision ticks per minute are higher than base in most configs (5v5 roughly 1.5-2x; a second
  3-seed set from seed 4 confirmed the 5v5 increase, 11v11 was +25%). The filter lag carries
  engagers and chasers (class <= 1, which do not avoid the carrier, and class 0, which avoids no
  one) closer before they stop. `correction_vel_corrected_p99` is not worse except 11v11 seed set
  1 (5.9 -> 12.4; r4 baseline 10.9).
- 5v5 pass completion is 3-5 points lower (82-84% vs 86-88%).
- The autopilot (human-path) class-0 rate stays 170-200: the controlled player is not filtered
  (input latency invariant). Giving him the AI turn-rate limiter (`limit_turn_rate(.., false)`)
  was tried and measured no change (home side 5v5 class 0 322 -> 297, 11v11 266 -> 267, raw
  flips within noise), so it was reverted.

## Follow-up experiments (cost guards)

1. Collisions, hypothesis "filter lag near bodies": bypass the filter while any body is within
   1.5 m or the sampler changed the command by > 0.3 m/s in the last 3 ticks. Result: collision
   ticks unchanged within noise (5v5 hard any 4.53 -> 4.57, teammate 1.97 -> 1.85; 11v11 3.00 ->
   2.90; 5v5 autopilot 8.29 -> 8.73) while raw flips gave back most of the gain (11v11 16.6 ->
   25.3, 5v5 31 -> 51). Not kept; (b) (unfiltered closing component into `contact_constrain`) was
   not tried because (a) already removes all filtering near bodies and did not help.
2. Pass completion: (a) receiver never filtered: 5v5 pass % back to base (87.8 / 86.4 vs 87.9 /
   86.1) but class 0 back to 131-136. Kept as a compromise: the receiver is unfiltered within
   `ai::RECEIVER_UNFILTERED_MM` = 5 m of the ball and gets the 0.09 s constant farther out. 4 m
   tripped `tests/exclusivity.rs` 11v11 dual-chaser bound (51.8 vs < 50, chaotic: 6 m gave 29.5),
   6 m tripped `tests/team_shape.rs` 5v5 settled MID bound (4.74 vs < 4.5); 5 m passes both. Over
   two 3-seed sets 5v5 pass % is 86.8 / 88.0 / 86.7 / 87.8 vs base 87.9 / 86.1 / 86.8 / 88.3;
   class 0 rises from about 67 to 98-110 (gate <= 90 now missed by 8-20). (b) carrier exclusion
   was not tried: the earlier carrier on/off pair (s5b / s5c) showed no pass-completion cost from
   filtering the carrier (89.1 / 88.0).
