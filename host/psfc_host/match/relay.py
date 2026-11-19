"""Authoritative UDP datagram relay and `match_input_log` recorder (Phase 4)."""

from __future__ import annotations

import asyncio
import json
from typing import Any

from psfc_host.ledger.store import WorldLedgerStore


IDLE_INPUT_HEX = "0000000000000000"


def _frame_dict_to_hex(frame: dict[str, Any]) -> str:
    mx = int(frame.get("move_x", 0)) & 0xFF
    my = int(frame.get("move_y", 0)) & 0xFF
    ax = int(frame.get("aim_x", 0)) & 0xFF
    ay = int(frame.get("aim_y", 0)) & 0xFF
    btns = int(frame.get("buttons", 0)) & 0xFFFF
    res = int(frame.get("reserved", 0)) & 0xFFFF
    raw = bytes([mx, my, ax, ay]) + btns.to_bytes(2, "little") + res.to_bytes(2, "little")
    return raw.hex()


class MatchInputRelayRecorder:
    """Stores mirrored per-tick input batches in `match_input_log` and reconstructs `frames_hex`."""

    def __init__(self, store: WorldLedgerStore) -> None:
        self.store = store

    def record_input_batch(
        self,
        *,
        session_id: str,
        side: str,
        first_tick: int,
        frames: list[dict[str, Any] | str],
    ) -> int:
        norm_side = side.lower()
        if norm_side not in ("home", "away"):
            raise ValueError(f"Invalid side '{side}'")
        written = 0
        for offset, item in enumerate(frames):
            tick = int(first_tick) + offset
            if tick <= 0:
                continue
            hx = item if isinstance(item, str) else _frame_dict_to_hex(item)
            if len(hx) != 16:
                raise ValueError(f"Expected 16-char 8-byte input_hex, got '{hx}'")
            self.store.conn.execute(
                """
                INSERT OR IGNORE INTO match_input_log (session_id, tick, side, input_hex)
                VALUES (?, ?, ?, ?)
                """,
                (session_id, tick, norm_side, hx.lower()),
            )
            written += 1
        self.store.conn.commit()
        return written

    def assemble_session_frames_hex(self, session_id: str) -> tuple[int, str]:
        rows = self.store.conn.execute(
            """
            SELECT tick, side, input_hex
            FROM match_input_log
            WHERE session_id = ?
            ORDER BY tick ASC
            """,
            (session_id,),
        ).fetchall()
        if not rows:
            return (0, "")
        max_tick = max(int(r["tick"]) for r in rows)
        by_tick: dict[tuple[int, str], str] = {
            (int(r["tick"]), str(r["side"])): str(r["input_hex"]) for r in rows
        }
        chunks: list[str] = []
        for t in range(1, max_tick + 1):
            h_hex = by_tick.get((t, "home"), IDLE_INPUT_HEX)
            a_hex = by_tick.get((t, "away"), IDLE_INPUT_HEX)
            chunks.append(h_hex + a_hex)
        return (max_tick, "".join(chunks))


class UdpMatchRelayProtocol(asyncio.DatagramProtocol):
    """Asyncio UDP relay forwarding `NetMessage` datagrams between two peers while logging inputs."""

    def __init__(self, session_id: str, recorder: MatchInputRelayRecorder) -> None:
        self.session_id = session_id
        self.recorder = recorder
        self.transport: asyncio.DatagramTransport | None = None
        self.peers_by_side: dict[str, tuple[str, int]] = {}

    def connection_made(self, transport: asyncio.BaseTransport) -> None:
        self.transport = transport  # type: ignore[assignment]

    def datagram_received(self, data: bytes, addr: tuple[str, int]) -> None:
        try:
            msg = json.loads(data.decode("utf-8"))
        except Exception:
            return
        if not isinstance(msg, dict):
            return

        if "InputBatch" in msg and isinstance(msg["InputBatch"], dict):
            ib = msg["InputBatch"]
            side = str(ib.get("side", "Home")).lower()
            self.peers_by_side[side] = addr
            first_tick = int(ib.get("first_tick", 1))
            frames = list(ib.get("frames", []))
            self.recorder.record_input_batch(
                session_id=self.session_id,
                side=side,
                first_tick=first_tick,
                frames=frames,
            )
            opp = "away" if side == "home" else "home"
            if opp in self.peers_by_side and self.transport is not None:
                self.transport.sendto(data, self.peers_by_side[opp])
        elif "StateHash" in msg and isinstance(msg["StateHash"], dict):
            sh = msg["StateHash"]
            side = str(sh.get("side", "Home")).lower()
            self.peers_by_side[side] = addr
            opp = "away" if side == "home" else "home"
            if opp in self.peers_by_side and self.transport is not None:
                self.transport.sendto(data, self.peers_by_side[opp])
