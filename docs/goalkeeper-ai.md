# Goalkeeper AI: scenario state machine

The goalkeeper (index 0 on each side) is driven by a small, deterministic scenario state machine in
`sim/psfc-sim/src/ai.rs` (`decide_goalkeeper`). Each tick it returns the movement / dive intent
(`AiMoveIntent`) and an updated `GoalkeeperBrain`. The simulation stores that brain in
`MatchState::gk_brain[side]`. The brain is serialised with `#[serde(default)]`, so older saves still
load, and it is part of `MatchState::compute_hash`.

```text
GoalkeeperBrain { scenario, commit_ticks, entered_tick, last_decision_tick, target, reentry_block_ticks, switches }
```

`compute_goalkeeper_intent(state, side, rush)` still exists. It is a pure wrapper that returns only the
intent.

## Why the old keeper "always came out"

The old `compute_goalkeeper_intent` re-evaluated everything from scratch every tick. It had no memory and
four outward triggers:

1. `rush_loose_ball`: sprint at *any* loose ball inside the box within 7.2 m, with no check on who gets
   there first.
2. "GK Rush to Contain": whenever an opponent had the ball in the box and no defender was within 2.65 m
   goal-side, the keeper ran out to a 1.45 m stand-off.
3. Cross command: sprint at any loose ball above 1.2 m within 6.2 m (5v5) or 7.6 m (11v11) of the goal
   line, with no reach check against attackers.
4. No commitment and no hysteresis, so the keeper flipped between the arc and a rush whenever a threshold
   was crossed.

Two attributes were barely used. `positioning` had no effect on keeper decisions, and `goalkeeping` only
set reaction latency and dive reach.

## Decision order (each tick)

1. **Possession**: the keeper owns the ball.
2. **SetPiece**: outside open play (dead-ball stance).
3. **HumanRush**: `BTN_GK_RUSH` is held.
4. **Shot**: an incoming shot. This beats every positional scenario, including a committed 1v1.
5. **Committed continuation**: if a committed scenario (`CrossClaim`, `Sweep`, `Collect`, `OneVOne`)
   still has `commit_ticks > 0`, it is re-checked with abort thresholds. If the check passes, the scenario
   continues. If it fails, the commitment is aborted and `reentry_block_ticks = 24`.
6. **Loose ball**: `CrossClaim` / `CrossHold` for a high ball, otherwise `Collect` (in the box) or `Sweep`
   (behind the line).
7. A live penalty delivery keeps the keeper on the line.
8. **OneVOne**: blocked during `reentry_block_ticks`.
9. **Set** (opponent ball, or a loose ball that is not the keeper's) or **Support** (own outfielder has the
   ball). Either becomes **Recover** when the keeper is out of position.

## Scenario table

| # | Scenario | Entry | Target / behaviour | Exit / abort |
|---|---|---|---|---|
| 1 | `Set` | Default when no other scenario applies | Angle-bisector arc. Depth = `min_arc + (d-4m)/(plateau-4m) * (max_arc-min_arc)` (5v5 1.38-3.8 m, plateau 22 m; 11v11 1.45-4.25 m, plateau 28 m), plus the `positioning` bonus ramped in from 5 m to 12 m of ball distance. Capped at goal-area depth - 0.6 m and at ball distance - 1 m. Target deadband: 0.7 m in depth, 0.4 m across. Jockey (feet set) within 1.25 m, or within 2 m when an opponent has the ball inside shooting range (20 m / 25 m). **Never sprints outward.** | Any other scenario's entry condition |
| 2 | `Shot` | Loose ball moving toward goal > 5.2 m/s within 26.5 m / 31 m, and either a live shot (`shot_launched_tick > 0`) or a fast (> 9 m/s) ball on frame | The existing save execution: reflex latency hold, then lateral shuffle or dive (low skim or airborne) | The ball is no longer shot-like, or its projected line is outside the guardable width |
| 3a | `CrossClaim` | High ball (z > 1.2 m, or rising > 2.5 m/s from > 0.25 m) whose path enters the box. The keeper's first reachable point (hands up to 2.9 m) must lie inside the box **and** within `command_radius` of the goal centre, with `gk_t + claim_margin <= best attacker t` (attacker header reach 2.5 m). A point outside the 6-yard box needs a further +6 ticks. **Punch rule:** inside the 6-yard box, a contested ball (`gk_t <= attacker_t + 2`) is attacked anyway, and the contact code punches it clear when crowded | Sprint to the claim point. Commitment `clamp(gk_t + 8, 12, 70)` ticks | Abort when the ball is owned, or an attacker is now more than 2 ticks earlier. Hands over to `Collect` if the ball drops |
| 3b | `CrossHold` | High ball into the box that the keeper cannot safely claim | Near-post / central stance 1.25 m / 1.45 m off the line, `y = 0.35 * landing_y`. Live corner: the stance is ±0.42 m toward the corner side. Jockey | The ball lands or is owned |
| 4 | `Sweep` | Loose ground ball whose keeper intercept point lies outside the box, within `sweep_radius` of the goal centre, and behind the defensive line (deeper than every own outfielder + 1 m, or no defender beats the attacker). Requires `gk_t + sweep_margin <= attacker t`, and no defender clearly first (`mate_t + 4 <= gk_t` and `mate_t + 2 <= opp_t` means "the defender handles it") | Sprint to the intercept. Commitment `clamp(gk_t + 6, 12, 75)` | Abort when the ball is owned or an attacker is now more than 2 ticks earlier. Then `Recover` |
| 5 | `OneVOne` | Opponent carrier within box depth + 2.5 m / 3.5 m, inside the box width, attacking the goal (velocity > 1.5 m/s toward it) or already in the box, and **no covering defender**: none goal-side within 2.2 m of the carrier-to-goal line, and none within 1.6 m of the carrier | Advance on the bisector. Far out: `0.42 * ball distance`. From half the box depth inwards, blend to `ball distance - standoff` (standoff 1.5-2.5 m by `positioning`). Capped at a style limit (goal area + 25 / 35 / 50 % of the rest of the box for line / commanding / sweeper). Sprint only when > 2.8 m from the target **and** the carrier is > 6 m away. Jockey (decelerate, stand big) within 2.2 m of the target or 4.5 m of the carrier. **Never dives**: only `Shot` dives. Commitment 24 ticks | Abort to `Set` when a defender recovers goal-side (cover width tightened to 1.4 m, tight radius 1.0 m) or the carrier drifts 3 m past the entry radius. A shot preempts the scenario (spread on the shot) |
| 6 | `Collect` | Loose ball whose keeper intercept point is inside the box, `gk_t + claim_margin <= attacker t` (+3 ticks beyond the 6-yard box), **no attacker within 1.5 m** of the ball or of the intercept point, and no defender clearly first | Sprint to the intercept. Commitment `clamp(gk_t + 6, 10, 60)` | As for Sweep. Becomes `Possession` once the ball is caught |
| 7 | `Possession` | The keeper owns or holds the ball | The existing outlet step toward `choose_safe_gk_distribution`. Distribution still fires in `sim.rs` when `gk_hold_ticks` reaches 0 | The ball is released |
| 7b | `Support` | An own outfielder has the ball. This persists through the team's own passes and through loose balls the team is first to | Set depth for the ball, shaded central (`y * 0.5`) as a square back-pass outlet | Opponent ball, or a loose ball the opponent is first to |
| 8 | `Recover` | More than 2.5 m from the Set / Support target (stays until within 1.0 m) | Full-speed sprint on the straight line back to the target. No jockey, never idles | Within 1.0 m of the target |
| 9 | `SetPiece` | Outside open play (`goalkeeper_set_piece_stance`: corner ±0.42 m at 1.45 m / 1.65 m; free kick within box + 14 m, wall-side: 0.9 m / 1.0 m off the line, bisector shaded 0.45 m to the far post; penalty on the line), or a live penalty delivery | Walk or jockey to the stance | The phase changes or the shot is detected |
| - | `HumanRush` | `BTN_GK_RUSH` held (outside attack context): loose ball within box + 4.5 m, or an opponent carrier within box + 5.5 m | The old behaviour: sprint at the loose ball, or contain at a 1.45 m stand-off on the bisector | Release, which returns to the normal scenarios (usually `Recover`) |

### Commitment and hysteresis

- **Committed scenarios** (`CrossClaim`, `Sweep`, `Collect`, `OneVOne`) hold for `commit_ticks`. While a
  commitment runs, only `Possession`, `HumanRush` and `Shot` can preempt it. Continuation uses abort
  thresholds with a 2-tick slack: the keeper keeps going unless an attacker is now clearly first.
- **Abort penalty**: after an aborted commitment, `reentry_block_ticks = 24`. During that window, outward
  loose-ball entries need a further +4 ticks of margin and `OneVOne` cannot be re-entered.
- **Stay hysteresis**: re-evaluating the scenario the keeper is already in uses a 2-tick lower margin
  (Cross / Collect / Sweep) or wider radii (1v1: +3 m radius, tighter cover test).
- **Set ↔ Recover**: enter beyond 2.5 m, leave within 1.0 m.
- **Set / Support target deadband**: 0.7 m in depth and 0.4 m across. Support persists through the team's
  own passes, so possession does not churn on every pass.

## Keeper style (derived, documented in `goalkeeper_style`)

| Style | Rule | Effect |
|---|---|---|
| Sweeper | `positioning >= 80 && pace >= 60` | `sweep_radius = box_depth + clamp(pos + pace - 140, 0, 100) / 200 * box_depth` (up to 1.5 x box depth). Sweep margin -2 ticks. 1v1 cap 50 % |
| Commanding | `goalkeeping >= 85 && strength >= 70` (otherwise) | `command_radius x 1.15`. Claim margin -1 tick. Sweep radius x 1.1. 1v1 cap 35 % |
| Line | Everyone else | `sweep_radius = ga + (box - ga) * clamp((pos + pace - 100) / 90, 0.25, 0.8)`. 1v1 cap 25 % |

The goal area ("6-yard box") is `penalty_box_depth / 3` deep: 5.5 m in 11v11 and 3.83 m in 5v5. Its half
width is `goal_half_width + depth`.

Sample keepers: Lammens (82 gk / 81 pos / 56 pace) is a **Line** keeper. Raya (88 / 87 / str 76) is
**Commanding**. Neither is a sweeper, because `PlayerAttributes::keeper` sets pace to 56.

## Attribute effects (`goalkeeper_profile`)

| Parameter | Formula |
|---|---|
| Set depth bonus | `clamp((positioning - 60) * 37, -500, 1500)` mm. Positioning 100 gives +1.48 m; positioning 46 or lower gives -0.5 m (hugs the line) |
| Command radius | `ga * clamp(900 + (gk + pos - 150) * 8, 750, 1350) / 1000` (x 1.15 for commanding). Lammens: 5.5 m in 11v11. Raya: 7.0 m |
| Claim margin | `max(2, base - (gk - 75) / 10 - commanding)` |
| Sweep margin | `max(4, base - (pos - 75) / 10 - 2 * sweeper)` |
| 1v1 standoff | `2500 - (clamp(pos, 50, 100) - 50) * 20` mm, so 2.5 m down to 1.5 m |
| Shot reflex | `clamp(13 - gk / 20, 7, 12) + difficulty adj` ticks (min 6), +3 when screened (unchanged line-of-sight check) |
| Dive reach | `1850 + 12 * gk` mm (unchanged) |

## Difficulty effects (CPU side; a human side's keeper uses Normal)

| | Claim base | Sweep base | Reflex adj | Judgement error |
|---|---|---|---|---|
| Easy | 9 | 12 | +3 ticks | ±6 ticks on its own arrival estimate (deterministic hash of `tick / 30` and side) |
| Normal | 6 | 9 | 0 | ±2 ticks |
| Hard | 4 | 7 | -1 | 0 (near-optimal) |

## CPU-vs-CPU metrics (Hard vs Hard, 10 minutes of open play, per keeper)

Harness: `cpu_vs_cpu_keeper_metrics_within_bounds` in `sim/psfc-sim/tests/goalkeeper_scenarios.rs`. The
"before" numbers come from the same harness run on the unmodified code (commit 10e07a3).

- A **rush** is the keeper leaving the 6-yard box while moving toward the ball without owning it.
- A rush is **won** when the keeper touches the ball within 1.5 s.
- A **flip** is a reversal of the keeper's outward/inward velocity, with a 1 m/s dead-band.
- A **flicker** is leaving a scenario after 4 ticks or fewer.

| Match | Before: rushes (won) H / A | After: rushes (won) H / A | Before: flips/min | After: flips/min | After: switches/min | After: flicker/min | Goals before -> after |
|---|---|---|---|---|---|---|---|
| 5v5 s11 | 38 (23) / 37 (20) | 1 (1) / 3 (2) | 19.5 / 20.6 | 21.2 / 26.0 | 43.5 / 39.6 | 4.9 / 3.8 | 18 -> 10 |
| 5v5 s22 | 39 (17) / 43 (16) | 4 (3) / 2 (2) | 21.0 / 23.0 | 24.4 / 22.4 | 44.3 / 40.8 | 4.5 / 3.8 | 22 -> 7 |
| 5v5 s33 | 32 (16) / 44 (17) | 1 (1) / 4 (3) | 23.2 / 21.3 | 21.9 / 19.2 | 37.9 / 41.0 | 2.5 / 3.0 | 14 -> 6 |
| 11v11 s11 | 10 (3) / 18 (5) | 10 (9) / 11 (9) | 9.8 / 13.1 | 15.5 / 16.5 | 29.4 / 34.4 | 2.1 / 2.5 | 11 -> 9 |
| 11v11 s22 | 10 (6) / 7 (3) | 13 (10) / 10 (8) | 10.1 / 10.1 | 22.7 / 15.2 | 35.5 / 29.8 | 2.3 / 2.3 | 12 -> 6 |
| 11v11 s33 | 11 (2) / 7 (4) | 10 (6) / 2 (1) | 14.4 / 8.5 | 27.5 / 11.9 | 40.7 / 31.6 | 4.4 / 2.5 | 10 -> 9 |

- **Outside the penalty box**: 0.00 % before and 0.00-0.03 % after. The old keeper rushed *inside* the box;
  he never left it.
- **Bounds asserted**: outside box < 2 %, rushes <= 16 per 10 min, switches <= 60/min, flicker <= 6/min.
- **5v5**: rushes fell about 90 % (38 -> 2.5 on average), and the share of rushes that win the ball rose
  from about 45 % to about 80 %.
- **11v11**: the rush count is similar (10.5 -> 9.3), but the share won rose from about 37 % to about
  77 %. The remaining 11v11 rushes are mostly `OneVOne` approaches and `Collect`.
- **Switches/min**: there is no "before" figure, because the old code had no scenarios. Most switches are
  genuine `Set` <-> `Support` possession changes.
- **Velocity flips**: similar in 5v5 and higher in 11v11 (about 11 -> 18 per minute). This comes from
  ordinary Set / Support tracking of a moving ball and Recover runs, not from scenario flicker.
- **Goals conceded fell** (5v5 about 18 -> 8, 11v11 about 11 -> 8 per 10 minutes). The keeper is no longer
  caught off his line. The analytics / balance owners should re-check scoring rates.

## Known limits and follow-ups

- Set-piece placement *before* the kick is done by `setup_set_piece` in `sim.rs`, and
  `step_all_players` does not run outside open play. `goalkeeper_set_piece_stance` therefore only takes
  effect if those callers adopt it. Live corner and penalty deliveries are handled by the brain.
- The GK jump-claim trigger in the aerial block and the catch / punch resolution in `sim.rs` are unchanged.
  The brain decides *where* the keeper goes. The contact code still decides catch, parry or punch.
