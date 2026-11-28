# Team-shape AI: formation shapes by phase, role behaviours, style, movement model

This covers the outfield defensive and team-shape AI in `sim/psfc-sim` (`ai.rs`, `config.rs`,
`state.rs`, and the motion integration in `sim.rs`). Goalkeeper AI is not covered here.

## 1. Team plan, once per side per tick

`update_team_plan` (ai.rs) runs on the first AI outfield call of each side's tick. It writes into
`MatchState::ai_memory: RefCell<[TeamAiMemory; 2]>`. The field has `#[serde(default)]`, is
serialized, and is included in `compute_hash`. It needs interior mutability because the AI plans
from `&MatchState` in the middle of a tick. A state saved before this field existed still loads.

| Step | What it decides | Hysteresis / commitment |
|---|---|---|
| Ball reference | `ref_ball` is the ball position plus 0.12 s of ball velocity, low-passed. This sets the reaction delay. | Per-tick weight = `react_pm × (0.7 + 0.3 × team positioning)`: Easy 140‰, Normal 230‰, Hard 360‰ |
| Phase | Attack, Defend, Counter-press, Retreat or Counter-attack (below) | Ownership must change to leave a phase, and each transition has a timer |
| Slots | Player → formation slot. Uses the exact 24-permutation optimum for 4 outfielders, and 2-opt from the previous assignment otherwise | A switch costs ~3 m (900 dm²) per moved player, an assignment is held for ≥ 30 ticks, and a role-band mismatch costs (band diff)² × 60 000 dm² |
| Presser | The AI outfielder with the best time-to-ball, adjusted by style | The current presser is kept unless a challenger is better by `margin` ticks (Easy 10, Normal 8, Hard 6), or by 3 ticks once the presser has been held for `hold` ticks (40 / 28 / 20). He hands over to the CPU-controlled player only when that player is clearly faster |
| Lane-closers / second man | Counter-press: two lane-closers (`second_presser`, `third_presser`) on the two best receiver lanes; never back-liners. Outside it, on a press trigger (heavy touch, back to goal, touchline), a second man shadows 3.5 m behind-side of the presser (round 4, area T; `docs/transition-and-defending.md`) | Incumbent lanes keep a 3 m bonus |
| Cover | The player nearest the cover point: half-way between the ball and the presser, plus 4.5 m (5v5) or 6.5 m (11v11) toward our goal. CB, holding mid and fixo are preferred | The current cover gets a 3 m bonus |
| Marks | One arbiter (round 4): incumbents first, then free opponents in danger order to the nearest free defender within 7 m (5v5) or 9 m (11v11) of his defend anchor. The rest offence and Low-work-rate front slots do not mark; in counter-press only the back line and holding slot mark. A back-liner keeps a runner in behind until a deeper teammate covers his line to goal | Incumbent kept while within 1.35 × the zone; after a 24-tick dwell he may switch to a free opponent 3 m closer |

### Phases

```
            we win it                 timer (Counter 150 / Press 100 / Possession 60 ticks)
 DEFEND ─────────────────► COUNTER-ATTACK ───────────────────────────────► ATTACK
   ▲                                                                          │ we lose it
   │ shape reached (80%) or 570 t                                             ▼
   └──── RETREAT ◄── decision after 6/9/12 t: own >= theirs within 10 m (7 m), ── COUNTER-PRESS
            ▲        >= 4 (2) behind the ball, ball beyond 35%, work-rate > 450? no   (reacting)
            │                                                                    │ yes
            └── window 300 t × style (1.2/1.0/0.6) × work rate, floor 180; ── COUNTER-PRESS
                stoppage; pass > 16 m; ball escapes 9 m (5v5) / 14 m (11v11)
```

Round 4, area T (`docs/transition-and-defending.md`) replaced the 40-150 tick counter-press and the
75-tick RETREAT with the measured football windows above.

## 2. Shapes per formation and phase

`config::formation_shape(formation, n)` parses any `"a-b-c[-d]"` string whose rows add up to the
outfield count. Unknown strings fall back to the default split, the same as `formation_anchors`. The
role of each slot comes from its row and position in the formation, not from the player listed at
that index. `role_aware_formation_anchors` now maps kick-off and set-piece anchors by role too.
Before this change, the 11v11 sample squads (listed GK, DEF, MID, MID, FWD, DEF, …) put a forward at
right-back.

Each slot has `(attack_depth, attack_y)` and `(defend_depth, defend_y)`. Depth runs from 0 (back
line) to 1000 (front line) of the block. y is in permille of the pitch half-width. The role
transforms are:

| Role | Defend | Attack |
|---|---|---|
| CB | depth 0 (holds the line), y = base | depth 0, y × 1.25 (splits wide to build up) |
| FB | depth 30, y × 0.70 (tucks in) | depth 540, y × 1.12 (overlaps). Ball-side FB +150 depth. Far-side FB drops to depth 260 and y × 0.65 (rest defence) |
| Holding (CDM) | depth 280, y × 0.5 (screens in front of the CBs) | depth 190 (stays behind the ball) |
| CM | depth ≥ 400 | depth ≥ 540 |
| Wide mid | depth 420, y × 0.96 (holds width) | depth 720, y × 1.12 |
| Winger | depth 640, y × 0.86 | depth 900, y × 1.08 |
| Striker | depth 1000 (leads the line). The rest offence (one Low-work-rate striker/pivo) stays an outlet in 11v11: up to 6 m beyond the ball, never past halfway. Every other striker, pivo and winger drops to a work-rate holding line goal-side of the ball (Low 4 / Medium 7 / High 10 m; 5v5 2 / 3.5 / 5 m), no deeper than 55% / 35% / 15% of the block (round 4) | depth 1000, pinned to the opponents' offside line − 0.4 m |
| Fixo (5v5) | depth 0 | depth 0, and always ≥ 8.8 m behind the ball (rest defence) |
| Ala (5v5) | depth 430 | depth 660, y × 1.15 |
| Pivo (5v5) | depth 1000 | pinned to the offside line |

Examples (attacking → right, `G` = goal; defend is compact and narrow, attack is long and wide):

```
4-3-3 DEFEND (ball in midfield)              4-3-3 ATTACK
|          W     ST     W        |           |        FB           W           |
|G   FB  CB CB  FB   CM H CM      |   →       |G  CB   H     CM         ST       |
|                                 |           |   CB         CM               W  |
                                             |        FB(tucked)                 |

1-2-1 DEFEND (5v5)                           1-2-1 ATTACK
|G  F   A A   P |                            |G   F        A      P |
                                             |               A       |
```

### Block geometry (millimetres along the attack direction from our goal line; "ball" means `ref_ball`)

- **Defend:** `line = min_line + 0.55 × (ball − pivot)`, never closer than `gap_min` to the ball.
  | | min_line | pivot | gap_min | block length | len_min | width | ball-side shift |
  |---|---|---|---|---|---|---|---|
  | 5v5 | 4.5 m | 10 m | 8 m | 17 m | 9 m | 0.70 | 0.32 |
  | 11v11 | 7 m | 16 m | 14 m | 30 m | 26 m | 0.72 | 0.38 |

  - **Offside tie:** the line never stays above the deepest opponent unless offside protects it. That
    means the opponent's attacking third in 5v5, or our half in 11v11:
    `line ≤ max(deepest_opp − 0.8 m, offside zone)`.
  - **Drop:** the line drops 2 m (5v5) or 3.5 m (11v11) × team positioning when the carrier is
    unpressured (nobody within 2.8 m) and facing our goal.
  - **Retreat:** drops a further 2.5 m (5v5) or 4.5 m (11v11).
  - **Maximum line by style:** Press 24 / 48 m, Possession 21 / 42 m, Counter 17 / 34 m (5v5 / 11v11).
    Press also shortens the block by 1 / 2 m and narrows it by 3%.
- **Attack (rest defence):** `line = min_line + 0.6 × (ball − pivot)`, always at least 8 m (5v5) or
  18 m (11v11) behind the ball. The front is the offside line. The block length is clamped to
  14–32 m (5v5) or 34–56 m (11v11).
- **Counter-press:** the attacking slots are compressed around the ball: line = ball − 9 m (5v5) or
  − 17 m (11v11), length 13 m / 24 m, width 0.78, shift 0.5.

### Behaviour on top of the anchors

- **One presser** contains goal-side at a gap of `1.55 m − 0.6 × (aggression − 500) mm + stamina decay
  + difficulty` (Hard −0.10 m, Easy +0.15 m), clamped to 1.05–2.1 m. In counter-press the gap is
  0.95 m and he sprints.
- **Cover** sits behind the presser, never more than 2 m higher than the back line.
- **Inside-box cutback marking** is kept from the old AI, but only the back-line and holding slots use
  it. The forwards no longer collapse into the box.
- **Anti-swarm spacing:** every defender other than the presser keeps at least 2.9 m from the carrier
  (3 m for the cover, the lane-closers and the second man), outside the 2.5 m engager radius (round 4;
  2.2 m before).
- **Recovery runs (round 4):** in every defensive phase, the counter-press included, a player more
  than 1 m ahead of the ball who is more than 30 ticks from his target (or any such player while the
  ball travels at our goal faster than 4 m/s) latches a recovery sprint until 3 m goal-side. Bursts
  are capped at 180 / 210 / 240 ticks by defensive work rate (back-liners uncapped), then the
  high-speed band (<= 7 m/s). He is class 2, never class 3. This replaces the old
  `1 m / 2.5 m + 2.5 m × recovery` deficit, which made faster players wait longer. Details in
  `docs/transition-and-defending.md`.
- **Shared line (round 4):** back-liners stand on one lag-tracked team line value (`line_prog`)
  with one team-wide judgement error. Marking may move a back-liner at most 2.5 m deep or 1.2 m high
  from it, unless he is tracking a runner in behind. The line tracks runs by style: Counter = Deep,
  Possession = Balanced, Press = High.
- **Attack (round 4, see [attacking-support-and-runs.md](attacking-support-and-runs.md)):** the
  5v5 vacated-cutback run and the support triangle are kept; the triangle pick is now committed
  (18-tick dwell, a challenger must be 2 m closer to the 9.5 m target). The five fixed pockets are
  replaced by scored support candidates (the slot, a PES ring at 14 m / 20 m around the carrier,
  the nearest opponent Voronoi vertex and the offside-line point, scored Control x Value x
  LaneOpen, re-scored every 20 ticks, switched only at 1.15x after a 60-tick commitment), with
  GRF spacing as the fallback for players without a scored candidate and a final-lane nudge after
  spacing, reservation and run push. Runs come from one arbiter per team (one penetrate runner,
  one support-from-behind runner, up to three box runners on a cross, one decoy): the penetrate
  runner gets an end-point beyond the offside line, holds at the onside limit (stutter, never
  reversing while onside) and is released by our pass toward him or by the CPU pass decision that
  selects him. `passer_ready` is a Schmitt trigger (on above -0.10, off below -0.30, 12-tick
  hold). In counter-attacks, forwards and wide players without a scored candidate or a run role
  still push to the onside line without waiting for the passer.
- **Decision hysteresis (round 4):** `reserve_destination` remembers who yielded last tick (1.5 m
  margin to swap), the teammate-yield and keeper-corridor side signs and the curved-run bend sign
  are sticky for the episode, and the committed target passes a 0.4 m re-target deadband and an
  alpha-beta filter (the role's alpha, beta 0.02). Every intent path that bypasses the shape
  (receive, chase, contain, non-open-play) now writes the target memory, so returning to shape
  continues the filter instead of snapping.
- **Judgement error:** the anchor's depth gets smooth deterministic noise (no RNG is consumed). Its
  amplitude is `noise × (100 − positioning) × 30 mm × err` (Easy 1.6, Normal 1.0, Hard 0.55), and
  60% of that for non-back-line players. It drifts over a 150-tick period. Since round 4 the back
  line shares one term, keyed by side and using the team's average positioning.

## 3. Style derivation

- **Per player** (`player_style`, each 0–1000):
  - discipline = positioning × 10
  - aggression = 500 + 12 × (tackling + strength − 2 × positioning)
  - recovery = 5 × (pace + acceleration)
  - work_rate = stamina × 10 × (0.3 + 0.7 × current stamina)
  - tackle_confidence = 8 × tackling + 2 × positioning
- **What each one drives:**
  - Discipline sets the target low-pass weight (`120 + discipline / 5` ‰), which controls how early a
    player reaches his anchor. It also sets the arrive dead-zone (`160 + 0.35 × (1000 − discipline)` mm),
    which controls how precisely he stands on it.
  - Aggression sets the presser gap and gives a press-candidate bonus of ±5 ticks. Centre-backs get
    +14 ticks when the ball is more than 8 m beyond their line, so they hold the line.
  - Work rate sets the pressing intensity, which decays with stamina. It feeds the counter-press
    window and the stamina gate on presser sprints.
  - Tackle confidence sets the CPU defender's tackle cadence (`base × (1400 − conf) / 540`) and reach
    (`+ (conf − 860) / 5` mm). Players with low confidence also lunge at a carrier who is half turned
    away from them (front/side threshold `480 + (600 − conf)` ‰).
- **Per team** (`team_style`): compare the averages of outfield stamina + pace + tackling (press) and
  passing + dribbling (possession) with the forwards' average pace − 6 (counter). The formation name
  adds +3: 4-3-3, 2-2, 4-2-4 and 3-4-3 add to press; 4-4-2, 1-1-2, 5-3-2, 3-5-2, 4-5-1 and 5-4-1 add to
  counter; 4-2-3-1, 1-2-1, 4-1-4-1 and 4-3-2-1 add to possession. Ties resolve Possession > Press >
  Counter. Both sample squads come out as Possession.
- **Difficulty** sets the reaction delay, the judgement-error scale, the presser hold and margin
  (table in §1), the presser gap, and the existing CPU defender cadence and hold distance. A human
  side's AI teammates use Normal.

## 4. Movement model (sim side)

All of it is deterministic integer or Q32.32 arithmetic.

1. **Arrive:** the AI target is committed: it is low-passed and sprint/jockey decisions have
   hysteresis (sprint off below 55% of the on-threshold, jockey 2.6 m on / 3.2 m off). The commanded
   speed is braking-consistent, `0.75 × √(2 × 9 m/s² × (d − deadzone))`, so a player stops on his
   target instead of overshooting it.
2. **Command low-pass** (`filter_velocity_command`): `cmd += k × (desired − cmd)` with
   `k = 0.25 + 0.002 × agility` (0.42 at agility 85, about a 2-tick lag). It applies to AI and
   CPU-driven outfielders off the ball. It never applies to human-controlled players or ball carriers.
3. **Realistic acceleration** (`realistic_velocity_step`): applies to AI and CPU-driven outfielders off
   the ball, except the intended receiver of an in-flight pass. It works in polar form, so it cannot
   overshoot:
   - acceleration ≤ 6 + 0.03 × acceleration m/s² (6–9)
   - braking ≤ 9 + 0.03 × acceleration m/s² (9–12)
   - heading turns toward the command by at most `a_lat / v`, with `a_lat = 8 + 0.06 × agility` m/s²
   - a request for more than 90° brakes while turning
   - from rest (< 0.5 m/s) the whole change uses the acceleration cap
4. **Turn-rate limit for the remaining outfielders** (`limit_turn_rate`: humans, carriers, receivers):
   lateral acceleration is capped at `11 + 0.08 × agility` m/s², × 1.6 for humans. It is active only
   above 1.2 m/s (AI) or 4.5 m/s (human), so a human's first-tick response is unchanged. A turn beyond
   the cap bleeds up to 30% of speed.
5. **Facing** (`limit_facing_turn`, AI and CPU-driven off the ball): up to 20°/tick at rest, falling
   to 9°/tick at 8.5 m/s, scaled by 0.8 + 0.004 × agility. While standing (< 0.5 m/s) the pivot is at
   most 3.5°/tick with a 4° deadband, and a 1° deadband applies while moving.
6. **Continuous loose-ball intercept** (`smooth_ball_intercept`): it interpolates the point where the
   chaser's needed ticks meet the ball's ticks, between 4-tick waypoints. The chase target therefore
   slides along the ball path instead of snapping between waypoints, which was the main cause of
   flicker in the CPU-controlled chaser. Within 1.5 m of a low ball, he attacks the ball directly.
7. **CPU off-ball defender** (`generate_cpu_input`): arrives at the contain point (scaled inside
   0.9 m), and a player already at sprint speed keeps sprinting until he is inside 70% of the
   close-down distance.

Human-controlled players keep the original acceleration model (explosive, ~26 m/s²) and the original
facing model, so their input latency is unchanged.

## 5. Metrics before and after

Source: `sim/psfc-sim/tests/team_shape.rs`, CPU vs CPU, 10 match-minutes per match (5-minute halves).
"Before" is commit 10e07a3 plus two read-only metric shims (`defensive_shape_anchor` = the old
`def_shift` anchor, `committed_presser` = the old `designated_ai_presser_idx`), run with the same
test file. The 6-seed figures come from `TEAM_SHAPE_SEEDS=6`.

| Metric (outfielders) | 5v5 Hard, before (6 seeds) | 5v5 Hard, after (6 seeds) | 11v11 Normal, before (6 seeds) | 11v11 Normal, after (6 seeds) |
|---|---|---|---|---|
| Anchor distance while defending, DEF / MID / FWD (m) | 4.97 / 5.50 / 5.77 | 4.08 / 7.36 / 7.15 | 6.00 / 6.10 / 7.25 | 7.31 / 6.54 / 13.85 |
| Same, settled (opponent ≥ 3 s on the ball) (m) | 1.77 / 2.37 / 3.10 | 2.50 / 1.73 / 2.92 | 1.33 / 2.71 / 1.59 | 2.53 / 3.57 / 4.01 |
| Back-line x-spread (m) | 5.37 | 3.81 | 2.79 | 3.26 |
| Line-height jitter (mm/tick) | 73.3 | 44.9 | 64.7 | 47.7 |
| Swarming (≥ 2 defenders within 1.6 m of the carrier) | 0.54% | 1.12% | 0.19% | 0.24% |
| Outfielders goal-side of the ball while defending | 56.8% | 70.1% | 67.7% | 73.5% |
| Presser changes per defending minute | 0.2 | 0.7 | 0.3 | 0.2 |
| Heading change 15–45° per tick | 0.92% | 0.15% | 0.73% | 0.04% |
| Acceleration reversals per player-minute | 168.5 | 58.2 | 115.7 | 27.8 |
| \|dv\|/dt p50 / p95 (m/s²) | 26.2 / 33.2 | 10.8 / 24.2 | 21.2 / 28.2 | 7.2 / 17.2 |
| Position second difference p50 / p95 (m/s²) | 25.2 / 32.2 | 10.8 / 21.8 | 18.2 / 25.2 | 3.8 / 14.2 |
| Velocity-change direction reversals above 1 m/s (per 1000) | 62.9 | 23.3 | 49.5 | 13.8 |
| Facing change p95 (°/tick) | 21.4 | 15.6 | 17.1 | 13.9 |
| Facing reversals (per 1000) | 60.4 | 24.8 | 39.3 | 13.0 |
| Facing jumps over 30° (per 1000) | 1.01 | 0.98 | 0.45 | 0.38 |
| Idle facing twitch over 4° | 3.07% | 0.06% | 1.08% | 0.12% |
| Goals / shots per match | 18.7 / 83.0 | 9.2 / 38.5 | 9.0 / 43.0 | 2.2 / 12.5 |
| Passes per match (completion) | 301 (84%) | 338 (85%) | 344 (80%) | 296 (83%) |

The 11v11 "after" column was run before the final striker-outlet tweak. The committed state's
3-seed run (the one the test asserts on) gives these 11v11 figures:

- anchor distance while defending: 6.30 / 5.91 / 12.71 m
- settled anchor distance: 2.07 / 3.46 / 3.37 m
- back-line spread 2.87 m, line jitter 39.1 mm/tick
- swarming 0.20%, 78.0% goal-side of the ball
- 28.0 acceleration reversals per player-minute, 1.67 goals and 12 shots per match

**Reading the numbers:**

- Movement smoothness improves by 2–4× on every jitter measure.
- The defensive line is steadier (−30–40% jitter), and more players defend from goal-side of the ball.
- Anchor distance while defending got worse for 5v5 midfielders and 11v11 forwards. This comes from
  the transition phases. The attacking shape is now really stretched (full-backs overlap, the striker
  is pinned on the last defender), and recovery runs now happen at realistic 6–9 m/s² instead of
  ~26 m/s².
- In the settled defence the distances are within about 1–2.5 m of the baseline. The new anchors
  also include deliberate judgement error and mark offsets.
- Goals fell sharply from the baseline's very high rates. Passing volume and completion did not fall,
  so play still flows.

## 6. Constants index

| Constant | Value |
|---|---|
| `DEF_LINE_RATE_PM` / `ATT_LINE_RATE_PM` | 550 / 600 ‰ of ball movement |
| Slot switch penalty / role-band penalty / minimum hold | 900 dm² / 60 000 dm² × diff² / 30 ticks |
| Presser hold / margin (Easy, Normal, Hard) | 40 / 10, 28 / 8, 20 / 6 ticks |
| Reaction low-pass (Easy, Normal, Hard) | 140, 230, 360 ‰ per tick |
| Judgement-error scale (Easy, Normal, Hard) | 1.6, 1.0, 0.55 |
| Accelerate / brake / lateral caps (AI, off the ball) | 6 + 0.03 × acc / 9 + 0.03 × acc / 8 + 0.06 × agility m/s² |
| Command filter k | 0.25 + 0.002 × agility |
| Facing rate | 20 → 9 °/tick × (0.8 + 0.004 × agility); 3.5 °/tick and 4° deadband while idle |
| Support re-score / commit / switch margin (round 4) | 20 ticks / 60 ticks / 1.15× |
| Support Range L (11v11 / 5v5), reservation exclusion, leash | 14 / 9 m (diagonals × √2), 10 / 6 m, 18 / 10 m |
| Run arbiter cadence / commit / expiry / called-run commit | 20 / 60 / 240 / 90 ticks |
| `passer_ready` Schmitt | on > −0.10, off < −0.30, 12-tick hold |
| Target deadband / alpha-beta beta | 0.4 m / 0.02 |
