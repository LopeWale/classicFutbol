"""Seed a ready-to-play Parallel Season FC 5v5 world with inherited baseline history and open fixtures."""

from __future__ import annotations

from psfc_host.catalogue.clubs import ClubCatalogue
from psfc_host.ingest.proposal_gate import ProposalGate
from psfc_host.ledger.store import WorldLedgerStore

DEFAULT_WORLD_ID = "world-prem-2026"


def ensure_demo_world(store: WorldLedgerStore, catalogue: ClubCatalogue) -> str:
    worlds = store.list_worlds()
    if any(w["id"] == DEFAULT_WORLD_ID for w in worlds):
        return DEFAULT_WORLD_ID

    gate = ProposalGate(store, catalogue)

    store.create_world(
        world_id=DEFAULT_WORLD_ID,
        name="Friends Premier Parallel Season",
        competition_name="Premier League (5v5 Parallel Staging)",
        season_label="2026/27",
        season_timezone="Europe/London",
        default_match_format="5v5",
        caused_by="commissioner",
    )

    # Create Round 1-4 fixtures (initially imported before club takeovers at Round 3)
    fixtures_spec = [
        # Round 1 (Pre-takeover inherited real history)
        ("fix-r1-mun-che", 1, "man-utd", "chelsea", "2026-08-16T14:00:00Z", 2, 1),
        ("fix-r1-ars-liv", 1, "arsenal", "liverpool", "2026-08-16T16:30:00Z", 2, 2),
        # Round 2 (Pre-takeover inherited real history)
        ("fix-r2-liv-mun", 2, "liverpool", "man-utd", "2026-08-23T15:00:00Z", 1, 1),
        ("fix-r2-che-ars", 2, "chelsea", "arsenal", "2026-08-23T17:30:00Z", 1, 3),
        # Round 3 (Takeover active from Round 3 onward!)
        ("fix-r3-mun-ars", 3, "man-utd", "arsenal", "2026-08-30T16:30:00Z", 1, 2),
        ("fix-r3-liv-che", 3, "liverpool", "chelsea", "2026-08-30T14:00:00Z", 2, 0),
        # Round 4 (Upcoming open Parallel Season fixtures)
        ("fix-r4-mun-liv", 4, "man-utd", "liverpool", "2026-09-13T15:00:00Z", 0, 2),
        ("fix-r4-ars-che", 4, "arsenal", "chelsea", "2026-09-13T17:30:00Z", 2, 1),
    ]

    for fid, rnd, home, away, ko, rhg, rag in fixtures_spec:
        store.create_fixture(
            fixture_id=fid,
            world_id=DEFAULT_WORLD_ID,
            round_no=rnd,
            home_club_id=home,
            away_club_id=away,
            kickoff_utc=ko,
            window_open_utc="2026-08-01T00:00:00Z",
            window_close_utc="2026-12-31T23:59:59Z",
            result_source="imported",
            state="open",
            real_home_goals=rhg,
            real_away_goals=rag,
        )

    # Ingest Round 1 & Round 2 real-world baseline through the Proposal Gate
    gate.submit_fixture_result_import(
        world_id=DEFAULT_WORLD_ID,
        fixture_id="fix-r1-mun-che",
        home_goals=2,
        away_goals=1,
        events=[
            {"event_type": "Goal", "footballer_id": "mun-fwd-1", "assist_footballer_id": "mun-mid-2", "minute": 24},
            {"event_type": "Goal", "footballer_id": "che-mid-2", "assist_footballer_id": "che-fwd-1", "minute": 58},
            {"event_type": "Goal", "footballer_id": "mun-mid-2", "assist_footballer_id": "mun-mid-1", "minute": 81},
            {"event_type": "Save", "footballer_id": "mun-gk-1", "minute": 88},
        ],
    )
    gate.submit_fixture_result_import(
        world_id=DEFAULT_WORLD_ID,
        fixture_id="fix-r1-ars-liv",
        home_goals=2,
        away_goals=2,
        events=[
            {"event_type": "Goal", "footballer_id": "ars-fwd-1", "assist_footballer_id": "ars-mid-2", "minute": 14},
            {"event_type": "Goal", "footballer_id": "liv-fwd-1", "assist_footballer_id": "liv-mid-1", "minute": 39},
            {"event_type": "Goal", "footballer_id": "ars-mid-2", "assist_footballer_id": "ars-mid-1", "minute": 63},
            {"event_type": "Goal", "footballer_id": "liv-fwd-1", "assist_footballer_id": "liv-mid-2", "minute": 79},
            {"event_type": "Save", "footballer_id": "ars-gk-1", "minute": 90},
        ],
    )
    gate.submit_fixture_result_import(
        world_id=DEFAULT_WORLD_ID,
        fixture_id="fix-r2-liv-mun",
        home_goals=1,
        away_goals=1,
        events=[
            {"event_type": "Goal", "footballer_id": "liv-mid-2", "assist_footballer_id": "liv-fwd-1", "minute": 33},
            {"event_type": "Goal", "footballer_id": "mun-fwd-1", "assist_footballer_id": "mun-mid-2", "minute": 71},
            {"event_type": "Save", "footballer_id": "liv-gk-1", "minute": 84},
        ],
    )
    gate.submit_fixture_result_import(
        world_id=DEFAULT_WORLD_ID,
        fixture_id="fix-r2-che-ars",
        home_goals=1,
        away_goals=3,
        events=[
            {"event_type": "Goal", "footballer_id": "ars-fwd-1", "assist_footballer_id": "ars-mid-2", "minute": 19},
            {"event_type": "Goal", "footballer_id": "che-mid-2", "assist_footballer_id": None, "minute": 44},
            {"event_type": "Goal", "footballer_id": "ars-mid-1", "assist_footballer_id": "ars-fwd-1", "minute": 67},
            {"event_type": "Goal", "footballer_id": "ars-fwd-1", "assist_footballer_id": "ars-mid-2", "minute": 85},
        ],
    )

    # Register two human participants taking over Manchester United and Arsenal at Round 3!
    store.register_participant("part-alex", "Alex (Cartridge #01)", DEFAULT_WORLD_ID)
    store.register_participant("part-sam", "Sam (Cartridge #02)", DEFAULT_WORLD_ID)

    store.assign_club(
        assignment_id="asgn-mun-alex",
        world_id=DEFAULT_WORLD_ID,
        club_id="man-utd",
        participant_id="part-alex",
        effective_from_round=3,
        baseline_summary={"inherited_played": 2, "inherited_points": 4},
    )
    store.assign_club(
        assignment_id="asgn-ars-sam",
        world_id=DEFAULT_WORLD_ID,
        club_id="arsenal",
        participant_id="part-sam",
        effective_from_round=3,
        baseline_summary={"inherited_played": 2, "inherited_points": 4},
    )

    # Unrepresented Round 3 match (Liverpool vs Chelsea) continues importing from real football!
    gate.submit_fixture_result_import(
        world_id=DEFAULT_WORLD_ID,
        fixture_id="fix-r3-liv-che",
        home_goals=2,
        away_goals=0,
        events=[
            {"event_type": "Goal", "footballer_id": "liv-fwd-1", "assist_footballer_id": "liv-def-1", "minute": 29},
            {"event_type": "Goal", "footballer_id": "liv-mid-1", "assist_footballer_id": "liv-fwd-1", "minute": 74},
            {"event_type": "Save", "footballer_id": "liv-gk-1", "minute": 62},
        ],
    )

    return DEFAULT_WORLD_ID
