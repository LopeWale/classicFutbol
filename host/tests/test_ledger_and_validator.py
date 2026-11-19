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

    # 1. Freeze Round 3 Man Utd vs Arsenal snapshot
    r_open = client.post(
        "/api/fixtures/fix-r3-mun-ars/sessions",
        json={"actor_id": "part-alex", "format_name": "5v5", "half_duration_ticks": 300},
    )
    assert r_open.status_code == 200
    frozen_r3 = r_open.json()["snapshot"]
    r3_mun_ids = [p["footballer_id"] for p in frozen_r3["home"]["starters"]]
    assert "mun-fwd-1" in r3_mun_ids  # M. Rashford is frozen into Round 3!

    # 2. Ingest external transfer moving M. Rashford from man-utd to chelsea
    r_tr = client.post(
        f"/api/worlds/{DEFAULT_WORLD_ID}/ingest/transfer",
        json={
            "footballer_id": "mun-fwd-1",
            "from_club_id": "man-utd",
            "to_club_id": "chelsea",
            "new_shirt": 19,
        },
    )
    assert r_tr.status_code == 200
    assert r_tr.json()["decision"] == "applied"

    # 3. Verify Round 3 frozen snapshot is 100% UNCHANGED, while future Round 4 snapshot reflects transfer!
    r_snap_after = client.get("/api/fixtures/fix-r3-mun-ars/snapshot")
    assert r_snap_after.status_code == 200
    assert [
        p["footballer_id"] for p in r_snap_after.json()["snapshot"]["home"]["starters"]
    ] == r3_mun_ids

    r_open_r4 = client.post(
        "/api/fixtures/fix-r4-mun-liv/sessions",
        json={"actor_id": "part-alex", "format_name": "5v5", "half_duration_ticks": 300},
    )
    assert r_open_r4.status_code == 200
    r4_mun_ids = [p["footballer_id"] for p in r_open_r4.json()["snapshot"]["home"]["starters"]]
    assert "mun-fwd-1" not in r4_mun_ids


def test_proposal_gate_parks_imports_for_represented_and_played_fixtures() -> None:
    app = create_app(db_path=":memory:", seed_demo=True)
    client = TestClient(app)

    # Attempt to import a real-world 0-5 result onto represented `fix-r3-mun-ars` (`played_vs_friend`)
    r_imp = client.post(
        f"/api/worlds/{DEFAULT_WORLD_ID}/ingest/result",
        json={
            "fixture_id": "fix-r3-mun-ars",
            "home_goals": 0,
            "away_goals": 5,
            "events": [{"event_type": "Goal", "footballer_id": "ars-fwd-1", "minute": 10}],
        },
    )
    assert r_imp.status_code == 200
    body = r_imp.json()
    assert body["decision"] == "parked"
    assert body["reason"] == "world_precedence"

    # Verify Saka's inherited goals did NOT absorb the parked goal (stays at 3 from R1 + R2)
    players = client.get(f"/api/worlds/{DEFAULT_WORLD_ID}/players").json()["players"]
    saka = next(p for p in players if p["footballer_id"] == "ars-fwd-1")
    assert saka["inherited"]["goals"] == 3
    assert saka["earned"]["goals"] == 0
    assert saka["combined"]["goals"] == 3


def test_wasm_to_psfc_replay_end_to_end_validation_idempotency_and_correction() -> None:
    assert resolve_replay_binary().is_file()
    app = create_app(db_path=":memory:", seed_demo=True)
    client = TestClient(app)

    # Open 5v5 session on Round 4 Man Utd vs Liverpool (short 360-tick = 6s halves for fast test)
    r_open = client.post(
        "/api/fixtures/fix-r4-mun-liv/sessions",
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
        "/api/admin/fixtures/fix-r4-mun-liv/correct",
        json={
            "new_home_goals": 3,
            "new_away_goals": 1,
            "new_events": [
                {
                    "event_type": "Goal",
                    "footballer_id": "mun-fwd-1",
                    "assist_footballer_id": "mun-mid-2",
                    "minute": 12,
                },
                {
                    "event_type": "Goal",
                    "footballer_id": "mun-fwd-1",
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
                    "footballer_id": "liv-fwd-1",
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
    # Man Utd had 4 inherited pts (W 2-1 vs CHE, D 1-1 vs LIV) + 3 earned pts (W 3-1 vs LIV) = 7 pts!
    assert mun_row["inherited_points"] == 4
    assert mun_row["earned_points"] == 3
    assert mun_row["points"] == 7

    players = client.get(f"/api/worlds/{DEFAULT_WORLD_ID}/players").json()["players"]
    rashford = next(p for p in players if p["footballer_id"] == "mun-fwd-1")
    # Rashford had 2 inherited goals (R1 + R2) + 2 earned goals (corrected R4) = 4 combined goals!
    assert rashford["inherited"]["goals"] == 2
    assert rashford["earned"]["goals"] == 2
    assert rashford["combined"]["goals"] == 4


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

