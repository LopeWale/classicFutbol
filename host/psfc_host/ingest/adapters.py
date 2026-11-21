"""External football-data provider adapter and identity-resolution queue (Phase 2)."""

from __future__ import annotations

import json
import uuid
from typing import Any, Protocol

from psfc_host.ingest.proposal_gate import ProposalGate
from psfc_host.ledger.store import WorldLedgerStore, utc_now_iso


class FootballDataSource(Protocol):
    """Swappable interface for external real-world football data providers."""

    provider_name: str

    def fetch_fixture_results(self) -> list[dict[str, Any]]: ...


DEFAULT_EXTERNAL_CLUB_MAPPINGS: list[tuple[str, str, str]] = [
    ("football_data_v4", "club", "ext-club-57"),    # arsenal
    ("football_data_v4", "club", "ext-club-397"),   # brighton
    ("football_data_v4", "club", "ext-club-65"),    # man-city
    ("football_data_v4", "club", "ext-club-1044"),  # bournemouth
    ("football_data_v4", "club", "ext-club-64"),    # liverpool
    ("football_data_v4", "club", "ext-club-341"),   # leeds
    ("football_data_v4", "club", "ext-club-62"),    # everton
    ("football_data_v4", "club", "ext-club-61"),    # chelsea
    ("football_data_v4", "club", "ext-club-73"),    # tottenham
    ("football_data_v4", "club", "ext-club-402"),   # brentford
    ("football_data_v4", "club", "ext-club-58"),    # aston-villa
    ("football_data_v4", "club", "ext-club-67"),    # newcastle
    ("football_data_v4", "club", "ext-club-1076"),  # coventry
    ("football_data_v4", "club", "ext-club-322"),   # hull
    ("football_data_v4", "club", "ext-club-63"),    # fulham
    ("football_data_v4", "club", "ext-club-349"),   # ipswich
    ("football_data_v4", "club", "ext-club-351"),   # nottm-forest
    ("football_data_v4", "club", "ext-club-71"),    # sunderland
    ("football_data_v4", "club", "ext-club-66"),    # man-utd
    ("football_data_v4", "club", "ext-club-354"),   # crystal-palace
]

_DEFAULT_INTERNAL_MAP = {
    "ext-club-57": "arsenal",
    "ext-club-397": "brighton",
    "ext-club-65": "man-city",
    "ext-club-1044": "bournemouth",
    "ext-club-64": "liverpool",
    "ext-club-341": "leeds",
    "ext-club-62": "everton",
    "ext-club-61": "chelsea",
    "ext-club-73": "tottenham",
    "ext-club-402": "brentford",
    "ext-club-58": "aston-villa",
    "ext-club-67": "newcastle",
    "ext-club-1076": "coventry",
    "ext-club-322": "hull",
    "ext-club-63": "fulham",
    "ext-club-349": "ipswich",
    "ext-club-351": "nottm-forest",
    "ext-club-71": "sunderland",
    "ext-club-66": "man-utd",
    "ext-club-354": "crystal-palace",
}


class ProviderFeedAdapter:
    """Normalizes external provider payloads via `external_identity` and parks unknown IDs."""

    def __init__(self, store: WorldLedgerStore, gate: ProposalGate) -> None:
        self.store = store
        self.gate = gate
        self.ensure_default_mappings()

    def ensure_default_mappings(self) -> None:
        ts = utc_now_iso()
        for provider, etype, ext_id in DEFAULT_EXTERNAL_CLUB_MAPPINGS:
            self.store.conn.execute(
                """
                INSERT OR IGNORE INTO external_identity (provider, entity_type, external_id, internal_id, resolved_at)
                VALUES (?, ?, ?, ?, ?)
                """,
                (provider, etype, ext_id, _DEFAULT_INTERNAL_MAP[ext_id], ts),
            )
        self.store.conn.commit()

    def lookup_internal_id(self, provider: str, entity_type: str, external_id: str) -> str | None:
        row = self.store.conn.execute(
            """
            SELECT internal_id FROM external_identity
            WHERE provider = ? AND entity_type = ? AND external_id = ?
            """,
            (provider, entity_type, external_id),
        ).fetchone()
        return str(row["internal_id"]) if row is not None else None

    def enqueue_unknown_identity(
        self,
        *,
        world_id: str,
        provider: str,
        entity_type: str,
        external_id: str,
        display_hint: str,
        raw_context: dict[str, Any],
    ) -> dict[str, Any]:
        existing = self.store.conn.execute(
            """
            SELECT * FROM identity_resolution_queue
            WHERE world_id = ? AND provider = ? AND entity_type = ? AND external_id = ? AND status = 'pending'
            """,
            (world_id, provider, entity_type, external_id),
        ).fetchone()
        if existing is not None:
            return dict(existing)

        qid = f"idq-{uuid.uuid4().hex[:10]}"
        ts = utc_now_iso()
        self.store.conn.execute(
            """
            INSERT INTO identity_resolution_queue (
                id, world_id, provider, entity_type, external_id, display_hint,
                raw_context_json, status, resolved_internal_id, created_at, resolved_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, 'pending', NULL, ?, NULL)
            """,
            (
                qid,
                world_id,
                provider,
                entity_type,
                external_id,
                display_hint,
                json.dumps(raw_context, separators=(",", ":"), sort_keys=True),
                ts,
            ),
        )
        self.store.append_event(
            world_id=world_id,
            event_type="identity_resolution_required",
            aggregate_type="identity_queue",
            aggregate_id=qid,
            payload={
                "queue_id": qid,
                "provider": provider,
                "entity_type": entity_type,
                "external_id": external_id,
                "display_hint": display_hint,
            },
            caused_by=f"ingest:{provider}",
            occurred_at=ts,
        )
        row = self.store.conn.execute(
            "SELECT * FROM identity_resolution_queue WHERE id = ?",
            (qid,),
        ).fetchone()
        return dict(row) if row is not None else {}

    def resolve_identity_mapping(
        self,
        *,
        queue_id: str,
        internal_id: str,
        actor_id: str = "commissioner",
    ) -> dict[str, Any]:
        row = self.store.conn.execute(
            "SELECT * FROM identity_resolution_queue WHERE id = ?",
            (queue_id,),
        ).fetchone()
        if row is None:
            raise KeyError(f"Identity queue item '{queue_id}' not found")
        ts = utc_now_iso()
        self.store.conn.execute(
            """
            INSERT OR REPLACE INTO external_identity (provider, entity_type, external_id, internal_id, resolved_at)
            VALUES (?, ?, ?, ?, ?)
            """,
            (row["provider"], row["entity_type"], row["external_id"], internal_id, ts),
        )
        self.store.conn.execute(
            """
            UPDATE identity_resolution_queue
            SET status = 'resolved', resolved_internal_id = ?, resolved_at = ?
            WHERE id = ?
            """,
            (internal_id, ts, queue_id),
        )
        self.store.append_event(
            world_id=row["world_id"],
            event_type="identity_mapping_resolved",
            aggregate_type="identity_queue",
            aggregate_id=queue_id,
            payload={
                "queue_id": queue_id,
                "provider": row["provider"],
                "entity_type": row["entity_type"],
                "external_id": row["external_id"],
                "internal_id": internal_id,
            },
            caused_by=actor_id,
            occurred_at=ts,
        )
        updated = self.store.conn.execute(
            "SELECT * FROM identity_resolution_queue WHERE id = ?",
            (queue_id,),
        ).fetchone()
        return dict(updated) if updated is not None else {}

    def list_identity_queue(self, world_id: str) -> list[dict[str, Any]]:
        rows = self.store.conn.execute(
            "SELECT * FROM identity_resolution_queue WHERE world_id = ? ORDER BY created_at DESC",
            (world_id,),
        ).fetchall()
        out = []
        for r in rows:
            d = dict(r)
            d["raw_context"] = json.loads(d.pop("raw_context_json"))
            out.append(d)
        return out

    def ingest_provider_fixture_payload(
        self,
        *,
        world_id: str,
        provider: str,
        external_fixture_id: str,
        external_home_club_id: str,
        external_away_club_id: str,
        home_goals: int,
        away_goals: int,
        events: list[dict[str, Any]] | None = None,
        target_fixture_id: str | None = None,
    ) -> dict[str, Any]:
        """Normalize an external provider match payload, queuing unknown clubs or forwarding to ProposalGate."""
        raw = {
            "external_fixture_id": external_fixture_id,
            "external_home_club_id": external_home_club_id,
            "external_away_club_id": external_away_club_id,
            "home_goals": home_goals,
            "away_goals": away_goals,
        }
        home_cid = self.lookup_internal_id(provider, "club", external_home_club_id)
        if home_cid is None:
            q = self.enqueue_unknown_identity(
                world_id=world_id,
                provider=provider,
                entity_type="club",
                external_id=external_home_club_id,
                display_hint=f"Unmapped home club ({external_home_club_id})",
                raw_context=raw,
            )
            return {"status": "queued_for_identity_resolution", "queue_item": q}

        away_cid = self.lookup_internal_id(provider, "club", external_away_club_id)
        if away_cid is None:
            q = self.enqueue_unknown_identity(
                world_id=world_id,
                provider=provider,
                entity_type="club",
                external_id=external_away_club_id,
                display_hint=f"Unmapped away club ({external_away_club_id})",
                raw_context=raw,
            )
            return {"status": "queued_for_identity_resolution", "queue_item": q}

        fx_id = target_fixture_id or self.lookup_internal_id(provider, "fixture", external_fixture_id)
        if fx_id is None:
            cand = self.store.conn.execute(
                """
                SELECT id FROM fixture
                WHERE world_id = ? AND home_club_id = ? AND away_club_id = ?
                ORDER BY round ASC LIMIT 1
                """,
                (world_id, home_cid, away_cid),
            ).fetchone()
            if cand is None:
                q = self.enqueue_unknown_identity(
                    world_id=world_id,
                    provider=provider,
                    entity_type="fixture",
                    external_id=external_fixture_id,
                    display_hint=f"{home_cid} vs {away_cid} ({external_fixture_id})",
                    raw_context=raw,
                )
                return {"status": "queued_for_identity_resolution", "queue_item": q}
            fx_id = str(cand["id"])

        prop = self.gate.submit_fixture_result_import(
            world_id=world_id,
            fixture_id=fx_id,
            home_goals=home_goals,
            away_goals=away_goals,
            events=events or [],
            provider=provider,
        )
        return {"status": prop["decision"], "proposal": prop}
