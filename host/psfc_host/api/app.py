"""FastAPI World API for Parallel Season FC + 5v5 Browser/WASM Client host."""

from __future__ import annotations

import os
from pathlib import Path
from typing import Any

from fastapi import FastAPI, HTTPException
from fastapi.responses import FileResponse, HTMLResponse, Response
from fastapi.staticfiles import StaticFiles
from pydantic import BaseModel, Field

from psfc_host.catalogue.clubs import GAMEPLAY_VERSION, ClubCatalogue
from psfc_host.ingest.proposal_gate import ProposalGate
from psfc_host.ledger.projections import project_player_stats, project_standings
from psfc_host.ledger.store import WorldLedgerStore
from psfc_host.match.validator import (
    MatchSessionService,
    ReplayValidationError,
    resolve_replay_binary,
)
from psfc_host.seed import DEFAULT_WORLD_ID, ensure_demo_world

REPO_ROOT = Path(__file__).resolve().parents[3]
WEB_DIR = Path(__file__).resolve().parents[1] / "web"
STATIC_DIR = WEB_DIR / "static"
BUILT_WASM_PATH = (
    REPO_ROOT / "sim" / "target" / "wasm32-unknown-unknown" / "release" / "psfc_wasm.wasm"
)


class ClubTakeoverRequest(BaseModel):
    participant_id: str = Field(..., min_length=2)
    display_name: str = Field(..., min_length=1)
    club_id: str = Field(..., min_length=2)
    effective_from_round: int = Field(default=3, ge=1)


class OpenSessionRequest(BaseModel):
    actor_id: str = "part-alex"
    format_name: str = Field(default="5v5", pattern="^(5v5|11v11)$")
    half_duration_ticks: int | None = Field(default=None, ge=60, le=36_000)
    home_controller: dict[str, Any] | None = None
    away_controller: dict[str, Any] | None = None
    seed: int | None = None


class CompleteSessionRequest(BaseModel):
    actor_id: str = "part-alex"
    frames_hex: str | None = None
    frames: list[Any] | None = None
    claimed_final_hash_hex: str | None = None
    claimed_score: list[int] | None = None
    hash_checkpoints: list[dict[str, Any]] = Field(default_factory=list)
    require_finished: bool = True


class IngestResultRequest(BaseModel):
    fixture_id: str
    home_goals: int = Field(ge=0)
    away_goals: int = Field(ge=0)
    events: list[dict[str, Any]] = Field(default_factory=list)
    provider: str = "real_football_feed_v1"


class IngestTransferRequest(BaseModel):
    footballer_id: str
    from_club_id: str
    to_club_id: str
    new_shirt: int = Field(default=99, ge=1, le=99)
    provider: str = "real_football_feed_v1"


class CorrectResultRequest(BaseModel):
    new_home_goals: int = Field(ge=0)
    new_away_goals: int = Field(ge=0)
    new_events: list[dict[str, Any]] = Field(default_factory=list)
    actor_id: str = "commissioner"
    reason: str = "commissioner audit ruling"


def create_app(db_path: str | Path | None = None, seed_demo: bool = True) -> FastAPI:
    resolved_db = db_path or os.environ.get("PSFC_DB_PATH", "/tmp/psfc_world.db")

    app = FastAPI(
        title="classicFutbol — Parallel Season FC World API",
        version=GAMEPLAY_VERSION,
        description="Event-sourced Parallel Season host, Proposal Gate, and deterministic 5v5/11v11 replay validator.",
    )

    state_holder: dict[str, Any] = {}

    def _init_services(reset_file: bool = False) -> None:
        if reset_file and str(resolved_db) != ":memory:":
            old = state_holder.get("store")
            if old is not None:
                old.close()
            for suffix in ("", "-wal", "-shm"):
                p = Path(f"{resolved_db}{suffix}")
                if p.exists():
                    p.unlink()

        store = WorldLedgerStore(resolved_db)
        catalogue = ClubCatalogue()
        gate = ProposalGate(store, catalogue)
        sessions = MatchSessionService(store, catalogue)
        if seed_demo:
            ensure_demo_world(store, catalogue)
        state_holder["store"] = store
        state_holder["catalogue"] = catalogue
        state_holder["gate"] = gate
        state_holder["sessions"] = sessions

    _init_services(reset_file=False)

    def _clubs_summary(cat: ClubCatalogue) -> dict[str, dict[str, Any]]:
        return {
            cid: {
                "club_id": c.club_id,
                "name": c.name,
                "short_name": c.short_name,
                "country": c.country,
                "stadium": c.stadium,
                "primary_color": c.primary_color,
                "secondary_color": c.secondary_color,
            }
            for cid, c in cat.clubs.items()
        }

    @app.get("/api/health")
    def health() -> dict[str, Any]:
        replay_ok = False
        replay_path = None
        try:
            rp = resolve_replay_binary()
            replay_ok = True
            replay_path = str(rp)
        except FileNotFoundError:
            pass
        wasm_ok = (STATIC_DIR / "wasm" / "psfc_wasm.wasm").is_file() or BUILT_WASM_PATH.is_file()
        return {
            "status": "ok",
            "gameplay_version": GAMEPLAY_VERSION,
            "default_format": "5v5 (4 outfield + 1 GK)",
            "replay_verifier_ready": replay_ok,
            "replay_binary": replay_path,
            "wasm_client_ready": wasm_ok,
        }

    @app.get("/api/worlds")
    def list_worlds() -> dict[str, Any]:
        store: WorldLedgerStore = state_holder["store"]
        return {"worlds": store.list_worlds(), "default_world_id": DEFAULT_WORLD_ID}

    @app.get("/api/worlds/{world_id}/standings")
    def get_standings(world_id: str) -> dict[str, Any]:
        store: WorldLedgerStore = state_holder["store"]
        cat: ClubCatalogue = state_holder["catalogue"]
        try:
            world = store.get_world(world_id)
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        events = store.list_events(world_id)
        rep = set(store.represented_clubs(world_id).keys())
        proj = project_standings(events, _clubs_summary(cat), represented_club_ids=rep)
        return {"world": world, **proj}

    @app.get("/api/worlds/{world_id}/players")
    def get_players(world_id: str) -> dict[str, Any]:
        store: WorldLedgerStore = state_holder["store"]
        cat: ClubCatalogue = state_holder["catalogue"]
        try:
            store.get_world(world_id)
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        events = store.list_events(world_id)
        stats = project_player_stats(events, cat.all_footballers())
        return {"world_id": world_id, "players": stats}

    @app.get("/api/worlds/{world_id}/fixtures")
    def get_fixtures(world_id: str) -> dict[str, Any]:
        store: WorldLedgerStore = state_holder["store"]
        cat: ClubCatalogue = state_holder["catalogue"]
        try:
            store.get_world(world_id)
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        clubs = _clubs_summary(cat)
        fixtures = store.list_fixtures(world_id)
        for f in fixtures:
            f["home_club"] = clubs.get(f["home_club_id"])
            f["away_club"] = clubs.get(f["away_club_id"])
        return {"world_id": world_id, "fixtures": fixtures}

    @app.get("/api/worlds/{world_id}/events")
    def get_events(world_id: str, since_seq: int = 0) -> dict[str, Any]:
        store: WorldLedgerStore = state_holder["store"]
        return {"world_id": world_id, "events": store.list_events(world_id, since_seq=since_seq)}

    @app.get("/api/worlds/{world_id}/proposals")
    def get_proposals(world_id: str) -> dict[str, Any]:
        gate: ProposalGate = state_holder["gate"]
        return {"world_id": world_id, "proposals": gate.list_proposals(world_id)}

    @app.get("/api/worlds/{world_id}/overview")
    def get_world_overview(world_id: str) -> dict[str, Any]:
        store: WorldLedgerStore = state_holder["store"]
        cat: ClubCatalogue = state_holder["catalogue"]
        gate: ProposalGate = state_holder["gate"]
        try:
            world = store.get_world(world_id)
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc

        clubs = _clubs_summary(cat)
        events = store.list_events(world_id)
        assignments = store.list_assignments(world_id)
        rep = {a["club_id"] for a in assignments}
        standings_proj = project_standings(events, clubs, represented_club_ids=rep)
        player_stats = project_player_stats(events, cat.all_footballers())
        fixtures = store.list_fixtures(world_id)
        for f in fixtures:
            f["home_club"] = clubs.get(f["home_club_id"])
            f["away_club"] = clubs.get(f["away_club_id"])

        return {
            "world": world,
            "clubs": list(clubs.values()),
            "assignments": assignments,
            "standings": standings_proj["standings"],
            "players": player_stats[:16],
            "fixtures": fixtures,
            "recent_events": list(reversed(events[-25:])),
            "total_events": len(events),
            "proposals": gate.list_proposals(world_id)[:12],
        }

    @app.post("/api/worlds/{world_id}/assignments")
    def assign_club_endpoint(world_id: str, req: ClubTakeoverRequest) -> dict[str, Any]:
        store: WorldLedgerStore = state_holder["store"]
        cat: ClubCatalogue = state_holder["catalogue"]
        if req.club_id not in cat.clubs:
            raise HTTPException(status_code=404, detail=f"Unknown club '{req.club_id}'")
        store.register_participant(req.participant_id, req.display_name, world_id)
        try:
            asgn = store.assign_club(
                assignment_id=f"asgn-{req.club_id}-{req.participant_id}",
                world_id=world_id,
                club_id=req.club_id,
                participant_id=req.participant_id,
                effective_from_round=req.effective_from_round,
            )
        except ValueError as exc:
            raise HTTPException(status_code=409, detail=str(exc)) from exc
        return {"assignment": asgn}

    @app.post("/api/fixtures/{fixture_id}/sessions")
    def open_fixture_session(fixture_id: str, req: OpenSessionRequest) -> dict[str, Any]:
        sessions: MatchSessionService = state_holder["sessions"]
        try:
            return sessions.open_session(
                fixture_id=fixture_id,
                actor_id=req.actor_id,
                format_name=req.format_name,
                half_duration_ticks=req.half_duration_ticks,
                home_controller=req.home_controller,
                away_controller=req.away_controller,
                seed=req.seed,
            )
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        except ValueError as exc:
            raise HTTPException(status_code=409, detail=str(exc)) from exc

    @app.get("/api/fixtures/{fixture_id}/snapshot")
    def get_fixture_snapshot(fixture_id: str) -> dict[str, Any]:
        store: WorldLedgerStore = state_holder["store"]
        snap = store.get_frozen_snapshot(fixture_id)
        if snap is None:
            raise HTTPException(status_code=404, detail=f"No frozen snapshot for '{fixture_id}'")
        return snap

    @app.post("/api/sessions/{session_id}/complete")
    def complete_fixture_session(session_id: str, req: CompleteSessionRequest) -> dict[str, Any]:
        sessions: MatchSessionService = state_holder["sessions"]
        try:
            return sessions.complete_session(
                session_id=session_id,
                frames_hex=req.frames_hex,
                frames=req.frames,
                claimed_final_hash_hex=req.claimed_final_hash_hex,
                claimed_score=req.claimed_score,
                hash_checkpoints=req.hash_checkpoints,
                require_finished=req.require_finished,
                actor_id=req.actor_id,
            )
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        except ReplayValidationError as exc:
            raise HTTPException(status_code=422, detail=str(exc)) from exc
        except ValueError as exc:
            raise HTTPException(status_code=409, detail=str(exc)) from exc

    @app.post("/api/worlds/{world_id}/ingest/result")
    def ingest_external_result(world_id: str, req: IngestResultRequest) -> dict[str, Any]:
        gate: ProposalGate = state_holder["gate"]
        try:
            return gate.submit_fixture_result_import(
                world_id=world_id,
                fixture_id=req.fixture_id,
                home_goals=req.home_goals,
                away_goals=req.away_goals,
                events=req.events,
                provider=req.provider,
            )
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc

    @app.post("/api/worlds/{world_id}/ingest/transfer")
    def ingest_external_transfer(world_id: str, req: IngestTransferRequest) -> dict[str, Any]:
        gate: ProposalGate = state_holder["gate"]
        try:
            return gate.submit_transfer_import(
                world_id=world_id,
                footballer_id=req.footballer_id,
                from_club_id=req.from_club_id,
                to_club_id=req.to_club_id,
                new_shirt=req.new_shirt,
                provider=req.provider,
            )
        except (KeyError, ValueError) as exc:
            raise HTTPException(status_code=400, detail=str(exc)) from exc

    @app.post("/api/admin/fixtures/{fixture_id}/correct")
    def admin_correct_result(fixture_id: str, req: CorrectResultRequest) -> dict[str, Any]:
        store: WorldLedgerStore = state_holder["store"]
        try:
            res = store.correct_result_admin(
                fixture_id=fixture_id,
                new_home_goals=req.new_home_goals,
                new_away_goals=req.new_away_goals,
                new_events=req.new_events,
                actor_id=req.actor_id,
                reason=req.reason,
            )
            return {"accepted_result": res}
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        except ValueError as exc:
            raise HTTPException(status_code=409, detail=str(exc)) from exc

    @app.post("/api/worlds/{world_id}/reset-demo")
    def reset_demo_world(world_id: str) -> dict[str, Any]:
        if str(resolved_db) == ":memory:":
            _init_services(reset_file=False)
        else:
            _init_services(reset_file=True)
        return {"status": "reset", "world_id": world_id}

    # --- Browser / WASM 5v5 Match Client Routes ---

    @app.get("/wasm/psfc_wasm.wasm")
    def serve_wasm() -> Response:
        staged = STATIC_DIR / "wasm" / "psfc_wasm.wasm"
        target = staged if staged.is_file() else BUILT_WASM_PATH
        if not target.is_file():
            raise HTTPException(status_code=404, detail="psfc_wasm.wasm not built yet")
        return FileResponse(
            target,
            media_type="application/wasm",
            headers={"Cache-Control": "no-store"},
        )

    if STATIC_DIR.is_dir():
        app.mount("/static", StaticFiles(directory=str(STATIC_DIR)), name="static")

    @app.get("/", response_class=HTMLResponse)
    def serve_index() -> Response:
        idx = WEB_DIR / "index.html"
        if idx.is_file():
            return FileResponse(idx, media_type="text/html", headers={"Cache-Control": "no-store"})
        return HTMLResponse("<h1>Parallel Season FC Host Running</h1><p>API available at /api/health</p>")

    return app


app = create_app()
