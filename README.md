# classicFutbol- a Parallel Season FC

Complete product vision, gameplay requirements, profile-cartridge system, and technical architecture

Parallel Season FC is a fully 3D, controller-driven football game inspired by the playing experience of classic FIFA titles from approximately 2009–2012, built around a shared season that you and your friends change by playing your clubs’ actual fixtures.

Each participant chooses a real club, receives a personally 3D-printed controller, and carries a removable USB profile cartridge containing their identity, team setup, career records, and saved progress.

When you create your profile, the game imports your club’s available football history up to that point. From then onward, your performances create its new sporting history. Real football continues supplying fixtures and squad changes, but your played matches determine your results, statistics, achievements, and standing in your friends’ football world.

Real football provides the starting world. You and your friends play what happens next.

⸻

1. The experience at the centre of the game

You represent Manchester United. Your friend represents Arsenal.

When Manchester United has a scheduled fixture against an unrepresented club, you play against the computer. When Manchester United and Arsenal meet, you and your friend play each other.

You can play online from your respective homes or meet in person with your individual controllers and profile cartridges.

At the end of the match, the result becomes part of the shared season. It updates both clubs, their footballers’ statistics, the competition standings, and your head-to-head history.

Matches between clubs nobody represents continue using the corresponding real-world results.

The season therefore combines three sources of football activity:

Fixture	Source of the parallel-season result
Two represented clubs	Their participants play each other.
One represented club	That participant plays the computer-controlled opponent.
Neither club is represented	The system imports the corresponding real-world match result.

A participant who is temporarily unavailable still owns their club. Their absence must not silently convert a friend-versus-friend fixture into a computer match.

The purpose is not simply to play occasional friendlies using real team names. It is to take responsibility for a club throughout its football season, with every match contributing to a persistent world.

⸻

2. The personal hardware kit

Every participant receives a personal kit comprising a compact, PS1-inspired controller and a removable USB profile cartridge in a small, Game Boy–inspired shell.

The controller

The controller is the participant’s physical equipment for playing the game. Its printed enclosure can carry personal colours, a name, a number, or other identifying design details.

It provides the controls needed for the complete football experience, including analog movement, player switching, passing, shooting, defending, and menu navigation.

The profile cartridge

The cartridge stores the participant’s profile and meaningful saved career information. It is not the game software, and it is not merely an account identifier.

The cartridge should attach securely to the controller for storage and transport. When it is time to play, the participant removes it and inserts it into a profile-loading slot.

The slot may be part of the controller system or a separate profile-loading unit, but the experience must remain consistent:

Insert cartridge → recognise participant → load career → synchronise → identify available fixture → play.

Welcoming a new participant

Adding someone to the group includes printing and assembling their controller and cartridge shell, testing their controls, creating their profile, and introducing their club into the shared season.

This physical onboarding is part of the game’s identity. Joining the league means receiving your own equipment and becoming part of the group’s football history.

A damaged controller must not destroy the participant’s career. The profile remains usable with a replacement or compatible borrowed controller.

⸻

3. Personal profiles within one shared football world

The system must distinguish the person, their club assignment, and the shared season.

Personal identity

The profile identifies the participant and stores their display name, presentation preferences, controller mappings, camera settings, and career records.

Club assignment

The profile records which club the participant represents, when they took over, and which shared football world that assignment belongs to.

Within the same world, a club should have one active representative unless the group explicitly enables shared control.

Shared world

The world contains the agreed competition history: participants, club assignments, fixtures, accepted results, statistics, standings, draws, and trophies.

Your cartridge and your friend’s cartridge therefore do not contain unrelated seasons. They contain personal saves linked to the same season.

World identifiers must prevent records from unrelated groups being mixed accidentally. A result in one group must not change another group’s Manchester United.

Club ownership changes also need an effective date. A new representative inherits the club’s established world history rather than replacing it.

⸻

4. Profile creation and the takeover point

Creating a profile and selecting a club establishes a takeover point: the boundary between inherited football history and the history the participant creates.

The initial import

The system loads the available information needed to establish the club and its playable squad.

Information area	Initial profile content
Club identity	Name, country, league, colours, stadium information, and other supported club details.
Club history	Previous seasons, historical results, honours, and relevant records.
Current season	Completed fixtures, accepted results, competition participation, standings, and accumulated statistics.
Current squad	Players, positions, shirt numbers, membership, and available eligibility information.
Footballer profiles	Biographical information, preferred foot, physical characteristics, career history, and available performance data.
Upcoming football	Scheduled fixtures, competition stages, known opponents, and relevant dates.
Playable team model	Initial footballer attributes, positions, team setup, and associated game content.

The interface must report missing information honestly. Unavailable historical assists, for example, must not appear as confirmed zero assists.

Imported information needs its source, applicable period, retrieval time, and coverage status. Different external identifiers for the same club or footballer must map to one internal identity.

Starting partway through a season

A participant joining midseason inherits the club’s existing position at their takeover point.

However, the shared world takes precedence wherever it already contains accepted events.

If your Manchester United has already beaten an unrepresented Chelsea, a friend taking over Chelsea must inherit that defeat. Importing Chelsea’s real-world season cannot erase your result.

Historical imports fill in the world; they do not rewrite events your group has already established.

⸻

5. The rules governing history, statistics, and live data

The central data boundary must remain visible and enforceable.

Information	Behaviour after takeover
Historical records from before takeover	Remain part of the inherited background.
Results involving the represented club	Come from its accepted played matches.
Footballer match statistics	Come from events in those played matches.
Club points, form, and achievements	Are calculated from the parallel season.
Real transfers and loans	Continue updating squad membership.
Real fixture changes	Continue informing the schedule.
Later real-world goals, assists, and results for replaced fixtures	Do not overwrite or supplement the played records.
Matches between unrepresented clubs	Can contribute their corresponding real-world results and available events.

Inherited and earned records

The game should distinguish:

Before takeover · Under your control · Combined record

Suppose a striker has four league goals when you begin, and you score twice with him in your next league match.

His combined total becomes six. Two were earned under your control.

Subsequent real-world goals for that footballer do not get added to the fixtures you have replaced.

Trophies and club records follow the same principle. Historical honours remain inherited history; new parallel-season honours must be earned within your world.

Import events, not contradictory totals

Background real-world updates must be applied to identified fixtures.

Importing a provider’s latest season-wide total could inadvertently replace your results or count the real version of a fixture you already played. The system must instead calculate its statistics from the accepted history of the shared world.

When a played 3–1 match replaces an actual 0–0 match, its goals, scorers, assists, appearances, cards, and other recorded events supply that fixture’s contribution.

Missing background match details remain marked as incomplete rather than invented.

Statistics, form, and ability

These are related but distinct:

Statistics record what happened: goals, assists, appearances, saves, passing accuracy, and other match events.

Form describes recent performance within the parallel world.

Ability attributes govern how a footballer performs in the simulation, including pace, finishing, passing, positioning, stamina, and goalkeeping.

Initial attributes should be associated with an identifiable starting model. Any continuing form or development system must use documented rules and distinguish earned changes from imported information.

Real-world performance updates must not silently change a participant’s recorded achievements or override their gameplay-driven progression.

⸻

6. Live transfers and squad continuity

The actual club’s transfers, loans, departures, and registrations continue affecting the playable squad.

If the real club sells a footballer, that player leaves the available squad according to the transfer’s effective date and the fixture’s eligibility policy.

Their previous achievements remain intact.

A player who scored 12 goals for your Manchester United retains those goals in Manchester United’s historical record after leaving. If they join your friend’s club, their subsequent contributions belong to that club.

Transfers move squad membership, not historical credit.

New arrivals receive a playable profile without replacing any history already established for that footballer in the shared world.

Match snapshots

Every competitive match must retain a snapshot of its squads, attributes, eligibility, and rules.

A transfer update cannot remove a footballer during a match. A completed match must continue showing the actual lineup used.

Squad eligibility must use the fixture’s defined reference point rather than whichever participant happened to refresh their profile most recently.

Injuries and discipline

Cards and disciplinary consequences generated by parallel matches belong to the parallel season.

Any additional real-world availability restrictions, such as imported injuries, must be an explicit season setting. They must not silently replace gameplay-generated events or disciplinary records.

⸻

7. The fixture and competition engine

The system should automatically identify the competitions the selected club participates in and assemble one combined calendar.

That includes domestic leagues, domestic cups, the Champions League, Europa League, Conference League, and applicable Super Cups.

The relevant competition format and rules must be associated with the represented season rather than assumed to be identical across every tournament.

One fixture, one result

A fixture has one accepted outcome affecting both clubs.

If you play Manchester United against an unrepresented Liverpool, your match determines both teams’ result. Liverpool must not separately receive the real-world outcome for that same fixture.

Every accepted result needs an explicit source: played against a friend, played against the computer, imported, generated, or administratively resolved.

Independent standings

The game calculates its own standings from its accepted results.

Points, goal difference, goals scored, head-to-head records, and other tiebreakers must follow the competition’s rules.

The actual league table can appear as a comparison, but cannot overwrite the parallel table.

Cup progression

Played results determine who advances.

If you win a cup tie that your real club lost, your club remains in the parallel competition. The real-world winner cannot be reinserted automatically by the next fixture import.

Actual later-round fixtures may be reused only where their participants and conditions remain valid. Otherwise, the competition engine must form a valid parallel draw and schedule using the appropriate rules and available dates.

A resulting fixture with no real-world equivalent cannot borrow an unrelated real score. Represented teams play it; an unrepresented-versus-unrepresented fixture requires an identified generated-result policy.

Season completion and continuation

The system must calculate final positions, cup winners, individual awards, and qualification outcomes from the parallel record.

Completed seasons remain archived. Continuing into another season must not silently reset trophies or replace earned qualification with contradictory real-world outcomes.

Where the parallel world produces different competition entrants, the subsequent schedule must accommodate them consistently.

⸻

8. Match-day scheduling without requiring the real kickoff time

A participant must play their club’s fixture on its assigned playing day, but does not need to be awake for the actual kickoff.

The system stores two separate concepts:

Real kickoff: The actual football match’s scheduled time.

Playing window: The period during which the participant must play their version.

For example, an actual 3:00 a.m. kickoff can correspond to a fixture the participant plays at 7:30 p.m. that day.

Clear deadlines

The season needs a declared scheduling timezone, with local-time displays for participants.

The interface must show when the fixture opens, when it is due, and whether the deadline concerns starting or finishing the match.

The assigned match day must not shift unexpectedly because a participant travels or changes their device timezone.

Friend-versus-friend appointments

When two represented clubs meet, either participant can propose a time within the window. Both can accept, and the agreed appointment appears on their dashboards.

The system should recognise their scheduled fixture when both profiles are loaded, but require readiness confirmation before starting a competitive session.

Exceptions

Rules must cover postponements, no-shows, connection failures, rescheduling, and exceptional extensions.

A deadline failure must not silently substitute the actual result. Any administrative outcome needs a visible reason and history.

Result sources must be assigned in advance. Participants cannot wait for the actual score and choose whichever version benefits them.

Unfinished earlier fixtures must also remain visible where they affect eligibility, statistics, or later competition progression.

⸻

9. Full 3D football gameplay

The gameplay target is 11-a-side football with direct, real-time control, a familiar broadcast-style presentation, and the on-pitch feel of classic FIFA.

One participant controls their selected footballer while teammates act under team AI. The participant switches between teammates as play develops.

Movement and dribbling

The game needs analog movement, acceleration, deceleration, sprinting, changes of direction, close control, shielding, first touches, and ball-carrying at different speeds.

The footballer’s orientation, movement, attributes, fatigue, and pressure should affect how actions unfold.

Receiving a pass, turning with the ball, protecting possession, and running into space should be distinct interactions.

Skill moves, feints, and changes of pace should let participants create space through controller input.

Passing and attacking

Passing should include short passes, driven passes, long passes, through balls, lofted through balls, crosses, cutbacks, and first-time combinations.

The system needs directional input, power control, receiver selection, and identifiable assistance settings.

Participants should be able to play a give-and-go, trigger supporting runs, switch play, build through midfield, and attack with width.

Shooting

Finishing should include standard shots, power shots, placed shots, chips, headers, volleys, and first-time attempts.

Aim, power, timing, body position, preferred foot, balance, defensive pressure, and footballer attributes should contribute to the outcome.

Shots can be saved, blocked, deflected, rebound, strike the goal frame, or score.

Defending

Defending needs positioning, jockeying, interceptions, standing tackles, sliding tackles, shoulder challenges, clearances, and teammate pressure.

Participants must be able to defend by reading play and selecting appropriate actions.

Challenges should visibly communicate possession changes and foul decisions.

Goalkeeping

Goalkeepers need positioning, reactions, diving, catching, parrying, claiming crosses, rushing out, and distribution.

The participant needs relevant goalkeeper commands and control during appropriate situations, including penalties and restarts.

Ball simulation

The ball requires three-dimensional flight, spin, bounce, rolling, collisions, and deflections.

Dribbling must coexist with loose balls, heavy touches, rebounds, contested possession, and interceptions. The ball must not behave as permanently attached to a footballer.

Player switching

The game needs button-based and directional switching, with configurable automatic-switching behaviour.

Controlled-player indicators must remain clear. Transfers between human and AI control should preserve ongoing movement and positional responsibilities.

Team AI

Teammates must provide attacking support, maintain spacing, recognise passing opportunities, make runs, mark opponents, track runners, defend space, and react to transitions.

Computer opponents should use the same football rules and an identifiable difficulty profile.

Difficulty settings and assistance policies must remain consistent across official fixtures under the group’s competitive rules.

⸻

10. Team preparation, rules, and presentation

Squad management

Before kickoff, participants can select the starting eleven and substitutes, choose a formation, assign positions, and inspect attributes and availability.

They can assign captaincy and set-piece responsibilities, save tactical setups, and define relevant team instructions.

During the match, permitted controls support substitutions, formation changes, and tactical adjustments.

Football rules

The rules system needs kickoffs, throw-ins, corners, goal kicks, offsides, fouls, advantage, free kicks, penalties, cards, substitutions, halftime, added time, and full time.

It also needs extra time, penalty shootouts, aggregate scores, and other competition-dependent rules.

Match events must feed the statistics and disciplinary systems rather than exist only as animations.

Visual presentation

The game needs a 3D pitch, footballers, ball, goals, kits, stadium environment, lighting, and animated actions.

The broadcast-style camera should expose useful height, angle, and zoom settings and keep play readable during open play and set pieces.

Animations should correspond to the actual action: receiving, striking, tackling, heading, saving, and celebrating.

Interface and audio

The match interface needs the score, clock, team identity, controlled-player indicators, player names, stamina information, and a tactical radar.

It must clearly communicate substitutions, cards, injuries, and stoppages.

Audio should cover ball contact, whistles, crowd responses, and relevant events. Replays and highlights should connect to the match’s recorded events without preventing timely continuation.

⸻

11. Controller behaviour and firmware requirements

The custom controller must expose enough inputs for the complete gameplay system while retaining the compact physical design.

The functional control set includes two analog sticks, directional navigation, face buttons, shoulder and trigger controls, and menu controls.

Mappings must be configurable rather than permanently tied to one layout.

Setup and calibration

A controller screen should provide device detection, button testing, stick calibration, dead-zone adjustment, sensitivity, axis inversion, and saved mappings.

The game must clearly associate each controller with its loaded participant profile.

Disconnecting and reconnecting a controller should restore the appropriate assignment rather than give the participant control of the wrong team.

Firmware

The firmware needs reliable simultaneous input handling, button debouncing, calibrated analog values, saved configuration, and recoverable updates.

Battery reporting and vibration feedback should operate where the hardware includes them.

Cartridge integration

The electrical design must support controller input and profile operations together.

A cartridge dock used for carrying the device is not automatically a functioning data reader. The complete hardware design must establish how the cartridge is read and written without interrupting gameplay input.

Profile loading, saving, and controller operation must be tested as one integrated experience.

⸻

12. Cartridge contents, synchronisation, and recovery

The cartridge must contain a usable local representation of the participant’s career.

Stored information	Purpose
Profile identity	Recognise the participant and their presentation settings.
World and club membership	Load the correct shared season and club assignment.
Takeover record	Preserve the historical baseline and starting point.
Career records	Carry completed fixtures, achievements, rivalry history, and statistics.
Current team state	Retain relevant squad information, lineups, formations, and tactics.
Personal settings	Restore controller mapping, camera preferences, and permitted gameplay settings.
Synchronisation metadata	Identify which accepted events and updates the cartridge contains.
Save-format information	Support validation and compatible upgrades.

Large common assets—such as stadium models and animations—belong with the game installation.

Shared authority and portable saves

The shared host maintains the agreed competition record. The cartridge carries the participant’s synchronised save.

An older cartridge must not reverse newer accepted results. Copying a cartridge must not create a second official result or allow two participants to claim the same identity simultaneously.

Synchronisation should compare known revisions and accepted events rather than blindly replacing data based on file modification time.

Safe saving

The game must preserve the last valid save if a write is interrupted.

Visible states should include Detected, Loading, Synchronising, Ready, Saving, and Safe to remove.

If a cartridge cannot be updated after a completed match, the accepted result remains on the host and is synchronised later.

Recovery

Lost or damaged cartridges must be replaceable from the shared record and backups.

Identity recovery should allow a lost device’s access to be withdrawn without deleting its owner’s football history.

Local profile browsing and practice should remain possible without an external internet connection. Official offline results require an explicit policy for authorisation and reconciliation; they cannot silently overwrite conflicting shared results when reconnected.

⸻

13. Same-room and online multiplayer

Both ways of playing should feed the same career system.

Same-room play

Two participants insert their cartridges, connect their controllers, and load their profiles on the same game installation.

The game identifies their shared world and any eligible head-to-head fixture. Each participant confirms their club, controller assignment, squad, and readiness.

They play on the same display, each controlling their own team.

Online play

Participants load their profiles on separate clients and join the fixture’s match session.

The lobby confirms identities, clubs, kits, squad snapshots, rules, gameplay versions, and connection status.

Both clients participate in one agreed match state.

Session integrity

The match system must synchronise the clock, score, footballers, ball, and football events while maintaining responsive control.

It needs defined pause limits, reconnection behaviour, abandonment handling, and recovery from temporary interruptions.

The result must come from the validated match session, not an arbitrary score submitted by a client or found on a cartridge.

Submitting the same completion message repeatedly must still produce only one accepted result.

A friendly rematch must never overwrite the competitive fixture.

⸻

14. The player-facing pages

The interface should make the season easy to understand without requiring participants to inspect technical records.

Page	Main functions
Profile station	Detect cartridges, recognise participants, show synchronisation status, and assign controllers.
Create or join world	Accept invitations, establish profiles, claim available clubs, and inspect group rules.
Club selection	Browse clubs and preview squads, history, competitions, and ownership status.
Club dashboard	Show today’s fixture, upcoming matches, recent results, squad changes, and friends’ activity.
Team management	Select lineups, formations, substitutes, tactics, and set-piece assignments.
Fixture centre	View playing windows, arrange appointments, inspect opponents, and enter lobbies.
Competitions	Display parallel tables, brackets, progression, and real-world comparisons.
Footballer profiles	Separate inherited records from parallel performances, form, and attributes.
Career and rivalries	Display honours, records, head-to-head history, milestones, and season archives.
Controller settings	Test hardware, calibrate inputs, and save mappings.
Administration	Resolve scheduling problems, inspect data failures, manage membership, and correct records.

Real-world and parallel information must have distinct labels. A participant should never have to guess which version of a statistic they are viewing.

⸻

15. Technical architecture

The architecture needs clear responsibilities and interfaces so external data, gameplay, hardware, and competition records do not become inseparable.

Component	Responsibility
Controller interface	Translate hardware inputs into consistent game actions and apply calibration and mappings.
Profile-cartridge manager	Detect media, validate saves, load identities, synchronise records, and perform safe writes.
Game client	Present menus, render the 3D match, manage cameras and audio, and capture participant inputs.
Football simulation	Execute movement, ball behaviour, football actions, AI, tactics, and rules.
Match-session service	Authorise participants, establish sessions, synchronise play, handle recovery, and validate completion.
Football-data adapters	Retrieve external information, report coverage, track provenance, and normalise identifiers.
Club and player catalogue	Maintain stable identities, profiles, histories, and squad membership.
Squad and attribute system	Build playable team models and preserve fixture-specific snapshots.
Season and competition engine	Manage entries, fixtures, schedules, draws, progression, and qualification.
Results and statistics system	Accept outcomes, record events, calculate statistics and tables, and process corrections.
Community and administration	Manage accounts, invitations, ownership, appointments, notifications, and disputes.
Hosting and operations	Provide storage, deployment, backups, monitoring, updates, and recovery.

Core information flow

The competition engine supplies an eligible fixture.

The squad system supplies its team snapshots and rules.

The match-session system authorises the participants and runs the official session.

The completed session submits its validated outcome and recorded events.

The results system updates statistics, standings, and progression.

The profile system synchronises the resulting career changes to the participants’ cartridges.

External data updates enter through controlled integration points. They must not directly mutate an active match or overwrite accepted results.

Core records

The persistent model needs stable records for participants, worlds, club assignments, footballers, squad memberships, takeover snapshots, competitions, fixtures, match sessions, events, accepted results, and cartridge revisions.

Real-world fixture identifiers and parallel fixture identifiers must remain linked but distinct.

Each component should expose clear contracts, making it possible to change a data adapter or improve a gameplay system without rewriting the entire product.

⸻

16. Hosting, configuration, and administration

The deployment must support your self-hosted environment, including the Raspberry Pi hosting target and your other servers.

The player-facing 3D runtime, match-session hosting, data synchronisation, and persistent season storage should be separately identifiable responsibilities.

Actual deployment testing must measure frame rate, input responsiveness, network latency, simulation timing, memory usage, simultaneous sessions, and recovery behaviour.

Season configuration

Configuration needs to cover match duration, difficulty, assistance policies, scheduling timezone, deadlines, squad eligibility, roster updates, real-world availability constraints, competition rules, and administrative exceptions.

Settings must be visible and versioned. Competitive rules cannot change silently during a fixture.

Administrative controls

The host needs to inspect failed data updates, resolve identity mappings, reschedule fixtures, manage club ownership, review disputed results, and repair competition state.

Corrections must preserve the original record, the reason, and who authorised the change.

Applying a correction should rebuild affected statistics and standings without counting both versions.

Protection and recovery

Backups must preserve profiles, assignments, configuration, snapshots, fixtures, results, events, and archives.

Cartridges should carry structured save data, not executable instructions. Profile loading must validate that data, and the cartridge must not contain unrestricted server-administration credentials.

External data and visual content should have identifiable sources and permission records.

⸻

17. The social season and its lasting history

The game should preserve the personal meaning of playing together.

Each participant gets a trophy cabinet, season summaries, rivalry records, memorable match results, and records achieved under their control.

The group can compare parallel outcomes with real football without confusing the two.

For friends whose clubs rarely meet in official competitions, separately identified friendlies and community tournaments can provide additional opportunities to play. Their results must remain separate from the official club calendar unless deliberately included in a distinct competition.

At season end, the game should show where each club finished, which trophies were won, standout footballer performances, head-to-head records, and the difference between the inherited position and the participant’s final result.

Those records remain available in later seasons.

⸻

18. A complete example session

The following is illustrative, not a live fixture.

Manchester United and Arsenal are scheduled to meet in your shared season.

The actual kickoff is early in the morning. Your playing window lasts through the assigned day, and you agree to meet at 7:30 p.m.

You arrive with your printed controller and attached cartridge. Your friend brings his.

You insert both cartridges. The game recognises you, loads your clubs and records, synchronises accepted updates, and identifies the official fixture.

You confirm controller assignments, choose your lineups, and ready up.

You play the 3D match and Manchester United wins 3–2.

The game records the goals, assists, appearances, cards, substitutions, and other supported events. It updates both clubs, the league table, and your rivalry history.

The host accepts the result once. Both cartridges receive their new career state.

The interface confirms that saving is complete. You remove the cartridges, attach them to your controllers, and take your updated careers home.

Your next fixture is against an unrepresented club. You play the computer on that fixture’s assigned day, and the season continues.

⸻

19. Acceptance criteria for the complete experience

The product should demonstrate the following end-to-end behaviours:

Area	Required demonstration
Personal kit	A custom controller works with a removable cartridge that carries meaningful saved profile data.
Profile portability	Inserting the cartridge on another compatible installation restores the participant’s identity, team setup, and career records.
Historical takeover	Club selection establishes a traceable baseline without overwriting existing shared-world events.
Gameplay-owned records	Played results and statistics replace their real-world counterparts without double counting.
Live squad changes	Transfers update membership while preserving completed-match snapshots and historical achievements.
Full football gameplay	Two participants play a complete 3D, controller-driven, 11-a-side match.
Both multiplayer settings	Official fixtures work online and in person with correctly assigned profiles and controllers.
Day-based scheduling	Participants can play during the assigned day without matching the real kickoff time.
Background football	Unrepresented fixtures import the correct results without overriding played fixtures.
Competition integrity	Tables and cup progression remain coherent after the parallel season diverges from reality.
Reliable saving	Completion is accepted once, cartridges synchronise safely, and lost media can be recovered.
New participants	A friend can join with a new kit and inherit their club’s existing world history without restarting the season.
Persistence	Careers and competition records survive restarts, updates, and season completion.

Parallel Season FC turns your controller into personal equipment, your cartridge into a portable football career, and your club’s real calendar into a season of matches you and your friends actually play.