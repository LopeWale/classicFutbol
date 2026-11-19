# Parallel Season FC — Requirements Review

Review of the product vision and system requirements document ("Parallel Season FC: Complete product vision, gameplay requirements, profile-cartridge system, and technical architecture"), carried out before designing the system architecture in `02-system-architecture.md`.

## 1. Overall assessment

The document is unusually strong on the part most hobby game projects get wrong: the **record-keeping invariants**. The following rules are stated clearly and are individually testable:

- One fixture, one accepted result, with an explicit result source.
- The shared world takes precedence over later imports (takeover precedence).
- Import *events* against identified fixtures, never provider-wide *totals*.
- Every competitive match keeps a snapshot of squads, attributes, eligibility and rules.
- Completion is idempotent: repeated submission yields one accepted result.
- An older cartridge can never reverse newer accepted results.
- Corrections preserve the original record and rebuild projections without double counting.
- Result sources are assigned in advance; nobody picks the version that suits them after the fact.

Together these describe an **append-only ledger with derived projections**, and the architecture should be built around that observation rather than around a conventional CRUD database.

The document is comparatively weak on **technical constraints**: no engine, language, data provider or hardware platform is chosen, and some requirements (notably online play and result validation) have unstated implications that should be made explicit. Those are listed below.

## 2. Findings that change the architecture

### 2.1 The repository's `.gitignore` is a placeholder

`.gitignore` is the Adventure Game Studio template (`Game.agf`, `acsprset.spr`, `AudioCache/`), chosen at random when the repository was created. It did not influence any decision in this review or in the architecture; it simply needs replacing with Godot, Rust and Python ignores when the repository is initialised in phase 0.

### 2.2 Deterministic simulation is implied but never stated

Three requirements only hold together if the football simulation is deterministic (same inputs, same starting state → bit-identical outcome on every machine):

- "Both clients participate in one agreed match state."
- "The result must come from the validated match session, not an arbitrary score submitted by a client."
- "Responsive control" while synchronised online.

A deterministic, fixed-timestep simulation that is independent of the rendering engine gives you all three for free: rollback netcode for responsive 2-player play, desync detection by state hash, and **result validation by replaying the input log headlessly on the host**. This should be promoted to a first-class requirement; it is the single most consequential technical decision in the project.

### 2.3 Where the host runs and where the 3D client runs are separate questions

Section 16 names "the Raspberry Pi hosting target" and "your other servers". The available hardware is a DigitalOcean droplet, several Raspberry Pis and two Chromebooks, plus participants' own PCs.

- **World host.** Any of these can run the host services; a Pi 5 or a small droplet is ample. The architecture recommends the droplet as primary because its public address removes NAT traversal from online play, with the Pi as backup target or alternative primary (ADR-0002).
- **3D client on the droplet.** Not viable. The droplet has no GPU, software rendering of 22 animated footballers cannot reach playable frame rates, and streaming video to a Chromebook or Pi would add 60–150 ms of input latency to a game that lives on responsiveness.
- **Raspberry Pi 5 as a console client.** Plausible with a deliberate low-poly art budget and Godot's Compatibility renderer, and it fits the product's console identity well. It is gated on a measured render test in phase 1 rather than promised. Pi 4 and earlier are not targets. Several Pis cannot be combined to render one frame.
- **Chromebooks.** Supported best-effort through the Linux container; a browser build is the long-term route and is kept open by compiling the simulation to WebAssembly (ADR-0002).

### 2.4 Season-rollover divergence is a full competition generator

Section 7 says "where the parallel world produces different competition entrants, the subsequent schedule must accommodate them". If your Manchester United qualifies for the Champions League in the parallel world but the real one did not, there are **no real fixtures to import** for that whole competition. The competition engine therefore needs a complete format-driven generator (league phase/group stage, knockout draws, seeding, dates) for every supported competition, not merely a fallback for late cup rounds. This is a large piece of work and should be planned as its own phase.

### 2.5 The unrepresented-versus-unrepresented generated-result policy is undefined

Once a draw diverges, fixtures between two unrepresented clubs have no real equivalent. The document requires "an identified generated-result policy" but does not define one. The architecture proposes a seeded, rating-based model (expected goals from squad strength → Poisson draw → generated scorers), recorded with `source = generated` and the model version, so results are reproducible and auditable.

### 2.6 The cartridge read/write path is a hardware decision, not a software one

Section 11 correctly notes that "a cartridge dock used for carrying the device is not automatically a functioning data reader". A decision is needed between:

1. Controller contains a USB hub; the cartridge slot is a downstream port; the cartridge is a USB mass-storage device. The PC sees a gamepad and a flash drive independently. **Recommended** — gameplay input and profile I/O never contend, and a separate "profile-loading unit" is just a hub with a slot.
2. Controller MCU reads an SD-type cartridge and exposes it as composite HID + mass storage. Fewer parts, but firmware complexity and the risk of input starvation during writes.

### 2.7 Match duration shapes every statistic

Nothing states the in-game match length (classic FIFA defaults were 4–6 minute halves). Short halves produce far fewer goals, cards and passes per match than real football, so parallel statistics will diverge in scale from inherited ones. The group should fix match duration as a versioned season setting and the UI should state it next to any parallel statistic.

## 3. Gaps the document should close

| # | Gap | Why it matters | Suggested resolution |
|---|-----|----------------|----------------------|
| G1 | Data provider unnamed | Coverage, cost, rate limits and licence terms decide what "import events" can actually mean. Lineups and goal events for every unrepresented match are usually a paid tier. | Pick a provider early; design adapters so a second provider can be added. Treat event-level coverage as "best effort, marked incomplete" from day one. |
| G2 | Intellectual property | Real club names, kits, badges and player likenesses are licensed properties. | Fine for a private group. Do not distribute builds with real assets; keep asset packs separable from the game. |
| G3 | Opponent lineup for CPU matches | Playing window may precede or follow the real kickoff, so the real lineup may not exist yet. | Default: team AI selects from the fixture's squad snapshot. Optional season setting: use real lineup when already known. |
| G4 | Offline official results | "Explicit policy for authorisation and reconciliation" is required but not given. | v1: official fixtures require the host to be reachable (LAN counts). Offline play is practice only. Later: signed input-log submission with deferred validation. |
| G5 | Shared control of one club | Mentioned as optional. | Out of scope for v1; data model should allow more than one active assignment per club so it can be enabled later. |
| G6 | Suspensions and injuries | Cards are recorded; whether yellow-card accumulation suspends players is not stated. | Season setting with competition-specific thresholds; default on for parallel discipline, off for imported injuries. |
| G7 | Postponed real fixtures | A real postponement moves the parallel fixture too, but a played parallel fixture must not be "un-played". | Fixture lifecycle must forbid any import from changing a fixture in `played`/`accepted` state. |
| G8 | "Assigned day" across DST | Playing windows must not shift when clocks change or a participant travels. | Store windows as season-timezone local dates plus explicit open/close instants computed once by the host. |
| G9 | Dispute resolution | Admin corrections are described; who may authorise them is not. | Roles: `participant`, `commissioner`, `host-admin`. Corrections need a commissioner. |
| G10 | Group size | Never stated. | Design assumption: ≤ 20 participants per world, ≤ 4 concurrent matches. Correctness, not scale, is the design driver. |
| G11 | Spectators / third controller in the room | Not mentioned. | Out of scope for v1; the controller-assignment model should not preclude it. |
| G12 | Audio, commentary, licensing of crowd audio | Section 10 asks for audio. | Procedural crowd and whistle only; no commentary in v1. |

## 4. Requirements that are large and should be staged

The gameplay chapter (section 9) alone describes most of a commercial sports title. The design assumes the following staging so a playable, record-keeping season exists long before the football "feels like FIFA":

1. **Playable core** — movement, pass, shoot, tackle, switching, goalkeeper AI, rules for goals, out of play, kick-off, basic fouls. Capsule-and-stick visuals are acceptable.
2. **Complete rules** — offside, cards, set pieces, substitutions, extra time, penalties.
3. **Feel** — skill moves, driven/lofted variants, first-time play, jockeying, animation quality, camera polish.
4. **Presentation** — stadiums, kits, broadcast UI, replays.

The season ledger, scheduling, import and cartridge systems do not depend on stages 3 and 4 and can proceed in parallel.

## 5. Design assumptions adopted

Where the document is silent, the architecture adopts these assumptions. Each should be confirmed by the group.

- A1. One world host per friend group, self-hosted on the DigitalOcean droplet or a Raspberry Pi 5; clients are participants' PCs, optional Raspberry Pi 5 consoles, and Chromebooks on a best-effort basis.
- A2. ≤ 20 participants, ≤ 4 concurrent official matches, one season active at a time per world.
- A3. Official matches require host connectivity; same-room play uses one client talking to the host.
- A4. The trust model is "friends", but validation is still done properly because deterministic replay makes it cheap.
- A5. Real club and player data are for private use only.
- A6. Controllers connect by USB cable (no wireless in v1); the cartridge is a USB mass-storage device.
- A7. The game client runs on Windows and Linux x86-64 and Linux arm64 (Raspberry Pi 5); macOS is nice-to-have; ChromeOS is reached through its Linux container.
