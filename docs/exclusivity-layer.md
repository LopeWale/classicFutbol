# Exclusivity layer: one chaser, one runner per destination, yield radius, cruise speed, keeper arbitration, priority classes

This is priority one of `reports/FIFA PES player movement coordination.md` ("Exclusive roles, not
repulsion, make CPU teams look coordinated"; "Recommended design for the Rust sim"). It lives in
`sim/psfc-sim/src/ai.rs` (team plan and outfield intents) and `state.rs` (`TeamAiMemory`). It does
not touch the goalkeeper decision code or the motion, push-out, tackle and ball-contact code in
`sim.rs`.

All decisions are made once per side per tick in `update_team_plan` and stored in `TeamAiMemory`.
The new fields are `chaser`, `chaser_ticks`, `chaser_eval_tick`, `chaser_pt`, `keeper_claim` and
`move_class`. Each has `#[serde(default)]` and is hashed in `compute_hash`. The rollback and replay
tests pass, and a state saved before these fields existed still loads.

## 1. Rules

### Elected chaser (pipeline stage 2)

`elect_chaser` picks at most one loose-ball chaser per team. Read it with
`ai::elected_chaser(state, side)`.

| Case | Chaser |
|---|---|
| The ball is owned | None |
| Keeper committed (`Collect`, `CrossClaim`, `Sweep`, or human `HumanRush`) | None, and `keeper_claim` is set |
| Our pass is in flight | The intended receiver |
| Human side | The human-controlled player is never elected. He *counts* as the chaser when his intercept time is within 6 ticks of the best AI teammate's, or when he is closing (on the ball, or running at ≥ 2 m/s toward his intercept point) and no AI teammate is clearly better by the ratio rule. AI teammates then stand off |
| Otherwise | The best intercept time wins (`best_intercept_from_waypoints`, the existing waypoint machinery) |

The election uses these rules:

- **Contest gate (agent2d):** a new chaser is elected only if `t ≤ 18` or `t < t_opp + 18` ticks.
  This is agent2d's `self ≤ 3 || self < opp + 3`, with its 100 ms cycles converted to 60 Hz ticks.
- **Release:** an incumbent gives the ball up only when the opponent is first by 30 ticks.
- **Hysteresis (GRF):** the incumbent keeps the role unless `(best + 30)·10 < (incumbent + 30)·8`
  (ratio < 0.8, with K = 500 ms = 30 ticks). Challengers are checked every 8 ticks.
  An invalid incumbent (sent off, ball owned) is replaced at once. Ties go to the lower index.
- **Ranking costs:** centre-backs pay +14 ticks when the ball is more than 8 m beyond the line (the
  same rule as the presser ranking). The CPU-controlled player gets −4 ticks, which agrees with the
  CPU auto-switch.

Once elected:

- The chaser runs `chase_intent`: the continuous intercept, and the ball itself once within 1.5 m.
- Every other outfielder runs his normal tactical intent.
- On a CPU side the controlled player chases only if he is the elected chaser. Otherwise he follows
  `outfield_shape_intent` like his AI teammates. The off-ball branch of `generate_cpu_input`
  implements this, and the old contain/chase movement is now `cpu_engage_move`.

### Engager cap

On a loose ball that a chaser or the keeper owns exclusively, there is no presser and no cover.
Everyone else goes to his slot.

With an opposition carrier:

- The original presser rule is kept: an AI presser is committed only when he is clearly faster than
  the controlled player.
- When an AI presser is committed, a CPU-controlled player stands off and becomes the cover.
  Otherwise the controlled player engages himself.
- The chaser is never the presser.
- Every non-presser keeps a goal-side gap from the carrier: 2.2 m, or 3 m for the cover. The second
  presser in counter-press, previously exempt, now keeps the gap too.
- At most two players engage the carrier: the presser (or the controlled player) and the cover.

### One runner per destination and support spacing (possession only)

Possession here means we own the ball, or our pass is in flight.

- **Support field (GRF `GetSupportPosition_ForceField`):** the six nearest teammates repel the
  attacking target with weight 0.4 × (scale − d). The scale is 10.5 m in 11v11 and 6.5 m in 5v5.
  The total shift is capped at 3 m (11v11) or 2 m (5v5).
- **Reservation (Konami US 8,616,946):** a teammate's committed target, refreshed this tick or last
  tick, reserves a 6 m disc (5 m in 5v5).
  - If my target falls inside the disc, the player farther from that destination moves his target
    out to the disc edge. Ties go to the higher index. Two passes are made.
  - The carrier and the chaser hold no destination.

### Teammate yield radius (Konami US 2010/0240429)

The reserved path is one of these:

- our carrier's next 0.5 s (`pos → pos + 0.5·vel`)
- the elected chaser's run to his intercept point (`chaser_pt`)
- the presser's run, on a loose ball that nobody is elected for

A teammate yields in two cases:

- His target lies within R of that path. The target moves to the nearest path point plus the radial
  through his current position, scaled to `R + L·(1 − d/R)`. That distance is never less than R and
  is continuous at `d = R`.
- He is already standing inside R. He steps out from where he is, so he does not cross the path.

The radial is replaced by the perpendicular on his side when it would run along the path or off the
pitch (the patent's rotation by θ). Back-line defenders in a defensive phase have two exceptions:
they yield only by target, and they move sideways at the same depth when that clears R. A yielding
player is class 3.

### Off-ball cruise speed (Konami, official 70%)

- A class-3 player never sprints. His move command is capped at `0.70 × sprint_mult` (top speed =
  `pace × stamina × sprint_mult`, with `sprint_mult` 1.22 in 11v11 and 1.205 in 5v5).
- Class 0–1 players and triggered tasks (class 2) sprint as before.
- Triggered tasks are recovery runs, retreat sprints, counter runs, reserved attacking runs,
  `run_boost`, marking, being out of position beyond the sprint threshold, and back-line or holding
  players keeping the line in a defensive phase.

### Keeper–defender arbitration

- While `keeper_committed(state, side)` is true, no outfielder is elected chaser.
- The corridor from the keeper to `gk_brain.target` (the claim point) is reserved. It extends 1.5 m
  past the claim point and is 1.8 m wide on each side (1.5 m in 5v5).
- A defender whose target or position is inside the corridor steps aside laterally to 1.2 m outside
  it, on his own side.
- The goalkeeper code is unchanged; the layer only reads `gk_brain`.

### Movement priority class: `ai::move_priority(state, side, idx) -> u8`

| Class | Who |
|---|---|
| 0 | The ball carrier, the elected chaser (including the human-controlled player when he counts as the chaser), and the keeper while committed to a loose ball |
| 1 | Engagers: presser, second presser, cover. Also the human-controlled player when he is not class 0 |
| 2 | Tactical movers: triggered tasks (see the cruise-speed section) |
| 3 | Idle or cruising to his slot, or yielding. These players yield most |

The function is cheap and deterministic:

- It reads the tick's plan through `try_borrow`, so it is safe while `ai_memory` is mutably
  borrowed (it then returns 2).
- The plan is fresh when `plan_tick ∈ {tick, tick+1}`. Outside open play, or with a stale plan, it
  returns 0 for the carrier and 2 for everyone else.
- A keeper that is not committed is class 2. Sent-off players are class 3.
- `assign_move_classes` writes the baseline values at the end of `update_team_plan`, and
  `outfield_shape_intent` refines each AI player's own entry (1, 2 or 3).

Other public read-outs: `keeper_committed`, `yield_target` (pure geometry), and `shape_slot_anchor`
(the phase-block slot anchor before refinements).

## 2. Parameters

| Parameter | Value | Unit | Label |
|---|---|---|---|
| Chaser switch rule | `(best+30)·10 < (inc+30)·8` | ticks | derived from GRF 0.8 ratio, 500 ms |
| Challenger evaluation period | 8 | ticks | estimate (brief: 6–10) |
| Contest gate / release margin | 18 / 30 | ticks | agent2d 3 cycles → ticks (derived) / estimate |
| CPU-controlled ranking bonus | 4 | ticks | estimate |
| Centre-back hold-line chase cost | +14 when the ball is > 8 m beyond the line | ticks | existing presser rule |
| Human "closing" | on the ball (≤ 2.5 m) or ≥ 2 m/s toward the intercept | m, m/s | estimate |
| Engagers on the carrier | ≤ 2 (presser or controlled player + cover) | — | GRF hunters cap 2 (official code) |
| Non-engager gap / cover gap | 2.2 / 3.0 | m | existing anti-swarm |
| Destination reservation | 6 (5v5: 5) | m | brief; Konami US 8,616,946 gives no value |
| Support-field teammate repulsion | weight 0.4, scale 10.5 (5v5: 6.5), 6 nearest, cap 3 (2) | —, m | GRF (official code); 5v5 scale and caps estimate |
| Yield R / L | 3.0 / 2.0 (5v5: 2.5 / 1.5) | m | estimate (report: 2–3 / 1–1.5; brief: ≈3 / ≈2) |
| Carrier near-term path | 0.5 | s | estimate |
| Cruise speed | 70% of top speed | — | official (Konami) |
| Keeper corridor half-width / overrun / step-out | 1.8 (5v5: 1.5) / 1.5 / 1.2 | m | estimate |

## 3. Before / after

Source: `sim/psfc-sim/tests/exclusivity.rs`. CPU vs CPU, idle inputs, 3 seeds × 10 match-minutes:
5v5 Hard vs Hard and 11v11 Normal vs Normal. "Before" is commit 2dd0e3b measured with the same
behavioural metric code.

How the metrics are defined:

- A behavioural chaser is on a loose ball, at ≥ 4.5 m/s, ≤ 25 m from the ball, heading
  (cos ≥ 0.92) at his own best intercept point, and within 20 ticks of the team's best intercept
  time.
- A contact is a teammate pair closer than 0.9 m.

| Metric | 5v5 before | 5v5 after | 11v11 before | 11v11 after |
|---|---|---|---|---|
| Behavioural dual-chaser ticks / open-play min | 158.3 | 68.7 | 87.7 | 26.0 |
| Elected-chaser invariant violations (more than one class-0 outfielder) | n/a | 0 | n/a | 0 |
| Chaser switches / loose-ball min (switches < 8 ticks apart) | n/a | 3.9 (0) | n/a | 5.0 (0) |
| Teammate pairs < 0.9 m / open-play min | 135.1 | 42.5 | 87.2 | 30.6 |
| Swarming (≥ 2 defenders within 1.6 m of the carrier) | 1.11% | 0.54% | 1.18% | 0.45% |
| ≥ 3 defenders within 3 m of the carrier | 0.20% | 0.08% | 0.10% | 0.30% |
| Nearest-teammate mean in possession | 10.40 m | 10.89 m | 11.78 m | 12.12 m |
| Nearest teammate < 4 m in possession | 17.1% | 11.9% | 6.3% | 4.3% |
| Goals / shots per match | 9.67 / 41.0 | 9.67 / 44.7 | 1.33 / 13.7 | 1.67 / 10.3 |
| Passes per match (completion) | 377 (85%) | 361 (86%) | 316 (84%) | 322 (84%) |

Team-shape suite (`tests/team_shape.rs`, unchanged), 6 seeds (`TEAM_SHAPE_SEEDS=6`), before → after:

| Metric | 5v5 | 11v11 |
|---|---|---|
| Swarming | 1.11% → 0.64% | 1.13% → 0.24% |
| Settled anchor distance DEF / MID / FWD (m) | 2.86 / 2.46 / 2.99 → 4.13 / 3.98 / 3.20 | 4.14 / 3.77 / 6.14 → 3.42 / 3.48 / 4.89 |
| Line-height jitter (mm/tick) | 51.4 → 59.0 | 58.3 → 58.7 |
| Presser changes / defending-min | 0.7 → 1.2 | 0.6 → 1.6 |

The 3-seed gate the suite asserts on passes. On 6 seeds, the base commit itself fails three 11v11
bounds: line jitter 58.3 (bound < 58), settled DEF 4.14 (< 4) and settled FWD 6.14 (< 6). Those
bounds sit at the noise edge. With 3 seeds, small changes move them by ±1 m and ±1 mm/tick.

**Reading the numbers:**

- Dual chasing falls by 57% (5v5) and 70% (11v11), and teammate contacts by 69% and 65%.
- The residual behavioural "dual chasers" are not a second elected chaser (the invariant is 0).
  They are the chaser plus a class-2 player whose run or mark heads the same way: a runner
  continuing his run toward a pass, or a marker tracking the receiver.
- Most of the remaining contacts are two teammates running the same lane to destinations far apart,
  or one teammate crossing another's path. These are path conflicts, not destination conflicts, so
  they belong to the velocity-stage avoidance (Area J), which can consume `move_priority`.
- The nearest-teammate mean was already above the report's estimates (9–10 m for 11v11, 6–7 m for
  5v5) and stays there. The support field repels only within its scale, so it trims the clustered
  tail (time under 4 m falls by about a third) rather than lowering the mean. Pulling the mean down
  would need a team-shape change (block width or length), which this layer does not make.
- ≥ 3 within 3 m rises slightly in 11v11 (0.10% → 0.30%). The cover's 3 m gap and the 2.2 m
  anti-swarm gap both fall inside a 3 m disc. Within the 1.6 m contain distance (swarming) the rate
  falls by 2.6× (1.18% → 0.45%).
- 5v5 settled-defence anchor distances rose: the controlled CPU player now stands off as cover
  instead of pressing alongside an AI presser. They stay inside the suite's bounds.

## 4. Tests

`sim/psfc-sim/tests/exclusivity.rs` uses the public API only.

- `loose_ball_has_exactly_one_chaser_with_hysteresis`:
  - at most one class-0 outfielder per team on every tick, and it is the elected chaser
  - a 0.6 m-closer challenger never takes over
  - a clearly better one takes over within 9 ticks and keeps the role
  - the non-chaser never runs at the ball
- `teammate_inside_yield_radius_moves_out_along_the_radial`: the carrier is walked onto a teammate's
  slot until the slot is within R. The teammate settles ≥ R − 0.15 m away, on his radial, and no
  teammate ends inside R.
- `yield_target_geometry`: the pure-function cases (≥ R, continuity, along-path teammate).
- `uninvolved_players_cruise_below_seventy_percent_of_top_speed`: a player who has been class 3 for
  20 consecutive ticks stays ≤ 0.72 × top speed while the shape shifts.
- `committed_keeper_collect_has_no_outfield_chaser_and_defender_steps_aside`:
  - no chaser and no class-0 outfielder while the keeper is in `Collect`
  - the keeper is class 0
  - a defender placed on the keeper→claim line moves laterally away from it (lateral gain greater
    than his movement along the line) and does not take the ball
- `exclusivity_metrics_5v5_hard` / `exclusivity_metrics_11v11_normal`: the table above, with bounds
  (the invariants at 0; dual-chaser, contact, swarming and spacing bounds; goals, shots and pass
  completion so that play is not broken).

## 5. Deliberately left out

- **Hungarian slot assignment every 24 ticks with an incumbent bonus.** The existing exact or 2-opt
  assignment with a 30-tick hold already gives exclusive slots. It was left alone to avoid churning
  the tuned shape metrics.
- **Contain Stamina budget and cooldown for a second presser, and the press-suppression region near
  goal (Konami US 8,398,487).** These change defensive intensity, not exclusivity.
- **The man-marker cap of 3.** Marks are already exclusive (`taken[]`). The cap was not added
  because the zonal mark count rarely exceeds it.
- **Pitch-control or Voronoi scoring of run destinations.** Reservation applies to the existing
  pocket, run and slot targets.
- **Keeper rules:** the "nearest defender covers the goal-line point" part of the claim protocol,
  and the 18–30-tick claim flag. The layer reads the keeper's own committed scenario (whose commit
  timer already acts as the flag) and does not change the goalkeeper module.
- **Path-crossing avoidance between non-class-0 players.** That is Area J's velocity-stage solver.

## Calling the CPU input generator from outside the tick

`ai::generate_cpu_input` now runs the team plan (`update_team_plan`) and, for the controlled player,
the shape intent with its smoothed target memory. Both write into `MatchState::ai_memory`, which is
serialized and hashed. Inside `MatchSimulation::tick` that is the intended once-per-tick update for a
CPU side. Anything that wants a CPU suggestion for a *human* side from outside the tick (the WASM
`sim_auto_step` auto-pilot, HUD hints, tools) must call `ai::suggest_cpu_input`, which snapshots and
restores the AI memory around the call and leaves `state_hash` unchanged. Without it the recorded
input log no longer reproduces the match: the host's `psfc-replay verify` reported a checkpoint hash
mismatch for auto-piloted matches when this layer first landed. `tests/autopilot_replay.rs` pins the
invariant (suggestion leaves the hash unchanged; an auto-piloted log replays to identical hashes).
