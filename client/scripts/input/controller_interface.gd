class_name ControllerInterface
extends Node
## Action-based FIFA 10–12 input abstraction layer for Godot 4 (`PsfcMatchSim`).
## Separates Layer 1 (physical gamepad / keyboard with 360° radial deadzone & analog triggers)
## from Layer 2 (JSON-serializable ControlProfile with Classic/Alternate/Two-Button/Custom presets,
## distinct Attacking/Defending binding maps, and deterministic `InputFrame.reserved` settings sync).

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
			"switch_player": "south",
		},
	},
}

class DeviceCalibration:
	var dead_zone: float = 0.12
	var invert_x: bool = false
	var invert_y: bool = true # Godot screen +Y is down; pitch +Y is up/far touchline

# Maps side (0 = Home, 1 = Away) -> bound joypad GUID ("" if keyboard or unbound)
var bound_guid_by_side: Dictionary = {0: "", 1: ""}
var active_device_by_side: Dictionary = {0: -1, 1: -1}
var calibration_by_guid: Dictionary = {}
var control_profile: Dictionary = {}
var pending_settings_sync: Array[bool] = [true, true]

func _ready() -> void:
	load_control_profile()
	Input.joy_connection_changed.connect(_on_joy_connection_changed)
	_auto_bind_connected_pads()

func default_control_profile(preset_name: String = "classic") -> Dictionary:
	var p_key := preset_name if CONTROL_PRESETS.has(preset_name) else "classic"
	var base: Dictionary = CONTROL_PRESETS[p_key]
	return {
		"version": 2,
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

func load_control_profile() -> void:
	control_profile = default_control_profile("classic")
	if FileAccess.file_exists(PROFILE_PATH):
		var f := FileAccess.open(PROFILE_PATH, FileAccess.READ)
		if f != null:
			var parsed = JSON.parse_string(f.get_as_text())
			if typeof(parsed) == TYPE_DICTIONARY:
				for k in control_profile.keys():
					if parsed.has(k):
						control_profile[k] = parsed[k]
	pending_settings_sync = [true, true]

func save_control_profile() -> void:
	var f := FileAccess.open(PROFILE_PATH, FileAccess.WRITE)
	if f != null:
		f.store_string(JSON.stringify(control_profile, "\t"))
	pending_settings_sync = [true, true]

func set_preset(preset_name: String) -> void:
	if CONTROL_PRESETS.has(preset_name):
		control_profile["preset"] = preset_name
		control_profile["attack"] = CONTROL_PRESETS[preset_name]["attack"].duplicate()
		control_profile["defense"] = CONTROL_PRESETS[preset_name]["defense"].duplicate()
		save_control_profile()

func rebind_action(context: String, action_key: String, physical_input: String) -> void:
	if context != "attack" and context != "defense":
		return
	var ctx_map: Dictionary = control_profile[context]
	var prev_phys: String = str(ctx_map.get(action_key, ""))
	if prev_phys == physical_input:
		return
	for other_key in ctx_map.keys():
		if other_key != action_key and str(ctx_map[other_key]) == physical_input:
			ctx_map[other_key] = prev_phys
			break
	ctx_map[action_key] = physical_input
	control_profile["preset"] = "custom"
	save_control_profile()

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
		reserved = ((_auto_switch_code() & 0x3) << 12) | (clampi(sprint_analog, 0, 127) & 0x7F)
	return (low_buttons & 0xFFFF) | ((reserved & 0xFFFF) << 16)

func _auto_bind_connected_pads() -> void:
	for dev_id in Input.get_connected_joypads():
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
	var guid := Input.get_joy_guid(device_id)
	for side in [0, 1]:
		if bound_guid_by_side[side] == guid:
			active_device_by_side[side] = device_id if connected else -1

func sample_input_frame(side: int) -> Dictionary:
	var dev_id: int = active_device_by_side.get(side, -1)
	if dev_id >= 0:
		return _sample_joypad(side, dev_id, bound_guid_by_side[side])
	if side == 0:
		return _sample_keyboard_p1(side)
	return {
		"move_x": 0,
		"move_y": 0,
		"aim_x": 0,
		"aim_y": 0,
		"buttons": _pack_buttons_with_reserved(side, 0, 0),
	}

func detect_controller_profile(dev_id: int) -> String:
	var name_lc := Input.get_joy_name(dev_id).to_lower()
	if name_lc.contains("psfc") or name_lc.contains("parallel season"):
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

func _sample_joypad(side: int, dev_id: int, guid: String) -> Dictionary:
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
	return _map_physical_to_frame(side, lvec, rvec, lt_val, rt_val, phys, extra_skill)

func _map_physical_to_frame(
	side: int,
	lvec: Vector2,
	rvec: Vector2,
	lt_val: float,
	rt_val: float,
	phys: Dictionary,
	extra_skill: bool = false
) -> Dictionary:
	var att: Dictionary = control_profile.get("attack", CONTROL_PRESETS["classic"]["attack"])
	var def: Dictionary = control_profile.get("defense", CONTROL_PRESETS["classic"]["defense"])
	var btns: int = 0

	# Attacking (In Possession) bindings
	if phys.get(att.get("pass", "south"), false):
		btns |= BTN_PASS
	if phys.get(att.get("shoot", "east"), false):
		btns |= BTN_SHOOT
	if phys.get(att.get("cross", "west"), false):
		btns |= BTN_LOB
	if phys.get(att.get("through_ball", "north"), false):
		btns |= BTN_THROUGH
	if phys.get(att.get("finesse", "rb"), false):
		btns |= BTN_FINESSE
	if phys.get(att.get("pace_control", "lt"), false):
		btns |= BTN_PACE_CONTROL
	if phys.get(att.get("chip_run", "lb"), false):
		btns |= BTN_CALL_RUN

	# Defending (Out of Possession) bindings
	if phys.get(def.get("contain", "south"), false):
		btns |= BTN_CONTAIN
	if phys.get(def.get("standing_tackle", "east"), false):
		btns |= BTN_TACKLE
	if phys.get(def.get("sliding_tackle", "west"), false):
		btns |= BTN_SLIDE
	if phys.get(def.get("rush_gk", "north"), false):
		btns |= BTN_GK_RUSH
	if phys.get(def.get("teammate_contain", "rb"), false):
		btns |= BTN_TEAMMATE_CONTAIN
	if phys.get(def.get("jockey", "lt"), false):
		btns |= BTN_JOCKEY
	if phys.get(def.get("switch_player", "lb"), false):
		btns |= BTN_SWITCH

	# Analog or Digital Sprint (> 0.10 deadzone)
	var sprint_key_att: String = str(att.get("sprint", "rt"))
	var sprint_key_def: String = str(def.get("sprint", "rt"))
	var depth_of := func(k: String) -> float:
		if k == "rt":
			return rt_val
		if k == "lt":
			return lt_val
		return 1.0 if phys.get(k, false) else 0.0
	var sprint_depth: float = maxf(depth_of.call(sprint_key_att), depth_of.call(sprint_key_def))
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

func _sample_keyboard_p1(side: int) -> Dictionary:
	var mx := 0.0
	var my := 0.0
	if Input.is_key_pressed(KEY_D) or Input.is_key_pressed(KEY_RIGHT):
		mx += 1.0
	if Input.is_key_pressed(KEY_A) or Input.is_key_pressed(KEY_LEFT):
		mx -= 1.0
	if Input.is_key_pressed(KEY_W) or Input.is_key_pressed(KEY_UP):
		my += 1.0
	if Input.is_key_pressed(KEY_S) or Input.is_key_pressed(KEY_DOWN):
		my -= 1.0
	var v := Vector2(mx, my)
	if v.length() > 1.0:
		v = v.normalized()

	var sprinting := Input.is_key_pressed(KEY_SHIFT)
	var lt_held := Input.is_key_pressed(KEY_L) or Input.is_key_pressed(KEY_F)
	var phys: Dictionary = {
		"south": Input.is_key_pressed(KEY_J) or Input.is_key_pressed(KEY_SPACE),
		"east": Input.is_key_pressed(KEY_K) or Input.is_key_pressed(KEY_ENTER),
		"west": Input.is_key_pressed(KEY_U),
		"north": Input.is_key_pressed(KEY_I),
		"lb": Input.is_key_pressed(KEY_Q),
		"rb": Input.is_key_pressed(KEY_E),
		"lt": lt_held,
		"rt": sprinting,
		"select": Input.is_key_pressed(KEY_Z),
	}
	return _map_physical_to_frame(side, v, Vector2.ZERO, 1.0 if lt_held else 0.0, 1.0 if sprinting else 0.0, phys, false)

func _apply_circular_deadzone(raw: Vector2, dz: float) -> Vector2:
	var mag := raw.length()
	if mag <= dz:
		return Vector2.ZERO
	var scaled := clampf((minf(mag, 1.0) - dz) / (1.0 - dz), 0.0, 1.0)
	return (raw / mag) * scaled
