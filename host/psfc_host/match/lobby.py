"""Online 2-player lobby, ready-check, pause budget, and disconnect/abandonment rules (Phase 4)."""

from __future__ import annotations

from datetime import datetime, timedelta, timezone
from typing import Any

from psfc_host.ledger.store import WorldLedgerStore, utc_now_iso


RECONNECT_GRACE_SECONDS = 90


class OnlineLobbyService:
    """Coordinates ready checks, pause budgets, and disconnect abandonment for online sessions."""

    def __init__(self, store: WorldLedgerStore) -> None:
        self.store = store

    def get_session(self, session_id: str) -> dict[str, Any]:
        row = self.store.conn.execute(
            "SELECT * FROM match_session WHERE id = ?",
            (session_id,),
        ).fetchone()
        if row is None:
            raise KeyError(f"Session '{session_id}' not found")
        return dict(row)

    def mark_ready(self, *, session_id: str, side: str, actor_id: str) -> dict[str, Any]:
        sess = self.get_session(session_id)
        if side not in ("home", "away"):
            raise ValueError("side must be 'home' or 'away'")
        col = "home_ready" if side == "home" else "away_ready"
        self.store.conn.execute(
            f"UPDATE match_session SET {col} = 1 WHERE id = ?",
            (session_id,),
        )
        updated = self.get_session(session_id)
        if int(updated["home_ready"]) == 1 and int(updated["away_ready"]) == 1:
            if updated["state"] in ("created", "ready_check", "paused"):
                self.store.conn.execute(
                    "UPDATE match_session SET state = 'running', reconnect_deadline_utc = NULL WHERE id = ?",
                    (session_id,),
                )
        else:
            if updated["state"] in ("created", "running"):
                self.store.conn.execute(
                    "UPDATE match_session SET state = 'ready_check' WHERE id = ?",
                    (session_id,),
                )
        self.store.append_event(
            world_id=sess["world_id"],
            event_type="lobby_ready_updated",
            aggregate_type="match_session",
            aggregate_id=session_id,
            payload={"session_id": session_id, "side": side, "ready": True},
            caused_by=actor_id,
        )
        return self.get_session(session_id)

    def request_pause(self, *, session_id: str, actor_id: str) -> dict[str, Any]:
        sess = self.get_session(session_id)
        if sess["state"] != "running":
            raise ValueError(f"Cannot pause session in state '{sess['state']}'")
        rem = int(sess["pause_budget_remaining"])
        if rem <= 0:
            raise ValueError("Pause budget exhausted for this session")
        self.store.conn.execute(
            "UPDATE match_session SET state = 'paused', pause_budget_remaining = ? WHERE id = ?",
            (rem - 1, session_id),
        )
        self.store.append_event(
            world_id=sess["world_id"],
            event_type="session_paused",
            aggregate_type="match_session",
            aggregate_id=session_id,
            payload={"session_id": session_id, "pause_budget_remaining": rem - 1},
            caused_by=actor_id,
        )
        return self.get_session(session_id)

    def resume_session(self, *, session_id: str, actor_id: str) -> dict[str, Any]:
        sess = self.get_session(session_id)
        if sess["state"] != "paused":
            raise ValueError(f"Session '{session_id}' is not paused")
        self.store.conn.execute(
            "UPDATE match_session SET state = 'running', reconnect_deadline_utc = NULL WHERE id = ?",
            (session_id,),
        )
        self.store.append_event(
            world_id=sess["world_id"],
            event_type="session_resumed",
            aggregate_type="match_session",
            aggregate_id=session_id,
            payload={"session_id": session_id},
            caused_by=actor_id,
        )
        return self.get_session(session_id)

    def report_disconnect(self, *, session_id: str, disconnected_actor: str) -> dict[str, Any]:
        sess = self.get_session(session_id)
        deadline = (
            datetime.now(timezone.utc) + timedelta(seconds=RECONNECT_GRACE_SECONDS)
        ).strftime("%Y-%m-%dT%H:%M:%SZ")
        self.store.conn.execute(
            "UPDATE match_session SET state = 'paused', reconnect_deadline_utc = ? WHERE id = ?",
            (deadline, session_id),
        )
        self.store.append_event(
            world_id=sess["world_id"],
            event_type="peer_disconnected",
            aggregate_type="match_session",
            aggregate_id=session_id,
            payload={
                "session_id": session_id,
                "disconnected_actor": disconnected_actor,
                "reconnect_deadline_utc": deadline,
            },
            caused_by=disconnected_actor,
        )
        return self.get_session(session_id)

    def abandon_session(
        self,
        *,
        session_id: str,
        reason: str = "reconnect_timeout",
        actor_id: str = "lobby_watchdog",
    ) -> dict[str, Any]:
        """Abandon an interrupted official session and escalate the fixture to `awaiting_admin`."""
        sess = self.get_session(session_id)
        if sess["state"] in ("completed", "abandoned"):
            return sess
        ts = utc_now_iso()
        fixture_id = sess["fixture_id"]
        world_id = sess["world_id"]

        self.store.conn.execute(
            "UPDATE match_session SET state = 'abandoned', ended_at = ? WHERE id = ?",
            (ts, session_id),
        )
        if sess["kind"] == "official":
            self.store.conn.execute(
                "UPDATE fixture SET state = 'awaiting_admin' WHERE id = ?",
                (fixture_id,),
            )
        self.store.append_event(
            world_id=world_id,
            event_type="session_abandoned",
            aggregate_type="match_session",
            aggregate_id=session_id,
            payload={
                "session_id": session_id,
                "fixture_id": fixture_id,
                "reason": reason,
            },
            caused_by=actor_id,
            occurred_at=ts,
            commit=False,
        )
        if sess["kind"] == "official":
            self.store.append_event(
                world_id=world_id,
                event_type="admin_resolution_required",
                aggregate_type="fixture",
                aggregate_id=fixture_id,
                payload={
                    "fixture_id": fixture_id,
                    "session_id": session_id,
                    "reason": f"session_abandoned:{reason}",
                },
                caused_by=actor_id,
                occurred_at=ts,
                commit=False,
            )
        self.store.conn.commit()
        return self.get_session(session_id)
