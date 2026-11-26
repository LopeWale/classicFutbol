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

---

## 3. Controller Profile Payload (`preferences.controller_mapping`)

The cartridge carries the player's button layout as **data**, validated against
[`control_profile.schema.json`](../shared/schemas/control_profile.schema.json) (ControlProfile **version 2**):

```jsonc
"controller_mapping": {
  "layout": "auto",              // Layer-1 hardware pad profile (auto | snes_usb | snes_std | switch | dinput | custom)
  "custom_map": null,            // optional calibrated raw-button map for unknown pads
  "control_profile": {           // Layer-2 ControlProfile v2 — canonical snake_case
    "version": 2,
    "preset": "classic",         // classic | alternate | two_button | custom
    "auto_switch": "auto", "auto_switch_move_assist": "low",
    "pass_assist": "assisted", "through_assist": "assisted", "shot_assist": "assisted", "cross_assist": "assisted",
    "defending_mode": "tactical", "analog_sprint": true,
    "attack":  { "pass": "south", "shoot": "east", "cross": "west", "through_ball": "north",
                 "sprint": "rt", "finesse": "rb", "pace_control": "lt", "chip_run": "lb" },
    "defense": { "contain": "south", "standing_tackle": "east", "sliding_tackle": "west", "rush_gk": "north",
                 "sprint": "rt", "teammate_contain": "rb", "jockey": "lt", "switch_player": "lb" }
  },
  "fifa_profile": { "...": "same object; alias read by older clients" }
}
```

Rules:

- **Physical, never gameplay**: bindings name *positional physical inputs* (`south`, `east`, `west`, `north`, `lb`, `rb`, `lt`, `rt`, `select`), never `BTN_*` bits. The client's Layer 2 turns them into the `InputFrame` bitmask, so the same cartridge works on an Xbox, PlayStation, Switch, SNES-style or PSFC RP2040 pad, and the HUD prompts show that pad's glyphs.
- **Two maps**: `attack` (in possession) and `defense` (out of possession) are both evaluated every tick; psfc-sim's `PossessionContext` picks the half that applies (deterministic and replayable).
- **Shared inputs / unbound**: several actions may share one input (the `two_button` preset puts pass, through ball and cross on `south`; the client resolves one per press from match context). `null` = unbound.
- **Key case**: snake_case is canonical on the cartridge and in Godot (`user://psfc_control_profile.json`). The web client keeps camelCase in memory and converts at load/save; it also accepts legacy camelCase files.
- **Settings sync**: on load, import and match start the eight gameplay settings are sent to psfc-sim as a `0x4000`-tagged `InputFrame.reserved` word (`ControlSettings::encode_reserved`).
- **Portability**: the web Controllers screen has **EXPORT JSON / IMPORT JSON** buttons; an exported file (or a whole `controller_mapping` object) can be imported directly. Hosts that don't send a profile may store a legacy mapping object; clients ignore keys they don't know and fall back to Classic.
