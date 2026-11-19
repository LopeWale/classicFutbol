"""Match-session coordinator and authoritative `psfc-replay` validator."""

from __future__ import annotations

import hashlib
import json
import os
import subprocess
import tempfile
import uuid
from pathlib import Path
from typing import Any

from psfc_host.catalogue.clubs import GAMEPLAY_VERSION, ClubCatalogue
from psfc_host.ledger.store import WorldLedgerStore, utc_now_iso


REPO_ROOT = Path(__file__).resolve().parents[3]
DEFAULT_REPLAY_BIN = REPO_ROOT / "sim" / "target" / "release" / "psfc-replay"


class ReplayValidationError(ValueError):
    """Raised when `psfc-replay` rejects a client-submitted input log or hash checkpoint."""


def resolve_replay_binary() -> Path:
    env_override = os.environ.get("PSFC_REPLAY_BIN")
    if env_override:
        p = Path(env_override)
        if p.is_file():
            return p
    if DEFAULT_REPLAY_BIN.is_file():
        return DEFAULT_REPLAY_BIN
    debug_bin = REPO_ROOT / "sim" / "target" / "debug" / "psfc-replay"
    if debug_bin.is_file():
        return debug_bin
    raise FileNotFoundError(
        f"psfc-replay binary not found at '{DEFAULT_REPLAY_BIN}'. Build with `cargo build --release -p psfc-replay`."
    )


def _deterministic_seed_for_fixture(fixture_id: str) -> int:
    digest = hashlib.sha256(fixture_id.encode("utf-8")).digest()
    # Keep within 53-bit safe integer range so browser JS numbers and Rust u64 agree losslessly
    return int.from_bytes(digest[:6], "little") or 20_261_118


class MatchSessionService:
    """Manages official competitive sessions, frozen snapshots, and deterministic replay validation."""

    def __init__(
        self,
        store: WorldLedgerStore,
        catalogue: ClubCatalogue,
        replay_bin: Path | None = None,
    ) -> None:
        self.store = store
        self.catalogue = catalogue
        self.replay_bin = replay_bin

    def _bin(self) -> Path:
        return self.replay_bin or resolve_replay_binary()

    def open_session(
        self,
        *,
        fixture_id: str,
        actor_id: str = "participant",
        format_name: str = "5v5",
        half_duration_ticks: int | None = None,
        home_controller: dict[str, Any] | None = None,
        away_controller: dict[str, Any] | None = None,
        seed: int | None = None,
    ) -> dict[str, Any]:
        fx = self.store.get_fixture(fixture_id)
        world_id = fx["world_id"]

        if fx["state"] == "accepted":
            raise ValueError(f"Fixture '{fixture_id}' already has an accepted result")
        if fx["result_source"] not in ("played_vs_friend", "played_vs_cpu"):
            raise ValueError(
                f"Fixture '{fixture_id}' has result_source='{fx['result_source']}', not playable in an official session"
            )

        # If snapshot was already frozen for this official fixture, preserve immutability!
        existing_snap = self.store.get_frozen_snapshot(fixture_id)
        if existing_snap is not None:
            active_sess = self.store.conn.execute(
                """
                SELECT * FROM match_session
                WHERE fixture_id = ? AND state IN ('created', 'ready_check', 'running', 'paused')
                ORDER BY started_at DESC LIMIT 1
                """,
                (fixture_id,),
            ).fetchone()
            if active_sess is not None:
                return {
                    "session_id": active_sess["id"],
                    "fixture_id": fixture_id,
                    "world_id": world_id,
                    "reused_frozen_snapshot": True,
                    "snapshot": existing_snap["snapshot"],
                }

        rep = self.store.represented_clubs(world_id)
        if home_controller is None:
            home_controller = (
                {"kind": "Human"}
                if fx["home_club_id"] in rep
                else {"kind": "Cpu", "difficulty": "Normal"}
            )
        if away_controller is None:
            if fx["result_source"] == "played_vs_friend":
                # Default to Human if 2P local or Cpu(Normal) fallback if solo testing; caller can specify
                away_controller = {"kind": "Human"}
            else:
                away_controller = (
                    {"kind": "Human"}
                    if fx["away_club_id"] in rep and fx["home_club_id"] not in rep
                    else {"kind": "Cpu", "difficulty": "Normal"}
                )

        chosen_seed = seed if seed is not None else _deterministic_seed_for_fixture(fixture_id)
        snapshot = (
            existing_snap["snapshot"]
            if existing_snap is not None
            else self.catalogue.build_fixture_snapshot(
                fixture_id=fixture_id,
                home_club_id=fx["home_club_id"],
                away_club_id=fx["away_club_id"],
                seed=chosen_seed,
                format_name=format_name,
                home_controller=home_controller,
                away_controller=away_controller,
                half_duration_ticks=half_duration_ticks,
                extra_time_enabled=bool(fx.get("extra_time_enabled", 0)),
            )
        )

        ts = utc_now_iso()
        session_id = f"sess-{uuid.uuid4().hex[:12]}"

        if existing_snap is None:
            self.store.conn.execute(
                """
                INSERT INTO fixture_snapshot (fixture_id, snapshot_json, seed, format_name, gameplay_version, taken_at)
                VALUES (?, ?, ?, ?, ?, ?)
                """,
                (
                    fixture_id,
                    json.dumps(snapshot, separators=(",", ":"), sort_keys=True),
                    chosen_seed,
                    snapshot["format"]["name"],
                    GAMEPLAY_VERSION,
                    ts,
                ),
            )
            self.store.append_event(
                world_id=world_id,
                event_type="snapshot_frozen",
                aggregate_type="fixture",
                aggregate_id=fixture_id,
                payload={
                    "fixture_id": fixture_id,
                    "format_name": snapshot["format"]["name"],
                    "players_per_side": snapshot["format"]["players_per_side"],
                    "seed": chosen_seed,
                    "gameplay_version": GAMEPLAY_VERSION,
                },
                caused_by=actor_id,
                occurred_at=ts,
                commit=False,
            )

        self.store.conn.execute(
            """
            INSERT INTO match_session (id, fixture_id, world_id, kind, state, started_at, ended_at, format_name, gameplay_version, final_hash_hex)
            VALUES (?, ?, ?, 'official', 'running', ?, NULL, ?, ?, NULL)
            """,
            (session_id, fixture_id, world_id, ts, snapshot["format"]["name"], GAMEPLAY_VERSION),
        )
        self.store.conn.execute(
            "UPDATE fixture SET state = 'in_session' WHERE id = ?",
            (fixture_id,),
        )
        self.store.append_event(
            world_id=world_id,
            event_type="session_opened",
            aggregate_type="match_session",
            aggregate_id=session_id,
            payload={
                "session_id": session_id,
                "fixture_id": fixture_id,
                "format_name": snapshot["format"]["name"],
                "players_per_side": snapshot["format"]["players_per_side"],
            },
            caused_by=actor_id,
            occurred_at=ts,
            commit=True,
        )

        return {
            "session_id": session_id,
            "fixture_id": fixture_id,
            "world_id": world_id,
            "reused_frozen_snapshot": existing_snap is not None,
            "snapshot": snapshot,
        }

    def run_replay_verifier(
        self,
        snapshot: dict[str, Any],
        input_log: dict[str, Any],
    ) -> dict[str, Any]:
        """Execute `psfc-replay verify` against a frozen snapshot and recorded input log."""
        replay_bin = self._bin()
        with tempfile.TemporaryDirectory(prefix="psfc_verify_") as tmpdir:
            snap_path = Path(tmpdir) / "snapshot.json"
            inp_path = Path(tmpdir) / "inputs.json"
            snap_path.write_text(json.dumps(snapshot), encoding="utf-8")
            inp_path.write_text(json.dumps(input_log), encoding="utf-8")

            proc = subprocess.run(
                [
                    str(replay_bin),
                    "verify",
                    "--snapshot",
                    str(snap_path),
                    "--inputs",
                    str(inp_path),
                ],
                capture_output=True,
                text=True,
                timeout=15,
                check=False,
            )

        if proc.returncode != 0:
            detail = (proc.stderr or proc.stdout).strip()
            raise ReplayValidationError(f"psfc-replay rejected match log: {detail}")

        try:
            report = json.loads(proc.stdout)
        except json.JSONDecodeError as exc:
            raise ReplayValidationError(f"Invalid JSON from psfc-replay: {exc}") from exc

        if not report.get("hash_verified", False):
            raise ReplayValidationError(
                f"State hash mismatch: {report.get('hash_mismatch_reason')}"
            )
        return report

    def complete_session(
        self,
        *,
        session_id: str,
        frames_hex: str | None = None,
        frames: list[Any] | None = None,
        claimed_final_hash_hex: str | None = None,
        claimed_score: list[int] | None = None,
        hash_checkpoints: list[dict[str, Any]] | None = None,
        require_finished: bool = True,
        actor_id: str = "participant",
    ) -> dict[str, Any]:
        """Deterministically replay the match input log via `psfc-replay` and record the authoritative result."""
        sess = self.store.conn.execute(
            "SELECT * FROM match_session WHERE id = ?",
            (session_id,),
        ).fetchone()
        if sess is None:
            raise KeyError(f"Match session '{session_id}' not found")

        fixture_id = sess["fixture_id"]
        world_id = sess["world_id"]

        # IDEMPOTENCY GUARANTEE: duplicate completion returns existing accepted_result!
        existing_accepted = self.store.get_accepted_result(fixture_id)
        if existing_accepted is not None:
            return {
                "idempotent_duplicate": True,
                "accepted_result": existing_accepted,
                "replay_report": None,
            }

        frozen = self.store.get_frozen_snapshot(fixture_id)
        if frozen is None:
            raise ValueError(f"No frozen snapshot found for fixture '{fixture_id}'")

        fx = self.store.get_fixture(fixture_id)
        input_log_payload: dict[str, Any] = {
            "session_id": session_id,
            "claimed_final_hash_hex": claimed_final_hash_hex,
            "claimed_score": claimed_score,
            "hash_checkpoints": hash_checkpoints or [],
        }
        if frames_hex is not None:
            input_log_payload["frames_hex"] = frames_hex
        elif frames is not None:
            input_log_payload["frames"] = frames
        else:
            raise ReplayValidationError("Either 'frames_hex' or 'frames' must be supplied")

        report = self.run_replay_verifier(frozen["snapshot"], input_log_payload)

        if require_finished and not report.get("finished", False):
            raise ReplayValidationError(
                f"Match simulation did not reach FullTime ({report.get('ticks_simulated')} ticks simulated)"
            )

        auth_home = int(report["home_goals"])
        auth_away = int(report["away_goals"])
        went_aet = bool(report.get("went_to_extra_time", False))
        shootout = report.get("shootout_score")
        sh_home = int(shootout[0]) if isinstance(shootout, list) and len(shootout) == 2 else None
        sh_away = int(shootout[1]) if isinstance(shootout, list) and len(shootout) == 2 else None
        final_hash = str(report["final_hash_hex"])
        replay_events = list(report.get("events", []))

        score_discrepancy = (
            claimed_score is not None
            and len(claimed_score) == 2
            and [int(claimed_score[0]), int(claimed_score[1])] != [auth_home, auth_away]
        )

        ts = utc_now_iso()
        self.store.append_event(
            world_id=world_id,
            event_type="session_completed",
            aggregate_type="match_session",
            aggregate_id=session_id,
            payload={
                "session_id": session_id,
                "fixture_id": fixture_id,
                "ticks_simulated": report["ticks_simulated"],
                "final_hash_hex": final_hash,
                "elapsed_replay_ms": report["elapsed_ms"],
                "client_claimed_score": claimed_score,
                "authoritative_score": [auth_home, auth_away],
                "went_to_extra_time": went_aet,
                "shootout_score": [sh_home, sh_away] if sh_home is not None else None,
                "score_discrepancy_detected": score_discrepancy,
            },
            caused_by=actor_id,
            occurred_at=ts,
            commit=False,
        )

        res_ev = self.store.append_event(
            world_id=world_id,
            event_type="result_accepted",
            aggregate_type="fixture",
            aggregate_id=fixture_id,
            payload={
                "fixture_id": fixture_id,
                "competition_id": fx.get("competition_id"),
                "stage": fx.get("stage", "league"),
                "session_id": session_id,
                "home_club_id": fx["home_club_id"],
                "away_club_id": fx["away_club_id"],
                "home_goals": auth_home,
                "away_goals": auth_away,
                "went_to_extra_time": went_aet,
                "shootout_score": [sh_home, sh_away] if sh_home is not None else None,
                "source": fx["result_source"],
                "final_hash_hex": final_hash,
                "format_name": report["format_name"],
                "players_per_side": report["players_per_side"],
            },
            caused_by="psfc-replay",
            occurred_at=ts,
            commit=False,
        )

        self.store.append_event(
            world_id=world_id,
            event_type="match_events_recorded",
            aggregate_type="fixture",
            aggregate_id=fixture_id,
            payload={
                "fixture_id": fixture_id,
                "session_id": session_id,
                "source": fx["result_source"],
                "events": replay_events,
            },
            caused_by="psfc-replay",
            occurred_at=ts,
            commit=False,
        )

        self.store.conn.execute(
            """
            INSERT INTO accepted_result (
                fixture_id, world_id, session_id, source,
                home_goals, away_goals, went_to_extra_time, shootout_home, shootout_away,
                final_hash_hex, accepted_at, event_seq
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
            (
                fixture_id,
                world_id,
                session_id,
                fx["result_source"],
                auth_home,
                auth_away,
                int(went_aet),
                sh_home,
                sh_away,
                final_hash,
                ts,
                res_ev["seq"],
            ),
        )
        self.store.conn.execute(
            "UPDATE match_session SET state = 'completed', ended_at = ?, final_hash_hex = ? WHERE id = ?",
            (ts, final_hash, session_id),
        )
        self.store.conn.execute(
            "UPDATE fixture SET state = 'accepted' WHERE id = ?",
            (fixture_id,),
        )
        self.store.conn.commit()

        return {
            "idempotent_duplicate": False,
            "accepted_result": self.store.get_accepted_result(fixture_id),
            "score_discrepancy_ignored": score_discrepancy,
            "replay_report": {
                "fixture_id": fixture_id,
                "session_id": session_id,
                "format_name": report["format_name"],
                "players_per_side": report["players_per_side"],
                "ticks_simulated": report["ticks_simulated"],
                "final_hash_hex": final_hash,
                "hash_verified": True,
                "home_goals": auth_home,
                "away_goals": auth_away,
                "went_to_extra_time": went_aet,
                "shootout_score": [sh_home, sh_away] if sh_home is not None else None,
                "possession_home_pct": report["possession_home_pct"],
                "elapsed_ms": report["elapsed_ms"],
                "event_count": len(replay_events),
            },
        }
