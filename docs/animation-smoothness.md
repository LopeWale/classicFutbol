# Animation smoothness (renderer side)

Scope: `host/psfc_host/web/static/engine3d.js` (sub-tick interpolation, locomotion, GK get-up) and
the interpolation driver in `app.js`. The simulation and wasm are not touched. Base: `10e07a3`.

## Measurement method

Harness: `host/psfc_host/web/static/tests/anim/anim_harness.js` (Playwright + Chromium).

```
cd host && PSFC_DB_PATH=/tmp/x.db uv run uvicorn psfc_host.api.app:app --port 8765
node host/psfc_host/web/static/tests/anim/anim_harness.js --label after --out docs/anim \
     --seconds 30 --cadences 120,60,40,60j
```

- An init script replaces `performance.now` and `requestAnimationFrame` with a virtual clock and
  seeds `Math.random`, so every run sees the same frame cadence and the same sim ticks (the attract
  match has a fixed seed). Cadences: 120 fps, 60 fps, 40 fps (forces 1-2 sim steps per frame) and
  `60j` (60 fps with +-4 ms seeded frame-time jitter).
- 4 s warm-up, then 30 s recorded. GPU presentation is skipped while measuring (animation runs
  before present); it is re-enabled only for screenshot frames.
- Per frame and per visible actor: root position, root yaw, hips/spine/chest/knee bone quaternions,
  ankle world positions, foot-pin state. Values are converted to physical rates with the frame dt:
  - `pos`: root linear acceleration |dv|/dt (m/s^2), outfield players.
  - `yaw`: root yaw angular acceleration (rad/s^2).
  - `hips`, `spine`, `knee`: bone angular acceleration |w_t - w_(t-1)|/dt (rad/s^2), knee = max(L, R).
  - `bob`: hip-height acceleration (m/s^2).
  - `skate`: horizontal world speed of an ankle that is at sole height (within 1.2 cm) on two
    consecutive frames (m/s) - i.e. a "planted" foot that slides. `pinSkate`: same, restricted to
    frames where the stance pin is active. `movSkate`: restricted to actors moving > 1 m/s.
  - Pops: frames above a threshold (pos > 60 m/s^2, yaw > 400 rad/s^2, hips/spine/chest > 900
    rad/s^2, skate > 2.5 m/s). `slides`: non-teleport frames where the root moves > 13.5 m/s.
    `teleports`: root moves > 3 m in one frame (restarts; expected).
  - Frames with a teleport on either side are excluded from the acceleration statistics.
- Sim stream: the packed state is read every frame and kept when it changes; at 120 fps every 60 Hz
  tick is observed, which gives sim-only jitter statistics (independent of the renderer).
- Screenshots: when an outfield player is airborne (jump > 0.2 m) the harness moves the camera beside
  them for one frame and grabs the canvas (ascent, then contact/peak).

Results: `docs/anim/before.json`, `docs/anim/after.json` (p50/p95/p99/max per metric, pop counts,
sim-stream stats), `docs/anim/*_header_*.png`.

## Before / after (same harness, same ticks)

p95 unless noted. Lower is better.

| metric | 120 before | 120 after | 60 before | 60 after | 40 before | 40 after | 60j before | 60j after |
|---|---|---|---|---|---|---|---|---|
| root pos accel p95 (m/s^2) | 68.2 | **40.6** | 36.2 | 36.2 | 36.8 | 34.3 | 37.7 | 38.1 |
| root pos accel p99 | 75.1 | **62.3** | 77.0 | **44.6** | 58.2 | 55.6 | 60.5 | **49.6** |
| pos pops (> 60 m/s^2) | 4830 | **550** | 173 | 151 | 92 | 79 | 164 | 167 |
| yaw accel p95 (rad/s^2) | 248.9 | **192.7** | 244.1 | **190.3** | 223.5 | **180.8** | 235.3 | **187.9** |
| yaw accel p99 | 362.0 | 285.3 | 355.4 | 284.5 | 326.3 | 268.0 | 345.0 | 275.4 |
| yaw pops (> 400) | 221 | **111** | 96 | **54** | 48 | **27** | 94 | **48** |
| hips / spine accel p95 | 48.6 / 46.2 | 47.2 / 46.6 | 42.6 / 36.8 | 41.4 / 36.8 | 40.2 / 35.4 | 39.5 / 35.3 | 42.6 / 38.2 | 41.7 / 37.7 |
| knee accel p95 | 1118 | 1164 | 946 | 973 | 803 | 848 | 926 | 986 |
| hip bob accel p95 | 29.7 | 30.0 | 28.0 | 27.9 | 25.4 | 25.4 | 28.1 | 28.0 |
| planted-foot skate mean (m/s) | 2.1 | **1.4** | 2.0 | **1.2** | 1.9 | **1.0** | 2.0 | **1.2** |
| planted-foot skate p95 | 5.2 | 5.5 | 4.8 | **4.3** | 4.5 | **3.7** | 4.8 | **4.3** |
| skate while pinned, mean | 1.5 | **0.9** | 1.5 | **0.9** | 1.4 | **0.8** | 1.5 | **0.9** |
| skate pops (> 2.5 m/s) | 8339 | **3385** | 3646 | **1401** | 2207 | **727** | 3688 | **1412** |
| GK root pos accel p95 | 64.7 | 53.9 | 33.2 | 33.2 | 33.1 | 31.4 | 34.5 | 37.2 |
| GK hips / spine accel p95 | 41.2 / 39.9 | 39.5 / 34.6 | 34.8 / 32.9 | 32.9 / 29.9 | 33.4 / 30.9 | 31.5 / 27.4 | 34.3 / 32.7 | 33.5 / 29.3 |
| one-frame slides | 27 | **13** | 23 | **8** | 10 | 7 | 15 | 7 |
| teleports (expected restarts) | 22 | 22 | 22 | 22 | 22 | 22 | 22 | 22 |
| console errors | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

Notes:
- At 60 fps a pure lerp is already one sample per tick, so the Hermite change shows mainly at 120 fps
  (p95 68 -> 41, pops 4830 -> 550) and in the p99 tails at 60/60j (phase-change pairing fix).
- The remaining root acceleration p95 of ~36 m/s^2 is the sim itself (see below), not the renderer.
- Knee p95 is +3..6 %: the earlier heel rise adds knee flexion speed at toe-off. Accepted for the
  ~40 % skate reduction. Hips/spine are unchanged within noise.
- Remaining slides are restart repositioning: 1-2.5 m corrections glide over ~0.1 s, > 2.5 m snap.

Header/aerial poses (ascent, wind-up, snap, descent, landing from the previous round) are untouched;
`docs/anim/before_header_*` vs `after_header_*` show the same ascent and contact frames, pixel-near
identical.

## Changes and constants

`engine3d.js`
- `interpPlanarHermite` (new, near `lerpAngleRad`): cubic Hermite between prev/curr ticks with
  tangents `m0 = (u0 + s)/2`, `m1 = s + (s - u0)/2` where `s = dx/h`. The sim integrates
  semi-implicitly (`|dx/h - v1|` p50 0.03 m/s), so this reproduces constant acceleration exactly and
  keeps rendered velocity continuous across ticks; the analytic derivative is passed on as the
  actor velocity so stride and root motion agree. Falls back to lerp when the step is not explained by
  the reported velocity (`HERMITE_CONSISTENCY_MS = 0.6`) or on impacts (`HERMITE_MAX_DV_MS = 6`).
  Tangents never reverse the tick's travel direction (no creep-back after a hard stop).
- renderFrame: ball x/y and z use the same pair, alpha and scheme as players (one frame of reference).
- renderFrame: non-velocity displacement per tick (separation push, restart nudge) between
  `INTERP_CORRECTION_MIN_M = 0.03` and `INTERP_CORRECTION_MAX_M = 2.5` is absorbed once per tick
  (`tickAdvanced`) into a per-actor visual offset decaying with `INTERP_CORRECTION_TAU_S = 0.09`;
  above 2.5 m or a > 4 m step it snaps as a teleport.
- Yaw: SmoothDamp time `YAW_SMOOTH_AI_S = 0.074` for AI players (was 0.058); the user-controlled
  player keeps `YAW_SMOOTH_CTRL_S = 0.058`. Peak yaw acceleration on a facing step scales with
  1/smoothTime^2; sim facing moves up to 21 deg per tick (p95).
- `ACCEL_SMOOTH_S = 0.09` (was 0.075) for the lean/bank acceleration input; the sim's per-tick
  acceleration is a +-30 m/s^2 staircase.
- Locomotion weight rises instantly but releases through SmoothDamp `LOCO_RELEASE_S = 0.16`; the
  sim brakes at ~30 m/s^2, so raw speed crosses the 0.68 m/s blend band in 1-2 ticks and the run
  pose used to collapse into idle in one frame. The foot gait (`evaluateDirectionalFootTarget`,
  new `locoFloor` arg) uses the same released weight, and the stride phase keeps advancing
  (`1.7 Hz * locoWeight`) as settle steps while the weight releases.
- Foot pinning: stance envelope is flat-topped (`smoothstep(0,.16,u) * (1 - smoothstep(.84,1,u))`,
  was `sin(u*pi)`), pin no longer holds the foot height through toe-off, `applyWorldFootPin` returns
  its weight and the foot-goal damp is bypassed in proportion to it. Stance half-span grows from
  `0.11 + v*0.054` to `0.13 + v*0.062` (the gait under-strode: foot travel was 52-76 % of body travel
  per stance, which the pin had to fight). `FOOT_PIN_IK_BYPASS = 0` (0.5 and 0.8 were measured:
  knee p95 +45..70 %).
- Toe-off: heel rise starts at `TOE_OFF_START_U = 0.6` (was 0.72) and lifts `0.044 + 0.05*runBlend`
  (was 0.044), carried continuously into early swing. This removed most high-slip samples, which
  the harness located at stance end / early swing at > 5 m/s.
- Idle: per-actor breathing rate `0.72..1.12 Hz` and a slow per-actor lateral weight shift
  (hip x 1.4 cm, hip roll 0.035 rad) so idle lines do not bob in sync.
- GK dive / turf tumble get-up: `GETUP_RECOVER_S = 0.5` window after action 4/5 ends, during which
  hip height rises at most 2.4 m/s and hip/spine/chest smoothing is lengthened by up to 0.12 s
  (was a ~0.1 s snap from 0.36 m to 0.88 m hip height).

`app.js` (interpolation driver only)
- `simInterpPairBroken`: prev/curr pair resets only on a half change or a packed-layout change.
  It used to reset on every phase change, which made every actor jump one tick in one frame at each
  open-play/restart transition; per-actor teleport detection already lives in renderFrame.
- Backlog clamp: when the 6-step cap is hit, the remaining accumulator is dropped so alpha cannot sit
  at 1.0 for several frames.
- Verified: alpha stays in [0, 1); 0-step frames keep the pair and advance alpha; multi-step frames
  leave prev/curr as the last two consecutive ticks.

## Sim-side jitter (for the simulation owners; not changed here)

From the packed stream at 120 fps (every tick observed, 17 450 player-tick samples):

| quantity | value |
|---|---|
| position 2nd difference p50 / p95 / p99 | 25.2 / 36.0 / 38.3 m/s^2 |
| per-tick velocity change p50 / p95 | 0.43 / 0.56 m/s per tick (26 / 34 m/s^2) |
| velocity change reverses direction tick to tick (> 0.15 m/s both ticks) | 44 per 1000 samples |
| facing step p95 / p99 | 21.1 / 26.5 deg per tick |
| facing reverses direction tick to tick (> 3 deg both) | 19 per 1000 samples |
| facing step > 30 deg in one tick | 0.9 per 1000 samples |
| idle (< 0.3 m/s) facing change > 4 deg | 0.65 % of idle samples |
| `|dx/h - v|` p50 / p99 | 0.03 / 1.25 m/s (p99 = position corrections without velocity) |

Example (player index 3, ~15 s): 7.4 m/s to 0 in 14 ticks at a constant 0.54 m/s per tick
(32 m/s^2, ~3.3 g), then immediately re-accelerating at ~30 m/s^2 with facing jumping 10 deg in one
tick. Accelerations of this size are applied most of the time (median 25 m/s^2), i.e. the movement
model is close to bang-bang; real players brake at roughly 6-9 m/s^2. The renderer can only smooth
the pose response (lean, loco release, settle steps); the root path itself follows the sim. Facing
reversals (19/1000) are what remains in the yaw pop count after the damping change.

## Not done / follow-ups
- Cross-claim reach and catch-to-distribute transitions for the GK were not reworked; there is no
  measured pop on them in the 30 s attract window (GK bone pops 2 at 60 fps, before and after).
- Control-marker switch: the ring moves with the newly controlled player instantly (HUD is hidden
  in attract mode, so it is not covered by the harness); a short fade on switch would help.
- Knee angular acceleration remains the noisiest bone signal (~950 rad/s^2 p95 at 60 fps); a
  dedicated pass on the swing-phase heel fold curve would be the next step.
