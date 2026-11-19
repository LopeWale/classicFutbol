"""Participant fixture kickoff appointments (`propose`, `accept`, `decline`) (Phase 2)."""

from __future__ import annotations

import uuid
from typing import Any

from psfc_host.ledger.store import WorldLedgerStore, utc_now_iso


class AppointmentService:
    """Manages kickoff time negotiation between participants for represented fixtures."""

    def __init__(self, store: WorldLedgerStore) -> None:
        self.store = store

    def propose_appointment(
        self,
        *,
        fixture_id: str,
        proposed_by: str,
        proposed_at_utc: str,
    ) -> dict[str, Any]:
        fx = self.store.get_fixture(fixture_id)
        world_id = fx["world_id"]
        appt_id = f"appt-{uuid.uuid4().hex[:10]}"
        ts = utc_now_iso()
        self.store.conn.execute(
            """
            INSERT INTO appointment (id, fixture_id, world_id, proposed_by, proposed_at_utc, accepted_by, state, created_at)
            VALUES (?, ?, ?, ?, ?, NULL, 'proposed', ?)
            """,
            (appt_id, fixture_id, world_id, proposed_by, proposed_at_utc, ts),
        )
        self.store.append_event(
            world_id=world_id,
            event_type="appointment_proposed",
            aggregate_type="appointment",
            aggregate_id=appt_id,
            payload={
                "appointment_id": appt_id,
                "fixture_id": fixture_id,
                "proposed_by": proposed_by,
                "proposed_at_utc": proposed_at_utc,
            },
            caused_by=proposed_by,
            occurred_at=ts,
        )
        return self.get_appointment(appt_id)

    def respond_appointment(
        self,
        *,
        appointment_id: str,
        responder_id: str,
        accept: bool,
    ) -> dict[str, Any]:
        appt = self.get_appointment(appointment_id)
        if appt["state"] != "proposed":
            raise ValueError(f"Appointment '{appointment_id}' is already '{appt['state']}'")
        new_state = "accepted" if accept else "declined"
        accepted_by = responder_id if accept else None
        ts = utc_now_iso()
        self.store.conn.execute(
            "UPDATE appointment SET state = ?, accepted_by = ? WHERE id = ?",
            (new_state, accepted_by, appointment_id),
        )
        self.store.append_event(
            world_id=appt["world_id"],
            event_type="appointment_confirmed" if accept else "appointment_declined",
            aggregate_type="appointment",
            aggregate_id=appointment_id,
            payload={
                "appointment_id": appointment_id,
                "fixture_id": appt["fixture_id"],
                "responder_id": responder_id,
                "proposed_at_utc": appt["proposed_at_utc"],
                "state": new_state,
            },
            caused_by=responder_id,
            occurred_at=ts,
        )
        return self.get_appointment(appointment_id)

    def get_appointment(self, appointment_id: str) -> dict[str, Any]:
        row = self.store.conn.execute(
            "SELECT * FROM appointment WHERE id = ?",
            (appointment_id,),
        ).fetchone()
        if row is None:
            raise KeyError(f"Appointment '{appointment_id}' not found")
        return dict(row)

    def list_appointments(self, world_id: str) -> list[dict[str, Any]]:
        rows = self.store.conn.execute(
            "SELECT * FROM appointment WHERE world_id = ? ORDER BY created_at DESC",
            (world_id,),
        ).fetchall()
        return [dict(r) for r in rows]
