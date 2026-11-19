class_name CartridgeManager
extends Node
## Manages removable USB profile cartridges (`PSFC-CART-V1`, `cartridge/SPEC.md`).
## Never executes code from a cartridge; validates signed envelopes via World Host API
## and uses two-generation A/B CRC-safe slot writes (`slot_a.psfc` / `slot_b.psfc`).

enum CartridgeState {
	NONE,
	DETECTED,
	VALIDATING,
	LOADING,
	SYNCHRONISING,
	READY,
	SAVING,
	SAFE_TO_REMOVE,
	REJECTED,
}

signal state_changed(new_state: CartridgeState, detail: String)

var current_state: CartridgeState = CartridgeState.NONE
var active_envelope: Dictionary = {}
var active_generation: int = 0
var mount_path: String = ""

const REQUIRED_ENVELOPE_KEYS := ["manifest", "manifest_hash", "signature_alg", "signature"]

func inspect_cartridge_directory(path: String) -> bool:
	mount_path = path
	_set_state(CartridgeState.DETECTED, "Detected volume at %s" % path)

	var best_gen := 0
	var best_env: Dictionary = {}

	for slot_name in ["slot_a.psfc", "slot_b.psfc"]:
		var slot_path := path.path_join(slot_name)
		if FileAccess.file_exists(slot_path):
			var raw_bytes := FileAccess.get_file_as_bytes(slot_path)
			var unpacked := _unpack_slot_frame(raw_bytes)
			if unpacked.size() == 2 and int(unpacked[0]) > best_gen:
				best_gen = int(unpacked[0])
				best_env = unpacked[1]

	# Fallback to plain envelope.json if present during developer testing
	if best_env.is_empty():
		var env_file := path.path_join("envelope.json")
		if FileAccess.file_exists(env_file):
			var parsed = JSON.parse_string(FileAccess.get_file_as_string(env_file))
			if typeof(parsed) == TYPE_DICTIONARY:
				best_gen = 1
				best_env = parsed

	if best_env.is_empty():
		_set_state(CartridgeState.REJECTED, "No valid PSFC-CART-V1 slot found")
		return false

	_set_state(CartridgeState.VALIDATING, "Validating signed envelope schema")
	for key in REQUIRED_ENVELOPE_KEYS:
		if not best_env.has(key):
			_set_state(CartridgeState.REJECTED, "Missing envelope field: %s" % key)
			return false

	var manifest: Dictionary = best_env.get("manifest", {})
	if str(manifest.get("cartridge_magic", "")) != "PSFC-CART-V1":
		_set_state(CartridgeState.REJECTED, "Unsupported cartridge_magic")
		return false

	_set_state(CartridgeState.LOADING, "Loaded generation %d" % best_gen)
	active_generation = best_gen
	active_envelope = best_env
	_set_state(CartridgeState.READY, "Profile ready (%s)" % str(manifest.get("participant_id", "unknown")))
	return true

func verify_with_host_async(host_api_base: String, http: HTTPRequest) -> void:
	if active_envelope.is_empty():
		return
	_set_state(CartridgeState.SYNCHRONISING, "Verifying HMAC-SHA256 signature with World Host")
	var headers := PackedStringArray(["Content-Type: application/json"])
	var body := JSON.stringify({"envelope": active_envelope})
	http.request(host_api_base + "/api/cartridge/verify", headers, HTTPClient.METHOD_POST, body)

func save_signed_envelope_safely(new_envelope: Dictionary) -> bool:
	if mount_path == "":
		return false
	_set_state(CartridgeState.SAVING, "Writing inactive generation slot")

	var gen_a := _probe_slot_generation(mount_path.path_join("slot_a.psfc"))
	var gen_b := _probe_slot_generation(mount_path.path_join("slot_b.psfc"))
	var next_gen := maxi(gen_a, gen_b) + 1
	var target_name := "slot_a.psfc" if gen_a <= gen_b else "slot_b.psfc"
	var target_path := mount_path.path_join(target_name)
	var tmp_path := target_path + ".tmp"

	var frame_bytes := _pack_slot_frame(next_gen, new_envelope)
	var f := FileAccess.open(tmp_path, FileAccess.WRITE)
	if f == null:
		_set_state(CartridgeState.REJECTED, "Failed to open temporary slot file")
		return false
	f.store_buffer(frame_bytes)
	f.flush()
	f.close()

	DirAccess.rename_absolute(tmp_path, target_path)
	active_generation = next_gen
	active_envelope = new_envelope
	_set_state(CartridgeState.SAFE_TO_REMOVE, "Saved gen %d to %s — safe to remove" % [next_gen, target_name])
	return true

func _probe_slot_generation(slot_path: String) -> int:
	if not FileAccess.file_exists(slot_path):
		return 0
	var unpacked := _unpack_slot_frame(FileAccess.get_file_as_bytes(slot_path))
	return int(unpacked[0]) if unpacked.size() == 2 else 0

func _pack_slot_frame(generation: int, envelope: Dictionary) -> PackedByteArray:
	var body := JSON.stringify(envelope).to_utf8_buffer()
	var crc := _crc32(body)
	var header := PackedByteArray()
	header.resize(8)
	header.encode_u32(0, generation)
	header.encode_u32(4, crc)
	header.append_array(body)
	return header

func _unpack_slot_frame(raw: PackedByteArray) -> Array:
	if raw.size() < 9:
		return []
	var gen := raw.decode_u32(0)
	var claimed_crc := raw.decode_u32(4)
	var body := raw.slice(8)
	if _crc32(body) != claimed_crc:
		return []
	var parsed = JSON.parse_string(body.get_string_from_utf8())
	if typeof(parsed) != TYPE_DICTIONARY:
		return []
	return [gen, parsed]

func _crc32(data: PackedByteArray) -> int:
	var crc: int = 0xFFFFFFFF
	for b in data:
		crc ^= int(b)
		for _bit in range(8):
			if (crc & 1) != 0:
				crc = (crc >> 1) ^ 0xEDB88320
			else:
				crc >>= 1
	return (~crc) & 0xFFFFFFFF

func _set_state(next: CartridgeState, detail: String) -> void:
	current_state = next
	state_changed.emit(next, detail)
