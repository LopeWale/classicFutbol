"""Proposal Gate enforcing takeover & parallel-world precedence over external football imports."""

from __future__ import annotations

import json
import uuid
from typing import Any

from psfc_host.catalogue.clubs import ClubCatalogue
from psfc_host.ledger.store import WorldLedgerStore, utc_now_iso


class ProposalGate:
    """Quarantines external football-data payloads as proposals before touching the ledger."""

    def __init__(self, store: WorldLedgerStore, catalogue: ClubCatalogue) -> None:
        self.store = store
        self.catalogue = catalogue

    def _record_raw_payload(self, provider: str, endpoint: str, raw: dict[str, Any]) -> str:
        payload_id = f"pay-{uuid.uuid4().hex[:12]}"
        self.store.conn.execute(
            """
            INSERT INTO import_payload (id, provider, endpoint, retrieved_at, raw_json)
            VALUES (?, ?, ?, ?, ?)
            """,
            (
                payload_id,
                provider,
                endpoint,
                utc_now_iso(),
                json.dumps(raw, separators=(",", ":"), sort_keys=True),
            ),
        )
        return payload_id

    def _record_proposal(
        self,
        *,
        payload_id: str,
        world_id: str,
        target_type: str,
        target_id: str,
        proposed: dict[str, Any],
        decision: str,
        reason: str,
    ) -> dict[str, Any]:
        prop_id = f"prop-{uuid.uuid4().hex[:12]}"
        ts = utc_now_iso()
        self.store.conn.execute(
            """
            INSERT INTO import_proposal (
                id, payload_id, world_id, target_type, target_id,
                proposed_json, decision, reason, decided_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
            (
                prop_id,
                payload_id,
                world_id,
                target_type,
                target_id,
                json.dumps(proposed, separators=(",", ":"), sort_keys=True),
                decision,
                reason,
                ts,
            ),
        )
        return {
            "proposal_id": prop_id,
            "payload_id": payload_id,
            "world_id": world_id,
            "target_type": target_type,
            "target_id": target_id,
            "decision": decision,
            "reason": reason,
            "decided_at": ts,
        }

    def submit_fixture_result_import(
        self,
        *,
        world_id: str,
        fixture_id: str,
        home_goals: int,
        away_goals: int,
        events: list[dict[str, Any]] | None = None,
        provider: str = "real_football_feed_v1",
    ) -> dict[str, Any]:
        """Apply or park an external real-world match result according to Parallel World Precedence."""
        self.store.get_world(world_id)
        fx = self.store.get_fixture(fixture_id)
        if fx["world_id"] != world_id:
            raise ValueError(
                f"Fixture '{fixture_id}' belongs to world '{fx['world_id']}', not '{world_id}'"
            )

        raw = {
            "fixture_id": fixture_id,
            "home_goals": home_goals,
            "away_goals": away_goals,
            "events": events or [],
        }
        payload_id = self._record_raw_payload(provider, f"/fixtures/{fixture_id}/result", raw)

        # IDEMPOTENT EVENT ENRICHMENT FOR SCORE-ONLY IMPORTED FIXTURES:
        # If an unrepresented `imported` fixture was previously accepted without player events
        # (e.g. provider delayed goal/save feed or identity resolution completed later), allow
        # attaching the missing events once as long as the accepted source is `'imported'` and
        # the score matches, while continuing to reject imports for played fixtures.
        if fx["state"] == "accepted" and fx["result_source"] == "imported" and events:
            existing_acc = self.store.get_accepted_result(fixture_id)
            has_recorded_events = (
                self.store.conn.execute(
                    """
                    SELECT 1 FROM world_event
                    WHERE world_id = ? AND aggregate_id = ? AND type = 'imported_match_events_recorded'
                    LIMIT 1
                    """,
                    (world_id, fixture_id),
                ).fetchone()
                is not None
            )
            if (
                existing_acc is not None
                and existing_acc.get("source") == "imported"
                and int(existing_acc["home_goals"]) == home_goals
                and int(existing_acc["away_goals"]) == away_goals
                and not has_recorded_events
            ):
                ts = utc_now_iso()
                prop = self._record_proposal(
                    payload_id=payload_id,
                    world_id=world_id,
                    target_type="fixture",
                    target_id=fixture_id,
                    proposed=raw,
                    decision="applied",
                    reason="imported_events_enriched",
                )
                self.store.append_event(
                    world_id=world_id,
                    event_type="imported_match_events_recorded",
                    aggregate_type="fixture",
                    aggregate_id=fixture_id,
                    payload={
                        "fixture_id": fixture_id,
                        "source": "imported",
                        "enriched": True,
                        "events": events,
                    },
                    caused_by=f"ingest:{provider}",
                    occurred_at=ts,
                    commit=True,
                )
                return prop

        # WORLD PRECEDENCE RULE:
        # 1. Any fixture already in_session, played, accepted, or awaiting_admin is immutable to imports.
        # 2. Any fixture advance-assigned to played_vs_friend / played_vs_cpu / generated / admin
        #    cannot be silently replaced by an external feed!
        if (
            fx["state"] in ("in_session", "played", "accepted", "awaiting_admin")
            or fx["result_source"] != "imported"
        ):
            prop = self._record_proposal(
                payload_id=payload_id,
                world_id=world_id,
                target_type="fixture",
                target_id=fixture_id,
                proposed=raw,
                decision="parked",
                reason="world_precedence",
            )
            self.store.append_event(
                world_id=world_id,
                event_type="import_parked",
                aggregate_type="fixture",
                aggregate_id=fixture_id,
                payload={
                    "proposal_id": prop["proposal_id"],
                    "fixture_id": fixture_id,
                    "fixture_state": fx["state"],
                    "fixture_result_source": fx["result_source"],
                    "reason": "world_precedence",
                    "parked_score": [home_goals, away_goals],
                },
                caused_by=f"ingest:{provider}",
            )
            return prop

        # Eligible imported fixture (pre-takeover baseline or unrepresented match)
        ts = utc_now_iso()
        prop = self._record_proposal(
            payload_id=payload_id,
            world_id=world_id,
            target_type="fixture",
            target_id=fixture_id,
            proposed=raw,
            decision="applied",
            reason="imported_source_eligible",
        )
        res_ev = self.store.append_event(
            world_id=world_id,
            event_type="imported_result_accepted",
            aggregate_type="fixture",
            aggregate_id=fixture_id,
            payload={
                "fixture_id": fixture_id,
                "competition_id": fx.get("competition_id"),
                "stage": fx.get("stage", "league"),
                "home_club_id": fx["home_club_id"],
                "away_club_id": fx["away_club_id"],
                "home_goals": home_goals,
                "away_goals": away_goals,
                "source": "imported",
                "provider": provider,
            },
            caused_by=f"ingest:{provider}",
            occurred_at=ts,
            commit=False,
        )
        if events:
            self.store.append_event(
                world_id=world_id,
                event_type="imported_match_events_recorded",
                aggregate_type="fixture",
                aggregate_id=fixture_id,
                payload={
                    "fixture_id": fixture_id,
                    "source": "imported",
                    "events": events,
                },
                caused_by=f"ingest:{provider}",
                occurred_at=ts,
                commit=False,
            )

        self.store.conn.execute(
            """
            INSERT INTO accepted_result (
                fixture_id, world_id, session_id, source,
                home_goals, away_goals, final_hash_hex, accepted_at, event_seq
            )
            VALUES (?, ?, NULL, 'imported', ?, ?, NULL, ?, ?)
            """,
            (fixture_id, world_id, home_goals, away_goals, ts, res_ev["seq"]),
        )
        self.store.conn.execute(
            "UPDATE fixture SET state = 'accepted' WHERE id = ?",
            (fixture_id,),
        )
        self.store.commit()
        return prop

    def submit_transfer_import(
        self,
        *,
        world_id: str,
        footballer_id: str,
        from_club_id: str,
        to_club_id: str,
        new_shirt: int = 99,
        provider: str = "real_football_feed_v1",
    ) -> dict[str, Any]:
        """Apply a squad transfer interval to future snapshots without mutating frozen snapshots."""
        world = self.store.get_world(world_id)
        ts = utc_now_iso()
        raw = {
            "footballer_id": footballer_id,
            "from_club_id": from_club_id,
            "to_club_id": to_club_id,
            "new_shirt": new_shirt,
        }
        if world.get("status") == "completed":
            payload_id = self._record_raw_payload(provider, "/transfers", raw)
            return self._record_proposal(
                payload_id=payload_id,
                world_id=world_id,
                target_type="footballer",
                target_id=footballer_id,
                proposed=raw,
                decision="parked",
                reason="world_completed",
            )
        world_cat = self.catalogue.for_world(self.store, world_id)
        joined = world_cat.record_transfer(
            footballer_id=footballer_id,
            from_club_id=from_club_id,
            to_club_id=to_club_id,
            occurred_at=ts,
            new_shirt=new_shirt,
        )
        # Keep in-memory default catalogue synced as well when applicable
        if any(
            m.footballer_id == footballer_id and m.effective_to is None
            for m in self.catalogue.clubs[from_club_id].squad
        ):
            self.catalogue.record_transfer(
                footballer_id=footballer_id,
                from_club_id=from_club_id,
                to_club_id=to_club_id,
                occurred_at=ts,
                new_shirt=new_shirt,
            )
        payload_id = self._record_raw_payload(provider, "/transfers", raw)
        prop = self._record_proposal(
            payload_id=payload_id,
            world_id=world_id,
            target_type="footballer",
            target_id=footballer_id,
            proposed=raw,
            decision="applied",
            reason="future_squad_interval_updated",
        )
        self.store.append_event(
            world_id=world_id,
            event_type="imported_squad_membership_changed",
            aggregate_type="footballer",
            aggregate_id=footballer_id,
            payload={
                "footballer_id": footballer_id,
                "name": joined.name,
                "from_club_id": from_club_id,
                "to_club_id": to_club_id,
                "new_shirt": new_shirt,
                "effective_from": ts,
            },
            caused_by=f"ingest:{provider}",
            occurred_at=ts,
        )
        return prop

    def list_proposals(self, world_id: str) -> list[dict[str, Any]]:
        rows = self.store.conn.execute(
            "SELECT * FROM import_proposal WHERE world_id = ? ORDER BY decided_at DESC",
            (world_id,),
        ).fetchall()
        out = []
        for r in rows:
            d = dict(r)
            d["proposed"] = json.loads(d.pop("proposed_json"))
            out.append(d)
        return out
