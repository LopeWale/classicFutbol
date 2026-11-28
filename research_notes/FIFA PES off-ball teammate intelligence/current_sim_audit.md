# Current sim audit: off-ball AI at main d95d1b7 (internal code audit, not external research)

This note is a read-only audit of `sim/psfc-sim` made on 2026-11-28 at commit d95d1b7 (after PRs #23 and #24). Line numbers refer to that commit. It exists so the report and the implementation briefs target real gaps rather than assumed ones. Paths are under `sim/psfc-sim/src/` unless they start with `docs/`.

## Five findings that matter most

1. **Forwards jog back after a turnover.** In COUNTER_PRESS every non-engager skips all defensive refinements and gets move class 3 (ai.rs:3732, 4110-4121). In DEFEND and RETREAT, forwards cannot mark, and the 11v11 striker slot is pinned in front of the ball.
2. **Move classes are stale for CPU sides.** The avoidance frame is built before CPU-side AI players compute their intents, so on a CPU side avoidance only sees the baseline classes and never class 3.
3. **The pocket search has a lateral flip-flop.** The two forward pockets are mirror images (+-2.6 m), and the "keep my pocket" bias is measured against the lagging smoothed target (ai.rs:3866-3916).
4. **No run ever goes beyond the offside line, and the number of runners is not capped.** Every run target is clamped to the onside limit, and every runner-role player runs at once (ai.rs:3939-3960).
5. **Teammate contain can produce three engagers.** The contain mate is invisible to the team plan, so the human, the contain mate and an AI presser can all engage.

Two corrections: `best_short_idx` is the corner-kick short-pass receiver picker in sim.rs:2203-2266, not part of `generate_cpu_input`. ai.rs `realistic_velocity_step` (4371) is dead code; AI players off the ball use `avoidance::motion_step` (avoidance.rs:634, called at sim.rs:3882).

## 1. Attacking (own side has the ball)

**Formation slot and phase shift**
- `attack_block` (ai.rs:2670-2715) sets the rest-defence line at `min_line + 0.6 x (ball - pivot)`, always at least 18 m (11v11) or 8 m (5v5) behind the ball. The front is the onside line minus 0.4 m (2687-2688); length clamped to 34-56 m (11v11) or 14-32 m (5v5). Possession style raises `min_line` by 2 m (1 m in 5v5); Counter style lowers it by 3 m (1.5 m) and narrows width to 940 permille. Ball-side shift 240 permille (11v11) or 180 (5v5).
- `slot_pos` (2719-2745): ball-side full-back +150 depth; far-side full-back tucks in to depth 260 with y x 0.65; striker and pivo capped at `front_cap`, which pins them to the offside line.
- `phase_block` (2752-2759) uses ATTACK or COUNTER_ATTACK whenever the team owns the ball.
- `assign_slots` (2772-2885): exact search over orderings in 5v5, pairwise-swap search otherwise. A new assignment is adopted only after 30 held ticks and only if strictly cheaper, with a 900 dm2 (about 3 m) penalty per moved player (2795-2797, 2875).

**Role refinements**
- 5v5 fixo rest defence: pinned behind the ball and drawn toward the ball's y (3811-3829). 5v5 vacated-cutback run fires when nobody is in the central box (3830-3853).
- Support triangle: `designated_support_triangle_idx` (1298-1318) picks the teammate whose distance to the ball is closest to 9.5 m, with a 2.5 m bonus for midfielders. If his target is more than 14.5 m (13.8 m in 5v5) from the ball it is pulled to 11.8 m (9.5 m) along the same bearing (3854-3861). No angle or lane check despite the doc comment "open passing angle"; no hysteresis, re-picked every tick.

**Pocket search, the only lane-aware support logic** (3862-3917)
- Five fixed offsets around `att_target`: zero, (+2.8, +2.6), (+2.8, -2.6), (-1.8, +-3.4 chosen by index parity), (+3.6, 0) m.
- Score = `nearest_opp + bias - 2 x lane_shadow_pen - mate_crowd_pen`. `lane_shadow_pen` checks opponents within 1.85 m of the straight ball->candidate segment (3880-3895): the only "is an opponent blocking carrier->teammate" check off the ball. Candidates within 5 m of the ball are skipped. Bias +0.95 for the anchor and +0.9 if within 1.5 m of `prev_target` (3906-3909).
- Not checked: the lane check runs before `support_spacing`, the run push and `reserve_destination`, so the final target is never re-checked for an open lane. No distance or angle to the carrier, no receiver orientation.

**Support spacing (GRF force-field teammate term)**: `support_spacing` (4156-4183) only in possession or while our pass is in flight (3929-3933). Six nearest teammates repel the target with weight 0.4 over a 10.5 m scale (6.5 m in 5v5); total shift capped at 3 m (2 m). Index-based fallback direction within 150 mm.

**One runner per destination**: `reserve_destination` (4189-4243), possession only (3961-3963). If a teammate's live target (this tick or last) is within 6 m (5 m in 5v5), whichever player is farther moves his target out to 6 m; two passes. Carrier and chaser exempt.

**Runs**
- Who runs (3934-3960): anyone with `run_boost_ticks > 0`, or anyone with role Striker, Pivo, Winger, Ala, WideMid (or `PlayerRole::Forward`) who is not a rest-defence player, while the opponent does not own the ball and either `passer_ready` or COUNTER_ATTACK holds.
- `passer_ready` (3920-3927) only means the carrier's facing has x x attack_sign > -0.15 (not facing backwards). Pressure, space and carrier speed are ignored.
- Run push is a constant forward offset on `att_target`: `run_boost` 11.5 m (7.8 m in 5v5); striker or pivo 7.8 m (5.6); counter-attack 8 m (5); other runners 5.6 m (3.8).
- Clamp to the onside limit (3950-3955): `compute_onside_limit_x` (2342-2368) takes the second-last opponent (keeper included), or the ball if further forward, or the 5v5 one-third zone, minus 0.35 m. Runs hold the onside line and never go in behind until the ball is played.
- No cap on simultaneous runners; only `reserve_destination` separates end points.
- Sprinting: `reserved_run` and `counter_run` make the player class 2 (4102-4105); `counter_sprint` needs more than 3 m to the target (4093); `run_boost` sprints while the opponent does not own the ball (4094).
- Curved run around a blocker, attacking phases only: bends the move vector when an opponent is 0.15-2.2 m ahead and within 1.05 m laterally (4064-4086).

**`BTN_CALL_RUN` -> `trigger_teammate_run`**: rising edge, attacking context only (sim.rs:868-871); also a passing modifier (sim.rs:422, 3227, 3293, 3416). Runner (sim.rs:2832-2854) = the outfielder (not keeper, not controlled) with the highest x progress, +6 m bonus for `PlayerRole::Forward`; space, lanes, offside and whether he is already running are not considered. No target is chosen: it sets `run_boost_ticks = 90` and the shape intent supplies the target (+11.5 m, still clamped onside). The CPU never presses `BTN_CALL_RUN` or `BTN_TEAMMATE_CONTAIN`.

**Other `run_boost` writers**: space throw-in receiver 45 (sim.rs:2149); teammates within 15 m of goal on a corner cross 40 (2365); knock-on by the carrier 24 (3353); through-ball or wing-pass receiver 45 (4721); tackle winner 16 (5025, 5209); first-touch knock 22 (6525); reset 0 (1102).

**CPU-controlled carrier (`generate_cpu_input`, ai.rs:1794-2189)**
- Decision beat every 16 / 9 / 5 ticks (Easy / Normal / Hard), or every 3 ticks when an opponent is within 1.85 m (1705-1710, 1818-1821).
- Shot (1823-1860): angle cos >= 0.44, distance (box-finish, or long shot if shooting >= 80), `is_shot_corridor_blocked` on either corner; lob and finesse by difficulty.
- Byline cutback (1862-1905) to a box teammate with 2.1 m clearance; wing cross (1907-1929) as a lob at the penalty spot if any teammate is in the box.
- Pass scoring (1953-2127): every teammate as a feet target and a through-ball target (5.2 m lead along his velocity, or straight forward); offside teammates filtered (1977-1984); time-to-intercept margin per opponent to the end point and to the closest point on the lane, plus a point-blank block test (2033-2058); lofted switch allowed when the ground pass is unsafe (2066-2072). Score = forward gain + threat gain + receiver space + margin bonus + through bonus - return-pass penalty - keeper penalty.
- Release rules (2130-2142): "killer ball", progression under moderate pressure, or a safe recycle under tight pressure. Clearance only when deep and swarmed (2144-2159). Otherwise dribble at goal centre with `evade_stride`; sprint depends on free space ahead; Hard adds a skill move (2162-2188).

**Gaps (attacking)**
- Support targets are not checked for an open carrier->target lane after spacing, reservation or run push; the pocket search uses five fixed offsets with no distance or angle preference.
- Run timing uses only "not facing backwards"; pressure, space ahead, first touch and carrier speed are ignored.
- No run in behind the line is ever initiated off the ball; every run target is clamped onside.
- Nothing limits how many players run at once; no staggering.
- No third-man runs, no dynamic overlaps or underlaps (only the static full-back +150 depth), no come-short timing beyond the support-triangle pull, no decoy runs.
- Onside handling is a hard clamp: no curved run along the line, no timing to the pass release.
- `BTN_CALL_RUN` picks the most advanced player, not the best runner, with no target, lane or offside awareness.
- The CPU carrier's pass choice does not coordinate with off-ball runs; it reads only current positions and velocities.

## 2. Defending (opponent has the ball)

**Phase machine** (`update_team_plan`, ai.rs:2923-3003)
- Losing the ball from a non-defensive phase gives COUNTER_PRESS if all three hold (2977-2978): our players within 12 m (8 m in 5v5) of the ball >= theirs; ball progress > 35% of the pitch; team work rate > 450. Otherwise RETREAT. `phase_origin` is stored.
- COUNTER_PRESS -> RETREAT if the ball escapes 14 m (9 m) back past the origin (2982-2985). COUNTER_PRESS -> DEFEND after `cp_window` (2986-2988) = Press 150 / Possession 110 / Counter 55 ticks x team work rate / 1000, minimum 40 (2936-2942). RETREAT -> DEFEND after 75 ticks (2990-2993).
- Winning the ball from a defensive phase gives COUNTER_ATTACK, then ATTACK after `ca_window` = Counter 150 / Press 100 / Possession 60 ticks (2943-2960). Loose ball: phase held (2995-3000).
- Counter-press shape: `phase_block` uses `attack_block(counter_press)` with `attacking = true` (2756, 2689-2700): line = ball - 17 m (-9 m in 5v5), length 24 m (13 m), width 780 permille, shift 500 permille. Non-engagers stand at their attack-depth slots, compressed.

**Presser, second presser and cover election** (3030-3182)
- Candidates: outfielders within 16 m (20 m in counter-press; 11.5 / 14 m in 5v5) of the ball; excluded: the controlled player (both CPU and human sides), the chaser, everyone on a loose ball when a chaser or keeper claim exists (3081-3090).
- Cost (3061-3075): ticks to the ball predicted 8 ticks ahead, +14 for a back-line slot when the ball is more than 8 m beyond the line, -(aggression - 500)/100, +(500 - work rate)/100.
- Hysteresis (3092-3101): incumbent kept unless a challenger is better by `margin` (Easy 10 / Normal 8 / Hard 6); after `hold` ticks (40 / 28 / 20) the margin drops to 3 (2502-2508).
- Controlled-player rule (3102-3106): an AI presser is committed only if faster than the controlled player by 5 ticks (2 if incumbent), or the incumbent is within +3 ticks.
- Second presser: counter-press only, next-ranked player with cost below 70 (3107-3112).
- Cover point (3123-3176): midway between the ball and the presser plus 6.5 m (4.5 m in 5v5) toward our goal; nearest player wins with role preference (CB, holding, fixo 0; FB, CM +3 m; others +8 m) and a 3 m incumbent bonus. On a CPU side the controlled player becomes the cover when an AI presser exists.
- Engager cap (3967-3985): anyone who is not the presser keeps a goal-side gap of at least 2.2 m from the carrier (3 m for the cover).

**What each engager does** (`outfield_shape_intent`): counter-press presser (3674-3680) ball + goal direction x (0.95 m + stamina term), forced sprint, alpha 0.65; counter-press second presser (3681-3699) 45% along the carrier -> nearest opponent line (a lane block; nearest opponent re-picked every tick, no hysteresis), sprints beyond 2.5 m, alpha 0.50; normal presser (3700-3713) goal-side contain gap 1.55 m - 0.6 x (aggression - 500) mm + stamina term + difficulty, clamped 1.05-2.1 m, alpha 0.60, press sprint beyond 1.4 m if stamina > 400 + (1000 - work rate)/4 (4092); cover (3714-3731) behind the presser, clamped to at most the back line + 2 m (except counter-press), alpha 0.38.

**Man-marking** (3184-3251): DEFEND and RETREAT only; greedy by slot `defend_depth` (back line picks first); excluded: controlled player, chaser, presser, cover, and Striker, Pivo, Winger slots (3194-3200). Candidates: nearest opponent within 9 m (7 m in 5v5) of the marker's defend anchor, not the carrier, not taken, no further forward than the block front + 2 m. Hold: previous mark stays while within 1.35 x the zone; replaceable after `hold` ticks by a candidate at least 3 m closer (3225-3235); because the greedy loop sets `taken[]`, an earlier-ordered defender can take my mark and I lose it immediately. No explicit handover: a runner going in behind is followed only while within 12.15 m of the marker's anchor, which moves with the block, not the runner. Mark point (3773-3790): opponent + 1.35 m along a blend of 65% goal-side and 35% ball-side; the player moves 52-60% of the way from his anchor toward it; back-line and holding players clamped to their line -4 m / +3 m.

**Back-line height and offside line**: `defend_block` (2602-2666) sets the line at `min_line + 0.55 x (ball - pivot)`, at least 14 m (8 m in 5v5) from the ball (2626). Offside tie (2627-2636): line kept <= deepest opponent (keeper excluded) - 0.8 m, never below halfway (11v11) or one-third (5v5): a "never above an attacker" rule; no step-up or trap. Drop by 3.5 m x team positioning when the carrier is unpressured (nobody within 2.8 m) and facing our goal (2637-2651). Retreat subtracts 4.5 m (2.5 m) (2652-2654). Style clamps `max_line` (2610-2617). Judgement noise (3737-3745): smooth error per player, 150-tick period, scaled by (100 - positioning) and difficulty, 60% for non-back-liners; independent per player, so the line is uneven with no shared line value. Back-liners hold height by yielding only sideways (4007-4016); holding a line is a "triggered" class-2 task (4107-4108). Compactness: length 30 m (17 m in 5v5), minimum 26 m (9 m); width 720 permille (700), Press style -30; shift 380 permille (320) of `ref_ball.y` (2605-2625, 2742). `defensive_shape_anchor` (3257-3268) is only a read-out of the DEFEND block for metrics and tests.

**Cover shadows and lane blocking by non-engagers**: only the counter-press second presser (3681-3699), the cutback-zone mark shading 58% toward the pass lane for back line and holding only (3746-3772; nearest box opponent re-picked every tick), and the 35% ball-side shade of a man-mark. Zonal holders without a mark stand on their slot and block no lane. The presser stands on the ball->goal line and does not angle to cut a pass.

**Teammate contain** (ai.rs:1385-1408): contain point 1.48 m (Tactical) or 0.65 m (Legacy) goal-side of the ball predicted 5 ticks ahead; normalized direction beyond 0.14 m; sprint beyond 1.45 m; jockey within 1.75 m. No smoothing, no class refinement, no plan exclusion. Selection and stamina in sim.rs:837-866.

**Tactical vs Legacy for AI teammates**: the only AI-side difference is the contain gap. `DefendingMode` otherwise only affects the human-controlled player's contain button (sim.rs:3613-3661) and facing (sim.rs:3934); the team plan ignores it.

**Cruise cap and move classes while defending**
- Baseline (`assign_move_classes`, 3470-3488): 0 carrier or chaser; 1 presser, second presser, cover, human-controlled player; 2 everyone else.
- Per-player refinement (4101-4121): class 1 if engaged; class 2 if triggered and not yielding; otherwise class 3 with `sprint = false` and the move vector capped at `CRUISE_TOP_SPEED_PM (700) x 1.22` (1.205 in 5v5) of non-sprint top speed = 70% of sprint speed (3317, 4117-4121).
- "Triggered" in defence: `force_sprint` (recovery or retreat sprint); a man-mark (DEFEND or RETREAT only); back-line or holding role in any defensive phase; `run_boost`; `dist_sprint`, the far-from-slot threshold: more than 5.2 m (3.9 m in 5v5) from the smoothed target, staying on until below 55% of that (4089-4091).
- Recovery sprint (3792-3800) needs both: the player ahead of the ball by more than 1 m, and his deficit to `raw` exceeding `thresh = 2.5 m (1 m in 5v5) + recovery x 2.5 mm`, i.e. 2.5-5.0 m. Because recovery = (pace + acceleration) x 5, faster players wait longer.
- Retreat sprint (3801-3806): RETREAT only, player ahead of the ball and more than 8 m (5 m) from `raw`.

**Why forwards jog back after a turnover**
1. In COUNTER_PRESS (the usual result of losing the ball high up): the whole non-engager defensive branch is skipped (`else if phase != AI_PHASE_COUNTER_PRESS`, 3732), so no recovery sprint, no retreat sprint, no mark. The anchor is the attack-depth slot in the compressed counter-press block, so forwards stay high. They are not engaged, the back-line/holding trigger does not apply, the target is near so `dist_sprint` is false. Result: class 3, sprint forced off, 70% cap (4110-4121), for the whole `cp_window` of 40-150 ticks.
2. In DEFEND and RETREAT: Striker, Pivo and Winger slots can never mark (3200). The 11v11 striker's defensive slot is pinned at ball + 6 m up to halfway (2737-2740), so his deficit stays small and the recovery trigger rarely fires. The retreat sprint needs more than 8 m to target and lasts at most 75 ticks. Unless `dist_sprint` fires, they are class 3.
3. Any yielding player is class 3 (4112), overriding triggers.
4. AI top speed is also scaled by 0.76-1.0 x stamina (sim.rs:3731-3736).
5. No work-rate attribute. `player_style` (2428-2439) has discipline, aggression, recovery (pace + acceleration), work_rate (stamina attribute x current stamina) and tackle_confidence. Work rate is used only in the press cost (3071), the press-sprint stamina gate (4092) and the team counter-press window and gate (2924-2942, 2977). It plays no part in recovery-run decisions or class assignment.

**Gaps (defending)**: no recovery runs in counter-press for non-engagers; forwards have no defensive duty (never mark, never screen, striker stays as outlet by design); recovery timing inverted and without a work-rate term; no cover shadow or lane blocking by the presser or zonal holders; no mark handover and no explicit tracking of a runner in behind (a mark can be stolen within a tick); no coordinated line step-up or offside trap and no shared line value; no second-man press outside counter-press other than the human-triggered teammate contain; no press triggers (bad touch, back to goal, sideline trap).

## 3. Steering and target smoothing (lateral flip sources besides avoidance)

**Smoothed target**: low-pass (4035-4041) `target = prev_target.lerp(raw, alpha)`, base alpha = 120 + discipline/5 permille (0.12-0.32 per tick, 3669); overrides: counter-press presser 0.65, presser 0.60, second presser 0.50, cover 0.38 (at least 0.65 within 4.5 m of the carrier); clamped 0.08-0.90; applies only when `target_tick == tick - 1`, otherwise the target snaps to `raw`. Paths that write no target memory: chase (`chase_intent`, 3543-3559), pass receiver (1331-1363), teammate contain (1385-1408), non-open-play (1365-1383), the human-controlled player; returning from any of these snaps to `raw` with no smoothing, and `reserve_destination` ignores their stale reservations (4209-4213). Arrive dead zone (4046-4062) 160-510 mm by discipline, doubled below 0.4 m/s; speed is braking-consistent 0.75 x sqrt(18 x d). Sprint flag on above 5.2 m, off below 2.86 m; jockey on at 2.6 m, off at 3.2 m (4088-4097, 4123-4130); the stored sprint flag is the value after the class-3 override, so yielding or class 3 erases sprint hysteresis for the next tick.

**Motion stage** (sim.rs:2856-3958): `filter_velocity_command` (ai.rs:4307-4329), k = 0.25 + 0.002 x agility (0.25-0.45), applies to AI and CPU-driven non-carriers in Normal action (sim.rs:3829-3845) after `select_velocity` (sim.rs:3813-3825); resets when not run on the previous tick. AI players off the ball use `avoidance::motion_step` (avoidance.rs:634-656): heading cap a_lat/v clamped to 2-12 degrees per tick, a request beyond 90 degrees brakes to 0. `limit_turn_rate` (ai.rs:4335-4364) only for humans, carriers and receivers (sim.rs:3895-3897). `limit_facing_turn` (4408-4429) for the realistic movers (sim.rs:3953-3955).

**Sources that can change a target's lateral side tick to tick**
- Pocket search (3866-3916): candidates 1 and 2 mirrored at +-2.6 m; the keep-previous bias compares against the smoothed `prev_target`, not the previously chosen candidate; while the target lags neither pocket may be within 1.5 m, so the choice can flip sides. The parity-signed back offset (3870) is fixed per player.
- `designated_support_triangle_idx` (1298-1318) re-picked every tick, so the 11.8 m pull can jump between players.
- `yield_target` (3578-3612): side sign from `perp.dot(mate_pos - a)`; a teammate near the carrier's path line flips side; radial from `mate_pos - closest`.
- `keeper_corridor_yield` (4248-4269): sign from the lateral position relative to the keeper->claim line.
- `reserve_destination` (4214-4236): who yields depends on `d_me` vs `d_mate`; a near-tie swaps who moves, with a jump up to 6 m; index-based lateral direction when targets coincide within 0.2 m, mirrored near the touchline.
- `support_spacing` (4173-4179): index-based +-y fallback within 150 mm.
- Curved run (4076-4081): `bend_sign` from the blocker's lateral offset; a blocker almost dead ahead flips the bend; applied straight to `move_vec` with no smoothing.
- Counter-press second presser lane (3686-3695) and cutback mark (3753-3770): nearest opponent re-picked every tick, no hysteresis.
- Anti-swarm push (3978-3983): direction from `raw - carrier`, jumps when `raw` crosses the carrier.
- `passer_ready` toggling (3920-3927): the carrier's facing crossing -0.15 switches the run push (3.8-11.5 m along x) and class 2/3 (sprint) on and off.
- Phase flips on ownership change swap attack vs defend slot y values (2720-2724): large jumps.
- Slot reassignment (2875) allowed after 30 ticks with only a 900 dm2 penalty per moved player; the 5v5 exhaustive search runs every tick; a swap of two players' slots swaps their sides.
- Mark reassignment (3225-3235): a stolen mark through greedy `taken[]` bypasses hysteresis.
- Presser (3092-3106): the controlled player is excluded from ranking; after an auto-switch puts control on the AI presser, the incumbent becomes invalid and the presser is re-elected from scratch.
- Avoidance class staleness: `build_frame` (sim.rs:2862) runs before this tick's AI intents. For CPU sides `generate_cpu_input` (sim.rs:334) has already run `update_team_plan`, which rewrites all classes to the baseline (3470-3488), and only the controlled player is refined, so avoidance sees no class-3 players on CPU sides; on human sides it sees the previous tick's refined classes. docs/avoidance-and-contact.md:172 still mentions a fallback classifier; avoidance.rs:159 already calls `ai::move_priority`.

**Gaps (steering)**: no hysteresis on the chosen pocket index, the support-triangle pick, the nearest-opponent lane and cutback targets, or the yield and bend side signs; smoothing resets after the chase, receive and contain paths; avoidance classes inconsistent between CPU and human sides.

## 4. Human-side specifics

**Controlled player**: no AI intent on a human side (sim.rs:2908, 2917-2921); excluded from presser ranking (ai.rs:3083), from cover (`!cpu_side`, 3142) and from marks (3196). If no AI presser is clearly faster (3104), `presser = NONE`, which makes the human the presser by default; the cover point is then computed from his position (3131-3132). Chaser: never elected, but counts as the chaser when within 6 ticks of the best AI or when closing and not clearly beaten (3390, 3416-3424); the AI teammates then yield around his path (3998). On a CPU side the controlled player runs `outfield_shape_intent` through `generate_cpu_input` (2195-2219); he engages if presser (cannot happen, excluded from ranking) or if there is no presser and he is not the cover; otherwise he becomes the cover (3165-3175).

**`move_priority`** (3517-3539): human-controlled player class 1 (3480), or 0 while he is the chaser (3535-3537); every other non-carrier non-chaser at least class 1 (`c.max(1)`). In avoidance he gets `human_adjust` (sim.rs:3818-3820), no command filter and no realistic acceleration (3829-3833, 3876-3880), and no stamina penalty on top speed (3731-3733).

**Auto-switch** (`auto_switch_player`, sim.rs:2708-2788): human side uses its settings mode, CPU sides AssistedBall (828-832). Blocked when we own the ball, manual lock or cooldown is active, an input is buffered, our pass was launched within 50 ticks, or a tackle or slide is in progress. Utility (`defensive_switch_utility`, 2589-2625) = -195 x intercept ticks - 0.075 x distance - air-landing penalty + goal-side bonus; switches when the gain exceeds 1,120, then a 14-tick cooldown. Mismatch: this is not the plan's press cost; on CPU sides `generate_cpu_input` (334) plans before the switch (832), so the plan's `controlled` is one tick stale.

**Teammate contain** (sim.rs:835-866): human only, while holding the button outside attacking context; the mate is picked by the switch utility, excluding the controlled player, mates on cooldown and mates without contain stamina; stamina drops 1 every 2 ticks, at zero a 150-tick cooldown. The plan does not know about the contain mate, who can differ from the AI presser, so the human, the contain mate and the AI presser can all engage, breaking the two-engager cap. His class stays at the baseline (usually 2) and his target bypasses the low-pass.

**`BTN_CALL_RUN`**: human only (section 1); most advanced player plus Forward bonus, 90-tick boost; during our pass in flight the run trigger takes precedence over switching (sim.rs:10668-10684 test).

**Gaps (human side)**: no coordination between teammate contain and the plan; auto-switch and press election use different metrics, causing presser re-election churn; the call-run picker has no target, lane or offside logic; on a human side `presser = NONE` delegates the press to the human even when he is far away, and the AI waits rather than covering the press role.
