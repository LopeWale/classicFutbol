# Simulation controls audit (Area B): power bar, input buffer, assistance, defending, sprint, kick physics

Scope: `sim/psfc-sim` only (deterministic Q32.32 fixed point, 60 Hz). Line numbers are for the commit
that adds this file. The jump/header code (auto-jump `p.vz` assignments, controlled aerial jump block,
aerial candidate ranking, header contest) was not modified.

## Status per specification bullet

| # | Spec bullet | Status | Where |
|---|-------------|--------|-------|
| 1a | Charge starts on ButtonDown (rising edge) for pass / through / lob-cross / shot | Already correct | `sim.rs:3292` (`kick_rising`, on-ball), `sim.rs:3365-3412` (pre-contact buffer), set pieces `sim.rs:655` (`just_pressed`) |
| 1b | Aim vector and power recorded every tick while held | Already correct (aim is re-sampled each tick, "receiver lock late") | `sim.rs:3200-3230` (open play), `sim.rs:396-430` (buffer), `sim.rs:595-640` (set piece) |
| 1c | Execute on ButtonUp, or automatically at max charge | Already correct; max now per action | `sim.rs:3218`, `sim.rs:614`, `sim.rs:421` |
| 1d | Per-action bar fill time (shot slower than pass) | **Fixed in this commit.** Was one 22-tick (367 ms) bar for everything. Now pass/through 30 ticks (500 ms), lob/cross 36 ticks (600 ms), shot 54 ticks (900 ms) | `input.rs:33-72` (`max_charge_ticks_for`, `charge_permille_for`, `charge_units_to_ticks`), `sim.rs:30` (`charge_norm_for`) |
| 1e | Overcharged shot balloons over the bar in a tuned, predictable way | **Fixed in this commit.** Was a step discontinuity at 84% fill (vz jumped about 1.1 to 2.5 m/s) plus a fixed vz, so the miss height depended on distance. Now vz is continuous at 84% and blends linearly to a balloon that crosses the goal line 0.9 m + composure term (`(100 - shooting) * 12 mm`) over the bar at 100%, at any distance, with gravity and the vertical Magnus dip taken into account. No RNG on the vertical axis | `sim.rs:4270-4320` |
| 1f | HUD `charge_permille` matches the sim in `Charging` and in a charging `BufferedAction` | **Fixed in this commit.** Both `charge_ratio_permille()` used a hard-coded `/22`. They now use `charge_permille_for(button, ticks)`, the same normalisation `execute_kick` uses. psfc-wasm already reads these two functions (`psfc-wasm/src/lib.rs:399-405`), so **no wasm layout change** | `state.rs:109-113`, `state.rs:316-320` |
| 1g | Set-piece charging consistent with open play | **Fixed in this commit.** Same per-action max and normalisation. CPU timeout defaults and AI targets in the old 22-unit scale are converted with `charge_units_to_ticks` | `sim.rs:614`, `sim.rs:687`, `sim.rs:2028`, `sim.rs:2185`, `sim.rs:1979`, `sim.rs:529`; `ai.rs:466-471`, `ai.rs:476-480`, `ai.rs:1307`, `ai.rs:1529` |
| 2a | 250 ms pre-contact buffer (`INPUT_BUFFER_TTL_TICKS = 15`) | Already correct. TTL is refreshed while the button is held and counts down from release | `sim.rs:396-445` |
| 2b | Buffered pass consumed at contact with recorded power and aim | Already correct; the one-touch minimum is now converted to per-action ticks | `sim.rs:6454-6482` |
| 2c | Releasing early locks the power | Already correct (`charging = false`, ticks frozen) | `sim.rs:421-429` |
| 2d | Buffer created ≤ 250 ms before contact fires with the button already up | Already correct; now covered by a test | test `buffered_pass_fires_on_contact_with_recorded_power_and_aim` |
| 2e | Buffer never fires a defensive action (contain vs pass on South) | Already correct. Buffer holds only `CHARGE_MASK` buttons; contain/jockey are suppressed while a buffer exists (`attacking_receive_or_buffer`); the new test asserts that only kick bits are present and the kicker ends in `Cooldown` | `sim.rs:3521-3530`, existing test `codex_review_dual_map_isolation_and_pass_buffer_no_contain` |
| 3a | RawVector vs TargetVector, Manual = 100% raw | Already correct (pass, through, lob/cross, shot) | pass `sim.rs:4543-4567`, shot `sim.rs:4136-4148` |
| 3b | Semi: blend direction, power manual | Already correct (pass 50/50, shot 55/45). Semi and Manual use the same `solver_dist = pref_dist(charge)`, so the speeds are identical | `sim.rs:4547-4552`, `sim.rs:4136-4148` |
| 3c | Assisted: snap to the best target, power scaled by distance, no blasted 3 m pass | **Fixed in this commit.** Distance was already auto, but the bar still pushed the arrival speed to 10 m/s and the speed floor was 8.2 m/s, so a 3 m full-bar pass left at about 11 m/s. Assisted ground passes now cap the arrival speed at `4.2 + 0.32·d` m/s with a 6.2 m/s floor (3 m full bar gives about 6.7 m/s; 20 m gives about 16.5 m/s) | `sim.rs:4716-4732` |
| 3d | Through-ball assist targets the runner's lead space | Already correct (72% user aim + 28% run lane, lead distance scales with charge, clamped short of the GK) | `sim.rs:4471-4489` |
| 3e | Shot Assisted snaps toward the far corner away from the GK | **Fixed in this commit.** A neutral stick with a centred GK (`|gk_y| ≤ 0.32 m`) used to aim dead centre, straight at the keeper. It now picks the far post across the shooter (opposite the shooter's lateral offset, or opposite the GK's tiny offset) | `sim.rs:4111-4123` |
| 4a | Legacy: holding Contain auto-moves the defender toward the carrier and auto-tackles in range | Already correct (press point 0.28 m goal-side, sprint when > 1.3 m, auto standing tackle at ≤ 1.18 m) | `sim.rs:3586-3601`, `sim.rs:3434-3438` |
| 4b | Tactical: Contain shadows at a set distance; the standing tackle is manual | Already correct (shadow point 1.55 m goal-side, forces jockey, no auto-tackle) | `sim.rs:3602-3629` |
| 4c | Jockey (LT) strafes facing the ball at reduced speed | Already correct; now tested | speed `sim.rs:3707-3712`, facing `sim.rs:3818-3828` |
| 4d | Teammate Contain (RB) summons a second AI presser; contain stamina drains and cools down | Already correct; stamina depletion, cooldown, hand-over and regeneration are now tested | `sim.rs:826-856`, regen `sim.rs:354-358` |
| 5a | 360° movement uses the full stick magnitude (walk, jog, run) | Already correct (radial deadzone `input.rs:376`; `desired_vel = move_dir.scale(top_speed)`) | `sim.rs:3776` |
| 5b | Analog sprint scales top speed smoothly; when off, any press = 1.0 | Already correct (`sprint_throttle_permille` → `eff_sprint_throttle` → `sprint_mult = 1 + bonus·throttle`) | `input.rs:330`, `sim.rs:3522`, `sim.rs:3717` |
| 6a | Ground pass solver arrives at the target speed | **Fixed in this commit.** `v0 = sqrt(v² + 2·4.05·d)` ignored the viscous term: short passes left 0.5 to 0.7 m/s too fast and long driven passes arrived 1.5 to 2.7 m/s too slow (35 m @ 15.2 m/s arrived at 12.5). It is replaced by an exact backward integration of the discrete rolling stepper (≤ 600 iterations, pure `Fp`) | `sim.rs:7376-7403` |
| 6b | Lob/cross apex and flight-time solving | **Fixed in this commit.** The analytic guess landed 5 to 8% short (it ignored the speed-scaled vertical Magnus and the spin decay). The analytic guess is kept as the seed, then refined by 4 fixed shooting iterations through the same `step_free_ball_kinematics` stepper | `sim.rs:7405-7447` |
| 6b' | A tap shot is a placed finish, not an FC-style low drill | **Fixed in this commit.** Any shot below 28% fill used to become a Low Driven drill (≥ 20.5 m/s, flat). FIFA 10–12 had no low-driven shot, so Low Driven is now only the explicit double-tap latch (`BufferedAction::low_driven`), and a tap goes through the normal driven/placed branch. The tap speed curve itself was not re-tuned (about 22 m/s for an 80-shooting player); see Deferred | `sim.rs:4086-4088` |
| 6c | Shot power→velocity curve, body-angle/sprint error, curl | Reviewed; no change needed. Speed is `16.6 + 0.068·shooting + 10.4·charge` m/s × body-angle power keep; lateral error = attribute base × (1 + turn + sprint + wide-angle + overcharge penalties); finesse sets `spin_side ±4.95/5.4`, and the stepper applies speed-proportional Magnus in `Fp` (`sim.rs:7138+`) | `sim.rs:4151-4187`, `sim.rs:4258` |
| — | Determinism | Kept: all new code is `Fp`/integer with bounded loops, and no state fields were added. `identical_inputs_and_seed_produce_bit_identical_hash_series` and `rollback_save_and_load_restores_exact_future_trajectory` pass | — |

### Deferred / not changed (with reason)
- **Placed-tap shot speed**: FIFA 10–12 tap shots read as about 12–16 m/s placed finishes. Here the driven curve starts at `16.6 + 0.068·shooting` m/s, so a tap is still firm. Lowering the bottom of the curve changes CPU finishing balance, so it is left for a tuning pass.
- **No power-bar speed setting**: EA shipped a power-bar speed slider, but all 14 bits of the `0x4000` ControlSettings word are used, so it was not added. It would need a second settings page, which is a replay-format change.
- **Alternative fill times**: the research agent estimated 36/42/48 ticks (pass/cross/shot). This commit uses 30/36/54 to follow the product brief's 0.9–1.0 s shot bar and keep passes snappy. Both are within the evidence, which has no first-party numbers. Retune via `input.rs` constants only.
- **Set-piece Charging end-to-end test**: set pieces share `max_charge_ticks_for` and `charge_norm_for`, but setting up a restart is heavy, so it is covered by code review, not a dedicated test.
- **Teammate-contain presser re-selection each tick**: `best_tc` is re-ranked every tick and could flicker between two near-equal pressers. This is not a spec violation, so it was left alone to avoid changing AI behaviour.
- **Buffered action ownership**: the buffer is per side and fires from whichever teammate makes contact (the controlled player switches to that receiver). This matches FIFA's behaviour, so it is unchanged.
- **`sim.rs:369` `needless_range_loop` clippy warning** and the test-only `inconsistent_digit_grouping` warnings were already there before this change and are untouched.

## Research findings and decisions

### Power-bar fill time per action
- No first-party source gives FIFA 10–12 bar fill times in seconds. EA's FIFA 12 free-kick guide treats the bar as a distance control: "if you're anywhere around the D then keep the power below the middle zone of the bar". Penalty guides state that holding Shoot too long sends the ball over the bar. Later titles (FIFA 17/18) document that an overcharged driven shot "balloon[s] over the bar".
  - EA, FIFA 12 free kicks guide: https://www.ea.com/en-gb/news/backpage-fifa-12-free-kicks-guide
  - FIFA 12 penalty tutorial (gameranx): https://gameranx.com/updates/id/2965/article/fifa-12-penalty-tutorial-how-to-take-a-penalty/
  - EA, FIFA 13 shooting techniques: https://ea.com/en-gb/news/fifa-13-tactical-tips-shooting-techniques
  - FIFA 23 power shot guides (bar-overcharge behaviour): https://fut.gg/news/fifa-23-guide-how-to-take-and-score-power-shots
  - EA, FIFA 19 timed finishing (second press over a bar): https://www.ea.com/news/fifa-19-gameplay-timed-finishing
- Decision: a 22-tick (367 ms) shot bar leaves too few distinguishable power steps for the "below the middle zone" placement those guides describe. Following the product brief (full shot bar about 0.9 to 1.0 s, pass bar faster), the fill times are: shot 900 ms, lob/cross 600 ms (lofted deliveries need a finer range), pass/through 500 ms (short passes are mostly taps, so quick release matters most). The values are constants in `input.rs`, so they can be retuned in one place.

### Deterministic kick physics (custom fixed-point engine)
- Rolling ball: the turf model is Coulomb resistance plus viscous damping, `dv/dt = -c - k v` (c = 1.88 m/s², k = -60·ln(0.997) ≈ 0.18 s⁻¹). The closed form `d = (v0 - v1)/k - (c/k²)·ln((c + k v0)/(c + k v1))` needs `ln`, which `math.rs` does not provide, and an iterative root solve. Integrating the discrete stepper backwards is exact for the integrator actually in use, and cheap. Offline check against the stepper: arrival error below 0.12 m/s for d = 3 to 35 m and v_arr = 5.2 to 15.2 m/s (asserted in `kick_solvers_match_the_ball_stepper`). The old formula was off by up to 2.7 m/s.
- Lobs/crosses: quadratic drag and Magnus have no closed-form inverse. The standard approach in deterministic sports sims is "shooting", meaning you integrate the same fixed-step integrator and correct the launch. Using the shared `step_free_ball_kinematics` keeps the solver exactly in step with `step_ball` and `ai::predict_ball_3d_at_ticks`. Reference on ball aerodynamics (drag coefficient around 0.2 to 0.3, Magnus side force behind curl): Goff & Carré, "Trajectory analysis of a soccer ball", Am. J. Phys. 77, 1020 (2009), https://doi.org/10.1119/1.3197414 ; Asai et al., "The curve kick of a football I: impact with the foot", Sports Engineering (2002), https://doi.org/10.1046/j.1460-2687.2002.00108.x
- Fixed-step integration for rollback and replay: everything stays Q32.32 with a fixed iteration count (no data-dependent float paths), following the deterministic-lockstep guidance in Gaffer On Games, "Deterministic Lockstep" (https://gafferongames.com/post/deterministic_lockstep/) and "Fix Your Timestep!" (https://gafferongames.com/post/fix_your_timestep/).

## Tests added (all in `sim/psfc-sim/src/sim.rs`, `mod tests`)
- `power_bar_fill_times_differ_per_action_and_hud_matches_sim`: pass, lob and shot auto-fire at 30, 36 and 54 ticks; the HUD permille equals the sim charge every tick; the buffered-action permille and the unit conversion are checked.
- `buffered_pass_fires_on_contact_with_recorded_power_and_aim`: button released before contact; fires inside 250 ms; recorded charge locked; 45° aim honoured; power equals a direct kick with the same charge; it fires a pass, never a defensive action.
- `pass_assistance_manual_semi_assisted_direction_and_power`: Manual stays about 30° off the mate; Assisted snaps onto the mate; Semi lands in between; Semi power equals Manual power; Assisted 3 m pass stays soft and 20 m is auto-powered; Manual power does not depend on distance.
- `shot_assistance_far_corner_blend_and_predictable_overcharge`: neutral Assisted shot goes to the corner away from the GK (both sides); Manual > Semi > Assisted for a wide aim; 83% bar stays under the bar, the red zone rises monotonically, 100% clears the bar by more than 0.4 m (Manual and Assisted, 11 m and 18 m).
- `kick_solvers_match_the_ball_stepper`: ground solver arrival speed and lob solver position at flight time, both checked against the real stepper.
- `analog_sprint_half_throttle_and_stick_magnitude_scale_speed`: 50% trigger gives a speed strictly between jog and full sprint; a light stick walks.
- `jockey_strafes_slower_facing_ball_and_teammate_contain_stamina_cools_down`.
- An existing assertion was updated: the "100% red-bar overcharge" shot in `tackle_dispossession_and_directional_pass_shot_mechanics_work` now holds for 60 ticks, not 27, because the shot bar is 54 ticks long.

## Client impact
- No wasm bit-layout change. `flags` bits 15..25 still carry `charge_permille` 0..1000; it is now per action.
- `cue.action_ticks` (bits 4..11) still carries raw charge ticks, which can now reach 54 for shots. The JS reads it only for skill and dive progress, so nothing breaks. A client that derived the bar from `ticks / 22` would need to use `charge_permille` instead.
- The local power-bar HUD prediction (if any) should use fill times of 500 / 600 / 900 ms for pass / lob / shot.
