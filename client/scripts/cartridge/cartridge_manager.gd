class_name CartridgeManager
extends Node
## Manages removable USB profile cartridges (README §12, Architecture §4.1 & §6.6).
## Never executes code from a cartridge; validates manifest schema and uses A/B safe writes.

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
var active_manifest: Dictionary = {}
var mount_path: String = ""

const REQUIRED_KEYS := ["format_version", "world_id", "participant_id", "device_id", "active_slot", "revision"]

func inspect_cartridge_directory(path: String) -> bool:
	mount_path = path
	_set_state(CartridgeState.DETECTED, "Detected volume at %s" % path)

	var manifest_file := path.path_join("manifest.json")
	if not FileAccess.file_exists(manifest_file):
		_set_state(CartridgeState.REJECTED, "Missing manifest.json")
		return false

	_set_state(CartridgeState.VALIDATING, "Validating manifest schema")
	var raw := FileAccess.get_file_as_string(manifest_file)
	var parsed = JSON.parse_string(raw)
	if typeof(parsed) != TYPE_DICTIONARY:
		_set_state(CartridgeState.REJECTED, "Corrupt manifest JSON")
		return false

	for key in REQUIRED_KEYS:
		if not parsed.has(key):
			_set_state(CartridgeState.REJECTED, "Missing required field: %s" % key)
			return false

	var slot: String = str(parsed["active_slot"])
	if slot != "A" and slot != "B":
		_set_state(CartridgeState.REJECTED, "Invalid active_slot '%s'" % slot)
		return false

	_set_state(CartridgeState.LOADING, "Loading slot %s career cache" % slot)
	active_manifest = parsed
	_set_state(CartridgeState.READY, "Profile ready (%s)" % str(parsed["participant_id"]))
	return true

func save_revision_safely(profile_payload: Dictionary, new_revision: Dictionary) -> bool:
	if current_state != CartridgeState.READY and current_state != CartridgeState.SAFE_TO_REMOVE:
		return false
	_set_state(CartridgeState.SAVING, "Writing inactive generation")

	var current_slot := str(active_manifest.get("active_slot", "A"))
	var next_slot := "B" if current_slot == "A" else "A"
	var slot_dir := mount_path.path_join("slot_" + next_slot)
	DirAccess.make_dir_recursive_absolute(slot_dir)

	var profile_path := slot_dir.path_join("profile.json")
	var f := FileAccess.open(profile_path, FileAccess.WRITE)
	if f == null:
		_set_state(CartridgeState.REJECTED, "Failed to write inactive slot")
		return false
	f.store_string(JSON.stringify(profile_payload, "\t"))
	f.flush()
	f.close()

	# Flip manifest.active_slot only after payload write completes
	active_manifest["active_slot"] = next_slot
	active_manifest["revision"] = new_revision
	var mf := FileAccess.open(mount_path.path_join("manifest.json"), FileAccess.WRITE)
	if mf == null:
		return false
	mf.store_string(JSON.stringify(active_manifest, "\t"))
	mf.flush()
	mf.close()

	_set_state(CartridgeState.SAFE_TO_REMOVE, "Saved to slot %s — safe to remove" % next_slot)
	return true

func _set_state(next: CartridgeState, detail: String) -> void:
	current_state = next
	state_changed.emit(next, detail)
