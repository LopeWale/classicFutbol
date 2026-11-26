# FIFA 10–12 control semantics and kick/header physics for `psfc-sim`, with a gap analysis

Status: research note, v1. Gap analysis is written against commit `2d467f1` (branch `feat/keen-pascal-bvdins`). Three other work streams change the input layer, the charge/assist/defending code and the jump/header/renderer code at the same time, so line numbers in §7 may shift. Re-check them before acting.

Every number in this document carries one of these labels:

| Label | Meaning |
|---|---|
| **(official)** | EA manual, EA site, EA press release or EA developer interview |
| **(community)** | Measured or reported by players, guide sites or forums. Not confirmed by EA. |
| **(paper)** | Peer-reviewed sports-science or physics source |
| **(code)** | Value currently in this repository |
| **(estimate)** | My estimate or derivation. Tune it; do not treat it as fact. |

## Contents

1. [Executive summary](#1-executive-summary)
2. [FIFA 10–12 control semantics](#2-fifa-1012-control-semantics)
3. [Power bars, buffering and first-time play](#3-power-bars-buffering-and-first-time-play)
4. [Action-based input abstraction for a rollback sim](#4-action-based-input-abstraction-for-a-rollback-sim)
5. [Kick physics in Q32.32](#5-kick-physics-in-q3232)
6. [Aerial contests and heading](#6-aerial-contests-and-heading)
7. [Gap analysis (code as of `2d467f1`)](#7-gap-analysis-code-as-of-2d467f1)
8. [Things I could not verify](#8-things-i-could-not-verify)
9. [Sources](#9-sources)

---

## 1. Executive summary

1. **Two-Button in FIFA was a contextual scheme, not a remap.** A passes and B shoots. The game chooses ground pass, lob, cross or through ball, and chooses finesse or chip. Sprint is automatic. On defence A presses or tackles and B slides. X and Y do nothing **(official/community, §2.5)**. Our `two_button` preset is Classic with `switchPlayer` moved onto `south`. Every Contain press therefore also sends a `BTN_SWITCH` rising edge.
2. **The ground-pass solver does not match the ball stepper.** `solve_ground_pass_speed` assumes a constant 4.05 m/s² deceleration. `step_free_ball_kinematics` applies 1.88 m/s² plus a 0.18 s⁻¹ viscous term. Arrival speed comes out −22 % to +60 % off target (§5.2). An exact discrete inversion fits in about 30 lines of `Fp` code with no `ln`/`exp` and lands within 0.5 %.
3. **The lob/cross solver lands 4–10 % short** (§5.3). The fix is a 2–3 iteration shooting method over the existing deterministic stepper. It is bit-identical to flight by construction.
4. **A full bar is 22 ticks (367 ms) (code). That is too short to pick power reliably.** Each of the four "bars" lasts 5.5 ticks (92 ms). EA never published fill times, but it ships a "Power Bar" speed slider (FIFA 12+) **(official/community)**. I recommend 36–54 ticks per action type (estimate), exposed as a synced setting.
5. **Any shot charged below 28 % becomes a low-driven shot** (`sim.rs:4088`). That matches FC 24/25 behaviour, and double-tap matches FC 26. FIFA 10–12 had no low-driven shot: a tap was a soft, placed shot **(official/community, §3.4)**. Put this behind a "classic/modern" flag.
6. **The 14-bit settings word is full.** Lob, save and passing-power assistance, two-button mode, power-bar speed and pass-receiver lock cannot be synced yet. I propose a `0xC000` extended settings page, which needs `decode_substitution` to also check bit 14 (§4.4).
7. **Headers are too hot and too accurate.** Contest headers (`sim.rs:6209`) fly at 16.9–19.1 m/s with no aim error and always count as on target (`sim.rs:6226`). Aerial finishes through `execute_kick` fly at 19–22 m/s. A collision model built from heading biomechanics gives 10–16 m/s for a 13 m/s cross **(paper → estimate, §6.4)**.
8. **Jumps start too late, and ties always go to Home.** The jump decision looks 140 ms ahead (`sim.rs:3048`), but reaching apex takes ~0.34 s (20 ticks). Ties in the aerial contest break toward Home and the lower index (`sim.rs:5798`). Heading, jumping and height attributes do not exist (`snapshot.rs:17-28`).
9. **The renderer shows a chest trap.** Hands sit at ~1.0–1.15 m body height in front of the chest. There is no cock-back phase, and the header "snap" keys off proximity rather than time-to-contact (`engine3d.js:4718-4740, 5578-5606`). §6.5 gives a four-phase header pose and a contrasting chest-trap pose with joint targets taken from heading studies.
10. **Leave the aerodynamics alone.** Drag `k = 0.014 m⁻¹` matches Cd ≈ 0.26 **(paper: Cd 0.25–0.30)**, and the Magnus term is linear in speed, which is the correct low-spin approximation. Spend effort on the solvers, the error model and the aerial logic.

---

## 2. FIFA 10–12 control semantics

### 2.1 What changed per title (official)

| Title | Control-relevant headline features |
|---|---|
| FIFA 10 (2009) | First true **360° dribbling**, replacing the 8-way movement of FIFA 09. 360° defensive movement. Better slide-tackle targeting and press marking. [fifplay FIFA 10 features (EA release text); FIFA 10 PC blog; GameSpot impressions] |
| 2010 FIFA World Cup (Apr 2010) | Introduced **Two-Button** controls ("Dad Pad"). [Eurogamer; EA news] |
| FIFA 11 (2010) | **Pro Passing**: an "ideal" power line, under-hit passes are slow and over-hit ones are hard to trap, plus contextual error for angle, pressure, weak foot and attributes. Two-Button carried over. Separate manual-controls leaderboard. [GameSpot hands-on; GamingBolt interview with David Rutter; EA "Personality goes a long way"; Wikipedia] |
| FIFA 12 (2011) | **Tactical Defending** (Contain, Teammate Contain, manual Jockey, timed Standing Tackle, Push/Pull), **Precision Dribbling** (hold LB) and the Player Impact Engine. The **Legacy** defending option keeps the FIFA 11 press/auto-tackle. [EA press release; EA Backpage; FIFA 12 manual; GamingBolt interview with Aaron McHardy] |

### 2.2 Classic attacking map, FIFA 12 (Xbox 360 / PS3)

Sources: the FIFA 12 manual (EA PC PDF and its console text copy), the Meristation FIFA 12 controls guide and the FIFA 12 PS3 fan manual. FIFA 10 and 11 differ only where noted. The FIFA 10 modifiers RB = finesse and LB = chip are confirmed by FIFA 09/10 guides.

| Action | Xbox | PS | Tap / hold / multi-tap | Notes |
|---|---|---|---|---|
| Move / dribble | LS | L | analogue, 360° (FIFA 10+) | Release LS + LB = stop and face goal. Release LS + RT = stop ball / shield. |
| Sprint | RT (hold) | R2 | analogue when Analog Sprint is On | FIFA 11 community default: Analog Sprint **Off**. |
| Ground pass / header | A | X | hold = power (Pro Passing line, FIFA 11+) | Under Assisted, power also picks the receiver (more power selects a farther target) **(community)**. |
| Lob pass / cross / header | X | ■ | hold = power | Cross: ■ = cross, ■■ = low cross, ■■■ = ground cross. LB + ■ (×1/×2/×3) = early cross variants. LT + ■ = rabona/flair cross. |
| Through ball | Y | ▲ | hold = power | LB + Y = lobbed through ball. |
| Shoot / volley / header | B | ● | hold = power | Volley or header is chosen by context from ball height. |
| Finesse shot | RB + B | R1 + ● | modifier held at press | Inside-foot curl, less pace, more accuracy. |
| Chip shot | LB + B | L1 + ● | modifier | |
| Flair shot / pass | LT + B, LT/RB + A/Y | L2 + ● … | modifier | FIFA 12 manual: "a higher chance of missing their target". |
| Fake shot / fake pass | B then A (+dir), X then A | ● then X, ■ then X | sequence | Our sim's fake-shot cancel (`sim.rs:3135-3166`) follows this. |
| Give-and-go | LB + A | L1 + X | | |
| Cancel | LT + RT | L2 + R2 | chord | Our `super_cancel` = Sprint + (Pace Control or Jockey) (`sim.rs:379-385`) is equivalent. |
| Skilled dribbling / skill modifier | LT (hold) | L2 | | LT + RS = skill moves (FIFA 12 PS3 fan manual). |
| Precision dribbling (FIFA 12) | LB (hold) | L1 | hold | Also activates automatically near lines and in traffic (manual). |
| Knock-on / first touch | RS flick (RT + RS in later titles) | R | flick, double flick = far | FIFA 12 fan manual: "Right Analog (Flick) Knock On; (2x Flick) Knock On Far". |
| Dummy / let ball run | RB (hold) when receiving | R1 | hold | |
| Flick up | LT + RB | L2 + R1 | chord | |
| Switch to GK (FIFA 12) | Back | Select | toggle | |

**Low-driven shot.** No low or driven shot input appears in any FIFA 10–12 control list I found. LB + RB + B ("Low Shot / Downward Header") is documented for FIFA 21/22 **(official, EA accessibility pages)**. FC 25 made a quick tap contextually low **(community)**. FC 26 moved it to *power up, then press shoot again* **(official, FC 26 Pitch Notes)**. I did not verify when LB + RB + B first appeared.

### 2.3 Defending, FIFA 12: Tactical vs Legacy (Classic map)

| Action | Xbox | Tactical (FIFA 12 default) | Legacy (= FIFA 10/11 behaviour) |
|---|---|---|---|
| Contain / Press | A (hold) | "Contain positions a defender in front of an attacker, but allows you to decide when the best time to launch a tackle is" (manual). The defender stops about **a metre** from the carrier, an "invisible wall" **(official, EA Backpage demo guide)**. LS steers distance and direction while held. | "Press": runs at the carrier and **auto-tackles** when close (Backpage: FIFA 11 "press and auto-tackle"). |
| Teammate Contain / Secondary press | RB (hold) | An AI teammate marks the dribbler "but will only launch a tackle when the opportunity is right" (manual). In practice they rarely tackle themselves **(community, esports-news FIFA 12 tackling guide)**. | Secondary press, effectively a second auto-tackler (Backpage: "secondary press" made FIFA 11 pressing uncontrolled). |
| Standing tackle / Push-Pull | B (tap) | Manual timed tackle. Shoulder-to-shoulder or from behind it becomes push/pull: "knock them off balance with a shoulder challenge or a small tug" (manual). A mistimed tackle leaves you out of position. | Tackle exists, but Press does most of the work. |
| Sliding tackle | X | Same in both. | |
| Jockey | LT (hold) | Manual jockey: face the carrier and strafe with LS, no invisible wall. LT + RT = running jockey, "sprinting while jockeying means slower reaction times" **(official, Backpage)**. | Exists, rarely needed. |
| Switch player | LB | Nearest or next-best. Directional switch with the RS (FIFA 12 PS3 fan manual: "Right Analog: Directional Player Switch"). | Same. |
| Clearance | B in own half | Context. | Same. |
| GK rush | Y (hold) | Charge / punch. | Same. |

Legacy defending stayed a menu option in later titles: "Legacy Defending gives you the familiar defending controls of past FIFA titles" **(official, EA FIFA 20–22 accessibility settings pages)**.

### 2.4 Controller settings menu, FIFA 11/12 and later

FIFA 11 *My FIFA 11 → Settings → Controls → Xbox 360 Controller*, in menu order. Defaults are **(community)**, from two independent GameFAQs posts that agree, and match a FIFA 11 settings thread on Operation Sports.

| Setting | Options | FIFA 11 default | Later EA definition (official) |
|---|---|---|---|
| Auto Switching | Auto / Air Balls / Manual | Air Balls | Later titles: Manual / Only Loose Balls / Only Air Balls / Air & Loose / Auto. A community explainer for FIFA 23 puts the "air ball" threshold at **8 ft (≈2.4 m)** **(community)**. |
| Auto Switching Move Assistance | None / Low / High (Medium added later) | None | "keep the player you AUTO switch to moving in his current direction for a short period of time" |
| Passing Power Assistance | On / Off | Off | Auto-scales pass power. Forced off online **(community)**. |
| Pass Assistance | Assisted / Semi / Manual | Assisted | "whether pass direction and power will be assisted … into the receiver's path and avoid opponent players" |
| Through Pass Assistance | Assisted / Semi / Manual | Assisted | Same wording, for through balls. Manual = "the direction you point is the direction the pass goes, and the longer you hold the pass button, the stronger the pass will be". |
| Shot Assistance | Assisted / Semi / Manual | Assisted | "whether shot direction will be assisted to always aim towards the goal" |
| Cross Assistance | Assisted / Semi / Manual | Semi | "the amount of power that is used during cross direction passing" |
| Lob Pass Assistance | Assisted / Semi / Manual | Assisted | "consistency of lob passes and how strong the pass will be" |
| Save Assistance | Assisted / Semi / Manual | Semi | "whether balls relatively close to your body will be automatically saved" |
| Analog Sprint | On / Off | Off | "control how fast you sprint" |
| Defending (FIFA 12+) | Tactical / Legacy | Tactical | See §2.3 |
| Pass Receiver Lock (later titles) | Early / Late | Late | |

FIFA 12 also added **gameplay sliders**, including **"Power Bar – modifies how quickly/slowly the power bar fills up when you request any type of pass or shot"** (official description from the FIFA 22 page; the FIFA 12 slider is listed in a 2011 community slider guide), plus Pass/Shot Error and Pass/Shot Speed.

### 2.5 Two-Button controls: what they actually did

| Aspect | Behaviour | Source |
|---|---|---|
| Buttons | **A = pass**, **B = shoot**. Movement on LS. **Sprint automated.** | Eurogamer hands-on with line producer Simon Humber (official interview) |
| Contextual attack | A: "the computer will work out whether it needs to kick it in the air, or on the ground". It also crosses when you are in a crossing position. B: "the game will calculate the best shot … chip … finesse … or simply blast it". | Outcyders preview (community) |
| Power | "The longer you hold down a button, the harder a shot or pass will be" (interview). A review says power is largely automated in practice. | Eurogamer (official interview); Outcyders review (community) |
| Defence | A = **pass and press** ("the function changes according to whether you're in possession or not"), B = **shoot and slide tackle**. One preview describes A as tackle and B as slide. | GameSpot review; Outcyders preview |
| Other buttons | Not locked out: you can still hold the bumper for a chip or the trigger to sprint. In FIFA 12's Two-Button layout X and Y "have no use", LB switches player and RT sprints. | Outcyders; Meristation FIFA 12 guide |
| Known weaknesses | Prefers high crosses over cut-backs across goal. Weak shots. | GamingBolt, Outcyders reviews |

**Takeaway for us.** Two-Button is a sim-side inference mode, not a binding preset. The pass type (ground, lob, cross, through) and shot type (normal, finesse, chip) must be chosen by the sim from context: receiver distance and lane blockage, wing zone, run timing, and goalkeeper position. Defence needs one button that presses, which means Legacy semantics, plus one that slides. Switch must stay on its own button (LB).

### 2.6 Right stick (FIFA 10–12)

| Context | RS function | Source |
|---|---|---|
| In possession, LT not held | Flick = knock-on / first touch; double flick = knock-on far | FIFA 12 PS3 fan manual |
| In possession, LT held | Skill moves (star-rated) | Meristation; fan manual |
| Out of possession | Directional player switch | Fan manual |
| GK, own box | RS + direction = dive | Meristation |

---

## 3. Power bars, buffering and first-time play

### 3.1 What is publicly known

| Fact | Label | Source |
|---|---|---|
| Hold to power. Shot power also sets height, and overpowering sends it over the bar. | official | EA FIFA Mobile guide; FIFA 22 shooting guides |
| The bar is drawn in 4 segments ("bars"). Penalty advice is 2–2.5 bars, long shots 3–3.5 bars, free kicks "just over three bars". | community | Goal.com, RealSport101, Polygon, Dexerto, Overgear |
| FIFA 11 Pro Passing shows an "ideal power line" only *after* the kick. Under-hit = slow (interceptable), over-hit = sails past or is hard to trap. | official-ish (GameSpot hands-on, Rutter interview) | |
| Under Assisted passing, power chooses *which* receiver along the aim line ("1-2-3-4 method"). | community | fifaaddiction |
| The Power Bar slider changes fill speed (FIFA 12+). | official (description) / community (FIFA 12 slider list) | |
| Pre-pressing pass or through ball while a pass is incoming "preload[s] a first-touch pass" (2010 WC review). That is input buffering before contact. | community (GameSpot review) | |
| FC 26 Low Driven: "pressing the Shot button again after powering up your shot". The second press does not need to be timed; only the first press sets power. | official / community (Operation Sports) | |
| FIFA 19–21 Timed Finishing: second tap at the moment of contact; mistiming increases error. | official (EA settings page) | |

**Not public:** how many milliseconds a full bar takes in FIFA 10–12, or the exact power→speed curve. See §8 for how to measure them.

### 3.2 Recommended mapping for `psfc-sim` (estimate)

Keep `charge_norm ∈ (0, 1]` as the only interface between the bar and the solvers, so the solvers never see ticks.

```text
charge_norm = clamp(ticks_held, 1, FILL[action]) / FILL[action]

FILL[action] (ticks @60 Hz)       current (code)   recommended (estimate)
  ground pass / through           22               36   (0.60 s)
  lob / cross                     22               42   (0.70 s)
  shot                            22               48   (0.80 s)
  power-bar slider (synced)       —                FILL × (1.5 − slider/100), slider 0..100, default 50
```

Rationale (estimate): human key-hold timing error is tens of milliseconds. With 4 visual segments, each segment should be ≥ 9 ticks (150 ms) so that "2 bars" and "2.5 bars" are different, repeatable inputs. 22 ticks gives 5.5 ticks per segment. If the current fast feel is a deliberate arcade choice, keep 22 as the "fast" slider end.

Behaviour at full bar: release-to-execute is the spec. At `FILL`, either auto-release (current behaviour at `sim.rs:3217`) or keep holding at 1.0 until release. FIFA guides for later titles describe release as the trigger; I could not confirm FIFA 12. Auto-release at full is the safer choice for rollback, because it caps how long a charge lasts, which bounds misprediction.

### 3.3 Power → outcome

| Kick | Power use | Recommended outcome shape (estimate) |
|---|---|---|
| Ground pass, Manual / Semi | power → **distance** to stop/arrival point | `d_pref = d_min + charge_norm·(d_max − d_min)`, then solve v₀ for the arrival speed (§5.2) |
| Ground pass, Assisted | power → **receiver selection** along the aim cone, then v₀ solved to that receiver | Matches FIFA's "more power = farther man" **(community)**. Already done via `pref_dist` (`sim.rs:4385`). |
| Lob / cross | power → distance plus apex | flight time `T = T0 + c·d`, apex from T (§5.3) |
| Shot | power → speed **and** loft | `v = v_min + (v_max − v_min)·shape(charge)` with `shape` slightly convex; loft rises with charge; ≥ 0.85 = "ballooning" error |
| Overcharge | > 0.84 adds vertical error (code, `sim.rs:4167, 4283`) | Keep. This is the FIFA "row Z" behaviour **(community)**. |

### 3.4 Low-driven shot semantics: pick one per "era" flag

| Era flag | Trigger | Code today |
|---|---|---|
| FIFA 10–12 (classic) | None. A tap shot is a soft, placed side-foot shot (low speed, low error). | Not available |
| FC 24/25 (contextual) | Low charge ⇒ low driven | `charge_norm < 0.28` ⇒ low driven (`sim.rs:4088`) |
| FC 26 (modifier) | After powering up, press shoot again before contact | Double-tap latch on the buffer (`sim.rs:423-431`) |

Recommendation: add `shot_semantics: Classic | Modern` to the extended settings page (§4.4). In Classic mode, `charge_norm < 0.28` should give a **placed** shot at 12–16 m/s (estimate), not a 20–23 m/s drilled one. Today a tap from a 50-rated shooter leaves at 20.5 m/s (§5.4).

### 3.5 Buffering and first touch

| Item | Value | Label |
|---|---|---|
| Pre-contact buffer TTL | 15 ticks = 250 ms, refreshed every tick the button is held (`sim.rs:28, 414-421`) | code |
| Spec target | 150–250 ms | spec |
| Effective window | hold duration + 250 ms after release | code |
| Buffer eligibility | ball loose, own team receiving **or** "explicit first time", distance ≤ 8.5 m (`sim.rs:3371-3384`) | code |
| One-touch minimum power | 5 ticks for a pass, 6 for a shot (`sim.rs:6422`) = 23 %/27 % of the current bar | code |

Recommendations (estimate):
- Keep 250 ms. If FILL grows to 36–48 ticks, express the one-touch minimum as a fraction (≈ 0.25), not in ticks.
- First-time kicks should carry their own error multiplier of 1.3–1.8× depending on incoming speed and body angle. FIFA 11 Pro Passing names "first-time passes … more challenging" **(official-ish)**. The code applies none today.
- A buffered button that never meets the ball must expire silently, which it does (TTL). It must also never turn into a defensive action. That holds today because `side_has_buffer` gates the defensive branch (`sim.rs:3428`).

---

## 4. Action-based input abstraction for a rollback sim

### 4.1 Reference architecture (industry practice)

| System | Concept | Takeaway |
|---|---|---|
| Steam Input | **Action Sets**: only one active per device ("menu", "driving", "walking"). **Action Set Layers** overlay or override bindings. | Context = action set. Modifiers (LT skill layer, LB precision) = layers. |
| Unity Input System | **Action Maps** as input contexts (Gameplay / UI / Debug). Bindings are separate from actions. A central input manager enables and disables maps "to avoid input leaks". | Never let one press both confirm a menu and act in gameplay. |
| Godot InputMap | Named actions bound to many events. Runtime rebinding. Not persisted automatically. | Persist the profile yourself (we do: `localStorage` / `user://`). |

The standard pipeline is: **device → normalised physical state → (active context) → logical actions → game**. The step that matters for us is *who decides the context*.

### 4.2 This repo's approach: map both contexts, OR the bits, let the sim decide

`mapPhysicalToSimInput` (`app.js:1805-1886`) sets bits for both the attack map and the defence map every tick. The sim's `PossessionContext` (state machine) chooses which bits mean anything.

| Pros | Cons / pitfalls |
|---|---|
| Context comes from **authoritative, rollback-consistent sim state**, not a client guess. During a turnover tick both interpretations are in the frame, so the re-sim picks the right one. | **Same-button conflicts inside one context are invisible to the sim.** Two-Button binds `contain` and `switchPlayer` to `south` (`app.js:80-101`), so every Contain press also produces a `BTN_SWITCH` rising edge (`sim.rs:788`). |
| Rising edges come from `prev_buttons` stored *in sim state*, so they are rollback-safe. | **16 button bits are all used** (`input.rs:6-26`). There is no room for new logical actions such as player lock or directional run without a format change. |
| Replays are profile-independent at the sim boundary (bits are logical). | **Some context logic still lives on the client.** Sprint is chosen with `isAttacking` (`app.js:1835`). If a custom profile maps sprint differently per context, mispredicted possession gives the wrong sprint bit. |
| Loose-ball ambiguity is solved in one place (the buffer-vs-defence branch). | **Settings ticks overwrite the gameplay `reserved` word.** On those ticks `sprint_analog_raw()` returns 0, which reads as *full* sprint for one tick (`input.rs:279-301`). The auto-switch mode bits are absent for that tick as well. |
| | **Firmware bypasses all of this.** `firmware/main.c:145-153` emits `BTN_PASS|BTN_TACKLE` and `BTN_LOB|BTN_SLIDE` on one pin with no dual-map flag, no FINESSE/CONTAIN/PACE bits, and RB→`CALL_RUN`. |
| | **Double deadzone.** The client applies a 0.12 radial deadzone with rescaling (`app.js:1515-1519`), then the sim applies another 10/127 ≈ 0.079 (`input.rs:338`). Effective raw deadzone ≈ 0.19 (estimate). |

### 4.3 Recommendations

1. **Validate profiles for intra-context collisions** in `sanitizeControlProfile` (`app.js:146`) and in the Godot mirror. Within one map (attack or defence), one physical input may serve only one action, except for documented chords. Reject, or auto-swap with a prompt, during rebinding.
2. **Make Two-Button a sim mode, not a binding preset.** Add `two_button: bool` to the extended settings (§4.4). In that mode the sim:
   - maps `BTN_PASS` → {ground, lob, cross, through} by context (wing zone ⇒ cross; best receiver beyond a blocked lane ⇒ lob; runner past the last defender with lead ⇒ through);
   - maps `BTN_SHOOT` → {normal, finesse, chip} (finesse when the far post is open at an angle, chip when the GK is ≥ 4 m off his line);
   - in defence maps `BTN_PASS` → Legacy press with auto-tackle and `BTN_SHOOT` → slide;
   - forces auto sprint.
   The client binds only south, east and LB.
3. **Resolve sprint in the sim.** Always set `BTN_SPRINT` if *either* context's sprint key is held and carry its analogue depth, or require the same sprint key in both maps (validation).
4. **Hold the last gameplay `reserved` value across settings ticks** in sim state, so a settings tick does not change analogue sprint or auto-switch.
5. **Firmware:** emit physical button bits plus a profile ID, and let the host map them to logical actions. Alternatively, ship a firmware "Classic dual-map" table that sets `RESERVED_DUAL_MAP_FLAG` and the full logical bit set.
6. **Remove one deadzone layer** (keep the client's, set the sim's to just reject sign noise, ~3/127).
7. **Rollback note.** GGPO-style prediction repeats the last remote input. Held charge buttons predict well. Rising edges (taps, switch) are the mispredictions, so keep all "tap vs hold" classification in the sim and none in the client. The repo already does this.

### 4.4 Extended settings page (format proposal)

Today: `0x8000` = substitution (bit 15 only checked), `0x4000` = settings (bits 0–13 all used). Proposal:

```text
bit15 bit14
  1     0   substitution          (decode_substitution must check bit14 == 0)
  0     1   settings page 0       (unchanged)
  1     1   settings page 1 (new):
              bits 0-1  lob_assist        (Assisted/Semi/Manual)
              bits 2-3  save_assist
              bit  4    passing_power_assist
              bit  5    two_button
              bit  6    pass_receiver_lock (Early/Late)
              bit  7    shot_semantics (Classic/Modern)
              bits 8-11 power_bar_speed (0..15 → slider 0..100)
              bits 12-13 spare
```

Changing the substitution decode changes replay semantics, so bump the replay format version and add a golden-hash test.

### 4.5 Settings menu structure (recommended, mirrors FIFA 11/12)

```text
Controls
├─ Button Configuration: Classic | Alternate | Two Button | Custom
│    ├─ Attack map (8 actions)  ─ rebind, conflict check
│    ├─ Defence map (8 actions) ─ rebind, conflict check
│    └─ Button Help (glyphs from the active profile)
├─ Controller Settings (order as FIFA 11)
│    Auto Switching · Auto Switching Move Assistance · Passing Power Assistance
│    Pass · Through · Shot · Cross · Lob · Save Assistance · Analog Sprint
│    Defending: Tactical | Legacy · Pass Receiver Lock · Shot semantics
└─ Gameplay Sliders: Power Bar speed (synced), [offline only: pass/shot error]
```

---

## 5. Kick physics in Q32.32

### 5.1 Reference numbers

| Quantity | Value | Label / source |
|---|---|---|
| Ball mass | 0.41–0.45 kg (Laws); 0.43 kg typical; trajectory study ball 0.424 kg | official (IFAB) / paper (Goff 2012) |
| Ball radius | 0.11 m (68–70 cm circumference) | official |
| Air density | 1.225 kg/m³ | — |
| `ρA/(2m)` | 0.0541 m⁻¹ | derived |
| Cd, supercritical (free kicks, shots) | 0.25–0.30 (Bray & Kerwin 2003); 0.17 non-spinning at 15.6–19.3 m/s (Goff & Carré) | paper |
| Cd, subcritical (below ~12–15 m/s) | ≈ 0.5, dropping to 0.1–0.2 after the crisis; Re_crit 2.2–3.0 × 10⁵ | paper (Asai 2007; IPU review) |
| Drag constant `k = ρACd/(2m)` | 0.0081 (Cd 0.15), 0.0135 (0.25), 0.0162 (0.30), 0.0244 (0.45) m⁻¹ | derived |
| C_L (Magnus) | 0.23–0.29 (Bray & Kerwin); ≈ 0.35 at Sp 0.5, plateaus with Sp (Goff & Carré) | paper |
| Spin | free kick / corner ≈ 8–10 rev/s (7.93 rev/s corner example; 608 rpm at Sp 0.5, 14 m/s) | paper |
| Lateral Magnus accel., 25 m/s free kick | ≈ 6.1 m/s² (0.6 g), ~2 m deflection in < 1 s | paper (Stathopoulos 2009 model) |
| Max instep kick | 18–35 m/s; elite men 25.9–27.4 m/s means; pros 29.9 ± 2.9 | paper (Kellis & Katis 2007 review; Petrolo 2023 SR) |
| Side-foot kick | 23.4 ± 1.7 m/s vs instep 28.0 ± 2.1 | paper (Levanon & Dapena 1998 via review) |
| Weak foot (non-dominant) | men 23.6 vs 25.3 m/s (−7 %); women 18.9 vs 21.5 (−12 %) | paper (Barfield et al. 2002) |
| Rebound | FIFA turf: ball rebound 0.6–1.0 m from 2 m ⇒ e ≈ 0.55–0.71 | official (FIFA Quality Concept via Fleming 2011) |
| Rolling | FIFA turf: ball from a 1 m ramp (v₀ ≈ 3.46 m/s) must roll **4–10 m** (1-star) / 4–8 m (2-star). Low-speed deceleration fit `0.40 + 0.17·v` m/s² | official (FIFA via Penn State / Fleming) / paper-grade report (ISSS) |
| Header jump height | header jump 53.3 ± 5.3 cm, vertical jump 45.9 ± 8.1 cm (n = 35 soccer players) | paper (Matsuda 2015) |

Engine constants for comparison (code): drag 0.014 m⁻¹ (≈ Cd 0.26 ✓), ground bounce e 0.585/0.61 ✓, rolling `1.88 + 0.18·v` (5v5: 1.75) m/s², Magnus `spin_side·1.08·clamp(v/15.5, 0.45, 1.38)`.

**Is linear-in-v Magnus right?** At constant spin ω the spin parameter is S = rω/v and C_L ≈ c·S in the low-S regime, so `F ∝ C_L v² ∝ ω v`. The engine's linear term is the physically right first-order model. It deviates only at high S, where C_L plateaus. Keep it.

**Rolling is "heavy".** On the FIFA ramp test the engine ball stops in **2.6 m**, below the 4 m minimum (calculation below). That may be a deliberate gameplay choice. It is fine as long as the solvers invert the stepper the engine actually uses.

| v₀ (m/s) | Engine stop distance / time | `0.6 + 0.10v + 0.0135v²` (estimate) | `0.8 + 0.10v` (estimate) |
|---|---|---|---|
| 3.46 (FIFA ramp) | 2.6 m / 1.6 s | 6.7 m / 4.2 s | 5.8 m / 3.5 s |
| 10 | 16.5 m / 3.7 s | 29.3 m / 7.9 s | 35.0 m / 8.0 s |
| 20 | 48.9 m / 5.9 s | 58.4 m / 10.0 s | 99.5 m / 12.5 s |

If realism is wanted, the middle column (rolling + viscous + aero drag on the ground) passes the FIFA ramp test and keeps long balls from rolling 100 m (estimate).

### 5.2 Ground pass: exact discrete inversion (no ln/exp)

The stepper per tick (`sim.rs:7181-7190`) is `v ← (v − a·dt)·r`, with `a = 1.88`, `r = 0.997`, `dt = 1/60`. Error of the current constant-deceleration solver (`sim.rs:7329`, `a_eff = 4.05`):

| d (m) | target v_arr | v₀ solver | arrival actually | v₀ exact |
|---|---|---|---|---|
| 5 | 5.2 | 8.22 | 5.96 (+15 %) | 7.62 |
| 10 | 5.2 | 10.39 | 6.30 (+21 %) | 9.62 |
| 20 | 5.2 | 13.75 | 6.28 (+21 %) | 13.08 |
| 20 | 10.0 | 16.19 | 9.58 (−4 %) | 16.50 |
| 40 | 10.0 | 20.59 | **7.80 (−22 %)** | 22.11 |
| 40 | 5.2 | 18.74 | 4.55 (−12 %) | 19.05 |

(Simulated with the engine's discrete stepper.) Short passes arrive too hot and long passes arrive dead. Players feel this as short passes bouncing off feet and long passes being intercepted.

**Closed form of the recurrence.** `v_{n+1} = r·v_n − r·a·dt` has fixed point `v* = −r·a·dt/(1−r)` (= −10.413 m/s here), so

```text
v_n − v* = rⁿ (v₀ − v*)
d_N      = dt · Σ_{n=1..N} v_n = dt · [ r (v₀ − v_N)/(1 − r) + N·v* ]
```

Given distance `d` and target arrival `v_arr`, unknowns are v₀ and the tick count N:

```text
v₀(N)    = v_arr + (d/dt − N·v*)·(1 − r)/r
resid(N) = r^N·(v₀(N) − v*) − (v_arr − v*)       // monotone in N
N*       = smallest N with resid(N) ≤ 0           // binary search, N ∈ [1, 1024] → ≤ 10 steps
v₀       = v₀(N*)
```

- `r^N` comes from exponentiation by squaring in `Fp` (≤ 10 multiplies, deterministic).
- Precompute `(1−r)/r`, `v*` and `1/dt` as `const Fp`.
- Verified in a reference model: d = 10/20/40 m reach the target arrival within 0.5 % (tick quantisation).
- Subtract the launch offset first (`kicker_pos + dir·0.34 m`, `sim.rs:4695`): `d ← d − 0.34`.
- 5v5 uses `a = 1.75`. Make `a` and `r` shared constants used by both the stepper and the solver, so they cannot drift apart again.
- If the ground model gains an aero term (§5.1), the recurrence is no longer linear. Fall back to the shooting method (§5.3) with the 1-D stepper.

**Arrival speed targets (estimate).** Receivers in FIFA trap comfortably at about 4–9 m/s. Current targets are 5.2 + 4.8·charge (pass), 4.2 + 3.8·charge (through) and 15.2 + 4.4·charge (driven) (code, `sim.rs:4672-4686`). The driven target is fine as an "arrives hot" pass. Keep all three, since they become correct once the solver is exact.

### 5.3 Lob, cross and chip: closed form plus a shooting-method correction

Drag-free closed form (useful as the first guess):

```text
given horizontal distance D, flight time T, launch z0, target z1, effective gravity g_e
v_xy = D / T
v_z  = (z1 − z0)/T + g_e·T/2
apex = z0 + v_z² / (2 g_e)        (if v_z > 0),   t_apex = v_z / g_e
g_e  = g + c_M·spin_top·spd_scale  (backspin spin_top < 0 lowers g_e)
```

Horizontal motion with quadratic drag only (horizontal-dominant flight): `x(t) = ln(1 + k v₀ t)/k`, so `v₀ = (e^{kD} − 1)/(kT)`. The code instead uses `D/T·(1 + 0.007·D)`. That is close to the drag-only inversion for short balls, but it ignores that drag acts on the *3-D* speed and that the Magnus term scales with speed. Measured with the engine stepper:

| D (m) | T (s) | Lands at | Error |
|---|---|---|---|
| 15 | 1.30 | 14.5 m | −3 % |
| 25 | 1.70 | 23.4 m | −6 % |
| 35 | 1.95 | 32.3 m | −8 % |
| 45 | 1.95 | 40.5 m | −10 % |

**Fix (deterministic shooting method):**

```text
(vx, vz) ← closed_form(D, T)                    // guess
repeat 2–3 times:
    (D_hit, z_hit, T_hit) ← fly(vx, vz, spin)   // same fn as step_free_ball_kinematics,
                                                // stop when z ≤ z1 on descent or tick cap 180
    vx ← vx · D / D_hit                         // range scales ~linearly with vx
    vz ← vz + (z1 − z_hit)/T                     // height correction
```

`ai::predict_ball_3d_at_ticks` already calls the same stepper, so the result is bit-identical to real flight on every platform. Cost is ≤ 3 × 120 ticks of a ~40-op step, done once per kick, which is negligible.

Cross presets (estimate): whipped cross T = 0.9–1.2 s with apex 2.5–3.5 m; floated cross T = 1.4–1.8 s with apex 5–7 m; driven or low cross apex ≤ 1.6 m. A ground cross (FIFA ■■■) should be a ground pass through §5.2 aimed at the penalty spot or near post.

### 5.4 Shot speed vs power

Current (code, `sim.rs:4215-4290`): `v = 16.6 + 0.068·SHO + 10.4·charge` (+0.8 overcharge, +0.9 low-driven):

| SHO | tap (1/22) | 0.5 | 0.84 | full + over |
|---|---|---|---|---|
| 50 | 20.5 | 25.2 | 28.7 | 31.2 |
| 70 | 21.8 | 26.6 | 30.1 | 32.6 |
| 85 | 22.9 | 27.6 | 31.1 | 33.6 |
| 95 | 23.5 | 28.3 | 31.8 | 34.3 |

The top end matches research (25–35 m/s). The bottom end is too high for "placed" finishes. Recommended (estimate):

```text
v_max(SHO, foot) = (24 + 0.11·SHO) · foot_mult      // 29.5 m/s @50, 34.5 @95
foot_mult        = 1.0 strong, 0.93 weak             // paper: −7 % non-dominant
v(charge)        = v_place + (v_max − v_place)·charge^1.3,   v_place = 12 m/s
finesse          = 0.85·v,   chip: separate solver (§5.3) with apex over GK reach (2.4–2.8 m)
```

The `charge^1.3` curve gives finer control at low power, where FIFA players do most of their placement **(community: "2–2.5 bars")**.

### 5.5 Error model (direction, height, speed)

FIFA 11 named the factors **(official)**: attribute, weak foot, pressure, angle (the "180 pass") and power. FIFA 22+ adds body angle and sprint. The code has attribute, body angle, sprint, wide angle and overcharge for shots (`sim.rs:4146-4186`), and attribute plus blind-turn for passes (`sim.rs:4597-4612`). Proposed unified form (estimate):

```text
σ_dir = σ_base(attr) · (1 + Σ wᵢ·fᵢ)          // radians; draw with triangular dist (sum of 2 uniforms)
σ_base(attr)  = (100 − attr)·0.0007 + 0.004     // ≈ 0.6° at 99, 3.0° at 40
f_body   = (1 − cos θ_body)/2   w = 1.2         // 0 facing, 1 fully reversed
f_sprint = max(0, v_kicker − 6.6)/2.4  w = 0.4
f_weak   = weak_foot ? (5 − wf_stars)/4 : 0   w = 0.8
f_press  = clamp((1.6 − d_nearest_opp)/1.2, 0, 1)  w = 0.6
f_first  = first_time ? clamp(v_ball_in/15, 0, 1) : 0   w = 0.5
f_height = clamp((z_ball − 0.3)/0.9, 0, 1)     w = 0.5   // volleys/half-volleys
f_power  = max(0, charge − 0.84)/0.16           w = 1.5 (vertical only)
σ_speed  = 0.03·v·(1 + f_press + f_first)      // ±3 % speed jitter
```

- Draw RNG values in a fixed order every kick (dir, then height, then speed), even when a weight is 0. That keeps RNG consumption constant across code paths and makes rollback divergence easier to debug.
- Apply assist *before* error, as the code already does: Assisted moves the mean, never the variance.
- `preferred_foot` and `weak_foot` (1–5) must be added to `PlayerAttributes` (`snapshot.rs:17-28`). FIFA has had weak-foot ratings for years. I did not verify the first title.

### 5.6 Fixed-point notes

- Precompute every constant as `const Fp` from milli-units, as the code already does. Never derive constants at runtime from division chains that differ between call sites.
- No `ln` or `exp` is needed anywhere above. Ground passes use the recurrence plus `r^N`. Lobs use the shooting method.
- Keep `spd_scale`-style clamps. They bound the Q32.32 products: the largest intermediate in the Magnus term is ~5.5 × 1.08 × 1.38 ≈ 8.2, far from overflow.
- Sorting candidates by `(metric, side, idx)` is deterministic but **biased** (§6.2). Determinism and fairness are separate requirements.

---

## 6. Aerial contests and heading

### 6.1 Who should jump

Recommended (estimate):

1. When an aerial ball (z > 1.4 m at the predicted interception point) is launched, predict its path once with `predict_ball_3d_at_ticks` and find the **landing/interception window** where `z_ball ∈ [head_reach_standing, head_reach_jump]`.
2. Per team, choose **at most two** contestants: the controlled player (human), if within reach, and the AI player with the earliest arrival at the window. Today every eligible loose-ball player within about 1.35 m jumps on its own (`sim.rs:3026-3058`).
3. `reach_jump = forehead_height + jump_apex(attr)`, with `forehead_height ≈ stature − 0.10 m` (estimate) and `jump_apex = 0.45–0.65 m` (paper: 0.53 ± 0.05 m in header jumps). Add `height_cm` and `jumping` (and `heading`) attributes. Today the code uses a fixed 1.65 m head (`sim.rs:5779`) and `strength` for jump velocity (`sim.rs:3052`).

### 6.2 Timing window

```text
t_apex = v_z0 / g ≈ 0.31–0.37 s (19–22 ticks)        // v_z0 = 3.0–3.6 m/s (estimate from paper heights)
take-off tick = T_contact − t_rise(h_needed),  t_rise from z(t) = v_z0 t − g t²/2 = h_needed (choose the ascending root)
human: jump on press of pass/shoot/lob (buffered) ⇒ use the same take-off computation, but only if pressed
       within [T_contact − 30, T_contact − 8] ticks; earlier ⇒ jump now (mistimed), later ⇒ standing header/chest
```

- The current lift estimate looks 140 ms ahead (`sim.rs:3048`) while ascent takes ~340 ms, so players jump late and are often still rising at contact.
- **Contest resolution (estimate):**
  `score = −dist3d(forehead_at_contact, ball) + 0.004·(jumping+heading+strength)/3 + 0.08·is_first_to_window + rng_tri(±0.05)`.
  Replace the `(metric, side.index(), idx)` sort (`sim.rs:5798`), which always gives ties to Home and the lower index. If you keep a deterministic tie-break, alternate it with `tick & 1` or seed it from the RNG.
- Physical outcome for the loser: on a near-tie (< 0.15 m), both players get a 10–20 tick stagger.

### 6.3 Header power and accuracy vs attributes

| Fact | Value | Label |
|---|---|---|
| Standing header adds to ball speed | +3.00 ± 0.93 m/s (skilled) vs +1.52 ± 0.43 (less skilled) | paper (Isas 2001) |
| Trunk ROM in a powerful header | 27° (skilled) vs 17° | paper |
| Neck angular velocity at contact | 396 ± 153 °/s; peak within 8 ms of contact | paper |
| Trunk angular velocity at contact | 282 ± 28 °/s | paper |
| Effective striking mass, jumping header | 13.8 % of body mass (≈ 10 kg) | paper (Kristensen 2002) |
| Diving header ball-speed enhancement | > 9 m/s; head speed 7.3 m/s | paper (MDPI 2024) |
| Low-speed purposeful header | 4.3 m/s in → 5.5 m/s out | paper (PMC3899908) |

Outgoing speed from a 1-D impact (estimate):

```text
v_out = [ (e·M − m)·u + (1 + e)·M·V_head ] / (M + m)      // u = incoming speed toward head, sign so out is +
        m = 0.43, M = 0.10–0.14·body_mass (≈ 8–11 kg), e ≈ 0.5–0.6, V_head = 3–6 m/s (attr-scaled)
example u = 13 m/s, M = 10, e = 0.55, V_head = 4 → 12.3 m/s ;  V_head = 6 → 15.2 m/s
```

The current values are 16.9–19.1 m/s for the contest header (`sim.rs:6209`, `14.5 + 0.048·SHO`) and 19–22 m/s for the aerial finish in `execute_kick` (`sim.rs:4228`). Both sit 20–60 % above this envelope. Accuracy: today the contest header aims at a perfect corner with no error and always counts as on target. Use §5.5 with `σ_base(heading)`, `f_height = 0`, and add `f_contest` (w = 0.8) when an opponent is within 0.6 m.

### 6.4 Defensive header and flick-on

- Clear: aim away from goal with +25–40° loft, at 10–14 m/s (code: 13.4 m/s, loft from vz = 3.15).
- Flick-on: when the ball comes from behind relative to facing, keep 60–80 % of incoming speed with a deflection ≤ 30° (estimate). Not implemented.

### 6.5 Animation reference: header vs chest trap

The current procedural pose (`engine3d.js:4830-4850` spine, `5578-5606` arms, `4900-4918` head look-at) has these problems:
- hands at y ≈ 1.00–1.15 m body space, in front of the torso (z −0.16…+0.17);
- spine pitch only +0.02…+0.17 rad;
- head pitch clamped to −0.18 rad while airborne, so the player looks up at the ball with the chest facing it;
- the snap triggers on proximity (`ballDistHoriz < 1.35`), not on time-to-contact.

That silhouette (chest square to the ball, arms near the chest, chin up) **is** a chest-trap silhouette.

**Header phases.** Descriptive sources: NSCAA via Matsuda 2015 ("moves his head and shoulder backward, bends his knee, then moves his head, shoulder and legs forward as he hits the ball"), Isas 2001 and Kristensen 2002. Joint targets are my estimates fitted to those numbers.

| Phase (time to contact) | Trunk (pitch, +fwd) | Neck/head | Arms | Legs |
|---|---|---|---|---|
| 1. Take-off (−0.34 … −0.25 s) | −5…0° | neutral, eyes on ball | both arms **swing up** forward to shoulder height (arm swing raises jump height **(paper)**) | take-off leg extends; lead knee drives up |
| 2. Cock (−0.25 … −0.06 s) | **extend back 15–27°** (arch) | **neck extended 15–25°**, chin slightly up but eyes on ball | arms **up and out**, elbows at or above shoulder, forearms angled forward: balance and space | knees flex 60–90° behind (body "bow") |
| 3. Strike (−0.06 … +0.03 s) | **snap forward** to +10–20° at ~280°/s | **neck flexes** at ~400°/s, peaking within ~8 ms of contact; contact on the **forehead/hairline**, chin tucked | arms **driven down and back** hard (counter-rotation that powers the trunk whip) | legs whip forward with the trunk |
| 4. Follow-through and land (+0.03 … +0.4 s) | +20–30° | head follows the ball direction | arms come forward and down for landing | knees absorb |

**Chest trap, for contrast (estimate):** trunk **leans back** 15–30° and **stays** there. The sternum is pushed out and up toward the ball. Arms are **spread wide and back/down** (hands below shoulder height, behind the frontal plane), never raised in front. The head is pulled back with chin *down* to watch the ball onto the chest. There is no neck snap at contact, and the trunk "gives" backward (cushion) instead of whipping forward.

**What distinguishes them on screen:** (a) the sign of the trunk motion at contact (header: forward whip; trap: backward give), (b) arm height and direction (header: up, then driven down; trap: wide and low), (c) head (header: forehead leads; trap: chin down, chest leads).

**Data the renderer needs from the sim (estimate):** a per-player `aerial_phase` with `ticks_to_contact` (or the contact tick) and `intent ∈ {Header, Clear, Flick, ChestControl}`. With these the renderer can schedule phases 1–4 in time instead of guessing from distance. This is snapshot/presentation data only: it does not need to enter `state_hash` if it is derived on the host from deterministic state.

---

## 7. Gap analysis (code as of `2d467f1`)

Ranking: **Impact** = how much a player feels it (H/M/L). **Risk** = implementation and determinism risk (H/M/L). **#** = suggested order (high impact, low risk first).

| # | Spec item | What exists (file:line) | Missing / wrong | Recommended change | Impact | Risk |
|---|---|---|---|---|---|---|
| 1 | Kick physics: ground pass solver | `solve_ground_pass_speed` `sim.rs:7329` (constant `a_eff` 3.85/4.05) | Does not match `step_free_ball_kinematics` `sim.rs:7181-7190`. Arrival −22 % … +21 % (§5.2). Launch offset ignored. | Exact discrete inversion (§5.2). Share `a`, `r` constants with the stepper. Golden test: solved pass arrives at ±3 % of target for d = 5…40 m. | H | L |
| 2 | Kick physics: lob/cross solver | `solve_ballistic_lob_launch` `sim.rs:7338` | Lands 3–10 % short. Ignores 3-D drag coupling and `spd_scale`. | 2–3 iteration shooting method over the stepper (§5.3). | H | L |
| 3 | Correct heading: outcome | contest header `sim.rs:6187-6260` | 16.9–19.1 m/s, no aim error, **always `shots_on_target += 1`** (`sim.rs:6226`, stat bug), uses `shooting`. Aerial finish 19–22 m/s (`sim.rs:4228`). | Impact model (§6.3). Error per §5.5. Compute `on_target` like `execute_kick` does (`sim.rs:4297-4305`). Add `heading` attribute. | H | L |
| 4 | Two-Button preset | `app.js:80-101`; Godot `controller_interface.gd:72-95` | Classic + `switchPlayer` on `south`, which collides with Contain (switch edge at `sim.rs:788`). No contextual inference. Sprint not automatic. | Sim mode `two_button` (§4.3 #2) synced on settings page 1 (§4.4). Client binds south / east / LB only. | H | M |
| 5 | Correct jumping: timing and selection | auto-jump `sim.rs:3026-3058`; buffered jump `sim.rs:3414-3428` | 140 ms look-ahead vs ~340 ms ascent. All nearby players jump. Lift from `strength`. Fixed 1.65 m head. | Contact-window prediction plus take-off scheduling (§6.1-6.2). Add `height_cm`, `jumping`. At most two contestants per team. | H | M |
| 6 | Aerial contest fairness | sort `sim.rs:5798`; 3-D metric `sim.rs:5776-5783` | Ties → Home and lower idx. No attributes or RNG in contest. | Score function (§6.2). Symmetric tie-break. | M | L |
| 7 | Header animation | `engine3d.js:4718-4740, 4830-4850, 4900-4918, 5578-5606` | Chest-trap silhouette. Proximity-triggered snap. No cock phase. | Four-phase pose keyed on `ticks_to_contact` (§6.5). Sim exposes `aerial_phase` in the snapshot. | H | M (renderer only) |
| 8 | Charge-on-press / release-to-execute | `Charging` `sim.rs:3126-3230`, `MAX_CHARGE_TICKS = 22` `sim.rs:26`, rising edge `sim.rs:3250-3330` | Behaviour OK. Fill 367 ms is very fast (§3.2). One FILL for every action. | Per-action FILL (36/42/48 ticks, estimate) plus a synced Power Bar slider. Express one-touch minimums as fractions. | M | L |
| 9 | Buffering 150–250 ms | `INPUT_BUFFER_TTL_TICKS = 15` `sim.rs:28`; buffer step `sim.rs:362-438`; consume `sim.rs:6409-6435` | Within spec. First-time kicks carry no extra error. | Add `f_first` (§5.5). Keep the TTL. | M | L |
| 10 | Low-driven semantics | `sim.rs:4088` (<28 % ⇒ low driven); double-tap `sim.rs:423-431` | FIFA 10–12 had no low-driven shot. Tap shots are ≥ 20.5 m/s drills. | `shot_semantics` Classic/Modern flag (§3.4). Classic tap = 12–16 m/s placed shot. | M | L |
| 11 | Manual/Semi/Assisted per action | `AssistLevel` `input.rs:66-90`; shot blend `sim.rs:4130-4144` (Semi = 55/45); pass cone `sim.rs:4391-4400` | Lob, save and passing-power assistance missing. Error model lacks weak foot, pressure, first-time, ball height. | Settings page 1 (§4.4). Unified error model (§5.5). Add `preferred_foot` / `weak_foot`. | M | M |
| 12 | Settings sync format | `ControlSettings::encode_reserved` `input.rs:166-178` (14 bits full); `decode_substitution` checks bit 15 only `input.rs:231-240` | No room for new settings. A settings tick zeroes analogue sprint and auto-switch for that tick (`input.rs:279-301`). | `0xC000` page 1 plus bit-14 check on substitution (§4.4). Latch the last gameplay `reserved` in sim state. Bump the replay version. | M | M (format change) |
| 13 | Action-based remappable input + JSON profile | presets `app.js:29-125`; sanitize `app.js:146-178`; persistence `app.js:659-664` (`psfc_control_profile_v2`); glyphs `app.js:202`, used `app.js:3128, 4051`; Godot mirror | No conflict validation. `custom` not in the preset cycle (`app.js:3233`). Sprint context chosen client-side (`app.js:1835`). | Conflict check on rebind and load. Add `custom` to the cycle. Sprint resolution in the sim (§4.3 #3). Snapshot test that the JS and GDScript presets are identical. | M | L |
| 14 | Legacy vs Tactical defending | `DefendingMode` `input.rs:92-110`; legacy auto-tackle ≤ 1.18 m `sim.rs:3433-3437`; contain/jockey flags `sim.rs:3534-3535`; teammate contain stamina `sim.rs:820-840` | Structure matches FIFA 12. Tactical contain stand-off should be ≈ 1 m **(official, "about a metre")**. Teammate Contain should rarely tackle **(official/community)**. Being changed by the defending work stream. | Verify the stand-off ≈ 0.9–1.1 m and the teammate-contain tackle rate in tests. In Two-Button defence, use Legacy press. | M | L |
| 15 | 360° dribbling and analogue sprint | radial deadzone `input.rs:333-344`; client deadzone `app.js:1515-1519, 1636`; `sprint_throttle_permille` `input.rs:286-301`; client sprint depth `app.js:1834-1841` | Works. Double deadzone (≈ 0.19 effective, estimate). i8 sticks give ~0.45° resolution at full deflection, which is fine. | Reduce the sim deadzone to ~3/127. Keep analogue sprint on by default (FIFA 11 default was Off **(community)**; either is defensible). | L | L |
| 16 | Air-ball auto-switch threshold | `z > 0.85 m` `sim.rs:2591, 2706` | FIFA explainer says ~8 ft (2.4 m) **(community)**. 0.85 m switches on bouncing balls. | Raise to 1.8–2.4 m (estimate), or switch on the *predicted* apex of lofted passes (`pass_in_flight` with vz > 0). | L | L |
| 17 | Settings menu structure | pause/HUD toggles `app.js:3220-3260` | Flat toggles. No Lob/Save/Power-assist entries. No slider. | Menu per §4.5. | L | L |
| 18 | Firmware | `firmware/main.c:145-153` | Emits logical bits directly. Shared pins (PASS\|TACKLE, LOB\|SLIDE). No dual-map flag. RB→CALL_RUN. | Emit physical bits plus a profile, or set `RESERVED_DUAL_MAP_FLAG` with full Classic dual-map bits. | L (few users) | L |
| 19 | Rolling resistance realism | stepper `sim.rs:7181-7190` | Ball stops in 2.6 m on the FIFA ramp test (needs 4–10 m). No aero drag on the ground. | Optional: `0.6 + 0.10v + 0.0135v²` (estimate) after #1/#2 land. Re-tune pass arrival targets. This is a gameplay-feel change. | L–M | M (retune) |

What is already right and should be kept: dual-map OR'ing with sim-side context; rising edges taken from sim-stored `prev_buttons`; the fake-shot cancel and pass transitions; overcharge ballooning; Semi blend for shots; the drag constant; the linear Magnus model; ground restitution; Legacy/Tactical as a per-side synced setting; Auto Switching Move Assistance durations (FIFA publishes no durations, so 18/42 ticks are reasonable estimates).

---

## 8. Things I could not verify

| Item | Status | How to get it |
|---|---|---|
| FIFA 10–12 power-bar fill time (ms) for pass, shot and cross | Not published. No reliable community measurement found. | Record 60 fps footage of FIFA 12 with the default Power Bar slider (50). Count frames from bar appearance to full. Repeat for pass, shot and cross. |
| Whether FIFA 12 shots fire on release or at animation contact when the button is held | Later guides say "release to execute". FIFA 12 not confirmed. | Same footage: hold through full bar and observe. |
| First FIFA with LB + RB + shoot (low shot) | FIFA 21/22 confirmed (official). Earlier titles unverified. | EA manuals for FIFA 13–16. |
| Exact FIFA 11 Two-Button defence (A = press vs A = tackle) | Two reviews disagree slightly. | 2010 WC / FIFA 11 in-game Button Help screen. |
| FIFA 12 console manual control tables | The EA PC PDF only covers tips. Full tables are in-game ("Button Help"). Tables here come from Meristation and a fan manual (community). | In-game screen captures. |
| Spin decay rate of a football in flight | No source found. Engine τ ≈ 1.85 s (code). | Trajectory papers (Goff & Carré 2010 EJP) if needed. It matters little over 1–2 s flights. |
| Air-ball auto-switch threshold in FIFA 11/12 | Only a FIFA 23-era community figure (8 ft). | — |
| Header and chest-trap joint angles | Phases from papers. Angles are my estimates. | Mocap reference or annotated broadcast frames. |

Network note: `www.ea.com` and `eaassets-a.akamaihd.net` do not resolve from this session, so EA pages were read through a third-party fetch service (Parallel). Quotes from those pages are their extracted text.

---

## 9. Sources

EA / official
- FIFA 12 manual (PC, Xbox 360 controller glyphs): https://eaassets-a.akamaihd.net/eahelp/manuals/fifa-12-manuals_PC.pdf
- FIFA 12 in-game manual text copy (console button names): https://s3.amazonaws.com/szmanuals/1e0df7584b6767f5f9c85c6486d0498f
- EA Backpage, Tactical Defending Help (2011): https://www.ea.com/en-gb/news/the-backpage-tactical-defending-help
- EA Backpage, FIFA 12 Demo Guide (contain stops "about a metre"): https://www.ea.com/en-gb/news/the-backpage-fifa-12-demo-guide
- EA press release, FIFA 12 Player Impact Engine, Precision Dribbling, Tactical Defending: https://news.ea.com/press-releases/press-releases-details/2011/EA-Sports-Revolutionizes-FIFA-Soccer-12-with-New-Player-Impact-Engine/default.aspx
- EA, FIFA 11 "Personality goes a long way" (Pro Passing): https://www.ea.com/en-gb/news/fifa-11-personality-goes-a-long-way
- EA, 2010 FIFA World Cup out now (Two-Button): https://www.ea.com/en-gb/news/ea-sports-2010-fifa-world-cup-out-now
- EA, 2010 FIFA World Cup demo / soundtrack (Two-Button): https://www.ea.com/en-gb/news/ea-sports-2010-world-cup-south-africa-soundtrack
- EA accessibility, FIFA 20 PC controller settings: https://www.ea.com/able/resources/fifa/fifa-20/pc/controller-settings
- EA accessibility, FIFA 21 PC controller settings: https://www.ea.com/able/resources/fifa/fifa-21/pc/controller-settings
- EA accessibility, FIFA 22 PS4/PS5/PC customise controls: https://www.ea.com/able/resources/fifa/fifa-22/ps4/customise-controls-settings , https://www.ea.com/able/resources/fifa/fifa-22/pc/customise-controls-settings
- EA accessibility, FIFA 22 basic controls (Low Shot L1 + R1 + ●): https://www.ea.com/able/resources/fifa/fifa-22/ps4/basic-controls
- EA accessibility, FIFA 22 game settings (Power Bar slider): https://www.ea.com/able/resources/fifa/fifa-22/ps4/game-settings
- EA FC 26 Pitch Notes, Gameplay Deep Dive (double-tap Low Driven): https://www.ea.com/en/games/ea-sports-fc/fc-26/news/pitch-notes-fc26-gameplay-deep-dive
- EA Help, How to shoot in EA SPORTS FC: https://help.ea.com/en/articles/ea-sports-fc/how-to-shoot/
- EA FIFA Mobile button controls (hold = power/height): https://www.ea.com/news/gameplay-guide-button-controls

Interviews / press / reviews
- Eurogamer, 2010 FIFA World Cup hands-on (Simon Humber, "Dad Pad"): https://www.eurogamer.net/2010-fifa-world-cup-final-hands-on
- GameSpot, 2010 FIFA World Cup review (two-button pass/press, shoot/slide; preloaded first touch): https://www.gamespot.com/reviews/2010-fifa-world-cup-south-africa-review/1900-6260563
- Outcyders, Two Button preview and review: https://www.outcyders.net/preview/simpler-two-button-controls-for-2010-fifa-world-cup , https://www.outcyders.net/review/2010-fifa-world-cup-south-africa-review
- GamingBolt, 2010 WC review: https://gamingbolt.com/2010-fifa-world-cup-south-africa-review
- GamingBolt, FIFA 11 interview (David Rutter, Pro Passing): https://gamingbolt.com/fifa-2011-an-exclusive-interview-with-the-team-at-ea
- GamingBolt, FIFA 12 interview (Aaron McHardy, Tactical Defending): https://gamingbolt.com/fifa-12-mega-interview-with-lead-gameplay-director-aaron-mchardy
- GameSpot, FIFA 11 hands-on (ideal power line): https://www.gamespot.com/articles/fifa-soccer-11-hands-on/1100-6265051
- GameSpot, FIFA 10 impressions (360° dribbling): https://www.gamespot.com/articles/fifa-10-impressions/1100-6211111
- FIFPlay, FIFA 10 features (EA release text): https://www.fifplay.com/fifa-10-features
- FIFA Infinity, FIFA 10 PC gameplay blog: https://www.fifa-infinity.com/fifa-10/fifa-10-gameplay-innovations-blog
- Wikipedia, FIFA 11: https://en.wikipedia.org/wiki/FIFA_11

Community guides / forums
- Meristation FIFA 12 controls (Classic / Alternate / Two-Button): https://as.com/meristation/2011/09/26/guia_pagina/1317016802_035457.html
- FIFA 12 PS3 fan manual (Scribd): https://www.scribd.com/doc/76340450/FIFA12-Fan-Manual-PS3-v1-0
- Esports News UK, FIFA 12 tackling guide: https://esports-news.co.uk/2011/09/29/fifa-12-tackling-guide
- GameFAQs, FIFA 11 default controller settings: https://gamefaqs.gamespot.com/boards/988949-fifa-soccer-11/56611611
- Operation Sports, FIFA 11 settings: https://forums.operationsports.com/forums/forum/soccer/ea-sports-fc-and-fifa/ea-sports-fifa-last-gen/388074-fifa-11-settings-discussion
- PlayStationTrophies, FIFA 11 settings options: https://www.playstationtrophies.org/forum/topic/95163-wireless-controller-settings-help
- SoccerGaming, FIFA 12 slider guide (Power Bar slider): https://soccergaming.com/forums/threads/a-guide-to-fifa-12-gameplay-sliders.168555
- FIFAUTeam, FIFA 23 controller settings (air-ball 8 ft): https://fifauteam.com/fifa-23-controller-settings
- fifaaddiction, power picks the receiver: https://fifaaddiction.com/fifa-tutorials/fifa-attacking-tutorial/how-to-master-passing-in-fifa
- FIFPlay, FIFA 21/22 shooting controls: https://www.fifplay.com/fifa-21-shooting-and-finishing-tips , https://www.fifplay.com/fifa-22-shooting-and-finishing-tips
- Goal.com, FIFA 21 controls and penalties: https://www.goal.com/en/news/fifa-21-controls-attacking-defending-goalkeeping-playstation-xbox-pc/1dgx1kuk9i6sn1bpumurztbvhg , https://www.goal.com/en-us/news/fifa-21-penalties-how-to-score-spot-kicks-in-the-new-game/1nb1teq1csnvl1rv204lnaayiz
- RealSport101, FIFA 20 penalties; FC 26 low driven: https://realsport101.com/article/fifa-20-how-to-score-a-penalty-tutorial-fut-career-ps4-xbox , https://realsport101.com/article/how-to-perform-low-driven-shots-in-fc-26
- Operation Sports, FC 26 low driven: https://www.operationsports.com/how-to-perform-low-driven-shots-in-ea-fc-26
- Polygon, FIFA 18 guide (free kick ~3 bars): https://www.polygon.com/2017/10/3/16381416/beginners-guide-passing-defending-shooting-set-pieces-skill-challenges

Physics / sports science
- Asai et al., Fundamental aerodynamics of the soccer ball, Sports Eng 2007: https://link.springer.com/article/10.1007/bf02844207
- Goff, Investigations into soccer aerodynamics via trajectory analysis (Cd 0.17, C_L vs Sp): https://eprints.whiterose.ac.uk/id/eprint/98035/1/1-s2.0-S1877705812016414-main.pdf
- Goff, Power and spin in the beautiful game, Physics Today 2010: https://physicstoday.aip.org/quick-study/power-and-spin-in-the-beautiful-game
- Bray & Kerwin, Modelling the flight of a soccer ball in a direct free kick, J Sports Sci 2003: https://www.tandfonline.com/doi/abs/10.1080/0264041031000070994
- Passmore et al., Aerodynamic performance of FIFA-approved footballs, 2012: https://journals.sagepub.com/doi/10.1177/1754337111415768 (PDF: https://scispace.com/pdf/the-aerodynamic-performance-of-a-range-of-fifa-approved-2o7x35zd55.pdf)
- Creating drag and lift curves from soccer trajectories, Eur J Phys 2017: https://iopscience.iop.org/article/10.1088/1361-6404/aa6fcd
- Stathopoulos, Simulating key aspects of soccer (Cd 0.15, 0.6 g lateral, 7.93 rev/s): http://ejst.uniwa.gr/issues/issue_13_2009/stathopoulos_13.pdf
- Soccer ball aerodynamics review (Asai drag crisis figure): https://ipu-japan.ac.jp/ipu/wp-content/uploads/2025/12/KBB_SCHOLAR_kyobo_16583367.pdf
- Kellis & Katis, Biomechanical characteristics of instep kick, JSSM 2007: https://pmc.ncbi.nlm.nih.gov/articles/PMC3786235/
- Systematic review, instep kick in elite adults (ball speed 25.9–27.4 m/s): https://drantoniopetrolo.ca/wp-content/uploads/2024/01/KINEMATIC-DETERMINANTS-OF-THE-INSTEP-SOCCER-KICK-IN-ELITE-ADULT-SOCCER-PLAYERS_A-SYSTEMATIC-REVIEW.pdf
- Nunome et al., Side-foot vs instep kinetics: https://pubmed.ncbi.nlm.nih.gov/12471312
- Barfield et al., Kinematic instep kicking differences (dominant vs non-dominant): https://pmc.ncbi.nlm.nih.gov/articles/PMC3967432/
- Markovic et al., Tests of maximum kicking performance: https://pubmed.ncbi.nlm.nih.gov/16823350
- Cerrah et al., Kick type vs ball velocity: https://www.mjssm.me/?sekcija=article&artid=150
- Isas 2001, Determination of the impact mass in soccer heading: https://ojs.ub.uni-konstanz.de/cpa/article/download/3901/3618
- Kristensen 2002, Segmental characteristics in powerful soccer heading: https://ojs.ub.uni-konstanz.de/cpa/article/download/726/648
- Matsuda 2015, Jump ability of soccer players (header jump 53 cm): https://www.jstage.jst.go.jp/article/jssfenfs/12/0/12_1/_pdf/-char/en
- Fílter et al. 2021, Effect of ball inclusion on jump performance: https://pubmed.ncbi.nlm.nih.gov/35475740
- Diving header ball-speed enhancement, Appl Sci 2024: https://www.mdpi.com/2076-3417/14/2/946
- Neck strength imbalance in heading (4.3 → 5.5 m/s): https://pmc.ncbi.nlm.nih.gov/articles/PMC3899908
- Rusdiana et al. 2020, Jump heading mechanics: https://efsupit.ro/index.php/jpes/article/download/2506/2506/6804
- Fleming 2011, Artificial turf systems (FIFA ball roll 4–10 m, rebound 0.6–1.0 m): https://mccmeetingspublic.blob.core.usgovcloudapi.net/rollwdtx-meet-b277da13af6547f5a4e9d3f76b8b1712/ITEM-Attachment-001-bcef15f7f5924e27909d4abc305cc760.pdf
- Penn State SSRC ball roll report (FIFA requirements): https://plantscience.psu.edu/research/centers/ssrc/documents/soccer-ball-roll-report.pdf
- ISSS, Ball roll behaviour (deceleration 0.40 + 0.17v, v = 3.46 m/s from 1 m ramp): https://www.isss-sportsurfacescience.org/downloads/documents/ZPKPAJJUWY_Ball_Roll_BehaviorKS.pdf
- ISSS, Gabrielsen ball roll presentation: https://www.isss-sportsurfacescience.org/downloads/documents/A7ZD60AH55_Gabrielsen_Ball_Roll.pdf.pdf

Input architecture / determinism
- Steamworks, Steam Input concepts (Action Sets): https://partner.steamgames.com/doc/features/steam_controller/concepts?l=english
- Steamworks, Action Set Layers: https://partner.steamgames.com/doc/features/steam_controller/action_set_layers
- Unity blog, Input System action maps as contexts: https://unity.com/blog/input-system-event-driven-architecture-for-scalable-controls
- Godot docs, InputMap / InputEvent: https://docs.godotengine.org/en/stable/tutorials/inputs/inputevent.html , https://docs.godotengine.org/en/latest/classes/class_inputmap.html
- Gaffer on Games, Floating Point Determinism: https://gafferongames.com/post/floating_point_determinism
- Gaffer on Games, Deterministic Lockstep: https://gafferongames.com/post/deterministic_lockstep
- GGPO: https://www.ggpo.net/
