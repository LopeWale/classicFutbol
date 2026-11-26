class_name ControllerInterface
extends Node
## Action-based FIFA 10–12 input abstraction layer for Godot 4 (`PsfcMatchSim`).
## Separates Layer 1 (physical gamepad / keyboard with 360° radial deadzone & analog triggers)
## from Layer 2 (JSON-serializable ControlProfile with Classic/Alternate/Two-Button/Custom presets,
## distinct Attacking/Defending binding maps, and deterministic `InputFrame.reserved` settings sync).
##
## Serialization: `control_profile` is stored in the CANONICAL snake_case form defined by
## `shared/schemas/control_profile.schema.json` (version 2) at `user://psfc_control_profile.json`.
## The web client (`host/psfc_host/web/static/control_profile.js`) writes the identical form;
## `sanitize_control_profile()` also accepts the web's legacy camelCase keys, so a profile exported
## from either client (or read from a cartridge `preferences.controller_mapping`) loads here.
## A binding may be `null` (unbound) and several actions may share one physical input.

const BTN_PASS: int = 1 << 0
const BTN_THROUGH: int = 1 << 1
const BTN_LOB: int = 1 << 2
const BTN_SHOOT: int = 1 << 3
const BTN_SPRINT: int = 1 << 4
const BTN_SWITCH: int = 1 << 5
const BTN_JOCKEY: int = 1 << 6
const BTN_TACKLE: int = 1 << 7
const BTN_SLIDE: int = 1 << 8
const BTN_CALL_RUN: int = 1 << 9
const BTN_SKILL: int = 1 << 10
const BTN_FINESSE: int = 1 << 11
const BTN_CONTAIN: int = 1 << 12
const BTN_TEAMMATE_CONTAIN: int = 1 << 13
const BTN_GK_RUSH: int = 1 << 14
const BTN_PACE_CONTROL: int = 1 << 15

const PROFILE_PATH: String = "user://psfc_control_profile.json"
const PROFILE_VERSION: int = 2
const PHYSICAL_INPUTS: Array = ["south", "east", "west", "north", "rt", "rb", "lt", "lb", "select"]
const PRESET_NAMES: Array = ["classic", "alternate", "two_button", "custom"]
## SDL game-controller mapping for the PSFC RP2040 pad (firmware v2 reports physical buttons in
## W3C standard order: b0 south .. b15 dpad right; axes X, Y, Z, Rz). Registered at runtime with the
## device's real GUID (see `_register_psfc_mapping`), so no platform-specific GUID is hardcoded.
const PSFC_SDL_MAPPING_BODY: String = "a:b0,b:b1,x:b2,y:b3,leftshoulder:b4,rightshoulder:b5,lefttrigger:b6,righttrigger:b7,back:b8,start:b9,leftstick:b10,rightstick:b11,dpup:b12,dpdown:b13,dpleft:b14,dpright:b15,leftx:a0,lefty:a1,rightx:a2,righty:a3"

const ATTACK_ACTION_BITS: Dictionary = {
	"pass": BTN_PASS,
	"shoot": BTN_SHOOT,
	"cross": BTN_LOB,
	"through_ball": BTN_THROUGH,
	"finesse": BTN_FINESSE,
	"pace_control": BTN_PACE_CONTROL,
	"chip_run": BTN_CALL_RUN,
}
const DEFENSE_ACTION_BITS: Dictionary = {
	"contain": BTN_CONTAIN,
	"standing_tackle": BTN_TACKLE,
	"sliding_tackle": BTN_SLIDE,
	"rush_gk": BTN_GK_RUSH,
	"teammate_contain": BTN_TEAMMATE_CONTAIN,
	"jockey": BTN_JOCKEY,
	"switch_player": BTN_SWITCH,
}
## Pass-family actions: when 2+ share ONE physical input (Two-Button), one bit is chosen per press.
const PASS_FAMILY: Array = ["pass", "through_ball", "cross"]

## Two-Button (FIFA 2010 World Cup / FIFA 11 / FIFA 12 casual scheme) — see the design note in
## host/psfc_host/web/static/control_profile.js. South = every pass (ground / through / cross chosen
## from context), East = shoot/clear; defending South = switch, East = tackle + contain (hold to press).
## Uses only existing InputFrame bits; no psfc-sim change.
const CONTROL_PRESETS: Dictionary = {
	"classic": {
		"attack": {
			"pass": "south",
			"shoot": "east",
			"cross": "west",
			"through_ball": "north",
			"sprint": "rt",
			"finesse": "rb",
			"pace_control": "lt",
			"chip_run": "lb",
		},
		"defense": {
			"contain": "south",
			"standing_tackle": "east",
			"sliding_tackle": "west",
			"rush_gk": "north",
			"sprint": "rt",
			"teammate_contain": "rb",
			"jockey": "lt",
			"switch_player": "lb",
		},
	},
	"alternate": {
		"attack": {
			"pass": "south",
			"shoot": "west",
			"cross": "east",
			"through_ball": "north",
			"sprint": "rt",
			"finesse": "rb",
			"pace_control": "lt",
			"chip_run": "lb",
		},
		"defense": {
			"contain": "south",
			"standing_tackle": "east",
			"sliding_tackle": "west",
			"rush_gk": "north",
			"sprint": "rt",
			"teammate_contain": "rb",
			"jockey": "lt",
			"switch_player": "lb",
		},
	},
	"two_button": {
		"attack": {
			"pass": "south",
			"shoot": "east",
			"cross": "south",
			"through_ball": "south",
			"sprint": "rt",
			"finesse": null,
			"pace_control": null,
			"chip_run": null,
		},
		"defense": {
			"contain": "east",
			"standing_tackle": "east",
			"sliding_tackle": null,
			"rush_gk": null,
			"sprint": "rt",
			"teammate_contain": null,
			"jockey": null,
			"switch_player": "south",
		},
		"settings": {
			"pass_assist": "assisted",
			"through_assist": "assisted",
			"shot_assist": "assisted",
			"cross_assist": "assisted",
			"defending_mode": "legacy",
		},
	},
}

class DeviceCalibration:
	var dead_zone: float = 0.12
	var invert_x: bool = false
	var invert_y: bool = false # Godot camera at +Z looks -Z: stick UP (-1) -> move_y < 0 -> -Z (up-screen)

# Maps side (0 = Home, 1 = Away) -> bound joypad GUID ("" if keyboard or unbound)
var bound_guid_by_side: Dictionary = {0: "", 1: ""}
var active_device_by_side: Dictionary = {0: -1, 1: -1}
var calibration_by_guid: Dictionary = {}
var control_profile: Dictionary = {}
var pending_settings_sync: Array[bool] = [true, true]
## Two-Button pass resolver: per-side latched bit and optional match context supplied by the caller
## via `set_pass_context(side, {"forward":..,"wide":..,"stick_fwd":..,"stick_mag":..})`.
var pass_latch_by_side: Array[int] = [0, 0]
var pass_context_by_side: Array[Dictionary] = [{}, {}]

func _ready() -> void:
	load_control_profile()
	Input.joy_connection_changed.connect(_on_joy_connection_changed)
	_auto_bind_connected_pads()

func default_control_profile(preset_name: String = "classic") -> Dictionary:
	var p_key := preset_name if CONTROL_PRESETS.has(preset_name) else "classic"
	var base: Dictionary = CONTROL_PRESETS[p_key]
	var prof := {
		"version": PROFILE_VERSION,
		"preset": p_key,
		"auto_switch": "auto",             # "auto" | "air_balls" | "manual"
		"auto_switch_move_assist": "low",  # "none" | "low" | "high"
		"pass_assist": "assisted",         # "assisted" | "semi" | "manual"
		"through_assist": "assisted",
		"shot_assist": "assisted",
		"cross_assist": "assisted",
		"defending_mode": "tactical",      # "tactical" | "legacy"
		"analog_sprint": true,
		"attack": base["attack"].duplicate(),
		"defense": base["defense"].duplicate(),
	}
	if base.has("settings"):
		var preset_settings: Dictionary = base["settings"]
		for k in preset_settings.keys():
			prof[k] = preset_settings[k]
	return prof

## camelCase (web legacy) -> snake_case (canonical)
static func _snake(key: String) -> String:
	var out := ""
	for i in key.length():
		var ch := key[i]
		if ch != ch.to_lower():
			out += "_" + ch.to_lower()
		else:
			out += ch
	return out

static func _pick(raw: Dictionary, snake_key: String) -> Variant:
	if raw.has(snake_key):
		return raw[snake_key]
	for k in raw.keys():
		if _snake(str(k)) == snake_key:
			return raw[k]
	return null

## Validates a parsed profile (canonical snake_case or legacy camelCase) against the schema's enums.
## Missing / invalid bindings fall back to the preset; an explicit `null` stays unbound.
func sanitize_control_profile(raw: Variant) -> Dictionary:
	if typeof(raw) != TYPE_DICTIONARY:
		return default_control_profile("classic")
	var src: Dictionary = raw
	# Accept a whole cartridge `preferences.controller_mapping` object too.
	if src.has("control_profile") and typeof(src["control_profile"]) == TYPE_DICTIONARY:
		src = src["control_profile"]
	elif src.has("fifa_profile") and typeof(src["fifa_profile"]) == TYPE_DICTIONARY:
		src = src["fifa_profile"]
	var preset := str(src.get("preset", "classic"))
	if not PRESET_NAMES.has(preset):
		preset = "classic"
	var base: Dictionary = CONTROL_PRESETS.get(preset, CONTROL_PRESETS["classic"])
	var prof := default_control_profile("classic")
	prof["preset"] = preset

	var enum_field := func(key: String, allowed: Array, fallback: String) -> String:
		var v = _pick(src, key)
		return str(v) if typeof(v) == TYPE_STRING and allowed.has(v) else fallback
	prof["auto_switch"] = enum_field.call("auto_switch", ["auto", "air_balls", "manual"], "auto")
	prof["auto_switch_move_assist"] = enum_field.call("auto_switch_move_assist", ["none", "low", "high"], "low")
	for k in ["pass_assist", "through_assist", "shot_assist", "cross_assist"]:
		prof[k] = enum_field.call(k, ["assisted", "semi", "manual"], "assisted")
	prof["defending_mode"] = enum_field.call("defending_mode", ["tactical", "legacy"], "tactical")
	var analog = _pick(src, "analog_sprint")
	prof["analog_sprint"] = bool(analog) if typeof(analog) == TYPE_BOOL else true

	for ctx in ["attack", "defense"]:
		var raw_map: Dictionary = {}
		if src.has(ctx) and typeof(src[ctx]) == TYPE_DICTIONARY:
			raw_map = src[ctx]
		var base_map: Dictionary = base[ctx]
		var out_map: Dictionary = {}
		for action in base_map.keys():
			var has_key: bool = raw_map.has(action)
			for rk in raw_map.keys():
				if _snake(str(rk)) == action:
					has_key = true
			var v = _pick(raw_map, action)
			if has_key and v == null:
				out_map[action] = null
			elif typeof(v) == TYPE_STRING and PHYSICAL_INPUTS.has(v):
				out_map[action] = v
			else:
				out_map[action] = base_map[action]
		prof[ctx] = out_map
	return prof

func load_control_profile() -> void:
	control_profile = default_control_profile("classic")
	if FileAccess.file_exists(PROFILE_PATH):
		var f := FileAccess.open(PROFILE_PATH, FileAccess.READ)
		if f != null:
			var parsed = JSON.parse_string(f.get_as_text())
			if typeof(parsed) == TYPE_DICTIONARY:
				control_profile = sanitize_control_profile(parsed)
	pass_latch_by_side = [0, 0]
	pending_settings_sync = [true, true]

func save_control_profile() -> void:
	var f := FileAccess.open(PROFILE_PATH, FileAccess.WRITE)
	if f != null:
		f.store_string(export_control_profile_json())
	pending_settings_sync = [true, true]

## Canonical schema v2 JSON text (same shape the web EXPORT JSON button writes).
func export_control_profile_json() -> String:
	var out := control_profile.duplicate(true)
	out["version"] = PROFILE_VERSION
	return JSON.stringify(out, "\t")

## Imports profile JSON text (web export, Godot file or cartridge controller_mapping); queues a 0x4000 sync.
func import_control_profile_json(text: String) -> bool:
	var parsed = JSON.parse_string(text)
	if typeof(parsed) != TYPE_DICTIONARY:
		return false
	control_profile = sanitize_control_profile(parsed)
	pass_latch_by_side = [0, 0]
	save_control_profile()
	return true

func set_preset(preset_name: String) -> void:
	if CONTROL_PRESETS.has(preset_name):
		var base: Dictionary = CONTROL_PRESETS[preset_name]
		control_profile["preset"] = preset_name
		control_profile["attack"] = base["attack"].duplicate()
		control_profile["defense"] = base["defense"].duplicate()
		if base.has("settings"):
			var preset_settings: Dictionary = base["settings"]
			for k in preset_settings.keys():
				control_profile[k] = preset_settings[k]
		save_control_profile()

## `physical_input` = one of PHYSICAL_INPUTS, or null to unbind. Same-context collisions auto-swap.
func rebind_action(context: String, action_key: String, physical_input: Variant) -> void:
	if context != "attack" and context != "defense":
		return
	if physical_input != null and not PHYSICAL_INPUTS.has(physical_input):
		return
	var ctx_map: Dictionary = control_profile[context]
	if not ctx_map.has(action_key):
		return
	var prev_phys = ctx_map[action_key]
	if prev_phys == physical_input:
		return
	if physical_input != null:
		for other_key in ctx_map.keys():
			if other_key != action_key and ctx_map[other_key] == physical_input:
				ctx_map[other_key] = prev_phys
				break
	ctx_map[action_key] = physical_input
	control_profile["preset"] = "custom"
	save_control_profile()

func set_pass_context(side: int, ctx: Dictionary) -> void:
	if side >= 0 and side < 2:
		pass_context_by_side[side] = ctx

## Mirrors `resolveContextPass` in control_profile.js.
static func resolve_context_pass(mask: int, ctx: Dictionary) -> int:
	if mask == 0:
		return 0
	var forward: float = float(ctx.get("forward", 0.0))
	var wide: float = float(ctx.get("wide", 0.0))
	var stick_mag: float = float(ctx.get("stick_mag", 0.0))
	var stick_fwd: float = float(ctx.get("stick_fwd", 0.0))
	if (mask & BTN_LOB) != 0 and forward > 0.35 and wide > 0.55:
		return BTN_LOB
	if (mask & BTN_THROUGH) != 0 and forward > -0.15 and stick_mag > 0.6 and stick_fwd / maxf(stick_mag, 0.000001) > 0.8:
		return BTN_THROUGH
	if (mask & BTN_PASS) != 0:
		return BTN_PASS
	if (mask & BTN_THROUGH) != 0:
		return BTN_THROUGH
	return BTN_LOB

func encode_control_settings_reserved() -> int:
	var assist_code := func(v: String) -> int:
		return 1 if v == "semi" else (2 if v == "manual" else 0)
	var move_code := func(v: String) -> int:
		return 1 if v == "low" else (2 if v == "high" else 0)
	var sw_code := _auto_switch_code()
	var def_bit := 1 if str(control_profile.get("defending_mode", "tactical")) == "legacy" else 0
	var analog_bit := 1 if bool(control_profile.get("analog_sprint", true)) else 0
	return (
		0x4000
		| (assist_code.call(str(control_profile.get("pass_assist", "assisted"))) & 0x3)
		| ((assist_code.call(str(control_profile.get("through_assist", "assisted"))) & 0x3) << 2)
		| ((assist_code.call(str(control_profile.get("shot_assist", "assisted"))) & 0x3) << 4)
		| ((assist_code.call(str(control_profile.get("cross_assist", "assisted"))) & 0x3) << 6)
		| ((def_bit & 0x1) << 8)
		| ((move_code.call(str(control_profile.get("auto_switch_move_assist", "low"))) & 0x3) << 9)
		| ((analog_bit & 0x1) << 11)
		| ((sw_code & 0x3) << 12)
	) & 0xFFFF

func _auto_switch_code() -> int:
	var sw := str(control_profile.get("auto_switch", "auto"))
	return 1 if sw == "air_balls" else (2 if sw == "manual" else 0)

func _pack_buttons_with_reserved(side: int, low_buttons: int, sprint_analog: int) -> int:
	var reserved: int = 0
	if side >= 0 and side < 2 and pending_settings_sync[side]:
		reserved = encode_control_settings_reserved()
		pending_settings_sync[side] = false
	else:
		# Bit 11 (0x0800) tags dual-map ControlProfile frames so psfc-sim disables legacy BTN_JOCKEY attacking aliases.
		reserved = ((_auto_switch_code() & 0x3) << 12) | 0x0800 | (clampi(sprint_analog, 0, 127) & 0x7F)
	return (low_buttons & 0xFFFF) | ((reserved & 0xFFFF) << 16)

func _register_psfc_mapping(dev_id: int) -> void:
	if detect_controller_profile(dev_id) == "PSFC RP2040 Pad" and not Input.is_joy_known(dev_id):
		Input.add_joy_mapping("%s,PSFC RP2040 Pad,%s" % [Input.get_joy_guid(dev_id), PSFC_SDL_MAPPING_BODY], true)

func _auto_bind_connected_pads() -> void:
	for dev_id in Input.get_connected_joypads():
		_register_psfc_mapping(dev_id)
		var guid := Input.get_joy_guid(dev_id)
		if bound_guid_by_side[0] == "":
			bind_device_to_side(dev_id, 0)
		elif bound_guid_by_side[1] == "" and bound_guid_by_side[0] != guid:
			bind_device_to_side(dev_id, 1)

func bind_device_to_side(device_id: int, side: int) -> bool:
	var guid := Input.get_joy_guid(device_id)
	var other_side := 1 - side
	if bound_guid_by_side[other_side] != "" and bound_guid_by_side[other_side] == guid:
		# Prevent a reconnecting controller from hijacking the opponent's team
		return false
	bound_guid_by_side[side] = guid
	active_device_by_side[side] = device_id
	if not calibration_by_guid.has(guid):
		calibration_by_guid[guid] = DeviceCalibration.new()
	return true

func _on_joy_connection_changed(device_id: int, connected: bool) -> void:
	if connected:
		_register_psfc_mapping(device_id)
	var guid := Input.get_joy_guid(device_id)
	for side in [0, 1]:
		if bound_guid_by_side[side] == guid:
			active_device_by_side[side] = device_id if connected else -1

func sample_input_frame(side: int, is_attacking: bool = false) -> Dictionary:
	var dev_id: int = active_device_by_side.get(side, -1)
	if dev_id >= 0:
		return _sample_joypad(side, dev_id, bound_guid_by_side[side], is_attacking)
	if side == 0:
		return _sample_keyboard_p1(side, is_attacking)
	return {
		"move_x": 0,
		"move_y": 0,
		"aim_x": 0,
		"aim_y": 0,
		"buttons": _pack_buttons_with_reserved(side, 0, 0),
	}

func detect_controller_profile(dev_id: int) -> String:
	var name_lc := Input.get_joy_name(dev_id).to_lower()
	var guid_lc := Input.get_joy_guid(dev_id).to_lower()
	# PSFC RP2040 pad (VID 0x1209 / PID 0xF5FC) reports physical buttons in standard order (firmware v2).
	if name_lc.contains("psfc") or name_lc.contains("parallel season") or (guid_lc.contains("0912") and guid_lc.contains("fcf5")):
		return "PSFC RP2040 Pad"
	if name_lc.contains("dualsense") or name_lc.contains("dualshock") or name_lc.contains("ps5") or name_lc.contains("ps4") or name_lc.contains("wireless controller"):
		return "PlayStation DualSense/DS4"
	if name_lc.contains("8bitdo"):
		return "8BitDo Controller"
	if name_lc.contains("switch") or name_lc.contains("pro controller"):
		return "Switch Pro Controller"
	if name_lc.contains("logitech") or name_lc.contains("f310") or name_lc.contains("f710"):
		return "Logitech Gamepad"
	if name_lc.contains("xbox") or name_lc.contains("xinput") or name_lc.contains("microsoft"):
		return "Xbox XInput Controller"
	return "Standard USB/BT Gamepad"

func _sample_joypad(side: int, dev_id: int, guid: String, is_attacking: bool = false) -> Dictionary:
	var cal: DeviceCalibration = calibration_by_guid.get(guid, DeviceCalibration.new())
	var lvec := _apply_circular_deadzone(
		Vector2(Input.get_joy_axis(dev_id, JOY_AXIS_LEFT_X), Input.get_joy_axis(dev_id, JOY_AXIS_LEFT_Y)),
		cal.dead_zone
	)
	var rvec := _apply_circular_deadzone(
		Vector2(Input.get_joy_axis(dev_id, JOY_AXIS_RIGHT_X), Input.get_joy_axis(dev_id, JOY_AXIS_RIGHT_Y)),
		maxf(cal.dead_zone, 0.16)
	)

	# D-Pad fallback when left stick is neutral
	if lvec.length() < 0.05:
		var dx := 0.0
		var dy := 0.0
		if Input.is_joy_button_pressed(dev_id, JOY_BUTTON_DPAD_RIGHT):
			dx += 1.0
		if Input.is_joy_button_pressed(dev_id, JOY_BUTTON_DPAD_LEFT):
			dx -= 1.0
		if Input.is_joy_button_pressed(dev_id, JOY_BUTTON_DPAD_DOWN):
			dy += 1.0
		if Input.is_joy_button_pressed(dev_id, JOY_BUTTON_DPAD_UP):
			dy -= 1.0
		var dv := Vector2(dx, dy)
		if dv.length() > 1.0:
			dv = dv.normalized()
		lvec = dv

	if cal.invert_x:
		lvec.x = -lvec.x
		rvec.x = -rvec.x
	if cal.invert_y:
		lvec.y = -lvec.y
		rvec.y = -rvec.y

	var lt_val: float = clampf(Input.get_joy_axis(dev_id, JOY_AXIS_TRIGGER_LEFT), 0.0, 1.0)
	if Input.is_joy_button_pressed(dev_id, JOY_BUTTON_PADDLE1):
		lt_val = 1.0
	var rt_val: float = clampf(Input.get_joy_axis(dev_id, JOY_AXIS_TRIGGER_RIGHT), 0.0, 1.0)
	if Input.is_joy_button_pressed(dev_id, JOY_BUTTON_PADDLE2):
		rt_val = 1.0

	var phys: Dictionary = {
		"south": Input.is_joy_button_pressed(dev_id, JOY_BUTTON_A),
		"east": Input.is_joy_button_pressed(dev_id, JOY_BUTTON_B),
		"west": Input.is_joy_button_pressed(dev_id, JOY_BUTTON_X),
		"north": Input.is_joy_button_pressed(dev_id, JOY_BUTTON_Y),
		"lb": Input.is_joy_button_pressed(dev_id, JOY_BUTTON_LEFT_SHOULDER),
		"rb": Input.is_joy_button_pressed(dev_id, JOY_BUTTON_RIGHT_SHOULDER),
		"lt": lt_val > 0.10,
		"rt": rt_val > 0.10,
		"select": Input.is_joy_button_pressed(dev_id, JOY_BUTTON_BACK),
	}

	var extra_skill := Input.is_joy_button_pressed(dev_id, JOY_BUTTON_RIGHT_STICK) or Input.is_joy_button_pressed(dev_id, JOY_BUTTON_LEFT_STICK)
	return _map_physical_to_frame(side, lvec, rvec, lt_val, rt_val, phys, extra_skill, is_attacking)

## Returns the bound physical input name, or "" when unbound / invalid.
static func _binding(ctx_map: Dictionary, action: String) -> String:
	var v = ctx_map.get(action, null)
	return str(v) if typeof(v) == TYPE_STRING and PHYSICAL_INPUTS.has(v) else ""

func _map_physical_to_frame(
	side: int,
	lvec: Vector2,
	rvec: Vector2,
	lt_val: float,
	rt_val: float,
	phys: Dictionary,
	extra_skill: bool = false,
	is_attacking: bool = false
) -> Dictionary:
	var att: Dictionary = control_profile.get("attack", CONTROL_PRESETS["classic"]["attack"])
	var def: Dictionary = control_profile.get("defense", CONTROL_PRESETS["classic"]["defense"])
	var btns: int = 0
	var ctx_pass_mask: int = 0

	# Attacking (In Possession) bindings — shared pass-family inputs go to the context resolver
	for action in ATTACK_ACTION_BITS.keys():
		var key := _binding(att, action)
		if key == "" or not bool(phys.get(key, false)):
			continue
		if PASS_FAMILY.has(action):
			var sharers := 0
			for other in PASS_FAMILY:
				if _binding(att, other) == key:
					sharers += 1
			if sharers > 1:
				ctx_pass_mask |= int(ATTACK_ACTION_BITS[action])
				continue
		btns |= int(ATTACK_ACTION_BITS[action])

	# Defending (Out of Possession) bindings
	for action in DEFENSE_ACTION_BITS.keys():
		var key := _binding(def, action)
		if key != "" and bool(phys.get(key, false)):
			btns |= int(DEFENSE_ACTION_BITS[action])

	# Two-Button: latch one pass bit at the press edge for the whole hold (power charge)
	if side >= 0 and side < 2:
		if ctx_pass_mask == 0:
			pass_latch_by_side[side] = 0
		else:
			if pass_latch_by_side[side] == 0 or (ctx_pass_mask & pass_latch_by_side[side]) == 0:
				pass_latch_by_side[side] = resolve_context_pass(ctx_pass_mask, pass_context_by_side[side])
			btns |= pass_latch_by_side[side]

	# Analog or Digital Sprint (> 0.10 deadzone), resolved strictly from active Attack/Defense context
	var active_sprint_key: String = _binding(att, "sprint") if is_attacking else _binding(def, "sprint")
	var sprint_depth: float = 0.0
	if active_sprint_key == "rt":
		sprint_depth = rt_val
	elif active_sprint_key == "lt":
		sprint_depth = lt_val
	elif active_sprint_key != "":
		sprint_depth = 1.0 if phys.get(active_sprint_key, false) else 0.0
	var sprint_analog: int = 0
	if sprint_depth > 0.10:
		btns |= BTN_SPRINT
		if bool(control_profile.get("analog_sprint", true)):
			sprint_analog = clampi(int(round(sprint_depth * 127.0)), 1, 127)
		else:
			sprint_analog = 127

	if extra_skill or phys.get("select", false):
		btns |= BTN_SKILL

	# Always keep pure Left Stick in (move_x, move_y) and pure Right Stick in (aim_x, aim_y):
	# psfc-sim aims human kicks with Left Stick while keeping Right Stick independent for
	# Directional Player Switching (even while holding Contain/Jockey) and Skill Moves / Knock-On!
	var mx: int = clampi(int(round(lvec.x * 127.0)), -127, 127)
	var my: int = clampi(int(round(lvec.y * 127.0)), -127, 127)
	var rx_i: int = clampi(int(round(rvec.x * 127.0)), -127, 127)
	var ry_i: int = clampi(int(round(rvec.y * 127.0)), -127, 127)

	return {
		"move_x": mx,
		"move_y": my,
		"aim_x": rx_i,
		"aim_y": ry_i,
		"buttons": _pack_buttons_with_reserved(side, btns, sprint_analog),
	}

## Keyboard P1 matches the web client's physical layout (GLYPH_TABLES.keyboard in control_profile.js):
## south Space/J, east L/Enter, west U/C, north K, lb Q, rb E, lt F, rt Shift, select Z.
func _sample_keyboard_p1(side: int, is_attacking: bool = false) -> Dictionary:
	var mx := 0.0
	var my := 0.0
	if Input.is_key_pressed(KEY_D) or Input.is_key_pressed(KEY_RIGHT):
		mx += 1.0
	if Input.is_key_pressed(KEY_A) or Input.is_key_pressed(KEY_LEFT):
		mx -= 1.0
	if Input.is_key_pressed(KEY_W) or Input.is_key_pressed(KEY_UP):
		my -= 1.0
	if Input.is_key_pressed(KEY_S) or Input.is_key_pressed(KEY_DOWN):
		my += 1.0
	var v := Vector2(mx, my)
	if v.length() > 1.0:
		v = v.normalized()

	var sprinting := Input.is_key_pressed(KEY_SHIFT)
	var lt_held := Input.is_key_pressed(KEY_F)
	var phys: Dictionary = {
		"south": Input.is_key_pressed(KEY_J) or Input.is_key_pressed(KEY_SPACE),
		"east": Input.is_key_pressed(KEY_L) or Input.is_key_pressed(KEY_ENTER),
		"west": Input.is_key_pressed(KEY_U) or Input.is_key_pressed(KEY_C),
		"north": Input.is_key_pressed(KEY_K),
		"lb": Input.is_key_pressed(KEY_Q),
		"rb": Input.is_key_pressed(KEY_E),
		"lt": lt_held,
		"rt": sprinting,
		"select": Input.is_key_pressed(KEY_Z),
	}
	return _map_physical_to_frame(side, v, Vector2.ZERO, 1.0 if lt_held else 0.0, 1.0 if sprinting else 0.0, phys, false, is_attacking)

func _apply_circular_deadzone(raw: Vector2, dz: float) -> Vector2:
	var mag := raw.length()
	if mag <= dz:
		return Vector2.ZERO
	var scaled := clampf((minf(mag, 1.0) - dz) / (1.0 - dz), 0.0, 1.0)
	return (raw / mag) * scaled
