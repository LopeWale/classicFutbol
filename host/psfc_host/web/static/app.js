import {
  ClassicFifa3DEngine,
  CAMERA_LABELS,
  WEATHER_LABELS,
  resolveMatchKits,
} from "./engine3d.js";

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
const BTN_SKILL = 1 << 10;

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

const CLUB_ORDER = [
  "arsenal",
  "brighton",
  "man-city",
  "bournemouth",
  "liverpool",
  "leeds",
  "everton",
  "chelsea",
  "tottenham",
  "brentford",
  "aston-villa",
  "newcastle",
  "coventry",
  "hull",
  "fulham",
  "ipswich",
  "nottm-forest",
  "sunderland",
  "man-utd",
  "crystal-palace",
];
const SIDE_MODES = [
  { id: "home_vs_cpu", label: "P1 HOME vs CPU" },
  { id: "away_vs_cpu", label: "CPU vs P1 AWAY" },
  { id: "local_2p", label: "2P LOCAL COUCH" },
  { id: "cpu_vs_cpu", label: "CPU vs CPU SPECTATE" },
];
const FORMAT_OPTIONS = [
  { id: "5v5", label: "5v5 ARENA (4 + GK)" },
  { id: "11v11", label: "11v11 FULL STADIUM" },
];
const DURATION_OPTIONS = [
  { ticks: 3600, label: "1 MIN HALVES" },
  { ticks: 7200, label: "2 MIN HALVES" },
  { ticks: 14400, label: "4 MIN HALVES" },
  { ticks: 1800, label: "30 SEC QUICK" },
];
const DIFF_OPTIONS = [
  { code: 1, name: "Easy", label: "AMATEUR" },
  { code: 2, name: "Normal", label: "PROFESSIONAL" },
  { code: 3, name: "Hard", label: "WORLD CLASS" },
];

// Normalized (x%, y%) coordinates on the vertical 2D tactical pitch board (bottom = own goal, top = opponent goal)
const FORMATION_COORDS_5V5 = {
  "1-2-1": [
    [50, 86], // GK
    [50, 66], // Pivot Defender
    [23, 44], // Left Wing
    [77, 44], // Right Wing
    [50, 21], // Striker
  ],
  "2-2": [
    [50, 86], // GK
    [28, 64], // Left Back
    [72, 64], // Right Back
    [28, 28], // Left Forward
    [72, 28], // Right Forward
  ],
  "1-1-2": [
    [50, 86], // GK
    [50, 68], // Sweeper
    [50, 47], // Playmaker
    [27, 22], // Left Striker
    [73, 22], // Right Striker
  ],
};

const DEFAULT_WORLD_ID = "world-prem-2026";
const DEFAULT_CUP_ID = "comp-facup-2026";
const REPLAY_CAPACITY = 360;

// --- Spatial Stereo WebAudio Console & Stadium Sound Engine ---

const GameAudio = (() => {
  let audioCtx = null;
  let enabled = true;
  let ambienceGain = null;
  let ambienceWanted = false;

  function ensureCtx() {
    if (!enabled) return null;
    if (!audioCtx) {
      const Ctor = window.AudioContext || window.webkitAudioContext;
      if (!Ctor) return null;
      audioCtx = new Ctor();
    }
    if (audioCtx.state === "suspended") {
      audioCtx.resume().catch(() => {});
    }
    if (ambienceWanted && !ambienceGain && audioCtx.state === "running") {
      startAmbienceBed(audioCtx);
    }
    return audioCtx;
  }

  function connectSpatial(ac, node, pan = 0) {
    if (ac.createStereoPanner) {
      const panner = ac.createStereoPanner();
      panner.pan.setValueAtTime(Math.max(-0.78, Math.min(0.78, pan)), ac.currentTime);
      node.connect(panner).connect(ac.destination);
    } else {
      node.connect(ac.destination);
    }
  }

  function startAmbienceBed(ac) {
    if (ambienceGain) return;
    const dur = 3.0;
    const bufSize = Math.floor(ac.sampleRate * dur);
    const buf = ac.createBuffer(1, bufSize, ac.sampleRate);
    const ch = buf.getChannelData(0);
    let last = 0;
    for (let i = 0; i < bufSize; i++) {
      const w = Math.random() * 2 - 1;
      last = (last + 0.03 * w) / 1.03;
      const seam = Math.sin((i / bufSize) * Math.PI);
      ch[i] = last * (0.7 + 0.3 * seam) * 2.2;
    }
    const src = ac.createBufferSource();
    src.buffer = buf;
    src.loop = true;
    const lp = ac.createBiquadFilter();
    lp.type = "bandpass";
    lp.frequency.setValueAtTime(420, ac.currentTime);
    lp.Q.setValueAtTime(0.65, ac.currentTime);
    ambienceGain = ac.createGain();
    ambienceGain.gain.setValueAtTime(enabled && ambienceWanted ? 0.028 : 0.0, ac.currentTime);
    src.connect(lp).connect(ambienceGain).connect(ac.destination);
    src.start();
  }

  function setMatchAmbience(active) {
    ambienceWanted = Boolean(active);
    if (audioCtx && ambienceGain) {
      ambienceGain.gain.setTargetAtTime(enabled && ambienceWanted ? 0.028 : 0.0, audioCtx.currentTime, 0.25);
    } else if (ambienceWanted && audioCtx && audioCtx.state === "running") {
      startAmbienceBed(audioCtx);
    }
  }

  function uiTick(freq = 520) {
    const ac = ensureCtx();
    if (!ac) return;
    const now = ac.currentTime;
    const osc = ac.createOscillator();
    const gain = ac.createGain();
    osc.type = "triangle";
    osc.frequency.setValueAtTime(freq, now);
    osc.frequency.exponentialRampToValueAtTime(freq * 1.25, now + 0.04);
    gain.gain.setValueAtTime(0.06, now);
    gain.gain.exponentialRampToValueAtTime(0.001, now + 0.045);
    osc.connect(gain).connect(ac.destination);
    osc.start(now);
    osc.stop(now + 0.05);
  }

  function uiConfirm() {
    uiTick(660);
    setTimeout(() => uiTick(880), 45);
  }

  function playKick(power = 0.65, pan = 0) {
    const ac = ensureCtx();
    if (!ac) return;
    const now = ac.currentTime;
    const osc = ac.createOscillator();
    const gain = ac.createGain();
    osc.type = "triangle";
    osc.frequency.setValueAtTime(142 + power * 52, now);
    osc.frequency.exponentialRampToValueAtTime(38, now + 0.095);
    gain.gain.setValueAtTime(0.21 * Math.min(1.25, power + 0.35), now);
    gain.gain.exponentialRampToValueAtTime(0.001, now + 0.1);
    osc.connect(gain);
    connectSpatial(ac, gain, pan);
    osc.start(now);
    osc.stop(now + 0.105);
  }

  function playWhistle(longBlast = false, pan = 0) {
    const ac = ensureCtx();
    if (!ac) return;
    const now = ac.currentTime;
    const dur = longBlast ? 0.42 : 0.18;
    for (const freq of [2780, 2945]) {
      const osc = ac.createOscillator();
      const gain = ac.createGain();
      osc.type = "sine";
      osc.frequency.setValueAtTime(freq, now);
      osc.frequency.linearRampToValueAtTime(freq - 55, now + dur);
      gain.gain.setValueAtTime(0.055, now);
      gain.gain.exponentialRampToValueAtTime(0.001, now + dur);
      osc.connect(gain);
      connectSpatial(ac, gain, pan * 0.4);
      osc.start(now);
      osc.stop(now + dur + 0.01);
    }
  }

  function playSaveSlap(pan = 0) {
    const ac = ensureCtx();
    if (!ac) return;
    const now = ac.currentTime;
    const osc = ac.createOscillator();
    const gain = ac.createGain();
    osc.type = "square";
    osc.frequency.setValueAtTime(320, now);
    osc.frequency.exponentialRampToValueAtTime(86, now + 0.08);
    gain.gain.setValueAtTime(0.115, now);
    gain.gain.exponentialRampToValueAtTime(0.001, now + 0.085);
    osc.connect(gain);
    connectSpatial(ac, gain, pan);
    osc.start(now);
    osc.stop(now + 0.09);
  }

  function playWoodworkClang(pan = 0) {
    const ac = ensureCtx();
    if (!ac) return;
    const now = ac.currentTime;
    for (const [freq, amp, dur] of [
      [780, 0.12, 0.26],
      [1290, 0.075, 0.19],
    ]) {
      const osc = ac.createOscillator();
      const gain = ac.createGain();
      osc.type = "triangle";
      osc.frequency.setValueAtTime(freq, now);
      osc.frequency.exponentialRampToValueAtTime(freq * 0.94, now + dur);
      gain.gain.setValueAtTime(amp, now);
      gain.gain.exponentialRampToValueAtTime(0.001, now + dur);
      osc.connect(gain);
      connectSpatial(ac, gain, pan);
      osc.start(now);
      osc.stop(now + dur + 0.01);
    }
  }

  function playNetSwish(pan = 0) {
    const ac = ensureCtx();
    if (!ac) return;
    const now = ac.currentTime;
    const dur = 0.22;
    const len = Math.floor(ac.sampleRate * dur);
    const buf = ac.createBuffer(1, len, ac.sampleRate);
    const ch = buf.getChannelData(0);
    for (let i = 0; i < len; i++) ch[i] = (Math.random() * 2 - 1) * (1 - i / len);
    const src = ac.createBufferSource();
    src.buffer = buf;
    const bp = ac.createBiquadFilter();
    bp.type = "bandpass";
    bp.frequency.setValueAtTime(2100, now);
    bp.Q.setValueAtTime(1.4, now);
    const gain = ac.createGain();
    gain.gain.setValueAtTime(0.14, now);
    gain.gain.exponentialRampToValueAtTime(0.001, now + dur);
    src.connect(bp).connect(gain);
    connectSpatial(ac, gain, pan);
    src.start(now);
  }

  function playGoalCrowdRoar(pan = 0) {
    const ac = ensureCtx();
    if (!ac) return;
    playNetSwish(pan);
    const now = ac.currentTime;
    const dur = 1.55;
    const bufSize = Math.floor(ac.sampleRate * dur);
    const buf = ac.createBuffer(1, bufSize, ac.sampleRate);
    const ch = buf.getChannelData(0);
    let last = 0;
    for (let i = 0; i < bufSize; i++) {
      const white = Math.random() * 2 - 1;
      last = (last + 0.06 * white) / 1.06;
      ch[i] = last * 3.1;
    }
    const src = ac.createBufferSource();
    src.buffer = buf;
    const filter = ac.createBiquadFilter();
    filter.type = "bandpass";
    filter.frequency.setValueAtTime(650, now);
    filter.Q.setValueAtTime(0.85, now);
    const gain = ac.createGain();
    gain.gain.setValueAtTime(0.01, now);
    gain.gain.linearRampToValueAtTime(0.25, now + 0.18);
    gain.gain.exponentialRampToValueAtTime(0.002, now + dur);
    src.connect(filter).connect(gain).connect(ac.destination);
    src.start(now);
  }

  function toggle() {
    enabled = !enabled;
    if (audioCtx && ambienceGain) {
      ambienceGain.gain.setTargetAtTime(enabled && ambienceWanted ? 0.028 : 0.0, audioCtx.currentTime, 0.15);
    }
    return enabled;
  }

  return {
    ensureCtx,
    setMatchAmbience,
    uiTick,
    uiConfirm,
    playKick,
    playWhistle,
    playSaveSlap,
    playWoodworkClang,
    playGoalCrowdRoar,
    toggle,
  };
})();

// --- Application State ---

const canvas = document.getElementById("pitch-canvas");
const radarCanvas = document.getElementById("radar-canvas");
const radarCtx = radarCanvas.getContext("2d");
const engine3d = new ClassicFifa3DEngine(canvas);

const enc = new TextEncoder();
const dec = new TextDecoder();

let wasm = null;
let currentScreen = "title"; // "title" | "menu" | "prematch" | "squad" | "season" | "controllers" | "match"
let previousScreen = "menu";
let focusedBladeIdx = 0;

let activeWorldId = DEFAULT_WORLD_ID;
let activeCompFilter = "all";
let latestOverview = null;
let currentSnapshot = null;
let isAttractMatch = true;
let matchSessionSeq = 0;
let attractRestartInFlight = false;

// Match setup selections (Pre-Match Screen)
let pmHomeClub = "man-utd";
let pmAwayClub = "arsenal";
let pmSideIdx = 0; // 0 = P1 Home vs CPU, 1 = CPU vs P1 Away, 2 = 2P Couch, 3 = CPU Spectate
let pmFmtIdx = 0; // 0 = 5v5, 1 = 11v11
let pmDurIdx = 1; // 2 min default
let pmDiffIdx = 1; // Professional (Normal)
let pendingOfficialFixture = null; // Set when entering Pre-Match from an Official Parallel Fixture!

// Live match & deterministic substitution state
let activeOfficialSession = null;
let paused = false;
let fulltimeShown = false;
let goalBannerTicks = 0;
let lastShootoutScore = null;
let homePossTicks = 1;
let awayPossTicks = 1;
let matchEventsLog = [];
let queuedSubReserved = [0, 0]; // 16-bit encoded substitution per side for next sim_tick!
let subsUsedCount = [0, 0];
let subsSide = 0;
let subsArmedSlot = null;

// Instant Replay Theater
let replayBuffer = [];
let replayActive = false;
let replayCursor = 0.0;
let replayCamIdx = 0;
const REPLAY_CAM_MODES = ["orbit", "closeup", "broadcast"];
const REPLAY_CAM_LABELS = {
  orbit: "CAM: ORBIT 360°",
  closeup: "CAM: ACTION CLOSE",
  broadcast: "CAM: TV REPLAY",
};

// Squad Builder state
let activeSquadClub = "man-utd";
let swapArmedStarterId = null;
const draftLineups = {};

// Cartridge state
let latestSyncedEnvelope = null;

// Input tracking
const keysDown = new Set();
let prevPadNavTick = 0;
let prevStartBtn = false;

// --- WASM Helpers ---

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

// --- Screen Router ---

const SCREEN_TITLES = {
  menu: "MAIN MENU",
  prematch: "MATCH SETUP & SELECT SIDES",
  squad: "TEAM MANAGEMENT (11 → 5)",
  season: "PARALLEL SEASON HUB",
  controllers: "CONTROLLERS & PROFILE CARTRIDGE",
};

function navigateTo(screen) {
  if (currentScreen !== screen && currentScreen !== "match" && currentScreen !== "title") {
    previousScreen = currentScreen;
  }
  currentScreen = screen;

  document.querySelectorAll(".game-screen").forEach((el) => {
    const match = el.getAttribute("data-screen") === screen;
    el.hidden = !match;
    el.classList.toggle("active", match);
  });

  const topbar = document.getElementById("console-topbar");
  const showTopbar = screen !== "title" && screen !== "match";
  topbar.hidden = !showTopbar;
  if (showTopbar) {
    document.getElementById("ct-screen-title").textContent = SCREEN_TITLES[screen] || "PARALLEL SEASON FC";
  }

  document.body.classList.toggle("in-live-match", screen === "match");
  document.body.classList.toggle("in-prematch-showcase", screen === "prematch");
  GameAudio.setMatchAmbience(screen === "match");

  if (screen === "title" || screen === "menu" || screen === "season" || screen === "controllers") {
    engine3d.setViewMode("attract");
    if (!isAttractMatch) {
      startAttractBackgroundMatch();
    }
  } else if (screen === "prematch") {
    engine3d.setViewMode("showcase");
    renderPreMatchScreen();
  } else if (screen === "squad") {
    engine3d.setViewMode("squad");
    renderSquadBuilder();
  } else if (screen === "match") {
    engine3d.setViewMode("match");
  }
}

// --- Gamepad, SNES 2.4GHz / Bluetooth Retro Pad, WebHID & Keyboard Input ---

const PAD_CUSTOM_MAP_KEY = "psfc_custom_pad_map_v1";
const PAD_PROFILE_STORAGE_KEY = "psfc_pad_profile_v1";

let customPadMap = (() => {
  try {
    const raw = localStorage.getItem(PAD_CUSTOM_MAP_KEY);
    return raw ? JSON.parse(raw) : null;
  } catch (_e) {
    return null;
  }
})();

const webHidPads = [];
const padAxisBaselines = new Map();
let lastGamepadSig = "";
let autoSwitchedTo2P = false;
let prevSelectBtn = false;
let pmFocusedRowIdx = 0;

// Interactive 12-step Controller Calibration Wizard state
const CALIB_STEPS = [
  { key: "up", label: "Press D-PAD UP" },
  { key: "down", label: "Press D-PAD DOWN" },
  { key: "left", label: "Press D-PAD LEFT" },
  { key: "right", label: "Press D-PAD RIGHT" },
  { key: "south", label: "Press B (BOTTOM · Ground Pass / Tackle)" },
  { key: "east", label: "Press A (RIGHT · Shoot / Back)" },
  { key: "west", label: "Press Y (LEFT · Lob / Slide Tackle)" },
  { key: "north", label: "Press X (TOP · Through Ball)" },
  { key: "l1", label: "Press L SHOULDER (Switch / Finesse)" },
  { key: "r1", label: "Press R SHOULDER (Sprint)" },
  { key: "select", label: "Press SELECT (Skill Move / Replay)" },
  { key: "start", label: "Press START (Kick Off / Pause)" },
];
let calibState = null; // { stepIdx, draft, waitingRelease, restAxes }

function hex4(n) {
  return Number(n || 0).toString(16).padStart(4, "0");
}

function isSnesSignature(idStr, btnCount = 16) {
  const s = String(idStr || "").toLowerCase();
  if (
    s.includes("0079") ||
    s.includes("0810") ||
    s.includes("0583") ||
    s.includes("20bc") ||
    s.includes("snes") ||
    s.includes("super nintendo") ||
    s.includes("sfc") ||
    s.includes("sn30") ||
    s.includes("sf30") ||
    s.includes("retroflag") ||
    s.includes("ibuffalo") ||
    s.includes("2.4g") ||
    s.includes("generic usb joystick")
  ) {
    return true;
  }
  return btnCount > 0 && btnCount <= 11;
}

function classifyGamepadVendor(idStr, btnCount = 16) {
  const s = String(idStr || "").toLowerCase();
  if (s.includes("psfc") || s.includes("rp2040") || s.includes("2e8a")) return "PSFC RP2040 PAD";
  if (isSnesSignature(s, btnCount)) return "SNES RETRO PAD";
  if (s.includes("2dc8") || s.includes("8bitdo")) return "8BITDO PAD";
  if (s.includes("057e") || s.includes("pro controller") || s.includes("nintendo")) return "SWITCH PAD";
  if (s.includes("045e") || s.includes("xbox") || s.includes("xinput")) return "XBOX CONTROLLER";
  if (s.includes("054c") || s.includes("dualsense") || s.includes("dualshock") || s.includes("wireless controller")) {
    return "PLAYSTATION PAD";
  }
  if (s.includes("046d") || s.includes("logitech")) return "LOGITECH PAD";
  return "USB / BT GAMEPAD";
}

/**
 * Collect all connected browser Gamepads (safely skipping sparse `null` slots on macOS/Safari/Chrome)
 * plus any WebHID direct USB receiver pads.
 */
function getConnectedPads() {
  const out = [];
  if (navigator.getGamepads) {
    try {
      const rawList = navigator.getGamepads();
      if (rawList) {
        for (let i = 0; i < rawList.length; i++) {
          const p = rawList[i];
          if (p && p.connected) out.push(p);
        }
      }
    } catch (_e) {
      // Ignore transient Gamepad API access error
    }
  }
  for (let i = 0; i < webHidPads.length; i++) {
    if (webHidPads[i].connected) out.push(webHidPads[i]);
  }
  return out;
}

function decodeWebHidReport(vPad, bytes) {
  if (!bytes || bytes.length === 0) return;
  if (!vPad._restBytes) {
    vPad._restBytes = new Uint8Array(bytes);
    vPad._centeredBytes = new Set();
  }
  const rest = vPad._restBytes;
  for (let i = 0; i < bytes.length; i++) {
    if (bytes[i] >= 0x70 && bytes[i] <= 0x8f) {
      vPad._centeredBytes.add(i);
    } else if (i < rest.length && !(rest[i] >= 0x70 && rest[i] <= 0x8f)) {
      // Clear any bit that was held on packet 0 as soon as we observe 0 in that bit
      rest[i] &= bytes[i];
    }
  }

  // Detect X/Y D-Pad byte pair on standard 7/8-byte USB SNES / DragonRise / Microntek HID reports (`0x00=-1, 0x7f/0x80=0, 0xff=+1`)
  let axX = 0;
  let axY = 0;
  const candidatePairs = [
    [0, 1],
    [3, 4],
    [2, 3],
    [1, 2],
  ];
  const isAxisLikeByte = (idx) =>
    vPad._centeredBytes.has(idx) ||
    (bytes[idx] >= 0x70 && bytes[idx] <= 0x8f) ||
    ((idx === 0 || idx === 1) && (bytes[idx] === 0x00 || bytes[idx] === 0xff));

  for (const [ix, iy] of candidatePairs) {
    if (iy < bytes.length && isAxisLikeByte(ix) && isAxisLikeByte(iy)) {
      const vx = (bytes[ix] - 127.5) / 127.5;
      const vy = (bytes[iy] - 127.5) / 127.5;
      if (Math.abs(vx) > 0.35 || Math.abs(vy) > 0.35 || (axX === 0 && axY === 0)) {
        axX = Math.abs(vx) > 0.35 ? Math.sign(vx) : 0;
        axY = Math.abs(vy) > 0.35 ? Math.sign(vy) : 0;
        if (axX !== 0 || axY !== 0) break;
      }
    }
  }

  // Clear button states
  for (let i = 0; i < vPad.buttons.length; i++) {
    vPad.buttons[i].pressed = false;
    vPad.buttons[i].value = 0;
  }

  if (bytes.length >= 7 && (bytes[5] & 0x0f) === 0x0f) {
    vPad._isDragonRiseFormat = true;
  }

  // Standard DragonRise / Microntek USB SNES (`byte[5]` upper nibble = buttons 0..3, lower nibble = hat; `byte[6]` = L, R, Select, Start)
  if (bytes.length >= 7 && (vPad._isDragonRiseFormat || (rest[5] & 0x0f) === 0x0f)) {
    const b5 = bytes[5];
    const b6 = bytes[6];
    for (let k = 0; k < 4; k++) {
      const on = ((b5 >>> (4 + k)) & 1) === 1;
      vPad.buttons[k].pressed = on;
      vPad.buttons[k].value = on ? 1 : 0;
    }
    const hat = b5 & 0x0f;
    if (hat < 8 && axX === 0 && axY === 0) {
      const dirs = [
        [0, -1],
        [1, -1],
        [1, 0],
        [1, 1],
        [0, 1],
        [-1, 1],
        [-1, 0],
        [-1, -1],
      ];
      axX = dirs[hat][0];
      axY = dirs[hat][1];
    }
    const setBtn = (idx, on) => {
      if (idx < vPad.buttons.length) {
        vPad.buttons[idx].pressed = on;
        vPad.buttons[idx].value = on ? 1 : 0;
      }
    };
    // Shoulder L / R -> slots 4, 5
    setBtn(4, (b6 & 0x01) !== 0);
    setBtn(5, (b6 & 0x02) !== 0);
    if ((b6 & 0x30) !== 0 || vPad._uses10BtnMeta) {
      // 10-button DragonRise layout: bits 2,3 = L2,R2 (slots 6,7), bits 4,5 = Select,Start (slots 8,9)
      if ((b6 & 0x30) !== 0) vPad._uses10BtnMeta = true;
      setBtn(6, (b6 & 0x04) !== 0);
      setBtn(7, (b6 & 0x08) !== 0);
      setBtn(8, (b6 & 0x10) !== 0);
      setBtn(9, (b6 & 0x20) !== 0);
    } else {
      // 8-button USB SNES layout: bits 2,3 = Select,Start -> aligned directly to slots 8,9 consumed by sampleRawGamepad!
      setBtn(8, (b6 & 0x04) !== 0);
      setBtn(9, (b6 & 0x08) !== 0);
    }
  } else {
    // Generic fallback: map changed non-axis bit positions against resting HID report
    const activeBits = [];
    for (let bi = 0; bi < bytes.length; bi++) {
      if (vPad._centeredBytes.has(bi) || (rest[bi] >= 0x70 && rest[bi] <= 0x8f)) continue;
      const diff = bytes[bi] & ~rest[bi];
      for (let bit = 0; bit < 8; bit++) {
        if (((rest[bi] >>> bit) & 1) === 0) {
          activeBits.push(((diff >>> bit) & 1) === 1);
        }
      }
    }
    if (activeBits.length <= 8) {
      // Single button byte (8-button USB SNES layout): 0..5 = Face + L/R, 6..7 = Select/Start -> slots 8,9
      for (let i = 0; i < Math.min(6, activeBits.length); i++) {
        vPad.buttons[i].pressed = activeBits[i];
        vPad.buttons[i].value = activeBits[i] ? 1 : 0;
      }
      if (activeBits.length > 6) {
        vPad.buttons[8].pressed = activeBits[6];
        vPad.buttons[8].value = activeBits[6] ? 1 : 0;
      }
      if (activeBits.length > 7) {
        vPad.buttons[9].pressed = activeBits[7];
        vPad.buttons[9].value = activeBits[7] ? 1 : 0;
      }
    } else {
      for (let i = 0; i < Math.min(vPad.buttons.length, activeBits.length); i++) {
        vPad.buttons[i].pressed = activeBits[i];
        vPad.buttons[i].value = activeBits[i] ? 1 : 0;
      }
    }
  }

  vPad.axes[0] = axX;
  vPad.axes[1] = axY;
}

async function pairWebHidUsbReceiver() {
  if (!navigator.hid || typeof navigator.hid.requestDevice !== "function") return;
  try {
    const devices = await navigator.hid.requestDevice({
      filters: [
        { usagePage: 0x01, usage: 0x04 }, // Joystick
        { usagePage: 0x01, usage: 0x05 }, // Gamepad
      ],
    });
    for (const dev of devices) {
      if (webHidPads.some((p) => p._dev === dev)) continue;
      if (!dev.opened) await dev.open();
      const vPad = {
        id: `${dev.productName || "USB SNES Receiver"} (${hex4(dev.vendorId)}:${hex4(dev.productId)})`,
        index: 100 + webHidPads.length,
        connected: true,
        mapping: "",
        axes: [0, 0, 0, 0],
        buttons: Array.from({ length: 10 }, () => ({ pressed: false, value: 0 })),
        _dev: dev,
        _isWebHid: true,
        _restBytes: null,
        _centeredBytes: null,
        _isDragonRiseFormat: false,
        _uses10BtnMeta: false,
      };
      dev.addEventListener("inputreport", (ev) => {
        const view = new Uint8Array(ev.data.buffer, ev.data.byteOffset, ev.data.byteLength);
        decodeWebHidReport(vPad, view);
      });
      webHidPads.push(vPad);
    }
    updateGamepadBadges();
    GameAudio.uiConfirm();
  } catch (_err) {
    // User cancelled WebHID chooser or device busy
  }
}

if (navigator.hid && typeof navigator.hid.addEventListener === "function") {
  navigator.hid.addEventListener("disconnect", (ev) => {
    for (const p of webHidPads) {
      if (p._dev === ev.device) p.connected = false;
    }
    updateGamepadBadges();
  });
}

function updateGamepadBadges() {
  const pads = getConnectedPads();
  const pill = document.getElementById("gamepad-status-pill");
  const titlePad = document.getElementById("title-pad-pill");
  if (pads.length > 0) {
    const label = classifyGamepadVendor(pads[0].id, pads[0].buttons ? pads[0].buttons.length : 16);
    const extra = pads.length > 1 ? ` · 2P READY (${pads.length})` : "";
    pill.textContent = `${label}${extra}`;
    pill.classList.add("pad-connected");
    if (titlePad) {
      titlePad.textContent =
        pads.length >= 2
          ? `${pads.length} CONTROLLERS CONNECTED · PRESS START OR B`
          : `${label} CONNECTED · PRESS START OR B`;
    }
  } else {
    pill.textContent = "PAD / KEYBOARD READY";
    pill.classList.remove("pad-connected");
    if (titlePad) titlePad.textContent = "PLUG IN ANY SNES / XBOX / PS5 / USB PAD & PRESS START";
  }

  // Update Dual-Pad Live Slot Pills on SCREEN 5
  for (let slot = 0; slot < 2; slot++) {
    const box = document.getElementById(`pad-slot-${slot}`);
    const txt = document.getElementById(`pad-slot-${slot}-text`);
    if (!box || !txt) continue;
    const p = pads[slot];
    if (p) {
      const shortVendor = classifyGamepadVendor(p.id, p.buttons ? p.buttons.length : 16);
      box.classList.add("connected");
      txt.textContent = `${shortVendor} · ${p.buttons.length}B/${p.axes.length}A (${String(p.id).slice(0, 28)})`;
    } else {
      box.classList.remove("connected");
      txt.textContent = `Waiting — Press START or B on Controller ${slot + 1}`;
    }
  }

  const guide = document.getElementById("pad-connect-guide");
  if (guide) {
    guide.hidden = pads.length >= 2;
  }

  // Automatically enable LOCAL 2P (Home vs Away) the first time 2 controllers connect outside a live match!
  if (
    !autoSwitchedTo2P &&
    pads.length >= 2 &&
    currentScreen !== "match" &&
    (!pendingOfficialFixture || pendingOfficialFixture.result_source === "played_vs_friend")
  ) {
    autoSwitchedTo2P = true;
    const local2pIdx = SIDE_MODES.findIndex((m) => m.id === "local_2p");
    if (local2pIdx >= 0 && pmSideIdx === 0) {
      pmSideIdx = local2pIdx;
      if (currentScreen === "prematch") renderPreMatchScreen();
    }
  }
}

function syncGamepadConnectionState() {
  const pads = getConnectedPads();
  const sig = pads.map((p) => `${p.index}:${p.id}`).join("|");
  if (sig !== lastGamepadSig) {
    lastGamepadSig = sig;
    updateGamepadBadges();
  }
}

window.addEventListener("gamepadconnected", updateGamepadBadges);
window.addEventListener("gamepaddisconnected", updateGamepadBadges);

function decodeHatDirection(hatVal) {
  // Standard 8-way HID POV hat encodes active steps 0..7 in [-1.0, +1.0] (-1, -5/7, -3/7, -1/7, +1/7, +3/7, +5/7, +1);
  // neutral/released is > 1.05 (e.g. +1.2857 / +3.2857), and an unused 0-centered axis sits near 0.0 (|v| <= 0.06).
  if (hatVal < -1.05 || hatVal > 1.05 || Math.abs(hatVal) <= 0.06) return [0, 0];
  const step = Math.max(0, Math.min(7, Math.round(((hatVal + 1.0) * 7.0) / 2.0)));
  const dirs = [
    [0, -1],
    [1, -1],
    [1, 0],
    [1, 1],
    [0, 1],
    [-1, 1],
    [-1, 0],
    [-1, -1],
  ];
  return dirs[step] || [0, 0];
}

function updatePadAxisTrackers(pad) {
  if (!pad.axes || pad.axes.length === 0) return null;
  const key = `${pad.index}|${pad.id}`;
  let states = padAxisBaselines.get(key);
  if (!states || states.length !== pad.axes.length) {
    states = Array.from(pad.axes, (v) => {
      const num = Number(v || 0);
      return {
        initial: num,
        min: num,
        max: num,
        sawCenter: Math.abs(num) < 0.35,
      };
    });
    padAxisBaselines.set(key, states);
    return states;
  }
  for (let i = 0; i < pad.axes.length; i++) {
    const raw = Number(pad.axes[i] || 0);
    const st = states[i];
    if (raw < st.min) st.min = raw;
    if (raw > st.max) st.max = raw;
    if (Math.abs(raw) < 0.35 || (st.min < -0.4 && st.max > 0.4)) {
      st.sawCenter = true;
    }
  }
  return states;
}

function evalCustomBinding(pad, spec, axisStates) {
  if (!spec) return false;
  if (spec.type === "btn") {
    const bt = pad.buttons[spec.index];
    return Boolean(bt && (bt.pressed || bt.value > 0.4));
  }
  if (spec.type === "axis") {
    const v = Number(pad.axes[spec.index] || 0);
    return v * spec.sign > 0.45;
  }
  if (spec.type === "hat") {
    const v = Number(pad.axes[spec.index] || 0);
    const [hx, hy] = decodeHatDirection(v);
    if (spec.dx && spec.dy) return hx === spec.dx && hy === spec.dy;
    if (spec.dx) return hx === spec.dx;
    if (spec.dy) return hy === spec.dy;
    // Backwards-compatibility if a legacy single-value hat binding was stored in localStorage
    if (typeof spec.value === "number") {
      const [shx, shy] = decodeHatDirection(spec.value);
      return (shx !== 0 && hx === shx) || (shy !== 0 && hy === shy);
    }
    return false;
  }
  return false;
}

function stepPadCalibrationWizard() {
  if (!calibState) return;
  const pads = getConnectedPads();
  if (!pads.length) return;

  const step = CALIB_STEPS[calibState.stepIdx];
  if (!step) return;

  let anyHeldDuringRelease = false;
  let detectedSpec = null;

  for (const pad of pads) {
    const axisStates = updatePadAxisTrackers(pad);

    let btnPressedIdx = -1;
    for (let bi = 0; bi < pad.buttons.length; bi++) {
      const bt = pad.buttons[bi];
      if (bt && (bt.pressed || bt.value > 0.5)) {
        btnPressedIdx = bi;
        break;
      }
    }

    let hatDir = [0, 0];
    if (pad.axes && pad.axes.length > 9) {
      hatDir = decodeHatDirection(Number(pad.axes[9] || 0));
    }

    if (calibState.waitingRelease) {
      const primaryActive =
        pad.axes &&
        ((pad.axes.length > 0 && Math.abs(Number(pad.axes[0] || 0)) > 0.4) ||
          (pad.axes.length > 1 && Math.abs(Number(pad.axes[1] || 0)) > 0.4));
      let secActive = false;
      if (pad.axes && axisStates) {
        for (let ai = 2; ai < pad.axes.length; ai++) {
          if (ai === 9) continue;
          const v = Number(pad.axes[ai] || 0);
          const st = axisStates[ai];
          if ((ai >= 6 || (st && st.sawCenter)) && Math.abs(v) > 0.45) {
            secActive = true;
            break;
          }
        }
      }
      if (btnPressedIdx >= 0 || hatDir[0] !== 0 || hatDir[1] !== 0 || primaryActive || secActive) {
        anyHeldDuringRelease = true;
      }
      continue;
    }

    let activeSpec = null;
    if (btnPressedIdx >= 0) {
      activeSpec = { type: "btn", index: btnPressedIdx };
    } else if (hatDir[0] !== 0 || hatDir[1] !== 0) {
      const wantVert = step.key === "up" || step.key === "down";
      const wantHoriz = step.key === "left" || step.key === "right";
      if (wantVert && hatDir[1] !== 0) {
        activeSpec = { type: "hat", index: 9, dx: 0, dy: hatDir[1] };
      } else if (wantHoriz && hatDir[0] !== 0) {
        activeSpec = { type: "hat", index: 9, dx: hatDir[0], dy: 0 };
      } else {
        activeSpec = {
          type: "hat",
          index: 9,
          dx: hatDir[0] !== 0 && hatDir[1] === 0 ? hatDir[0] : 0,
          dy: hatDir[1] !== 0 && hatDir[0] === 0 ? hatDir[1] : hatDir[1] || 0,
        };
      }
    } else if (pad.axes) {
      const rest = calibState.restAxes.get(pad.index) || [];
      for (let ai = 0; ai < pad.axes.length; ai++) {
        if (ai === 9) continue;
        const v = Number(pad.axes[ai] || 0);
        const r = rest[ai] || 0;
        const st = axisStates && ai < axisStates.length ? axisStates[ai] : null;
        const effectiveRest = ai >= 2 && ai <= 5 && st && !st.sawCenter && Math.abs(r) > 0.80 ? r : 0;
        if (Math.abs(v - effectiveRest) > 0.55 && Math.abs(v) > 0.45) {
          activeSpec = { type: "axis", index: ai, sign: v > effectiveRest ? 1 : -1 };
          break;
        }
      }
    }

    if (activeSpec && !detectedSpec) {
      detectedSpec = activeSpec;
    }
  }

  if (calibState.waitingRelease) {
    if (!anyHeldDuringRelease) {
      calibState.waitingRelease = false;
      for (const pad of pads) {
        calibState.restAxes.set(
          pad.index,
          Array.from(pad.axes || [], (v) => Number(v || 0))
        );
      }
    }
    return;
  }

  if (detectedSpec) {
    calibState.draft[step.key] = detectedSpec;
    calibState.stepIdx++;
    calibState.waitingRelease = true;
    GameAudio.uiTick(540);
    advanceOrFinishCalibration();
  }
}

function advanceOrFinishCalibration() {
  const banner = document.getElementById("pad-calib-banner");
  const promptEl = document.getElementById("pad-calib-prompt");
  if (!calibState || calibState.stepIdx >= CALIB_STEPS.length) {
    if (calibState) {
      customPadMap = calibState.draft;
      try {
        localStorage.setItem(PAD_CUSTOM_MAP_KEY, JSON.stringify(customPadMap));
        localStorage.setItem(PAD_PROFILE_STORAGE_KEY, "custom");
      } catch (_e) {}
      const sel = document.getElementById("pad-profile-select");
      if (sel) sel.value = "custom";
      GameAudio.uiConfirm();
    }
    calibState = null;
    if (banner) banner.hidden = true;
    return;
  }
  if (banner) banner.hidden = false;
  if (promptEl) {
    promptEl.textContent = `STEP ${calibState.stepIdx + 1}/${CALIB_STEPS.length}: ${CALIB_STEPS[calibState.stepIdx].label}`;
  }
}

function startPadCalibration() {
  calibState = {
    stepIdx: 0,
    draft: {},
    waitingRelease: true,
    restAxes: new Map(),
  };
  advanceOrFinishCalibration();
}

function sampleRawGamepad(logicalSlot) {
  const pads = getConnectedPads();
  const pad = pads[logicalSlot];
  if (!pad) return null;

  const profileSel = document.getElementById("pad-profile-select");
  const profile = profileSel ? profileSel.value : "auto";
  const idLc = String(pad.id || "").toLowerCase();
  const btnCount = pad.buttons ? pad.buttons.length : 0;
  const axisStates = updatePadAxisTrackers(pad);

  const snesDetected = isSnesSignature(idLc, btnCount);
  const useCustom = profile === "custom" && customPadMap;
  const isSnesUsb =
    profile === "snes_usb" ||
    (profile === "auto" && snesDetected && pad.mapping !== "standard");
  const isSnesStd =
    profile === "snes_std" ||
    (profile === "auto" && snesDetected && pad.mapping === "standard");
  const isSnesMode = isSnesUsb || isSnesStd || (useCustom && snesDetected);
  const isSwitchSwap = profile === "switch" || isSnesStd;
  const isDirectInput =
    profile === "dinput" ||
    (profile === "auto" &&
      !isSnesUsb &&
      pad.mapping !== "standard" &&
      !idLc.includes("xbox") &&
      !idLc.includes("xinput") &&
      !idLc.includes("045e") &&
      !idLc.includes("054c") &&
      !idLc.includes("dualsense") &&
      !idLc.includes("dualshock"));

  const cleanAxis = (idx, deadzone = 0.22) => {
    if (!pad.axes || idx >= pad.axes.length) return 0;
    const raw = Number(pad.axes[idx] || 0);
    const st = axisStates && idx < axisStates.length ? axisStates[idx] : null;
    // Reject unpressed DirectInput Z/Rz trigger axes (2..5) stuck at ±1.0 until center (`|raw| < 0.35`) or full-swing is observed!
    if (idx >= 2 && idx <= 5 && st && !st.sawCenter && Math.abs(st.initial) > 0.80 && Math.abs(raw - st.initial) < 0.25) {
      return 0;
    }
    return Math.abs(raw) < deadzone ? 0 : Math.max(-1, Math.min(1, raw));
  };

  const b = (i) => Boolean(pad.buttons[i] && (pad.buttons[i].pressed || pad.buttons[i].value > 0.42));
  const trig = (i) => Boolean(pad.buttons[i] && (pad.buttons[i].pressed || pad.buttons[i].value > 0.24));

  let lx = 0;
  let ly = 0;
  let rx = 0;
  let ry = 0;
  let south = false;
  let east = false;
  let west = false;
  let north = false;
  let l1 = false;
  let r1 = false;
  let l2 = false;
  let r2 = false;
  let selectBtn = false;
  let startBtn = false;

  if (useCustom) {
    if (evalCustomBinding(pad, customPadMap.left, axisStates)) lx -= 1;
    if (evalCustomBinding(pad, customPadMap.right, axisStates)) lx += 1;
    if (evalCustomBinding(pad, customPadMap.up, axisStates)) ly -= 1;
    if (evalCustomBinding(pad, customPadMap.down, axisStates)) ly += 1;
    if (lx === 0 && ly === 0) {
      lx = cleanAxis(0, 0.25);
      ly = cleanAxis(1, 0.25);
    }
    south = evalCustomBinding(pad, customPadMap.south, axisStates);
    east = evalCustomBinding(pad, customPadMap.east, axisStates);
    west = evalCustomBinding(pad, customPadMap.west, axisStates);
    north = evalCustomBinding(pad, customPadMap.north, axisStates);
    l1 = evalCustomBinding(pad, customPadMap.l1, axisStates);
    r1 = evalCustomBinding(pad, customPadMap.r1, axisStates);
    selectBtn = evalCustomBinding(pad, customPadMap.select, axisStates);
    startBtn = evalCustomBinding(pad, customPadMap.start, axisStates);
  } else {
    lx = cleanAxis(0, 0.24);
    ly = cleanAxis(1, 0.24);
    if (!isSnesMode) {
      rx = cleanAxis(2, 0.22);
      const rawRy =
        isDirectInput && pad.axes && pad.axes.length >= 6 && Math.abs(cleanAxis(3, 0.05)) < 0.05
          ? cleanAxis(5, 0.22)
          : cleanAxis(3, 0.22);
      ry = rawRy;
    }

    let dpadX = 0;
    let dpadY = 0;
    if (b(14)) dpadX -= 1;
    if (b(15)) dpadX += 1;
    if (b(12)) dpadY -= 1;
    if (b(13)) dpadY += 1;

    // Fallback secondary D-Pad axes (`axes[4]/axes[5]` or `axes[6]/axes[7]` on multi-axis USB adapters)
    if (dpadX === 0 && dpadY === 0 && lx === 0 && ly === 0 && pad.axes) {
      for (const [axI, ayI] of [
        [6, 7],
        [4, 5],
        [2, 3],
      ]) {
        if (isSnesMode || axI >= 4) {
          const sx = cleanAxis(axI, 0.45);
          const sy = cleanAxis(ayI, 0.45);
          if (sx !== 0 || sy !== 0) {
            dpadX = Math.sign(sx);
            dpadY = Math.sign(sy);
            break;
          }
        }
      }
    }

    // Stateless 8-way POV Hat switch (`axes[9]`): active in [-1.0, +1.0], neutral > +1.05 or 0.0
    if (dpadX === 0 && dpadY === 0 && lx === 0 && ly === 0 && pad.axes && pad.axes.length > 9) {
      const [hx, hy] = decodeHatDirection(Number(pad.axes[9] || 0));
      dpadX = hx;
      dpadY = hy;
    }

    if (lx === 0 && ly === 0 && (dpadX !== 0 || dpadY !== 0)) {
      lx = dpadX;
      ly = dpadY;
    }

    if (isSnesUsb) {
      // Classic 2.4GHz / USB SNES HID (`0079:0011` / `0810:e501`): 0=X(Top), 1=A(Right), 2=B(Bottom), 3=Y(Left)
      north = b(0);
      east = b(1);
      south = b(2);
      west = b(3);
      l1 = b(4);
      r1 = b(5);
      l2 = btnCount >= 10 && b(6);
      r2 = btnCount >= 10 && b(7);
      selectBtn = b(8) || (btnCount <= 9 && b(6));
      startBtn = b(9) || (btnCount <= 9 && b(7));
    } else if (isSwitchSwap) {
      // Physical Nintendo/SNES label match: Bottom `B` = Pass (`b(0)`), Right `A` = Shoot (`b(1)`), Left `Y` = Lob (`b(2)`), Top `X` = Through (`b(3)`)
      south = b(0);
      east = b(1);
      west = b(2);
      north = b(3);
      l1 = b(4);
      r1 = b(5);
      l2 = !isSnesMode && trig(6);
      r2 = !isSnesMode && trig(7);
      selectBtn = b(8) || (isSnesMode && btnCount <= 9 && b(6));
      startBtn = b(9) || (isSnesMode && btnCount <= 9 && b(7));
    } else if (isDirectInput) {
      south = b(1);
      east = b(2);
      west = b(0);
      north = b(3);
      l1 = b(4);
      r1 = b(5);
      l2 = btnCount > 9 && trig(6);
      r2 = btnCount > 9 && trig(7);
      selectBtn = b(8) || (btnCount <= 9 && b(6));
      startBtn = b(9) || (btnCount <= 9 && b(7));
    } else {
      south = b(0);
      east = b(1);
      west = b(2);
      north = b(3);
      l1 = b(4);
      r1 = b(5);
      l2 = trig(6);
      r2 = trig(7);
      selectBtn = b(8);
      startBtn = b(9);
    }
  }

  // Normalize diagonal D-Pad vectors so 8-way SNES movement has true unit magnitude
  const magL = Math.hypot(lx, ly);
  if (magL > 1.0) {
    lx /= magL;
    ly /= magL;
  }

  let buttons = 0;
  if (south) buttons |= BTN_PASS | BTN_TACKLE;
  if (east) buttons |= BTN_SHOOT;
  if (west) buttons |= BTN_LOB | BTN_SLIDE;
  if (north) buttons |= isSnesMode ? BTN_THROUGH | BTN_CALL_RUN : BTN_THROUGH;

  if (isSnesMode || useCustom) {
    // 6-Action SNES Football Layout: R = Sprint, L = Switch Defender + Finesse/Driven/Jockey, SELECT = Skill Move!
    if (r1 || r2) buttons |= BTN_SPRINT;
    if (l1 || l2) buttons |= BTN_SWITCH | BTN_JOCKEY;
    if (selectBtn) buttons |= BTN_SKILL;
  } else {
    if (r2) buttons |= BTN_SPRINT;
    if (l2) buttons |= BTN_JOCKEY;
    if (l1) buttons |= BTN_SWITCH;
    if (r1) buttons |= BTN_CALL_RUN | BTN_TACKLE;
    if (b(11) || b(10) || selectBtn) buttons |= BTN_SKILL;
  }

  const mx = Math.max(-127, Math.min(127, Math.round(lx * 127)));
  const my = Math.max(-127, Math.min(127, Math.round(ly * 127)));
  const rStickX = Math.max(-127, Math.min(127, Math.round(rx * 127)));
  const rStickY = Math.max(-127, Math.min(127, Math.round(ry * 127)));
  const aimingKick = (buttons & (BTN_PASS | BTN_THROUGH | BTN_LOB | BTN_SHOOT)) !== 0;

  return {
    mx,
    my,
    ax: aimingKick ? mx : rStickX,
    ay: aimingKick ? my : rStickY,
    btns: buttons,
    south,
    east,
    west,
    north,
    l1: l1 || l2,
    r1: r1 || r2,
    select: selectBtn,
    start: startBtn,
    lx,
    ly,
    rx,
    ry,
    id: pad.id,
    rawPad: pad,
  };
}

function isPadInputActive(gp) {
  return Boolean(
    gp &&
      (gp.mx !== 0 ||
        gp.my !== 0 ||
        gp.ax !== 0 ||
        gp.ay !== 0 ||
        gp.btns !== 0 ||
        gp.start ||
        gp.select)
  );
}

function samplePrimaryHumanInput(allowArrows) {
  // In 1P modes (`allowArrows === true`), respond to WHICHEVER connected controller is being used!
  if (allowArrows) {
    const pads = getConnectedPads();
    for (let i = 0; i < pads.length; i++) {
      const gpAny = sampleRawGamepad(i);
      if (isPadInputActive(gpAny)) return gpAny;
    }
  } else {
    const gp = sampleRawGamepad(0);
    if (isPadInputActive(gp)) return gp;
  }

  let dx = 0;
  let dy = 0;
  if (keysDown.has("KeyA") || (allowArrows && keysDown.has("ArrowLeft"))) dx -= 1;
  if (keysDown.has("KeyD") || (allowArrows && keysDown.has("ArrowRight"))) dx += 1;
  if (keysDown.has("KeyW") || (allowArrows && keysDown.has("ArrowUp"))) dy -= 1;
  if (keysDown.has("KeyS") || (allowArrows && keysDown.has("ArrowDown"))) dy += 1;
  if (dx !== 0 && dy !== 0) {
    dx *= Math.SQRT1_2;
    dy *= Math.SQRT1_2;
  }

  let btns = 0;
  if (keysDown.has("Space") || keysDown.has("KeyJ")) btns |= BTN_PASS | BTN_TACKLE;
  if (keysDown.has("KeyK")) btns |= BTN_THROUGH;
  if (keysDown.has("KeyU")) btns |= BTN_LOB | BTN_SLIDE;
  if (keysDown.has("KeyL") || keysDown.has("Enter")) btns |= BTN_SHOOT;
  if (keysDown.has("ShiftLeft") || keysDown.has("ShiftRight")) btns |= BTN_SPRINT;
  if (keysDown.has("KeyQ")) btns |= BTN_SWITCH;
  if (keysDown.has("KeyF")) btns |= BTN_JOCKEY;
  if (keysDown.has("KeyE")) btns |= BTN_TACKLE;
  if (keysDown.has("KeyC")) btns |= BTN_SLIDE;
  if (keysDown.has("KeyR")) btns |= BTN_CALL_RUN;
  if (keysDown.has("KeyZ")) btns |= BTN_SKILL;

  const mx = Math.round(dx * 127);
  const my = Math.round(dy * 127);
  const aiming = (btns & (BTN_PASS | BTN_THROUGH | BTN_LOB | BTN_SHOOT)) !== 0;
  return { mx, my, ax: aiming ? mx : 0, ay: aiming ? my : 0, btns };
}

function sampleSecondaryHumanInput() {
  const gp2 = sampleRawGamepad(1);
  if (isPadInputActive(gp2)) return gp2;

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
  if (keysDown.has("Numpad1") || keysDown.has("Period")) btns |= BTN_PASS | BTN_TACKLE;
  if (keysDown.has("Numpad2") || keysDown.has("Slash")) btns |= BTN_SHOOT;
  if (keysDown.has("Numpad3")) btns |= BTN_THROUGH;
  if (keysDown.has("Numpad0")) btns |= BTN_SKILL;
  if (keysDown.has("ControlRight") || keysDown.has("RightControl")) btns |= BTN_SPRINT;
  if (keysDown.has("Comma")) btns |= BTN_SWITCH | BTN_TACKLE;

  const mx = Math.round(dx * 127);
  const my = Math.round(dy * 127);
  const aiming = (btns & (BTN_PASS | BTN_THROUGH | BTN_LOB | BTN_SHOOT)) !== 0;
  return { mx, my, ax: aiming ? mx : 0, ay: aiming ? my : 0, btns };
}

// Update live Controller Visualizer when on SCREEN_CONTROLLERS (merges P1 & P2 so testing either SNES pad lights immediately!)
function updateControllerVisualizer() {
  if (currentScreen !== "controllers") return;
  if (calibState) {
    stepPadCalibrationWizard();
  }

  const pads = getConnectedPads();
  let activeGp = sampleRawGamepad(0);
  for (let i = 0; i < pads.length; i++) {
    const candidate = sampleRawGamepad(i);
    if (isPadInputActive(candidate)) {
      activeGp = candidate;
      break;
    }
  }

  const lDot = document.getElementById("vis-lstick");
  const rDot = document.getElementById("vis-rstick");
  const kLx = keysDown.has("KeyD") || keysDown.has("ArrowRight") ? 1 : keysDown.has("KeyA") || keysDown.has("ArrowLeft") ? -1 : 0;
  const kLy = keysDown.has("KeyS") || keysDown.has("ArrowDown") ? 1 : keysDown.has("KeyW") || keysDown.has("ArrowUp") ? -1 : 0;
  const lx = activeGp && (activeGp.lx !== 0 || activeGp.ly !== 0) ? activeGp.lx : kLx;
  const ly = activeGp && (activeGp.lx !== 0 || activeGp.ly !== 0) ? activeGp.ly : kLy;
  const rx = activeGp ? activeGp.rx : 0;
  const ry = activeGp ? activeGp.ry : 0;

  lDot.style.transform = `translate(calc(-50% + ${lx * 19}px), calc(-50% + ${ly * 19}px))`;
  rDot.style.transform = `translate(calc(-50% + ${rx * 19}px), calc(-50% + ${ry * 19}px))`;

  document.getElementById("vis-btn-south").classList.toggle("lit", Boolean((activeGp && activeGp.south) || keysDown.has("Space") || keysDown.has("KeyJ")));
  document.getElementById("vis-btn-east").classList.toggle("lit", Boolean((activeGp && activeGp.east) || keysDown.has("KeyL") || keysDown.has("Enter")));
  document.getElementById("vis-btn-west").classList.toggle("lit", Boolean((activeGp && activeGp.west) || keysDown.has("KeyU")));
  document.getElementById("vis-btn-north").classList.toggle("lit", Boolean((activeGp && activeGp.north) || keysDown.has("KeyK")));

  const l1El = document.getElementById("vis-btn-l1");
  const r1El = document.getElementById("vis-btn-r1");
  const selEl = document.getElementById("vis-btn-select");
  const stEl = document.getElementById("vis-btn-start");
  if (l1El) l1El.classList.toggle("lit", Boolean((activeGp && activeGp.l1) || keysDown.has("KeyQ") || keysDown.has("KeyF")));
  if (r1El) r1El.classList.toggle("lit", Boolean((activeGp && activeGp.r1) || keysDown.has("ShiftLeft") || keysDown.has("ShiftRight")));
  if (selEl) selEl.classList.toggle("lit", Boolean((activeGp && activeGp.select) || keysDown.has("KeyZ")));
  if (stEl) stEl.classList.toggle("lit", Boolean(activeGp && activeGp.start));

  const rawEl = document.getElementById("vis-raw-readout");
  if (rawEl) {
    if (activeGp && activeGp.rawPad) {
      const p = activeGp.rawPad;
      const pressedBtns = [];
      for (let i = 0; i < p.buttons.length; i++) {
        if (p.buttons[i] && (p.buttons[i].pressed || p.buttons[i].value > 0.4)) pressedBtns.push(`B${i}`);
      }
      const mvStr =
        activeGp.mx !== 0 || activeGp.my !== 0 ? `MV(${activeGp.mx},${activeGp.my}) ` : "";
      const axStr =
        p.axes && p.axes.length >= 2
          ? `AX(${Number(p.axes[0] || 0).toFixed(1)},${Number(p.axes[1] || 0).toFixed(1)})`
          : "";
      rawEl.textContent = pressedBtns.length
        ? `PRESSED: ${pressedBtns.join("+")} ${mvStr}${axStr}`.trim()
        : `READY · ${mvStr}${axStr || "IDLE"}`.trim();
    } else {
      rawEl.textContent = "RAW: IDLE";
    }
  }
}

// --- Console Keyboard & Gamepad Screen Navigation ---

function updateFocusedHubBlade(newIdx) {
  const blades = Array.from(document.querySelectorAll(".hub-blade"));
  if (!blades.length) return;
  focusedBladeIdx = (newIdx + blades.length) % blades.length;
  blades.forEach((b, i) => b.classList.toggle("focused", i === focusedBladeIdx));
  GameAudio.uiTick(480);
}

function updateFocusedPreMatchRow(newIdx) {
  const rows = Array.from(document.querySelectorAll("#screen-prematch .pm-setting-row"));
  if (!rows.length) return;
  pmFocusedRowIdx = (newIdx + rows.length) % rows.length;
  rows.forEach((r, i) => r.classList.toggle("pad-focused", i === pmFocusedRowIdx));
}

function stepFocusedPreMatchRow(dir) {
  const pairs = [
    ["pm-side-prev", "pm-side-next"],
    ["pm-fmt-prev", "pm-fmt-next"],
    ["pm-dur-prev", "pm-dur-next"],
    ["pm-diff-prev", "pm-diff-next"],
    ["pm-cam-prev", "pm-cam-next"],
    ["pm-wx-prev", "pm-wx-next"],
  ];
  const pair = pairs[pmFocusedRowIdx];
  if (!pair) return;
  const btn = document.getElementById(dir < 0 ? pair[0] : pair[1]);
  if (btn && !btn.disabled) btn.click();
}

function triggerHubBlade(navKey) {
  GameAudio.uiConfirm();
  if (navKey === "prematch-exhibition") {
    pendingOfficialFixture = null;
    navigateTo("prematch");
  } else {
    navigateTo(navKey);
  }
}

window.addEventListener("pointerdown", () => GameAudio.ensureCtx(), { once: true });

window.addEventListener("keydown", (ev) => {
  if (document.activeElement && ["SELECT", "INPUT", "TEXTAREA"].includes(document.activeElement.tagName)) {
    return;
  }
  GameAudio.ensureCtx();
  keysDown.add(ev.code);

  if (currentScreen === "title") {
    if (["Enter", "Space"].includes(ev.code)) {
      ev.preventDefault();
      GameAudio.uiConfirm();
      navigateTo("menu");
    }
    return;
  }

  if (currentScreen === "menu") {
    if (ev.code === "ArrowDown" || ev.code === "KeyS") {
      ev.preventDefault();
      updateFocusedHubBlade(focusedBladeIdx + 1);
    } else if (ev.code === "ArrowUp" || ev.code === "KeyW") {
      ev.preventDefault();
      updateFocusedHubBlade(focusedBladeIdx - 1);
    } else if (ev.code === "Enter" || ev.code === "Space") {
      ev.preventDefault();
      const blades = document.querySelectorAll(".hub-blade");
      const active = blades[focusedBladeIdx];
      if (active) triggerHubBlade(active.getAttribute("data-nav"));
    }
    return;
  }

  if (currentScreen === "prematch") {
    if (ev.code === "Escape") {
      ev.preventDefault();
      GameAudio.uiTick(360);
      navigateTo(pendingOfficialFixture ? "season" : "menu");
    } else if (ev.code === "ArrowDown") {
      ev.preventDefault();
      updateFocusedPreMatchRow(pmFocusedRowIdx + 1);
      GameAudio.uiTick(460);
    } else if (ev.code === "ArrowUp") {
      ev.preventDefault();
      updateFocusedPreMatchRow(pmFocusedRowIdx - 1);
      GameAudio.uiTick(460);
    } else if (ev.code === "ArrowLeft") {
      ev.preventDefault();
      stepFocusedPreMatchRow(-1);
    } else if (ev.code === "ArrowRight") {
      ev.preventDefault();
      stepFocusedPreMatchRow(1);
    } else if (ev.code === "KeyY") {
      ev.preventDefault();
      activeSquadClub = SIDE_MODES[pmSideIdx].id === "away_vs_cpu" ? pmAwayClub : pmHomeClub;
      navigateTo("squad");
    } else if (ev.code === "Enter") {
      ev.preventDefault();
      launchSelectedMatch();
    }
    return;
  }

  if (currentScreen === "squad" || currentScreen === "season" || currentScreen === "controllers") {
    if (ev.code === "Escape") {
      ev.preventDefault();
      if (calibState) {
        calibState = null;
        const banner = document.getElementById("pad-calib-banner");
        if (banner) banner.hidden = true;
        return;
      }
      GameAudio.uiTick(360);
      navigateTo(currentScreen === "squad" && previousScreen === "prematch" ? "prematch" : "menu");
    }
    return;
  }

  // Live Match Keys
  if (currentScreen === "match") {
    if (ev.code === "Escape") {
      ev.preventDefault();
      if (replayActive) {
        exitInstantReplay();
      } else if (!fulltimeShown) {
        togglePauseModal();
      }
      return;
    }
    if (ev.code === "Tab") {
      ev.preventDefault();
      toggleInstantReplay();
      return;
    }
    if (ev.code === "KeyV") {
      ev.preventDefault();
      cycleMatchCamera();
      return;
    }
    if (ev.code === "KeyM") {
      ev.preventDefault();
      cycleMatchWeather();
      return;
    }
    if (ev.code === "Minus" || ev.code === "NumpadSubtract") {
      ev.preventDefault();
      engine3d.adjustZoom(0.08);
      GameAudio.uiTick(440);
      return;
    }
    if (ev.code === "Equal" || ev.code === "NumpadAdd") {
      ev.preventDefault();
      engine3d.adjustZoom(-0.08);
      GameAudio.uiTick(560);
      return;
    }
    if (["Space", "ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"].includes(ev.code)) {
      ev.preventDefault();
    }
  }
});

window.addEventListener(
  "wheel",
  (ev) => {
    if (currentScreen === "match" && !paused) {
      engine3d.adjustZoom(ev.deltaY > 0 ? 0.06 : -0.06);
    }
  },
  { passive: true }
);

window.addEventListener("keyup", (ev) => keysDown.delete(ev.code));
window.addEventListener("blur", () => keysDown.clear());

function sampleAnyMenuGamepad() {
  const pads = getConnectedPads();
  let chosen = null;
  let anyStart = false;
  let anySelect = false;
  for (let i = 0; i < pads.length; i++) {
    const gp = sampleRawGamepad(i);
    if (!gp) continue;
    if (gp.start) anyStart = true;
    if (gp.select) anySelect = true;
    if (!chosen || (!isPadInputActive(chosen) && isPadInputActive(gp))) {
      chosen = gp;
    }
  }
  if (!chosen) return null;
  return { ...chosen, start: anyStart, select: anySelect };
}

function pollGamepadMenuActions(nowMs) {
  if (calibState) return; // Don't navigate screens while calibrating buttons!
  const gp = sampleAnyMenuGamepad();
  if (!gp) {
    prevStartBtn = false;
    prevSelectBtn = false;
    return;
  }

  const startEdge = gp.start && !prevStartBtn;
  const selectEdge = gp.select && !prevSelectBtn;
  prevStartBtn = gp.start;
  prevSelectBtn = gp.select;

  if (currentScreen === "title") {
    if (startEdge || gp.south || gp.east) {
      GameAudio.ensureCtx();
      prevPadNavTick = nowMs + 340;
      GameAudio.uiConfirm();
      navigateTo("menu");
    }
    return;
  }

  if (currentScreen === "match") {
    if (startEdge && !fulltimeShown) {
      if (replayActive) exitInstantReplay();
      else togglePauseModal();
    } else if (selectEdge && paused && !fulltimeShown) {
      toggleInstantReplay();
    } else if (paused && !replayActive && nowMs >= prevPadNavTick) {
      if (gp.south || gp.east) {
        prevPadNavTick = nowMs + 300;
        togglePauseModal();
      }
    }
    return;
  }

  if (nowMs < prevPadNavTick) return;

  if (currentScreen === "menu") {
    if (gp.ly > 0.55) {
      prevPadNavTick = nowMs + 210;
      updateFocusedHubBlade(focusedBladeIdx + 1);
    } else if (gp.ly < -0.55) {
      prevPadNavTick = nowMs + 210;
      updateFocusedHubBlade(focusedBladeIdx - 1);
    } else if (gp.south || startEdge) {
      prevPadNavTick = nowMs + 320;
      const active = document.querySelectorAll(".hub-blade")[focusedBladeIdx];
      if (active) triggerHubBlade(active.getAttribute("data-nav"));
    }
  } else if (currentScreen === "prematch") {
    updateFocusedPreMatchRow(pmFocusedRowIdx);
    if (gp.ly > 0.55) {
      prevPadNavTick = nowMs + 200;
      updateFocusedPreMatchRow(pmFocusedRowIdx + 1);
      GameAudio.uiTick(460);
    } else if (gp.ly < -0.55) {
      prevPadNavTick = nowMs + 200;
      updateFocusedPreMatchRow(pmFocusedRowIdx - 1);
      GameAudio.uiTick(460);
    } else if (gp.lx < -0.55) {
      prevPadNavTick = nowMs + 220;
      stepFocusedPreMatchRow(-1);
    } else if (gp.lx > 0.55) {
      prevPadNavTick = nowMs + 220;
      stepFocusedPreMatchRow(1);
    } else if (gp.l1) {
      prevPadNavTick = nowMs + 250;
      const b = document.getElementById("pm-home-next");
      if (b && !b.disabled) b.click();
    } else if (gp.r1) {
      prevPadNavTick = nowMs + 250;
      const b = document.getElementById("pm-away-next");
      if (b && !b.disabled) b.click();
    } else if (gp.south || startEdge) {
      prevPadNavTick = nowMs + 350;
      launchSelectedMatch();
    } else if (gp.east) {
      prevPadNavTick = nowMs + 320;
      navigateTo(pendingOfficialFixture ? "season" : "menu");
    } else if (gp.north || gp.west) {
      prevPadNavTick = nowMs + 320;
      activeSquadClub = SIDE_MODES[pmSideIdx].id === "away_vs_cpu" ? pmAwayClub : pmHomeClub;
      navigateTo("squad");
    }
  } else if (currentScreen === "controllers") {
    // On Controllers Live Tester, hold SELECT + press START (or vice versa) to return to Main Menu so A/East stays testable!
    if ((gp.start && selectEdge) || (gp.select && startEdge)) {
      prevPadNavTick = nowMs + 320;
      navigateTo("menu");
    }
  } else if (gp.east) {
    prevPadNavTick = nowMs + 320;
    navigateTo("menu");
  }
}

// --- Background Attract Match & Match Session Lifecycle ---

function _mkOfflineAttrs(pace, drib, pass, shoot, tackle, pos, str, gk) {
  return {
    pace,
    acceleration: pace,
    dribbling: drib,
    passing: pass,
    shooting: shoot,
    tackling: tackle,
    positioning: pos,
    stamina: 85,
    strength: str,
    goalkeeping: gk,
  };
}

const OFFLINE_2026_CLUBS = {
  "man-utd": {
    club_id: "man-utd",
    club_name: "Manchester United",
    short_name: "MUN",
    primary_color: "#DA291C",
    secondary_color: "#FFFFFF",
    formation_5v5: "1-2-1",
    formation_11v11: "4-3-3",
    squad: [
      { footballer_id: "mun-gk-2", name: "S. Lammens", shirt_number: 1, role: "Goalkeeper", attributes: _mkOfflineAttrs(56, 48, 80, 35, 42, 81, 76, 82) },
      { footballer_id: "mun-def-1", name: "L. Martinez", shirt_number: 6, role: "Defender", attributes: _mkOfflineAttrs(77, 70, 83, 60, 86, 86, 82, 15) },
      { footballer_id: "mun-mid-1", name: "K. Mainoo", shirt_number: 37, role: "Midfielder", attributes: _mkOfflineAttrs(80, 86, 84, 75, 78, 82, 72, 12) },
      { footballer_id: "mun-mid-2", name: "B. Fernandes", shirt_number: 8, role: "Midfielder", attributes: _mkOfflineAttrs(76, 85, 90, 86, 70, 84, 72, 12) },
      { footballer_id: "mun-fwd-4", name: "M. Cunha", shirt_number: 10, role: "Forward", attributes: _mkOfflineAttrs(86, 87, 82, 84, 44, 85, 79, 10) },
      { footballer_id: "mun-def-2", name: "D. Dalot", shirt_number: 2, role: "Defender", attributes: _mkOfflineAttrs(85, 72, 79, 60, 80, 80, 77, 15) },
      { footballer_id: "mun-def-3", name: "M. de Ligt", shirt_number: 4, role: "Defender", attributes: _mkOfflineAttrs(74, 68, 75, 58, 85, 85, 88, 15) },
      { footballer_id: "mun-def-5", name: "L. Yoro", shirt_number: 15, role: "Defender", attributes: _mkOfflineAttrs(83, 70, 78, 56, 84, 84, 82, 15) },
      { footballer_id: "mun-mid-4", name: "Y. Tielemans", shirt_number: 18, role: "Midfielder", attributes: _mkOfflineAttrs(71, 82, 86, 80, 75, 82, 74, 12) },
      { footballer_id: "mun-fwd-5", name: "B. Mbeumo", shirt_number: 19, role: "Forward", attributes: _mkOfflineAttrs(89, 85, 80, 85, 44, 85, 77, 10) },
      { footballer_id: "mun-fwd-6", name: "B. Sesko", shirt_number: 30, role: "Forward", attributes: _mkOfflineAttrs(88, 80, 73, 84, 44, 85, 86, 10) },
    ],
  },
  arsenal: {
    club_id: "arsenal",
    club_name: "Arsenal",
    short_name: "ARS",
    primary_color: "#EF0107",
    secondary_color: "#FFFFFF",
    formation_5v5: "1-2-1",
    formation_11v11: "4-3-3",
    squad: [
      { footballer_id: "ars-gk-1", name: "D. Raya", shirt_number: 1, role: "Goalkeeper", attributes: _mkOfflineAttrs(56, 48, 85, 35, 42, 87, 76, 88) },
      { footballer_id: "ars-def-1", name: "W. Saliba", shirt_number: 2, role: "Defender", attributes: _mkOfflineAttrs(84, 72, 82, 58, 90, 90, 87, 15) },
      { footballer_id: "ars-mid-1", name: "D. Rice", shirt_number: 41, role: "Midfielder", attributes: _mkOfflineAttrs(80, 83, 86, 78, 89, 86, 84, 12) },
      { footballer_id: "ars-mid-2", name: "M. Odegaard", shirt_number: 8, role: "Midfielder", attributes: _mkOfflineAttrs(79, 90, 91, 84, 68, 85, 70, 12) },
      { footballer_id: "ars-fwd-1", name: "B. Saka", shirt_number: 7, role: "Forward", attributes: _mkOfflineAttrs(89, 90, 86, 87, 48, 86, 76, 10) },
      { footballer_id: "ars-def-3", name: "Gabriel", shirt_number: 6, role: "Defender", attributes: _mkOfflineAttrs(78, 68, 76, 62, 88, 88, 89, 15) },
      { footballer_id: "ars-def-5", name: "R. Calafiori", shirt_number: 33, role: "Defender", attributes: _mkOfflineAttrs(81, 76, 83, 64, 85, 85, 83, 15) },
      { footballer_id: "ars-def-4", name: "J. Timber", shirt_number: 12, role: "Defender", attributes: _mkOfflineAttrs(84, 78, 82, 60, 84, 84, 80, 15) },
      { footballer_id: "ars-mid-4", name: "B. Guimaraes", shirt_number: 39, role: "Midfielder", attributes: _mkOfflineAttrs(77, 85, 87, 79, 84, 84, 78, 12) },
      { footballer_id: "ars-fwd-4", name: "E. Eze", shirt_number: 10, role: "Forward", attributes: _mkOfflineAttrs(85, 90, 84, 83, 46, 84, 74, 10) },
      { footballer_id: "ars-fwd-5", name: "V. Gyokeres", shirt_number: 14, role: "Forward", attributes: _mkOfflineAttrs(87, 83, 77, 88, 44, 87, 88, 10) },
    ],
  },
};

function buildOffline2026Snapshot(formatName, seed, halfTicks, homeCtrl, awayCtrl, homeClubId, awayClubId) {
  const is11 = formatName === "11v11";
  const rosters = (latestOverview && latestOverview.rosters) || {};
  const buildSide = (cid, fallbackKey) => {
    const r = rosters[cid];
    if (r && Array.isArray(r.squad) && r.squad.length >= 5) {
      const clean = (m) => ({
        footballer_id: m.footballer_id,
        name: m.name,
        shirt_number: m.shirt_number,
        role: m.role,
        attributes: m.attributes,
      });
      const starters = is11
        ? r.squad.slice(0, 11).map(clean)
        : r.squad.filter((m) => m.selected_5v5).slice(0, 5).map(clean);
      const stIds = new Set(starters.map((m) => m.footballer_id));
      const substitutes = r.squad.filter((m) => !stIds.has(m.footballer_id)).map(clean);
      return {
        club_id: r.club_id,
        club_name: r.name,
        short_name: r.short_name,
        primary_color: r.primary_color,
        secondary_color: r.secondary_color,
        formation: is11 ? r.formation_11v11 : r.formation_5v5,
        starters,
        substitutes,
      };
    }
    const fb = OFFLINE_2026_CLUBS[cid] || OFFLINE_2026_CLUBS[fallbackKey];
    const count = is11 ? 11 : 5;
    return {
      club_id: fb.club_id,
      club_name: fb.club_name,
      short_name: fb.short_name,
      primary_color: fb.primary_color,
      secondary_color: fb.secondary_color,
      formation: is11 ? fb.formation_11v11 : fb.formation_5v5,
      starters: fb.squad.slice(0, count),
      substitutes: fb.squad.slice(count),
    };
  };

  return {
    fixture_id: "offline-2026-27",
    gameplay_version: "0.1.0-5v5",
    seed,
    format: is11
      ? {
          name: "11v11",
          players_per_side: 11,
          pitch_length_mm: 105000,
          pitch_width_mm: 68000,
          goal_width_mm: 7320,
          goal_height_mm: 2440,
          penalty_box_depth_mm: 16500,
          penalty_box_width_mm: 40320,
          center_circle_radius_mm: 9150,
        }
      : {
          name: "5v5",
          players_per_side: 5,
          pitch_length_mm: 56000,
          pitch_width_mm: 36000,
          goal_width_mm: 4000,
          goal_height_mm: 2100,
          penalty_box_depth_mm: 9000,
          penalty_box_width_mm: 18000,
          center_circle_radius_mm: 5500,
        },
    rules: {
      half_duration_ticks: halfTicks,
      offsides_enabled: is11,
      fouls_enabled: true,
      extra_time_enabled: false,
      home_controller: homeCtrl,
      away_controller: awayCtrl,
    },
    home: buildSide(homeClubId || "man-utd", "man-utd"),
    away: buildSide(awayClubId || "arsenal", "arsenal"),
  };
}

async function startAttractBackgroundMatch() {
  if (!wasm || attractRestartInFlight) return;
  attractRestartInFlight = true;
  const seq = ++matchSessionSeq;
  isAttractMatch = true;
  activeOfficialSession = null;
  paused = false;
  fulltimeShown = false;

  const seedLo = 20261121;
  try {
    try {
      const res = await fetch("/api/exhibition/snapshot", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          world_id: activeWorldId,
          home_club_id: "liverpool",
          away_club_id: "man-utd",
          format_name: "5v5",
          half_duration_ticks: 14400,
          home_controller: { kind: "Cpu", difficulty: "Hard" },
          away_controller: { kind: "Cpu", difficulty: "Hard" },
          seed: seedLo,
        }),
      });
      if (seq !== matchSessionSeq || !isAttractMatch || currentScreen === "match") return;
      if (res.ok) {
        const data = await res.json();
        if (seq !== matchSessionSeq || !isAttractMatch || currentScreen === "match") return;
        wasmWriteInputText(JSON.stringify(data.snapshot));
        if (wasm.sim_init_from_snapshot_json()) {
          currentSnapshot = data.snapshot;
          engine3d.loadSnapshotScene(currentSnapshot);
          resetSimInterpState();
          return;
        }
      }
    } catch (_e) {
      // Offline 2026–27 snapshot fallback
    }

    if (seq !== matchSessionSeq || !isAttractMatch || currentScreen === "match") return;
    const offlineSnap = buildOffline2026Snapshot(
      "5v5",
      seedLo,
      14400,
      { kind: "Cpu", difficulty: "Hard" },
      { kind: "Cpu", difficulty: "Hard" },
      "man-utd",
      "arsenal"
    );
    wasmWriteInputText(JSON.stringify(offlineSnap));
    if (wasm.sim_init_from_snapshot_json()) {
      currentSnapshot = offlineSnap;
      engine3d.loadSnapshotScene(currentSnapshot);
      resetSimInterpState();
    }
  } finally {
    attractRestartInFlight = false;
  }
}

function getAvailableClubOrder() {
  const rosters = latestOverview && latestOverview.rosters;
  if (rosters && typeof rosters === "object") {
    const clubIds = Object.keys(rosters);
    if (clubIds.length > 0) {
      return CLUB_ORDER.filter((c) => clubIds.includes(c)).concat(
        clubIds.filter((c) => !CLUB_ORDER.includes(c))
      );
    }
  }
  return Object.keys(OFFLINE_2026_CLUBS);
}

function getClubRosterWithFallback(clubId) {
  const rosters = (latestOverview && latestOverview.rosters) || {};
  if (rosters[clubId]) return rosters[clubId];
  const fb = OFFLINE_2026_CLUBS[clubId];
  if (!fb) return null;
  return {
    club_id: fb.club_id,
    name: fb.club_name,
    short_name: fb.short_name,
    stadium: fb.club_id === "arsenal" ? "Emirates Stadium" : "Old Trafford",
    primary_color: fb.primary_color,
    secondary_color: fb.secondary_color,
    formation_5v5: fb.formation_5v5,
    formation_11v11: fb.formation_11v11,
    squad: fb.squad.map((p, idx) => ({
      ...p,
      overall: Math.round(
        (p.attributes.pace + p.attributes.passing + p.attributes.shooting + p.attributes.positioning) / 4
      ),
      selected_5v5: idx < 5,
      default_starter_5v5: idx < 5,
    })),
  };
}

function getOfficialFixtureRepresentation(fx) {
  const assignments = (latestOverview && latestOverview.assignments) || [];
  const homeClubId = fx.homeClubId || (fx.home_club && fx.home_club.club_id) || fx.home_club_id;
  const awayClubId = fx.awayClubId || (fx.away_club && fx.away_club.club_id) || fx.away_club_id;
  const homeAsgn = assignments.find((a) => a.club_id === homeClubId) || null;
  const awayAsgn = assignments.find((a) => a.club_id === awayClubId) || null;
  const homeRep = Boolean(homeAsgn);
  const awayRep = Boolean(awayAsgn);
  const actorId =
    (homeAsgn && homeAsgn.participant_id) ||
    (awayAsgn && awayAsgn.participant_id) ||
    "part-alex";
  return { homeClubId, awayClubId, homeRep, awayRep, actorId };
}

function getAllowedOfficialSideIndices(fx) {
  if (!fx || fx.result_source === "played_vs_friend") return [0, 1, 2, 3];
  const rep = getOfficialFixtureRepresentation(fx);
  if (rep.awayRep && !rep.homeRep) return [1, 3];
  if (rep.homeRep && !rep.awayRep) return [0, 3];
  return [0, 1, 3];
}

function defaultOfficialSideIdx(fx) {
  if (!fx) return 0;
  if (fx.result_source === "played_vs_friend") return 2;
  const rep = getOfficialFixtureRepresentation(fx);
  if (rep.awayRep && !rep.homeRep) return 1;
  return 0;
}

function normalizeOfficialSideIdx(fx, idx) {
  if (!fx) return idx;
  const allowed = getAllowedOfficialSideIndices(fx);
  return allowed.includes(idx) ? idx : defaultOfficialSideIdx(fx);
}

function sideIdxFromSnapshotRules(rules) {
  if (!rules) return pmSideIdx;
  const hHuman = rules.home_controller && rules.home_controller.kind === "Human";
  const aHuman = rules.away_controller && rules.away_controller.kind === "Human";
  if (hHuman && aHuman) return 2;
  if (!hHuman && aHuman) return 1;
  if (hHuman && !aHuman) return 0;
  return 3;
}

function resolvePreMatchControllers() {
  const diff = DIFF_OPTIONS[pmDiffIdx];
  const sideMode = SIDE_MODES[pmSideIdx].id;
  if (sideMode === "local_2p") {
    return {
      homeSpec: { kind: "Human" },
      awaySpec: { kind: "Human" },
      homeCode: 0,
      awayCode: 0,
    };
  }
  if (sideMode === "away_vs_cpu") {
    return {
      homeSpec: { kind: "Cpu", difficulty: diff.name },
      awaySpec: { kind: "Human" },
      homeCode: diff.code,
      awayCode: 0,
    };
  }
  if (sideMode === "cpu_vs_cpu") {
    return {
      homeSpec: { kind: "Cpu", difficulty: diff.name },
      awaySpec: { kind: "Cpu", difficulty: diff.name },
      homeCode: diff.code,
      awayCode: diff.code,
    };
  }
  return {
    homeSpec: { kind: "Human" },
    awaySpec: { kind: "Cpu", difficulty: diff.name },
    homeCode: 0,
    awayCode: diff.code,
  };
}

function setupLiveMatchHud(snapshot, contextLabel) {
  ++matchSessionSeq;
  currentSnapshot = snapshot;
  isAttractMatch = false;
  paused = false;
  fulltimeShown = false;
  replayBuffer = [];
  replayActive = false;
  goalBannerTicks = 0;
  lastShootoutScore = null;
  homePossTicks = 1;
  awayPossTicks = 1;
  matchEventsLog = [];
  queuedSubReserved = [0, 0];
  subsUsedCount = [0, 0];
  subsArmedSlot = null;
  subsSide = SIDE_MODES[pmSideIdx].id === "away_vs_cpu" ? 1 : 0;

  document.getElementById("pause-modal").hidden = true;
  document.getElementById("pause-subs-panel").hidden = true;
  document.getElementById("pause-modal-card").classList.remove("subs-open");
  document.getElementById("fulltime-modal").hidden = true;
  document.getElementById("replay-bar").hidden = true;
  document.getElementById("radar-wrap").hidden = false;
  document.getElementById("goal-banner").hidden = true;
  document.getElementById("verify-banner").hidden = true;
  document.getElementById("hud-home-card").hidden = true;
  document.getElementById("hud-away-card").hidden = true;
  syncWeatherLabels();

  const kits = resolveMatchKits(
    snapshot.home.club_id,
    snapshot.away.club_id,
    snapshot.home.primary_color,
    snapshot.away.primary_color
  );

  document.getElementById("sb-context").textContent = contextLabel;
  document.getElementById("sb-home-code").textContent = snapshot.home.short_name;
  document.getElementById("sb-away-code").textContent = snapshot.away.short_name;
  document.getElementById("sb-home-swatch").style.background = kits.homeOutfield.shirt;
  document.getElementById("sb-away-swatch").style.background = kits.awayOutfield.shirt;

  engine3d.loadSnapshotScene(snapshot);
  resetSimInterpState();
  navigateTo("match");
  GameAudio.playWhistle(false);
}

async function launchSelectedMatch() {
  if (!wasm) return;
  ++matchSessionSeq;
  GameAudio.uiConfirm();

  const fmtName = FORMAT_OPTIONS[pmFmtIdx].id;
  const halfTicks = DURATION_OPTIONS[pmDurIdx].ticks;

  // Official Parallel Season Fixture Session
  if (pendingOfficialFixture) {
    const fx = pendingOfficialFixture;
    pmSideIdx = normalizeOfficialSideIdx(fx, pmSideIdx);
    const ctrls = resolvePreMatchControllers();
    const rep = getOfficialFixtureRepresentation(fx);

    const res = await fetch(`/api/fixtures/${encodeURIComponent(fx.id)}/sessions`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        actor_id: rep.actorId,
        format_name: fmtName,
        half_duration_ticks: halfTicks,
        home_controller: ctrls.homeSpec,
        away_controller: ctrls.awaySpec,
      }),
    });
    if (!res.ok) {
      const err = await res.json().catch(() => ({}));
      alert(`Could not open official match session: ${err.detail || res.statusText}`);
      return;
    }
    const payload = await res.json();
    wasmWriteInputText(JSON.stringify(payload.snapshot));
    if (!wasm.sim_init_from_snapshot_json()) {
      alert(`Engine snapshot error: ${wasmReadText()}`);
      return;
    }
    pmSideIdx = sideIdxFromSnapshotRules(payload.snapshot.rules);
    activeOfficialSession = {
      sessionId: payload.session_id,
      fixtureId: payload.fixture_id,
      actorId: rep.actorId,
      submitting: false,
      completed: false,
    };
    const etTag = payload.snapshot.rules.extra_time_enabled ? " · ET + PENS" : "";
    setupLiveMatchHud(payload.snapshot, `OFFICIAL ${fx.stageLabel} · ${payload.snapshot.format.name}${etTag}`);
    refreshWorldOverview();
    return;
  }

  const ctrls = resolvePreMatchControllers();

  // Exhibition Kick Off Match
  activeOfficialSession = null;
  const seedLo = (Date.now() & 0x7fffffff) >>> 0;
  try {
    const res = await fetch("/api/exhibition/snapshot", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        world_id: activeWorldId,
        home_club_id: pmHomeClub,
        away_club_id: pmAwayClub,
        format_name: fmtName,
        half_duration_ticks: halfTicks,
        home_controller: ctrls.homeSpec,
        away_controller: ctrls.awaySpec,
        seed: seedLo,
      }),
    });
    if (res.ok) {
      const data = await res.json();
      wasmWriteInputText(JSON.stringify(data.snapshot));
      if (wasm.sim_init_from_snapshot_json()) {
        setupLiveMatchHud(
          data.snapshot,
          `EXHIBITION · ${data.snapshot.format.name} · ${data.snapshot.home.formation} vs ${data.snapshot.away.formation}`
        );
        return;
      }
    }
  } catch (_e) {
    // Offline 2026–27 snapshot fallback
  }

  const offlineSnap = buildOffline2026Snapshot(
    fmtName,
    seedLo,
    halfTicks,
    ctrls.homeSpec,
    ctrls.awaySpec,
    pmHomeClub,
    pmAwayClub
  );
  wasmWriteInputText(JSON.stringify(offlineSnap));
  if (wasm.sim_init_from_snapshot_json()) {
    setupLiveMatchHud(offlineSnap, `EXHIBITION · ${offlineSnap.format.name}`);
  }
}

async function submitOfficialSessionResult() {
  if (!wasm || !activeOfficialSession || activeOfficialSession.submitting || activeOfficialSession.completed) {
    return;
  }
  wasm.sim_export_summary_json();
  let summary = JSON.parse(wasmReadText());
  if (!summary.finished) {
    wasm.sim_auto_step(90000);
    wasm.sim_export_summary_json();
    summary = JSON.parse(wasmReadText());
  }
  lastShootoutScore = summary.shootout_score || null;

  const btn = document.getElementById("btn-submit-session");
  activeOfficialSession.submitting = true;
  btn.textContent = "VERIFYING REPLAY LOG…";

  wasm.sim_export_input_log_json();
  const log = JSON.parse(wasmReadText());
  const verifyBanner = document.getElementById("verify-banner");

  try {
    const res = await fetch(`/api/sessions/${encodeURIComponent(activeOfficialSession.sessionId)}/complete`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        actor_id: activeOfficialSession.actorId || "part-alex",
        frames_hex: log.frames_hex,
        claimed_final_hash_hex: log.claimed_final_hash_hex,
        claimed_score: log.claimed_score,
        hash_checkpoints: log.hash_checkpoints,
      }),
    });
    const data = await res.json();
    if (!res.ok) {
      throw new Error(data.detail || "Host psfc-replay unavailable or rejected log");
    }
    activeOfficialSession.completed = true;
    btn.hidden = true;
    const ar = data.accepted_result;
    const penTag =
      ar.shootout_home !== null && ar.shootout_home !== undefined
        ? ` (${ar.shootout_home}-${ar.shootout_away} PENS)`
        : ar.went_to_extra_time
          ? " (AET)"
          : "";
    verifyBanner.hidden = false;
    verifyBanner.innerHTML = `<strong>OFFICIAL RESULT VERIFIED &amp; RECORDED:</strong> ${currentSnapshot.home.short_name} ${ar.home_goals} - ${ar.away_goals} ${currentSnapshot.away.short_name}${penTag}`;
    await refreshWorldOverview();
  } catch (err) {
    verifyBanner.hidden = false;
    verifyBanner.innerHTML = `<strong>Verification Notice:</strong> ${err.message}`;
  } finally {
    activeOfficialSession.submitting = false;
    btn.textContent = "VERIFY & RECORD OFFICIAL RESULT";
  }
}

// --- Pre-Match Screen Renderer ---

function computeClubRatings(roster) {
  if (!roster || !roster.squad) return { att: 82, mid: 82, def: 81 };
  const byRole = (r) => roster.squad.filter((p) => p.role === r).map((p) => p.overall);
  const avg = (arr, fb) => (arr.length ? Math.round(arr.reduce((a, b) => a + b, 0) / arr.length) : fb);
  return {
    att: avg(byRole("Forward"), 83),
    mid: avg(byRole("Midfielder"), 83),
    def: avg([...byRole("Defender"), ...byRole("Goalkeeper")], 82),
  };
}

function renderPreMatchTeamCard(prefix, clubId, kitColorHex, kitTextColorHex) {
  const r = getClubRosterWithFallback(clubId);
  if (!r) return;

  const crest = document.getElementById(`pm-${prefix}-crest`);
  crest.textContent = r.short_name;
  crest.style.background = kitColorHex || r.primary_color;
  crest.style.color = kitTextColorHex || "#ffffff";

  document.getElementById(`pm-${prefix}-name`).textContent = r.name;
  document.getElementById(`pm-${prefix}-stadium`).textContent = r.stadium;

  const fmtName = FORMAT_OPTIONS[pmFmtIdx].id;
  document.getElementById(`pm-${prefix}-fmt`).textContent =
    fmtName === "11v11" ? r.formation_11v11 : r.formation_5v5;

  const rat = computeClubRatings(r);
  document.getElementById(`pm-${prefix}-ratings`).innerHTML = `
    <div class="pm-rat-box"><div class="pm-rat-num">${rat.att}</div><div class="pm-rat-lbl">ATT</div></div>
    <div class="pm-rat-box"><div class="pm-rat-num">${rat.mid}</div><div class="pm-rat-lbl">MID</div></div>
    <div class="pm-rat-box"><div class="pm-rat-num">${rat.def}</div><div class="pm-rat-lbl">DEF</div></div>
  `;

  const starters = fmtName === "11v11" ? r.squad : r.squad.filter((p) => p.selected_5v5);
  const ul = document.getElementById(`pm-${prefix}-starters`);
  ul.innerHTML = starters
    .map(
      (p) => `
      <li>
        <span><strong>#${p.shirt_number}</strong> ${p.name}</span>
        <span class="ovr-pill">${p.overall}</span>
      </li>`
    )
    .join("");
}

function renderPreMatchScreen() {
  const locked = Boolean(pendingOfficialFixture);

  if (locked) {
    pmHomeClub = pendingOfficialFixture.homeClubId;
    pmAwayClub = pendingOfficialFixture.awayClubId;
    pmSideIdx = normalizeOfficialSideIdx(pendingOfficialFixture, pmSideIdx);
  } else {
    const availableClubs = getAvailableClubOrder();
    if (!availableClubs.includes(pmHomeClub)) {
      pmHomeClub = availableClubs[0] || "man-utd";
    }
    if (!availableClubs.includes(pmAwayClub) || pmAwayClub === pmHomeClub) {
      pmAwayClub = availableClubs.find((c) => c !== pmHomeClub) || pmHomeClub;
    }
  }

  document.getElementById("pm-home-prev").disabled = locked;
  document.getElementById("pm-home-next").disabled = locked;
  document.getElementById("pm-away-prev").disabled = locked;
  document.getElementById("pm-away-next").disabled = locked;

  document.getElementById("pm-context-banner").textContent = locked
    ? `OFFICIAL · ${pendingOfficialFixture.stageLabel}`
    : "EXHIBITION MATCH";

  const hRoster = getClubRosterWithFallback(pmHomeClub);
  const aRoster = getClubRosterWithFallback(pmAwayClub);
  const kits = resolveMatchKits(
    pmHomeClub,
    pmAwayClub,
    hRoster && hRoster.primary_color,
    aRoster && aRoster.primary_color
  );

  const isAwayClash = aRoster && kits.awayOutfield.shirt.toLowerCase() !== aRoster.primary_color.toLowerCase();
  document.getElementById("pm-away-kit-tag").textContent = isAwayClash ? "CLASH AWAY KIT" : "HOME KIT";

  renderPreMatchTeamCard("home", pmHomeClub, kits.homeOutfield.shirt, kits.homeOutfield.numberColor);
  renderPreMatchTeamCard("away", pmAwayClub, kits.awayOutfield.shirt, kits.awayOutfield.numberColor);

  document.getElementById("pm-side-val").textContent = SIDE_MODES[pmSideIdx].label;
  document.getElementById("pm-fmt-val").textContent = FORMAT_OPTIONS[pmFmtIdx].label;
  document.getElementById("pm-dur-val").textContent = DURATION_OPTIONS[pmDurIdx].label;
  document.getElementById("pm-diff-val").textContent = DIFF_OPTIONS[pmDiffIdx].label;
  document.getElementById("pm-cam-val").textContent = CAMERA_LABELS[engine3d.cameraMode];
  syncWeatherLabels();

  engine3d.updatePreMatchShowcase(hRoster, aRoster);
}

// --- Match HUD, Weather, Live Substitutions, Radar, Pause & Instant Replay ---

function showGoalBroadcastBanner(title, sub, ticks = 150) {
  document.getElementById("goal-banner-title").textContent = title;
  document.getElementById("goal-banner-sub").textContent = sub;
  document.getElementById("goal-banner").hidden = false;
  goalBannerTicks = ticks;
}

function syncWeatherLabels() {
  const mode = engine3d.weatherMode || "night";
  const full = WEATHER_LABELS[mode] || "FLOODLIT NIGHT";
  const pmEl = document.getElementById("pm-wx-val");
  if (pmEl) pmEl.textContent = full;
  const hudEl = document.getElementById("btn-hud-wx");
  if (hudEl) hudEl.textContent = `WX: ${mode.toUpperCase()}`;
  const pauseEl = document.getElementById("btn-pause-wx");
  if (pauseEl) pauseEl.textContent = `WEATHER: ${full}`;
}

function cycleMatchWeather() {
  engine3d.cycleWeatherPreset();
  syncWeatherLabels();
  GameAudio.uiTick(580);
}

function cycleMatchCamera() {
  const mode = engine3d.cycleCameraPreset();
  const label = CAMERA_LABELS[mode];
  document.getElementById("btn-hud-cam").textContent = `CAM: ${mode.toUpperCase()}`;
  document.getElementById("btn-pause-cam").textContent = `CAMERA ANGLE: ${label}`;
  GameAudio.uiTick(600);
}

function togglePauseModal() {
  paused = !paused;
  const modal = document.getElementById("pause-modal");
  modal.hidden = !paused;
  if (paused && currentSnapshot) {
    document.getElementById("pause-scoreline").textContent =
      `${currentSnapshot.home.short_name} ${document.getElementById("sb-score").textContent} ${currentSnapshot.away.short_name}`;
    if (!document.getElementById("pause-subs-panel").hidden) {
      renderPauseSubsPanel();
    }
  }
  GameAudio.uiTick(paused ? 420 : 620);
}

function togglePauseSubsDrawer(forceOpen = null) {
  const panel = document.getElementById("pause-subs-panel");
  const card = document.getElementById("pause-modal-card");
  const nextOpen = forceOpen !== null ? forceOpen : panel.hidden;
  panel.hidden = !nextOpen;
  card.classList.toggle("subs-open", nextOpen);
  if (nextOpen) {
    subsArmedSlot = null;
    renderPauseSubsPanel();
  }
  GameAudio.uiTick(540);
}

function renderPauseSubsPanel() {
  if (!currentSnapshot) return;
  if (wasm && wasm.sim_export_teams_json && wasm.sim_export_teams_json() > 2) {
    try {
      const liveTeams = JSON.parse(wasmReadText());
      if (liveTeams.home && liveTeams.away) {
        currentSnapshot.home = liveTeams.home;
        currentSnapshot.away = liveTeams.away;
        if (Array.isArray(liveTeams.subs_used)) subsUsedCount = liveTeams.subs_used;
      }
    } catch (_e) {
      // Keep snapshot teams
    }
  }

  const homeBtn = document.getElementById("btn-subs-home");
  const awayBtn = document.getElementById("btn-subs-away");
  homeBtn.textContent = currentSnapshot.home.short_name || "HOME";
  awayBtn.textContent = currentSnapshot.away.short_name || "AWAY";
  homeBtn.classList.toggle("active", subsSide === 0);
  awayBtn.classList.toggle("active", subsSide === 1);

  const used = subsUsedCount[subsSide] || 0;
  const rem = Math.max(0, 3 - used);
  document.getElementById("subs-remaining-pill").textContent = `SUBS LEFT: ${rem} / 3`;

  const team = subsSide === 0 ? currentSnapshot.home : currentSnapshot.away;
  const starters = team.starters || [];
  const bench = team.substitutes || team.bench || [];
  const packed = currSimPacked || readPackedState();
  const pps = packed ? packed[6] : starters.length;

  const hintEl = document.getElementById("subs-status-hint");
  if (!bench.length) {
    hintEl.textContent = "All 11 official squad members are already on the pitch in 11v11 mode.";
  } else if (rem <= 0) {
    hintEl.textContent = `${team.short_name} has used all 3 substitutions in this match.`;
  } else if (subsArmedSlot !== null && starters[subsArmedSlot]) {
    const outM = starters[subsArmedSlot];
    const reqRole = subsArmedSlot === 0 ? "Goalkeeper" : "Outfielder";
    hintEl.textContent = `Subbing OFF #${outM.shirt_number} ${outM.name} — click a ${reqRole} on the bench to bring ON!`;
  } else {
    hintEl.textContent = "Click any active player on the pitch, then click a 100% fresh bench reserve to swap.";
  }

  const sList = document.getElementById("subs-starters-list");
  sList.innerHTML = "";
  starters.slice(0, pps).forEach((m, slotIdx) => {
    let stamPct = 100;
    let sentOff = false;
    let hasYellow = false;
    if (packed && packed.length >= 18 + (subsSide * pps + slotIdx + 1) * 10) {
      const flags = packed[18 + (subsSide * pps + slotIdx) * 10 + 9] >>> 0;
      stamPct = Math.round(((flags >>> 4) & 0x7ff) / 10);
      sentOff = ((flags >>> 26) & 1) === 1;
      hasYellow = ((flags >>> 27) & 1) === 1;
    }
    const rb = roleBadgeMeta(m.role);
    const stamCol = stamPct < 35 ? "#ef4444" : stamPct < 60 ? "#facc15" : "#22c55e";
    const cardBadge = sentOff
      ? `<span class="fph-card card-red" title="Sent Off"></span>`
      : hasYellow
        ? `<span class="fph-card" title="Yellow Card"></span>`
        : "";
    const disabled = sentOff || rem <= 0 || !bench.length;

    const row = document.createElement("div");
    row.className = `subs-row${subsArmedSlot === slotIdx ? " armed" : ""}${disabled ? " disabled" : ""}`;
    row.innerHTML = `
      <div class="subs-row-left">
        <strong>#${m.shirt_number}</strong>
        <span class="role-tag ${rb.cls}">${rb.label}</span>
        <span class="subs-row-name">${m.name}</span>
        ${cardBadge}
      </div>
      <div class="subs-row-right">
        <span>${stamPct}%</span>
        <span class="subs-stam-mini"><span style="width:${stamPct}%;background:${stamCol}"></span></span>
      </div>
    `;
    if (!disabled) {
      row.addEventListener("click", () => {
        subsArmedSlot = subsArmedSlot === slotIdx ? null : slotIdx;
        GameAudio.uiTick(520);
        renderPauseSubsPanel();
      });
    }
    sList.appendChild(row);
  });

  const bList = document.getElementById("subs-bench-list");
  bList.innerHTML = "";
  if (!bench.length) {
    bList.innerHTML = `<div class="subs-row disabled"><span>No unused reserves in 11v11</span></div>`;
    return;
  }

  bench.forEach((m, benchIdx) => {
    const rb = roleBadgeMeta(m.role);
    const isGk = m.role === "Goalkeeper";
    const a = m.attributes || {};
    const ovr =
      m.overall ||
      (isGk
        ? Math.round(((a.goalkeeping || 84) * 2 + (a.positioning || 82) + (a.passing || 76)) / 4)
        : Math.round(
            ((a.pace || 82) +
              (a.shooting || 80) +
              (a.passing || 82) +
              (a.dribbling || 82) +
              (a.tackling || 75) +
              (a.positioning || 82)) /
              6
          ));
    const roleMismatch =
      subsArmedSlot !== null && ((subsArmedSlot === 0 && !isGk) || (subsArmedSlot > 0 && isGk));
    const disabled = rem <= 0 || roleMismatch;

    const row = document.createElement("div");
    row.className = `subs-row${disabled ? " disabled" : ""}`;
    row.innerHTML = `
      <div class="subs-row-left">
        <strong>#${m.shirt_number}</strong>
        <span class="role-tag ${rb.cls}">${rb.label}</span>
        <span class="subs-row-name">${m.name}</span>
      </div>
      <div class="subs-row-right">
        <span class="ovr-pill">${ovr}</span>
        <span style="color:#4ade80;font-weight:800">100%</span>
      </div>
    `;
    if (!disabled) {
      row.addEventListener("click", () => {
        let targetSlot = subsArmedSlot;
        if (targetSlot === null) {
          // Auto-pick the most fatigued legal starter if user clicked bench directly!
          if (isGk) {
            targetSlot = 0;
          } else {
            let minStam = 9999;
            targetSlot = 1;
            for (let k = 1; k < pps; k++) {
              const fl = packed ? packed[18 + (subsSide * pps + k) * 10 + 9] >>> 0 : 1000 << 4;
              if (((fl >>> 26) & 1) === 1) continue;
              const st = (fl >>> 4) & 0x7ff;
              if (st < minStam) {
                minStam = st;
                targetSlot = k;
              }
            }
          }
        }
        queuedSubReserved[subsSide] = 0x8000 | ((targetSlot & 0x1f) << 5) | (benchIdx & 0x1f);
        subsArmedSlot = null;
        GameAudio.uiConfirm();
        togglePauseModal(); // Unpauses immediately so next 60Hz tick applies the substitution!
      });
    }
    bList.appendChild(row);
  });
}

function toggleInstantReplay() {
  if (replayBuffer.length < 2) return;
  if (replayActive) {
    exitInstantReplay();
    return;
  }
  paused = true;
  document.getElementById("pause-modal").hidden = true;
  replayActive = true;
  replayCursor = Math.max(0, replayBuffer.length - 180);
  const bar = document.getElementById("replay-bar");
  const scrub = document.getElementById("replay-scrub");
  scrub.max = String(Math.max(1, replayBuffer.length - 1));
  scrub.value = String(Math.floor(replayCursor));
  bar.hidden = false;
  document.getElementById("radar-wrap").hidden = true;
  GameAudio.uiConfirm();
}

function exitInstantReplay() {
  replayActive = false;
  paused = false;
  document.getElementById("replay-bar").hidden = true;
  document.getElementById("radar-wrap").hidden = false;
  GameAudio.uiTick(500);
}

function openFullTimeSummaryModal() {
  if (fulltimeShown || !wasm || !currentSnapshot) return;
  fulltimeShown = true;
  wasm.sim_export_summary_json();
  const summary = JSON.parse(wasmReadText());
  lastShootoutScore = summary.shootout_score || null;

  const penStr = lastShootoutScore ? ` (${lastShootoutScore[0]}-${lastShootoutScore[1]}P)` : "";
  document.getElementById("ft-home-name").textContent = currentSnapshot.home.short_name;
  document.getElementById("ft-away-name").textContent = currentSnapshot.away.short_name;
  document.getElementById("ft-score").textContent = `${summary.score[0]} - ${summary.score[1]}${penStr}`;

  const hPct = Math.round((homePossTicks / (homePossTicks + awayPossTicks)) * 100);
  const sumStat = (arr, field) => (arr || []).reduce((acc, s) => acc + (s[field] || 0), 0);
  const hShots = sumStat(summary.home_player_stats, "shots");
  const aShots = sumStat(summary.away_player_stats, "shots");
  const hSaves = sumStat(summary.home_player_stats, "saves");
  const aSaves = sumStat(summary.away_player_stats, "saves");

  document.getElementById("ft-stats-grid").innerHTML = `
    <div class="ft-stat-box"><div class="ft-stat-val">${hPct}% - ${100 - hPct}%</div><div class="ft-stat-lbl">Possession</div></div>
    <div class="ft-stat-box"><div class="ft-stat-val">${hShots} - ${aShots}</div><div class="ft-stat-lbl">Total Shots</div></div>
    <div class="ft-stat-box"><div class="ft-stat-val">${hSaves} - ${aSaves}</div><div class="ft-stat-lbl">GK Saves</div></div>
  `;

  const evUl = document.getElementById("ft-events-list");
  evUl.innerHTML = matchEventsLog.length
    ? matchEventsLog.slice(0, 8).map((e) => `<li><strong>${e.minute}'</strong> ${e.detail}</li>`).join("")
    : "<li>Clean tactical contest — no goals recorded in regulation.</li>";

  const submitBtn = document.getElementById("btn-submit-session");
  submitBtn.hidden = !(activeOfficialSession && !activeOfficialSession.completed);
  document.getElementById("fulltime-modal").hidden = false;

  if (activeOfficialSession && !activeOfficialSession.completed) {
    submitOfficialSessionResult();
  }
}

function drainLiveMatchEvents(packed) {
  const len = wasm.sim_drain_events_json();
  if (len <= 2) return;
  const events = JSON.parse(wasmReadText());
  if (isAttractMatch) return;

  const halfLmm = currentSnapshot ? currentSnapshot.format.pitch_length_mm * 0.5 : 21000;
  const pan = packed ? Math.max(-0.78, Math.min(0.78, packed[9] / Math.max(1000, halfLmm))) : 0;

  for (const ev of events) {
    const kind = String(ev.event_type || "").toLowerCase();
    matchEventsLog.unshift(ev);

    if (kind === "goal" || kind === "shootout_goal") {
      GameAudio.playGoalCrowdRoar(pan);
      const impZ = packed ? packed[10] / 1000 : 0;
      const impY = packed ? packed[11] / 1000 : 1.25;
      engine3d.triggerGoalEffect(packed ? packed[15] : -1, impZ, impY);
      showGoalBroadcastBanner(kind === "shootout_goal" ? "PENALTY SCORED!" : "GOAL!", `${ev.minute}' · ${ev.detail}`, 175);
    } else if (kind === "woodwork") {
      GameAudio.playWoodworkClang(pan);
      showGoalBroadcastBanner("OFF THE WOODWORK!", `${ev.minute}' · ${ev.detail}`, 75);
    } else if (kind === "yellow_card") {
      GameAudio.playWhistle(true, pan);
      showGoalBroadcastBanner("YELLOW CARD", `${ev.minute}' · ${ev.detail}`, 115);
    } else if (kind === "red_card") {
      GameAudio.playWhistle(true, pan);
      showGoalBroadcastBanner("RED CARD — SENT OFF!", `${ev.minute}' · ${ev.detail}`, 140);
    } else if (kind === "substitution") {
      GameAudio.uiConfirm();
      if (wasm.sim_export_teams_json && wasm.sim_export_teams_json() > 2 && currentSnapshot) {
        try {
          const live = JSON.parse(wasmReadText());
          currentSnapshot.home = live.home;
          currentSnapshot.away = live.away;
          if (Array.isArray(live.subs_used)) subsUsedCount = live.subs_used;
          const subSide = ev.team === "Away" ? 1 : 0;
          const teamObj = subSide === 0 ? currentSnapshot.home : currentSnapshot.away;
          const subInId = ev.footballer_id || ev.player_id;
          const slotIdx = (teamObj.starters || []).findIndex((p) => p.footballer_id === subInId);
          if (slotIdx >= 0) {
            engine3d.replacePlayerActor(subSide, slotIdx, teamObj.starters[slotIdx], teamObj);
          }
        } catch (_e) {
          // Ignore JSON sync error
        }
      }
      showGoalBroadcastBanner("SUBSTITUTION", `${ev.minute}' · ${ev.detail}`, 130);
    } else if (kind === "shot") {
      GameAudio.playKick(0.95, pan);
    } else if (kind === "pass") {
      GameAudio.playKick(0.45, pan);
    } else if (kind === "save") {
      GameAudio.playSaveSlap(pan);
    } else if (kind === "extra_time_start") {
      GameAudio.playWhistle(true, pan);
      showGoalBroadcastBanner("EXTRA TIME", ev.detail, 120);
    } else if (kind === "penalty_shootout_start") {
      GameAudio.playWhistle(true, pan);
      showGoalBroadcastBanner("PENALTY SHOOTOUT", ev.detail, 140);
    } else if (kind === "foul" || kind === "kick_off" || kind === "half_time") {
      GameAudio.playWhistle(false, pan);
    } else if (kind === "full_time") {
      GameAudio.playWhistle(true, 0);
      openFullTimeSummaryModal();
    }
  }
}

function updatePlayerCornerHuds(packed) {
  if (!currentSnapshot) return;
  const pps = packed[6];
  const homeCtrl = packed[7];
  const awayCtrl = packed[8];

  const updateCorner = (side, ctrlIdx, prefix) => {
    const n = side * pps + ctrlIdx;
    if (n < 0 || n >= pps * 2) return;
    const b = 18 + n * 10;
    const shirt = packed[b + 2];
    const flags = packed[b + 9] >>> 0;
    const stamina = ((flags >>> 4) & 0x7ff) / 1000;
    const charge = ((flags >>> 15) & 0x7ff) / 1000;
    const sentOff = ((flags >>> 26) & 1) === 1;
    const hasYellow = ((flags >>> 27) & 1) === 1;

    const team = side === 0 ? currentSnapshot.home : currentSnapshot.away;
    const member = team.starters && team.starters[ctrlIdx];
    const surname = member && member.name ? member.name.trim().split(/\s+/).slice(-1)[0].toUpperCase() : `PLAYER ${shirt}`;

    document.getElementById(`hud-${prefix}-num`).textContent = String(shirt);
    document.getElementById(`hud-${prefix}-name`).textContent = surname;
    const cardEl = document.getElementById(`hud-${prefix}-card`);
    if (cardEl) {
      cardEl.hidden = !(sentOff || hasYellow);
      cardEl.classList.toggle("card-red", sentOff);
    }
    const stamEl = document.getElementById(`hud-${prefix}-stam`);
    stamEl.style.width = `${Math.round(stamina * 100)}%`;
    stamEl.style.background = stamina < 0.3 ? "#ef4444" : stamina < 0.55 ? "#facc15" : "#22c55e";
    document.getElementById(`hud-${prefix}-pow`).style.width = `${Math.round(charge * 100)}%`;
  };

  updateCorner(0, homeCtrl, "home");
  updateCorner(1, awayCtrl, "away");
}

function drawPitchRadar(packed) {
  if (!currentSnapshot) return;
  const W = radarCanvas.width;
  const H = radarCanvas.height;
  const halfL = currentSnapshot.format.pitch_length_mm / 2000;
  const halfW = currentSnapshot.format.pitch_width_mm / 2000;

  radarCtx.clearRect(0, 0, W, H);

  // Alternating botanical mower stripes (`IMG_0350.webp`!)
  const stripes = 8;
  const sw = (W - 8) / stripes;
  for (let s = 0; s < stripes; s++) {
    radarCtx.fillStyle = s % 2 === 0 ? "rgba(32, 96, 36, 0.78)" : "rgba(42, 114, 45, 0.78)";
    radarCtx.fillRect(4 + s * sw, 4, sw + 0.5, H - 8);
  }

  radarCtx.strokeStyle = "rgba(248, 250, 252, 0.58)";
  radarCtx.lineWidth = 1.25;
  radarCtx.strokeRect(4, 4, W - 8, H - 8);
  radarCtx.beginPath();
  radarCtx.moveTo(W * 0.5, 4);
  radarCtx.lineTo(W * 0.5, H - 4);
  radarCtx.stroke();
  radarCtx.beginPath();
  radarCtx.arc(W * 0.5, H * 0.5, 14, 0, Math.PI * 2);
  radarCtx.stroke();

  // Penalty boxes on radar
  const boxW = (W - 8) * 0.16;
  const boxH = (H - 8) * 0.54;
  radarCtx.strokeRect(4, (H - boxH) * 0.5, boxW, boxH);
  radarCtx.strokeRect(W - 4 - boxW, (H - boxH) * 0.5, boxW, boxH);

  const mapX = (wx) => 4 + ((wx + halfL) / (halfL * 2)) * (W - 8);
  const mapY = (wy) => 4 + ((wy + halfW) / (halfW * 2)) * (H - 8);

  const kits = engine3d.stadiumData ? engine3d.stadiumData.kits : null;
  const homeCol = kits ? kits.homeOutfield.shirt : "#ef4444";
  const awayCol = kits ? kits.awayOutfield.shirt : "#38bdf8";

  const pps = packed[6];
  const homeCtrl = packed[7];
  const awayCtrl = packed[8];

  for (let n = 0; n < pps * 2; n++) {
    const b = 18 + n * 10;
    const side = packed[b];
    const idx = packed[b + 1];
    const flags = packed[b + 9] >>> 0;
    const sentOff = ((flags >>> 26) & 1) === 1;
    if (sentOff) continue;
    const px = mapX(packed[b + 4] / 1000);
    const py = mapY(packed[b + 5] / 1000);

    const isCtrl = side === 0 ? idx === homeCtrl : idx === awayCtrl;
    if (isCtrl) {
      radarCtx.strokeStyle = "#facc15";
      radarCtx.lineWidth = 1.5;
      radarCtx.beginPath();
      radarCtx.arc(px, py, 5.4, 0, Math.PI * 2);
      radarCtx.stroke();
    }

    radarCtx.fillStyle = side === 0 ? homeCol : awayCol;
    radarCtx.beginPath();
    radarCtx.arc(px, py, 3.7, 0, Math.PI * 2);
    radarCtx.fill();
    radarCtx.strokeStyle = "rgba(9, 15, 28, 0.8)";
    radarCtx.lineWidth = 0.9;
    radarCtx.stroke();
  }

  const bx = mapX(packed[9] / 1000);
  const by = mapY(packed[10] / 1000);
  radarCtx.fillStyle = "#ffffff";
  radarCtx.beginPath();
  radarCtx.arc(bx, by, 3.4, 0, Math.PI * 2);
  radarCtx.fill();
  radarCtx.strokeStyle = "#090f1c";
  radarCtx.lineWidth = 1.1;
  radarCtx.stroke();
}

// --- Main 60 Hz Game & Render Loop ---

function formatHalfBadge(half, phase) {
  if (phase === 12) return "FT";
  if (half === 3) return "ET1";
  if (half === 4) return "ET2";
  return `${half}H`;
}

let lastLoopMs = performance.now();
let simAccumMs = 0;
let prevSimPacked = null;
let currSimPacked = null;
const SIM_STEP_MS = 1000 / 60;

function resetSimInterpState() {
  simAccumMs = 0;
  const p = readPackedState();
  if (p) {
    currSimPacked = new Int32Array(p);
    prevSimPacked = new Int32Array(p);
  } else {
    currSimPacked = null;
    prevSimPacked = null;
  }
}

function mainLoop(nowMs) {
  const frameDelta = Math.min(100, Math.max(0, nowMs - lastLoopMs));
  lastLoopMs = nowMs;

  syncGamepadConnectionState();
  pollGamepadMenuActions(nowMs);
  updateControllerVisualizer();

  if (wasm && currentSnapshot) {
    if (currentScreen === "match" && replayActive && replayBuffer.length > 0) {
      const idx0 = Math.min(replayBuffer.length - 1, Math.floor(replayCursor));
      const idx1 = Math.min(replayBuffer.length - 1, idx0 + 1);
      const rAlpha = Math.max(0, Math.min(1, replayCursor - idx0));
      const camMode = REPLAY_CAM_MODES[replayCamIdx];
      engine3d.renderFrame(
        replayBuffer[idx1],
        {
          active: true,
          camMode,
          orbitAngle: replayCursor * 0.025 + 0.65,
        },
        replayBuffer[idx0],
        rAlpha
      );
      replayCursor += (frameDelta / SIM_STEP_MS) * 0.5;
      if (replayCursor >= replayBuffer.length) {
        replayCursor = 0;
      }
      document.getElementById("replay-scrub").value = String(Math.floor(replayCursor));
      requestAnimationFrame(mainLoop);
      return;
    }

    if (!paused) {
      simAccumMs += frameDelta;
    }
    const steps = paused ? 0 : Math.min(6, Math.floor(simAccumMs / SIM_STEP_MS));
    if (!paused) {
      simAccumMs -= steps * SIM_STEP_MS;
    }
    const interpAlpha = paused ? 1.0 : Math.min(1.0, Math.max(0.0, simAccumMs / SIM_STEP_MS));

    // Advance simulation ticks at deterministic 60.0 Hz wall-clock pace
    if (isAttractMatch) {
      for (let s = 0; s < steps; s++) {
        if (currSimPacked) prevSimPacked = currSimPacked;
        wasm.sim_tick(0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
        const sub = readPackedState();
        if (sub) {
          currSimPacked = new Int32Array(sub);
          if (!prevSimPacked || prevSimPacked[3] !== currSimPacked[3]) {
            prevSimPacked = currSimPacked;
          }
          if (sub[17] > 0) wasm.sim_drain_events_json();
        }
      }
      const p = currSimPacked || readPackedState();
      if (p && p[3] === 12 && !attractRestartInFlight) {
        startAttractBackgroundMatch();
      }
      engine3d.renderFrame(p, null, prevSimPacked || p, interpAlpha);
    } else {
      if (!paused && currentScreen === "match" && steps > 0) {
        const sideMode = SIDE_MODES[pmSideIdx].id;
        let hIn = { mx: 0, my: 0, ax: 0, ay: 0, btns: 0 };
        let aIn = { mx: 0, my: 0, ax: 0, ay: 0, btns: 0 };

        if (sideMode === "local_2p") {
          hIn = samplePrimaryHumanInput(false);
          aIn = sampleSecondaryHumanInput();
        } else if (sideMode === "away_vs_cpu") {
          aIn = samplePrimaryHumanInput(true);
        } else if (sideMode === "home_vs_cpu") {
          hIn = samplePrimaryHumanInput(true);
        }

        for (let s = 0; s < steps; s++) {
          if (currSimPacked) prevSimPacked = currSimPacked;
          const hBtns = ((hIn.btns & 0xffff) | ((queuedSubReserved[0] & 0xffff) << 16)) >>> 0;
          const aBtns = ((aIn.btns & 0xffff) | ((queuedSubReserved[1] & 0xffff) << 16)) >>> 0;
          queuedSubReserved[0] = 0;
          queuedSubReserved[1] = 0;

          wasm.sim_tick(hIn.mx, hIn.my, hIn.ax, hIn.ay, hBtns, aIn.mx, aIn.my, aIn.ax, aIn.ay, aBtns);
          const subPacked = readPackedState();
          if (subPacked) {
            currSimPacked = new Int32Array(subPacked);
            if (!prevSimPacked || prevSimPacked[3] !== currSimPacked[3]) {
              prevSimPacked = currSimPacked;
            }
          }
          if (subPacked && subPacked[3] !== 12) {
            if (replayBuffer.length >= REPLAY_CAPACITY) replayBuffer.shift();
            replayBuffer.push(new Int32Array(subPacked));
            if (subPacked[15] === 0) homePossTicks++;
            else if (subPacked[15] === 1) awayPossTicks++;
          }
          if (subPacked && subPacked[17] > 0) {
            drainLiveMatchEvents(subPacked);
          }
        }
      }

      const packed = currSimPacked || readPackedState();
      if (packed) {
        const clockTicks = packed[1];
        const half = packed[2];
        const phase = packed[3];
        const hScore = packed[4];
        const aScore = packed[5];

        if (currentScreen === "match") {
          const penSuffix = lastShootoutScore ? ` (${lastShootoutScore[0]}-${lastShootoutScore[1]}P)` : "";
          document.getElementById("sb-score").textContent = `${hScore} - ${aScore}${penSuffix}`;
          document.getElementById("sb-half").textContent = formatHalfBadge(half, phase);
          const totalSec = Math.floor(clockTicks / 60);
          const mm = String(Math.floor(totalSec / 60)).padStart(2, "0");
          const ss = String(totalSec % 60).padStart(2, "0");
          document.getElementById("sb-time").textContent = `${mm}:${ss}`;
          document.getElementById("sb-phase").textContent =
            phase === 0 ? "KICK OFF · PRESS PASS [SPACE / A]" : PHASE_LABELS[phase] || "OPEN PLAY";

          updatePlayerCornerHuds(packed);
          drawPitchRadar(packed);

          if (phase === 12 && !fulltimeShown) {
            openFullTimeSummaryModal();
          }
          if (goalBannerTicks > 0) {
            goalBannerTicks--;
            if (goalBannerTicks === 0) {
              document.getElementById("goal-banner").hidden = true;
            }
          }
        }

        engine3d.renderFrame(packed, null, prevSimPacked || packed, interpAlpha);
      }
    }
  } else {
    engine3d.renderFrame(null, null);
  }

  requestAnimationFrame(mainLoop);
}

// --- Team Management (Visual Pitch 11 → 5 Formation Board) ---

function ensureClubDraft(roster) {
  const cid = roster.club_id;
  if (!draftLineups[cid] || !draftLineups[cid].dirty) {
    draftLineups[cid] = {
      starterIds: Array.from(roster.starter_ids_5v5 || []),
      formation: roster.formation_5v5 || "1-2-1",
      dirty: false,
    };
  }
  return draftLineups[cid];
}

function roleBadgeMeta(role) {
  if (role === "Goalkeeper") return { label: "GK", cls: "role-gk", order: 0 };
  if (role === "Defender") return { label: "DEF", cls: "role-def", order: 1 };
  if (role === "Midfielder") return { label: "MID", cls: "role-mid", order: 2 };
  return { label: "FWD", cls: "role-fwd", order: 3 };
}

function renderSquadBuilder() {
  if (!latestOverview || !latestOverview.rosters) return;
  const orderedIds = getAvailableClubOrder();
  if (!latestOverview.rosters[activeSquadClub] && orderedIds.length > 0) {
    activeSquadClub = orderedIds[0];
  }
  const roster = latestOverview.rosters[activeSquadClub];
  if (!roster) return;

  const tabsEl = document.getElementById("sq-club-tabs");
  if (tabsEl) {
    tabsEl.innerHTML = "";
    for (const cid of orderedIds) {
      const cr = latestOverview.rosters[cid];
      if (!cr) continue;
      const btn = document.createElement("button");
      btn.type = "button";
      btn.className = `sq-club-tab${cid === activeSquadClub ? " active" : ""}`;
      btn.setAttribute("data-club", cid);
      btn.title = cr.name;
      btn.textContent = cr.short_name || cid.toUpperCase();
      btn.addEventListener("click", () => {
        activeSquadClub = cid;
        swapArmedStarterId = null;
        document.getElementById("squad-save-msg").hidden = true;
        GameAudio.uiTick();
        renderSquadBuilder();
      });
      tabsEl.appendChild(btn);
    }
  }

  const draft = ensureClubDraft(roster);
  document.querySelectorAll(".sq-fmt-pill").forEach((btn) => {
    btn.classList.toggle("active", btn.getAttribute("data-fmt") === draft.formation);
  });

  const byId = new Map(roster.squad.map((p) => [p.footballer_id, p]));
  const selectedMembers = draft.starterIds.map((id) => byId.get(id)).filter(Boolean);
  const gk = selectedMembers.find((m) => m.role === "Goalkeeper");
  const outfield = selectedMembers
    .filter((m) => m.role !== "Goalkeeper")
    .sort((a, b) => roleBadgeMeta(a.role).order - roleBadgeMeta(b.role).order);

  const orderedStarters = [...(gk ? [gk] : []), ...outfield];
  const validFive = Boolean(gk) && outfield.length === 4;

  const pill = document.getElementById("squad-count-pill");
  const saveBtn = document.getElementById("btn-save-lineup");
  const playBtn = document.getElementById("btn-play-with-squad");
  if (validFive) {
    const tag = draft.dirty ? "UNSAVED" : roster.custom_lineup_active ? "SAVED" : "DEFAULT 5";
    pill.textContent = `5 / 5 READY (${tag})`;
    saveBtn.disabled = false;
    playBtn.disabled = false;
  } else {
    pill.textContent = `${orderedStarters.length} / 5 SELECTED`;
    saveBtn.disabled = true;
    playBtn.disabled = true;
  }

  // Render 5 interactive tokens on the 2D Tactical Pitch Board!
  const coords = FORMATION_COORDS_5V5[draft.formation] || FORMATION_COORDS_5V5["1-2-1"];
  const tokensLayer = document.getElementById("tp-tokens-layer");
  tokensLayer.innerHTML = "";
  const kits = resolveMatchKits(roster.club_id, "chelsea", roster.primary_color, "#1d4ed8");

  orderedStarters.slice(0, 5).forEach((m, idx) => {
    const [xp, yp] = coords[idx] || [50, 50];
    const isGk = m.role === "Goalkeeper";
    const bg = isGk ? kits.homeGk.shirt : kits.homeOutfield.shirt;
    const fg = isGk ? kits.homeGk.numberColor : kits.homeOutfield.numberColor;
    const surname = m.name.trim().split(/\s+/).slice(-1)[0].toUpperCase();

    const token = document.createElement("div");
    token.className = `pitch-token${swapArmedStarterId === m.footballer_id ? " swap-armed" : ""}`;
    token.style.left = `${xp}%`;
    token.style.top = `${yp}%`;
    token.innerHTML = `
      <div class="pt-jersey" style="background:${bg};color:${fg}">${m.shirt_number}</div>
      <div class="pt-tag">${surname} · ${m.overall}</div>
    `;
    token.addEventListener("click", () => {
      if (isGk) {
        swapArmedStarterId = null;
      } else {
        swapArmedStarterId = swapArmedStarterId === m.footballer_id ? null : m.footballer_id;
      }
      GameAudio.uiTick(540);
      renderSquadBuilder();
    });
    tokensLayer.appendChild(token);
  });

  // Render full 11-player official squad list
  const listEl = document.getElementById("squad-roster-list");
  listEl.innerHTML = "";
  const selectedSet = new Set(draft.starterIds);

  for (const p of roster.squad) {
    const isSelected = selectedSet.has(p.footballer_id);
    const rb = roleBadgeMeta(p.role);
    const a = p.attributes;
    const attrs =
      p.role === "Goalkeeper"
        ? `GK ${a.goalkeeping} · POS ${a.positioning} · PAS ${a.passing}`
        : `PAC ${a.pace} · SHO ${a.shooting} · PAS ${a.passing} · DRI ${a.dribbling} · DEF ${a.tackling}`;

    const row = document.createElement("div");
    row.className = `squad-player-row${isSelected ? " selected" : ""}`;
    row.innerHTML = `
      <div class="sp-left">
        <span class="sp-shirt">#${p.shirt_number}</span>
        <span class="role-tag ${rb.cls}">${rb.label}</span>
        <span class="sp-name">${p.name}</span>
      </div>
      <div class="sp-right">
        <span class="sp-attrs">${attrs}</span>
        <span class="ovr-pill">${p.overall}</span>
      </div>
    `;

    row.addEventListener("click", () => {
      const msg = document.getElementById("squad-save-msg");
      if (p.role === "Goalkeeper") {
        draft.starterIds = [
          p.footballer_id,
          ...draft.starterIds.filter((id) => {
            const m = byId.get(id);
            return m && m.role !== "Goalkeeper";
          }),
        ];
        msg.hidden = false;
        msg.textContent = `${p.name} locked in at Goalkeeper (5v5 requires 1 GK + 4 Outfielders).`;
        renderSquadBuilder();
        return;
      }

      // Direct swap if a pitch token was armed and user clicked a bench outfielder!
      if (!isSelected && swapArmedStarterId && selectedSet.has(swapArmedStarterId)) {
        draft.starterIds = draft.starterIds.map((id) => (id === swapArmedStarterId ? p.footballer_id : id));
        swapArmedStarterId = null;
        draft.dirty = true;
        msg.hidden = true;
        GameAudio.uiConfirm();
        renderSquadBuilder();
        return;
      }

      if (isSelected) {
        draft.starterIds = draft.starterIds.filter((id) => id !== p.footballer_id);
        if (swapArmedStarterId === p.footballer_id) swapArmedStarterId = null;
        draft.dirty = true;
        msg.hidden = true;
      } else {
        const currentOut = draft.starterIds.filter((id) => {
          const m = byId.get(id);
          return m && m.role !== "Goalkeeper";
        });
        if (currentOut.length >= 4) {
          // Swap out the last outfielder automatically or prompt
          const droppedId = currentOut[currentOut.length - 1];
          draft.starterIds = draft.starterIds.map((id) => (id === droppedId ? p.footballer_id : id));
          const dropped = byId.get(droppedId);
          msg.hidden = false;
          msg.textContent = `Swapped in ${p.name} for ${dropped ? dropped.name : "starter"}.`;
        } else {
          draft.starterIds.push(p.footballer_id);
          msg.hidden = true;
        }
        draft.dirty = true;
      }
      GameAudio.uiTick(560);
      renderSquadBuilder();
    });

    listEl.appendChild(row);
  }
}

async function saveActiveClubLineup() {
  const roster = latestOverview && latestOverview.rosters && latestOverview.rosters[activeSquadClub];
  if (!roster) return false;
  const draft = ensureClubDraft(roster);
  const res = await fetch(
    `/api/worlds/${encodeURIComponent(activeWorldId)}/clubs/${encodeURIComponent(activeSquadClub)}/lineup`,
    {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        starter_ids: draft.starterIds,
        formation_5v5: draft.formation,
        actor_id: "part-alex",
      }),
    }
  );
  const out = await res.json();
  const msg = document.getElementById("squad-save-msg");
  msg.hidden = false;
  if (!res.ok) {
    msg.textContent = out.detail || "Could not save lineup";
    return false;
  }
  draft.dirty = false;
  draft.starterIds = Array.from(out.roster.starter_ids_5v5);
  draft.formation = out.roster.formation_5v5;
  msg.textContent = `Saved ${out.roster.name} (${out.roster.formation_5v5}) to active Parallel Season world!`;
  GameAudio.uiConfirm();
  await refreshWorldOverview();
  return true;
}

// --- Parallel Season World Hub Sync ---

async function refreshWorldsDropdown() {
  const res = await fetch("/api/worlds");
  if (!res.ok) return;
  const data = await res.json();
  const sel = document.getElementById("world-select");
  sel.innerHTML = "";
  for (const w of data.worlds) {
    const opt = document.createElement("option");
    opt.value = w.id;
    opt.textContent = `${w.season_label} · ${w.name} (${w.status})`;
    if (w.id === activeWorldId) opt.selected = true;
    sel.appendChild(opt);
  }
}

function buildFixtureCardElement(fx) {
  const isCup = fx.stage !== "league";
  const srcClass =
    fx.result_source === "played_vs_friend"
      ? "src-friend"
      : fx.result_source === "played_vs_cpu"
        ? "src-cpu"
        : fx.result_source === "generated"
          ? "src-generated"
          : "src-imported";

  const isPlayable =
    fx.state !== "accepted" &&
    (fx.result_source === "played_vs_friend" || fx.result_source === "played_vs_cpu");
  const isGeneratable = fx.state !== "accepted" && fx.result_source === "generated";
  const isAdminPending = fx.state === "awaiting_admin";

  let scoreText = "VS";
  if (fx.state === "accepted") {
    const pen =
      fx.accepted_shootout_home !== null && fx.accepted_shootout_home !== undefined
        ? ` (${fx.accepted_shootout_home}-${fx.accepted_shootout_away}P)`
        : fx.accepted_aet
          ? " (AET)"
          : "";
    scoreText = `${fx.accepted_home_goals} - ${fx.accepted_away_goals}${pen}`;
  } else if (fx.state === "in_session") {
    scoreText = "LIVE";
  } else if (isAdminPending) {
    scoreText = "RULING";
  }

  const stageBadge =
    fx.stage === "cup_sf"
      ? "FA CUP SEMI-FINAL"
      : fx.stage === "cup_final"
        ? "FA CUP FINAL"
        : `PREMIER LEAGUE R${fx.round}`;

  const realNote =
    fx.real_home_goals !== null && fx.real_away_goals !== null
      ? `Real Football: ${fx.real_home_goals}-${fx.real_away_goals}`
      : isCup
        ? "Parallel-Only Divergent Tie"
        : "";

  let actionHtml = `<span>${
    fx.state === "accepted"
      ? fx.accepted_source === "imported"
        ? "Inherited"
        : "Verified"
      : "Awaiting Real Kickoff"
  }</span>`;
  if (isPlayable) {
    actionHtml = `<button class="console-btn console-btn-primary btn-play-fx" type="button" style="font-size:14px;padding:6px 14px">
      ${fx.state === "in_session" ? "RESUME MATCH ▶" : "PLAY OFFICIAL ▶"}
    </button>`;
  } else if (isGeneratable) {
    actionHtml = `<button class="console-btn console-btn-gold btn-gen-fx" type="button" style="font-size:14px;padding:6px 14px">SIM POISSON</button>`;
  } else if (isAdminPending) {
    actionHtml = `<button class="console-btn console-btn-secondary btn-admin-fx" type="button" style="font-size:14px;padding:6px 14px">AWARD 3-0</button>`;
  }

  const card = document.createElement("div");
  card.className = `fixture-card${isCup ? " cup-card" : ""}`;
  card.innerHTML = `
    <div class="fx-top">
      <span>${stageBadge} · ${fx.state.toUpperCase()}</span>
      <span class="fx-source ${srcClass}">${fx.result_source.replace(/_/g, " ")}</span>
    </div>
    <div class="fx-mid">
      <span>${fx.home_club.name}</span>
      <span class="fx-score-box">${scoreText}</span>
      <span>${fx.away_club.name}</span>
    </div>
    <div class="fx-bottom">
      <span>${realNote}</span>
      ${actionHtml}
    </div>
  `;

  const playBtn = card.querySelector(".btn-play-fx");
  if (playBtn) {
    playBtn.addEventListener("click", () => {
      pendingOfficialFixture = {
        id: fx.id,
        homeClubId: fx.home_club.club_id || fx.home_club_id,
        awayClubId: fx.away_club.club_id || fx.away_club_id,
        result_source: fx.result_source,
        stageLabel: stageBadge,
      };
      pmSideIdx = defaultOfficialSideIdx(pendingOfficialFixture);
      GameAudio.uiConfirm();
      navigateTo("prematch");
    });
  }

  const genBtn = card.querySelector(".btn-gen-fx");
  if (genBtn) {
    genBtn.addEventListener("click", async () => {
      await fetch(`/api/fixtures/${encodeURIComponent(fx.id)}/generate`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({}),
      });
      await refreshWorldOverview();
    });
  }

  const adminBtn = card.querySelector(".btn-admin-fx");
  if (adminBtn) {
    adminBtn.addEventListener("click", async () => {
      await fetch(`/api/admin/fixtures/${encodeURIComponent(fx.id)}/resolve`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ home_goals: 3, away_goals: 0, reason: "commissioner walkover" }),
      });
      await refreshWorldOverview();
    });
  }

  return card;
}

async function refreshWorldOverview() {
  await refreshWorldsDropdown();
  const res = await fetch(`/api/worlds/${encodeURIComponent(activeWorldId)}/overview`);
  if (!res.ok) return;
  const data = await res.json();
  latestOverview = data;

  const hasSeasonInName = String(data.world.name || "").includes(String(data.world.season_label || ""));
  document.getElementById("top-world-chip").textContent = hasSeasonInName
    ? data.world.name.toUpperCase()
    : `${data.world.season_label} · ${data.world.name}`;
  document.getElementById("world-name").textContent = hasSeasonInName
    ? data.world.name
    : `${data.world.name} (${data.world.season_label})`;

  const repLabel = data.assignments
    .map((a) => `${a.club_id.toUpperCase()} (${a.participant_name.split(" ")[0]})`)
    .join(", ");
  document.getElementById("world-meta").textContent = `Represented: ${repLabel || "None"}`;

  // Main Menu Spotlight Card (Next Playable Official Fixture chronologically by kickoff_utc + Mini Standings)
  const nextPlayable = data.fixtures
    .filter(
      (fx) =>
        fx.state !== "accepted" &&
        (fx.result_source === "played_vs_friend" || fx.result_source === "played_vs_cpu")
    )
    .sort((a, b) => String(a.kickoff_utc || "").localeCompare(String(b.kickoff_utc || "")))[0];
  const hubMatchup = document.getElementById("hub-next-matchup");
  const hubMeta = document.getElementById("hub-next-meta");
  const hubPlayBtn = document.getElementById("btn-hub-play-official");

  if (nextPlayable) {
    const stg =
      nextPlayable.stage === "cup_sf"
        ? "FA Parallel Cup Semi-Final"
        : nextPlayable.stage === "cup_final"
          ? "FA Parallel Cup Final"
          : `Premier League · Round ${nextPlayable.round}`;
    hubMatchup.textContent = `${nextPlayable.home_club.short_name} vs ${nextPlayable.away_club.short_name}`;
    hubMeta.textContent = `${stg} · ${nextPlayable.result_source.replace(/_/g, " ").toUpperCase()}`;
    hubPlayBtn.disabled = false;
    hubPlayBtn.onclick = () => {
      pendingOfficialFixture = {
        id: nextPlayable.id,
        homeClubId: nextPlayable.home_club.club_id || nextPlayable.home_club_id,
        awayClubId: nextPlayable.away_club.club_id || nextPlayable.away_club_id,
        result_source: nextPlayable.result_source,
        stageLabel: stg.toUpperCase(),
      };
      pmSideIdx = defaultOfficialSideIdx(pendingOfficialFixture);
      GameAudio.uiConfirm();
      navigateTo("prematch");
    };
  } else {
    hubMatchup.textContent = "ALL MATCHDAYS PLAYED";
    hubMeta.textContent = "Advance FA Parallel Cup or Roll Over Season in Hub";
    hubPlayBtn.disabled = false;
    hubPlayBtn.textContent = "OPEN PARALLEL SEASON HUB ▶";
    hubPlayBtn.onclick = () => navigateTo("season");
  }

  const miniTbody = document.querySelector("#hub-mini-standings tbody");
  const topSix = data.standings.slice(0, 6);
  const extraRep = data.standings.filter((r) => r.position > 6 && r.represented);
  const miniRows = [];
  for (const r of topSix) {
    miniRows.push(`
      <tr>
        <td>${r.position}</td>
        <td><strong>${r.short_name}</strong> ${r.represented ? "★" : ""}</td>
        <td>${r.played}</td>
        <td>${r.gd > 0 ? `+${r.gd}` : r.gd}</td>
        <td><strong>${r.points}</strong></td>
      </tr>`);
  }
  if (extraRep.length > 0) {
    if (extraRep[0].position > 7) {
      miniRows.push(`<tr><td colspan="5" style="text-align:center;opacity:0.45;padding:2px 0">· · ·</td></tr>`);
    }
    for (const r of extraRep) {
      miniRows.push(`
        <tr>
          <td>${r.position}</td>
          <td><strong>${r.short_name}</strong> ★</td>
          <td>${r.played}</td>
          <td>${r.gd > 0 ? `+${r.gd}` : r.gd}</td>
          <td><strong>${r.points}</strong></td>
        </tr>`);
    }
  }
  miniTbody.innerHTML = miniRows.join("");

  // Honours banner
  const honStrip = document.getElementById("honours-strip");
  if (data.honours && data.honours.length > 0) {
    honStrip.hidden = false;
    honStrip.innerHTML = data.honours
      .map((h) => `<span>🏆 <strong>${h.category.replace(/_/g, " ").toUpperCase()}:</strong> ${h.winner_name}</span>`)
      .join("");
  } else {
    honStrip.hidden = true;
  }

  // Season Fixtures Grid — show open/live/upcoming fixtures chronologically by kickoff_utc first, then completed fixtures newest-first!
  const fxGrid = document.getElementById("fixture-list");
  fxGrid.innerHTML = "";
  const filtered = data.fixtures
    .filter((fx) => {
      if (activeCompFilter === "upcoming") return fx.state !== "accepted";
      if (activeCompFilter === "league") return fx.stage === "league";
      if (activeCompFilter === "knockout") return fx.stage !== "league";
      return true;
    })
    .slice()
    .sort((a, b) => {
      const aDone = a.state === "accepted" ? 1 : 0;
      const bDone = b.state === "accepted" ? 1 : 0;
      if (aDone !== bDone) return aDone - bDone;
      if (aDone === 0) {
        const cmpKo = String(a.kickoff_utc || "").localeCompare(String(b.kickoff_utc || ""));
        if (cmpKo !== 0) return cmpKo;
        return a.round - b.round;
      }
      const cmpKoDesc = String(b.kickoff_utc || "").localeCompare(String(a.kickoff_utc || ""));
      if (cmpKoDesc !== 0) return cmpKoDesc;
      return b.round - a.round;
    });
  filtered.forEach((fx) => fxGrid.appendChild(buildFixtureCardElement(fx)));

  // FA Parallel Cup Bracket Grid
  const cupGrid = document.getElementById("cup-bracket-container");
  cupGrid.innerHTML = "";
  data.fixtures.filter((fx) => fx.stage !== "league").forEach((fx) => cupGrid.appendChild(buildFixtureCardElement(fx)));

  // League Standings Table
  const stBody = document.getElementById("standings-tbody");
  stBody.innerHTML = data.standings
    .map((r) => {
      const d = r.delta_vs_real;
      const dHtml = d > 0 ? `<span class="pos-delta">+${d}</span>` : d < 0 ? `<span class="neg-delta">${d}</span>` : "0";
      return `
        <tr>
          <td>${r.position}</td>
          <td><strong>${r.club_name || r.name}</strong> ${r.represented ? "🎮" : ""}</td>
          <td>${r.played}</td>
          <td>${r.won} - ${r.drawn} - ${r.lost}</td>
          <td>${r.gd > 0 ? `+${r.gd}` : r.gd}</td>
          <td>${r.inherited_points}</td>
          <td>${r.earned_points}</td>
          <td><strong>${r.points}</strong></td>
          <td>${dHtml}</td>
        </tr>`;
    })
    .join("");

  // Top Scorers & Player Stats
  document.getElementById("players-tbody").innerHTML = data.players
    .map(
      (p) => `
      <tr>
        <td><strong>${p.name}</strong></td>
        <td>${p.club_short}</td>
        <td>${p.inherited.goals}G · ${p.inherited.assists}A</td>
        <td>${p.earned.goals}G · ${p.earned.assists}A</td>
        <td><strong>${p.combined.goals}G · ${p.combined.assists}A</strong></td>
        <td>${p.combined.skill_moves || 0}★ · ${p.combined.saves} SV</td>
      </tr>`
    )
    .join("");

  // Commissioner Appointments & Ledger
  const apptList = document.getElementById("appointment-list");
  apptList.innerHTML = "";
  for (const ap of data.appointments || []) {
    const li = document.createElement("li");
    li.innerHTML = `
      <span>${ap.fixture_id} · ${ap.proposed_at_utc} (${ap.state.toUpperCase()})</span>
      ${ap.state === "proposed" ? `<button class="console-btn console-btn-primary btn-acc" type="button" style="font-size:12px;padding:4px 10px">ACCEPT</button>` : ""}
    `;
    const btn = li.querySelector(".btn-acc");
    if (btn) {
      btn.addEventListener("click", async () => {
        await fetch(`/api/appointments/${encodeURIComponent(ap.id)}/respond`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ responder_id: "part-sam", accept: true }),
        });
        await refreshWorldOverview();
      });
    }
    apptList.appendChild(li);
  }

  document.getElementById("event-count").textContent = String(data.total_events);
  document.getElementById("ledger-list").innerHTML = data.recent_events
    .map((ev) => `<li><span>#${ev.seq} <strong>${ev.type}</strong></span><span>${ev.aggregate_id}</span></li>`)
    .join("");

  if (currentScreen === "prematch") renderPreMatchScreen();
  if (currentScreen === "squad") renderSquadBuilder();
}

// --- UI Event Wiring ---

document.getElementById("btn-press-start").addEventListener("click", () => {
  GameAudio.uiConfirm();
  navigateTo("menu");
});

document.getElementById("btn-home-logo").addEventListener("click", () => {
  GameAudio.uiTick(440);
  navigateTo("menu");
});

document.getElementById("btn-audio-toggle").addEventListener("click", (ev) => {
  const on = GameAudio.toggle();
  ev.currentTarget.textContent = on ? "SFX: ON" : "SFX: MUTED";
});

document.querySelectorAll(".hub-blade").forEach((blade, idx) => {
  blade.addEventListener("mouseenter", () => {
    focusedBladeIdx = idx;
    document.querySelectorAll(".hub-blade").forEach((b, i) => b.classList.toggle("focused", i === idx));
  });
  blade.addEventListener("click", () => triggerHubBlade(blade.getAttribute("data-nav")));
});

document.getElementById("btn-hub-full-table").addEventListener("click", () => {
  GameAudio.uiConfirm();
  navigateTo("season");
  document.querySelector('[data-stab="standings"]').click();
});

// Pre-Match Steppers
function stepClub(current, dir, avoid) {
  const order = getAvailableClubOrder();
  if (order.length === 0) return current;
  let idx = order.indexOf(current);
  if (idx < 0) idx = 0;
  for (let i = 0; i < order.length; i++) {
    idx = (idx + dir + order.length) % order.length;
    if (order[idx] !== avoid) return order[idx];
  }
  return order[0];
}

function stepSideMode(dir) {
  const allowed = getAllowedOfficialSideIndices(pendingOfficialFixture);
  let pos = allowed.indexOf(pmSideIdx);
  if (pos < 0) pos = 0;
  pos = (pos + dir + allowed.length) % allowed.length;
  pmSideIdx = allowed[pos];
}

document.getElementById("pm-home-prev").addEventListener("click", () => {
  pmHomeClub = stepClub(pmHomeClub, -1, pmAwayClub);
  GameAudio.uiTick();
  renderPreMatchScreen();
});
document.getElementById("pm-home-next").addEventListener("click", () => {
  pmHomeClub = stepClub(pmHomeClub, 1, pmAwayClub);
  GameAudio.uiTick();
  renderPreMatchScreen();
});
document.getElementById("pm-away-prev").addEventListener("click", () => {
  pmAwayClub = stepClub(pmAwayClub, -1, pmHomeClub);
  GameAudio.uiTick();
  renderPreMatchScreen();
});
document.getElementById("pm-away-next").addEventListener("click", () => {
  pmAwayClub = stepClub(pmAwayClub, 1, pmHomeClub);
  GameAudio.uiTick();
  renderPreMatchScreen();
});

document.getElementById("pm-side-prev").addEventListener("click", () => {
  stepSideMode(-1);
  GameAudio.uiTick();
  renderPreMatchScreen();
});
document.getElementById("pm-side-next").addEventListener("click", () => {
  stepSideMode(1);
  GameAudio.uiTick();
  renderPreMatchScreen();
});
document.getElementById("pm-fmt-prev").addEventListener("click", () => {
  pmFmtIdx = (pmFmtIdx - 1 + FORMAT_OPTIONS.length) % FORMAT_OPTIONS.length;
  GameAudio.uiTick();
  renderPreMatchScreen();
});
document.getElementById("pm-fmt-next").addEventListener("click", () => {
  pmFmtIdx = (pmFmtIdx + 1) % FORMAT_OPTIONS.length;
  GameAudio.uiTick();
  renderPreMatchScreen();
});
document.getElementById("pm-dur-prev").addEventListener("click", () => {
  pmDurIdx = (pmDurIdx - 1 + DURATION_OPTIONS.length) % DURATION_OPTIONS.length;
  GameAudio.uiTick();
  renderPreMatchScreen();
});
document.getElementById("pm-dur-next").addEventListener("click", () => {
  pmDurIdx = (pmDurIdx + 1) % DURATION_OPTIONS.length;
  GameAudio.uiTick();
  renderPreMatchScreen();
});
document.getElementById("pm-diff-prev").addEventListener("click", () => {
  pmDiffIdx = (pmDiffIdx - 1 + DIFF_OPTIONS.length) % DIFF_OPTIONS.length;
  GameAudio.uiTick();
  renderPreMatchScreen();
});
document.getElementById("pm-diff-next").addEventListener("click", () => {
  pmDiffIdx = (pmDiffIdx + 1) % DIFF_OPTIONS.length;
  GameAudio.uiTick();
  renderPreMatchScreen();
});
document.getElementById("pm-cam-prev").addEventListener("click", () => {
  cycleMatchCamera();
  renderPreMatchScreen();
});
document.getElementById("pm-cam-next").addEventListener("click", () => {
  cycleMatchCamera();
  renderPreMatchScreen();
});
document.getElementById("pm-wx-prev").addEventListener("click", () => {
  const modes = ["night", "day", "rain"];
  const cur = modes.indexOf(engine3d.weatherMode);
  engine3d.setWeatherPreset(modes[(cur - 1 + modes.length) % modes.length]);
  syncWeatherLabels();
  GameAudio.uiTick(560);
});
document.getElementById("pm-wx-next").addEventListener("click", () => {
  cycleMatchWeather();
});

document.getElementById("btn-pm-back").addEventListener("click", () => {
  GameAudio.uiTick(380);
  navigateTo(pendingOfficialFixture ? "season" : "menu");
});
document.getElementById("btn-pm-squads").addEventListener("click", () => {
  activeSquadClub = SIDE_MODES[pmSideIdx].id === "away_vs_cpu" ? pmAwayClub : pmHomeClub;
  GameAudio.uiConfirm();
  navigateTo("squad");
});
document.getElementById("btn-pm-kickoff").addEventListener("click", () => launchSelectedMatch());

// Squad Screen Controls
document.querySelectorAll(".sq-club-tab").forEach((btn) => {
  btn.addEventListener("click", () => {
    activeSquadClub = btn.getAttribute("data-club");
    swapArmedStarterId = null;
    document.getElementById("squad-save-msg").hidden = true;
    GameAudio.uiTick();
    renderSquadBuilder();
  });
});

document.querySelectorAll(".sq-fmt-pill").forEach((btn) => {
  btn.addEventListener("click", () => {
    const roster = latestOverview && latestOverview.rosters && latestOverview.rosters[activeSquadClub];
    if (!roster) return;
    const draft = ensureClubDraft(roster);
    draft.formation = btn.getAttribute("data-fmt");
    draft.dirty = true;
    GameAudio.uiTick();
    renderSquadBuilder();
  });
});

document.getElementById("btn-reset-lineup-default").addEventListener("click", () => {
  const roster = latestOverview && latestOverview.rosters && latestOverview.rosters[activeSquadClub];
  if (!roster) return;
  draftLineups[activeSquadClub] = {
    starterIds: roster.squad.filter((m) => m.default_starter_5v5).map((m) => m.footballer_id),
    formation: "1-2-1",
    dirty: true,
  };
  swapArmedStarterId = null;
  GameAudio.uiTick();
  renderSquadBuilder();
});

document.getElementById("btn-save-lineup").addEventListener("click", () => saveActiveClubLineup());

document.getElementById("btn-play-with-squad").addEventListener("click", async () => {
  const ok = await saveActiveClubLineup();
  if (!ok) return;
  pendingOfficialFixture = null;
  pmHomeClub = activeSquadClub;
  if (pmAwayClub === pmHomeClub) {
    pmAwayClub = stepClub(pmHomeClub, 1, pmHomeClub);
  }
  pmFmtIdx = 0;
  navigateTo("prematch");
});

document.getElementById("btn-sq-back").addEventListener("click", () => {
  GameAudio.uiTick(380);
  navigateTo(previousScreen === "prematch" ? "prematch" : "menu");
});

// Parallel Season Hub Sub-Tabs & Actions
document.querySelectorAll(".season-tab").forEach((btn) => {
  btn.addEventListener("click", () => {
    document.querySelectorAll(".season-tab").forEach((b) => b.classList.remove("active"));
    document.querySelectorAll(".season-pane").forEach((p) => (p.hidden = true));
    btn.classList.add("active");
    const target = document.getElementById(`spane-${btn.getAttribute("data-stab")}`);
    if (target) target.hidden = false;
    GameAudio.uiTick();
  });
});

document.querySelectorAll(".comp-pill").forEach((pill) => {
  pill.addEventListener("click", () => {
    document.querySelectorAll(".comp-pill").forEach((p) => p.classList.remove("active"));
    pill.classList.add("active");
    activeCompFilter = pill.getAttribute("data-comp");
    GameAudio.uiTick();
    refreshWorldOverview();
  });
});

document.getElementById("world-select").addEventListener("change", (ev) => {
  activeWorldId = ev.currentTarget.value;
  Object.keys(draftLineups).forEach((k) => delete draftLineups[k]);
  refreshWorldOverview();
});

document.getElementById("btn-advance-cup").addEventListener("click", async () => {
  const msg = document.getElementById("cup-divergence-msg");
  const comps = (latestOverview && latestOverview.competitions) || [];
  const cup = comps.find((c) => c.kind === "knockout");
  const cupId = cup ? cup.id : DEFAULT_CUP_ID;
  const res = await fetch(
    `/api/worlds/${encodeURIComponent(activeWorldId)}/competitions/${encodeURIComponent(cupId)}/advance`,
    { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({}) }
  );
  const out = await res.json();
  msg.hidden = false;
  msg.textContent = res.ok
    ? out.diverged_from_real_world
      ? `Parallel Divergence! Created Parallel-Only Final: ${out.final_fixture.home_club_id.toUpperCase()} vs ${out.final_fixture.away_club_id.toUpperCase()}`
      : `Created Final: ${out.final_fixture.home_club_id.toUpperCase()} vs ${out.final_fixture.away_club_id.toUpperCase()}`
    : out.detail || "Complete both Semi-Finals first!";
  await refreshWorldOverview();
});

document.getElementById("btn-rollover-season").addEventListener("click", async () => {
  const res = await fetch(`/api/worlds/${encodeURIComponent(activeWorldId)}/rollover`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ next_season_label: "2027/28" }),
  });
  const out = await res.json();
  const box = document.getElementById("rollover-msg");
  box.hidden = false;
  box.textContent = res.ok
    ? `Season Completed! Awarded ${out.honours.length} honours & created next season (${out.next_world.season_label}).`
    : out.detail || "Rollover failed";
  await refreshWorldOverview();
});

document.getElementById("btn-propose-appt").addEventListener("click", async () => {
  await fetch("/api/fixtures/fix-mw16-ars-mun/appointments", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ proposed_by: "part-alex", proposed_at_utc: "2026-12-20T16:30:00Z" }),
  });
  await refreshWorldOverview();
});

document.getElementById("btn-test-gate").addEventListener("click", async () => {
  const res = await fetch(`/api/worlds/${encodeURIComponent(activeWorldId)}/ingest/result`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ fixture_id: "fix-mw16-ars-mun", home_goals: 1, away_goals: 4, events: [] }),
  });
  const out = await res.json();
  const box = document.getElementById("gate-msg");
  box.hidden = false;
  box.textContent = `Proposal Gate Decision: ${String(out.decision).toUpperCase()} (${out.reason})`;
  await refreshWorldOverview();
});

document.getElementById("btn-reset-world").addEventListener("click", async () => {
  const res = await fetch(`/api/worlds/${encodeURIComponent(DEFAULT_WORLD_ID)}/reset-demo`, { method: "POST" });
  const box = document.getElementById("gate-msg");
  box.hidden = false;
  if (!res.ok) {
    box.textContent = "Demo reset is disabled in production mode.";
    return;
  }
  activeWorldId = DEFAULT_WORLD_ID;
  box.textContent = "Demo world reset to 2026/27 Premier League Matchweek 11.";
  await refreshWorldOverview();
});

document.getElementById("btn-season-back").addEventListener("click", () => {
  GameAudio.uiTick(380);
  navigateTo("menu");
});

// Controllers & Cartridge Screen Actions
(() => {
  const profSel = document.getElementById("pad-profile-select");
  if (profSel) {
    try {
      const savedProf = localStorage.getItem(PAD_PROFILE_STORAGE_KEY);
      if (savedProf && Array.from(profSel.options).some((o) => o.value === savedProf)) {
        profSel.value = savedProf;
      } else if (customPadMap) {
        profSel.value = "custom";
      }
    } catch (_e) {}
    profSel.addEventListener("change", () => {
      try {
        localStorage.setItem(PAD_PROFILE_STORAGE_KEY, profSel.value);
      } catch (_e) {}
      GameAudio.uiTick(500);
    });
  }

  const webHidBtn = document.getElementById("btn-pair-webhid");
  if (webHidBtn && navigator.hid && typeof navigator.hid.requestDevice === "function") {
    webHidBtn.hidden = false;
    webHidBtn.addEventListener("click", () => pairWebHidUsbReceiver());
  }

  const calibBtn = document.getElementById("btn-calibrate-pad");
  if (calibBtn) {
    calibBtn.addEventListener("click", () => {
      GameAudio.uiTick(520);
      startPadCalibration();
    });
  }

  const skipBtn = document.getElementById("btn-calib-skip");
  if (skipBtn) {
    skipBtn.addEventListener("click", () => {
      if (!calibState) return;
      calibState.stepIdx++;
      calibState.waitingRelease = true;
      GameAudio.uiTick(440);
      advanceOrFinishCalibration();
    });
  }

  const cancelBtn = document.getElementById("btn-calib-cancel");
  if (cancelBtn) {
    cancelBtn.addEventListener("click", () => {
      calibState = null;
      const banner = document.getElementById("pad-calib-banner");
      if (banner) banner.hidden = true;
      GameAudio.uiTick(360);
    });
  }
})();

document.getElementById("cart-device-select").addEventListener("change", (ev) => {
  const isAlex = ev.currentTarget.value.includes("alex");
  document.getElementById("cart-title-display").textContent = isAlex
    ? "ALEX · MANCHESTER UNITED"
    : "SAM · ARSENAL";
});

document.getElementById("btn-sync-cartridge").addEventListener("click", async () => {
  const [participantId, deviceId] = document.getElementById("cart-device-select").value.split("|");
  const profSel = document.getElementById("pad-profile-select");
  const res = await fetch(`/api/worlds/${encodeURIComponent(activeWorldId)}/cartridge/sync`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      participant_id: participantId,
      device_id: deviceId,
      camera_preset: engine3d.cameraMode,
      controller_profile: {
        layout: profSel ? profSel.value : "auto",
        custom_map: customPadMap || null,
      },
    }),
  });
  const out = await res.json();
  const box = document.getElementById("cartridge-msg");
  box.hidden = false;
  if (!res.ok) {
    box.textContent = out.detail || "Cartridge sync failed";
    return;
  }
  latestSyncedEnvelope = out.envelope;
  document.getElementById("btn-verify-cartridge").disabled = false;
  document.getElementById("btn-download-cartridge").disabled = false;
  document.getElementById("cart-rev-display").textContent =
    `REV #${out.envelope.manifest.revision} · SEQ #${out.envelope.manifest.world.last_event_seq}`;
  box.textContent = `Synced ${deviceId} (Rev #${out.envelope.manifest.revision}) with Ed25519/HMAC signature.`;
  GameAudio.uiConfirm();
});

document.getElementById("btn-verify-cartridge").addEventListener("click", async () => {
  if (!latestSyncedEnvelope) return;
  const res = await fetch("/api/cartridge/verify", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ envelope: latestSyncedEnvelope }),
  });
  const out = await res.json();
  const box = document.getElementById("cartridge-msg");
  box.hidden = false;
  box.textContent = `Verification: ${out.valid ? "AUTHENTIC CARTRIDGE" : "TAMPERED"} · Rev #${out.revision}`;
});

document.getElementById("btn-download-cartridge").addEventListener("click", () => {
  if (!latestSyncedEnvelope) return;
  const blob = new Blob([JSON.stringify(latestSyncedEnvelope, null, 2)], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = `${latestSyncedEnvelope.manifest.device_id}.psfc.json`;
  a.click();
  URL.revokeObjectURL(url);
});

document.getElementById("btn-ctrl-back").addEventListener("click", () => {
  GameAudio.uiTick(380);
  navigateTo("menu");
});

// Match HUD, Pause & Replay Buttons
document.getElementById("btn-hud-zoom-out").addEventListener("click", () => {
  engine3d.adjustZoom(0.09);
  GameAudio.uiTick(440);
});
document.getElementById("btn-hud-zoom-in").addEventListener("click", () => {
  engine3d.adjustZoom(-0.09);
  GameAudio.uiTick(560);
});
document.getElementById("btn-hud-wx").addEventListener("click", cycleMatchWeather);
document.getElementById("btn-hud-cam").addEventListener("click", cycleMatchCamera);
document.getElementById("btn-hud-replay").addEventListener("click", toggleInstantReplay);
document.getElementById("btn-hud-pause").addEventListener("click", togglePauseModal);

document.getElementById("btn-pause-resume").addEventListener("click", togglePauseModal);
document.getElementById("btn-pause-subs-toggle").addEventListener("click", () => togglePauseSubsDrawer());
document.getElementById("btn-pause-replay").addEventListener("click", toggleInstantReplay);
document.getElementById("btn-pause-cam").addEventListener("click", cycleMatchCamera);
document.getElementById("btn-pause-wx").addEventListener("click", cycleMatchWeather);
document.getElementById("btn-subs-home").addEventListener("click", () => {
  subsSide = 0;
  subsArmedSlot = null;
  GameAudio.uiTick(500);
  renderPauseSubsPanel();
});
document.getElementById("btn-subs-away").addEventListener("click", () => {
  subsSide = 1;
  subsArmedSlot = null;
  GameAudio.uiTick(500);
  renderPauseSubsPanel();
});
document.getElementById("btn-pause-sim").addEventListener("click", () => {
  if (!wasm) return;
  paused = false;
  document.getElementById("pause-modal").hidden = true;
  wasm.sim_auto_step(90000);
  resetSimInterpState();
  const p = readPackedState();
  drainLiveMatchEvents(p);
  openFullTimeSummaryModal();
});
document.getElementById("btn-pause-quit").addEventListener("click", () => {
  paused = false;
  document.getElementById("pause-modal").hidden = true;
  navigateTo(activeOfficialSession ? "season" : "menu");
});

document.getElementById("replay-scrub").addEventListener("input", (ev) => {
  replayCursor = parseFloat(ev.currentTarget.value) || 0;
});
document.getElementById("btn-replay-cam").addEventListener("click", (ev) => {
  replayCamIdx = (replayCamIdx + 1) % REPLAY_CAM_MODES.length;
  ev.currentTarget.textContent = REPLAY_CAM_LABELS[REPLAY_CAM_MODES[replayCamIdx]];
  GameAudio.uiTick();
});
document.getElementById("btn-replay-exit").addEventListener("click", exitInstantReplay);

document.getElementById("btn-submit-session").addEventListener("click", submitOfficialSessionResult);
document.getElementById("btn-ft-rematch").addEventListener("click", () => launchSelectedMatch());
document.getElementById("btn-ft-hub").addEventListener("click", () => {
  navigateTo(activeOfficialSession ? "season" : "menu");
});

// --- Boot Sequence ---

async function bootGame() {
  updateGamepadBadges();
  await refreshWorldOverview();

  const titlePill = document.getElementById("title-engine-pill");
  try {
    const resp = await fetch("/wasm/psfc_wasm.wasm");
    if (!resp.ok) throw new Error(`HTTP ${resp.status}`);
    const bytes = await resp.arrayBuffer();
    const { instance } = await WebAssembly.instantiate(bytes, {});
    wasm = instance.exports;
    titlePill.textContent = "3D FLOODLIT STADIUM · 60HZ ENGINE READY";
    await startAttractBackgroundMatch();
  } catch (err) {
    titlePill.textContent = `ENGINE LOAD ERROR: ${err.message}`;
  }

  requestAnimationFrame(mainLoop);
}

window.__psfcDiagnostics = {
  engine3d,
  getWasm: () => wasm,
  getSnapshot: () => currentSnapshot,
  getSubsUsed: () => subsUsedCount,
  stepTicks(count = 180) {
    if (!wasm || !currentSnapshot) return;
    for (let i = 0; i < count; i++) {
      wasm.sim_tick(0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
      const p = readPackedState();
      if (!p) break;
      if (p[3] !== 12) {
        if (replayBuffer.length >= REPLAY_CAPACITY) replayBuffer.shift();
        replayBuffer.push(new Int32Array(p));
      }
      if (p[17] > 0) drainLiveMatchEvents(p);
      if (p[3] === 12) break;
    }
    resetSimInterpState();
  },
  fastStepToPhase(targetPhase, maxTicks = 28000, requireAttackingHalf = false) {
    if (!wasm || !currentSnapshot) return false;
    const halfLmm = currentSnapshot.format.pitch_length_mm * 0.5;
    for (let i = 0; i < maxTicks; i++) {
      wasm.sim_tick(0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
      const p = readPackedState();
      if (!p) break;
      if (p[17] > 0) drainLiveMatchEvents(p);
      if (p[3] === 12) return false;
      if (p[3] === targetPhase) {
        if (!requireAttackingHalf) {
          resetSimInterpState();
          return true;
        }
        const pps = p[6];
        const ownerSide = p[15];
        const gk0X = p[18 + 4];
        const gk1X = p[18 + pps * 10 + 4];
        const attGoalX = (ownerSide === 0 ? Math.sign(gk1X || 1) : Math.sign(gk0X || -1)) * halfLmm;
        const dist = Math.hypot(attGoalX - p[9], p[10]);
        if (dist < halfLmm * 0.85) {
          // Step 4 ticks so wall defenders & taker settle cleanly on their marks!
          for (let k = 0; k < 4; k++) wasm.sim_tick(0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
          resetSimInterpState();
          return true;
        }
      }
    }
    resetSimInterpState();
    return false;
  },
};

bootGame();
