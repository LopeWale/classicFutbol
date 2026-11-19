"""SQLite-backed relational domain tables and strictly append-only `world_event` ledger."""

from __future__ import annotations

import json
import sqlite3
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


def utc_now_iso() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


SCHEMA_SQL = """
PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS world (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    competition_name TEXT NOT NULL,
    season_label TEXT NOT NULL,
    season_timezone TEXT NOT NULL,
    default_match_format TEXT NOT NULL DEFAULT '5v5',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS participant (
    id TEXT PRIMARY KEY,
    display_name TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS club_assignment (
    id TEXT PRIMARY KEY,
    world_id TEXT NOT NULL REFERENCES world(id),
    club_id TEXT NOT NULL,
    participant_id TEXT NOT NULL REFERENCES participant(id),
    effective_from_round INTEGER NOT NULL,
    effective_from TEXT NOT NULL,
    effective_to TEXT NULL,
    UNIQUE(world_id, club_id, effective_to)
);

CREATE TABLE IF NOT EXISTS fixture (
    id TEXT PRIMARY KEY,
    world_id TEXT NOT NULL REFERENCES world(id),
    round INTEGER NOT NULL,
    home_club_id TEXT NOT NULL,
    away_club_id TEXT NOT NULL,
    kickoff_utc TEXT NOT NULL,
    window_open_utc TEXT NOT NULL,
    window_close_utc TEXT NOT NULL,
    result_source TEXT NOT NULL CHECK (
        result_source IN ('played_vs_friend', 'played_vs_cpu', 'imported', 'generated', 'admin')
    ),
    state TEXT NOT NULL CHECK (
        state IN ('scheduled', 'open', 'in_session', 'played', 'accepted', 'void', 'postponed', 'awaiting_admin')
    ),
    real_home_goals INTEGER NULL,
    real_away_goals INTEGER NULL
);

CREATE TABLE IF NOT EXISTS fixture_snapshot (
    fixture_id TEXT PRIMARY KEY REFERENCES fixture(id),
    snapshot_json TEXT NOT NULL,
    seed INTEGER NOT NULL,
    format_name TEXT NOT NULL,
    gameplay_version TEXT NOT NULL,
    taken_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS match_session (
    id TEXT PRIMARY KEY,
    fixture_id TEXT NOT NULL REFERENCES fixture(id),
    world_id TEXT NOT NULL REFERENCES world(id),
    kind TEXT NOT NULL CHECK (kind IN ('official', 'friendly')),
    state TEXT NOT NULL CHECK (state IN ('created', 'running', 'completed', 'abandoned')),
    started_at TEXT NOT NULL,
    ended_at TEXT NULL,
    format_name TEXT NOT NULL,
    gameplay_version TEXT NOT NULL,
    final_hash_hex TEXT NULL
);

CREATE TABLE IF NOT EXISTS accepted_result (
    fixture_id TEXT PRIMARY KEY REFERENCES fixture(id),
    world_id TEXT NOT NULL REFERENCES world(id),
    session_id TEXT NULL REFERENCES match_session(id),
    source TEXT NOT NULL,
    home_goals INTEGER NOT NULL,
    away_goals INTEGER NOT NULL,
    final_hash_hex TEXT NULL,
    accepted_at TEXT NOT NULL,
    event_seq INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS world_event (
    seq INTEGER PRIMARY KEY AUTOINCREMENT,
    world_id TEXT NOT NULL REFERENCES world(id),
    type TEXT NOT NULL,
    aggregate_type TEXT NOT NULL,
    aggregate_id TEXT NOT NULL,
    payload_json TEXT NOT NULL,
    caused_by TEXT NOT NULL,
    occurred_at TEXT NOT NULL,
    supersedes_seq INTEGER NULL REFERENCES world_event(seq)
);

CREATE TRIGGER IF NOT EXISTS trg_world_event_no_update
BEFORE UPDATE ON world_event
BEGIN
    SELECT RAISE(ABORT, 'world_event ledger is strictly append-only; use a superseding event');
END;

CREATE TRIGGER IF NOT EXISTS trg_world_event_no_delete
BEFORE DELETE ON world_event
BEGIN
    SELECT RAISE(ABORT, 'world_event ledger is strictly append-only; deletion is forbidden');
END;

CREATE TABLE IF NOT EXISTS import_payload (
    id TEXT PRIMARY KEY,
    provider TEXT NOT NULL,
    endpoint TEXT NOT NULL,
    retrieved_at TEXT NOT NULL,
    raw_json TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS import_proposal (
    id TEXT PRIMARY KEY,
    payload_id TEXT NOT NULL REFERENCES import_payload(id),
    world_id TEXT NOT NULL REFERENCES world(id),
    target_type TEXT NOT NULL,
    target_id TEXT NOT NULL,
    proposed_json TEXT NOT NULL,
    decision TEXT NOT NULL CHECK (decision IN ('applied', 'parked', 'rejected')),
    reason TEXT NOT NULL,
    decided_at TEXT NOT NULL
);
"""


class WorldLedgerStore:
    """Thread-safe SQLite store backing the Parallel Season FC World API."""

    def __init__(self, db_path: str | Path = ":memory:") -> None:
        self.db_path = str(db_path)
        self._conn = sqlite3.connect(self.db_path, check_same_thread=False)
        self._conn.row_factory = sqlite3.Row
        self._conn.executescript(SCHEMA_SQL)
        self._conn.commit()

    @property
    def conn(self) -> sqlite3.Connection:
        return self._conn

    def close(self) -> None:
        self._conn.close()

    # --- Ledger append & query ---

    def append_event(
        self,
        *,
        world_id: str,
        event_type: str,
        aggregate_type: str,
        aggregate_id: str,
        payload: dict[str, Any],
        caused_by: str,
        occurred_at: str | None = None,
        supersedes_seq: int | None = None,
        commit: bool = True,
    ) -> dict[str, Any]:
        ts = occurred_at or utc_now_iso()
        cur = self._conn.execute(
            """
            INSERT INTO world_event (
                world_id, type, aggregate_type, aggregate_id,
                payload_json, caused_by, occurred_at, supersedes_seq
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            """,
            (
                world_id,
                event_type,
                aggregate_type,
                aggregate_id,
                json.dumps(payload, separators=(",", ":"), sort_keys=True),
                caused_by,
                ts,
                supersedes_seq,
            ),
        )
        if commit:
            self._conn.commit()
        seq = int(cur.lastrowid)
        return {
            "seq": seq,
            "world_id": world_id,
            "type": event_type,
            "aggregate_type": aggregate_type,
            "aggregate_id": aggregate_id,
            "payload": payload,
            "caused_by": caused_by,
            "occurred_at": ts,
            "supersedes_seq": supersedes_seq,
        }

    def list_events(self, world_id: str, since_seq: int = 0) -> list[dict[str, Any]]:
        rows = self._conn.execute(
            """
            SELECT seq, world_id, type, aggregate_type, aggregate_id,
                   payload_json, caused_by, occurred_at, supersedes_seq
            FROM world_event
            WHERE world_id = ? AND seq > ?
            ORDER BY seq ASC
            """,
            (world_id, since_seq),
        ).fetchall()
        return [
            {
                "seq": r["seq"],
                "world_id": r["world_id"],
                "type": r["type"],
                "aggregate_type": r["aggregate_type"],
                "aggregate_id": r["aggregate_id"],
                "payload": json.loads(r["payload_json"]),
                "caused_by": r["caused_by"],
                "occurred_at": r["occurred_at"],
                "supersedes_seq": r["supersedes_seq"],
            }
            for r in rows
        ]

    # --- World, Participants, & Club Assignments ---

    def create_world(
        self,
        *,
        world_id: str,
        name: str,
        competition_name: str,
        season_label: str,
        season_timezone: str = "Europe/London",
        default_match_format: str = "5v5",
        caused_by: str = "system",
    ) -> dict[str, Any]:
        ts = utc_now_iso()
        self._conn.execute(
            """
            INSERT INTO world (id, name, competition_name, season_label, season_timezone, default_match_format, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            """,
            (world_id, name, competition_name, season_label, season_timezone, default_match_format, ts),
        )
        self.append_event(
            world_id=world_id,
            event_type="competition_defined",
            aggregate_type="world",
            aggregate_id=world_id,
            payload={
                "name": name,
                "competition_name": competition_name,
                "season_label": season_label,
                "default_match_format": default_match_format,
            },
            caused_by=caused_by,
            occurred_at=ts,
        )
        return self.get_world(world_id)

    def get_world(self, world_id: str) -> dict[str, Any]:
        row = self._conn.execute("SELECT * FROM world WHERE id = ?", (world_id,)).fetchone()
        if row is None:
            raise KeyError(f"World '{world_id}' not found")
        return dict(row)

    def list_worlds(self) -> list[dict[str, Any]]:
        rows = self._conn.execute("SELECT * FROM world ORDER BY created_at ASC").fetchall()
        return [dict(r) for r in rows]

    def register_participant(self, participant_id: str, display_name: str, world_id: str) -> dict[str, Any]:
        ts = utc_now_iso()
        self._conn.execute(
            "INSERT OR IGNORE INTO participant (id, display_name, created_at) VALUES (?, ?, ?)",
            (participant_id, display_name, ts),
        )
        self.append_event(
            world_id=world_id,
            event_type="participant_joined",
            aggregate_type="participant",
            aggregate_id=participant_id,
            payload={"participant_id": participant_id, "display_name": display_name},
            caused_by=participant_id,
            occurred_at=ts,
        )
        return {"id": participant_id, "display_name": display_name, "created_at": ts}

    def assign_club(
        self,
        *,
        assignment_id: str,
        world_id: str,
        club_id: str,
        participant_id: str,
        effective_from_round: int,
        baseline_summary: dict[str, Any] | None = None,
    ) -> dict[str, Any]:
        active = self._conn.execute(
            """
            SELECT id, participant_id FROM club_assignment
            WHERE world_id = ? AND club_id = ? AND effective_to IS NULL
            """,
            (world_id, club_id),
        ).fetchone()
        if active is not None:
            raise ValueError(f"Club '{club_id}' is already assigned to '{active['participant_id']}'")

        ts = utc_now_iso()
        self._conn.execute(
            """
            INSERT INTO club_assignment (id, world_id, club_id, participant_id, effective_from_round, effective_from, effective_to)
            VALUES (?, ?, ?, ?, ?, ?, NULL)
            """,
            (assignment_id, world_id, club_id, participant_id, effective_from_round, ts),
        )
        self.append_event(
            world_id=world_id,
            event_type="club_assigned",
            aggregate_type="club_assignment",
            aggregate_id=assignment_id,
            payload={
                "assignment_id": assignment_id,
                "club_id": club_id,
                "participant_id": participant_id,
                "effective_from_round": effective_from_round,
            },
            caused_by=participant_id,
            occurred_at=ts,
            commit=False,
        )
        self.append_event(
            world_id=world_id,
            event_type="takeover_snapshot_recorded",
            aggregate_type="club",
            aggregate_id=club_id,
            payload={
                "club_id": club_id,
                "participant_id": participant_id,
                "effective_from_round": effective_from_round,
                "baseline": baseline_summary or {},
            },
            caused_by=participant_id,
            occurred_at=ts,
            commit=False,
        )
        # Recompute advance-assigned result_source for future open/scheduled fixtures!
        self._refresh_future_result_sources(world_id, from_round=effective_from_round, caused_by=participant_id)
        self._conn.commit()
        return self.list_assignments(world_id)[-1]

    def list_assignments(self, world_id: str) -> list[dict[str, Any]]:
        rows = self._conn.execute(
            """
            SELECT ca.*, p.display_name AS participant_name
            FROM club_assignment ca
            JOIN participant p ON p.id = ca.participant_id
            WHERE ca.world_id = ? AND ca.effective_to IS NULL
            ORDER BY ca.effective_from ASC
            """,
            (world_id,),
        ).fetchall()
        return [dict(r) for r in rows]

    def represented_clubs(self, world_id: str) -> dict[str, str]:
        """Return mapping `club_id -> participant_id` for active club assignments."""
        rows = self._conn.execute(
            "SELECT club_id, participant_id FROM club_assignment WHERE world_id = ? AND effective_to IS NULL",
            (world_id,),
        ).fetchall()
        return {r["club_id"]: r["participant_id"] for r in rows}

    def _refresh_future_result_sources(self, world_id: str, from_round: int, caused_by: str) -> None:
        rep = self.represented_clubs(world_id)
        fixtures = self._conn.execute(
            """
            SELECT id, home_club_id, away_club_id, result_source, state
            FROM fixture
            WHERE world_id = ? AND round >= ? AND state IN ('scheduled', 'open')
            """,
            (world_id, from_round),
        ).fetchall()
        for f in fixtures:
            h_rep = f["home_club_id"] in rep
            a_rep = f["away_club_id"] in rep
            if h_rep and a_rep:
                new_src = "played_vs_friend"
            elif h_rep or a_rep:
                new_src = "played_vs_cpu"
            else:
                new_src = "imported"
            if new_src != f["result_source"]:
                self._conn.execute(
                    "UPDATE fixture SET result_source = ? WHERE id = ?",
                    (new_src, f["id"]),
                )
                self.append_event(
                    world_id=world_id,
                    event_type="result_source_assigned",
                    aggregate_type="fixture",
                    aggregate_id=f["id"],
                    payload={"fixture_id": f["id"], "result_source": new_src},
                    caused_by=caused_by,
                    commit=False,
                )

    # --- Fixtures & Results ---

    def create_fixture(
        self,
        *,
        fixture_id: str,
        world_id: str,
        round_no: int,
        home_club_id: str,
        away_club_id: str,
        kickoff_utc: str,
        window_open_utc: str,
        window_close_utc: str,
        result_source: str,
        state: str = "open",
        real_home_goals: int | None = None,
        real_away_goals: int | None = None,
        caused_by: str = "system",
    ) -> dict[str, Any]:
        self._conn.execute(
            """
            INSERT INTO fixture (
                id, world_id, round, home_club_id, away_club_id,
                kickoff_utc, window_open_utc, window_close_utc,
                result_source, state, real_home_goals, real_away_goals
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
            (
                fixture_id,
                world_id,
                round_no,
                home_club_id,
                away_club_id,
                kickoff_utc,
                window_open_utc,
                window_close_utc,
                result_source,
                state,
                real_home_goals,
                real_away_goals,
            ),
        )
        self.append_event(
            world_id=world_id,
            event_type="fixture_created",
            aggregate_type="fixture",
            aggregate_id=fixture_id,
            payload={
                "fixture_id": fixture_id,
                "round": round_no,
                "home_club_id": home_club_id,
                "away_club_id": away_club_id,
                "kickoff_utc": kickoff_utc,
                "result_source": result_source,
                "real_reference_score": (
                    [real_home_goals, real_away_goals]
                    if real_home_goals is not None and real_away_goals is not None
                    else None
                ),
            },
            caused_by=caused_by,
        )
        return self.get_fixture(fixture_id)

    def get_fixture(self, fixture_id: str) -> dict[str, Any]:
        row = self._conn.execute(
            """
            SELECT f.*,
                   ar.home_goals AS accepted_home_goals,
                   ar.away_goals AS accepted_away_goals,
                   ar.source AS accepted_source,
                   ar.final_hash_hex AS accepted_hash_hex,
                   ar.accepted_at AS accepted_at,
                   ar.event_seq AS accepted_event_seq
            FROM fixture f
            LEFT JOIN accepted_result ar ON ar.fixture_id = f.id
            WHERE f.id = ?
            """,
            (fixture_id,),
        ).fetchone()
        if row is None:
            raise KeyError(f"Fixture '{fixture_id}' not found")
        return dict(row)

    def list_fixtures(self, world_id: str) -> list[dict[str, Any]]:
        rows = self._conn.execute(
            """
            SELECT f.*,
                   ar.home_goals AS accepted_home_goals,
                   ar.away_goals AS accepted_away_goals,
                   ar.source AS accepted_source,
                   ar.final_hash_hex AS accepted_hash_hex,
                   ar.accepted_at AS accepted_at,
                   ar.event_seq AS accepted_event_seq
            FROM fixture f
            LEFT JOIN accepted_result ar ON ar.fixture_id = f.id
            WHERE f.world_id = ?
            ORDER BY f.round ASC, f.kickoff_utc ASC, f.id ASC
            """,
            (world_id,),
        ).fetchall()
        return [dict(r) for r in rows]

    def get_frozen_snapshot(self, fixture_id: str) -> dict[str, Any] | None:
        row = self._conn.execute(
            "SELECT * FROM fixture_snapshot WHERE fixture_id = ?",
            (fixture_id,),
        ).fetchone()
        if row is None:
            return None
        d = dict(row)
        d["snapshot"] = json.loads(d.pop("snapshot_json"))
        return d

    def get_accepted_result(self, fixture_id: str) -> dict[str, Any] | None:
        row = self._conn.execute(
            "SELECT * FROM accepted_result WHERE fixture_id = ?",
            (fixture_id,),
        ).fetchone()
        return dict(row) if row is not None else None

    def correct_result_admin(
        self,
        *,
        fixture_id: str,
        new_home_goals: int,
        new_away_goals: int,
        new_events: list[dict[str, Any]] | None = None,
        actor_id: str = "commissioner",
        reason: str = "commissioner ruling",
    ) -> dict[str, Any]:
        """Append `result_corrected` superseding the prior result event so projections rebuild cleanly."""
        fx = self.get_fixture(fixture_id)
        existing = self.get_accepted_result(fixture_id)
        if existing is None:
            raise ValueError(f"Fixture '{fixture_id}' has no accepted result to correct")

        old_seq = int(existing["event_seq"])
        world_id = fx["world_id"]
        ts = utc_now_iso()
        corr_ev = self.append_event(
            world_id=world_id,
            event_type="result_corrected",
            aggregate_type="fixture",
            aggregate_id=fixture_id,
            payload={
                "fixture_id": fixture_id,
                "home_club_id": fx["home_club_id"],
                "away_club_id": fx["away_club_id"],
                "home_goals": new_home_goals,
                "away_goals": new_away_goals,
                "source": "admin",
                "reason": reason,
                "events": new_events or [],
            },
            caused_by=actor_id,
            occurred_at=ts,
            supersedes_seq=old_seq,
            commit=False,
        )
        self._conn.execute(
            """
            UPDATE accepted_result
            SET source = 'admin',
                home_goals = ?,
                away_goals = ?,
                accepted_at = ?,
                event_seq = ?
            WHERE fixture_id = ?
            """,
            (new_home_goals, new_away_goals, ts, corr_ev["seq"], fixture_id),
        )
        self._conn.commit()
        return self.get_accepted_result(fixture_id) or {}
