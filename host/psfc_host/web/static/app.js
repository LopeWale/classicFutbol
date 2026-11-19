(() => {
  "use strict";

  // InputFrame button bitmasks (must match sim/psfc-sim/src/input.rs)
  const BTN_PASS = 1 << 0;
  const BTN_THROUGH = 1 << 1;
  const BTN_LOB = 1 << 2;
  const BTN_SHOOT = 1 << 3;
  const BTN_SPRINT = 1 << 4;
  const BTN_SWITCH = 1 << 5;
  const BTN_JOCKEY = 1 << 6;
  const BTN_TACKLE = 1 << 7;
  const BTN_SLIDE = 1 << 8;
  const BTN_CALL_RUN = 1 << 9;

  const PHASE_LABELS = {
    0: "KICK OFF",
    1: "OPEN PLAY",
    2: "THROW-IN",
    3: "CORNER KICK",
    4: "GOAL KICK",
    5: "FREE KICK",
    6: "PENALTY KICK",
    10: "GOAL!",
    11: "HALF-TIME",
    12: "FULL-TIME",
  };

  const DEFAULT_WORLD_ID = "world-prem-2026";

  // DOM References
  const canvas = document.getElementById("pitch-canvas");
  const ctx = canvas.getContext("2d");
  const wasmPill = document.getElementById("wasm-status-pill");
  const gamepadPill = document.getElementById("gamepad-status-pill");
  const hashPill = document.getElementById("hash-status-pill");

  const formatSelect = document.getElementById("format-select");
  const modeSelect = document.getElementById("mode-select");
  const durationSelect = document.getElementById("duration-select");
  const btnExhibition = document.getElementById("btn-exhibition");
  const btnPause = document.getElementById("btn-pause");
  const btnSimRest = document.getElementById("btn-sim-rest");
  const btnSubmitSession = document.getElementById("btn-submit-session");

  const sbContext = document.getElementById("sb-context");
  const sbHomeSwatch = document.getElementById("sb-home-swatch");
  const sbAwaySwatch = document.getElementById("sb-away-swatch");
  const sbHomeCode = document.getElementById("sb-home-code");
  const sbAwayCode = document.getElementById("sb-away-code");
  const sbScore = document.getElementById("sb-score");
  const sbHalf = document.getElementById("sb-half");
  const sbTime = document.getElementById("sb-time");
  const sbPhase = document.getElementById("sb-phase");

  const goalBanner = document.getElementById("goal-banner");
  const goalBannerTitle = document.getElementById("goal-banner-title");
  const goalBannerSub = document.getElementById("goal-banner-sub");
  const verifyBanner = document.getElementById("verify-banner");
  const eventFeed = document.getElementById("match-event-feed");
  const possessionBadge = document.getElementById("possession-badge");

  // Runtime state
  let wasm = null;
  let currentSnapshot = null;
  let activeOfficialSession = null; // { sessionId, fixtureId }
  let paused = false;
  let goalBannerTicks = 0;
  let camX = 0.0;
  let camY = 0.0;
  let ballTrail = [];
  let homePossTicks = 1;
  let awayPossTicks = 1;
  const keysDown = new Set();
  const enc = new TextEncoder();
  const dec = new TextDecoder();

  // --- WASM Bridge Helpers ---

  function wasmReadText() {
    if (!wasm) return "";
    const ptr = wasm.wasm_output_ptr();
    const len = wasm.wasm_output_len();
    return dec.decode(new Uint8Array(wasm.memory.buffer, ptr, len));
  }

  function wasmWriteInputText(str) {
    const bytes = enc.encode(str);
    const ptr = wasm.wasm_prepare_input_buf(bytes.length);
    new Uint8Array(wasm.memory.buffer, ptr, bytes.length).set(bytes);
  }

  function readPackedState() {
    if (!wasm) return null;
    const count = wasm.sim_pack_render_state();
    if (!count) return null;
    const ptr = wasm.wasm_packed_state_ptr();
    return new Int32Array(wasm.memory.buffer, ptr, count);
  }

  // --- Controller Sampling (Keyboard + HTML5 Gamepad API) ---

  window.addEventListener("keydown", (ev) => {
    // Prevent page scroll on Space / Arrow keys when interacting with pitch
    if (
      document.activeElement &&
      ["SELECT", "INPUT", "TEXTAREA"].includes(document.activeElement.tagName)
    ) {
      return;
    }
    if (["Space", "ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"].includes(ev.code)) {
      ev.preventDefault();
    }
    keysDown.add(ev.code);
  });

  window.addEventListener("keyup", (ev) => {
    keysDown.delete(ev.code);
  });

  window.addEventListener("blur", () => keysDown.clear());

  function sampleGamepad(padIndex) {
    const pads = navigator.getGamepads ? navigator.getGamepads() : [];
    const pad = pads[padIndex];
    if (!pad || !pad.connected) return null;

    const deadzone = (v) => (Math.abs(v) < 0.16 ? 0 : v);
    const lx = deadzone(pad.axes[0] || 0);
    // Note: pitch +Y goes toward top/far or bottom/near consistently with screen:
    // screen +Y is down, so stick down (+axis[1]) -> +move_y!
    const ly = deadzone(pad.axes[1] || 0);
    const rx = deadzone(pad.axes[2] || 0);
    const ry = deadzone(pad.axes[3] || 0);

    const b = (i) => Boolean(pad.buttons[i] && pad.buttons[i].pressed);
    let buttons = 0;
    if (b(0)) buttons |= BTN_PASS; // Cross / A
    if (b(3)) buttons |= BTN_THROUGH; // Triangle / Y
    if (b(2)) buttons |= BTN_LOB; // Square / X
    if (b(1)) buttons |= BTN_SHOOT; // Circle / B
    if (b(7)) buttons |= BTN_SPRINT; // R2 / RT
    if (b(4)) buttons |= BTN_SWITCH | BTN_CALL_RUN; // L1 / LB
    if (b(6)) buttons |= BTN_JOCKEY; // L2 / LT
    if (b(1) && !b(0)) {
      // Defending stand tackle also bound to B/Circle or R1
    }
    if (b(5)) buttons |= BTN_TACKLE; // R1 / RB
    if (b(2)) {
      // Slide tackle when off-ball handled or dedicated X
    }

    return {
      mx: Math.round(lx * 127),
      my: Math.round(ly * 127),
      ax: Math.round(rx * 127),
      ay: Math.round(ry * 127),
      btns: buttons,
      id: pad.id,
    };
  }

  function sampleHomeInput(allowArrowKeys) {
    const gp = sampleGamepad(0);
    if (gp && (gp.mx !== 0 || gp.my !== 0 || gp.btns !== 0)) {
      gamepadPill.textContent = `Gamepad #1 Active`;
      return gp;
    }

    let dx = 0;
    let dy = 0;
    if (keysDown.has("KeyA") || (allowArrowKeys && keysDown.has("ArrowLeft"))) dx -= 1;
    if (keysDown.has("KeyD") || (allowArrowKeys && keysDown.has("ArrowRight"))) dx += 1;
    if (keysDown.has("KeyW") || (allowArrowKeys && keysDown.has("ArrowUp"))) dy -= 1;
    if (keysDown.has("KeyS") || (allowArrowKeys && keysDown.has("ArrowDown"))) dy += 1;

    if (dx !== 0 && dy !== 0) {
      dx *= Math.SQRT1_2;
      dy *= Math.SQRT1_2;
    }

    let btns = 0;
    if (keysDown.has("Space") || keysDown.has("KeyJ")) btns |= BTN_PASS;
    if (keysDown.has("KeyK")) btns |= BTN_THROUGH;
    if (keysDown.has("KeyU")) btns |= BTN_LOB;
    if (keysDown.has("KeyL") || keysDown.has("Enter")) btns |= BTN_SHOOT;
    if (keysDown.has("ShiftLeft") || keysDown.has("ShiftRight")) btns |= BTN_SPRINT;
    if (keysDown.has("KeyQ")) btns |= BTN_SWITCH;
    if (keysDown.has("KeyF")) btns |= BTN_JOCKEY;
    if (keysDown.has("KeyE")) btns |= BTN_TACKLE;
    if (keysDown.has("KeyC")) btns |= BTN_SLIDE;
    if (keysDown.has("KeyR")) btns |= BTN_CALL_RUN;

    const mx = Math.round(dx * 127);
    const my = Math.round(dy * 127);
    return { mx, my, ax: mx, ay: my, btns };
  }

  function sampleAwayLocalInput() {
    const gp2 = sampleGamepad(1);
    if (gp2 && (gp2.mx !== 0 || gp2.my !== 0 || gp2.btns !== 0)) {
      return gp2;
    }
    let dx = 0;
    let dy = 0;
    if (keysDown.has("ArrowLeft")) dx -= 1;
    if (keysDown.has("ArrowRight")) dx += 1;
    if (keysDown.has("ArrowUp")) dy -= 1;
    if (keysDown.has("ArrowDown")) dy += 1;
    if (dx !== 0 && dy !== 0) {
      dx *= Math.SQRT1_2;
      dy *= Math.SQRT1_2;
    }
    let btns = 0;
    if (keysDown.has("Numpad1") || keysDown.has("Period")) btns |= BTN_PASS;
    if (keysDown.has("Numpad2") || keysDown.has("Slash")) btns |= BTN_SHOOT;
    if (keysDown.has("Numpad3")) btns |= BTN_THROUGH;
    if (keysDown.has("RightControl") || keysDown.has("ControlRight")) btns |= BTN_SPRINT;
    if (keysDown.has("Comma")) btns |= BTN_SWITCH | BTN_TACKLE;

    const mx = Math.round(dx * 127);
    const my = Math.round(dy * 127);
    return { mx, my, ax: mx, ay: my, btns };
  }

  // --- Match Initialization (Exhibition & Official Host Session) ---

  function decodeModeControllers() {
    const val = modeSelect.value;
    if (val === "p1_cpu_easy") return { home: 0, away: 1 };
    if (val === "p1_cpu_hard") return { home: 0, away: 3 };
    if (val === "cpu_vs_cpu") return { home: 3, away: 3 };
    if (val === "local_2p") return { home: 0, away: 0 };
    return { home: 0, away: 2 };
  }

  function resetMatchUiForSnapshot(snapshot, contextLabel) {
    currentSnapshot = snapshot;
    ballTrail = [];
    homePossTicks = 1;
    awayPossTicks = 1;
    camX = 0;
    camY = 0;
    eventFeed.innerHTML = "";
    verifyBanner.hidden = true;
    goalBanner.hidden = true;

    sbContext.textContent = contextLabel;
    sbHomeCode.textContent = snapshot.home.short_name;
    sbAwayCode.textContent = snapshot.away.short_name;
    sbHomeSwatch.style.background = snapshot.home.primary_color;
    sbAwaySwatch.style.background = snapshot.away.primary_color;
    btnSubmitSession.hidden = !activeOfficialSession;
  }

  function startExhibitionMatch() {
    if (!wasm) return;
    activeOfficialSession = null;
    paused = false;
    btnPause.textContent = "Pause";

    const fmtCode = formatSelect.value === "11v11" ? 1 : 0;
    const halfTicks = parseInt(durationSelect.value, 10) || 3600;
    const ctrls = decodeModeControllers();
    const seedLo = (Date.now() & 0x7fffffff) >>> 0;

    const ok = wasm.sim_init_preset(fmtCode, seedLo, 0, halfTicks, ctrls.home, ctrls.away);
    if (!ok) {
      console.error("sim_init_preset error:", wasmReadText());
      return;
    }
    const snap = JSON.parse(wasmReadText());
    resetMatchUiForSnapshot(
      snap,
      `EXHIBITION · ${snap.format.name} (${snap.format.players_per_side}v${snap.format.players_per_side})`
    );
    renderWorldOverviewCardsHighlight();
  }

  async function launchOfficialFixtureSession(fixtureId, resultSource) {
    if (!wasm) return;
    const fmtName = formatSelect.value;
    const halfTicks = parseInt(durationSelect.value, 10) || 3600;
    const ctrls = decodeModeControllers();

    const ctrlObj = (code) => {
      if (code === 0) return { kind: "Human" };
      if (code === 1) return { kind: "Cpu", difficulty: "Easy" };
      if (code === 3) return { kind: "Cpu", difficulty: "Hard" };
      return { kind: "Cpu", difficulty: "Normal" };
    };

    const awayCtrl =
      resultSource === "played_vs_friend" && modeSelect.value === "local_2p"
        ? { kind: "Human" }
        : ctrlObj(ctrls.away === 0 ? 2 : ctrls.away);

    const res = await fetch(`/api/fixtures/${encodeURIComponent(fixtureId)}/sessions`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        actor_id: "part-alex",
        format_name: fmtName,
        half_duration_ticks: halfTicks,
        home_controller: ctrlObj(ctrls.home),
        away_controller: awayCtrl,
      }),
    });
    if (!res.ok) {
      const err = await res.json().catch(() => ({}));
      alert(`Could not open session: ${err.detail || res.statusText}`);
      return;
    }

    const payload = await res.json();
    const snapJson = JSON.stringify(payload.snapshot);
    wasmWriteInputText(snapJson);
    const ok = wasm.sim_init_from_snapshot_json();
    if (!ok) {
      alert(`WASM snapshot load failed: ${wasmReadText()}`);
      return;
    }

    activeOfficialSession = {
      sessionId: payload.session_id,
      fixtureId: payload.fixture_id,
      submitting: false,
      completed: false,
    };
    paused = false;
    btnPause.textContent = "Pause";
    resetMatchUiForSnapshot(
      payload.snapshot,
      `OFFICIAL R${payload.snapshot.fixture_id.includes("r3") ? "3" : "4"} · ${payload.snapshot.format.name}`
    );
    await refreshWorldOverview();
    window.scrollTo({ top: 0, behavior: "smooth" });
  }

  async function submitOfficialSessionResult() {
    if (!wasm || !activeOfficialSession || activeOfficialSession.submitting || activeOfficialSession.completed) {
      return;
    }
    // Ensure match has reached FullTime (if user clicked Verify early, fast-forward remaining ticks!)
    wasm.sim_export_summary_json();
    let summary = JSON.parse(wasmReadText());
    if (!summary.finished) {
      wasm.sim_auto_step(60000);
      wasm.sim_export_summary_json();
      summary = JSON.parse(wasmReadText());
    }

    activeOfficialSession.submitting = true;
    btnSubmitSession.textContent = "Verifying via psfc-replay…";

    wasm.sim_export_input_log_json();
    const log = JSON.parse(wasmReadText());

    try {
      const res = await fetch(
        `/api/sessions/${encodeURIComponent(activeOfficialSession.sessionId)}/complete`,
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            actor_id: "part-alex",
            frames_hex: log.frames_hex,
            claimed_final_hash_hex: log.claimed_final_hash_hex,
            claimed_score: log.claimed_score,
            hash_checkpoints: log.hash_checkpoints,
          }),
        }
      );
      const data = await res.json();
      if (!res.ok) {
        throw new Error(data.detail || "Replay verification failed");
      }

      activeOfficialSession.completed = true;
      btnSubmitSession.hidden = true;
      const rep = data.replay_report;
      verifyBanner.hidden = false;
      verifyBanner.innerHTML = `
        <strong>psfc-replay Authoritative Verification Passed (${rep ? rep.elapsed_ms : 0} ms)</strong><br/>
        Score Accepted: <strong>${currentSnapshot.home.short_name} ${data.accepted_result.home_goals} - ${data.accepted_result.away_goals} ${currentSnapshot.away.short_name}</strong>
        · Final Hash <code>#${data.accepted_result.final_hash_hex}</code> · Standings &amp; Earned Stats Updated!
      `;
      await refreshWorldOverview();
    } catch (err) {
      verifyBanner.hidden = false;
      verifyBanner.innerHTML = `<strong>Verification Error:</strong> ${err.message}`;
    } finally {
      activeOfficialSession.submitting = false;
      btnSubmitSession.textContent = "Verify & Record Result";
    }
  }

  // --- 3D Broadcast Camera Projection & Pitch Rendering ---

  function makeProjector(W, H, pitchLenM, pitchWidM) {
    // Smooth broadcast camera sitting on the south touchline elevated, looking inward & slightly tilted
    const zoomScale = pitchLenM > 80 ? 0.62 : 1.0;
    const camDistY = (pitchWidM * 0.92 + 17.0) / zoomScale;
    const camHeightZ = (23.5) / zoomScale;
    const lookY = camY * 0.25;
    const ry = -lookY + camDistY; // camera at (camX, lookY - camDistY, camHeightZ)? Let's use clean explicit basis!

    const eyeX = camX;
    const eyeY = camY * 0.28 + (pitchWidM * 0.86 + 15.5); // South side (+Y is near touchline, -Y is far touchline)
    const eyeZ = pitchLenM > 80 ? 36.0 : 22.5;

    const targetX = camX;
    const targetY = camY * 0.18 - 1.5;
    const targetZ = 0.0;

    // Forward vector F = normalize(target - eye)
    let fx = targetX - eyeX;
    let fy = targetY - eyeY;
    let fz = targetZ - eyeZ;
    const fl = Math.hypot(fx, fy, fz) || 1;
    fx /= fl;
    fy /= fl;
    fz /= fl;

    // World up = (0, 0, 1). Right R = normalize(F x Up) = normalize((fy, -fx, 0))
    let rx = fy;
    let ry2 = -fx;
    let rz = 0;
    const rl = Math.hypot(rx, ry2, rz) || 1;
    rx /= rl;
    ry2 /= rl;

    // Camera Up U = R x F
    const ux = ry2 * fz - rz * fy;
    const uy = rz * fx - rx * fz;
    const uz = rx * fy - ry2 * fx;

    const focal = H * 1.18;

    return function project(wx, wy, wz) {
      const dx = wx - eyeX;
      const dy = wy - eyeY;
      const dz = wz - eyeZ;

      const cx = dx * rx + dy * ry2 + dz * rz;
      const cy = dx * ux + dy * uy + dz * uz;
      const cz = dx * fx + dy * fy + dz * fz;
      const depth = Math.max(2.0, cz);
      const scale = focal / depth;

      return {
        x: W * 0.5 + cx * scale,
        y: H * 0.52 - cy * scale,
        s: scale,
      };
    };
  }

  function poly3D(project, pts, fillStyle, strokeStyle, lineWidth = 1.5) {
    ctx.beginPath();
    for (let i = 0; i < pts.length; i++) {
      const p = project(pts[i][0], pts[i][1], pts[i][2] || 0);
      if (i === 0) ctx.moveTo(p.x, p.y);
      else ctx.lineTo(p.x, p.y);
    }
    if (fillStyle) {
      ctx.closePath();
      ctx.fillStyle = fillStyle;
      ctx.fill();
    }
    if (strokeStyle) {
      ctx.strokeStyle = strokeStyle;
      ctx.lineWidth = lineWidth;
      ctx.stroke();
    }
  }

  function line3D(project, x1, y1, z1, x2, y2, z2, color, width = 1.8) {
    const a = project(x1, y1, z1);
    const b = project(x2, y2, z2);
    ctx.beginPath();
    ctx.moveTo(a.x, a.y);
    ctx.lineTo(b.x, b.y);
    ctx.strokeStyle = color;
    ctx.lineWidth = width;
    ctx.stroke();
  }

  function drawGoal3D(project, goalX, sign, goalHalfW, goalH) {
    const depth = sign * 1.65;
    const backX = goalX + depth;

    // Net translucent side & back panels
    poly3D(
      project,
      [
        [goalX, -goalHalfW, 0],
        [backX, -goalHalfW, 0],
        [backX, -goalHalfW, goalH],
        [goalX, -goalHalfW, goalH],
      ],
      "rgba(226, 232, 240, 0.14)",
      "rgba(226, 232, 240, 0.32)",
      1
    );
    poly3D(
      project,
      [
        [goalX, goalHalfW, 0],
        [backX, goalHalfW, 0],
        [backX, goalHalfW, goalH],
        [goalX, goalHalfW, goalH],
      ],
      "rgba(226, 232, 240, 0.14)",
      "rgba(226, 232, 240, 0.32)",
      1
    );
    poly3D(
      project,
      [
        [backX, -goalHalfW, 0],
        [backX, goalHalfW, 0],
        [backX, goalHalfW, goalH],
        [backX, -goalHalfW, goalH],
      ],
      "rgba(226, 232, 240, 0.12)",
      "rgba(226, 232, 240, 0.28)",
      1
    );
    // Roof net
    poly3D(
      project,
      [
        [goalX, -goalHalfW, goalH],
        [backX, -goalHalfW, goalH],
        [backX, goalHalfW, goalH],
        [goalX, goalHalfW, goalH],
      ],
      "rgba(226, 232, 240, 0.10)",
      null
    );

    // Crisp white posts & crossbar
    line3D(project, goalX, -goalHalfW, 0, goalX, -goalHalfW, goalH, "#ffffff", 3.4);
    line3D(project, goalX, goalHalfW, 0, goalX, goalHalfW, goalH, "#ffffff", 3.4);
    line3D(project, goalX, -goalHalfW, goalH, goalX, goalHalfW, goalH, "#ffffff", 3.6);
  }

  function drawScene(packed) {
    const W = canvas.width;
    const H = canvas.height;

    // Stadium night backdrop gradient
    const sky = ctx.createLinearGradient(0, 0, 0, H);
    sky.addColorStop(0, "#060b12");
    sky.addColorStop(0.32, "#0b1814");
    sky.addColorStop(1, "#091710");
    ctx.fillStyle = sky;
    ctx.fillRect(0, 0, W, H);

    if (!packed || !currentSnapshot) return;

    const fmt = currentSnapshot.format;
    const pitchLen = fmt.pitch_length_mm / 1000;
    const pitchWid = fmt.pitch_width_mm / 1000;
    const halfL = pitchLen / 2;
    const halfW = pitchWid / 2;
    const boxD = fmt.penalty_box_depth_mm / 1000;
    const boxHW = fmt.penalty_box_width_mm / 2000;
    const goalHW = fmt.goal_width_mm / 2000;
    const goalH = fmt.goal_height_mm / 1000;
    const circleR = fmt.center_circle_radius_mm / 1000;

    const ballX = packed[9] / 1000;
    const ballY = packed[10] / 1000;
    const ballZ = packed[11] / 1000;

    // Smooth camera pan clamped so goal mouths remain legible
    const maxPanX = halfL * 0.44;
    const targetCamX = Math.max(-maxPanX, Math.min(maxPanX, ballX * 0.62));
    camX += (targetCamX - camX) * 0.1;
    camY += (ballY - camY) * 0.08;

    const project = makeProjector(W, H, pitchLen, pitchWid);

    // Outer stadium apron
    poly3D(
      project,
      [
        [-halfL - 5, -halfW - 4.5, 0],
        [halfL + 5, -halfW - 4.5, 0],
        [halfL + 5, halfW + 4.5, 0],
        [-halfL - 5, halfW + 4.5, 0],
      ],
      "#0d2318",
      "#1b3b2b",
      2
    );

    // Mowed alternating grass stripes
    const stripeCount = fmt.players_per_side <= 6 ? 10 : 16;
    const stripeW = pitchLen / stripeCount;
    for (let i = 0; i < stripeCount; i++) {
      const x0 = -halfL + i * stripeW;
      const x1 = x0 + stripeW;
      const fill = i % 2 === 0 ? "#155e38" : "#186b40";
      poly3D(
        project,
        [
          [x0, -halfW, 0],
          [x1, -halfW, 0],
          [x1, halfW, 0],
          [x0, halfW, 0],
        ],
        fill,
        null
      );
    }

    // Advertising hoarding strip along far touchline
    poly3D(
      project,
      [
        [-halfL - 1.5, -halfW - 1.6, 0],
        [halfL + 1.5, -halfW - 1.6, 0],
        [halfL + 1.5, -halfW - 1.6, 0.95],
        [-halfL - 1.5, -halfW - 1.6, 0.95],
      ],
      "#101c2b",
      "rgba(56, 189, 248, 0.4)",
      1.2
    );

    // Crisp white chalk pitch markings
    const lineCol = "rgba(245, 250, 255, 0.84)";
    poly3D(
      project,
      [
        [-halfL, -halfW, 0.01],
        [halfL, -halfW, 0.01],
        [halfL, halfW, 0.01],
        [-halfL, halfW, 0.01],
      ],
      null,
      lineCol,
      2.1
    );
    // Halfway line
    line3D(project, 0, -halfW, 0.01, 0, halfW, 0.01, lineCol, 1.9);

    // Center circle
    ctx.beginPath();
    for (let s = 0; s <= 40; s++) {
      const a = (s / 40) * Math.PI * 2;
      const p = project(Math.cos(a) * circleR, Math.sin(a) * circleR, 0.01);
      if (s === 0) ctx.moveTo(p.x, p.y);
      else ctx.lineTo(p.x, p.y);
    }
    ctx.strokeStyle = lineCol;
    ctx.lineWidth = 1.8;
    ctx.stroke();

    // Penalty boxes (left & right)
    for (const sign of [-1, 1]) {
      const gx = sign * halfL;
      const ix = gx - sign * boxD;
      poly3D(
        project,
        [
          [gx, -boxHW, 0.01],
          [ix, -boxHW, 0.01],
          [ix, boxHW, 0.01],
          [gx, boxHW, 0.01],
        ],
        null,
        lineCol,
        1.8
      );
      // Penalty spot
      const penSpot = project(gx - sign * ((boxD * 2) / 3), 0, 0.01);
      ctx.fillStyle = "#ffffff";
      ctx.beginPath();
      ctx.arc(penSpot.x, penSpot.y, 2.6, 0, Math.PI * 2);
      ctx.fill();
    }

    // Unpack players & depth-sort (`wy` ascending: far `-Y` drawn first, near `+Y` drawn last)
    const pps = packed[6];
    const homeCtrl = packed[7];
    const awayCtrl = packed[8];
    const drawables = [];

    for (let n = 0; n < pps * 2; n++) {
      const b = 18 + n * 10;
      const side = packed[b + 0];
      const idx = packed[b + 1];
      const shirt = packed[b + 2];
      const role = packed[b + 3];
      const x = packed[b + 4] / 1000;
      const y = packed[b + 5] / 1000;
      const vx = packed[b + 6] / 1000;
      const vy = packed[b + 7] / 1000;
      const facingRad = (packed[b + 8] * Math.PI * 2) / 65536;
      const flags = packed[b + 9];
      const action = flags & 0xf;
      const stamina = ((flags >> 4) & 0x7ff) / 1000;
      const charge = ((flags >> 15) & 0x7ff) / 1000;
      const sentOff = ((flags >> 26) & 1) === 1;
      if (sentOff) continue;

      drawables.push({
        kind: "player",
        y,
        side,
        idx,
        shirt,
        role,
        x,
        vx,
        vy,
        facingRad,
        action,
        stamina,
        charge,
        isControlled: side === 0 ? idx === homeCtrl : idx === awayCtrl,
      });
    }

    drawables.push({ kind: "ball", y: ballY, x: ballX, z: ballZ });
    drawables.sort((a, b) => a.y - b.y);

    // Draw goals behind / around entities (draw far parts first, then entities)
    drawGoal3D(project, -halfL, -1, goalHW, goalH);
    drawGoal3D(project, halfL, 1, goalHW, goalH);

    // Ball velocity trail
    if (ballTrail.length > 1) {
      ctx.beginPath();
      for (let i = 0; i < ballTrail.length; i++) {
        const tp = project(ballTrail[i].x, ballTrail[i].y, ballTrail[i].z);
        if (i === 0) ctx.moveTo(tp.x, tp.y);
        else ctx.lineTo(tp.x, tp.y);
      }
      ctx.strokeStyle = "rgba(255, 255, 255, 0.28)";
      ctx.lineWidth = 2.2;
      ctx.stroke();
    }

    // Draw depth-sorted players & 3D ball
    for (const item of drawables) {
      if (item.kind === "ball") {
        const gnd = project(item.x, item.y, 0);
        const air = project(item.x, item.y, item.z);
        const r = Math.max(4.0, air.s * 0.22);

        // Ground shadow
        ctx.fillStyle = "rgba(0, 0, 0, 0.45)";
        ctx.beginPath();
        ctx.ellipse(gnd.x, gnd.y, r * 1.1, r * 0.55, 0, 0, Math.PI * 2);
        ctx.fill();

        // Soccer ball sphere
        const grad = ctx.createRadialGradient(
          air.x - r * 0.3,
          air.y - r * 0.3,
          r * 0.15,
          air.x,
          air.y,
          r
        );
        grad.addColorStop(0, "#ffffff");
        grad.addColorStop(0.75, "#e2e8f0");
        grad.addColorStop(1, "#64748b");
        ctx.fillStyle = grad;
        ctx.beginPath();
        ctx.arc(air.x, air.y, r, 0, Math.PI * 2);
        ctx.fill();
        ctx.strokeStyle = "#0f172a";
        ctx.lineWidth = 1.1;
        ctx.stroke();
        continue;
      }

      // Player rendering
      const foot = project(item.x, item.y, 0);
      const head = project(item.x, item.y, item.action === 4 ? 0.65 : 1.75);
      const pxRadius = Math.max(4.5, foot.s * 0.38);

      // Controlled-player classic arcade ring + facing chevron!
      if (item.isControlled) {
        const ringColor = item.side === 0 ? "#fbbf24" : "#38bdf8";
        ctx.strokeStyle = ringColor;
        ctx.lineWidth = 2.4;
        ctx.beginPath();
        ctx.ellipse(foot.x, foot.y, pxRadius * 1.9, pxRadius * 0.95, 0, 0, Math.PI * 2);
        ctx.stroke();

        // Facing directional pointer on turf
        const fx = item.x + Math.cos(item.facingRad) * 1.05;
        const fy = item.y + Math.sin(item.facingRad) * 1.05;
        const fp = project(fx, fy, 0.02);
        ctx.fillStyle = ringColor;
        ctx.beginPath();
        ctx.arc(fp.x, fp.y, 3.0, 0, Math.PI * 2);
        ctx.fill();
      } else if (item.role === 0) {
        // Subtle Goalkeeper glove/box indicator ring
        ctx.strokeStyle = "rgba(16, 185, 129, 0.55)";
        ctx.lineWidth = 1.4;
        ctx.beginPath();
        ctx.ellipse(foot.x, foot.y, pxRadius * 1.45, pxRadius * 0.7, 0, 0, Math.PI * 2);
        ctx.stroke();
      }

      // Ground shadow
      ctx.fillStyle = "rgba(0, 0, 0, 0.38)";
      ctx.beginPath();
      ctx.ellipse(foot.x, foot.y, pxRadius * 1.15, pxRadius * 0.55, 0, 0, Math.PI * 2);
      ctx.fill();

      // Kit colors (Goalkeepers wear high-visibility emerald/lime & gold!)
      let jerseyColor =
        item.side === 0
          ? currentSnapshot.home.primary_color
          : currentSnapshot.away.primary_color;
      let trimColor =
        item.side === 0
          ? currentSnapshot.home.secondary_color
          : currentSnapshot.away.secondary_color;

      // Avoid kit clash if both clubs wear red (e.g. MUN vs ARS or MUN vs LIV!)
      if (
        item.side === 1 &&
        currentSnapshot.home.primary_color.startsWith("#D") &&
        (currentSnapshot.away.primary_color.startsWith("#E") ||
          currentSnapshot.away.primary_color.startsWith("#C"))
      ) {
        jerseyColor = "#facc15";
        trimColor = "#0f172a";
      }
      if (item.role === 0) {
        jerseyColor = item.side === 0 ? "#10b981" : "#a855f7";
        trimColor = "#ffffff";
      }

      // Body capsule (tilts horizontally if Diving or Sliding!)
      const bodyH = Math.max(10, foot.y - head.y);
      ctx.fillStyle = jerseyColor;
      ctx.strokeStyle = trimColor;
      ctx.lineWidth = 1.5;

      ctx.beginPath();
      if (item.action === 4 || item.action === 3) {
        // Horizontal dive / slide lunge!
        ctx.ellipse(foot.x, foot.y - pxRadius * 0.8, pxRadius * 1.9, pxRadius * 0.85, 0, 0, Math.PI * 2);
      } else {
        ctx.roundRect(
          foot.x - pxRadius * 0.85,
          head.y + bodyH * 0.24,
          pxRadius * 1.7,
          bodyH * 0.72,
          pxRadius * 0.55
        );
      }
      ctx.fill();
      ctx.stroke();

      // Head
      ctx.fillStyle = "#f1c27d";
      ctx.beginPath();
      ctx.arc(head.x, head.y + bodyH * 0.16, Math.max(3.2, pxRadius * 0.58), 0, Math.PI * 2);
      ctx.fill();

      // Shirt number
      ctx.fillStyle = item.side === 1 && jerseyColor === "#facc15" ? "#0f172a" : "#ffffff";
      ctx.font = `700 ${Math.max(9, Math.round(pxRadius * 1.05))}px Inter, sans-serif`;
      ctx.textAlign = "center";
      ctx.fillText(String(item.shirt), foot.x, head.y + bodyH * 0.68);

      // Overhead Shot / Pass Power Charge Bar!
      if (item.charge > 0.02) {
        const bw = 32;
        const bh = 5;
        const bx = head.x - bw / 2;
        const by = head.y - 11;
        ctx.fillStyle = "rgba(9, 14, 21, 0.85)";
        ctx.fillRect(bx - 1, by - 1, bw + 2, bh + 2);
        ctx.fillStyle = item.charge > 0.78 ? "#ef4444" : "#fbbf24";
        ctx.fillRect(bx, by, bw * Math.min(1, item.charge), bh);
      }
    }

    // Classic Bottom-Center 2D Tactical Radar
    drawTacticalRadar(W, H, packed, halfL, halfW);
  }

  function drawTacticalRadar(W, H, packed, halfL, halfW) {
    const rw = 186;
    const rh = 112;
    const rx = (W - rw) * 0.5;
    const ry = H - rh - 10;

    ctx.fillStyle = "rgba(9, 14, 21, 0.76)";
    ctx.strokeStyle = "rgba(255, 255, 255, 0.25)";
    ctx.lineWidth = 1.2;
    ctx.beginPath();
    ctx.roundRect(rx, ry, rw, rh, 6);
    ctx.fill();
    ctx.stroke();

    // Halfway line on radar
    ctx.beginPath();
    ctx.moveTo(rx + rw * 0.5, ry + 4);
    ctx.lineTo(rx + rw * 0.5, ry + rh - 4);
    ctx.strokeStyle = "rgba(255, 255, 255, 0.18)";
    ctx.stroke();

    const pps = packed[6];
    const mapX = (wx) => rx + ((wx + halfL) / (halfL * 2)) * rw;
    const mapY = (wy) => ry + ((wy + halfW) / (halfW * 2)) * rh;

    for (let n = 0; n < pps * 2; n++) {
      const b = 18 + n * 10;
      const side = packed[b];
      const role = packed[b + 3];
      const sentOff = ((packed[b + 9] >> 26) & 1) === 1;
      if (sentOff) continue;
      const px = mapX(packed[b + 4] / 1000);
      const py = mapY(packed[b + 5] / 1000);

      ctx.fillStyle =
        role === 0 ? "#10b981" : side === 0 ? "#ef4444" : "#38bdf8";
      ctx.beginPath();
      ctx.arc(px, py, role === 0 ? 3.5 : 3.0, 0, Math.PI * 2);
      ctx.fill();
    }

    // Radar ball dot
    const bx = mapX(packed[9] / 1000);
    const by = mapY(packed[10] / 1000);
    ctx.fillStyle = "#ffffff";
    ctx.beginPath();
    ctx.arc(bx, by, 3.3, 0, Math.PI * 2);
    ctx.fill();
  }

  // --- Main 60 Hz Simulation & Render Loop ---

  function drainAndRenderMatchEvents() {
    const len = wasm.sim_drain_events_json();
    if (len <= 2) return;
    const events = JSON.parse(wasmReadText());
    for (const ev of events) {
      const li = document.createElement("li");
      li.innerHTML = `<span class="ev-min">${ev.minute}'</span><span>${ev.detail}</span>`;
      eventFeed.prepend(li);
      while (eventFeed.children.length > 18) {
        eventFeed.removeChild(eventFeed.lastChild);
      }

      if (ev.event_type === "Goal") {
        goalBannerTitle.textContent = "GOAL!";
        goalBannerSub.textContent = ev.detail;
        goalBanner.hidden = false;
        goalBannerTicks = 130;
      } else if (ev.event_type === "FullTime" && activeOfficialSession && !activeOfficialSession.completed) {
        submitOfficialSessionResult();
      }
    }
  }

  function frameLoop() {
    if (wasm && currentSnapshot) {
      if (!paused) {
        const is2P = modeSelect.value === "local_2p";
        const hIn = sampleHomeInput(!is2P);
        const aIn = is2P ? sampleAwayLocalInput() : { mx: 0, my: 0, ax: 0, ay: 0, btns: 0 };

        wasm.sim_tick(
          hIn.mx,
          hIn.my,
          hIn.ax,
          hIn.ay,
          hIn.btns,
          aIn.mx,
          aIn.my,
          aIn.ax,
          aIn.ay,
          aIn.btns
        );
      }

      const packed = readPackedState();
      if (packed) {
        const tick = packed[0];
        const clockTicks = packed[1];
        const half = packed[2];
        const phase = packed[3];
        const hScore = packed[4];
        const aScore = packed[5];
        const ownerSide = packed[15];

        if (!paused) {
          if (ownerSide === 0) homePossTicks++;
          else if (ownerSide === 1) awayPossTicks++;

          const bx = packed[9] / 1000;
          const by = packed[10] / 1000;
          const bz = packed[11] / 1000;
          ballTrail.push({ x: bx, y: by, z: bz });
          if (ballTrail.length > 10) ballTrail.shift();
        }

        // Update Scorebug & Clock
        sbScore.textContent = `${hScore} - ${aScore}`;
        sbHalf.textContent = phase === 12 ? "FT" : `${half}H`;
        const totalSec = Math.floor(clockTicks / 60);
        const mm = String(Math.floor(totalSec / 60)).padStart(2, "0");
        const ss = String(totalSec % 60).padStart(2, "0");
        sbTime.textContent = `${mm}:${ss}`;
        sbPhase.textContent = PHASE_LABELS[phase] || "PLAY";

        if (tick % 12 === 0) {
          wasm.sim_state_hash_hex();
          hashPill.textContent = `State Hash: #${wasmReadText()}`;
          const hPct = Math.round((homePossTicks / (homePossTicks + awayPossTicks)) * 100);
          possessionBadge.textContent = `Possession ${hPct}% - ${100 - hPct}%`;
        }

        if (packed[17] > 0) {
          drainAndRenderMatchEvents();
        }

        if (goalBannerTicks > 0) {
          goalBannerTicks--;
          if (goalBannerTicks === 0) goalBanner.hidden = true;
        }

        drawScene(packed);
      }
    }
    requestAnimationFrame(frameLoop);
  }

  // --- Parallel Season World Host UI Sync ---

  let latestOverview = null;

  function renderWorldOverviewCardsHighlight() {
    if (!latestOverview) return;
    const cards = document.querySelectorAll(".fixture-card");
    cards.forEach((el) => {
      const fid = el.getAttribute("data-fixture-id");
      el.classList.toggle(
        "active-session",
        Boolean(activeOfficialSession && activeOfficialSession.fixtureId === fid)
      );
    });
  }

  async function refreshWorldOverview() {
    const res = await fetch(`/api/worlds/${DEFAULT_WORLD_ID}/overview`);
    if (!res.ok) return;
    const data = await res.json();
    latestOverview = data;

    document.getElementById("world-name").textContent = data.world.name;
    const repLabel = data.assignments
      .map((a) => `${a.club_id.toUpperCase()} (${a.participant_name.split(" ")[0]})`)
      .join(", ");
    document.getElementById("world-meta").textContent =
      `${data.world.season_label} · Default ${data.world.default_match_format} · Active: ${repLabel}`;

    // 1. Render Fixtures
    const fxContainer = document.getElementById("fixture-list");
    fxContainer.innerHTML = "";
    for (const fx of data.fixtures) {
      const srcClass =
        fx.result_source === "played_vs_friend"
          ? "src-friend"
          : fx.result_source === "played_vs_cpu"
            ? "src-cpu"
            : "src-imported";

      const isPlayable =
        fx.state !== "accepted" &&
        (fx.result_source === "played_vs_friend" || fx.result_source === "played_vs_cpu");

      const scoreText =
        fx.state === "accepted"
          ? `${fx.accepted_home_goals} - ${fx.accepted_away_goals}`
          : fx.state === "in_session"
            ? "IN SESSION"
            : "vs";

      const card = document.createElement("div");
      card.className = "fixture-card";
      card.setAttribute("data-fixture-id", fx.id);
      if (activeOfficialSession && activeOfficialSession.fixtureId === fx.id) {
        card.classList.add("active-session");
      }

      const realNote =
        fx.real_home_goals !== null && fx.real_away_goals !== null
          ? `Real Ref: ${fx.real_home_goals}-${fx.real_away_goals}`
          : "";

      card.innerHTML = `
        <div class="fx-top">
          <span>Round ${fx.round} · ${fx.state.toUpperCase()}</span>
          <span class="fx-source ${srcClass}">${fx.result_source}</span>
        </div>
        <div class="fx-mid">
          <span>${fx.home_club.name}</span>
          <span class="fx-score-box">${scoreText}</span>
          <span>${fx.away_club.name}</span>
        </div>
        <div class="fx-bottom">
          <span>${realNote}${fx.accepted_hash_hex ? ` · Hash #${fx.accepted_hash_hex.slice(0, 8)}` : ""}</span>
          ${
            isPlayable
              ? `<button class="btn btn-primary btn-play-fx" data-fx="${fx.id}" data-src="${fx.result_source}">
                   ${fx.state === "in_session" ? "Resume 5v5 Session" : "Play Official 5v5"}
                 </button>`
              : `<span>${fx.accepted_source === "imported" ? "Inherited / Imported" : "Verified"}</span>`
          }
        </div>
      `;
      fxContainer.appendChild(card);
    }

    fxContainer.querySelectorAll(".btn-play-fx").forEach((btn) => {
      btn.addEventListener("click", () => {
        launchOfficialFixtureSession(
          btn.getAttribute("data-fx"),
          btn.getAttribute("data-src")
        );
      });
    });

    // 2. Render Standings
    const stBody = document.getElementById("standings-tbody");
    stBody.innerHTML = "";
    for (const r of data.standings) {
      const delta = r.delta_vs_real;
      const deltaStr =
        delta > 0 ? `<span class="pos-delta">+${delta}</span>` : delta < 0 ? `<span class="neg-delta">${delta}</span>` : "0";
      const tr = document.createElement("tr");
      tr.innerHTML = `
        <td>${r.position}</td>
        <td><strong>${r.short_name}</strong> ${r.represented ? "🎮" : ""}</td>
        <td>${r.played}</td>
        <td>${r.won}-${r.drawn}-${r.lost}</td>
        <td>${r.gd > 0 ? `+${r.gd}` : r.gd}</td>
        <td>${r.inherited_points}</td>
        <td>${r.earned_points}</td>
        <td><strong>${r.points}</strong></td>
        <td>${deltaStr}</td>
      `;
      stBody.appendChild(tr);
    }

    // 3. Render Inherited / Earned / Combined Player Stats
    const plBody = document.getElementById("players-tbody");
    plBody.innerHTML = "";
    for (const p of data.players) {
      const tr = document.createElement("tr");
      tr.innerHTML = `
        <td><strong>${p.name}</strong></td>
        <td>${p.club_short}</td>
        <td>${p.inherited.goals}G / ${p.inherited.assists}A</td>
        <td>${p.earned.goals}G / ${p.earned.assists}A</td>
        <td><strong>${p.combined.goals}G / ${p.combined.assists}A</strong></td>
        <td>${p.combined.saves}</td>
      `;
      plBody.appendChild(tr);
    }

    // 4. Render Ledger Events
    document.getElementById("event-count").textContent = String(data.total_events);
    const ledList = document.getElementById("ledger-list");
    ledList.innerHTML = "";
    for (const ev of data.recent_events) {
      const li = document.createElement("li");
      if (ev.type === "import_parked") li.classList.add("ev-parked");
      li.innerHTML = `<span class="ledger-seq">#${ev.seq}</span><strong>${ev.type}</strong> · <code>${ev.aggregate_id}</code>`;
      ledList.appendChild(li);
    }
  }

  // --- UI Event Wiring ---

  document.querySelectorAll(".tab").forEach((tabBtn) => {
    tabBtn.addEventListener("click", () => {
      document.querySelectorAll(".tab").forEach((b) => b.classList.remove("active"));
      document.querySelectorAll(".tab-pane").forEach((p) => (p.hidden = true));
      tabBtn.classList.add("active");
      const target = document.getElementById(`tab-${tabBtn.getAttribute("data-tab")}`);
      if (target) target.hidden = false;
    });
  });

  btnExhibition.addEventListener("click", startExhibitionMatch);
  formatSelect.addEventListener("change", () => {
    if (!activeOfficialSession) startExhibitionMatch();
  });
  modeSelect.addEventListener("change", () => {
    if (!activeOfficialSession) startExhibitionMatch();
  });

  btnPause.addEventListener("click", () => {
    paused = !paused;
    btnPause.textContent = paused ? "Resume" : "Pause";
  });

  btnSimRest.addEventListener("click", async () => {
    if (!wasm) return;
    wasm.sim_auto_step(60000);
    drainAndRenderMatchEvents();
    if (activeOfficialSession && !activeOfficialSession.completed) {
      await submitOfficialSessionResult();
    }
  });

  btnSubmitSession.addEventListener("click", submitOfficialSessionResult);

  document.getElementById("btn-test-gate").addEventListener("click", async () => {
    const res = await fetch(`/api/worlds/${DEFAULT_WORLD_ID}/ingest/result`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        fixture_id: "fix-r3-mun-ars",
        home_goals: 1,
        away_goals: 4,
        events: [{ event_type: "Goal", footballer_id: "ars-fwd-1", minute: 12 }],
        provider: "external_live_wire",
      }),
    });
    const out = await res.json();
    const box = document.getElementById("gate-result-msg");
    box.hidden = false;
    box.innerHTML = `Proposal <code>${out.proposal_id}</code> → Decision: <strong>${out.decision.toUpperCase()}</strong> (Reason: <code>${out.reason}</code>). Parallel World history preserved!`;
    await refreshWorldOverview();
  });

  document.getElementById("btn-reset-world").addEventListener("click", async () => {
    await fetch(`/api/worlds/${DEFAULT_WORLD_ID}/reset-demo`, { method: "POST" });
    activeOfficialSession = null;
    startExhibitionMatch();
    await refreshWorldOverview();
  });

  // --- Boot WASM & World Overview ---

  async function boot() {
    try {
      const resp = await fetch("/wasm/psfc_wasm.wasm");
      if (!resp.ok) throw new Error(`HTTP ${resp.status}`);
      const bytes = await resp.arrayBuffer();
      const { instance } = await WebAssembly.instantiate(bytes, {});
      wasm = instance.exports;
      wasmPill.textContent = "WASM Sim: 60Hz Fixed-Point Ready";
      startExhibitionMatch();
      requestAnimationFrame(frameLoop);
    } catch (err) {
      wasmPill.textContent = `WASM Error: ${err.message}`;
      console.error(err);
    }
    await refreshWorldOverview();
  }

  boot();
})();
