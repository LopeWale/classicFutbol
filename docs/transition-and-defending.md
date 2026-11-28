# Transition urgency and off-ball defending (round 4, area T)

This is the implementation note for Work areas 2 and 4 of
`docs/05-fifa-pes-off-ball-teammate-intelligence.md`. The code is in `sim/psfc-sim/src/ai.rs`: the
team plan (`update_team_plan`, `best_lanes`, `front_mode`), the blocks (`defend_block`,
`attack_block`, `slot_pos`), the defensive branch and class refinement of `outfield_shape_intent`,
and the teammate-contain intent. The memory fields are in `state.rs` `TeamAiMemory`. The tests are in
`sim/psfc-sim/tests/transition.rs` and `tests/defending.rs`.

Evidence labels follow the report: **[official]** EA or Konami statements, **[patent]**,
**[peer-reviewed]**, **[community]**, **[audit]**, **[estimate]** (my value, to be tuned against
the baseline). **[sim-calibrated]** means a value was moved from the report's figure because the
sim's own scale differs. Each such move is explained in the text below.

## 1. Phase machine (the plan writes it once per tick)

```
 lose the ball ──► COUNTER_PRESS (reacting) ──(6/9/12 ticks, Hard/Normal/Easy)──► decision
                       │  press: own >= theirs within 10 m (7 m 5v5), >= 4 (2) own outfielders
                       │         behind the ball, ball beyond 35% of the pitch, team work rate > 450
                       │  else ──────────────────────────────────────────────────────► RETREAT
                       │  window 300 ticks x style (Press 1.2 / Possession 1.0 / Counter 0.6)
                       │         x team work rate (750 = 1.0, clamped 0.7-1.2), floor 180
                       │  early exit: regain (COUNTER_ATTACK), a stoppage, a pass > 16 m,
                       │              ball escaping 14 m (9 m) back past the origin ───► RETREAT
 RETREAT ── until >= 80% of outfielders (the rest offence excluded) are within 5 m of their slot or
            goal-side of the ball (checked from tick 30), cap 570 ticks ──────────────► DEFEND
```

| Parameter | Value | Label |
|---|---|---|
| Reaction before press-or-fall-back | 6 / 9 / 12 ticks (Hard / Normal / Easy) | estimate (6-12) |
| Superiority radius, behind-ball minimum | 10 m and >= 4 (11v11); 7 m and >= 2 (5v5) | peer-reviewed; 5v5 estimate |
| Counter-press window | 300 x style x work rate, floor 180 ticks | peer-reviewed 5 s; official FIFA 19 7 s; estimate |
| Pass that beats the press | > 16 m | peer-reviewed |
| RETREAT | until shape, cap 570 ticks | peer-reviewed 9-10 s |
| Stamina while pressing | x2.5 for the presser, the two lane-closers and the chaser, counter-press only | official FIFA 21 (2.2-3) |

The stamina multiplier enters the existing sprint-drain path in `sim.rs` `step_all_players` (the
"5v5 Rush Short-Term Stamina" block): `drain_mod` is multiplied by `ai::stamina_drain_mult_pm`,
which is computed right after the AI intent.

## 2. Inside the window: presser, two lane-closers, everyone else on the recovery rule

- **Presser:** the elected chaser on a loose ball; otherwise the committed presser, who sprints. His
  contain point is shifted 0.75 m toward the most dangerous receiver lane (cover shadow).
- **Two lane-closers** (`second_presser`, `third_presser`): the next two in the press ranking with
  cost < 70. Back-line slots are never chosen, so the rest-defence core does not join the press. Each
  closer takes one of the two best receiver lanes (`best_lanes`). A lane is ranked by distance from
  the carrier minus half the receiver's progress toward our goal, and the incumbent lanes keep a
  3 m bonus. The two lanes are assigned so the closers' total distance is smallest. The closer stands
  on the lane's nearest point, at least 3 m from the carrier and at most 70% along the lane, and runs
  in the high-speed band. The lane memory is `closer_lane`.
- **Everyone else, in every defensive phase** (the old code skipped this branch in counter-press,
  ai.rs:3732 at d95d1b7): let `d` be the player's progress ahead of the ball.
  - If `d > 1 m` and he is more than 30 ticks from his target, or the ball is travelling toward our
    goal at more than 4 m/s, a **recovery sprint** latches on. It stays on while `d > -3 m` or the
    counter is still running. A player who is already 3 m goal-side keeps tracking the counter in
    the high-speed band.
  - Each burst is capped by work rate: Low 180 / Medium 210 / High 240 ticks, x0.65 after a
    high-intensity peak. Past the cap he runs in the **high-speed band**: a sprint gait capped at a
    real 7 m/s (x0.8 after a peak), relative to his stamina-scaled top speed. After 45 band ticks a
    new burst may start.
  - Back-liners caught upfield are not capped.
  - A recovery sprinter or band runner is **class 2, never class 3**, even while yielding.
  - Goal-side players near their target cruise at 70% (class 3, unchanged).
- **Counter-press anchors:** in counter-press every non-engager except the rest offence has his
  target clamped goal-side of the ball. That is 3 m for most players, or the work-rate holding line
  for front slots. Back-liners hold their defend-block slot on the shared line instead of the
  compressed attacking slot.

| Parameter | Value | Label |
|---|---|---|
| Lane-closers | 2, high-speed band, >= 3 m from the carrier, <= 70% along the lane | official tiers; count estimate |
| Recovery trigger | ahead > 1 m and (ticks-to-target > 30 or ball to our goal > 4 m/s) | estimate (30-45); audit |
| Recovery exit | 3 m goal-side of the ball, kept while the counter runs | peer-reviewed shape; estimate |
| Burst cap | 180 / 210 / 240 ticks by DWR band; rest 45 ticks; back line uncapped | peer-reviewed (Ju et al. 2023); estimate |
| High-speed band | <= 7 m/s real | peer-reviewed band |
| Fatigue | above 200 m of > 5.5 m/s running in the last 60 s (leaky integrator `hi_dist_mm`): cap x0.65, band x0.8 | peer-reviewed 20-53% drop; peak sim-calibrated |
| Cruise | 70% of top speed, class 3 | official (Konami) |

**Sim-calibrated values, and one deviation from the report:**

- **High-intensity peak: 200 m, not 70 m.** Sim outfielders run 100-290 m above 5.5 m/s per 60 s
  in 11v11, which is 2-4x real football. At 70 m everyone would count as fatigued all match.
- **The 10-12 m slot band is not used.** The report puts a defender who is behind the ball but
  10-12 m from his slot in the 5.5-7 m/s band. In the sim that slows the re-forming. On 8 seeds of
  11v11, `shape.defending.line_x_std_m` was 7.45 m with the band and 6.67 m without it, and
  behind-ball at 6 s fell 1.5 points with the band. So players far from their smoothed target keep
  sprinting (`dist_sprint` > 5.2 m, the pre-round-4 rule). The band is used only after a capped
  burst, by the lane-closers and by the second man.

## 3. Defensive work rate and the rest offence

`PlayerStyle::def_work_rate` reads `PlayerAttributes::work_rates(role).1`. The bands are Low <= 37,
Medium 38-62 and High >= 63 [official FIFA 11 attribute; band edges estimate]. In the sample squads
forwards are Low (36), midfielders High (64) and defenders High (82).

- **Rest offence** (`rest_offence`): exactly one Low-DWR player in a Striker or Pivo slot. The
  incumbent keeps the role while he is eligible; otherwise the role goes to the lowest DWR, then the
  most advanced slot. He is never threatened and never marks. In 11v11 he keeps the outlet slot:
  up to 6 m beyond the ball, never past halfway. If no forward is Low, nobody stays high.
- **Every other front slot** (striker, pivo, winger) gets a work-rate holding line (`FrontMode::Hold`).
  This replaces the 11v11 striker pinned at ball + 6 m. The slot drops to `min(slot, ball - hold)`
  but no deeper than its band's floor in the block.
  - Hold: Low 4 / Medium 7 / High 10 m (5v5: 2 / 3.5 / 5 m).
  - Floor: Low 55% / Medium 35% / High 15% of the block length (the attacking-midfield, midfield and
    deep bands).
  - Label: community FIFA 23 zones; the hold values are an estimate.
- **Threatened:** a Low-DWR player only within 40 m ahead of the ball; everyone else always.
- **Marking:** Medium- and High-DWR front-slot players may now mark. The exclusion at ai.rs:3200
  is lifted for them.

## 4. Rest defence while attacking (`attack_block`, `slot_pos`)

- The attacking back line holds 7 m (5v5: 4 m) goal-side of the deepest opposing outfielder.
- With the ball in the final third, the line is never more than 45 m (5v5: 18 m) from our goal line.
- Rest-defence centre-backs and the fixo stay within +-14 m laterally (+-8 m in 5v5), so the rest
  defence is about 28 m wide.
- Back-line slots never become lane-closers.
- Labels: peer-reviewed geometry (Forcher et al. 2023); the offsets are an estimate.

The loose rest-defence marks (5 m, tightening to 2-3 m) are **not** implemented. They need a hook in
the attacking branch of `outfield_shape_intent`, which belongs to area U.

## 5. Off-ball defending

- **Cover shadows.** The presser's contain point is shifted 0.75 m toward the most dangerous lane.
  Lanes are ranked by value (closeness to our goal of the receiver's 1 s look-ahead) times reach
  (closeness to the carrier).
- **Lane blocks.** A zonal holder without a mark who is not on the back line, and whose slot is
  within 12 m of the ball, stands on the best lane within a 3 m leash of his slot, at least 2.9 m
  from the ball. A holder farther from the ball keeps his slot.
- **Marking arbiter** (`update_team_plan` step 5), FIFA 14's "most dangerous player":
  1. Incumbents are processed first and kept while their opponent is within 1.35 x the zone of the
     slot.
  2. Free opponents are then taken in danger order (value x reach of the 1 s look-ahead point), each
     by the nearest free defender within the zone.
  3. After a 24-tick dwell, an incumbent may switch only to a free opponent who is 3 m closer.

  The old greedy `taken[]` steal (audit 3225-3235) is gone.
- **Runner in behind** (Konami US8597094). A back-liner keeps a marked opponent who is goal-side of
  the shared line while he is within 15 m and no teammate covers the runner's line to goal. A teammate
  covers it when he stands within La = 3 m of the runner-to-goal segment and closer to goal. The
  marker follows him at 90% weight with no line clamp; once a cover exists he hands the runner over.
  The mark point switches from the goal-side shadow to the intercept line only when the carrier faces
  the marked man within theta_a = 25 degrees and a cover exists.
- **Shared line** (`line_prog`). One value per team, lag-tracked at 120 permille per tick from the
  defend (or retreat) block line. Back-liners stand on it plus their slot depth offset, with one
  team-wide judgement-error term, which replaces the per-player noise at ai.rs:3737. A marking
  back-liner may leave it by at most 2.5 m deep or 1.2 m high, unless he tracks a runner. The holding
  screen keeps -4 / +3 m.
- **Run-tracking policy** (`defend_block`), using FC 25's bands:
  - Counter = Deep: the line drops with any runner (an opponent running at our goal faster than
    3.5 m/s, from 2 m in front of the line to 10 m beyond it).
  - Possession = Balanced: the line drops only when the runner's 1 s look-ahead beats it by 2 m.
  - Press = High: only when it beats the line by 6 m.

  The 3.5 m drop off an unpressured carrier and the -0.8 m offside tie are kept.
- **Cover tuck.** Once the carrier is at or past the shared line, the ball-far full-back tucks in to
  60% of his width.
- **Presser election.**
  - A midfielder is preferred (-3 ticks) in the middle third.
  - Zone handover: a candidate pays 2 ticks per metre that his defend slot is beyond 12 m laterally
    from the ball. Outside the counter-press, an incumbent is released once the carrier is beyond
    12 m laterally from his slot, or 8 m beyond his line (back-liners excepted). The nearest player
    in the new zone then takes over.
- **Press triggers** (FIFA 19): a heavy touch (ball more than 1.6 m off the carrier), a carrier
  facing his own goal, or a carrier within 5 m of the touchline sets `press_trigger_ticks` = 45 in
  DEFEND or RETREAT. While a trigger is active:
  - the presser sprints beyond 1.4 m whatever his stamina;
  - a second man (the best-ranked non-back-liner with cost < 50) shadows 3.5 m behind-side of the
    presser, toward the best lane, in the high-speed band;
  - the second man is class 1, keeps 3 m from the carrier and never tackles.
- **The human's contain mate** (`contain_mate`) is registered in the plan as the second engager:
  - he is excluded from the presser, closer, cover and mark elections;
  - an AI presser elected while contain is held is demoted to cover;
  - his target is low-passed through the plan's target memory (alpha 0.65) and his class is 1;
  - in sim.rs the contain selection skips a back-liner who owns a runner (`ai::owns_runner`).
- **Engager cap.** Every non-presser keeps at least 2.9 m from the carrier (3 m for the cover,
  closers and second man), outside the 2.5 m radius that `coord.engagers3` counts.

| Parameter | Value | Label |
|---|---|---|
| Presser lane shift | 0.75 m | concept official/patent/peer-reviewed; value estimate |
| Lane block | within 12 m of the ball, 3 m leash | official two-tier rule; values estimate |
| Mark arbiter | incumbents first, +3 m, dwell 24 ticks, danger order | official FIFA 14; audit; values estimate |
| Cover check La / theta_a | 3 m / 25 degrees | patent; values estimate |
| Runner tracking range | 15 m | estimate |
| Shared line lag | 120 permille / tick | estimate |
| Run-tracking bands | Deep 0 / Balanced 2 m / High 6 m look-ahead margin, runner > 3.5 m/s | official FC 25 bands; triggers estimate |
| Second man | 3.5 m behind-side, never inside 3 m | official FIFA 12; distance estimate |
| Engager gap | 2.9 m (3 m cover / closers / second man) | audit; estimate |
| Press triggers | heavy touch 1.6 m, back to goal, touchline 5 m, hold 45 ticks | official FIFA 19; values estimate |
| Zone handover | 12 m lateral, 8 m depth | official FIFA 12 rule; width estimate |

## 6. New memory fields (`TeamAiMemory`, all `#[serde(default)]`, hashed)

| Field | Type | Meaning |
|---|---|---|
| `cp_decided` | bool | Press-or-fall-back decision taken for the current counter-press |
| `third_presser` | u8 | Second lane-closer (`AI_NONE` default) |
| `closer_lane` | [u8; 2] | Receiver lane of each closer |
| `contain_mate` | u8 | Human's teammate-contain mate registered as the second engager |
| `rest_offence` | u8 | The Low-DWR forward kept high |
| `line_prog` | i64 | Shared back line, in mm of progress from our goal line (0 = unset) |
| `press_trigger_ticks` | u16 | Active press trigger countdown |
| `rec_flags` | Vec<u8> | Per player: recovery sprint latched |
| `burst_ticks` | Vec<u16> | Per player: ticks of the current recovery burst |
| `hi_dist_mm` | Vec<u32> | Per player: high-intensity distance in the last ~60 s (leaky integrator) |

A state saved without these fields loads (`tests/transition.rs`
`old_snapshot_without_area_t_fields_loads`).

## 7. Analytics additions (`psfc-replay` `analytics.rs`)

- `trans.forward_sprint_share_0_5s_7mps`, `trans.mid_sprint_share_0_5s_7mps`: the sprint shares
  at 7.0 m/s. The 6 m/s variants are kept.
- `trans.outfield_stamina_end_permille`: mean outfield stamina on the last open-play tick.
- `coord.mark_steal_per_defender_min`: mark-target changes per defender-minute while the old target
  is still within 1.35 x the zone of the defender.

## 8. Before and after

See the area-T report (the coordinator merges it into the round-4 results). One finding matters for
reading the transition gates: the 11v11 runs collapse outfield stamina to the sim's floor (150
permille) by the 10-minute mark in the **base** as well (`trans.outfield_stamina_end_permille`
155.8). At that point an AI sprinter tops out near 7.4 m/s, while ball carriers and passes in a
counter travel faster. `trans.forwards_staying_high_mean` therefore stays near 2 in 11v11 whatever
the recovery rule. With no burst cap at all it measured 1.67 (3 seeds); the base measured 2.03 on 8
seeds. The target of 1.0-1.3 needs the stamina model, or the attacking shape that leaves forwards
20-40 m ahead of the ball at the turnover, to change as well.

## 9. Follow-up 1: 11v11 long-term stamina

**Cause.** The sprint drain in `sim.rs` `step_all_players` was one rate for both formats: one
permille per `420 / drain_mod` ticks, about 9.5% of sprint ticks for a stamina-85 player. 5v5
balances it with a jog recovery of +2 every 6 ticks. 11v11 recovers only when standing, +1 every 12
ticks. So 11v11 outfielders drained about 6x faster net and hit the 150 floor before minute 10
(`trans.outfield_stamina_end_permille` 155.8 on 8 seeds at e53e13d; 5v5 ended near 920).

**Fix.** The 5v5 path is unchanged. 11v11 uses the same base rate x `STAM11_RATE_PM` (66 permille)
x the stamina left above the 150 floor, at a 42 000-tick resolution. The x2.5 pressing multiplier
is kept. Drain therefore slows as a player tires, which gives a long-term decay curve.

Measured (seed 1, CPU Normal vs Normal):

| clock | outfield | midfielders (High DWR) |
|---|---|---|
| 10 minutes | 885 | 886 |
| 45 minutes | 639 | 605 |
| 90 minutes (45-minute halves) | 510 | 450 |

On 8 seeds of 10-minute matches, outfielders end at 887.

The test `tests/transition.rs` `eleven_a_side_ten_minute_match_keeps_stamina_above_800` checks the
10-minute level. `eleven_a_side_ninety_minute_stamina_curve` (ignored, about 2 minutes) prints the
90-minute curve.

## 10. Follow-up 2: 11v11 re-tune after the stamina fix

The comparison base here is e53e13d with only the stamina fix applied ("base+stamina"), measured on
16 seeds of 11v11 Normal.

| Metric | base+stamina | before re-tune | after re-tune |
|---|---|---|---|
| `forwards_staying_high_mean` | 1.68 | 1.94 | 1.67 |
| behind the ball at 3 s / 6 s | 67.5 / 74.1% | 65.1 / 70.3% | 67.3 / 73.2% |
| `line_x_std_m` | 6.56 | 6.56 | 6.23 |
| `time_to_shape_static_s` | 1.00 | 1.12 | 1.09 |

What moved:

- **Counter-press window.** Now 180 x style x work rate, floor 90. The report's 300 / floor 180 cost
  about 2 points of behind-ball.
- **RETREAT.** Back to 75 ticks. "Until shape" cost 0.6 m on `line_x_std_m`.
- **Far-side full-back.** Rest-defence depth 90, was 260: -0.4 m on `line_x_std_m`.
- **Forward holding lines.** 8 / 10 / 12 m with floors 400 / 300 / 150: forwards staying high
  1.80 -> 1.68.
- **Lane-closers.** Only on lanes toward our goal.
- **Burst caps.** 210 / 240 / 240 ticks.
- **Counter latch.** When the ball runs at our goal faster than 2.5 m/s (was 4), players not yet
  3 m goal-side also latch a recovery sprint. On a loose ball only players more than 15 m from it
  latch.

The 15 m restriction is a trade-off. Without it (every player latches on any counter, including a
pass in flight), 16 seeds measured forwards 1.51 and behind-ball 70.4 / 76.3%. But
`tests/exclusivity.rs` `exclusivity_metrics_11v11_normal` then measured 64 behavioural
dual-chaser ticks per minute (bound 50), because recovery runs along the ball's path count as
second chasers. With the restriction it measures 44.5.

## 11. Follow-up 3: the 5v5 autopilot side

The 3-seed regression (behind-ball at 6 s 71 -> 64%) was seed noise. Measured on 128 seeds
(seed bases 1 and 65, 64 seeds each), home = the autopilot side:

| Metric | base e53e13d | area T |
|---|---|---|
| Autopilot side behind the ball at 6 s | 62.2% | 67.7% |
| Goals conceded by the autopilot side | 3.60 | 3.98 |

The goals difference is +0.38, about 1.5 standard errors.

Bisect, 64 seeds each, goals conceded by the autopilot side:

| Variant | Goals conceded |
|---|---|
| Full area T | 3.97 |
| No third closer and no second man | 3.88 |
| No zone handover and no midfielder preference | 3.83 |
| No rest-defence changes | 3.73 (behind-ball at 6 s back to base, 64.5%) |

- No single change isolates the goals difference.
- Two candidates are inactive in autopilot: the contain-mate registration and the presser
  demotion. `suggest_cpu_input` never presses `BTN_TEAMMATE_CONTAIN`.
