# Parallel Season FC — Removable Profile Cartridge Specification (`PSFC-CART-V1`)

## 1. Purpose & Authority Model

A Parallel Season FC Cartridge is a physical, pocketable USB-C flash token (see [`hardware/enclosures/cartridge_shell.scad`](../hardware/enclosures/cartridge_shell.scad)) that carries a participant's:

1. **Identity & Revocable Device Binding** (`participant_id`, `device_id`).
2. **Controller & Camera Preferences** (Hall-effect stick dead-zone, button layout, `broadcast_5v5` / `end_to_end` camera preset).
3. **Offline Career Cache** (`assigned_clubs`, `club_standings_snapshot`, `honours`, `last_event_seq`).

### Non-Negotiable Security Invariants
- **Data Only — Never Executable Code**: Clients (`psfc-godot` and Web/WASM) parse strictly UTF-8 JSON validated against [`cartridge_manifest.schema.json`](../shared/schemas/cartridge_manifest.schema.json). Scripts, shared libraries, and Godot resource `.tres`/`.pck` files on a cartridge volume are ignored.
- **Host Ledger is Always Authoritative**: A cartridge cannot overwrite official Parallel Season standings, match scores, or club ownership. When `cartridge.world.last_event_seq < host_event_seq`, the cartridge is flagged `stale_cache = true` and refreshed from the World API.
- **Host Cryptographic Signature (`HMAC-SHA256`)**: Every synced envelope carries `manifest_hash = SHA256(canonical_json(manifest))` and `signature = HMAC_SHA256(PSFC_SIGNING_SECRET, canonical_json(manifest))`. Any tampered byte fails verification at `POST /api/cartridge/verify`.
- **Lost Cartridge Recovery**: Revoking a lost cartridge via `POST /api/worlds/{world_id}/devices/{device_id}/revoke` immediately invalidates both its cartridge signature verification (`reason = "device_revoked"`) and its HS256 online JWTs without losing the participant's club or career history on the World Host.

---

## 2. Two-Generation Power-Loss-Safe Slot Layout

Flash media unplugged mid-write can corrupt an in-place file. Each mounted cartridge volume uses a two-slot ping-pong layout implemented in [`AtomicCartridgeSlotStorage`](../host/psfc_host/community/cartridge.py):

```text
/media/PSFC_CART/
├── slot_a.psfc    # Binary frame: [generation: u32 LE][crc32: u32 LE][canonical UTF-8 JSON envelope]
└── slot_b.psfc    # Binary frame: [generation: u32 LE][crc32: u32 LE][canonical UTF-8 JSON envelope]
```

### Binary Frame Header (8 Bytes)
| Offset | Type | Field | Description |
| --- | --- | --- | --- |
| `0..4` | `u32` (little-endian) | `generation` | Monotonically increasing write counter (`1, 2, 3, ...`) |
| `4..8` | `u32` (little-endian) | `crc32` | IEEE 802.3 CRC32 over `payload_bytes` (`8..EOF`) |
| `8..N` | UTF-8 bytes | `payload_bytes` | Canonical JSON serialization of `SignedCartridgeEnvelope` |

### Read & Write Protocol
1. **Mount / Read**:
   - Inspect both `slot_a.psfc` and `slot_b.psfc`.
   - Discard any slot shorter than 9 bytes, failing `CRC32`, or failing JSON parse.
   - Select the valid slot with the **highest `generation`**.
2. **Sync / Write**:
   - Identify the valid slot with the **older** (or invalid/missing) generation.
   - Compute `next_gen = max(gen_a, gen_b) + 1`.
   - Write `[next_gen][crc32][envelope_bytes]` to `<inactive_slot>.tmp`, flush, and atomically rename over `<inactive_slot>` while the previous generation slot remains untouched.
   - Wait until rename + sync completes before signalling **`SAFE_TO_REMOVE`**.
