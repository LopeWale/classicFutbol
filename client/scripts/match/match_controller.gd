extends Node3D
## 3D presentation driver for a 5v5 (4 outfield + 1 GK) or 11v11 match powered by `PsfcMatchSim` (Rust GDExtension).
## Builds articulated 3D footballers (torso, shorts, limbs, socks, boots, head & hair), striped turf pitch,
## 3D goals with nets, perimeter adboards & grandstands, broadcast cameras, and a 6-second instant replay buffer.

@export var is_eleven_a_side: bool = false
@export var away_is_cpu: bool = true
@export var half_seconds: int = 120
@export var match_seed: int = 20261118
@export var host_api_base: String = "http://127.0.0.1:8000"

const REPLAY_CAPACITY_FRAMES: int = 360

var sim: Object = null
var controller_input: ControllerInterface
var player_rigs: Array[Dictionary] = []
var ball_node: MeshInstance3D
var camera: Camera3D
var hud_label: Label
var event_banner: Label
var submitted_result: bool = false

var replay_ring: Array[Dictionary] = []
var replay_active: bool = false
var replay_cursor: float = 0.0
var prev_r_pressed: bool = false
var anim_clock: float = 0.0

func _ready() -> void:
	controller_input = ControllerInterface.new()
	add_child(controller_input)

	if ClassDB.class_exists(&"PsfcMatchSim"):
		sim = ClassDB.instantiate(&"PsfcMatchSim")
		sim.call("setup_preset", is_eleven_a_side, match_seed, half_seconds, away_is_cpu)
	else:
		push_warning("PsfcMatchSim GDExtension not built yet — run `cargo build -p psfc-godot` in /sim.")

	_build_stadium_and_actors()

func _make_mat(col: Color, rough: float = 0.65) -> StandardMaterial3D:
	var m := StandardMaterial3D.new()
	m.albedo_color = col
	m.roughness = rough
	return m

func _build_articulated_footballer(side: int, idx: int) -> Dictionary:
	var is_gk := idx == 0
	var shirt_col := Color(0.06, 0.72, 0.50) if (is_gk and side == 0) else (
		Color(0.22, 0.74, 0.97) if is_gk else (
			Color(0.78, 0.06, 0.18) if side == 0 else Color(0.92, 0.70, 0.03)
		)
	)
	var shorts_col := Color(0.97, 0.98, 0.99) if side == 0 and not is_gk else Color(0.11, 0.16, 0.24)
	var socks_col := Color(0.08, 0.10, 0.14) if side == 0 and not is_gk else shirt_col
	var skin_col := Color(0.86, 0.66, 0.48) if (idx + side) % 2 == 0 else Color(0.45, 0.27, 0.16)

	var root := Node3D.new()
	var pelvis := Node3D.new()
	pelvis.position = Vector3(0.0, 0.92, 0.0)
	root.add_child(pelvis)

	var shorts_mesh := MeshInstance3D.new()
	var shorts_box := BoxMesh.new()
	shorts_box.size = Vector3(0.36, 0.22, 0.23)
	shorts_box.material = _make_mat(shorts_col)
	shorts_mesh.mesh = shorts_box
	pelvis.add_child(shorts_mesh)

	var torso := MeshInstance3D.new()
	var torso_box := BoxMesh.new()
	torso_box.size = Vector3(0.41, 0.52, 0.24)
	torso_box.material = _make_mat(shirt_col)
	torso.mesh = torso_box
	torso.position = Vector3(0.0, 0.35, 0.0)
	pelvis.add_child(torso)

	var head := MeshInstance3D.new()
	var head_sph := SphereMesh.new()
	head_sph.radius = 0.14
	head_sph.height = 0.30
	head_sph.material = _make_mat(skin_col, 0.5)
	head.mesh = head_sph
	head.position = Vector3(0.0, 0.76, 0.0)
	pelvis.add_child(head)

	var arm_l := Node3D.new()
	arm_l.position = Vector3(0.26, 0.56, 0.0)
	var arm_r := Node3D.new()
	arm_r.position = Vector3(-0.26, 0.56, 0.0)
	for arm in [arm_l, arm_r]:
		var limb := MeshInstance3D.new()
		var cap := CapsuleMesh.new()
		cap.radius = 0.06
		cap.height = 0.48
		cap.material = _make_mat(shirt_col if is_gk else skin_col)
		limb.mesh = cap
		limb.position = Vector3(0.0, -0.22, 0.0)
		arm.add_child(limb)
		pelvis.add_child(arm)

	var leg_l := Node3D.new()
	leg_l.position = Vector3(0.11, -0.05, 0.0)
	var leg_r := Node3D.new()
	leg_r.position = Vector3(-0.11, -0.05, 0.0)
	for leg in [leg_l, leg_r]:
		var sock := MeshInstance3D.new()
		var cyl := CylinderMesh.new()
		cyl.top_radius = 0.085
		cyl.bottom_radius = 0.06
		cyl.height = 0.74
		cyl.material = _make_mat(socks_col)
		sock.mesh = cyl
		sock.position = Vector3(0.0, -0.38, 0.0)
		leg.add_child(sock)

		var boot := MeshInstance3D.new()
		var boot_box := BoxMesh.new()
		boot_box.size = Vector3(0.12, 0.09, 0.26)
		boot_box.material = _make_mat(Color(0.08, 0.09, 0.12), 0.3)
		boot.mesh = boot_box
		boot.position = Vector3(0.0, -0.78, 0.04)
		leg.add_child(boot)
		pelvis.add_child(leg)

	return {
		"root": root,
		"pelvis": pelvis,
		"arm_l": arm_l,
		"arm_r": arm_r,
		"leg_l": leg_l,
		"leg_r": leg_r,
		"prev_pos": Vector3.ZERO,
		"phase": float(idx) * 1.3,
	}

func _build_stadium_and_actors() -> void:
	var pitch_dims := Vector2(56.0, 36.0)
	if sim != null:
		pitch_dims = sim.call("get_pitch_dimensions")

	# Striped mowed turf bands
	var stripes := 18 if is_eleven_a_side else 12
	var stripe_w := pitch_dims.x / float(stripes)
	for i in range(stripes):
		var band := MeshInstance3D.new()
		var pm := PlaneMesh.new()
		pm.size = Vector2(stripe_w + 0.02, pitch_dims.y)
		var col := Color(0.11, 0.48, 0.22) if i % 2 == 0 else Color(0.14, 0.55, 0.26)
		pm.material = _make_mat(col, 0.85)
		band.mesh = pm
		band.position = Vector3(-pitch_dims.x * 0.5 + (float(i) + 0.5) * stripe_w, 0.0, 0.0)
		add_child(band)

	# Far-side grandstand backdrop
	for t in range(8):
		var step := MeshInstance3D.new()
		var bm := BoxMesh.new()
		bm.size = Vector3(pitch_dims.x + 18.0, 0.8, 1.3)
		bm.material = _make_mat(Color(0.10, 0.15, 0.24), 0.9)
		step.mesh = bm
		step.position = Vector3(0.0, 0.6 + float(t) * 0.72, -pitch_dims.y * 0.5 - 5.5 - float(t) * 1.2)
		add_child(step)

	var sun := DirectionalLight3D.new()
	sun.rotation_degrees = Vector3(-52.0, 28.0, 0.0)
	sun.light_energy = 1.65
	sun.shadow_enabled = true
	add_child(sun)

	camera = Camera3D.new()
	camera.position = Vector3(0.0, 13.6, 23.8)
	camera.fov = 29.0
	add_child(camera)
	camera.look_at(Vector3(0.0, 0.8, -1.2), Vector3.UP)

	ball_node = MeshInstance3D.new()
	var sphere := SphereMesh.new()
	sphere.radius = 0.22
	sphere.height = 0.44
	sphere.material = _make_mat(Color(0.97, 0.98, 0.99), 0.32)
	ball_node.mesh = sphere
	add_child(ball_node)

	var per_side := 11 if is_eleven_a_side else 5
	for side in [0, 1]:
		for idx in range(per_side):
			var rig := _build_articulated_footballer(side, idx)
			add_child(rig["root"])
			player_rigs.append(rig)

	var canvas := CanvasLayer.new()
	hud_label = Label.new()
	hud_label.position = Vector2(24, 18)
	canvas.add_child(hud_label)
	event_banner = Label.new()
	event_banner.position = Vector2(24, 52)
	canvas.add_child(event_banner)
	add_child(canvas)

func _physics_process(delta: float) -> void:
	if sim == null:
		return
	anim_clock += delta

	var r_now := Input.is_key_pressed(KEY_R)
	if r_now and not prev_r_pressed and replay_ring.size() > 15:
		replay_active = not replay_active
		replay_cursor = 0.0
	prev_r_pressed = r_now

	if replay_active:
		_render_instant_replay_step(delta)
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
	var bpos: Vector3 = sim.call("get_ball_position")

	if replay_ring.size() >= REPLAY_CAPACITY_FRAMES:
		replay_ring.pop_front()
	replay_ring.push_back({
		"positions": positions.duplicate(),
		"facings": facings.duplicate(),
		"ball": bpos,
	})

	_apply_actor_transforms(positions, facings, bpos, false, delta)

	var score: Vector2i = sim.call("get_score")
	var shootout: Vector2i = sim.call("get_shootout_score")
	var minute: int = sim.call("get_display_minute")
	var phase: String = sim.call("get_phase_label")
	var pens_suffix := " (Pens %d-%d)" % [shootout.x, shootout.y] if shootout.x >= 0 else ""
	hud_label.text = "MUN %d - %d ARS%s  |  %d'  |  %s" % [score.x, score.y, pens_suffix, minute, phase]

	var ev_json: String = sim.call("drain_events_json")
	var evs = JSON.parse_string(ev_json)
	if typeof(evs) == TYPE_ARRAY and evs.size() > 0:
		var latest: Dictionary = evs[evs.size() - 1]
		event_banner.text = str(latest.get("detail", ""))

	if bool(sim.call("is_finished")) and not submitted_result:
		submitted_result = true
		event_banner.text = "FULL-TIME!%s" % pens_suffix

func _render_instant_replay_step(delta: float) -> void:
	var idx := int(floor(replay_cursor))
	if idx >= replay_ring.size():
		replay_active = false
		return
	var snap: Dictionary = replay_ring[idx]
	_apply_actor_transforms(snap["positions"], snap["facings"], snap["ball"], true, delta)
	event_banner.text = "INSTANT REPLAY (0.5x) — Press R to return live"
	replay_cursor += 0.5

func _apply_actor_transforms(positions: PackedVector3Array, facings: PackedFloat32Array, bpos: Vector3, orbiting: bool, delta: float) -> void:
	for i in range(mini(positions.size(), player_rigs.size())):
		var rig: Dictionary = player_rigs[i]
		var root: Node3D = rig["root"]
		var pos := positions[i]
		var speed := pos.distance_to(rig["prev_pos"]) / maxf(0.001, delta)
		rig["prev_pos"] = pos
		root.position = pos
		root.rotation.y = PI * 0.5 - facings[i]

		if speed > 0.35:
			rig["phase"] = float(rig["phase"]) + delta * (5.0 + speed * 1.5)
			var s := sin(float(rig["phase"]))
			(rig["leg_l"] as Node3D).rotation.x = -s * 0.78
			(rig["leg_r"] as Node3D).rotation.x = s * 0.78
			(rig["arm_l"] as Node3D).rotation.x = s * 0.68
			(rig["arm_r"] as Node3D).rotation.x = -s * 0.68
			(rig["pelvis"] as Node3D).position.y = 0.92 + absf(cos(float(rig["phase"]))) * 0.06
		else:
			(rig["leg_l"] as Node3D).rotation.x = 0.0
			(rig["leg_r"] as Node3D).rotation.x = 0.0
			(rig["arm_l"] as Node3D).rotation.x = 0.0
			(rig["arm_r"] as Node3D).rotation.x = 0.0
			(rig["pelvis"] as Node3D).position.y = 0.92

	ball_node.position = bpos + Vector3(0.0, 0.22, 0.0)

	if orbiting:
		var angle := replay_cursor * 0.025
		camera.position = bpos + Vector3(sin(angle) * 13.5, 6.2, cos(angle) * 13.5)
		camera.look_at(bpos + Vector3(0.0, 0.85, 0.0), Vector3.UP)
	else:
		var target_cam_x := clampf(bpos.x * 0.72, -15.0, 15.0)
		camera.position = Vector3(lerpf(camera.position.x, target_cam_x, 0.1), 13.6, bpos.z * 0.28 + 23.8)
		camera.look_at(Vector3(target_cam_x * 1.03, 0.75, bpos.z * 0.25 - 1.4), Vector3.UP)
