"""Comprehensive tests for Parallel Season FC ledger invariants, Proposal Gate, and psfc-replay validation."""

from __future__ import annotations

import json
import sqlite3
import subprocess
import tempfile
from pathlib import Path

import pytest
from fastapi.testclient import TestClient

from psfc_host.api.app import create_app
from psfc_host.catalogue.clubs import ClubCatalogue
from psfc_host.ledger.store import WorldLedgerStore
from psfc_host.match.validator import resolve_replay_binary
from psfc_host.seed import DEFAULT_WORLD_ID


def _simulate_frames_for_snapshot(snapshot: dict) -> dict:
    """Use compiled psfc_wasm.wasm in Node.js to produce an authentic recorded input log for `snapshot`."""
    repo_root = Path(__file__).resolve().parents[2]
    wasm_path = (
        repo_root / "sim" / "target" / "wasm32-unknown-unknown" / "release" / "psfc_wasm.wasm"
    )
    with tempfile.TemporaryDirectory(prefix="psfc_test_wasm_") as tmp:
        snap_file = Path(tmp) / "snap.json"
        out_file = Path(tmp) / "log.json"
        snap_file.write_text(json.dumps(snapshot), encoding="utf-8")

        script = """
        const fs = require("fs");
        (async () => {
          const wasmBytes = fs.readFileSync(process.argv[1]);
          const snapBytes = fs.readFileSync(process.argv[2]);
          const outPath = process.argv[3];
          const { instance } = await WebAssembly.instantiate(wasmBytes, {});
          const ex = instance.exports;
          const dec = new TextDecoder();
          const readOut = () => dec.decode(new Uint8Array(ex.memory.buffer, ex.wasm_output_ptr(), ex.wasm_output_len()));

          const inPtr = ex.wasm_prepare_input_buf(snapBytes.length);
          new Uint8Array(ex.memory.buffer, inPtr, snapBytes.length).set(snapBytes);
          if (!ex.sim_init_from_snapshot_json()) {
            throw new Error("sim_init_from_snapshot_json failed: " + readOut());
          }

          for (let t = 0; t < 180; t++) {
            const mx = (t % 2 === 0) ? 100 : 80;
            const btns = (t === 1 || t === 65) ? 1 : ((t === 130) ? 8 : 16);
            ex.sim_tick(mx, 25, 0, 0, btns, -80, 0, 0, 0, (t === 95 ? 64 : 0));
          }
          ex.sim_auto_step(5000);
          ex.sim_export_input_log_json();
          fs.writeFileSync(outPath, readOut());
        })();
        """
        subprocess.run(
            ["node", "-e", script, str(wasm_path), str(snap_file), str(out_file)],
            check=True,
            timeout=15,
        )
        return json.loads(out_file.read_text(encoding="utf-8"))


def test_world_event_ledger_is_strictly_append_only() -> None:
    store = WorldLedgerStore(":memory:")
    store.create_world(
        world_id="w1",
        name="Test World",
        competition_name="League",
        season_label="2026/27",
    )
    with pytest.raises(sqlite3.IntegrityError, match="append-only"):
        store.conn.execute("UPDATE world_event SET caused_by = 'hacker' WHERE seq = 1")
    with pytest.raises(sqlite3.IntegrityError, match="append-only"):
        store.conn.execute("DELETE FROM world_event WHERE seq = 1")


def test_five_a_side_default_and_eleven_a_side_parameterization_in_catalogue() -> None:
    cat = ClubCatalogue()
    snap_5 = cat.build_fixture_snapshot("fx-5", "man-utd", "arsenal", seed=42, format_name="5v5")
    assert snap_5["format"]["players_per_side"] == 5
    assert snap_5["format"]["pitch_length_mm"] == 56_000
    assert snap_5["format"]["pitch_width_mm"] == 36_000
    assert snap_5["rules"]["offsides_enabled"] is False
    assert len(snap_5["home"]["starters"]) == 5
    assert snap_5["home"]["starters"][0]["role"] == "Goalkeeper"
    assert sum(1 for p in snap_5["home"]["starters"] if p["role"] != "Goalkeeper") == 4

    snap_11 = cat.build_fixture_snapshot("fx-11", "man-utd", "arsenal", seed=42, format_name="11v11")
    assert snap_11["format"]["players_per_side"] == 11
    assert snap_11["format"]["pitch_length_mm"] == 105_000
    assert snap_11["rules"]["offsides_enabled"] is True
    assert len(snap_11["home"]["starters"]) == 11
    assert snap_11["home"]["starters"][0]["role"] == "Goalkeeper"
    assert sum(1 for p in snap_11["home"]["starters"] if p["role"] != "Goalkeeper") == 10


def test_frozen_snapshot_immutability_across_external_transfer() -> None:
    app = create_app(db_path=":memory:", seed_demo=True)
    client = TestClient(app)

    # 1. Freeze Matchweek 27 Man Utd vs Arsenal snapshot
    r_open = client.post(
        "/api/fixtures/fix-mw27-mun-ars/sessions",
        json={"actor_id": "part-alex", "format_name": "5v5", "half_duration_ticks": 300},
    )
    assert r_open.status_code == 200
    frozen_mw27 = r_open.json()["snapshot"]
    mw27_mun_ids = [p["footballer_id"] for p in frozen_mw27["home"]["starters"]]
    assert "mun-fwd-4" in mw27_mun_ids  # M. Cunha is frozen into Matchweek 27!

    # 2. Ingest external transfer moving M. Cunha from man-utd to chelsea
    r_tr = client.post(
        f"/api/worlds/{DEFAULT_WORLD_ID}/ingest/transfer",
        json={
            "footballer_id": "mun-fwd-4",
            "from_club_id": "man-utd",
            "to_club_id": "chelsea",
            "new_shirt": 19,
        },
    )
    assert r_tr.status_code == 200
    assert r_tr.json()["decision"] == "applied"

    # 3. Verify frozen snapshot is 100% UNCHANGED, while newly opened Matchweek 23 snapshot reflects transfer!
    r_snap_after = client.get("/api/fixtures/fix-mw27-mun-ars/snapshot")
    assert r_snap_after.status_code == 200
    assert [
        p["footballer_id"] for p in r_snap_after.json()["snapshot"]["home"]["starters"]
    ] == mw27_mun_ids

    r_open_mw23 = client.post(
        "/api/fixtures/fix-mw23-mun-liv/sessions",
        json={"actor_id": "part-alex", "format_name": "5v5", "half_duration_ticks": 300},
    )
    assert r_open_mw23.status_code == 200
    mw23_mun_ids = [p["footballer_id"] for p in r_open_mw23.json()["snapshot"]["home"]["starters"]]
    assert "mun-fwd-4" not in mw23_mun_ids


def test_proposal_gate_parks_imports_for_represented_and_played_fixtures() -> None:
    app = create_app(db_path=":memory:", seed_demo=True)
    client = TestClient(app)

    # Attempt to import a real-world 0-5 result onto represented `fix-mw27-mun-ars` (`played_vs_friend`)
    r_imp = client.post(
        f"/api/worlds/{DEFAULT_WORLD_ID}/ingest/result",
        json={
            "fixture_id": "fix-mw27-mun-ars",
            "home_goals": 0,
            "away_goals": 5,
            "events": [{"event_type": "Goal", "footballer_id": "ars-fwd-1", "minute": 10}],
        },
    )
    assert r_imp.status_code == 200
    body = r_imp.json()
    assert body["decision"] == "parked"
    assert body["reason"] == "world_precedence"

    # Verify Saka's inherited goals did NOT absorb the parked goal (stays at 3 from MW1 + MW2 + MW6)
    players = client.get(f"/api/worlds/{DEFAULT_WORLD_ID}/players").json()["players"]
    saka = next(p for p in players if p["footballer_id"] == "ars-fwd-1")
    assert saka["inherited"]["goals"] == 3
    assert saka["earned"]["goals"] == 0
    assert saka["combined"]["goals"] == 3


def test_wasm_to_psfc_replay_end_to_end_validation_idempotency_and_correction() -> None:
    assert resolve_replay_binary().is_file()
    app = create_app(db_path=":memory:", seed_demo=True)
    client = TestClient(app)

    # Open 5v5 session on Man Utd vs Liverpool (short 360-tick = 6s halves for fast test)
    r_open = client.post(
        "/api/fixtures/fix-mw23-mun-liv/sessions",
        json={
            "actor_id": "part-alex",
            "format_name": "5v5",
            "half_duration_ticks": 360,
            "home_controller": {"kind": "Human"},
            "away_controller": {"kind": "Cpu", "difficulty": "Hard"},
        },
    )
    assert r_open.status_code == 200
    sess_id = r_open.json()["session_id"]
    snapshot = r_open.json()["snapshot"]

    # Simulate in WASM (wasm32) and export input log
    wasm_log = _simulate_frames_for_snapshot(snapshot)

    # 1. Tampered state hash MUST be rejected with 422!
    r_bad = client.post(
        f"/api/sessions/{sess_id}/complete",
        json={
            "actor_id": "part-alex",
            "frames_hex": wasm_log["frames_hex"],
            "claimed_final_hash_hex": "deadbeef00000000",
            "claimed_score": [99, 0],
            "hash_checkpoints": wasm_log["hash_checkpoints"],
        },
    )
    assert r_bad.status_code == 422

    # 2. Valid input log with LIED client score `[99, 0]` -> accepted using ONLY replay score!
    r_ok = client.post(
        f"/api/sessions/{sess_id}/complete",
        json={
            "actor_id": "part-alex",
            "frames_hex": wasm_log["frames_hex"],
            "claimed_final_hash_hex": wasm_log["claimed_final_hash_hex"],
            "claimed_score": [99, 0],
            "hash_checkpoints": wasm_log["hash_checkpoints"],
        },
    )
    assert r_ok.status_code == 200
    done = r_ok.json()
    assert done["idempotent_duplicate"] is False
    assert done["score_discrepancy_ignored"] is True
    assert done["accepted_result"]["home_goals"] != 99
    assert done["accepted_result"]["final_hash_hex"] == wasm_log["claimed_final_hash_hex"]

    # 3. Duplicate submission is idempotent ("one fixture, one result")
    r_dup = client.post(
        f"/api/sessions/{sess_id}/complete",
        json={
            "actor_id": "part-alex",
            "frames_hex": wasm_log["frames_hex"],
            "claimed_final_hash_hex": wasm_log["claimed_final_hash_hex"],
        },
    )
    assert r_dup.status_code == 200
    assert r_dup.json()["idempotent_duplicate"] is True

    # 4. Commissioner `result_corrected` rebuilds standings & earned/combined stats cleanly!
    r_corr = client.post(
        "/api/admin/fixtures/fix-mw23-mun-liv/correct",
        json={
            "new_home_goals": 3,
            "new_away_goals": 1,
            "new_events": [
                {
                    "event_type": "Goal",
                    "footballer_id": "mun-fwd-4",
                    "assist_footballer_id": "mun-mid-2",
                    "minute": 12,
                },
                {
                    "event_type": "Goal",
                    "footballer_id": "mun-fwd-4",
                    "assist_footballer_id": "mun-mid-1",
                    "minute": 27,
                },
                {
                    "event_type": "Goal",
                    "footballer_id": "mun-mid-2",
                    "assist_footballer_id": None,
                    "minute": 55,
                },
                {
                    "event_type": "Goal",
                    "footballer_id": "liv-fwd-4",
                    "assist_footballer_id": None,
                    "minute": 68,
                },
            ],
            "reason": "verified replay audit test",
        },
    )
    assert r_corr.status_code == 200

    standings = client.get(f"/api/worlds/{DEFAULT_WORLD_ID}/standings").json()["standings"]
    mun_row = next(r for r in standings if r["club_id"] == "man-utd")
    # Man Utd had 8 inherited pts (MW1-10: 1W 5D 4L) + 3 earned pts (W 3-1 vs LIV) = 11 pts!
    assert mun_row["inherited_points"] == 8
    assert mun_row["earned_points"] == 3
    assert mun_row["points"] == 11

    players = client.get(f"/api/worlds/{DEFAULT_WORLD_ID}/players").json()["players"]
    cunha = next(p for p in players if p["footballer_id"] == "mun-fwd-4")
    # M. Cunha had 2 inherited goals (MW2 + MW10) + 2 earned goals (corrected MW23) = 4 combined goals!
    assert cunha["inherited"]["goals"] == 2
    assert cunha["earned"]["goals"] == 2
    assert cunha["combined"]["goals"] == 4


def test_live_2026_27_premier_league_season_table_and_current_squads() -> None:
    app = create_app(db_path=":memory:", seed_demo=True)
    client = TestClient(app)

    ov = client.get(f"/api/worlds/{DEFAULT_WORLD_ID}/overview").json()
    assert ov["world"]["name"] == "Premier League 2026/27"
    assert ov["world"]["season_label"] == "2026/27"
    assert len(ov["clubs"]) == 20
    assert len(ov["standings"]) == 20

    # Verify promoted 2026-27 clubs (Coventry City, Hull City, Ipswich Town, Leeds, Sunderland) are present
    club_ids = {c["club_id"] for c in ov["clubs"]}
    assert {
        "arsenal",
        "brighton",
        "man-city",
        "bournemouth",
        "liverpool",
        "leeds",
        "everton",
        "chelsea",
        "tottenham",
        "brentford",
        "aston-villa",
        "newcastle",
        "coventry",
        "hull",
        "fulham",
        "ipswich",
        "nottm-forest",
        "sunderland",
        "man-utd",
        "crystal-palace",
    } == club_ids

    # Verify exact official Matchweek 11 (21 Nov 2026) Premier League standings & zero initial divergence
    by_club = {r["club_id"]: r for r in ov["standings"]}
    assert by_club["arsenal"]["points"] == 28 and by_club["arsenal"]["position"] == 1
    assert by_club["brighton"]["points"] == 22 and by_club["brighton"]["position"] == 2
    assert by_club["man-city"]["points"] == 22 and by_club["man-city"]["position"] == 3
    assert by_club["bournemouth"]["points"] == 18 and by_club["bournemouth"]["position"] == 4
    assert by_club["liverpool"]["points"] == 18 and by_club["liverpool"]["position"] == 5
    assert by_club["man-utd"]["points"] == 8 and by_club["man-utd"]["position"] == 19
    assert by_club["crystal-palace"]["points"] == 7 and by_club["crystal-palace"]["position"] == 20
    assert all(r["delta_vs_real"] == 0 for r in ov["standings"])

    # Verify 100% of accepted match goals (293 across 107 matches) are attributed in player projections
    all_players = client.get(f"/api/worlds/{DEFAULT_WORLD_ID}/players").json()["players"]
    assert sum(r["gf"] for r in ov["standings"]) == 293
    assert sum(p["inherited"]["goals"] for p in all_players) == 293
    assert all_players[0]["footballer_id"] == "mci-fwd-1" and all_players[0]["inherited"]["goals"] == 9
    assert all_players[1]["footballer_id"] == "liv-fwd-4" and all_players[1]["inherited"]["goals"] == 7

    # Verify authentic 2026-27 squads and absence of outdated 2023/24-2024/25 players
    rosters = ov["rosters"]
    mun_names = {p["name"] for p in rosters["man-utd"]["squad"]}
    ars_names = {p["name"] for p in rosters["arsenal"]["squad"]}
    liv_names = {p["name"] for p in rosters["liverpool"]["squad"]}
    che_names = {p["name"] for p in rosters["chelsea"]["squad"]}
    mci_names = {p["name"] for p in rosters["man-city"]["squad"]}

    assert {"S. Lammens", "M. Cunha", "B. Mbeumo", "B. Sesko", "Y. Tielemans"}.issubset(mun_names)
    assert {"V. Gyokeres", "E. Eze", "R. Calafiori", "B. Guimaraes"}.issubset(ars_names)
    assert {"F. Wirtz", "A. Isak", "J. Frimpong", "B. Barcola", "M. Kerkez"}.issubset(liv_names)
    assert {"E. Martinez", "C. Palmer", "Joao Pedro", "M. Rogers", "Estevao"}.issubset(che_names)
    assert {"G. Donnarumma", "R. Cherki", "E. Haaland", "E. Fernandez", "A. Semenyo"}.issubset(mci_names)

    all_names = {p["name"] for r in rosters.values() for p in r["squad"]}
    for outdated in (
        "M. Rashford",
        "Casemiro",
        "R. Hojlund",
        "M. Salah",
        "T. Alexander-Arnold",
        "L. Diaz",
        "D. Nunez",
        "C. Nkunku",
        "T. Partey",
    ):
        assert outdated not in all_names


def test_legacy_persistent_world_preserves_footballer_identities_and_partial_feed_enrichment() -> None:
    from psfc_host.ingest.proposal_gate import ProposalGate
    from psfc_host.ledger.projections import project_player_stats

    # 1. Pre-existing persistent DB outside /tmp/psfc_world.db keeps legacy footballer identities (`mun-fwd-1` == M. Rashford)
    with tempfile.TemporaryDirectory(prefix="psfc_legacy_db_") as tmp:
        db_file = Path(tmp) / "pre_update_world.sqlite"
        legacy_store = WorldLedgerStore(db_file)
        legacy_cat = ClubCatalogue(legacy_pre_2026_27=True)
        legacy_gate = ProposalGate(legacy_store, legacy_cat)

        legacy_store.create_world(
            world_id=DEFAULT_WORLD_ID,
            name="Friends Premier Parallel Season",
            competition_name="Premier League",
            season_label="2025/26",
        )
        legacy_store.create_competition(
            competition_id="comp-prem-2026",
            world_id=DEFAULT_WORLD_ID,
            name="Premier League",
            kind="league",
        )
        legacy_store.create_fixture(
            fixture_id="fix-legacy-r1-mun-che",
            world_id=DEFAULT_WORLD_ID,
            competition_id="comp-prem-2026",
            stage="league",
            round_no=1,
            home_club_id="man-utd",
            away_club_id="chelsea",
            kickoff_utc="2025-08-17T15:00:00Z",
            window_open_utc="2025-08-10T00:00:00Z",
            window_close_utc="2025-09-01T00:00:00Z",
            result_source="imported",
            state="open",
            real_home_goals=2,
            real_away_goals=0,
        )
        legacy_gate.submit_fixture_result_import(
            world_id=DEFAULT_WORLD_ID,
            fixture_id="fix-legacy-r1-mun-che",
            home_goals=2,
            away_goals=0,
            events=[
                {"event_type": "Goal", "footballer_id": "mun-fwd-1", "minute": 21},
                {"event_type": "Goal", "footballer_id": "mun-fwd-1", "minute": 67},
            ],
        )
        legacy_gate.submit_transfer_import(
            world_id=DEFAULT_WORLD_ID,
            footballer_id="mun-fwd-1",
            from_club_id="man-utd",
            to_club_id="arsenal",
            new_shirt=19,
        )
        legacy_events = legacy_store.list_events(DEFAULT_WORLD_ID)
        legacy_store.close()

        # Even if projected directly against the 2026-27 default catalogue, `mun-fwd-1` resolves to M. Rashford (never M. Cunha!)
        modern_cat = ClubCatalogue()
        direct_stats = project_player_stats(
            legacy_events,
            modern_cat.active_footballers(),
            historical_footballers=modern_cat.all_footballers(),
        )
        by_fid = {p["footballer_id"]: p for p in direct_stats}
        assert by_fid["mun-fwd-1"]["name"] == "M. Rashford"
        assert by_fid["mun-fwd-1"]["inherited"]["goals"] == 2
        assert by_fid["mun-fwd-4"]["name"] == "M. Cunha"
        assert by_fid["mun-fwd-4"]["inherited"]["goals"] == 0

        # Booting FastAPI on the pre-update persistent DB preserves the legacy world, Rashford's goals, and Rashford's transfer
        app_legacy = create_app(db_path=db_file, seed_demo=True)
        c_leg = TestClient(app_legacy)
        players_leg = {
            p["footballer_id"]: p
            for p in c_leg.get(f"/api/worlds/{DEFAULT_WORLD_ID}/players").json()["players"]
        }
        assert players_leg["mun-fwd-1"]["name"] == "M. Rashford"
        assert players_leg["mun-fwd-1"]["inherited"]["goals"] == 2
        assert "mun-fwd-4" not in players_leg
        ars_squad = c_leg.get(f"/api/worlds/{DEFAULT_WORLD_ID}/clubs/arsenal/squad").json()["roster"]["squad"]
        assert any(p["footballer_id"] == "mun-fwd-1" and p["name"] == "M. Rashford" for p in ars_squad)

    # 2. Incomplete imported player-event feeds remain eligible for later enrichment via `supersedes_seq`
    store = WorldLedgerStore(":memory:")
    cat = ClubCatalogue()
    gate = ProposalGate(store, cat)
    store.create_world(
        world_id="w-enrich",
        name="Enrich World",
        competition_name="Premier League",
        season_label="2026/27",
    )
    store.create_competition(
        competition_id="comp-enrich",
        world_id="w-enrich",
        name="Premier League",
        kind="league",
    )
    store.create_fixture(
        fixture_id="fix-bha-avl-partial",
        world_id="w-enrich",
        competition_id="comp-enrich",
        stage="league",
        round_no=1,
        home_club_id="brighton",
        away_club_id="aston-villa",
        kickoff_utc="2026-08-23T13:00:00Z",
        window_open_utc="2026-08-16T00:00:00Z",
        window_close_utc="2026-09-06T23:59:59Z",
        result_source="imported",
        state="open",
        real_home_goals=4,
        real_away_goals=0,
    )
    p_initial = gate.submit_fixture_result_import(
        world_id="w-enrich",
        fixture_id="fix-bha-avl-partial",
        home_goals=4,
        away_goals=0,
        events=[{"event_type": "Goal", "footballer_id": "bha-mid-1", "minute": 22}],
    )
    assert p_initial["decision"] == "applied"

    p_complete = gate.submit_fixture_result_import(
        world_id="w-enrich",
        fixture_id="fix-bha-avl-partial",
        home_goals=4,
        away_goals=0,
        events=[
            {"event_type": "Goal", "footballer_id": "bha-mid-1", "minute": 22},
            {"event_type": "Goal", "footballer_id": "bha-fwd-1", "minute": 41},
            {"event_type": "Goal", "footballer_id": "bha-fwd-2", "minute": 63},
            {"event_type": "Goal", "footballer_id": "bha-fwd-3", "minute": 84},
        ],
    )
    assert p_complete["decision"] == "applied"
    assert p_complete["reason"] == "imported_events_enriched"

    enriched_stats = project_player_stats(
        store.list_events("w-enrich"),
        cat.active_footballers(),
        historical_footballers=cat.all_footballers(),
    )
    assert sum(p["inherited"]["goals"] for p in enriched_stats) == 4

    # Once all 4 goals are recorded, further duplicate imports are parked as `world_precedence`
    p_after_full = gate.submit_fixture_result_import(
        world_id="w-enrich",
        fixture_id="fix-bha-avl-partial",
        home_goals=4,
        away_goals=0,
        events=[
            {"event_type": "Goal", "footballer_id": "bha-mid-1", "minute": 22},
            {"event_type": "Goal", "footballer_id": "bha-fwd-1", "minute": 41},
            {"event_type": "Goal", "footballer_id": "bha-fwd-2", "minute": 63},
            {"event_type": "Goal", "footballer_id": "bha-fwd-3", "minute": 84},
        ],
    )
    assert p_after_full["decision"] == "parked"
    assert p_after_full["reason"] == "world_precedence"


def test_browser_wasm_client_routes_serve_bundle() -> None:
    app = create_app(db_path=":memory:", seed_demo=True)
    client = TestClient(app)

    r_root = client.get("/")
    assert r_root.status_code == 200
    assert "classicFutbol" in r_root.text
    assert "pitch-canvas" in r_root.text

    r_js = client.get("/static/app.js")
    assert r_js.status_code == 200
    assert "sim_pack_render_state" in r_js.text

    r_wasm = client.get("/wasm/psfc_wasm.wasm")
    assert r_wasm.status_code == 200
    assert r_wasm.content[:4] == b"\x00asm"
