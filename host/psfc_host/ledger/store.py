"""SQLite-backed relational domain tables and strictly append-only `world_event` ledger."""

from __future__ import annotations

import json
import sqlite3
import uuid
from collections.abc import Iterator
from contextlib import contextmanager
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
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'completed')),
    parent_world_id TEXT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS competition (
    id TEXT PRIMARY KEY,
    world_id TEXT NOT NULL REFERENCES world(id),
    name TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('league', 'knockout')),
    format_name TEXT NOT NULL DEFAULT '5v5',
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'completed')),
    real_final_pair_json TEXT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS participant (
    id TEXT PRIMARY KEY,
    display_name TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS device (
    id TEXT PRIMARY KEY,
    participant_id TEXT NOT NULL REFERENCES participant(id),
    label TEXT NOT NULL,
    secret_hash TEXT NOT NULL,
    revoked INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    revoked_at TEXT NULL
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
    competition_id TEXT NULL REFERENCES competition(id),
    stage TEXT NOT NULL DEFAULT 'league',
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
    extra_time_enabled INTEGER NOT NULL DEFAULT 0,
    real_home_goals INTEGER NULL,
    real_away_goals INTEGER NULL
);

CREATE TABLE IF NOT EXISTS appointment (
    id TEXT PRIMARY KEY,
    fixture_id TEXT NOT NULL REFERENCES fixture(id),
    world_id TEXT NOT NULL REFERENCES world(id),
    proposed_by TEXT NOT NULL REFERENCES participant(id),
    proposed_at_utc TEXT NOT NULL,
    accepted_by TEXT NULL REFERENCES participant(id),
    state TEXT NOT NULL CHECK (state IN ('proposed', 'accepted', 'declined', 'expired')),
    created_at TEXT NOT NULL
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
    state TEXT NOT NULL CHECK (
        state IN ('created', 'ready_check', 'running', 'paused', 'completed', 'abandoned')
    ),
    home_ready INTEGER NOT NULL DEFAULT 0,
    away_ready INTEGER NOT NULL DEFAULT 0,
    pause_budget_remaining INTEGER NOT NULL DEFAULT 3,
    reconnect_deadline_utc TEXT NULL,
    started_at TEXT NOT NULL,
    ended_at TEXT NULL,
    format_name TEXT NOT NULL,
    gameplay_version TEXT NOT NULL,
    final_hash_hex TEXT NULL
);

CREATE TABLE IF NOT EXISTS match_input_log (
    session_id TEXT NOT NULL REFERENCES match_session(id),
    tick INTEGER NOT NULL,
    side TEXT NOT NULL CHECK (side IN ('home', 'away')),
    input_hex TEXT NOT NULL,
    PRIMARY KEY (session_id, tick, side)
);

CREATE TABLE IF NOT EXISTS accepted_result (
    fixture_id TEXT PRIMARY KEY REFERENCES fixture(id),
    world_id TEXT NOT NULL REFERENCES world(id),
    session_id TEXT NULL REFERENCES match_session(id),
    source TEXT NOT NULL,
    home_goals INTEGER NOT NULL,
    away_goals INTEGER NOT NULL,
    went_to_extra_time INTEGER NOT NULL DEFAULT 0,
    shootout_home INTEGER NULL,
    shootout_away INTEGER NULL,
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

CREATE TABLE IF NOT EXISTS external_identity (
    provider TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    external_id TEXT NOT NULL,
    internal_id TEXT NOT NULL,
    resolved_at TEXT NOT NULL,
    PRIMARY KEY (provider, entity_type, external_id)
);

CREATE TABLE IF NOT EXISTS identity_resolution_queue (
    id TEXT PRIMARY KEY,
    world_id TEXT NOT NULL REFERENCES world(id),
    provider TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    external_id TEXT NOT NULL,
    display_hint TEXT NOT NULL,
    raw_context_json TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('pending', 'resolved')),
    resolved_internal_id TEXT NULL,
    created_at TEXT NOT NULL,
    resolved_at TEXT NULL
);

CREATE TABLE IF NOT EXISTS cartridge_revision (
    id TEXT PRIMARY KEY,
    device_id TEXT NOT NULL REFERENCES device(id),
    participant_id TEXT NOT NULL REFERENCES participant(id),
    world_id TEXT NOT NULL REFERENCES world(id),
    rev_number INTEGER NOT NULL,
    last_event_seq INTEGER NOT NULL,
    manifest_hash TEXT NOT NULL,
    signature TEXT NOT NULL,
    manifest_json TEXT NOT NULL,
    issued_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS season_honour (
    id TEXT PRIMARY KEY,
    world_id TEXT NOT NULL REFERENCES world(id),
    season_label TEXT NOT NULL,
    category TEXT NOT NULL,
    winner_id TEXT NOT NULL,
    winner_name TEXT NOT NULL,
    detail_json TEXT NOT NULL,
    awarded_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS club_lineup_5v5 (
    world_id TEXT NOT NULL REFERENCES world(id),
    club_id TEXT NOT NULL,
    starter_ids_json TEXT NOT NULL,
    formation_5v5 TEXT NOT NULL,
    updated_by TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (world_id, club_id)
);
"""


class WorldLedgerStore:
    """Thread-safe SQLite store backing the Parallel Season FC World API."""

    def __init__(self, db_path: str | Path = ":memory:") -> None:
        self.db_path = str(db_path)
        self._batch_depth = 0
        self._conn = sqlite3.connect(self.db_path, check_same_thread=False)
        self._conn.row_factory = sqlite3.Row
        self._conn.executescript(SCHEMA_SQL)
        self._conn.commit()

    @property
    def conn(self) -> sqlite3.Connection:
        return self._conn

    def commit(self) -> None:
        if self._batch_depth == 0:
            self._conn.commit()

    @contextmanager
    def batch(self) -> Iterator[None]:
        self._batch_depth += 1
        try:
            yield
        except Exception:
            self._batch_depth -= 1
            if self._batch_depth == 0:
                self._conn.rollback()
            raise
        else:
            self._batch_depth -= 1
            if self._batch_depth == 0:
                self._conn.commit()

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
            self.commit()
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

    # --- World, Competitions, Participants, & Club Assignments ---

    def create_world(
        self,
        *,
        world_id: str,
        name: str,
        competition_name: str,
        season_label: str,
        season_timezone: str = "Europe/London",
        default_match_format: str = "5v5",
        parent_world_id: str | None = None,
        caused_by: str = "system",
    ) -> dict[str, Any]:
        ts = utc_now_iso()
        self._conn.execute(
            """
            INSERT INTO world (
                id, name, competition_name, season_label, season_timezone,
                default_match_format, status, parent_world_id, created_at
            )
            VALUES (?, ?, ?, ?, ?, ?, 'active', ?, ?)
            """,
            (
                world_id,
                name,
                competition_name,
                season_label,
                season_timezone,
                default_match_format,
                parent_world_id,
                ts,
            ),
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
                "parent_world_id": parent_world_id,
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

    def create_competition(
        self,
        *,
        competition_id: str,
        world_id: str,
        name: str,
        kind: str = "league",
        format_name: str = "5v5",
        real_final_pair: list[str] | None = None,
        caused_by: str = "commissioner",
    ) -> dict[str, Any]:
        ts = utc_now_iso()
        self._conn.execute(
            """
            INSERT INTO competition (id, world_id, name, kind, format_name, status, real_final_pair_json, created_at)
            VALUES (?, ?, ?, ?, ?, 'active', ?, ?)
            """,
            (
                competition_id,
                world_id,
                name,
                kind,
                format_name,
                json.dumps(real_final_pair) if real_final_pair else None,
                ts,
            ),
        )
        self.append_event(
            world_id=world_id,
            event_type="competition_defined",
            aggregate_type="competition",
            aggregate_id=competition_id,
            payload={
                "competition_id": competition_id,
                "name": name,
                "kind": kind,
                "format_name": format_name,
                "real_final_pair": real_final_pair,
            },
            caused_by=caused_by,
            occurred_at=ts,
        )
        return self.get_competition(competition_id)

    def get_competition(self, competition_id: str) -> dict[str, Any]:
        row = self._conn.execute(
            "SELECT * FROM competition WHERE id = ?",
            (competition_id,),
        ).fetchone()
        if row is None:
            raise KeyError(f"Competition '{competition_id}' not found")
        d = dict(row)
        d["real_final_pair"] = (
            json.loads(d["real_final_pair_json"]) if d.get("real_final_pair_json") else None
        )
        return d

    def list_competitions(self, world_id: str) -> list[dict[str, Any]]:
        rows = self._conn.execute(
            "SELECT * FROM competition WHERE world_id = ? ORDER BY created_at ASC",
            (world_id,),
        ).fetchall()
        out = []
        for r in rows:
            d = dict(r)
            d["real_final_pair"] = (
                json.loads(d["real_final_pair_json"]) if d.get("real_final_pair_json") else None
            )
            out.append(d)
        return out

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
        self._refresh_future_result_sources(world_id, from_round=effective_from_round, caused_by=participant_id)
        self.commit()
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

    def compute_advance_result_source(
        self,
        world_id: str,
        home_club_id: str,
        away_club_id: str,
        *,
        has_real_equivalent: bool = True,
    ) -> str:
        rep = self.represented_clubs(world_id)
        h_rep = home_club_id in rep
        a_rep = away_club_id in rep
        if h_rep and a_rep:
            return "played_vs_friend"
        if h_rep or a_rep:
            return "played_vs_cpu"
        return "imported" if has_real_equivalent else "generated"

    def _refresh_future_result_sources(self, world_id: str, from_round: int, caused_by: str) -> None:
        fixtures = self._conn.execute(
            """
            SELECT id, stage, round, home_club_id, away_club_id, result_source, state, real_home_goals
            FROM fixture
            WHERE world_id = ? AND state IN ('scheduled', 'open')
              AND (stage != 'league' OR round >= ?)
            """,
            (world_id, from_round),
        ).fetchall()
        for f in fixtures:
            if f["result_source"] == "admin":
                continue
            has_real = f["result_source"] != "generated"
            new_src = self.compute_advance_result_source(
                world_id,
                f["home_club_id"],
                f["away_club_id"],
                has_real_equivalent=has_real,
            )
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

    # --- Fixtures, Deadlines, & Results ---

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
        competition_id: str | None = None,
        stage: str = "league",
        extra_time_enabled: bool = False,
        real_home_goals: int | None = None,
        real_away_goals: int | None = None,
        caused_by: str = "system",
    ) -> dict[str, Any]:
        self._conn.execute(
            """
            INSERT INTO fixture (
                id, world_id, competition_id, stage, round, home_club_id, away_club_id,
                kickoff_utc, window_open_utc, window_close_utc,
                result_source, state, extra_time_enabled, real_home_goals, real_away_goals
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
            (
                fixture_id,
                world_id,
                competition_id,
                stage,
                round_no,
                home_club_id,
                away_club_id,
                kickoff_utc,
                window_open_utc,
                window_close_utc,
                result_source,
                state,
                int(extra_time_enabled),
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
                "competition_id": competition_id,
                "stage": stage,
                "round": round_no,
                "home_club_id": home_club_id,
                "away_club_id": away_club_id,
                "kickoff_utc": kickoff_utc,
                "result_source": result_source,
                "extra_time_enabled": extra_time_enabled,
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
                   ar.went_to_extra_time AS accepted_aet,
                   ar.shootout_home AS accepted_shootout_home,
                   ar.shootout_away AS accepted_shootout_away,
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

    def list_fixtures(
        self,
        world_id: str,
        competition_id: str | None = None,
    ) -> list[dict[str, Any]]:
        if competition_id:
            rows = self._conn.execute(
                """
                SELECT f.*,
                       ar.home_goals AS accepted_home_goals,
                       ar.away_goals AS accepted_away_goals,
                       ar.went_to_extra_time AS accepted_aet,
                       ar.shootout_home AS accepted_shootout_home,
                       ar.shootout_away AS accepted_shootout_away,
                       ar.source AS accepted_source,
                       ar.final_hash_hex AS accepted_hash_hex,
                       ar.accepted_at AS accepted_at,
                       ar.event_seq AS accepted_event_seq
                FROM fixture f
                LEFT JOIN accepted_result ar ON ar.fixture_id = f.id
                WHERE f.world_id = ? AND f.competition_id = ?
                ORDER BY f.round ASC, f.kickoff_utc ASC, f.id ASC
                """,
                (world_id, competition_id),
            ).fetchall()
        else:
            rows = self._conn.execute(
                """
                SELECT f.*,
                       ar.home_goals AS accepted_home_goals,
                       ar.away_goals AS accepted_away_goals,
                       ar.went_to_extra_time AS accepted_aet,
                       ar.shootout_home AS accepted_shootout_home,
                       ar.shootout_away AS accepted_shootout_away,
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

    def expire_overdue_windows(
        self,
        world_id: str,
        as_of_utc: str | None = None,
        actor_id: str = "scheduler",
    ) -> list[dict[str, Any]]:
        """Transition expired unplayed represented fixtures to `awaiting_admin` without substituting real scores."""
        cutoff = as_of_utc or utc_now_iso()
        overdue = self._conn.execute(
            """
            SELECT * FROM fixture
            WHERE world_id = ?
              AND state IN ('scheduled', 'open')
              AND result_source IN ('played_vs_friend', 'played_vs_cpu')
              AND window_close_utc < ?
            """,
            (world_id, cutoff),
        ).fetchall()
        transitioned: list[dict[str, Any]] = []
        for row in overdue:
            fid = row["id"]
            self._conn.execute(
                "UPDATE fixture SET state = 'awaiting_admin' WHERE id = ?",
                (fid,),
            )
            self.append_event(
                world_id=world_id,
                event_type="admin_resolution_required",
                aggregate_type="fixture",
                aggregate_id=fid,
                payload={
                    "fixture_id": fid,
                    "reason": "playing_window_expired",
                    "window_close_utc": row["window_close_utc"],
                    "as_of_utc": cutoff,
                    "result_source": row["result_source"],
                },
                caused_by=actor_id,
                commit=False,
            )
            transitioned.append(fid)
        self.commit()
        return [self.get_fixture(fid) for fid in transitioned]

    def resolve_awaiting_admin_fixture(
        self,
        *,
        fixture_id: str,
        home_goals: int,
        away_goals: int,
        actor_id: str = "commissioner",
        reason: str = "commissioner walkover / ruling",
    ) -> dict[str, Any]:
        fx = self.get_fixture(fixture_id)
        if self.get_accepted_result(fixture_id) is not None:
            return self.correct_result_admin(
                fixture_id=fixture_id,
                new_home_goals=home_goals,
                new_away_goals=away_goals,
                actor_id=actor_id,
                reason=reason,
            )
        ts = utc_now_iso()
        world_id = fx["world_id"]
        res_ev = self.append_event(
            world_id=world_id,
            event_type="result_accepted",
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
                "source": "admin",
                "reason": reason,
            },
            caused_by=actor_id,
            occurred_at=ts,
            commit=False,
        )
        self._conn.execute(
            """
            INSERT INTO accepted_result (
                fixture_id, world_id, session_id, source,
                home_goals, away_goals, went_to_extra_time, shootout_home, shootout_away,
                final_hash_hex, accepted_at, event_seq
            )
            VALUES (?, ?, NULL, 'admin', ?, ?, 0, NULL, NULL, NULL, ?, ?)
            """,
            (fixture_id, world_id, home_goals, away_goals, ts, res_ev["seq"]),
        )
        self._conn.execute(
            "UPDATE fixture SET state = 'accepted', result_source = 'admin' WHERE id = ?",
            (fixture_id,),
        )
        self.commit()
        return self.get_accepted_result(fixture_id) or {}

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
                "competition_id": fx.get("competition_id"),
                "stage": fx.get("stage", "league"),
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
        self.commit()
        return self.get_accepted_result(fixture_id) or {}

    def record_honour(
        self,
        *,
        world_id: str,
        season_label: str,
        category: str,
        winner_id: str,
        winner_name: str,
        detail: dict[str, Any],
        awarded_at: str,
    ) -> dict[str, Any]:
        hid = f"hon-{uuid.uuid4().hex[:10]}"
        self._conn.execute(
            """
            INSERT INTO season_honour (id, world_id, season_label, category, winner_id, winner_name, detail_json, awarded_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            """,
            (
                hid,
                world_id,
                season_label,
                category,
                winner_id,
                winner_name,
                json.dumps(detail, separators=(",", ":"), sort_keys=True),
                awarded_at,
            ),
        )
        return {
            "id": hid,
            "world_id": world_id,
            "season_label": season_label,
            "category": category,
            "winner_id": winner_id,
            "winner_name": winner_name,
            "detail": detail,
            "awarded_at": awarded_at,
        }

    def list_honours(self, world_id: str) -> list[dict[str, Any]]:
        rows = self._conn.execute(
            "SELECT * FROM season_honour WHERE world_id = ? ORDER BY awarded_at ASC, id ASC",
            (world_id,),
        ).fetchall()
        out = []
        for r in rows:
            d = dict(r)
            d["detail"] = json.loads(d.pop("detail_json"))
            out.append(d)
        return out

    # --- 5v5 Squad Selection (11 -> 5 Lineup & Formation Persistence) ---

    def save_club_lineup_5v5(
        self,
        *,
        world_id: str,
        club_id: str,
        starter_ids: list[str],
        formation_5v5: str,
        actor_id: str = "participant",
    ) -> dict[str, Any]:
        self.get_world(world_id)
        ts = utc_now_iso()
        self._conn.execute(
            """
            INSERT INTO club_lineup_5v5 (world_id, club_id, starter_ids_json, formation_5v5, updated_by, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(world_id, club_id) DO UPDATE SET
                starter_ids_json = excluded.starter_ids_json,
                formation_5v5 = excluded.formation_5v5,
                updated_by = excluded.updated_by,
                updated_at = excluded.updated_at
            """,
            (
                world_id,
                club_id,
                json.dumps(starter_ids, separators=(",", ":")),
                formation_5v5,
                actor_id,
                ts,
            ),
        )
        self.append_event(
            world_id=world_id,
            event_type="lineup_updated",
            aggregate_type="club",
            aggregate_id=club_id,
            payload={
                "club_id": club_id,
                "starter_ids": starter_ids,
                "formation_5v5": formation_5v5,
            },
            caused_by=actor_id,
            occurred_at=ts,
            commit=True,
        )
        return {
            "world_id": world_id,
            "club_id": club_id,
            "starter_ids": starter_ids,
            "formation_5v5": formation_5v5,
            "updated_by": actor_id,
            "updated_at": ts,
        }

    def get_club_lineup_5v5(self, world_id: str, club_id: str) -> dict[str, Any] | None:
        row = self._conn.execute(
            "SELECT * FROM club_lineup_5v5 WHERE world_id = ? AND club_id = ?",
            (world_id, club_id),
        ).fetchone()
        if row is None:
            return None
        return {
            "world_id": row["world_id"],
            "club_id": row["club_id"],
            "starter_ids": json.loads(row["starter_ids_json"]),
            "formation_5v5": row["formation_5v5"],
            "updated_by": row["updated_by"],
            "updated_at": row["updated_at"],
        }

    def list_club_lineups_5v5(self, world_id: str) -> dict[str, dict[str, Any]]:
        rows = self._conn.execute(
            "SELECT * FROM club_lineup_5v5 WHERE world_id = ?",
            (world_id,),
        ).fetchall()
        return {
            r["club_id"]: {
                "world_id": r["world_id"],
                "club_id": r["club_id"],
                "starter_ids": json.loads(r["starter_ids_json"]),
                "formation_5v5": r["formation_5v5"],
                "updated_by": r["updated_by"],
                "updated_at": r["updated_at"],
            }
            for r in rows
        }

    def reset_world_for_demo(self, world_id: str) -> None:
        """Delete records strictly scoped to `world_id` (and rolled-over child worlds) without touching other worlds."""
        self.get_world(world_id)  # Raises KeyError if world_id does not exist

        # Include any seasons rolled over directly from this demo world
        target_worlds = [world_id]
        queue = [world_id]
        while queue:
            curr = queue.pop(0)
            children = self._conn.execute(
                "SELECT id FROM world WHERE parent_world_id = ?",
                (curr,),
            ).fetchall()
            for ch in children:
                cid = str(ch["id"])
                if cid not in target_worlds:
                    target_worlds.append(cid)
                    queue.append(cid)

        placeholders = ",".join("?" for _ in target_worlds)
        params = tuple(target_worlds)

        self._conn.execute("DROP TRIGGER IF EXISTS trg_world_event_no_delete")
        try:
            self._conn.execute(
                f"DELETE FROM match_input_log WHERE session_id IN (SELECT id FROM match_session WHERE world_id IN ({placeholders}))",
                params,
            )
            self._conn.execute(f"DELETE FROM accepted_result WHERE world_id IN ({placeholders})", params)
            self._conn.execute(f"DELETE FROM match_session WHERE world_id IN ({placeholders})", params)
            self._conn.execute(
                f"DELETE FROM fixture_snapshot WHERE fixture_id IN (SELECT id FROM fixture WHERE world_id IN ({placeholders}))",
                params,
            )
            self._conn.execute(f"DELETE FROM appointment WHERE world_id IN ({placeholders})", params)
            self._conn.execute(f"DELETE FROM fixture WHERE world_id IN ({placeholders})", params)
            self._conn.execute(f"DELETE FROM competition WHERE world_id IN ({placeholders})", params)
            self._conn.execute(f"DELETE FROM club_assignment WHERE world_id IN ({placeholders})", params)
            self._conn.execute(f"DELETE FROM club_lineup_5v5 WHERE world_id IN ({placeholders})", params)
            self._conn.execute(f"DELETE FROM season_honour WHERE world_id IN ({placeholders})", params)
            self._conn.execute(f"DELETE FROM cartridge_revision WHERE world_id IN ({placeholders})", params)
            self._conn.execute(f"DELETE FROM identity_resolution_queue WHERE world_id IN ({placeholders})", params)
            self._conn.execute(f"DELETE FROM import_proposal WHERE world_id IN ({placeholders})", params)
            self._conn.execute(f"DELETE FROM world_event WHERE world_id IN ({placeholders})", params)
            self._conn.execute(f"DELETE FROM world WHERE id IN ({placeholders})", params)
        finally:
            self._conn.execute(
                """
                CREATE TRIGGER IF NOT EXISTS trg_world_event_no_delete
                BEFORE DELETE ON world_event
                BEGIN
                    SELECT RAISE(ABORT, 'world_event ledger is strictly append-only; deletion is forbidden');
                END
                """
            )
        self._conn.commit()

