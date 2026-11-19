class_name ControllerInterface
extends Node
## Translates raw HID gamepad / keyboard state into calibrated 60 Hz InputFrames
## (including Phase 4/6 `BTN_SKILL` & Right-Stick skill-move flicks) and preserves
## participant-to-device bindings across disconnects (README §11).

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

class DeviceCalibration:
	var dead_zone: float = 0.12
	var invert_x: bool = false
	var invert_y: bool = true # Godot screen +Y is down; pitch +Y is up/far touchline

# Maps side (0 = Home, 1 = Away) -> bound joypad GUID ("" if keyboard or unbound)
var bound_guid_by_side: Dictionary = {0: "", 1: ""}
var active_device_by_side: Dictionary = {0: -1, 1: -1}
var calibration_by_guid: Dictionary = {}

func _ready() -> void:
	Input.joy_connection_changed.connect(_on_joy_connection_changed)
	_auto_bind_connected_pads()

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
		return _sample_joypad(dev_id, bound_guid_by_side[side])
	if side == 0:
		return _sample_keyboard_p1()
	return {"move_x": 0, "move_y": 0, "aim_x": 0, "aim_y": 0, "buttons": 0}

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

func _sample_joypad(dev_id: int, guid: String) -> Dictionary:
	var cal: DeviceCalibration = calibration_by_guid.get(guid, DeviceCalibration.new())
	var lx := _apply_deadzone(Input.get_joy_axis(dev_id, JOY_AXIS_LEFT_X), cal.dead_zone)
	var ly := _apply_deadzone(Input.get_joy_axis(dev_id, JOY_AXIS_LEFT_Y), cal.dead_zone)
	var rx := _apply_deadzone(Input.get_joy_axis(dev_id, JOY_AXIS_RIGHT_X), cal.dead_zone)
	var ry := _apply_deadzone(Input.get_joy_axis(dev_id, JOY_AXIS_RIGHT_Y), cal.dead_zone)

	# D-Pad fallback for off-the-shelf Xbox / DualSense / 8BitDo / USB pads when left stick is neutral
	if absf(lx) < 0.05 and absf(ly) < 0.05:
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
		lx = dv.x
		ly = dv.y

	if cal.invert_x:
		lx = -lx
		rx = -rx
	if cal.invert_y:
		ly = -ly
		ry = -ry

	var btns: int = 0
	if Input.is_joy_button_pressed(dev_id, JOY_BUTTON_A):
		btns |= BTN_PASS | BTN_TACKLE
	if Input.is_joy_button_pressed(dev_id, JOY_BUTTON_Y):
		btns |= BTN_THROUGH
	if Input.is_joy_button_pressed(dev_id, JOY_BUTTON_X):
		btns |= BTN_LOB | BTN_SLIDE
	if Input.is_joy_button_pressed(dev_id, JOY_BUTTON_B):
		btns |= BTN_SHOOT
	if Input.get_joy_axis(dev_id, JOY_AXIS_TRIGGER_RIGHT) > 0.25 or Input.is_joy_button_pressed(dev_id, JOY_BUTTON_PADDLE2):
		btns |= BTN_SPRINT
	if Input.get_joy_axis(dev_id, JOY_AXIS_TRIGGER_LEFT) > 0.25 or Input.is_joy_button_pressed(dev_id, JOY_BUTTON_PADDLE1):
		btns |= BTN_JOCKEY
	if Input.is_joy_button_pressed(dev_id, JOY_BUTTON_LEFT_SHOULDER):
		btns |= BTN_SWITCH
	if Input.is_joy_button_pressed(dev_id, JOY_BUTTON_RIGHT_SHOULDER):
		btns |= BTN_CALL_RUN | BTN_TACKLE
	if Input.is_joy_button_pressed(dev_id, JOY_BUTTON_RIGHT_STICK) or Input.is_joy_button_pressed(dev_id, JOY_BUTTON_LEFT_STICK):
		btns |= BTN_SKILL

	return {
		"move_x": clampi(int(round(lx * 127.0)), -127, 127),
		"move_y": clampi(int(round(ly * 127.0)), -127, 127),
		"aim_x": clampi(int(round(rx * 127.0)), -127, 127),
		"aim_y": clampi(int(round(ry * 127.0)), -127, 127),
		"buttons": btns,
	}

func _sample_keyboard_p1() -> Dictionary:
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

	var btns: int = 0
	if Input.is_key_pressed(KEY_J) or Input.is_key_pressed(KEY_SPACE):
		btns |= BTN_PASS | BTN_TACKLE
	if Input.is_key_pressed(KEY_I):
		btns |= BTN_THROUGH
	if Input.is_key_pressed(KEY_U):
		btns |= BTN_LOB | BTN_SLIDE
	if Input.is_key_pressed(KEY_K):
		btns |= BTN_SHOOT
	if Input.is_key_pressed(KEY_SHIFT):
		btns |= BTN_SPRINT
	if Input.is_key_pressed(KEY_L):
		btns |= BTN_JOCKEY
	if Input.is_key_pressed(KEY_Q):
		btns |= BTN_SWITCH
	if Input.is_key_pressed(KEY_E):
		btns |= BTN_CALL_RUN
	if Input.is_key_pressed(KEY_Z):
		btns |= BTN_SKILL

	return {
		"move_x": clampi(int(round(v.x * 127.0)), -127, 127),
		"move_y": clampi(int(round(v.y * 127.0)), -127, 127),
		"aim_x": 0,
		"aim_y": 0,
		"buttons": btns,
	}

func _apply_deadzone(raw: float, dz: float) -> float:
	if absf(raw) < dz:
		return 0.0
	return signf(raw) * ((absf(raw) - dz) / (1.0 - dz))
