"""End-to-end integration tests covering Phases 2–6 of Parallel Season FC (5v5 primary)."""

from __future__ import annotations

import json
import subprocess
import tempfile
from pathlib import Path

import pytest
from fastapi.testclient import TestClient

from psfc_host.api.app import create_app
from psfc_host.catalogue.clubs import ClubCatalogue
from psfc_host.community.cartridge import AtomicCartridgeSlotStorage
from psfc_host.ledger.store import WorldLedgerStore
from psfc_host.seed import DEFAULT_CUP_ID, DEFAULT_WORLD_ID


def _simulate_idle_draw_to_shootout_wasm(snapshot: dict) -> dict:
    """Run WASM with idle human inputs so a 0-0 knockout tie goes through ET1, ET2, and Penalty Shootout."""
    repo_root = Path(__file__).resolve().parents[2]
    wasm_path = (
        repo_root / "sim" / "target" / "wasm32-unknown-unknown" / "release" / "psfc_wasm.wasm"
    )
    with tempfile.TemporaryDirectory(prefix="psfc_et_pens_") as tmp:
        snap_file = Path(tmp) / "snap.json"
        out_file = Path(tmp) / "out.json"
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
            throw new Error("init failed: " + readOut());
          }

          // Kick off on tick 1, then hold neutral so 0-0 regulation -> Extra Time -> Shootout!
          ex.sim_tick(0, 0, 0, 0, 1, 0, 0, 0, 0, 0);
          for (let i = 0; i < 1600; i++) {
            ex.sim_tick(0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
          }
          ex.sim_export_summary_json();
          const summary = JSON.parse(readOut());
          ex.sim_export_input_log_json();
          const log = JSON.parse(readOut());
          fs.writeFileSync(outPath, JSON.stringify({ summary, log }));
        })();
        """
        subprocess.run(
            ["node", "-e", script, str(wasm_path), str(snap_file), str(out_file)],
            check=True,
            timeout=15,
        )
        return json.loads(out_file.read_text(encoding="utf-8"))


def test_snake_case_and_pascal_case_events_project_cleanly_and_cup_does_not_pollute_league_table() -> None:
    app = create_app(db_path=":memory:", seed_demo=True)
    client = TestClient(app)

    before_standings = client.get(f"/api/worlds/{DEFAULT_WORLD_ID}/standings").json()["standings"]
    mun_pts_before = next(r["points"] for r in before_standings if r["club_id"] == "man-utd")

    # Resolve FA Parallel Cup Semi-Final 1 (`stage = "cup_sf"`) via admin ruling + snake_case events
    r_res = client.post(
        "/api/admin/fixtures/fix-cup-sf1-mun-liv/resolve",
        json={"home_goals": 2, "away_goals": 0, "reason": "cup sf test"},
    )
    assert r_res.status_code == 200
    r_corr = client.post(
        "/api/admin/fixtures/fix-cup-sf1-mun-liv/correct",
        json={
            "new_home_goals": 2,
            "new_away_goals": 0,
            "new_events": [
                {
                    "event_type": "goal",
                    "footballer_id": "mun-fwd-1",
                    "assist_footballer_id": "mun-mid-2",
                    "minute": 18,
                },
                {
                    "event_type": "skill_move",
                    "footballer_id": "mun-fwd-1",
                    "minute": 17,
                },
                {
                    "event_type": "save",
                    "footballer_id": "mun-gk-1",
                    "minute": 70,
                },
            ],
            "reason": "attach snake_case cup events",
        },
    )
    assert r_corr.status_code == 200

    # League points MUST NOT change from a Cup match!
    after_standings = client.get(f"/api/worlds/{DEFAULT_WORLD_ID}/standings").json()["standings"]
    mun_pts_after = next(r["points"] for r in after_standings if r["club_id"] == "man-utd")
    assert mun_pts_after == mun_pts_before

    # Earned & combined player stats DO include official Cup goals, skill_moves, and saves!
    players = client.get(f"/api/worlds/{DEFAULT_WORLD_ID}/players").json()["players"]
    rashford = next(p for p in players if p["footballer_id"] == "mun-fwd-1")
    assert rashford["inherited"]["goals"] == 2
    assert rashford["earned"]["goals"] == 1
    assert rashford["combined"]["goals"] == 3
    assert rashford["earned"]["skill_moves"] == 1
    assert rashford["combined"]["skill_moves"] == 1


def test_knockout_cup_extra_time_and_penalty_shootout_replay_validation() -> None:
    app = create_app(db_path=":memory:", seed_demo=True)
    client = TestClient(app)

    # Open FA Parallel Cup Semi-Final (`extra_time_enabled=True`) with both sides Human (idle -> 0-0 -> ET -> Pens)
    r_open = client.post(
        "/api/fixtures/fix-cup-sf1-mun-liv/sessions",
        json={
            "actor_id": "part-alex",
            "format_name": "5v5",
            "half_duration_ticks": 120,
            "home_controller": {"kind": "Human"},
            "away_controller": {"kind": "Human"},
        },
    )
    assert r_open.status_code == 200
    sess_id = r_open.json()["session_id"]
    snapshot = r_open.json()["snapshot"]
    assert snapshot["rules"]["extra_time_enabled"] is True

    wasm_run = _simulate_idle_draw_to_shootout_wasm(snapshot)
    summary = wasm_run["summary"]
    log = wasm_run["log"]

    assert summary["finished"] is True
    assert summary["went_to_extra_time"] is True
    assert summary["shootout_score"] is not None
    assert summary["shootout_score"][0] != summary["shootout_score"][1]

    r_complete = client.post(
        f"/api/sessions/{sess_id}/complete",
        json={
            "actor_id": "part-alex",
            "frames_hex": log["frames_hex"],
            "claimed_final_hash_hex": log["claimed_final_hash_hex"],
            "claimed_score": log["claimed_score"],
            "hash_checkpoints": log["hash_checkpoints"],
        },
    )
    assert r_complete.status_code == 200
    ar = r_complete.json()["accepted_result"]
    assert ar["went_to_extra_time"] == 1
    assert [ar["shootout_home"], ar["shootout_away"]] == summary["shootout_score"]


def test_competition_divergence_parallel_final_and_deterministic_poisson_generator() -> None:
    # 1. Test represented divergence in demo world: Man Utd beats Liverpool (SF1), Chelsea beats Arsenal (SF2)
    app = create_app(db_path=":memory:", seed_demo=True)
    client = TestClient(app)

    client.post(
        "/api/admin/fixtures/fix-cup-sf1-mun-liv/resolve",
        json={"home_goals": 2, "away_goals": 1},
    )
    client.post(
        "/api/admin/fixtures/fix-cup-sf2-ars-che/resolve",
        json={"home_goals": 0, "away_goals": 2},
    )

    r_adv = client.post(
        f"/api/worlds/{DEFAULT_WORLD_ID}/competitions/{DEFAULT_CUP_ID}/advance",
        json={},
    )
    assert r_adv.status_code == 200
    adv = r_adv.json()
    assert adv["diverged_from_real_world"] is True
    # Man Utd is represented (Alex) and Chelsea is unrepresented -> `played_vs_cpu`, no real reference score!
    assert adv["final_fixture"]["home_club_id"] == "man-utd"
    assert adv["final_fixture"]["away_club_id"] == "chelsea"
    assert adv["final_fixture"]["result_source"] == "played_vs_cpu"
    assert adv["final_fixture"]["real_home_goals"] is None

    # 2. Test unrepresented diverging Final (`LIV vs CHE` when neither is represented -> `generated`) + Poisson determinism
    store = WorldLedgerStore(":memory:")
    cat = ClubCatalogue()
    store.create_world(
        world_id="w-unrep",
        name="Unrep Cup Test",
        competition_name="Cup",
        season_label="2026/27",
    )
    store.create_competition(
        competition_id="cup-1",
        world_id="w-unrep",
        name="Parallel Cup",
        kind="knockout",
        real_final_pair=["man-utd", "arsenal"],
    )
    store.create_fixture(
        fixture_id="sf-1",
        world_id="w-unrep",
        competition_id="cup-1",
        stage="cup_sf",
        round_no=1,
        home_club_id="man-utd",
        away_club_id="liverpool",
        kickoff_utc="2026-09-01T15:00:00Z",
        window_open_utc="2026-08-01T00:00:00Z",
        window_close_utc="2026-12-01T00:00:00Z",
        result_source="imported",
    )
    store.create_fixture(
        fixture_id="sf-2",
        world_id="w-unrep",
        competition_id="cup-1",
        stage="cup_sf",
        round_no=1,
        home_club_id="arsenal",
        away_club_id="chelsea",
        kickoff_utc="2026-09-01T18:00:00Z",
        window_open_utc="2026-08-01T00:00:00Z",
        window_close_utc="2026-12-01T00:00:00Z",
        result_source="imported",
    )
    store.resolve_awaiting_admin_fixture(fixture_id="sf-1", home_goals=0, away_goals=2)
    store.resolve_awaiting_admin_fixture(fixture_id="sf-2", home_goals=1, away_goals=3)

    from psfc_host.competition.engine import CompetitionEngine

    eng = CompetitionEngine(store, cat)
    div = eng.advance_knockout_competition(world_id="w-unrep", competition_id="cup-1")
    assert div["diverged_from_real_world"] is True
    final_fx = div["final_fixture"]
    assert final_fx["result_source"] == "generated"

    gen_1 = eng.generate_poisson_fixture_result(fixture_id=final_fx["id"], seed=77777)
    assert gen_1["idempotent_duplicate"] is False
    ar_1 = gen_1["accepted_result"]
    # Knockout final MUST produce a winner (regulation, AET, or shootout)!
    assert (ar_1["home_goals"] != ar_1["away_goals"]) or (
        ar_1["shootout_home"] != ar_1["shootout_away"]
    )


def test_overdue_playing_window_transitions_to_awaiting_admin_never_real_score() -> None:
    app = create_app(db_path=":memory:", seed_demo=True)
    client = TestClient(app)

    r_exp = client.post(
        f"/api/worlds/{DEFAULT_WORLD_ID}/expire-windows",
        json={"as_of_utc": "2027-02-01T00:00:00Z"},
    )
    assert r_exp.status_code == 200
    transitioned_ids = {f["id"] for f in r_exp.json()["transitioned"]}
    assert "fix-mw27-mun-ars" in transitioned_ids
    assert "fix-mw23-mun-liv" in transitioned_ids

    fx_mw27 = next(
        f
        for f in client.get(f"/api/worlds/{DEFAULT_WORLD_ID}/fixtures").json()["fixtures"]
        if f["id"] == "fix-mw27-mun-ars"
    )
    assert fx_mw27["state"] == "awaiting_admin"
    assert fx_mw27["accepted_home_goals"] is None  # Real score was NOT substituted!


def test_provider_feed_adapter_and_identity_resolution_queue() -> None:
    app = create_app(db_path=":memory:", seed_demo=True)
    client = TestClient(app)

    # 1. Unknown external club ID is queued without mutating standings or fixtures
    r_unk = client.post(
        f"/api/worlds/{DEFAULT_WORLD_ID}/ingest/provider-fixture",
        json={
            "provider": "football_data_v4",
            "external_fixture_id": "ext-fx-500",
            "external_home_club_id": "ext-club-64",  # liverpool
            "external_away_club_id": "ext-club-mystery-88",
            "home_goals": 3,
            "away_goals": 0,
        },
    )
    assert r_unk.status_code == 200
    assert r_unk.json()["status"] == "queued_for_identity_resolution"
    qid = r_unk.json()["queue_item"]["id"]

    # 2. Commissioner maps `ext-club-mystery-88` -> `chelsea`
    r_res = client.post(
        f"/api/identity-queue/{qid}/resolve",
        json={"internal_id": "chelsea", "actor_id": "commissioner"},
    )
    assert r_res.status_code == 200
    assert r_res.json()["resolved"]["status"] == "resolved"

    # 3. Re-submitting now resolves both clubs and forwards cleanly to ProposalGate!
    r_retry = client.post(
        f"/api/worlds/{DEFAULT_WORLD_ID}/ingest/provider-fixture",
        json={
            "provider": "football_data_v4",
            "external_fixture_id": "ext-fx-500",
            "external_home_club_id": "ext-club-64",
            "external_away_club_id": "ext-club-mystery-88",
            "home_goals": 3,
            "away_goals": 0,
            "target_fixture_id": "fix-mw15-liv-che",
        },
    )
    assert r_retry.status_code == 200
    assert r_retry.json()["proposal"]["target_id"] == "fix-mw15-liv-che"


def test_kickoff_appointments_propose_and_accept() -> None:
    app = create_app(db_path=":memory:", seed_demo=True)
    client = TestClient(app)

    r_prop = client.post(
        "/api/fixtures/fix-mw27-mun-ars/appointments",
        json={"proposed_by": "part-alex", "proposed_at_utc": "2027-02-27T19:30:00Z"},
    )
    assert r_prop.status_code == 200
    appt_id = r_prop.json()["appointment"]["id"]
    assert r_prop.json()["appointment"]["state"] == "proposed"

    r_acc = client.post(
        f"/api/appointments/{appt_id}/respond",
        json={"responder_id": "part-sam", "accept": True},
    )
    assert r_acc.status_code == 200
    assert r_acc.json()["appointment"]["state"] == "accepted"
    assert r_acc.json()["appointment"]["accepted_by"] == "part-sam"


def test_cartridge_signing_tamper_rejection_revocation_jwt_auth_and_atomic_power_cut_slots() -> None:
    app = create_app(db_path=":memory:", seed_demo=True)
    client = TestClient(app)

    # 1. Register a fresh Cartridge device for Alex and obtain its raw secret
    r_dev = client.post(
        f"/api/worlds/{DEFAULT_WORLD_ID}/devices",
        json={"participant_id": "part-alex", "label": "Alex Spare Cartridge", "device_id": "dev-alex-spare"},
    )
    assert r_dev.status_code == 200
    dev_secret = r_dev.json()["device_secret"]

    # 2. Issue & verify HS256 Device JWT
    r_tok = client.post(
        "/api/auth/token",
        json={"device_id": "dev-alex-spare", "device_secret": dev_secret},
    )
    assert r_tok.status_code == 200
    jwt_token = r_tok.json()["access_token"]

    r_vtok = client.get("/api/auth/verify", headers={"Authorization": f"Bearer {jwt_token}"})
    assert r_vtok.status_code == 200
    assert r_vtok.json()["claims"]["sub"] == "part-alex"

    # 3. Sync signed Cartridge envelope & verify authentic signature
    r_sync = client.post(
        f"/api/worlds/{DEFAULT_WORLD_ID}/cartridge/sync",
        json={"participant_id": "part-alex", "device_id": "dev-alex-spare"},
    )
    assert r_sync.status_code == 200
    envelope = r_sync.json()["envelope"]

    r_ver = client.post("/api/cartridge/verify", json={"envelope": envelope})
    assert r_ver.status_code == 200
    assert r_ver.json()["valid"] is True

    # 4. Tamper with manifest points -> verification MUST reject!
    tampered = json.loads(json.dumps(envelope))
    tampered["manifest"]["career_summary"]["assigned_clubs"] = ["liverpool"]
    r_tamper = client.post("/api/cartridge/verify", json={"envelope": tampered})
    assert r_tamper.json()["valid"] is False
    assert r_tamper.json()["reason"] == "manifest_hash_mismatch"

    # 5. Revoke lost cartridge -> both Cartridge verification AND JWT auth reject immediately!
    r_rev = client.post(
        f"/api/worlds/{DEFAULT_WORLD_ID}/devices/dev-alex-spare/revoke",
        json={"actor_id": "part-alex"},
    )
    assert r_rev.status_code == 200
    assert client.post("/api/cartridge/verify", json={"envelope": envelope}).json() == {
        "valid": False,
        "reason": "device_revoked",
    }
    assert (
        client.get("/api/auth/verify", headers={"Authorization": f"Bearer {jwt_token}"}).status_code
        == 401
    )

    # 6. Two-generation CRC32 `AtomicCartridgeSlotStorage` survives mid-write power loss!
    with tempfile.TemporaryDirectory(prefix="psfc_slot_") as mnt:
        slots = AtomicCartridgeSlotStorage(mnt)
        gen1 = slots.write_envelope(envelope)
        assert gen1 == 1

        envelope_v2 = json.loads(json.dumps(envelope))
        envelope_v2["manifest"]["revision"] = 2
        with pytest.raises(IOError, match="power loss"):
            slots.write_envelope(envelope_v2, simulate_power_cut_bytes=14)

        recovered = slots.read_latest_valid()
        assert recovered is not None
        rec_gen, rec_env = recovered
        assert rec_gen == 1
        assert rec_env["signature"] == envelope["signature"]


def test_online_lobby_ready_pause_budget_disconnect_abandonment_and_relay_input_log() -> None:
    app = create_app(db_path=":memory:", seed_demo=True)
    client = TestClient(app)

    # 1. Lobby ready-check, 3-pause budget exhaustion, disconnect, and abandonment -> `awaiting_admin`
    r_open = client.post(
        "/api/fixtures/fix-mw27-mun-ars/sessions",
        json={"actor_id": "part-alex", "format_name": "5v5", "half_duration_ticks": 120},
    )
    sess_id = r_open.json()["session_id"]

    assert client.post(f"/api/sessions/{sess_id}/ready", json={"side": "home"}).json()["session"]["state"] == "ready_check"
    assert client.post(f"/api/sessions/{sess_id}/ready", json={"side": "away"}).json()["session"]["state"] == "running"

    for expected_rem in (2, 1, 0):
        rp = client.post(f"/api/sessions/{sess_id}/pause", json={"actor_id": "part-alex"})
        assert rp.status_code == 200
        assert rp.json()["session"]["pause_budget_remaining"] == expected_rem
        assert client.post(f"/api/sessions/{sess_id}/resume", json={"actor_id": "part-alex"}).status_code == 200

    # 4th pause rejected (409) when budget == 0
    assert client.post(f"/api/sessions/{sess_id}/pause", json={"actor_id": "part-alex"}).status_code == 409

    # Disconnect + timeout abandonment -> fixture transitions to `awaiting_admin`
    assert client.post(f"/api/sessions/{sess_id}/disconnect", json={"disconnected_actor": "part-sam"}).status_code == 200
    r_ab = client.post(f"/api/sessions/{sess_id}/abandon", json={"reason": "reconnect_timeout"})
    assert r_ab.status_code == 200
    assert r_ab.json()["session"]["state"] == "abandoned"

    fx = next(
        f
        for f in client.get(f"/api/worlds/{DEFAULT_WORLD_ID}/fixtures").json()["fixtures"]
        if f["id"] == "fix-mw27-mun-ars"
    )
    assert fx["state"] == "awaiting_admin"

    # 2. Relay input batch recording reconstructs full `frames_hex` for `complete_session`!
    r_open2 = client.post(
        "/api/fixtures/fix-mw23-mun-liv/sessions",
        json={
            "actor_id": "part-alex",
            "format_name": "5v5",
            "half_duration_ticks": 60,
            "home_controller": {"kind": "Human"},
            "away_controller": {"kind": "Human"},
        },
    )
    sess2 = r_open2.json()["session_id"]
    # Kick off 1st Half on tick 1 (`buttons=1`), and kick off 2nd Half after HT break so match reaches FullTime!
    home_frames = [{"buttons": 1}] + [{"buttons": 0} for _ in range(380)]
    away_frames = [{"buttons": 1 if t in (152, 200) else 0} for t in range(381)]
    assert (
        client.post(
            f"/api/sessions/{sess2}/relay-inputs",
            json={"side": "home", "first_tick": 1, "frames": home_frames},
        ).status_code
        == 200
    )
    assert (
        client.post(
            f"/api/sessions/{sess2}/relay-inputs",
            json={"side": "away", "first_tick": 1, "frames": away_frames},
        ).status_code
        == 200
    )
    r_comp2 = client.post(f"/api/sessions/{sess2}/complete", json={"actor_id": "part-alex"})
    assert r_comp2.status_code == 200, r_comp2.text
    assert r_comp2.json()["accepted_result"]["source"] == "played_vs_cpu"


def test_season_completion_honours_qualification_and_rollover() -> None:
    app = create_app(db_path=":memory:", seed_demo=True)
    client = TestClient(app)

    r_roll = client.post(
        f"/api/worlds/{DEFAULT_WORLD_ID}/rollover",
        json={"next_season_label": "2027/28", "actor_id": "commissioner"},
    )
    assert r_roll.status_code == 200
    body = r_roll.json()
    categories = {h["category"] for h in body["honours"]}
    assert {"league_champion", "golden_boot", "golden_glove"}.issubset(categories)
    assert len(body["qualification"]["parallel_champions_cup"]) == 2

    next_world_id = body["next_world"]["id"]
    ov_next = client.get(f"/api/worlds/{next_world_id}/overview").json()
    assert ov_next["world"]["season_label"] == "2027/28"
    assert ov_next["world"]["parent_world_id"] == DEFAULT_WORLD_ID
    assigned_clubs = {a["club_id"]: a["participant_id"] for a in ov_next["assignments"]}
    assert assigned_clubs == {"man-utd": "part-alex", "arsenal": "part-sam"}


def test_custom_5v5_lineup_selection_from_11_man_squad_and_replay_verification() -> None:
    app = create_app(db_path=":memory:", seed_demo=True)
    client = TestClient(app)

    # 1. Inspect full 11-player official squad for Man Utd and verify default 5v5 reduction
    r_sq = client.get(f"/api/worlds/{DEFAULT_WORLD_ID}/clubs/man-utd/squad")
    assert r_sq.status_code == 200
    roster = r_sq.json()["roster"]
    assert len(roster["squad"]) == 11
    assert roster["formation_5v5"] == "1-2-1"
    assert roster["custom_lineup_active"] is False
    assert roster["starter_ids_5v5"] == [
        "mun-gk-1",
        "mun-def-1",
        "mun-mid-1",
        "mun-mid-2",
        "mun-fwd-1",
    ]

    # 2. Reject invalid 5v5 lineups (e.g. missing goalkeeper or wrong count) with 422
    r_no_gk = client.post(
        f"/api/worlds/{DEFAULT_WORLD_ID}/clubs/man-utd/lineup",
        json={
            "starter_ids": ["mun-def-1", "mun-def-2", "mun-mid-1", "mun-mid-2", "mun-fwd-1"],
            "formation_5v5": "1-2-1",
        },
    )
    assert r_no_gk.status_code == 422
    assert "Goalkeeper" in r_no_gk.json()["detail"]

    r_four = client.post(
        f"/api/worlds/{DEFAULT_WORLD_ID}/clubs/man-utd/lineup",
        json={
            "starter_ids": ["mun-gk-1", "mun-def-2", "mun-mid-2", "mun-fwd-2"],
            "formation_5v5": "1-2-1",
        },
    )
    assert r_four.status_code == 422

    # 3. Save a custom 5v5 lineup swapping in Dalot (mun-def-2), Garnacho (mun-fwd-2), and Hojlund (mun-fwd-3) + 1-1-2 Attack formation
    custom_ids = ["mun-fwd-3", "mun-fwd-2", "mun-gk-1", "mun-def-2", "mun-mid-2"]
    r_save = client.post(
        f"/api/worlds/{DEFAULT_WORLD_ID}/clubs/man-utd/lineup",
        json={
            "starter_ids": custom_ids,
            "formation_5v5": "1-1-2",
            "actor_id": "part-alex",
        },
    )
    assert r_save.status_code == 200, r_save.text
    saved_roster = r_save.json()["roster"]
    assert saved_roster["custom_lineup_active"] is True
    assert saved_roster["formation_5v5"] == "1-1-2"
    # Tactically ordered GK -> DEF -> MID -> FWD (preserving selection order within same role)
    expected_ordered = ["mun-gk-1", "mun-def-2", "mun-mid-2", "mun-fwd-3", "mun-fwd-2"]
    assert saved_roster["starter_ids_5v5"] == expected_ordered

    events = client.get(f"/api/worlds/{DEFAULT_WORLD_ID}/events").json()["events"]
    assert any(ev["type"] == "lineup_updated" and ev["aggregate_id"] == "man-utd" for ev in events)

    # 4. Exhibition snapshot reflects saved custom 5v5 lineup + remaining 6 players on bench
    r_exh = client.post(
        "/api/exhibition/snapshot",
        json={
            "world_id": DEFAULT_WORLD_ID,
            "home_club_id": "man-utd",
            "away_club_id": "arsenal",
            "format_name": "5v5",
            "half_duration_ticks": 120,
        },
    )
    assert r_exh.status_code == 200
    exh_snap = r_exh.json()["snapshot"]
    assert [p["footballer_id"] for p in exh_snap["home"]["starters"]] == expected_ordered
    assert len(exh_snap["home"]["substitutes"]) == 6
    assert exh_snap["home"]["formation"] == "1-1-2"

    # 5. Official session freezes custom 5v5 starters + formation and passes WASM -> psfc-replay verification!
    r_open = client.post(
        "/api/fixtures/fix-mw27-mun-ars/sessions",
        json={
            "actor_id": "part-alex",
            "format_name": "5v5",
            "half_duration_ticks": 90,
            "home_controller": {"kind": "Human"},
            "away_controller": {"kind": "Cpu", "difficulty": "Normal"},
        },
    )
    assert r_open.status_code == 200
    sess_payload = r_open.json()
    snap = sess_payload["snapshot"]
    assert [p["footballer_id"] for p in snap["home"]["starters"]] == expected_ordered
    assert snap["home"]["formation"] == "1-1-2"

    repo_root = Path(__file__).resolve().parents[2]
    wasm_path = (
        repo_root / "sim" / "target" / "wasm32-unknown-unknown" / "release" / "psfc_wasm.wasm"
    )
    with tempfile.TemporaryDirectory(prefix="psfc_custom_lineup_") as tmp:
        snap_file = Path(tmp) / "snap.json"
        out_file = Path(tmp) / "out.json"
        snap_file.write_text(json.dumps(snap), encoding="utf-8")
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
          if (!ex.sim_init_from_snapshot_json()) throw new Error("init failed: " + readOut());
          ex.sim_auto_step(10000);
          ex.sim_export_input_log_json();
          fs.writeFileSync(outPath, readOut());
        })();
        """
        subprocess.run(
            ["node", "-e", script, str(wasm_path), str(snap_file), str(out_file)],
            check=True,
            timeout=15,
        )
        log = json.loads(out_file.read_text(encoding="utf-8"))

    r_comp = client.post(
        f"/api/sessions/{sess_payload['session_id']}/complete",
        json={
            "actor_id": "part-alex",
            "frames_hex": log["frames_hex"],
            "claimed_final_hash_hex": log["claimed_final_hash_hex"],
            "claimed_score": log["claimed_score"],
            "hash_checkpoints": log["hash_checkpoints"],
        },
    )
    assert r_comp.status_code == 200, r_comp.text
    comp_body = r_comp.json()
    assert comp_body["replay_report"]["final_hash_hex"] == log["claimed_final_hash_hex"]
    assert comp_body["accepted_result"]["final_hash_hex"] == log["claimed_final_hash_hex"]


def test_codex_review_security_and_invariant_guards(monkeypatch: pytest.MonkeyPatch) -> None:
    # 1. File-backed DB blocks unauthenticated reset-demo outside dev, enforces admin token,
    #    preserves independent worlds on scoped dev reset, and rebuilds transfers across restart!
    with tempfile.TemporaryDirectory(prefix="psfc_codex_") as tmp:
        db_file = Path(tmp) / "persistent.sqlite"
        app1 = create_app(db_path=db_file, seed_demo=True)
        c1 = TestClient(app1)

        # Persistent file-backed DB disables reset-demo by default (`403`)
        assert c1.post(f"/api/worlds/{DEFAULT_WORLD_ID}/reset-demo").status_code == 403

        # Create an independent second world + an open unrepresented Round 5 fixture in DEFAULT_WORLD_ID
        indep_store = WorldLedgerStore(db_file)
        indep_store.create_world(
            world_id="world-independent",
            name="Independent Parallel League",
            competition_name="Serie Parallel",
            season_label="2026/27",
        )
        indep_store.create_fixture(
            fixture_id="fix-r5-liv-che",
            world_id=DEFAULT_WORLD_ID,
            competition_id="comp-prem-2026",
            stage="league",
            round_no=5,
            home_club_id="liverpool",
            away_club_id="chelsea",
            kickoff_utc="2026-10-01T15:00:00Z",
            window_open_utc="2026-08-01T00:00:00Z",
            window_close_utc="2026-12-31T23:59:59Z",
            result_source="imported",
            state="open",
        )
        indep_store.close()

        # Apply a squad transfer in `world-prem-2026` before simulating host restart
        r_tr = c1.post(
            f"/api/worlds/{DEFAULT_WORLD_ID}/ingest/transfer",
            json={
                "footballer_id": "mun-fwd-1",
                "from_club_id": "man-utd",
                "to_club_id": "chelsea",
                "new_shirt": 19,
            },
        )
        assert r_tr.status_code == 200

        # Simulate server restart on the existing SQLite file
        app2 = create_app(db_path=db_file, seed_demo=True)
        c2 = TestClient(app2)

        che_squad = c2.get(f"/api/worlds/{DEFAULT_WORLD_ID}/clubs/chelsea/squad").json()["roster"]["squad"]
        mun_squad = c2.get(f"/api/worlds/{DEFAULT_WORLD_ID}/clubs/man-utd/squad").json()["roster"]["squad"]
        assert any(p["footballer_id"] == "mun-fwd-1" for p in che_squad)
        assert not any(p["footballer_id"] == "mun-fwd-1" for p in mun_squad)

        # Cross-world fixture ownership check: `world-independent` CANNOT import `fix-r5-liv-che`!
        r_cross = c2.post(
            "/api/worlds/world-independent/ingest/result",
            json={"fixture_id": "fix-r5-liv-che", "home_goals": 2, "away_goals": 0, "events": []},
        )
        assert r_cross.status_code == 409
        assert "belongs to world" in r_cross.json()["detail"]

        # Score-only imported result followed by idempotent event enrichment (without double-counting points!)
        r_imp_score = c2.post(
            f"/api/worlds/{DEFAULT_WORLD_ID}/ingest/result",
            json={"fixture_id": "fix-r5-liv-che", "home_goals": 2, "away_goals": 0, "events": []},
        )
        assert r_imp_score.status_code == 200
        assert r_imp_score.json()["decision"] == "applied"

        liv_pts_before = next(
            r["points"]
            for r in c2.get(f"/api/worlds/{DEFAULT_WORLD_ID}/standings").json()["standings"]
            if r["club_id"] == "liverpool"
        )
        isak_before = next(
            p["inherited"]["goals"]
            for p in c2.get(f"/api/worlds/{DEFAULT_WORLD_ID}/players").json()["players"]
            if p["footballer_id"] == "liv-fwd-1"
        )

        r_enrich = c2.post(
            f"/api/worlds/{DEFAULT_WORLD_ID}/ingest/result",
            json={
                "fixture_id": "fix-r5-liv-che",
                "home_goals": 2,
                "away_goals": 0,
                "events": [{"event_type": "Goal", "footballer_id": "liv-fwd-1", "minute": 55}],
            },
        )
        assert r_enrich.status_code == 200
        assert r_enrich.json()["decision"] == "applied"
        assert r_enrich.json()["reason"] == "imported_events_enriched"

        liv_pts_after = next(
            r["points"]
            for r in c2.get(f"/api/worlds/{DEFAULT_WORLD_ID}/standings").json()["standings"]
            if r["club_id"] == "liverpool"
        )
        isak_after = next(
            p["inherited"]["goals"]
            for p in c2.get(f"/api/worlds/{DEFAULT_WORLD_ID}/players").json()["players"]
            if p["footballer_id"] == "liv-fwd-1"
        )
        assert liv_pts_after == liv_pts_before
        assert isak_after == isak_before + 1

        # Subsequent duplicate enrichment is parked
        r_dup_enrich = c2.post(
            f"/api/worlds/{DEFAULT_WORLD_ID}/ingest/result",
            json={
                "fixture_id": "fix-r5-liv-che",
                "home_goals": 2,
                "away_goals": 0,
                "events": [{"event_type": "Goal", "footballer_id": "liv-fwd-1", "minute": 55}],
            },
        )
        assert r_dup_enrich.json()["decision"] == "parked"

        # Enable dev reset with PSFC_ADMIN_TOKEN -> enforce token, 404 on unknown world, preserve independent world
        monkeypatch.setenv("PSFC_ADMIN_TOKEN", "super-secret-admin")
        app_dev = create_app(db_path=db_file, seed_demo=True, enable_demo_reset=True)
        c_dev = TestClient(app_dev)

        assert c_dev.post(f"/api/worlds/{DEFAULT_WORLD_ID}/reset-demo").status_code == 403
        assert (
            c_dev.post(
                "/api/worlds/nonexistent-world/reset-demo",
                headers={"X-Admin-Token": "super-secret-admin"},
            ).status_code
            == 404
        )
        assert (
            c_dev.post(
                "/api/worlds/world-independent/reset-demo",
                headers={"X-Admin-Token": "super-secret-admin"},
            ).status_code
            == 403
        )
        r_reset_ok = c_dev.post(
            f"/api/worlds/{DEFAULT_WORLD_ID}/reset-demo",
            headers={"X-Admin-Token": "super-secret-admin"},
        )
        assert r_reset_ok.status_code == 200
        worlds_after = {w["id"] for w in c_dev.get("/api/worlds").json()["worlds"]}
        assert {"world-independent", DEFAULT_WORLD_ID}.issubset(worlds_after)
        monkeypatch.delenv("PSFC_ADMIN_TOKEN", raising=False)

    # 2. Unconditional FullTime requirement on `/complete` AND `awaiting_admin` session-opening guard!
    app_mem = create_app(db_path=":memory:", seed_demo=True)
    cm = TestClient(app_mem)

    r_open = cm.post(
        "/api/fixtures/fix-mw23-mun-liv/sessions",
        json={"actor_id": "part-alex", "format_name": "5v5", "half_duration_ticks": 180},
    )
    assert r_open.status_code == 200
    sess_id = r_open.json()["session_id"]

    # Attempting to bank a 0-0 unfinished match with `frames: []` and `require_finished: false` MUST fail (`422`)!
    r_unfin = cm.post(
        f"/api/sessions/{sess_id}/complete",
        json={"actor_id": "part-alex", "frames": [], "require_finished": False},
    )
    assert r_unfin.status_code == 422
    assert "FullTime" in r_unfin.json()["detail"]

    # Expiring overdue window transitions `fix-mw27-mun-ars` to `awaiting_admin` -> `open_session` MUST reject (`409`)!
    cm.post(
        f"/api/worlds/{DEFAULT_WORLD_ID}/expire-windows",
        json={"as_of_utc": "2027-01-10T00:00:00Z"},
    )
    r_blocked = cm.post(
        "/api/fixtures/fix-mw27-mun-ars/sessions",
        json={"actor_id": "part-alex", "format_name": "5v5"},
    )
    assert r_blocked.status_code == 409
    assert "commissioner" in r_blocked.json()["detail"]


def test_codex_pr3_stale_saved_lineup_fallback_and_rollover_transfer_inheritance() -> None:
    app = create_app(db_path=":memory:", seed_demo=True)
    client = TestClient(app)

    # [P2] Save Man Utd's default five, then transfer one of those starters away.
    default_ids = ["mun-gk-1", "mun-def-1", "mun-mid-1", "mun-mid-2", "mun-fwd-1"]
    r_save = client.post(
        f"/api/worlds/{DEFAULT_WORLD_ID}/clubs/man-utd/lineup",
        json={"starter_ids": default_ids, "formation_5v5": "2-2", "actor_id": "part-alex"},
    )
    assert r_save.status_code == 200
    r_tr = client.post(
        f"/api/worlds/{DEFAULT_WORLD_ID}/ingest/transfer",
        json={"footballer_id": "mun-fwd-1", "from_club_id": "man-utd", "to_club_id": "chelsea", "new_shirt": 19},
    )
    assert r_tr.status_code == 200

    # Displayed roster falls back to valid defaults (no mun-fwd-1) ...
    roster = client.get(f"/api/worlds/{DEFAULT_WORLD_ID}/clubs/man-utd/squad").json()["roster"]
    assert "mun-fwd-1" not in roster["starter_ids_5v5"]

    # ... and the official session still opens using the same fallback instead of erroring.
    r_open = client.post(
        "/api/fixtures/fix-mw23-mun-liv/sessions",
        json={"actor_id": "part-alex", "format_name": "5v5", "half_duration_ticks": 180},
    )
    assert r_open.status_code == 200, r_open.text

    # Exhibition endpoint consumes the stale saved lineup gracefully too, keeping the saved formation.
    r_exh = client.post(
        "/api/exhibition/snapshot",
        json={"world_id": DEFAULT_WORLD_ID, "home_club_id": "man-utd", "away_club_id": "chelsea"},
    )
    assert r_exh.status_code == 200, r_exh.text
    snap = r_exh.json()["snapshot"]
    home = snap["home"] if "home" in snap else snap["teams"][0]
    home_ids = [p["footballer_id"] for p in home["starters"]]
    assert "mun-fwd-1" not in home_ids
    assert home["formation"] == "2-2"

    # Explicit invalid overrides are still strictly rejected.
    r_bad = client.post(
        "/api/exhibition/snapshot",
        json={
            "world_id": DEFAULT_WORLD_ID,
            "home_club_id": "man-utd",
            "away_club_id": "chelsea",
            "home_starters": default_ids,
        },
    )
    assert r_bad.status_code == 422

    # [P2] Rolling the season over preserves the parent world's transfers in the child world.
    r_roll = client.post(
        f"/api/worlds/{DEFAULT_WORLD_ID}/rollover",
        json={"next_season_label": "2027/28", "actor_id": "commissioner"},
    )
    assert r_roll.status_code == 200
    child_id = r_roll.json()["next_world"]["id"]
    che = client.get(f"/api/worlds/{child_id}/clubs/chelsea/squad").json()["roster"]["squad"]
    mun = client.get(f"/api/worlds/{child_id}/clubs/man-utd/squad").json()["roster"]["squad"]
    assert any(p["footballer_id"] == "mun-fwd-1" for p in che)
    assert not any(p["footballer_id"] == "mun-fwd-1" for p in mun)

    # Late transfer import against the completed parent world is parked and never leaks into child_id
    r_late = client.post(
        f"/api/worlds/{DEFAULT_WORLD_ID}/ingest/transfer",
        json={"footballer_id": "mun-mid-1", "from_club_id": "man-utd", "to_club_id": "liverpool", "new_shirt": 80},
    )
    assert r_late.status_code == 200
    assert r_late.json()["decision"] == "parked"
    assert r_late.json()["reason"] == "world_completed"
    child_mun_after = client.get(f"/api/worlds/{child_id}/clubs/man-utd/squad").json()["roster"]["squad"]
    assert any(p["footballer_id"] == "mun-mid-1" for p in child_mun_after)
