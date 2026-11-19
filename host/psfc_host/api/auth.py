"""Device-bound HS256 JWT authentication and revocation checks (Phase 4)."""

from __future__ import annotations

import base64
import hashlib
import hmac
import json
import os
import time
from typing import Any

from psfc_host.community.cartridge import DEFAULT_SIGNING_SECRET
from psfc_host.ledger.store import WorldLedgerStore


def _b64url_encode(data: bytes) -> str:
    return base64.urlsafe_b64encode(data).rstrip(b"=").decode("ascii")


def _b64url_decode(s: str) -> bytes:
    pad = "=" * ((4 - len(s) % 4) % 4)
    return base64.urlsafe_b64decode(s + pad)


class DeviceAuthService:
    """Issues and validates device-bound HS256 JWT tokens against `device` table."""

    def __init__(self, store: WorldLedgerStore, secret: str | None = None) -> None:
        self.store = store
        self.secret = (
            secret or os.environ.get("PSFC_JWT_SECRET", DEFAULT_SIGNING_SECRET)
        ).encode("utf-8")

    def issue_token(
        self,
        *,
        device_id: str,
        device_secret: str,
        ttl_seconds: int = 86_400,
    ) -> dict[str, Any]:
        dev = self.store.conn.execute("SELECT * FROM device WHERE id = ?", (device_id,)).fetchone()
        if dev is None:
            raise PermissionError(f"Unknown device '{device_id}'")
        if int(dev["revoked"]) != 0:
            raise PermissionError(f"Device '{device_id}' has been revoked")

        supplied_hash = hashlib.sha256(device_secret.encode("utf-8")).hexdigest()
        if not hmac.compare_digest(supplied_hash, str(dev["secret_hash"])):
            raise PermissionError("Invalid device secret")

        now = int(time.time())
        header = {"alg": "HS256", "typ": "JWT"}
        payload = {
            "sub": dev["participant_id"],
            "dev": device_id,
            "iat": now,
            "exp": now + ttl_seconds,
        }
        signing_input = (
            f"{_b64url_encode(json.dumps(header, separators=(',', ':')).encode('utf-8'))}."
            f"{_b64url_encode(json.dumps(payload, separators=(',', ':')).encode('utf-8'))}"
        )
        sig = hmac.new(self.secret, signing_input.encode("ascii"), hashlib.sha256).digest()
        token = f"{signing_input}.{_b64url_encode(sig)}"
        return {
            "access_token": token,
            "token_type": "Bearer",
            "participant_id": dev["participant_id"],
            "device_id": device_id,
            "expires_at_epoch": now + ttl_seconds,
        }

    def verify_token(self, token: str) -> dict[str, Any]:
        parts = token.strip().split(".")
        if len(parts) != 3:
            raise PermissionError("Malformed JWT")
        signing_input = f"{parts[0]}.{parts[1]}"
        expected_sig = hmac.new(
            self.secret, signing_input.encode("ascii"), hashlib.sha256
        ).digest()
        try:
            actual_sig = _b64url_decode(parts[2])
            payload = json.loads(_b64url_decode(parts[1]).decode("utf-8"))
        except Exception as exc:
            raise PermissionError("Invalid JWT encoding") from exc

        if not hmac.compare_digest(expected_sig, actual_sig):
            raise PermissionError("Invalid JWT signature")
        if int(payload.get("exp", 0)) < int(time.time()):
            raise PermissionError("JWT expired")

        device_id = str(payload.get("dev") or "")
        dev = self.store.conn.execute("SELECT * FROM device WHERE id = ?", (device_id,)).fetchone()
        if dev is None or int(dev["revoked"]) != 0:
            raise PermissionError(f"Device '{device_id}' is revoked or unknown")
        return payload
