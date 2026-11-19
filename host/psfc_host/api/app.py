"""FastAPI World API for Parallel Season FC + 5v5 Browser/WASM Client host (Phases 1–6)."""

from __future__ import annotations

import os
import secrets
from pathlib import Path
from typing import Any

from fastapi import FastAPI, Header, HTTPException
from fastapi.responses import FileResponse, HTMLResponse, Response
from fastapi.staticfiles import StaticFiles
from pydantic import BaseModel, Field

from psfc_host.api.auth import DeviceAuthService
from psfc_host.catalogue.clubs import GAMEPLAY_VERSION, ClubCatalogue
from psfc_host.community.appointments import AppointmentService
from psfc_host.community.cartridge import CartridgeSyncService
from psfc_host.competition.engine import CompetitionEngine
from psfc_host.ingest.adapters import ProviderFeedAdapter
from psfc_host.ingest.proposal_gate import ProposalGate
from psfc_host.ledger.projections import project_player_stats, project_standings
from psfc_host.ledger.store import WorldLedgerStore
from psfc_host.match.lobby import OnlineLobbyService
from psfc_host.match.relay import MatchInputRelayRecorder
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
    home_starters: list[str] | None = None
    away_starters: list[str] | None = None
    home_formation: str | None = None
    away_formation: str | None = None


class UpdateLineup5v5Request(BaseModel):
    starter_ids: list[str]
    formation_5v5: str = "1-2-1"
    actor_id: str = "part-alex"


class ExhibitionSnapshotRequest(BaseModel):
    world_id: str | None = None
    home_club_id: str = "man-utd"
    away_club_id: str = "arsenal"
    format_name: str = Field(default="5v5", pattern="^(5v5|11v11)$")
    half_duration_ticks: int = Field(default=3600, ge=60, le=36_000)
    home_controller: dict[str, Any] | None = None
    away_controller: dict[str, Any] | None = None
    home_starters: list[str] | None = None
    away_starters: list[str] | None = None
    home_formation: str | None = None
    away_formation: str | None = None
    seed: int | None = None


class CompleteSessionRequest(BaseModel):
    actor_id: str = "part-alex"
    frames_hex: str | None = None
    frames: list[Any] | None = None
    claimed_final_hash_hex: str | None = None
    claimed_score: list[int] | None = None
    hash_checkpoints: list[dict[str, Any]] = Field(default_factory=list)


class IngestResultRequest(BaseModel):
    fixture_id: str
    home_goals: int = Field(ge=0)
    away_goals: int = Field(ge=0)
    events: list[dict[str, Any]] = Field(default_factory=list)
    provider: str = "real_football_feed_v1"


class ProviderFixtureIngestRequest(BaseModel):
    provider: str = "football_data_v4"
    external_fixture_id: str
    external_home_club_id: str
    external_away_club_id: str
    home_goals: int = Field(ge=0)
    away_goals: int = Field(ge=0)
    events: list[dict[str, Any]] = Field(default_factory=list)
    target_fixture_id: str | None = None


class ResolveIdentityRequest(BaseModel):
    internal_id: str = Field(..., min_length=2)
    actor_id: str = "commissioner"


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


class ResolveAdminFixtureRequest(BaseModel):
    home_goals: int = Field(ge=0)
    away_goals: int = Field(ge=0)
    actor_id: str = "commissioner"
    reason: str = "commissioner walkover / ruling"


class ExpireWindowsRequest(BaseModel):
    as_of_utc: str | None = None
    actor_id: str = "scheduler"


class ProposeAppointmentRequest(BaseModel):
    proposed_by: str = "part-alex"
    proposed_at_utc: str


class RespondAppointmentRequest(BaseModel):
    responder_id: str = "part-sam"
    accept: bool = True


class RegisterDeviceRequest(BaseModel):
    participant_id: str
    label: str = "Custom RP2040 Pad + Cartridge"
    device_id: str | None = None


class RevokeDeviceRequest(BaseModel):
    actor_id: str = "commissioner"


class CartridgeSyncRequest(BaseModel):
    participant_id: str = "part-alex"
    device_id: str = "dev-alex-01"
    camera_preset: str = "broadcast_5v5"
    controller_profile: dict[str, Any] | None = None


class IssueTokenRequest(BaseModel):
    device_id: str
    device_secret: str
    ttl_seconds: int = Field(default=86_400, ge=60, le=604_800)


class LobbyReadyRequest(BaseModel):
    side: str = Field(..., pattern="^(home|away)$")
    actor_id: str = "part-alex"


class LobbyActionRequest(BaseModel):
    actor_id: str = "part-alex"


class LobbyDisconnectRequest(BaseModel):
    disconnected_actor: str = "part-sam"


class LobbyAbandonRequest(BaseModel):
    reason: str = "reconnect_timeout"
    actor_id: str = "lobby_watchdog"


class RelayInputBatchRequest(BaseModel):
    side: str = Field(..., pattern="^(home|away)$")
    first_tick: int = Field(..., ge=1)
    frames: list[dict[str, Any] | str]


class AdvanceCompetitionRequest(BaseModel):
    actor_id: str = "competition_engine"


class GenerateFixtureRequest(BaseModel):
    seed: int | None = None
    actor_id: str = "poisson_generator"


class SeasonRolloverRequest(BaseModel):
    next_season_label: str = "2027/28"
    actor_id: str = "commissioner"


def create_app(
    db_path: str | Path | None = None,
    seed_demo: bool = True,
    enable_demo_reset: bool | None = None,
) -> FastAPI:
    resolved_db = db_path or os.environ.get("PSFC_DB_PATH", "/tmp/psfc_world.db")

    app = FastAPI(
        title="classicFutbol — Parallel Season FC World API",
        version=GAMEPLAY_VERSION,
        description=(
            "Event-sourced Parallel Season host, Proposal Gate, Competition Divergence Engine, "
            "Signed Cartridge Sync, Online Lobby/Relay, and deterministic 5v5/11v11 replay validator."
        ),
    )

    state_holder: dict[str, Any] = {}

    def _is_demo_reset_enabled() -> bool:
        if enable_demo_reset is not None:
            return enable_demo_reset
        if os.environ.get("PSFC_ENV", "development").lower() == "production":
            return False
        if "PSFC_ENABLE_DEMO_RESET" in os.environ:
            return os.environ["PSFC_ENABLE_DEMO_RESET"].lower() in ("1", "true", "yes")
        return str(resolved_db) in (":memory:", "/tmp/psfc_world.db")

    def _init_services() -> None:
        store = WorldLedgerStore(resolved_db)
        catalogue = ClubCatalogue()
        gate = ProposalGate(store, catalogue)
        sessions = MatchSessionService(store, catalogue)
        adapter = ProviderFeedAdapter(store, gate)
        appointments = AppointmentService(store)
        cartridge = CartridgeSyncService(store)
        auth = DeviceAuthService(store)
        lobby = OnlineLobbyService(store)
        relay = MatchInputRelayRecorder(store)
        competition_engine = CompetitionEngine(store, catalogue)
        if seed_demo:
            ensure_demo_world(store, catalogue)
        # Rehydrate persisted squad transfers onto the base catalogue on startup
        existing_world_ids = [w["id"] for w in store.list_worlds()]
        if DEFAULT_WORLD_ID in existing_world_ids:
            catalogue.replay_transfer_events(store.list_events(DEFAULT_WORLD_ID))
        elif len(existing_world_ids) == 1:
            catalogue.replay_transfer_events(store.list_events(existing_world_ids[0]))

        state_holder["store"] = store
        state_holder["catalogue"] = catalogue
        state_holder["gate"] = gate
        state_holder["sessions"] = sessions
        state_holder["adapter"] = adapter
        state_holder["appointments"] = appointments
        state_holder["cartridge"] = cartridge
        state_holder["auth"] = auth
        state_holder["lobby"] = lobby
        state_holder["relay"] = relay
        state_holder["competition_engine"] = competition_engine

    _init_services()

    def _world_catalogue(world_id: str | None) -> ClubCatalogue:
        store: WorldLedgerStore = state_holder["store"]
        base: ClubCatalogue = state_holder["catalogue"]
        if not world_id:
            return base
        return base.for_world(store, world_id)

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

    @app.get("/api/worlds/{world_id}/competitions")
    def get_competitions(world_id: str) -> dict[str, Any]:
        store: WorldLedgerStore = state_holder["store"]
        try:
            store.get_world(world_id)
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        return {"world_id": world_id, "competitions": store.list_competitions(world_id)}

    @app.get("/api/worlds/{world_id}/standings")
    def get_standings(world_id: str) -> dict[str, Any]:
        store: WorldLedgerStore = state_holder["store"]
        try:
            world = store.get_world(world_id)
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        cat = _world_catalogue(world_id)
        events = store.list_events(world_id)
        rep = set(store.represented_clubs(world_id).keys())
        proj = project_standings(events, _clubs_summary(cat), represented_club_ids=rep)
        return {"world": world, **proj}

    @app.get("/api/worlds/{world_id}/players")
    def get_players(world_id: str) -> dict[str, Any]:
        store: WorldLedgerStore = state_holder["store"]
        try:
            store.get_world(world_id)
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        cat = _world_catalogue(world_id)
        events = store.list_events(world_id)
        stats = project_player_stats(events, cat.all_footballers())
        return {"world_id": world_id, "players": stats}

    @app.get("/api/worlds/{world_id}/fixtures")
    def get_fixtures(world_id: str, competition_id: str | None = None) -> dict[str, Any]:
        store: WorldLedgerStore = state_holder["store"]
        try:
            store.get_world(world_id)
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        cat = _world_catalogue(world_id)
        clubs = _clubs_summary(cat)
        fixtures = store.list_fixtures(world_id, competition_id=competition_id)
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
        gate: ProposalGate = state_holder["gate"]
        appointments: AppointmentService = state_holder["appointments"]
        cartridge: CartridgeSyncService = state_holder["cartridge"]
        adapter: ProviderFeedAdapter = state_holder["adapter"]
        try:
            world = store.get_world(world_id)
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc

        cat = _world_catalogue(world_id)
        clubs = _clubs_summary(cat)
        world_lineups = store.list_club_lineups_5v5(world_id)
        rosters = cat.all_rosters(world_lineups)
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
            "competitions": store.list_competitions(world_id),
            "clubs": list(clubs.values()),
            "rosters": rosters,
            "assignments": assignments,
            "standings": standings_proj["standings"],
            "players": player_stats[:16],
            "fixtures": fixtures,
            "appointments": appointments.list_appointments(world_id)[:12],
            "honours": store.list_honours(world_id),
            "devices": cartridge.list_devices(),
            "identity_queue": adapter.list_identity_queue(world_id)[:12],
            "recent_events": list(reversed(events[-25:])),
            "total_events": len(events),
            "proposals": gate.list_proposals(world_id)[:12],
        }

    @app.get("/api/worlds/{world_id}/clubs")
    def list_world_club_rosters(world_id: str) -> dict[str, Any]:
        store: WorldLedgerStore = state_holder["store"]
        try:
            store.get_world(world_id)
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        cat = _world_catalogue(world_id)
        lineups = store.list_club_lineups_5v5(world_id)
        return {"world_id": world_id, "rosters": cat.all_rosters(lineups)}

    @app.get("/api/worlds/{world_id}/clubs/{club_id}/squad")
    def get_club_squad_endpoint(world_id: str, club_id: str) -> dict[str, Any]:
        store: WorldLedgerStore = state_holder["store"]
        try:
            store.get_world(world_id)
            cat = _world_catalogue(world_id)
            saved = store.get_club_lineup_5v5(world_id, club_id)
            roster = cat.get_club_roster(
                club_id,
                selected_starter_ids=saved["starter_ids"] if saved else None,
                formation_5v5=saved["formation_5v5"] if saved else None,
            )
            return {"world_id": world_id, "roster": roster}
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc

    @app.post("/api/worlds/{world_id}/clubs/{club_id}/lineup")
    def save_club_lineup_endpoint(
        world_id: str, club_id: str, req: UpdateLineup5v5Request
    ) -> dict[str, Any]:
        store: WorldLedgerStore = state_holder["store"]
        try:
            store.get_world(world_id)
            cat = _world_catalogue(world_id)
            validated, formation = cat.validate_5v5_selection(
                club_id, req.starter_ids, req.formation_5v5
            )
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        except ValueError as exc:
            raise HTTPException(status_code=422, detail=str(exc)) from exc

        ordered_ids = [m.footballer_id for m in validated]
        saved = store.save_club_lineup_5v5(
            world_id=world_id,
            club_id=club_id,
            starter_ids=ordered_ids,
            formation_5v5=formation,
            actor_id=req.actor_id,
        )
        return {
            "saved_lineup": saved,
            "roster": cat.get_club_roster(
                club_id, selected_starter_ids=ordered_ids, formation_5v5=formation
            ),
        }

    @app.post("/api/exhibition/snapshot")
    def create_exhibition_snapshot(req: ExhibitionSnapshotRequest) -> dict[str, Any]:
        store: WorldLedgerStore = state_holder["store"]
        if req.world_id:
            try:
                store.get_world(req.world_id)
            except KeyError as exc:
                raise HTTPException(status_code=404, detail=str(exc)) from exc
        cat = _world_catalogue(req.world_id)
        if req.home_club_id not in cat.clubs:
            raise HTTPException(status_code=404, detail=f"Unknown club '{req.home_club_id}'")
        if req.away_club_id not in cat.clubs:
            raise HTTPException(status_code=404, detail=f"Unknown club '{req.away_club_id}'")

        saved_home = (
            store.get_club_lineup_5v5(req.world_id, req.home_club_id)
            if req.world_id
            else None
        )
        saved_away = (
            store.get_club_lineup_5v5(req.world_id, req.away_club_id)
            if req.world_id
            else None
        )
        eff_home_starters = (
            req.home_starters
            if req.home_starters is not None
            else (saved_home["starter_ids"] if saved_home else None)
        )
        eff_away_starters = (
            req.away_starters
            if req.away_starters is not None
            else (saved_away["starter_ids"] if saved_away else None)
        )
        eff_home_fmt = (
            req.home_formation
            if req.home_formation is not None
            else (saved_home["formation_5v5"] if saved_home else None)
        )
        eff_away_fmt = (
            req.away_formation
            if req.away_formation is not None
            else (saved_away["formation_5v5"] if saved_away else None)
        )
        chosen_seed = req.seed if req.seed is not None else 20_261_119
        try:
            snapshot = cat.build_fixture_snapshot(
                fixture_id=f"exh-{req.home_club_id}-vs-{req.away_club_id}-{req.format_name}",
                home_club_id=req.home_club_id,
                away_club_id=req.away_club_id,
                seed=chosen_seed,
                format_name=req.format_name,
                home_controller=req.home_controller,
                away_controller=req.away_controller,
                half_duration_ticks=req.half_duration_ticks,
                extra_time_enabled=False,
                home_starters=eff_home_starters,
                away_starters=eff_away_starters,
                home_formation=eff_home_fmt,
                away_formation=eff_away_fmt,
            )
            return {"snapshot": snapshot}
        except ValueError as exc:
            raise HTTPException(status_code=422, detail=str(exc)) from exc

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
                home_starters=req.home_starters,
                away_starters=req.away_starters,
                home_formation=req.home_formation,
                away_formation=req.away_formation,
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
        relay: MatchInputRelayRecorder = state_holder["relay"]
        frames_hex = req.frames_hex
        if frames_hex is None and req.frames is None:
            _, assembled = relay.assemble_session_frames_hex(session_id)
            if assembled:
                frames_hex = assembled
        try:
            return sessions.complete_session(
                session_id=session_id,
                frames_hex=frames_hex,
                frames=req.frames,
                claimed_final_hash_hex=req.claimed_final_hash_hex,
                claimed_score=req.claimed_score,
                hash_checkpoints=req.hash_checkpoints,
                actor_id=req.actor_id,
            )
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        except ReplayValidationError as exc:
            raise HTTPException(status_code=422, detail=str(exc)) from exc
        except ValueError as exc:
            raise HTTPException(status_code=409, detail=str(exc)) from exc

    # --- Phase 2: Ingestion Adapter, Identity Queue, Deadlines, & Appointments ---

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
        except ValueError as exc:
            raise HTTPException(status_code=409, detail=str(exc)) from exc

    @app.post("/api/worlds/{world_id}/ingest/provider-fixture")
    def ingest_provider_fixture(
        world_id: str, req: ProviderFixtureIngestRequest
    ) -> dict[str, Any]:
        adapter: ProviderFeedAdapter = state_holder["adapter"]
        try:
            return adapter.ingest_provider_fixture_payload(
                world_id=world_id,
                provider=req.provider,
                external_fixture_id=req.external_fixture_id,
                external_home_club_id=req.external_home_club_id,
                external_away_club_id=req.external_away_club_id,
                home_goals=req.home_goals,
                away_goals=req.away_goals,
                events=req.events,
                target_fixture_id=req.target_fixture_id,
            )
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        except ValueError as exc:
            raise HTTPException(status_code=409, detail=str(exc)) from exc

    @app.get("/api/worlds/{world_id}/identity-queue")
    def get_identity_queue(world_id: str) -> dict[str, Any]:
        adapter: ProviderFeedAdapter = state_holder["adapter"]
        return {"world_id": world_id, "queue": adapter.list_identity_queue(world_id)}

    @app.post("/api/identity-queue/{queue_id}/resolve")
    def resolve_identity_queue_item(queue_id: str, req: ResolveIdentityRequest) -> dict[str, Any]:
        adapter: ProviderFeedAdapter = state_holder["adapter"]
        try:
            resolved = adapter.resolve_identity_mapping(
                queue_id=queue_id,
                internal_id=req.internal_id,
                actor_id=req.actor_id,
            )
            return {"resolved": resolved}
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

    @app.post("/api/worlds/{world_id}/expire-windows")
    def expire_playing_windows(world_id: str, req: ExpireWindowsRequest) -> dict[str, Any]:
        store: WorldLedgerStore = state_holder["store"]
        transitioned = store.expire_overdue_windows(
            world_id=world_id,
            as_of_utc=req.as_of_utc,
            actor_id=req.actor_id,
        )
        return {"transitioned": transitioned, "count": len(transitioned)}

    @app.post("/api/admin/fixtures/{fixture_id}/resolve")
    def admin_resolve_fixture(fixture_id: str, req: ResolveAdminFixtureRequest) -> dict[str, Any]:
        store: WorldLedgerStore = state_holder["store"]
        try:
            res = store.resolve_awaiting_admin_fixture(
                fixture_id=fixture_id,
                home_goals=req.home_goals,
                away_goals=req.away_goals,
                actor_id=req.actor_id,
                reason=req.reason,
            )
            return {"accepted_result": res}
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc

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

    @app.get("/api/worlds/{world_id}/appointments")
    def get_appointments(world_id: str) -> dict[str, Any]:
        appointments: AppointmentService = state_holder["appointments"]
        return {"world_id": world_id, "appointments": appointments.list_appointments(world_id)}

    @app.post("/api/fixtures/{fixture_id}/appointments")
    def propose_fixture_appointment(
        fixture_id: str, req: ProposeAppointmentRequest
    ) -> dict[str, Any]:
        appointments: AppointmentService = state_holder["appointments"]
        try:
            appt = appointments.propose_appointment(
                fixture_id=fixture_id,
                proposed_by=req.proposed_by,
                proposed_at_utc=req.proposed_at_utc,
            )
            return {"appointment": appt}
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc

    @app.post("/api/appointments/{appointment_id}/respond")
    def respond_fixture_appointment(
        appointment_id: str, req: RespondAppointmentRequest
    ) -> dict[str, Any]:
        appointments: AppointmentService = state_holder["appointments"]
        try:
            appt = appointments.respond_appointment(
                appointment_id=appointment_id,
                responder_id=req.responder_id,
                accept=req.accept,
            )
            return {"appointment": appt}
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        except ValueError as exc:
            raise HTTPException(status_code=409, detail=str(exc)) from exc

    # --- Phase 3: Cartridge Devices & Signed Sync ---

    @app.get("/api/worlds/{world_id}/devices")
    def get_devices(world_id: str, participant_id: str | None = None) -> dict[str, Any]:
        cartridge: CartridgeSyncService = state_holder["cartridge"]
        return {"world_id": world_id, "devices": cartridge.list_devices(participant_id)}

    @app.post("/api/worlds/{world_id}/devices")
    def register_device_endpoint(world_id: str, req: RegisterDeviceRequest) -> dict[str, Any]:
        cartridge: CartridgeSyncService = state_holder["cartridge"]
        return cartridge.register_device(
            participant_id=req.participant_id,
            world_id=world_id,
            label=req.label,
            device_id=req.device_id,
        )

    @app.post("/api/worlds/{world_id}/devices/{device_id}/revoke")
    def revoke_device_endpoint(
        world_id: str, device_id: str, req: RevokeDeviceRequest
    ) -> dict[str, Any]:
        cartridge: CartridgeSyncService = state_holder["cartridge"]
        try:
            return cartridge.revoke_device(
                device_id=device_id,
                world_id=world_id,
                actor_id=req.actor_id,
            )
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc

    @app.post("/api/worlds/{world_id}/cartridge/sync")
    def sync_cartridge_endpoint(world_id: str, req: CartridgeSyncRequest) -> dict[str, Any]:
        cartridge: CartridgeSyncService = state_holder["cartridge"]
        cat: ClubCatalogue = state_holder["catalogue"]
        try:
            envelope = cartridge.issue_signed_cartridge(
                world_id=world_id,
                participant_id=req.participant_id,
                device_id=req.device_id,
                clubs_summary=_clubs_summary(cat),
                controller_profile=req.controller_profile,
                camera_preset=req.camera_preset,
            )
            return {"envelope": envelope}
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        except PermissionError as exc:
            raise HTTPException(status_code=403, detail=str(exc)) from exc

    @app.post("/api/cartridge/verify")
    def verify_cartridge_endpoint(body: dict[str, Any]) -> dict[str, Any]:
        cartridge: CartridgeSyncService = state_holder["cartridge"]
        envelope = body.get("envelope") if isinstance(body.get("envelope"), dict) else body
        return cartridge.verify_cartridge_envelope(envelope)

    # --- Phase 4: Device JWT Auth, Online Lobby, & Relay Input Batches ---

    @app.post("/api/auth/token")
    def issue_device_jwt(req: IssueTokenRequest) -> dict[str, Any]:
        auth: DeviceAuthService = state_holder["auth"]
        try:
            return auth.issue_token(
                device_id=req.device_id,
                device_secret=req.device_secret,
                ttl_seconds=req.ttl_seconds,
            )
        except PermissionError as exc:
            raise HTTPException(status_code=401, detail=str(exc)) from exc

    @app.get("/api/auth/verify")
    def verify_device_jwt(
        token: str | None = None,
        authorization: str | None = Header(default=None),
    ) -> dict[str, Any]:
        auth: DeviceAuthService = state_holder["auth"]
        raw = token
        if not raw and authorization and authorization.lower().startswith("bearer "):
            raw = authorization[7:].strip()
        if not raw:
            raise HTTPException(status_code=401, detail="Missing token")
        try:
            claims = auth.verify_token(raw)
            return {"valid": True, "claims": claims}
        except PermissionError as exc:
            raise HTTPException(status_code=401, detail=str(exc)) from exc

    @app.post("/api/sessions/{session_id}/ready")
    def session_mark_ready(session_id: str, req: LobbyReadyRequest) -> dict[str, Any]:
        lobby: OnlineLobbyService = state_holder["lobby"]
        try:
            return {"session": lobby.mark_ready(session_id=session_id, side=req.side, actor_id=req.actor_id)}
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        except ValueError as exc:
            raise HTTPException(status_code=400, detail=str(exc)) from exc

    @app.post("/api/sessions/{session_id}/pause")
    def session_pause(session_id: str, req: LobbyActionRequest) -> dict[str, Any]:
        lobby: OnlineLobbyService = state_holder["lobby"]
        try:
            return {"session": lobby.request_pause(session_id=session_id, actor_id=req.actor_id)}
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        except ValueError as exc:
            raise HTTPException(status_code=409, detail=str(exc)) from exc

    @app.post("/api/sessions/{session_id}/resume")
    def session_resume(session_id: str, req: LobbyActionRequest) -> dict[str, Any]:
        lobby: OnlineLobbyService = state_holder["lobby"]
        try:
            return {"session": lobby.resume_session(session_id=session_id, actor_id=req.actor_id)}
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        except ValueError as exc:
            raise HTTPException(status_code=409, detail=str(exc)) from exc

    @app.post("/api/sessions/{session_id}/disconnect")
    def session_disconnect(session_id: str, req: LobbyDisconnectRequest) -> dict[str, Any]:
        lobby: OnlineLobbyService = state_holder["lobby"]
        try:
            return {
                "session": lobby.report_disconnect(
                    session_id=session_id, disconnected_actor=req.disconnected_actor
                )
            }
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc

    @app.post("/api/sessions/{session_id}/abandon")
    def session_abandon(session_id: str, req: LobbyAbandonRequest) -> dict[str, Any]:
        lobby: OnlineLobbyService = state_holder["lobby"]
        try:
            return {
                "session": lobby.abandon_session(
                    session_id=session_id, reason=req.reason, actor_id=req.actor_id
                )
            }
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc

    @app.post("/api/sessions/{session_id}/relay-inputs")
    def session_relay_inputs(session_id: str, req: RelayInputBatchRequest) -> dict[str, Any]:
        relay: MatchInputRelayRecorder = state_holder["relay"]
        try:
            written = relay.record_input_batch(
                session_id=session_id,
                side=req.side,
                first_tick=req.first_tick,
                frames=req.frames,
            )
            return {"session_id": session_id, "written": written}
        except ValueError as exc:
            raise HTTPException(status_code=400, detail=str(exc)) from exc

    # --- Phase 5: Competition Divergence, Poisson Generator, & Season Rollover ---

    @app.post("/api/worlds/{world_id}/competitions/{competition_id}/advance")
    def advance_competition_endpoint(
        world_id: str, competition_id: str, req: AdvanceCompetitionRequest
    ) -> dict[str, Any]:
        comp_engine: CompetitionEngine = state_holder["competition_engine"]
        try:
            return comp_engine.advance_knockout_competition(
                world_id=world_id,
                competition_id=competition_id,
                actor_id=req.actor_id,
            )
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        except ValueError as exc:
            raise HTTPException(status_code=409, detail=str(exc)) from exc

    @app.post("/api/fixtures/{fixture_id}/generate")
    def generate_fixture_endpoint(fixture_id: str, req: GenerateFixtureRequest) -> dict[str, Any]:
        comp_engine: CompetitionEngine = state_holder["competition_engine"]
        try:
            return comp_engine.generate_poisson_fixture_result(
                fixture_id=fixture_id,
                seed=req.seed,
                actor_id=req.actor_id,
            )
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        except ValueError as exc:
            raise HTTPException(status_code=409, detail=str(exc)) from exc

    @app.post("/api/worlds/{world_id}/rollover")
    def rollover_season_endpoint(world_id: str, req: SeasonRolloverRequest) -> dict[str, Any]:
        comp_engine: CompetitionEngine = state_holder["competition_engine"]
        try:
            return comp_engine.complete_season_and_rollover(
                world_id=world_id,
                next_season_label=req.next_season_label,
                actor_id=req.actor_id,
            )
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc

    @app.post("/api/worlds/{world_id}/reset-demo")
    def reset_demo_world(
        world_id: str,
        x_admin_token: str | None = Header(default=None),
        authorization: str | None = Header(default=None),
    ) -> dict[str, Any]:
        if not _is_demo_reset_enabled():
            raise HTTPException(
                status_code=403,
                detail="Demo reset endpoint is disabled outside development",
            )
        admin_token = os.environ.get("PSFC_ADMIN_TOKEN")
        if admin_token:
            bearer = (
                authorization[7:].strip()
                if authorization and authorization.lower().startswith("bearer ")
                else None
            )
            supplied = x_admin_token or bearer or ""
            if not supplied or not secrets.compare_digest(supplied, admin_token):
                raise HTTPException(status_code=403, detail="Invalid or missing admin token")

        store: WorldLedgerStore = state_holder["store"]
        try:
            store.get_world(world_id)
        except KeyError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        if world_id != DEFAULT_WORLD_ID:
            raise HTTPException(
                status_code=403,
                detail=f"Only the demo world '{DEFAULT_WORLD_ID}' may be reset",
            )

        store.reset_world_for_demo(world_id)
        fresh_catalogue = ClubCatalogue()
        ensure_demo_world(store, fresh_catalogue)
        state_holder["catalogue"] = fresh_catalogue
        state_holder["gate"] = ProposalGate(store, fresh_catalogue)
        state_holder["sessions"] = MatchSessionService(store, fresh_catalogue)
        state_holder["adapter"] = ProviderFeedAdapter(store, state_holder["gate"])
        state_holder["competition_engine"] = CompetitionEngine(store, fresh_catalogue)
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
