# Avoidance, motion limits, push-out and contact (Area J)

This implements priorities two, three and four of `reports/FIFA PES player movement coordination.md`
("Recommended design for the Rust sim"): predictive local avoidance on the desired velocity, a
reshaped motion limiter, a position-only push-out that never writes velocity, and gated contact
rules. Code: `sim/psfc-sim/src/avoidance.rs` (new), the motion integration in `step_all_players`,
`resolve_player_separation` / `resolve_body_contacts` and two stumble gates in `resolve_tackles`
(`sim/psfc-sim/src/sim.rs`), and `PlayerState::overlap_ticks` (`sim/psfc-sim/src/state.rs`).
Tests: `sim/psfc-sim/tests/avoidance.rs`.

## Pipeline per tick

```
step_all_players
  build_frame(state)            snapshot pos/vel/class of all 22 players, once, before anyone moves
  for each player (id order):
    AI intent / stick -> desired_vel           (unchanged)
    avoidance on desired_vel                   AI: select_velocity   human: human_adjust
                                               carrier, GK, scripted actions: untouched
    filter_velocity_command (AI, unchanged)    low-pass of the command
    motion limits                              AI off-ball: avoidance::motion_step (two-regime a(v))
                                               others: unchanged base_accel + limit_turn_rate
    contact_constrain                          drop only the closing component into a touching body
    pos += vel * dt
resolve_tackles                                stumble outcomes gated by Balance vs Strength
step_ball / resolve_ball_contacts              (unchanged)
resolve_player_separation
  build_frame(state)                           classes after possession changes this tick
  resolve_body_contacts                        shoulder challenge / from-behind foul / stamina
  resolve_overlaps                             position only, 3 iterations, capped, class/mass weighted
```

Avoidance runs on the desired velocity before the limiter, so the limiter turns its output into
a curved run instead of a slide (the report's single most important ordering constraint, as in
DetourCrowd). Every stage reads the tick-start snapshot, so results do not depend on the order in
which players are stepped.

## Algorithms

**Priority classes** (`priority_class`, the one call site, over `crate::ai::move_priority` from the
exclusivity layer, see `docs/exclusivity-layer.md`): 0 = ball carrier, elected chaser, keeper while
committed to a loose ball (Collect / CrossClaim / Sweep / HumanRush); 1 = engager (presser, second
presser, cover) or the human-controlled player; 2 = tactical mover (recovery run, reserved run,
marking, far from his slot); 3 = idle, cruising or yielding. Three avoidance-domain rules sit on
top of the plan: a player in a scripted action (tackle, slide, dive, stumble, skill move, kick
wind-up) is class 0 (never steered, never displaced); the intended receiver of a pass in flight is
class 0; the keeper is never steered by avoidance, so he is clamped to class <= 1 even while not
committed (others give way to him). A neighbour of a lower priority (higher class number) is
ignored; against a higher class the agent takes all of the responsibility. One exception: an
engager (class <= 1) does not avoid the opposing ball carrier in the velocity stage. Closing him
down is the engager's job and the carrier's body is handled by the contact rules below; without the
exception the 1 s constant-velocity sweep reads a presser's run as a collision and throttles him to
a crawl 4 m out, because the sweep cannot know he will stop at his contain point (found when the
exclusivity and avoidance layers were merged: the FIFA 12 teammate-contain test closed 0.49 m in
0.4 s instead of 0.6 m).

**Velocity stage** (`select_velocity`): DetourCrowd `sampleVelocityAdaptive` + `processSample` in
Q32.32.
- Broadphase: farfield box of each agent swept over the horizon (EA US 12,268,964 farfield
  boxes), max 8 neighbours sorted by `(dist^2, id)`, plus the EA frame-offset instances at 4 / 8 /
  12 frames (used to decide "contact imminent" for human players).
- Fast path: if following `v_des` hits no comfort disc inside the horizon, `v_des` is returned
  unchanged (most ticks; keeps shape behaviour intact).
- Pattern: centre + 8 at radius 1 + 8 at radius 1/2 rotated half a step = 17 samples, aligned with
  the desired heading, 3 depth levels (51 evaluations), start `res = 0.4 v_des`, `cr = 0.6 v_max`.
- Cost: `w_des |vc - v_des|/v_max + w_cur |vc - v|/v_max + w_side * side + w_toi / (0.1 + t/H)`.
- Relative velocity: `vc - v_o` against a higher class, otherwise `(vc - v)/s + v - v_o` with share
  `s = 0.7` (RVO's 0.5 is `2vc - v - v_o`; 0.7 is a 1.4x over-avoid for limiter lag).
- Inside a comfort disc: closing or passing through counts as an immediate hit; separating costs by
  time to exit. (Detour's `-tmin/2` rewarded running straight through a teammate.)
- Side preference: keep the neighbour on the side it already lies on relative to the approach
  velocity, ties keep it on the left. Both agents make the complementary choice, it is stable as
  the pair diverges, and no per-pair state is needed. (Detour's area test with current velocities
  flipped sides every tick in a symmetric head-on.)
- Comfort distance: 1.0 m between teammates, 0.85 m between opponents (markers and challengers may
  reach tackling range), plus `2 ticks x |v_rel|` (max 0.3 m) for the limiter lag.
- Giving way (any higher-class neighbour): `w_des x 0.4`, `w_toi x 2`, the report's `w_priority`.

**Human players** (`human_adjust`): only when already moving at >= 1.5 m/s and body contact is
predicted within 12 frames; forward speed along the stick is kept at >= 90% of the command and the
lateral offset is <= 30% of it, so heading never flips. The first tick from rest is never touched.

**Contact constraint** (`contact_constrain`): when two bodies are within 0.86 m, a player may not
close faster than the other recedes plus the remaining gap per tick (EA US10692266
non-penetration constraint). Only the closing component is removed; a runner blocked head-on keeps
60% of its speed as a tangential slide (FC 26 "more slippery"). This is a motion-stage rule
applied to every class including the carrier and keeper (it is not avoidance steering); it skips
scripted lunges, slides and dives. It is what removed most residual overlaps.

**Motion limits** (`motion_step`, AI outfielders off the ball; human, carrier and receiver paths are
unchanged): acceleration `8.8 + 0.012 acc` m/s^2 up to 4.2 m/s, linear down to `4.0 + 0.02 acc`
m/s^2 at 7.0 m/s, flat above; braking `9 + 0.03 acc` m/s^2, but sprint (> 5 m/s) into jockey only
`6.5 + 0.025 acc`; heading rate `a_lat / v` with `a_lat = 8 + 0.06 agility` m/s^2, clamped to 120-720
deg/s.

**Push-out** (`resolve_overlaps`): body radius 0.405 m (centre distance 0.81 m), slop 0.01 m,
relaxation 0.6, 3 iterations in ascending pair order, displacement averaged over contributing
pairs, weights `yield(class) / mass` with yield 0/1/2/3 (class 0 never moves unless both bodies are
class 0, then 50/50), cap 3 cm per tick per player, rising by 10% of the depth beyond 10 cm up to
6 cm for deep restart overlaps. Never reads velocity except for the stuck-nudge direction and never
writes it. A player overlapping for more than 10 consecutive ticks (`overlap_ticks`, serde default,
hashed) gets a 15 mm lateral nudge along the contact tangent (FC 26 "moved to the side").

**Contact rules** (`resolve_body_contacts`, opponent pairs in body contact with the carrier):
- shoulder challenge only side-by-side (bearing within 50 deg of square) and with both players
  faster than 3 m/s: a defender stronger by >= 8 (carrier shielding adds 8-16) costs the carrier
  speed through the existing `push_pull_slow_ticks` (off-balance, keeps the ball);
- contact from behind (defender behind, >= 1.5 m/s faster, closing > 2.5 m/s, first tick of the
  contact) routes to `handle_foul(.., from_behind = true)` with `30 - tackling/5` percent;
- hard contact (closing > 4.8 m/s) costs both 0.2% stamina (kept from the old pass);
- stumbles in `resolve_tackles` only when the carrier's Balance (mean of agility and strength; the
  attribute set has no Balance) does not beat the tackler's Strength by 10, and for slides only
  from behind or at > 4 m/s relative speed.
- re-tackle lockout already existed (dispossessed carrier gets `Cooldown` 28-30 ticks and cannot
  control the ball; `kicker_lockout` after pokes), so nothing was added.

## Parameters

| Parameter | Value | Unit | Label |
|---|---|---|---|
| Body radius | 0.405 (pair 0.81) | m | project brief 0.35-0.45 |
| Comfort distance teammates / opponents | 1.0 / 0.85 | m | estimate (report 0.8-1.2 per pair) |
| Comfort lag margin | 2 ticks x v_rel, max 0.3 | m | estimate, tuned |
| Horizon | 1.0 | s | estimate (report) |
| Neighbours | 8, by (dist^2, id) | - | Detour default 6 (official), report 6-8 |
| Frame-offset instances | 4 / 8 / 12 | frames | EA patent examples (official) |
| Samples | 17 x 3 = 51 | - | estimate (report), Detour 75 |
| velBias / w_des / w_cur / w_side / w_toi | 0.4 / 2.0 / 0.75 / 0.75 / 2.5 | - | Detour defaults (official) |
| Equal-class share | 0.7 | - | estimate, tuned (RVO 0.5) |
| Yield multipliers w_des / w_toi | 0.4 / 2.0 | - | estimate, tuned (report "w_priority") |
| Human gate / forward floor / lateral cap | 1.5 m/s / 90% / 30% | - | estimate |
| Contact touch distance / slide kept | 0.86 m / 60% | -, - | estimate |
| a(v) low / high regime | 9.2-10.0 to 4.2 m/s; 4.7-6.0 from 7.0 m/s | m/s^2 | shape: community (eFootball); high-regime value tuned to keep team-shape bounds |
| Braking normal / sprint-to-jockey | 9-12 / 6.5-9 | m/s^2 | Area F / FIFA 23 "slower into Jockey" (official wording, value estimate) |
| Heading rate | 120-720 | deg/s | estimate (report) |
| Push-out slop / relax / iterations | 0.01 / 0.6 / 3 | m, -, - | estimate (report) |
| Push-out cap | 0.03 (deep: up to 0.06) | m/tick | estimate (report 0.03) |
| Stuck threshold / nudge | 10 ticks / 15 mm | -, m | estimate (FC 26 behaviour official) |
| Shoulder bearing gate / speed gate / Strength margin | 50 deg / 3 m/s / 8 | -, m/s, - | FC 27 side-by-side rule (official), values estimate |
| From-behind gate | +1.5 m/s, closing 2.5 m/s, 30 - tackling/5 % | - | estimate |

## Before / after

Same harness (`tests/avoidance.rs`, 3 seeds x 10 min, CPU vs CPU, 30 open-play minutes each):

| Metric | 5v5 Hard before | 5v5 Hard after | 11v11 Normal before | 11v11 Normal after |
|---|---|---|---|---|
| Collisions (pair enters < 0.8 m) per min | 53.8 | 2.3 | 50.2 | 1.2 |
| Overlap pair-ticks (< 0.8 m) per min | 176.6 | 6.1 | 150.0 | 3.3 |
| Overlaps lasting > 10 ticks | 96 | 4 | 77 | 0 |
| Stuck pairs (> 10 ticks, relative speed < 0.5 m/s) | 14 | 0 | 12 | 0 |
| `abs(dx/dt - v)` p50 / p99 | 0.000 / 0.145 m/s | 0.000 / 0.000 m/s | 0.000 / 0.000 m/s | 0.000 / 0.000 m/s |
| Player-ticks with correction > 0.3 m/s | n/a | 0.057% | n/a | 0.018% |
| Max overlap depth | 0.044 m | 0.72 m | 0.011 m | 0.68 m |

Notes: the old pass resolved 100% of the overlap every tick and wrote a slip and a 1.26x closing
impulse into velocity, so overlaps were frequent but shallow and the velocity jumped; the 1.25 m/s
p99 quoted in the report was measured on the renderer's packed stream on an older sim, this harness
measures the sim directly. The remaining > 10 tick overlaps (4 in 5v5) are corner-kick placements
that put two players on the same spot at the restart; they separate monotonically. The max
correction (about 14 m/s) is the existing 0.22 m tackle displacement, not the push-out.

Team shape (`tests/team_shape.rs`, 3 seeds, all bounds pass): 5v5 swarming 1.11% -> 0.81%,
settled anchors DEF/MID/FWD 5.37/4.40/3.26 ->
4.98/4.15/3.69 m; 11v11 swarming 1.18% -> 0.40%, settled 3.76/3.46/5.77 -> 3.72/3.77/4.55 m, line
jitter 57.6 -> 57.2 mm/tick. With 8 seeds (`TEAM_SHAPE_SEEDS=8`) the 11v11 bounds for line jitter
(< 58) and settled DEF (< 4 m) fail on the unchanged base too (58.3 / 4.03) and after (58.1 / 4.19):
those bounds sit inside seed noise, and any change to player motion reshuffles them.

## Left out

- Area I's `crate::ai::move_priority`: the fallback classifier stands in; swap it in at
  `avoidance::priority_class` (one call site).
- The report's full high-speed taper (about 1 m/s^2 above 6.25 m/s, 3.3-4.1 s to top speed): it
  made AI defenders lag their anchors (11v11 settled DEF 4.5 m over 8 seeds) because
  `ai::estimate_ticks_to_point` still assumes the fast model. Adopt it together with an
  `estimate_ticks_to_point` that integrates the same `a(v)` (ai.rs, Area I).
- A braking gate for AI ball carriers (sprint-to-dribble fast stop only for Agility/Dribbling
  >= 80) was tried and removed: it shifted possession dynamics enough to push 5v5 settled anchor
  distances well outside the team-shape bounds. Human carrier response is unchanged.
- Per-pair side-bit hysteresis (stateless side rule instead), foot-plant budget, per-player mass
  from height, keeper arbiter, a jockey max-speed mode, spam penalty for repeated challenges.
- The renderer still reads the push-out displacement as position; blending it as a small root
  offset over a few frames (Detour arrangement) is renderer work.
