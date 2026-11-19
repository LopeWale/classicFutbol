extends Node3D
## 3D presentation driver for a 5v5 (4 outfield + 1 GK) or 11v11 match powered by `PsfcMatchSim` (Rust GDExtension).

@export var is_eleven_a_side: bool = false
@export var away_is_cpu: bool = true
@export var half_seconds: int = 120
@export var match_seed: int = 20261118
@export var host_api_base: String = "http://127.0.0.1:8000"

var sim: Object = null
var controller_input: ControllerInterface
var player_nodes: Array[Node3D] = []
var ball_node: MeshInstance3D
var camera: Camera3D
var hud_label: Label
var event_banner: Label
var submitted_result: bool = false

func _ready() -> void:
	controller_input = ControllerInterface.new()
	add_child(controller_input)

	if ClassDB.class_exists(&"PsfcMatchSim"):
		sim = ClassDB.instantiate(&"PsfcMatchSim")
		sim.call("setup_preset", is_eleven_a_side, match_seed, half_seconds, away_is_cpu)
	else:
		push_warning("PsfcMatchSim GDExtension not built yet — run `cargo build -p psfc-godot` in /sim.")

	_build_stadium_and_actors()

func _build_stadium_and_actors() -> void:
	var pitch_dims := Vector2(56.0, 36.0)
	if sim != null:
		pitch_dims = sim.call("get_pitch_dimensions")

	# Pitch surface
	var pitch := MeshInstance3D.new()
	var plane := PlaneMesh.new()
	plane.size = Vector2(pitch_dims.x + 6.0, pitch_dims.y + 6.0)
	var grass_mat := StandardMaterial3D.new()
	grass_mat.albedo_color = Color(0.13, 0.44, 0.20)
	plane.material = grass_mat
	pitch.mesh = plane
	add_child(pitch)

	# Directional stadium light
	var sun := DirectionalLight3D.new()
	sun.rotation_degrees = Vector3(-52.0, 28.0, 0.0)
	sun.shadow_enabled = true
	add_child(sun)

	# Broadcast camera
	camera = Camera3D.new()
	camera.position = Vector3(0.0, 22.0, 26.0)
	camera.fov = 42.0
	add_child(camera)
	camera.look_at(Vector3.ZERO, Vector3.UP)

	# Ball
	ball_node = MeshInstance3D.new()
	var sphere := SphereMesh.new()
	sphere.radius = 0.16
	sphere.height = 0.32
	ball_node.mesh = sphere
	add_child(ball_node)

	# Players (5 per side in 5v5, 11 per side in 11v11)
	var per_side := 11 if is_eleven_a_side else 5
	for side in [0, 1]:
		for idx in range(per_side):
			var holder := Node3D.new()
			var body := MeshInstance3D.new()
			var cap := CapsuleMesh.new()
			cap.radius = 0.34
			cap.height = 1.75
			var mat := StandardMaterial3D.new()
			if idx == 0:
				mat.albedo_color = Color(0.95, 0.78, 0.15) if side == 0 else Color(0.15, 0.85, 0.48)
			else:
				mat.albedo_color = Color(0.85, 0.12, 0.16) if side == 0 else Color(0.12, 0.32, 0.85)
			cap.material = mat
			body.mesh = cap
			body.position = Vector3(0.0, 0.88, 0.0)
			holder.add_child(body)
			add_child(holder)
			player_nodes.append(holder)

	# Simple HUD overlay
	var canvas := CanvasLayer.new()
	hud_label = Label.new()
	hud_label.position = Vector2(24, 18)
	canvas.add_child(hud_label)
	event_banner = Label.new()
	event_banner.position = Vector2(24, 52)
	canvas.add_child(event_banner)
	add_child(canvas)

func _physics_process(_delta: float) -> void:
	if sim == null:
		return

	var h := controller_input.sample_input_frame(0)
	var a := controller_input.sample_input_frame(1)
	sim.call(
		"tick",
		h["move_x"], h["move_y"], h["aim_x"], h["aim_y"], h["buttons"],
		a["move_x"], a["move_y"], a["aim_x"], a["aim_y"], a["buttons"]
	)

	var positions: PackedVector3Array = sim.call("get_player_positions")
	var facings: PackedFloat32Array = sim.call("get_player_facings_rad")
	for i in range(mini(positions.size(), player_nodes.size())):
		player_nodes[i].position = positions[i]
		player_nodes[i].rotation.y = -facings[i]

	var bpos: Vector3 = sim.call("get_ball_position")
	ball_node.position = bpos + Vector3(0.0, 0.16, 0.0)

	# Smooth broadcast camera follow
	var target_cam_x := clampf(bpos.x * 0.55, -14.0, 14.0)
	camera.position.x = lerpf(camera.position.x, target_cam_x, 0.08)
	camera.look_at(Vector3(bpos.x * 0.35, 0.0, bpos.z * 0.2), Vector3.UP)

	var score: Vector2i = sim.call("get_score")
	var minute: int = sim.call("get_display_minute")
	var phase: String = sim.call("get_phase_label")
	var state_hash: String = sim.call("get_state_hash_hex")
	hud_label.text = "MUN %d - %d ARS  |  %d'  |  %s  |  hash %s" % [score.x, score.y, minute, phase, state_hash.substr(0, 8)]

	var ev_json: String = sim.call("drain_events_json")
	var evs = JSON.parse_string(ev_json)
	if typeof(evs) == TYPE_ARRAY and evs.size() > 0:
		var latest: Dictionary = evs[evs.size() - 1]
		event_banner.text = str(latest.get("detail", ""))

	if bool(sim.call("is_finished")) and not submitted_result:
		submitted_result = true
		event_banner.text = "Full-Time! Deterministic hash: %s" % state_hash
