"""Signed Cartridge Sync, device revocation, and two-generation atomic slot storage (Phase 3)."""

from __future__ import annotations

import hashlib
import hmac
import json
import os
import secrets
import uuid
import zlib
from pathlib import Path
from typing import Any

from psfc_host.ledger.projections import project_standings
from psfc_host.ledger.store import WorldLedgerStore, utc_now_iso


DEFAULT_SIGNING_SECRET = "psfc-host-cartridge-signing-key-v1"


def _canonical_json_bytes(obj: dict[str, Any]) -> bytes:
    return json.dumps(obj, separators=(",", ":"), sort_keys=True).encode("utf-8")


class CartridgeSyncService:
    """Issues and verifies host-signed cartridge manifests and manages revocable device keys."""

    def __init__(self, store: WorldLedgerStore, signing_secret: str | None = None) -> None:
        self.store = store
        self.signing_secret = (
            signing_secret or os.environ.get("PSFC_SIGNING_SECRET", DEFAULT_SIGNING_SECRET)
        ).encode("utf-8")

    def register_device(
        self,
        *,
        participant_id: str,
        world_id: str,
        label: str = "Custom Controller + Cartridge",
        device_id: str | None = None,
    ) -> dict[str, Any]:
        did = device_id or f"dev-{uuid.uuid4().hex[:10]}"
        raw_secret = f"psfc_dev_{secrets.token_hex(16)}"
        secret_hash = hashlib.sha256(raw_secret.encode("utf-8")).hexdigest()
        ts = utc_now_iso()
        self.store.conn.execute(
            """
            INSERT OR REPLACE INTO device (id, participant_id, label, secret_hash, revoked, created_at, revoked_at)
            VALUES (?, ?, ?, ?, 0, ?, NULL)
            """,
            (did, participant_id, label, secret_hash, ts),
        )
        self.store.append_event(
            world_id=world_id,
            event_type="device_registered",
            aggregate_type="device",
            aggregate_id=did,
            payload={"device_id": did, "participant_id": participant_id, "label": label},
            caused_by=participant_id,
            occurred_at=ts,
        )
        return {
            "device_id": did,
            "participant_id": participant_id,
            "label": label,
            "device_secret": raw_secret,
            "created_at": ts,
        }

    def revoke_device(
        self,
        *,
        device_id: str,
        world_id: str,
        actor_id: str = "commissioner",
    ) -> dict[str, Any]:
        row = self.store.conn.execute("SELECT * FROM device WHERE id = ?", (device_id,)).fetchone()
        if row is None:
            raise KeyError(f"Device '{device_id}' not found")
        ts = utc_now_iso()
        self.store.conn.execute(
            "UPDATE device SET revoked = 1, revoked_at = ? WHERE id = ?",
            (ts, device_id),
        )
        self.store.append_event(
            world_id=world_id,
            event_type="device_revoked",
            aggregate_type="device",
            aggregate_id=device_id,
            payload={"device_id": device_id, "participant_id": row["participant_id"]},
            caused_by=actor_id,
            occurred_at=ts,
        )
        return {"device_id": device_id, "revoked": True, "revoked_at": ts}

    def list_devices(self, participant_id: str | None = None) -> list[dict[str, Any]]:
        if participant_id:
            rows = self.store.conn.execute(
                "SELECT id, participant_id, label, revoked, created_at, revoked_at FROM device WHERE participant_id = ? ORDER BY created_at ASC",
                (participant_id,),
            ).fetchall()
        else:
            rows = self.store.conn.execute(
                "SELECT id, participant_id, label, revoked, created_at, revoked_at FROM device ORDER BY created_at ASC"
            ).fetchall()
        return [dict(r) for r in rows]

    def issue_signed_cartridge(
        self,
        *,
        world_id: str,
        participant_id: str,
        device_id: str,
        clubs_summary: dict[str, dict[str, Any]],
        controller_profile: dict[str, Any] | None = None,
        camera_preset: str = "broadcast_5v5",
    ) -> dict[str, Any]:
        dev = self.store.conn.execute("SELECT * FROM device WHERE id = ?", (device_id,)).fetchone()
        if dev is None:
            raise KeyError(f"Device '{device_id}' not registered")
        if int(dev["revoked"]) != 0:
            raise PermissionError(f"Device '{device_id}' has been revoked")
        if dev["participant_id"] != participant_id:
            raise PermissionError(f"Device '{device_id}' does not belong to '{participant_id}'")

        world = self.store.get_world(world_id)
        events = self.store.list_events(world_id)
        last_seq = int(events[-1]["seq"]) if events else 0
        assignments = self.store.list_assignments(world_id)
        my_clubs = [a["club_id"] for a in assignments if a["participant_id"] == participant_id]
        rep = {a["club_id"] for a in assignments}
        standings = project_standings(events, clubs_summary, represented_club_ids=rep)["standings"]
        my_rows = [r for r in standings if r["club_id"] in my_clubs]

        prev_rev = self.store.conn.execute(
            "SELECT COALESCE(MAX(rev_number), 0) AS max_rev FROM cartridge_revision WHERE device_id = ? AND world_id = ?",
            (device_id, world_id),
        ).fetchone()
        rev_number = int(prev_rev["max_rev"]) + 1
        ts = utc_now_iso()

        manifest_body: dict[str, Any] = {
            "schema_version": "1.0.0",
            "cartridge_magic": "PSFC-CART-V1",
            "revision": rev_number,
            "issued_at": ts,
            "device_id": device_id,
            "participant_id": participant_id,
            "world": {
                "world_id": world_id,
                "name": world["name"],
                "season_label": world["season_label"],
                "default_match_format": world["default_match_format"],
                "last_event_seq": last_seq,
            },
            "preferences": {
                "camera_preset": camera_preset,
                "controller_mapping": controller_profile
                or {
                    "pass": "BTN_SOUTH",
                    "through": "BTN_NORTH",
                    "lob": "BTN_WEST",
                    "shoot": "BTN_EAST",
                    "sprint": "RT",
                    "jockey": "LT",
                    "switch_tackle": "LB",
                    "call_run_slide": "RB",
                    "skill_move": "R3_OR_RSTICK_FLICK",
                },
            },
            "career_summary": {
                "assigned_clubs": my_clubs,
                "club_standings_snapshot": my_rows,
                "honours": self.store.list_honours(world_id),
            },
        }

        canonical = _canonical_json_bytes(manifest_body)
        manifest_hash = hashlib.sha256(canonical).hexdigest()
        signature = hmac.new(self.signing_secret, canonical, hashlib.sha256).hexdigest()

        envelope = {
            "manifest": manifest_body,
            "manifest_hash": manifest_hash,
            "signature_alg": "HMAC-SHA256",
            "signature": signature,
        }

        rev_id = f"crev-{uuid.uuid4().hex[:10]}"
        self.store.conn.execute(
            """
            INSERT INTO cartridge_revision (
                id, device_id, participant_id, world_id, rev_number,
                last_event_seq, manifest_hash, signature, manifest_json, issued_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
            (
                rev_id,
                device_id,
                participant_id,
                world_id,
                rev_number,
                last_seq,
                manifest_hash,
                signature,
                json.dumps(envelope, separators=(",", ":"), sort_keys=True),
                ts,
            ),
        )
        self.store.append_event(
            world_id=world_id,
            event_type="cartridge_synced",
            aggregate_type="cartridge",
            aggregate_id=device_id,
            payload={
                "revision_id": rev_id,
                "device_id": device_id,
                "participant_id": participant_id,
                "revision": rev_number,
                "last_event_seq": last_seq,
                "manifest_hash": manifest_hash,
            },
            caused_by=participant_id,
            occurred_at=ts,
        )
        return envelope

    def verify_cartridge_envelope(self, envelope: dict[str, Any]) -> dict[str, Any]:
        """Verify cryptographic signature AND check that the issuing device is not revoked.

        Note: Cartridge contents are treated strictly as portable cache/preferences — never
        authoritative over the host ledger.
        """
        manifest = envelope.get("manifest")
        claimed_sig = str(envelope.get("signature") or "")
        claimed_hash = str(envelope.get("manifest_hash") or "")
        if not isinstance(manifest, dict) or not claimed_sig:
            return {"valid": False, "reason": "malformed_envelope"}

        canonical = _canonical_json_bytes(manifest)
        actual_hash = hashlib.sha256(canonical).hexdigest()
        expected_sig = hmac.new(self.signing_secret, canonical, hashlib.sha256).hexdigest()

        if not hmac.compare_digest(actual_hash, claimed_hash):
            return {"valid": False, "reason": "manifest_hash_mismatch"}
        if not hmac.compare_digest(expected_sig, claimed_sig):
            return {"valid": False, "reason": "invalid_signature"}

        device_id = str(manifest.get("device_id") or "")
        dev = self.store.conn.execute("SELECT * FROM device WHERE id = ?", (device_id,)).fetchone()
        if dev is None:
            return {"valid": False, "reason": "unknown_device"}
        if int(dev["revoked"]) != 0:
            return {"valid": False, "reason": "device_revoked"}

        world_id = str(manifest.get("world", {}).get("world_id") or "")
        latest_events = self.store.list_events(world_id) if world_id else []
        host_seq = int(latest_events[-1]["seq"]) if latest_events else 0
        cart_seq = int(manifest.get("world", {}).get("last_event_seq") or 0)

        return {
            "valid": True,
            "device_id": device_id,
            "participant_id": manifest.get("participant_id"),
            "revision": manifest.get("revision"),
            "stale_cache": cart_seq < host_seq,
            "cartridge_event_seq": cart_seq,
            "host_event_seq": host_seq,
        }


class AtomicCartridgeSlotStorage:
    """Two-generation (`slot_a.bin` / `slot_b.bin`) wear-safe, power-loss-resilient cartridge storage.

    Each slot frames `[generation: u32][crc32: u32][json_bytes]` so an unplugged USB-C cable or
    mid-write power cut on the newer slot cleanly falls back to the highest-generation valid slot!
    """

    def __init__(self, mount_dir: str | Path) -> None:
        self.mount_dir = Path(mount_dir)
        self.mount_dir.mkdir(parents=True, exist_ok=True)
        self.slot_a = self.mount_dir / "slot_a.psfc"
        self.slot_b = self.mount_dir / "slot_b.psfc"

    @staticmethod
    def _pack_frame(generation: int, envelope: dict[str, Any]) -> bytes:
        body = _canonical_json_bytes(envelope)
        crc = zlib.crc32(body) & 0xFFFFFFFF
        header = generation.to_bytes(4, "little") + crc.to_bytes(4, "little")
        return header + body

    @staticmethod
    def _unpack_frame(raw: bytes) -> tuple[int, dict[str, Any]] | None:
        if len(raw) < 9:
            return None
        gen = int.from_bytes(raw[:4], "little")
        claimed_crc = int.from_bytes(raw[4:8], "little")
        body = raw[8:]
        if (zlib.crc32(body) & 0xFFFFFFFF) != claimed_crc:
            return None
        try:
            parsed = json.loads(body.decode("utf-8"))
        except (UnicodeDecodeError, json.JSONDecodeError):
            return None
        if not isinstance(parsed, dict):
            return None
        return (gen, parsed)

    def read_latest_valid(self) -> tuple[int, dict[str, Any]] | None:
        candidates: list[tuple[int, dict[str, Any]]] = []
        for slot in (self.slot_a, self.slot_b):
            if slot.is_file():
                res = self._unpack_frame(slot.read_bytes())
                if res is not None:
                    candidates.append(res)
        if not candidates:
            return None
        candidates.sort(key=lambda item: item[0], reverse=True)
        return candidates[0]

    def write_envelope(
        self,
        envelope: dict[str, Any],
        *,
        simulate_power_cut_bytes: int | None = None,
    ) -> int:
        """Write to the older/inactive slot, leaving the previous valid generation intact."""
        a_state = self._unpack_frame(self.slot_a.read_bytes()) if self.slot_a.is_file() else None
        b_state = self._unpack_frame(self.slot_b.read_bytes()) if self.slot_b.is_file() else None

        gen_a = a_state[0] if a_state is not None else 0
        gen_b = b_state[0] if b_state is not None else 0
        next_gen = max(gen_a, gen_b) + 1
        target_slot = self.slot_a if gen_a <= gen_b else self.slot_b

        frame = self._pack_frame(next_gen, envelope)
        if simulate_power_cut_bytes is not None:
            # Truncated/corrupted mid-write power loss simulation!
            target_slot.write_bytes(frame[: max(1, simulate_power_cut_bytes)])
            raise IOError("Simulated USB/flash power loss mid-write")

        tmp_path = target_slot.with_suffix(".psfc.tmp")
        tmp_path.write_bytes(frame)
        os.replace(tmp_path, target_slot)
        return next_gen
