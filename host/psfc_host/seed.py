"""Seed the live 2026–27 Premier League Parallel Season FC world through Matchweek 11 (21 Nov 2026)."""

from __future__ import annotations

from datetime import datetime, timedelta, timezone
from typing import Any

from psfc_host.catalogue.clubs import ClubCatalogue
from psfc_host.community.cartridge import CartridgeSyncService
from psfc_host.ingest.proposal_gate import ProposalGate
from psfc_host.ledger.store import WorldLedgerStore

DEFAULT_WORLD_ID = "world-prem-2026"
DEFAULT_LEAGUE_ID = "comp-prem-2026"
DEFAULT_CUP_ID = "comp-facup-2026"

_CLUB_SLUG: dict[str, str] = {
    "arsenal": "ars",
    "brighton": "bha",
    "man-city": "mci",
    "bournemouth": "bou",
    "liverpool": "liv",
    "leeds": "lee",
    "everton": "eve",
    "chelsea": "che",
    "tottenham": "tot",
    "brentford": "bre",
    "aston-villa": "avl",
    "newcastle": "new",
    "coventry": "cov",
    "hull": "hul",
    "fulham": "ful",
    "ipswich": "ips",
    "nottm-forest": "nfo",
    "sunderland": "sun",
    "man-utd": "mun",
    "crystal-palace": "cry",
}

# Official 2026–27 Premier League completed matches through Saturday 21 November 2026 (107 matches, 293 goals).
COMPLETED_2026_27_RESULTS: list[tuple[int, str, str, str, int, int]] = [
    # Matchweek 1 (21–24 Aug 2026)
    (1, "arsenal", "coventry", "2026-08-21T19:00:00Z", 3, 0),
    (1, "hull", "man-utd", "2026-08-22T11:30:00Z", 2, 0),
    (1, "everton", "crystal-palace", "2026-08-22T14:00:00Z", 2, 0),
    (1, "ipswich", "sunderland", "2026-08-22T14:00:00Z", 2, 1),
    (1, "nottm-forest", "leeds", "2026-08-22T14:00:00Z", 0, 1),
    (1, "brentford", "tottenham", "2026-08-22T16:30:00Z", 3, 0),
    (1, "brighton", "aston-villa", "2026-08-23T13:00:00Z", 4, 0),
    (1, "man-city", "bournemouth", "2026-08-23T15:30:00Z", 2, 1),
    (1, "newcastle", "liverpool", "2026-08-23T18:00:00Z", 2, 2),
    (1, "fulham", "chelsea", "2026-08-24T19:00:00Z", 2, 3),
    # Matchweek 2 (28–31 Aug 2026)
    (2, "crystal-palace", "man-city", "2026-08-28T19:00:00Z", 1, 4),
    (2, "liverpool", "nottm-forest", "2026-08-29T11:30:00Z", 2, 2),
    (2, "bournemouth", "everton", "2026-08-29T14:00:00Z", 1, 1),
    (2, "coventry", "hull", "2026-08-29T14:00:00Z", 0, 1),
    (2, "tottenham", "newcastle", "2026-08-29T14:00:00Z", 0, 2),
    (2, "leeds", "brentford", "2026-08-29T16:30:00Z", 1, 1),
    (2, "sunderland", "fulham", "2026-08-30T13:00:00Z", 1, 0),
    (2, "chelsea", "brighton", "2026-08-30T15:30:00Z", 4, 3),
    (2, "man-utd", "ipswich", "2026-08-30T18:00:00Z", 5, 2),
    (2, "aston-villa", "arsenal", "2026-08-31T19:00:00Z", 0, 1),
    # Matchweek 3 (4–6 Sep 2026)
    (3, "ipswich", "liverpool", "2026-09-04T19:00:00Z", 0, 2),
    (3, "newcastle", "bournemouth", "2026-09-05T11:30:00Z", 2, 2),
    (3, "brentford", "sunderland", "2026-09-05T14:00:00Z", 1, 1),
    (3, "brighton", "leeds", "2026-09-05T14:00:00Z", 1, 1),
    (3, "fulham", "crystal-palace", "2026-09-05T14:00:00Z", 2, 3),
    (3, "man-city", "coventry", "2026-09-05T14:00:00Z", 1, 0),
    (3, "nottm-forest", "tottenham", "2026-09-05T16:30:00Z", 0, 0),
    (3, "hull", "aston-villa", "2026-09-06T13:00:00Z", 0, 0),
    (3, "everton", "man-utd", "2026-09-06T15:30:00Z", 2, 2),
    (3, "arsenal", "chelsea", "2026-09-06T18:00:00Z", 2, 1),
    # Matchweek 4 (12–14 Sep 2026)
    (4, "aston-villa", "nottm-forest", "2026-09-12T11:30:00Z", 1, 2),
    (4, "bournemouth", "brentford", "2026-09-12T14:00:00Z", 2, 2),
    (4, "chelsea", "hull", "2026-09-12T14:00:00Z", 2, 2),
    (4, "crystal-palace", "ipswich", "2026-09-12T14:00:00Z", 2, 3),
    (4, "liverpool", "fulham", "2026-09-12T14:00:00Z", 0, 0),
    (4, "tottenham", "everton", "2026-09-12T16:30:00Z", 0, 0),
    (4, "sunderland", "arsenal", "2026-09-13T13:00:00Z", 0, 2),
    (4, "coventry", "brighton", "2026-09-13T15:30:00Z", 0, 5),
    (4, "man-utd", "man-city", "2026-09-13T16:30:00Z", 0, 1),
    (4, "leeds", "newcastle", "2026-09-14T19:00:00Z", 4, 1),
    # Matchweek 5 (18–20 Sep 2026)
    (5, "brentford", "chelsea", "2026-09-18T19:00:00Z", 3, 0),
    (5, "tottenham", "aston-villa", "2026-09-19T11:30:00Z", 2, 3),
    (5, "brighton", "arsenal", "2026-09-19T14:00:00Z", 3, 0),
    (5, "everton", "ipswich", "2026-09-19T14:00:00Z", 1, 0),
    (5, "newcastle", "hull", "2026-09-19T14:00:00Z", 2, 1),
    (5, "nottm-forest", "coventry", "2026-09-19T14:00:00Z", 0, 1),
    (5, "bournemouth", "liverpool", "2026-09-19T16:30:00Z", 0, 1),
    (5, "leeds", "crystal-palace", "2026-09-20T13:00:00Z", 0, 0),
    (5, "man-city", "sunderland", "2026-09-20T15:30:00Z", 5, 3),
    (5, "fulham", "man-utd", "2026-09-20T16:30:00Z", 1, 1),
    # Matchweek 6 (10–12 Oct 2026)
    (6, "arsenal", "leeds", "2026-10-10T11:30:00Z", 2, 1),
    (6, "aston-villa", "brentford", "2026-10-10T14:00:00Z", 2, 2),
    (6, "chelsea", "bournemouth", "2026-10-10T14:00:00Z", 5, 1),
    (6, "ipswich", "fulham", "2026-10-10T14:00:00Z", 2, 1),
    (6, "sunderland", "brighton", "2026-10-10T14:00:00Z", 0, 2),
    (6, "man-utd", "tottenham", "2026-10-10T16:30:00Z", 1, 1),
    (6, "crystal-palace", "nottm-forest", "2026-10-11T13:00:00Z", 3, 1),
    (6, "hull", "everton", "2026-10-11T15:30:00Z", 2, 2),
    (6, "liverpool", "man-city", "2026-10-11T16:30:00Z", 0, 3),
    (6, "coventry", "newcastle", "2026-10-12T19:00:00Z", 4, 1),
    # Matchweek 7 (17–19 Oct 2026)
    (7, "everton", "chelsea", "2026-10-17T11:30:00Z", 0, 2),
    (7, "brentford", "liverpool", "2026-10-17T14:00:00Z", 1, 2),
    (7, "fulham", "hull", "2026-10-17T14:00:00Z", 1, 0),
    (7, "man-city", "ipswich", "2026-10-17T14:00:00Z", 3, 2),
    (7, "newcastle", "aston-villa", "2026-10-17T14:00:00Z", 0, 1),
    (7, "bournemouth", "sunderland", "2026-10-17T16:30:00Z", 2, 0),
    (7, "brighton", "crystal-palace", "2026-10-18T13:00:00Z", 5, 0),
    (7, "leeds", "man-utd", "2026-10-18T15:30:00Z", 1, 1),
    (7, "nottm-forest", "arsenal", "2026-10-18T16:30:00Z", 0, 1),
    (7, "tottenham", "coventry", "2026-10-19T19:00:00Z", 3, 1),
    # Matchweek 8 (23–25 Oct 2026)
    (8, "ipswich", "nottm-forest", "2026-10-23T19:00:00Z", 1, 1),
    (8, "aston-villa", "man-city", "2026-10-24T11:30:00Z", 2, 1),
    (8, "arsenal", "everton", "2026-10-24T14:00:00Z", 2, 0),
    (8, "coventry", "fulham", "2026-10-24T14:00:00Z", 0, 1),
    (8, "chelsea", "tottenham", "2026-10-24T14:00:00Z", 0, 2),
    (8, "crystal-palace", "newcastle", "2026-10-24T14:00:00Z", 0, 2),
    (8, "hull", "brentford", "2026-10-24T16:30:00Z", 1, 1),
    (8, "liverpool", "brighton", "2026-10-25T14:00:00Z", 2, 0),
    (8, "man-utd", "bournemouth", "2026-10-25T14:00:00Z", 0, 3),
    (8, "sunderland", "leeds", "2026-10-25T16:30:00Z", 0, 0),
    # Matchweek 9 (31 Oct – 2 Nov 2026)
    (9, "chelsea", "man-utd", "2026-10-31T12:30:00Z", 2, 0),
    (9, "bournemouth", "leeds", "2026-10-31T15:00:00Z", 2, 0),
    (9, "brentford", "nottm-forest", "2026-10-31T15:00:00Z", 2, 0),
    (9, "coventry", "sunderland", "2026-10-31T15:00:00Z", 3, 1),
    (9, "hull", "ipswich", "2026-10-31T15:00:00Z", 2, 2),
    (9, "man-city", "brighton", "2026-10-31T17:30:00Z", 0, 1),
    (9, "tottenham", "crystal-palace", "2026-11-01T14:00:00Z", 2, 0),
    (9, "aston-villa", "fulham", "2026-11-01T14:00:00Z", 1, 1),
    (9, "liverpool", "arsenal", "2026-11-01T16:30:00Z", 0, 1),
    (9, "newcastle", "everton", "2026-11-02T20:00:00Z", 0, 1),
    # Matchweek 10 (6–8 Nov 2026)
    (10, "everton", "coventry", "2026-11-06T20:00:00Z", 3, 1),
    (10, "leeds", "tottenham", "2026-11-07T12:30:00Z", 1, 0),
    (10, "arsenal", "hull", "2026-11-07T15:00:00Z", 1, 0),
    (10, "fulham", "newcastle", "2026-11-07T15:00:00Z", 1, 1),
    (10, "nottm-forest", "man-city", "2026-11-07T15:00:00Z", 3, 1),
    (10, "brighton", "brentford", "2026-11-07T17:30:00Z", 2, 0),
    (10, "crystal-palace", "liverpool", "2026-11-08T14:00:00Z", 1, 3),
    (10, "ipswich", "bournemouth", "2026-11-08T14:00:00Z", 1, 2),
    (10, "sunderland", "chelsea", "2026-11-08T14:00:00Z", 1, 0),
    (10, "man-utd", "aston-villa", "2026-11-08T16:30:00Z", 4, 4),
    # Matchweek 11 — Completed Saturday 21 November 2026 fixtures
    (11, "man-city", "fulham", "2026-11-21T12:30:00Z", 1, 1),
    (11, "aston-villa", "sunderland", "2026-11-21T15:00:00Z", 0, 0),
    (11, "bournemouth", "nottm-forest", "2026-11-21T15:00:00Z", 2, 0),
    (11, "chelsea", "leeds", "2026-11-21T15:00:00Z", 1, 4),
    (11, "coventry", "crystal-palace", "2026-11-21T15:00:00Z", 3, 2),
    (11, "tottenham", "ipswich", "2026-11-21T15:00:00Z", 2, 0),
    (11, "newcastle", "arsenal", "2026-11-21T17:30:00Z", 2, 2),
]

# Upcoming open 2026–27 Premier League fixtures (Matchweek 11 Sunday/Monday + Matchweek 12 + marquee ties).
# All upcoming league fixtures have real_home_goals=None, real_away_goals=None because they have not been played yet.
UPCOMING_2026_27_FIXTURES: list[tuple[str, int, str, str, str]] = [
    # Matchweek 11 — Remaining Sunday 22 Nov & Monday 23 Nov 2026
    ("fix-mw11-hul-bha", 11, "hull", "brighton", "2026-11-22T14:00:00Z"),
    ("fix-mw11-liv-mun", 11, "liverpool", "man-utd", "2026-11-22T16:30:00Z"),
    ("fix-mw11-bre-eve", 11, "brentford", "everton", "2026-11-23T20:00:00Z"),
    # Matchweek 12 — 27–29 November 2026
    ("fix-mw12-nfo-che", 12, "nottm-forest", "chelsea", "2026-11-27T20:00:00Z"),
    ("fix-mw12-lee-cov", 12, "leeds", "coventry", "2026-11-28T15:00:00Z"),
    ("fix-mw12-mun-bre", 12, "man-utd", "brentford", "2026-11-28T15:00:00Z"),
    ("fix-mw12-ips-avl", 12, "ipswich", "aston-villa", "2026-11-28T17:30:00Z"),
    ("fix-mw12-eve-liv", 12, "everton", "liverpool", "2026-11-29T14:00:00Z"),
    ("fix-mw12-bha-new", 12, "brighton", "newcastle", "2026-11-29T14:00:00Z"),
    ("fix-mw12-cry-hul", 12, "crystal-palace", "hull", "2026-11-29T14:00:00Z"),
    ("fix-mw12-sun-tot", 12, "sunderland", "tottenham", "2026-11-29T14:00:00Z"),
    ("fix-mw12-ful-bou", 12, "fulham", "bournemouth", "2026-11-29T14:00:00Z"),
    ("fix-mw12-ars-mci", 12, "arsenal", "man-city", "2026-11-29T16:30:00Z"),
    # Featured Winter Marquee Ties
    ("fix-mw15-liv-che", 15, "liverpool", "chelsea", "2026-12-09T20:00:00Z"),
    ("fix-mw16-ars-mun", 16, "arsenal", "man-utd", "2026-12-20T16:30:00Z"),
    ("fix-mw23-mun-liv", 23, "man-utd", "liverpool", "2027-01-23T17:30:00Z"),
    ("fix-mw27-mun-ars", 27, "man-utd", "arsenal", "2027-02-27T16:30:00Z"),
]

# Authentic 2026–27 goal, assist, and clean-sheet save events mapped onto completed matches.
SEEDED_MATCH_EVENTS: dict[str, list[dict[str, Any]]] = {
    "fix-mw1-ars-cov": [
        {"event_type": "Goal", "footballer_id": "ars-fwd-1", "assist_footballer_id": "ars-mid-2", "minute": 18},
        {"event_type": "Goal", "footballer_id": "ars-fwd-3", "assist_footballer_id": "ars-fwd-2", "minute": 52},
        {"event_type": "Goal", "footballer_id": "ars-mid-2", "assist_footballer_id": "ars-mid-1", "minute": 79},
        {"event_type": "Save", "footballer_id": "ars-gk-1", "minute": 64},
    ],
    "fix-mw1-eve-cry": [
        {"event_type": "Goal", "footballer_id": "eve-fwd-1", "assist_footballer_id": "eve-mid-2", "minute": 31},
        {"event_type": "Goal", "footballer_id": "eve-fwd-1", "assist_footballer_id": "eve-mid-1", "minute": 74},
        {"event_type": "Save", "footballer_id": "eve-gk-1", "minute": 83},
    ],
    "fix-mw1-nfo-lee": [
        {"event_type": "Goal", "footballer_id": "lee-fwd-1", "assist_footballer_id": "lee-mid-1", "minute": 66},
        {"event_type": "Save", "footballer_id": "lee-gk-1", "minute": 89},
    ],
    "fix-mw1-bha-avl": [
        {"event_type": "Goal", "footballer_id": "bha-mid-1", "assist_footballer_id": "bha-fwd-1", "minute": 22},
        {"event_type": "Save", "footballer_id": "bha-gk-1", "minute": 71},
    ],
    "fix-mw1-mci-bou": [
        {"event_type": "Goal", "footballer_id": "mci-fwd-1", "assist_footballer_id": "mci-mid-2", "minute": 14},
        {"event_type": "Goal", "footballer_id": "mci-fwd-1", "assist_footballer_id": "mci-mid-1", "minute": 68},
    ],
    "fix-mw1-new-liv": [
        {"event_type": "Goal", "footballer_id": "liv-fwd-1", "assist_footballer_id": "liv-mid-1", "minute": 27},
        {"event_type": "Goal", "footballer_id": "liv-fwd-1", "assist_footballer_id": "liv-mid-2", "minute": 81},
    ],
    "fix-mw1-ful-che": [
        {"event_type": "Goal", "footballer_id": "che-fwd-1", "assist_footballer_id": "che-mid-2", "minute": 19},
        {"event_type": "Goal", "footballer_id": "che-fwd-1", "assist_footballer_id": "che-mid-3", "minute": 55},
        {"event_type": "Goal", "footballer_id": "che-mid-3", "assist_footballer_id": "che-mid-2", "minute": 86},
    ],
    "fix-mw2-cry-mci": [
        {"event_type": "Goal", "footballer_id": "mci-fwd-1", "assist_footballer_id": "mci-mid-2", "minute": 21},
        {"event_type": "Goal", "footballer_id": "mci-mid-2", "assist_footballer_id": "mci-fwd-1", "minute": 43},
        {"event_type": "Goal", "footballer_id": "mci-fwd-1", "assist_footballer_id": "mci-mid-3", "minute": 67},
        {"event_type": "Goal", "footballer_id": "mci-mid-2", "assist_footballer_id": "mci-mid-1", "minute": 84},
    ],
    "fix-mw2-liv-nfo": [
        {"event_type": "Goal", "footballer_id": "liv-fwd-1", "assist_footballer_id": "liv-mid-1", "minute": 35},
        {"event_type": "Goal", "footballer_id": "liv-mid-1", "assist_footballer_id": "liv-fwd-1", "minute": 72},
    ],
    "fix-mw2-bou-eve": [
        {"event_type": "Goal", "footballer_id": "bou-fwd-1", "assist_footballer_id": "bou-mid-1", "minute": 49},
    ],
    "fix-mw2-che-bha": [
        {"event_type": "Goal", "footballer_id": "che-fwd-1", "assist_footballer_id": "che-mid-2", "minute": 16},
        {"event_type": "Goal", "footballer_id": "che-mid-3", "assist_footballer_id": "che-fwd-1", "minute": 39},
        {"event_type": "Goal", "footballer_id": "che-mid-3", "assist_footballer_id": "che-mid-1", "minute": 61},
        {"event_type": "Goal", "footballer_id": "che-mid-2", "assist_footballer_id": None, "minute": 78},
        {"event_type": "Goal", "footballer_id": "bha-mid-1", "assist_footballer_id": "bha-fwd-2", "minute": 53},
    ],
    "fix-mw2-mun-ips": [
        {"event_type": "Goal", "footballer_id": "mun-fwd-2", "assist_footballer_id": "mun-mid-2", "minute": 11},
        {"event_type": "Goal", "footballer_id": "mun-fwd-1", "assist_footballer_id": "mun-mid-1", "minute": 29},
        {"event_type": "Goal", "footballer_id": "mun-mid-2", "assist_footballer_id": "mun-fwd-1", "minute": 54},
        {"event_type": "Goal", "footballer_id": "mun-fwd-2", "assist_footballer_id": "mun-mid-2", "minute": 73},
        {"event_type": "Goal", "footballer_id": "mun-fwd-3", "assist_footballer_id": "mun-def-2", "minute": 88},
    ],
    "fix-mw2-avl-ars": [
        {"event_type": "Goal", "footballer_id": "ars-fwd-1", "assist_footballer_id": "ars-mid-2", "minute": 63},
        {"event_type": "Save", "footballer_id": "ars-gk-1", "minute": 87},
    ],
    "fix-mw3-ips-liv": [
        {"event_type": "Goal", "footballer_id": "liv-fwd-1", "assist_footballer_id": "liv-mid-1", "minute": 58},
        {"event_type": "Goal", "footballer_id": "liv-mid-2", "assist_footballer_id": "liv-mid-1", "minute": 76},
        {"event_type": "Save", "footballer_id": "liv-gk-1", "minute": 44},
    ],
    "fix-mw3-new-bou": [
        {"event_type": "Goal", "footballer_id": "bou-mid-2", "assist_footballer_id": "bou-fwd-1", "minute": 33},
        {"event_type": "Goal", "footballer_id": "bou-mid-2", "assist_footballer_id": "bou-mid-1", "minute": 79},
    ],
    "fix-mw3-mci-cov": [
        {"event_type": "Goal", "footballer_id": "mci-fwd-1", "assist_footballer_id": "mci-mid-2", "minute": 51},
        {"event_type": "Save", "footballer_id": "mci-gk-1", "minute": 82},
    ],
    "fix-mw3-eve-mun": [
        {"event_type": "Goal", "footballer_id": "eve-fwd-1", "assist_footballer_id": "eve-mid-2", "minute": 24},
        {"event_type": "Goal", "footballer_id": "mun-mid-2", "assist_footballer_id": "mun-fwd-1", "minute": 48},
        {"event_type": "Goal", "footballer_id": "mun-fwd-2", "assist_footballer_id": "mun-mid-3", "minute": 77},
    ],
    "fix-mw3-ars-che": [
        {"event_type": "Goal", "footballer_id": "ars-fwd-3", "assist_footballer_id": "ars-fwd-1", "minute": 28},
        {"event_type": "Goal", "footballer_id": "che-mid-2", "assist_footballer_id": "che-fwd-1", "minute": 60},
        {"event_type": "Goal", "footballer_id": "ars-fwd-2", "assist_footballer_id": "ars-mid-2", "minute": 83},
    ],
    "fix-mw4-avl-nfo": [
        {"event_type": "Goal", "footballer_id": "avl-fwd-1", "assist_footballer_id": "avl-mid-2", "minute": 37},
    ],
    "fix-mw4-bou-bre": [
        {"event_type": "Goal", "footballer_id": "bou-fwd-1", "assist_footballer_id": "bou-mid-2", "minute": 25},
        {"event_type": "Goal", "footballer_id": "bou-mid-2", "assist_footballer_id": None, "minute": 70},
    ],
    "fix-mw4-liv-ful": [
        {"event_type": "Save", "footballer_id": "liv-gk-1", "minute": 73},
    ],
    "fix-mw4-tot-eve": [
        {"event_type": "Save", "footballer_id": "tot-gk-1", "minute": 62},
        {"event_type": "Save", "footballer_id": "eve-gk-1", "minute": 81},
    ],
    "fix-mw4-sun-ars": [
        {"event_type": "Goal", "footballer_id": "ars-fwd-3", "assist_footballer_id": "ars-mid-1", "minute": 34},
        {"event_type": "Goal", "footballer_id": "ars-mid-2", "assist_footballer_id": "ars-fwd-1", "minute": 75},
        {"event_type": "Save", "footballer_id": "ars-gk-1", "minute": 68},
    ],
    "fix-mw4-cov-bha": [
        {"event_type": "Goal", "footballer_id": "bha-mid-1", "assist_footballer_id": "bha-fwd-1", "minute": 19},
        {"event_type": "Save", "footballer_id": "bha-gk-1", "minute": 59},
    ],
    "fix-mw4-mun-mci": [
        {"event_type": "Goal", "footballer_id": "mci-fwd-1", "assist_footballer_id": "mci-mid-1", "minute": 64},
        {"event_type": "Save", "footballer_id": "mci-gk-1", "minute": 88},
    ],
    "fix-mw4-lee-new": [
        {"event_type": "Goal", "footballer_id": "lee-fwd-1", "assist_footballer_id": "lee-fwd-3", "minute": 41},
    ],
    "fix-mw5-tot-avl": [
        {"event_type": "Goal", "footballer_id": "avl-fwd-1", "assist_footballer_id": "avl-mid-2", "minute": 32},
        {"event_type": "Goal", "footballer_id": "avl-fwd-1", "assist_footballer_id": "avl-fwd-2", "minute": 85},
    ],
    "fix-mw5-bha-ars": [
        {"event_type": "Goal", "footballer_id": "bha-mid-1", "assist_footballer_id": "bha-fwd-1", "minute": 27},
        {"event_type": "Goal", "footballer_id": "bha-fwd-1", "assist_footballer_id": "bha-mid-1", "minute": 58},
        {"event_type": "Goal", "footballer_id": "bha-fwd-2", "assist_footballer_id": None, "minute": 82},
        {"event_type": "Save", "footballer_id": "bha-gk-1", "minute": 74},
    ],
    "fix-mw5-eve-ips": [
        {"event_type": "Goal", "footballer_id": "eve-fwd-1", "assist_footballer_id": "eve-mid-1", "minute": 56},
        {"event_type": "Save", "footballer_id": "eve-gk-1", "minute": 86},
    ],
    "fix-mw5-bou-liv": [
        {"event_type": "Goal", "footballer_id": "liv-fwd-1", "assist_footballer_id": "liv-mid-1", "minute": 69},
        {"event_type": "Save", "footballer_id": "liv-gk-1", "minute": 90},
    ],
    "fix-mw5-lee-cry": [
        {"event_type": "Save", "footballer_id": "lee-gk-1", "minute": 77},
    ],
    "fix-mw5-mci-sun": [
        {"event_type": "Goal", "footballer_id": "mci-fwd-1", "assist_footballer_id": "mci-mid-2", "minute": 12},
        {"event_type": "Goal", "footballer_id": "mci-mid-2", "assist_footballer_id": "mci-mid-3", "minute": 38},
        {"event_type": "Goal", "footballer_id": "mci-fwd-1", "assist_footballer_id": "mci-fwd-2", "minute": 61},
        {"event_type": "Goal", "footballer_id": "mci-mid-2", "assist_footballer_id": None, "minute": 79},
    ],
    "fix-mw5-ful-mun": [
        {"event_type": "Goal", "footballer_id": "mun-mid-1", "assist_footballer_id": "mun-mid-2", "minute": 65},
    ],
    "fix-mw6-ars-lee": [
        {"event_type": "Goal", "footballer_id": "ars-fwd-1", "assist_footballer_id": "ars-mid-2", "minute": 23},
        {"event_type": "Goal", "footballer_id": "ars-mid-1", "assist_footballer_id": "ars-fwd-1", "minute": 71},
        {"event_type": "Goal", "footballer_id": "lee-fwd-1", "assist_footballer_id": "lee-mid-1", "minute": 54},
    ],
    "fix-mw6-avl-bre": [
        {"event_type": "Goal", "footballer_id": "avl-fwd-1", "assist_footballer_id": "avl-mid-1", "minute": 44},
    ],
    "fix-mw6-che-bou": [
        {"event_type": "Goal", "footballer_id": "che-fwd-1", "assist_footballer_id": "che-mid-2", "minute": 15},
        {"event_type": "Goal", "footballer_id": "che-fwd-1", "assist_footballer_id": "che-fwd-2", "minute": 42},
        {"event_type": "Goal", "footballer_id": "che-mid-3", "assist_footballer_id": "che-mid-2", "minute": 67},
        {"event_type": "Goal", "footballer_id": "che-mid-2", "assist_footballer_id": None, "minute": 81},
    ],
    "fix-mw6-sun-bha": [
        {"event_type": "Save", "footballer_id": "bha-gk-1", "minute": 69},
    ],
    "fix-mw6-mun-tot": [
        {"event_type": "Goal", "footballer_id": "mun-def-1", "assist_footballer_id": "mun-mid-2", "minute": 52},
    ],
    "fix-mw6-liv-mci": [
        {"event_type": "Goal", "footballer_id": "mci-fwd-1", "assist_footballer_id": "mci-mid-2", "minute": 29},
        {"event_type": "Goal", "footballer_id": "mci-mid-2", "assist_footballer_id": "mci-mid-1", "minute": 57},
        {"event_type": "Goal", "footballer_id": "mci-fwd-2", "assist_footballer_id": "mci-mid-3", "minute": 84},
        {"event_type": "Save", "footballer_id": "mci-gk-1", "minute": 72},
    ],
    "fix-mw7-eve-che": [
        {"event_type": "Goal", "footballer_id": "che-mid-3", "assist_footballer_id": "che-mid-2", "minute": 62},
    ],
    "fix-mw7-bre-liv": [
        {"event_type": "Goal", "footballer_id": "liv-fwd-1", "assist_footballer_id": "liv-mid-1", "minute": 38},
        {"event_type": "Goal", "footballer_id": "liv-mid-1", "assist_footballer_id": "liv-mid-2", "minute": 83},
    ],
    "fix-mw7-bou-sun": [
        {"event_type": "Goal", "footballer_id": "bou-fwd-1", "assist_footballer_id": "bou-mid-2", "minute": 31},
        {"event_type": "Goal", "footballer_id": "bou-mid-2", "assist_footballer_id": "bou-fwd-2", "minute": 74},
    ],
    "fix-mw7-bha-cry": [
        {"event_type": "Save", "footballer_id": "bha-gk-1", "minute": 60},
    ],
    "fix-mw7-lee-mun": [
        {"event_type": "Goal", "footballer_id": "lee-fwd-1", "assist_footballer_id": "lee-mid-2", "minute": 36},
        {"event_type": "Goal", "footballer_id": "mun-mid-3", "assist_footballer_id": "mun-mid-2", "minute": 78},
    ],
    "fix-mw7-nfo-ars": [
        {"event_type": "Goal", "footballer_id": "ars-mid-2", "assist_footballer_id": "ars-fwd-2", "minute": 67},
        {"event_type": "Save", "footballer_id": "ars-gk-1", "minute": 85},
    ],
    "fix-mw8-avl-mci": [
        {"event_type": "Goal", "footballer_id": "avl-fwd-1", "assist_footballer_id": "avl-mid-2", "minute": 59},
    ],
    "fix-mw8-ars-eve": [
        {"event_type": "Goal", "footballer_id": "ars-fwd-3", "assist_footballer_id": "ars-mid-2", "minute": 33},
        {"event_type": "Goal", "footballer_id": "ars-fwd-2", "assist_footballer_id": "ars-fwd-1", "minute": 76},
        {"event_type": "Save", "footballer_id": "ars-gk-1", "minute": 80},
    ],
    "fix-mw8-che-tot": [
        {"event_type": "Goal", "footballer_id": "tot-fwd-1", "assist_footballer_id": "tot-mid-2", "minute": 44},
        {"event_type": "Goal", "footballer_id": "tot-mid-2", "assist_footballer_id": "tot-mid-1", "minute": 81},
        {"event_type": "Save", "footballer_id": "tot-gk-1", "minute": 70},
    ],
    "fix-mw8-liv-bha": [
        {"event_type": "Goal", "footballer_id": "liv-fwd-1", "assist_footballer_id": "liv-mid-1", "minute": 29},
        {"event_type": "Goal", "footballer_id": "liv-fwd-2", "assist_footballer_id": "liv-mid-1", "minute": 74},
        {"event_type": "Save", "footballer_id": "liv-gk-1", "minute": 68},
    ],
    "fix-mw8-mun-bou": [
        {"event_type": "Goal", "footballer_id": "bou-fwd-1", "assist_footballer_id": "bou-mid-2", "minute": 26},
        {"event_type": "Goal", "footballer_id": "bou-mid-2", "assist_footballer_id": "bou-mid-1", "minute": 63},
    ],
    "fix-mw8-sun-lee": [
        {"event_type": "Save", "footballer_id": "lee-gk-1", "minute": 82},
    ],
    "fix-mw9-bou-lee": [
        {"event_type": "Goal", "footballer_id": "bou-fwd-1", "assist_footballer_id": "bou-fwd-2", "minute": 55},
    ],
    "fix-mw9-mci-bha": [
        {"event_type": "Goal", "footballer_id": "bha-fwd-3", "assist_footballer_id": "bha-fwd-1", "minute": 72},
        {"event_type": "Save", "footballer_id": "bha-gk-1", "minute": 88},
    ],
    "fix-mw9-tot-cry": [
        {"event_type": "Save", "footballer_id": "tot-gk-1", "minute": 79},
    ],
    "fix-mw9-liv-ars": [
        {"event_type": "Goal", "footballer_id": "ars-def-2", "assist_footballer_id": "ars-fwd-1", "minute": 64},
        {"event_type": "Save", "footballer_id": "ars-gk-1", "minute": 89},
    ],
    "fix-mw9-new-eve": [
        {"event_type": "Goal", "footballer_id": "eve-fwd-1", "assist_footballer_id": "eve-mid-2", "minute": 68},
        {"event_type": "Save", "footballer_id": "eve-gk-1", "minute": 84},
    ],
    "fix-mw10-eve-cov": [
        {"event_type": "Goal", "footballer_id": "eve-fwd-1", "assist_footballer_id": "eve-mid-2", "minute": 41},
    ],
    "fix-mw10-lee-tot": [
        {"event_type": "Save", "footballer_id": "lee-gk-1", "minute": 86},
    ],
    "fix-mw10-ars-hul": [
        {"event_type": "Goal", "footballer_id": "ars-def-1", "assist_footballer_id": "ars-mid-1", "minute": 57},
        {"event_type": "Save", "footballer_id": "ars-gk-1", "minute": 81},
    ],
    "fix-mw10-bha-bre": [
        {"event_type": "Save", "footballer_id": "bha-gk-1", "minute": 75},
    ],
    "fix-mw10-mun-avl": [
        {"event_type": "Goal", "footballer_id": "mun-mid-2", "assist_footballer_id": "mun-fwd-1", "minute": 14},
        {"event_type": "Goal", "footballer_id": "mun-fwd-2", "assist_footballer_id": "mun-mid-2", "minute": 39},
        {"event_type": "Goal", "footballer_id": "mun-fwd-1", "assist_footballer_id": "mun-mid-1", "minute": 66},
        {"event_type": "Goal", "footballer_id": "mun-mid-2", "assist_footballer_id": None, "minute": 87},
        {"event_type": "Goal", "footballer_id": "avl-fwd-1", "assist_footballer_id": "avl-fwd-2", "minute": 52},
    ],
    "fix-mw11-tot-ips": [
        {"event_type": "Save", "footballer_id": "tot-gk-1", "minute": 77},
    ],
    "fix-mw11-new-ars": [
        {"event_type": "Goal", "footballer_id": "new-fwd-1", "assist_footballer_id": "new-mid-2", "minute": 24},
        {"event_type": "Goal", "footballer_id": "ars-mid-3", "assist_footballer_id": "ars-mid-2", "minute": 51},
        {"event_type": "Goal", "footballer_id": "new-mid-1", "assist_footballer_id": "new-fwd-2", "minute": 72},
        {"event_type": "Goal", "footballer_id": "ars-def-3", "assist_footballer_id": "ars-mid-1", "minute": 89},
    ],
}


def _fixture_id(rnd: int, home: str, away: str) -> str:
    return f"fix-mw{rnd}-{_CLUB_SLUG[home]}-{_CLUB_SLUG[away]}"


def _playing_window_for_kickoff(kickoff_utc: str, *, grace_days: int = 14) -> tuple[str, str]:
    """Derive `[window_open_utc, window_close_utc]` from scheduled `kickoff_utc` so future 2027 ties stay open through their matchweek."""
    ko_dt = datetime.fromisoformat(kickoff_utc.replace("Z", "+00:00")).astimezone(timezone.utc)
    season_anchor = datetime(2026, 11, 21, 0, 0, 0, tzinfo=timezone.utc)
    open_dt = min(ko_dt - timedelta(days=7), season_anchor)
    close_dt = (ko_dt + timedelta(days=grace_days)).replace(hour=23, minute=59, second=59)
    return (
        open_dt.strftime("%Y-%m-%dT%H:%M:%SZ"),
        close_dt.strftime("%Y-%m-%dT%H:%M:%SZ"),
    )


def ensure_demo_world(store: WorldLedgerStore, catalogue: ClubCatalogue) -> str:
    worlds = store.list_worlds()
    existing = next((w for w in worlds if w["id"] == DEFAULT_WORLD_ID), None)
    if existing is not None:
        if (
            str(store.db_path) == "/tmp/psfc_world.db"
            and existing.get("name") == "Friends Premier Parallel Season"
        ):
            store.reset_world_for_demo(DEFAULT_WORLD_ID)
        else:
            return DEFAULT_WORLD_ID

    gate = ProposalGate(store, catalogue)
    cartridge_svc = CartridgeSyncService(store)

    with store.batch():
        store.create_world(
            world_id=DEFAULT_WORLD_ID,
            name="Premier League 2026/27",
            competition_name="Premier League",
            season_label="2026/27",
            season_timezone="Europe/London",
            default_match_format="5v5",
            caused_by="commissioner",
        )

        store.create_competition(
            competition_id=DEFAULT_LEAGUE_ID,
            world_id=DEFAULT_WORLD_ID,
            name="Premier League (2026/27 Parallel Staging)",
            kind="league",
            format_name="5v5",
            caused_by="commissioner",
        )
        store.create_competition(
            competition_id=DEFAULT_CUP_ID,
            world_id=DEFAULT_WORLD_ID,
            name="FA Parallel Cup (2026/27 Knockout)",
            kind="knockout",
            format_name="5v5",
            real_final_pair=["liverpool", "arsenal"],
            caused_by="commissioner",
        )

        # 1. Create & ingest all 107 completed 2026–27 Premier League matches (MW 1 through Sat 21 Nov 2026 MW 11)
        for rnd, home, away, ko, hg, ag in COMPLETED_2026_27_RESULTS:
            fid = _fixture_id(rnd, home, away)
            win_open, win_close = _playing_window_for_kickoff(ko)
            store.create_fixture(
                fixture_id=fid,
                world_id=DEFAULT_WORLD_ID,
                competition_id=DEFAULT_LEAGUE_ID,
                stage="league",
                round_no=rnd,
                home_club_id=home,
                away_club_id=away,
                kickoff_utc=ko,
                window_open_utc=win_open,
                window_close_utc=win_close,
                result_source="imported",
                state="open",
                extra_time_enabled=False,
                real_home_goals=hg,
                real_away_goals=ag,
            )
            gate.submit_fixture_result_import(
                world_id=DEFAULT_WORLD_ID,
                fixture_id=fid,
                home_goals=hg,
                away_goals=ag,
                events=SEEDED_MATCH_EVENTS.get(fid, []),
            )

        # 2. Create open upcoming Matchweek 11 (Sun/Mon 22–23 Nov), Matchweek 12 (27–29 Nov), & featured winter fixtures
        for fid, rnd, home, away, ko in UPCOMING_2026_27_FIXTURES:
            win_open, win_close = _playing_window_for_kickoff(ko)
            store.create_fixture(
                fixture_id=fid,
                world_id=DEFAULT_WORLD_ID,
                competition_id=DEFAULT_LEAGUE_ID,
                stage="league",
                round_no=rnd,
                home_club_id=home,
                away_club_id=away,
                kickoff_utc=ko,
                window_open_utc=win_open,
                window_close_utc=win_close,
                result_source="imported",
                state="open",
                extra_time_enabled=False,
                real_home_goals=None,
                real_away_goals=None,
            )

        # 3. Create FA Parallel Cup Semi-Final ties (`extra_time_enabled=True`)
        cup_semis_spec = [
            ("fix-cup-sf1-mun-liv", 1, "man-utd", "liverpool", "2027-04-18T15:30:00Z", 0, 2),
            ("fix-cup-sf2-ars-che", 1, "arsenal", "chelsea", "2027-04-19T16:30:00Z", 2, 1),
        ]
        for fid, rnd, home, away, ko, rhg, rag in cup_semis_spec:
            win_open, win_close = _playing_window_for_kickoff(ko)
            store.create_fixture(
                fixture_id=fid,
                world_id=DEFAULT_WORLD_ID,
                competition_id=DEFAULT_CUP_ID,
                stage="cup_sf",
                round_no=rnd,
                home_club_id=home,
                away_club_id=away,
                kickoff_utc=ko,
                window_open_utc=win_open,
                window_close_utc=win_close,
                result_source="imported",
                state="open",
                extra_time_enabled=True,
                real_home_goals=rhg,
                real_away_goals=rag,
            )

        # 4. Register human participants + Cartridge devices taking over Manchester United & Arsenal at Matchweek 11!
        store.register_participant("part-alex", "Alex (Cartridge #01)", DEFAULT_WORLD_ID)
        store.register_participant("part-sam", "Sam (Cartridge #02)", DEFAULT_WORLD_ID)
        cartridge_svc.register_device(
            participant_id="part-alex",
            world_id=DEFAULT_WORLD_ID,
            label="Alex RP2040 Pad + Cartridge #01",
            device_id="dev-alex-01",
        )
        cartridge_svc.register_device(
            participant_id="part-sam",
            world_id=DEFAULT_WORLD_ID,
            label="Sam RP2040 Pad + Cartridge #02",
            device_id="dev-sam-02",
        )

        store.assign_club(
            assignment_id="asgn-mun-alex",
            world_id=DEFAULT_WORLD_ID,
            club_id="man-utd",
            participant_id="part-alex",
            effective_from_round=11,
            baseline_summary={"inherited_played": 10, "inherited_points": 8},
        )
        store.assign_club(
            assignment_id="asgn-ars-sam",
            world_id=DEFAULT_WORLD_ID,
            club_id="arsenal",
            participant_id="part-sam",
            effective_from_round=11,
            baseline_summary={"inherited_played": 11, "inherited_points": 28},
        )

    return DEFAULT_WORLD_ID
