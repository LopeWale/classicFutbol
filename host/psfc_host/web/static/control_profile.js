// =========================================================================
// PSFC ControlProfile — pure, DOM-free input abstraction layer (Layer 2).
//
// Physical inputs (south/east/west/north/lb/rb/lt/rt/select + sticks) are
// mapped to ABSTRACT ACTIONS through two context maps:
//   * attack  = In Possession      (pass, shoot, cross, throughBall, ...)
//   * defense = Out of Possession  (contain, standingTackle, ...)
// Both maps are OR'd into one InputFrame `buttons` word every tick; the Rust
// `PossessionContext` state machine decides which half applies. That keeps
// every tick deterministic and replayable (see sim/psfc-sim/src/input.rs).
//
// Serialization
//   * In memory (this module, app.js): camelCase keys (`throughBall`).
//   * On disk / cartridge / export / Godot: CANONICAL snake_case keys
//     (`through_ball`) as defined by shared/schemas/control_profile.schema.json.
//   * Conversion happens only at the boundary: `profileToCanonical()` writes,
//     `sanitizeControlProfile()` reads (it accepts both spellings, so older
//     camelCase localStorage / exported files keep loading).
//
// This module is imported by app.js and by static/tests/control_profile.test.mjs.
// =========================================================================

// InputFrame button bitmasks (must match sim/psfc-sim/src/input.rs)
export const BTN_PASS = 1 << 0;
export const BTN_THROUGH = 1 << 1;
export const BTN_LOB = 1 << 2;
export const BTN_SHOOT = 1 << 3;
export const BTN_SPRINT = 1 << 4;
export const BTN_SWITCH = 1 << 5;
export const BTN_JOCKEY = 1 << 6;
export const BTN_TACKLE = 1 << 7;
export const BTN_SLIDE = 1 << 8;
export const BTN_CALL_RUN = 1 << 9;
export const BTN_SKILL = 1 << 10;
export const BTN_FINESSE = 1 << 11;
export const BTN_CONTAIN = 1 << 12;
export const BTN_TEAMMATE_CONTAIN = 1 << 13;
export const BTN_GK_RUSH = 1 << 14;
export const BTN_PACE_CONTROL = 1 << 15;

export const CONTROL_PROFILE_VERSION = 2;
export const CONTROL_PROFILE_SCHEMA_ID = "https://classicfutbol.local/schemas/control_profile.schema.json";

/** Physical inputs an action may be bound to (`null` = unbound). */
export const PHYSICAL_BINDABLE_INPUTS = ["south", "east", "west", "north", "rt", "rb", "lt", "lb", "select"];
export const PRESET_NAMES = ["classic", "alternate", "two_button", "custom"];

/** Action key -> InputFrame bit, per context. Sprint is special-cased (analog). */
export const ATTACK_ACTION_BITS = {
  pass: BTN_PASS,
  shoot: BTN_SHOOT,
  cross: BTN_LOB,
  throughBall: BTN_THROUGH,
  sprint: BTN_SPRINT,
  finesse: BTN_FINESSE,
  paceControl: BTN_PACE_CONTROL,
  chipRun: BTN_CALL_RUN,
};
export const DEFENSE_ACTION_BITS = {
  contain: BTN_CONTAIN,
  standingTackle: BTN_TACKLE,
  slidingTackle: BTN_SLIDE,
  rushGk: BTN_GK_RUSH,
  sprint: BTN_SPRINT,
  teammateContain: BTN_TEAMMATE_CONTAIN,
  jockey: BTN_JOCKEY,
  switchPlayer: BTN_SWITCH,
};
/** Pass-family actions: when two or more share ONE physical input the
 *  context-sensitive pass resolver picks exactly one of them per press. */
export const PASS_FAMILY_ACTIONS = ["pass", "throughBall", "cross"];

/*
 * TWO-BUTTON PRESET (FIFA 2010 World Cup -> FIFA 11 / FIFA 12 "Two Button")
 * -------------------------------------------------------------------------
 * EA's Two Button scheme, introduced with 2010 FIFA World Cup and carried into
 * FIFA 11/12, let newcomers play with the Left Stick plus two face buttons:
 * one button does EVERY pass (the game decides ground pass / through ball /
 * cross from the situation) and the other shoots (clears in your own half) or,
 * without the ball, tackles. Sprint and Change Player stayed available as
 * optional extras.
 *
 * Our implementation uses ONLY existing InputFrame bits — no sim change:
 *   Attack  South (A/✕)  pass + throughBall + cross share one input. The
 *                        client-side resolver (`resolveContextPass`) latches
 *                        ONE bit at the press edge: BTN_LOB when the ball is
 *                        in the wide attacking third (cross), BTN_THROUGH when
 *                        the stick is pushed hard toward goal from around the
 *                        halfway line or beyond, otherwise BTN_PASS. The latched
 *                        bit is held for the whole press, so the sim's power
 *                        charge (CHARGE_MASK) works unchanged.
 *           East  (B/○)  BTN_SHOOT (the sim already turns it into a clearance
 *                        in the defensive third).
 *           RT           Sprint (optional extra, analog/digital as configured).
 *   Defense South (A/✕)  Switch Player (BTN_SWITCH).
 *           East  (B/○)  standingTackle + contain share one input: BTN_TACKLE
 *                        fires on the press edge (the sim edge-triggers it) and
 *                        BTN_CONTAIN is held while the button stays down. With
 *                        Legacy defending (applied with the preset) that is the
 *                        FIFA 11 "press & tackle" button.
 *           RT           Sprint.
 * Every other action is unbound (`null`). Selecting the preset also applies
 * Assisted pass/through/shot/cross assistance and Legacy defending, as the
 * original casual mode did; players may change those afterwards.
 */
export const CONTROL_PRESETS = {
  classic: {
    label: "CLASSIC (FIFA)",
    description: "FIFA 10–12 Classic: A Pass · B Shoot · X Lob/Cross · Y Through · RT Sprint · RB Finesse · LT Pace/Skill · LB Chip/Run. Defending: A Contain · B Tackle · X Slide · Y Rush GK · RB Teammate Contain · LT Jockey · LB Switch.",
    attack: {
      pass: "south", // A / Cross -> Ground Pass / Header
      shoot: "east", // B / Circle -> Shoot / Volley / Clear
      cross: "west", // X / Square -> Lob Pass / Cross / Clear
      throughBall: "north", // Y / Triangle -> Through Ball
      sprint: "rt", // RT / R2 -> Sprint (Analog or Digital)
      finesse: "rb", // RB / R1 -> Finesse Shot Modifier
      paceControl: "lt", // LT / L2 -> Pace Control / Skill Modifier
      chipRun: "lb", // LB / L1 -> Chip Shot Modifier / Trigger Run
    },
    defense: {
      contain: "south", // A / Cross -> Contain (Hold)
      standingTackle: "east", // B / Circle -> Standing Tackle / Push-Pull (FIFA 12)
      slidingTackle: "west", // X / Square -> Sliding Tackle
      rushGk: "north", // Y / Triangle -> Rush Goalkeeper (Hold)
      sprint: "rt", // RT / R2 -> Sprint
      teammateContain: "rb", // RB / R1 -> Teammate Contain (Hold)
      jockey: "lt", // LT / L2 -> Jockey / Strafe
      switchPlayer: "lb", // LB / L1 -> Switch Player (Auto/Nearest)
    },
  },
  alternate: {
    label: "ALTERNATE (SWAP X-B)",
    description: "FIFA Alternate: Shoot moves to X/□ and Lob/Cross to B/○; everything else as Classic.",
    attack: {
      pass: "south",
      shoot: "west", // Alternate: X / Square shoots
      cross: "east", // Alternate: B / Circle crosses
      throughBall: "north",
      sprint: "rt",
      finesse: "rb",
      paceControl: "lt",
      chipRun: "lb",
    },
    defense: {
      contain: "south",
      standingTackle: "east",
      slidingTackle: "west",
      rushGk: "north",
      sprint: "rt",
      teammateContain: "rb",
      jockey: "lt",
      switchPlayer: "lb",
    },
  },
  two_button: {
    label: "TWO-BUTTON",
    description: "FIFA 11/12 Two Button (casual): A does every pass — the game picks ground pass, through ball or cross from the situation; B shoots/clears. Defending: A switches player, B tackles (hold to press). RT sprints. Applies Assisted passing/shooting and Legacy defending.",
    attack: {
      pass: "south",
      shoot: "east",
      cross: "south",
      throughBall: "south",
      sprint: "rt",
      finesse: null,
      paceControl: null,
      chipRun: null,
    },
    defense: {
      contain: "east",
      standingTackle: "east",
      slidingTackle: null,
      rushGk: null,
      sprint: "rt",
      teammateContain: null,
      jockey: null,
      switchPlayer: "south",
    },
    // Gameplay settings applied together with the preset (casual mode defaults).
    settings: {
      passAssist: "assisted",
      throughAssist: "assisted",
      shotAssist: "assisted",
      crossAssist: "assisted",
      defendingMode: "legacy",
    },
  },
};

export const ATTACK_ACTION_META = [
  { key: "pass", label: "Ground Pass / Header", short: "PASS" },
  { key: "shoot", label: "Shoot / Volley / Clear", short: "SHOOT" },
  { key: "cross", label: "Lob Pass / Cross / Clear", short: "CROSS" },
  { key: "throughBall", label: "Through Ball", short: "THROUGH" },
  { key: "sprint", label: "Sprint (Analog / Digital)", short: "SPRINT" },
  { key: "finesse", label: "Finesse Shot Modifier", short: "FINESSE" },
  { key: "paceControl", label: "Pace Control / Skill Modifier", short: "PACE" },
  { key: "chipRun", label: "Chip Modifier / Trigger Run", short: "CHIP/RUN" },
];

export const DEFENSE_ACTION_META = [
  { key: "contain", label: "Contain (Hold)", short: "CONTAIN" },
  { key: "standingTackle", label: "Standing Tackle / Push-Pull", short: "TACKLE" },
  { key: "slidingTackle", label: "Sliding Tackle", short: "SLIDE" },
  { key: "rushGk", label: "Rush Goalkeeper (Hold)", short: "RUSH GK" },
  { key: "sprint", label: "Sprint", short: "SPRINT" },
  { key: "teammateContain", label: "Teammate Contain (Hold)", short: "2ND MAN" },
  { key: "jockey", label: "Jockey / Strafe", short: "JOCKEY" },
  { key: "switchPlayer", label: "Switch Player (Auto / Nearest)", short: "SWITCH" },
];

const ASSIST_VALUES = ["assisted", "semi", "manual"];
const MOVE_ASSIST_VALUES = ["none", "low", "high"];
const AUTO_SWITCH_VALUES = ["auto", "air_balls", "manual"];

// ---------------------------------------------------------------------------
// Key-case conversion at the serialization boundary
// ---------------------------------------------------------------------------
export function camelToSnake(k) {
  return String(k).replace(/[A-Z]/g, (c) => `_${c.toLowerCase()}`);
}
export function snakeToCamel(k) {
  return String(k).replace(/_([a-z])/g, (_m, c) => c.toUpperCase());
}

function pickKey(raw, camelKey) {
  if (!raw || typeof raw !== "object") return undefined;
  if (Object.prototype.hasOwnProperty.call(raw, camelKey)) return raw[camelKey];
  const snake = camelToSnake(camelKey);
  if (Object.prototype.hasOwnProperty.call(raw, snake)) return raw[snake];
  return undefined;
}

// ---------------------------------------------------------------------------
// Profile construction / validation
// ---------------------------------------------------------------------------
export function createDefaultControlProfile(presetName = "classic") {
  const name = CONTROL_PRESETS[presetName] ? presetName : "classic";
  const base = CONTROL_PRESETS[name];
  const prof = {
    version: CONTROL_PROFILE_VERSION,
    preset: name,
    autoSwitch: "auto", // "auto" | "air_balls" | "manual"
    autoSwitchMoveAssist: "low", // "none" | "low" | "high"
    passAssist: "assisted", // "assisted" | "semi" | "manual"
    throughAssist: "assisted",
    shotAssist: "assisted",
    crossAssist: "assisted",
    defendingMode: "tactical", // "tactical" | "legacy"
    analogSprint: true,
    attack: { ...base.attack },
    defense: { ...base.defense },
  };
  if (base.settings) Object.assign(prof, base.settings);
  return prof;
}

/**
 * Accepts a profile in canonical snake_case (schema / export / Godot) OR the
 * legacy camelCase web form and returns a fully-populated in-memory profile.
 * Missing / invalid bindings fall back to the preset's binding; an explicit
 * `null` is preserved as "unbound".
 */
export function sanitizeControlProfile(raw) {
  const def = createDefaultControlProfile("classic");
  if (!raw || typeof raw !== "object" || Array.isArray(raw)) return def;
  const preset = PRESET_NAMES.includes(raw.preset) ? raw.preset : "classic";
  const validAssist = (v) => (ASSIST_VALUES.includes(v) ? v : "assisted");
  const validMove = (v) => (MOVE_ASSIST_VALUES.includes(v) ? v : "low");
  const validSw = (v) => (AUTO_SWITCH_VALUES.includes(v) ? v : "auto");
  const validPhys = (map, k, fb) => {
    const v = pickKey(map, k);
    if (v === null) return null;
    return PHYSICAL_BINDABLE_INPUTS.includes(v) ? v : fb;
  };
  // Custom profiles fall back to Classic for any binding that is missing.
  const presetBase = CONTROL_PRESETS[preset] || CONTROL_PRESETS.classic;
  const attack = {};
  for (const k of Object.keys(ATTACK_ACTION_BITS)) {
    attack[k] = validPhys(raw.attack, k, presetBase.attack[k]);
  }
  const defense = {};
  for (const k of Object.keys(DEFENSE_ACTION_BITS)) {
    defense[k] = validPhys(raw.defense, k, presetBase.defense[k]);
  }
  const analog = pickKey(raw, "analogSprint");
  return {
    version: CONTROL_PROFILE_VERSION,
    preset,
    autoSwitch: validSw(pickKey(raw, "autoSwitch")),
    autoSwitchMoveAssist: validMove(pickKey(raw, "autoSwitchMoveAssist")),
    passAssist: validAssist(pickKey(raw, "passAssist")),
    throughAssist: validAssist(pickKey(raw, "throughAssist")),
    shotAssist: validAssist(pickKey(raw, "shotAssist")),
    crossAssist: validAssist(pickKey(raw, "crossAssist")),
    defendingMode: pickKey(raw, "defendingMode") === "legacy" ? "legacy" : "tactical",
    analogSprint: typeof analog === "boolean" ? analog : true,
    attack,
    defense,
  };
}

/** In-memory profile -> canonical snake_case JSON object (schema v2). */
export function profileToCanonical(prof) {
  const p = sanitizeControlProfile(prof);
  const mapOut = (m) => {
    const out = {};
    for (const [k, v] of Object.entries(m)) out[camelToSnake(k)] = v;
    return out;
  };
  return {
    $schema: CONTROL_PROFILE_SCHEMA_ID,
    version: CONTROL_PROFILE_VERSION,
    preset: p.preset,
    auto_switch: p.autoSwitch,
    auto_switch_move_assist: p.autoSwitchMoveAssist,
    pass_assist: p.passAssist,
    through_assist: p.throughAssist,
    shot_assist: p.shotAssist,
    cross_assist: p.crossAssist,
    defending_mode: p.defendingMode,
    analog_sprint: p.analogSprint,
    attack: mapOut(p.attack),
    defense: mapOut(p.defense),
  };
}

/** Applies a named preset's bindings (and its bundled settings, if any) in place. */
export function applyPresetToProfile(prof, presetName) {
  const preset = CONTROL_PRESETS[presetName];
  if (!preset) {
    if (presetName === "custom") prof.preset = "custom";
    return prof;
  }
  prof.preset = presetName;
  prof.attack = { ...preset.attack };
  prof.defense = { ...preset.defense };
  if (preset.settings) Object.assign(prof, preset.settings);
  return prof;
}

/**
 * Rebinds `actionKey` in `context` to `newPhys` (or `null` to unbind).
 * A collision inside the same context is auto-swapped onto the previous
 * physical input so two actions never accidentally double-fire.
 * Returns true when something changed.
 */
export function rebindProfileAction(prof, context, actionKey, newPhys) {
  if ((context !== "attack" && context !== "defense") || !prof[context]) return false;
  if (newPhys !== null && !PHYSICAL_BINDABLE_INPUTS.includes(newPhys)) return false;
  const map = prof[context];
  if (!Object.prototype.hasOwnProperty.call(map, actionKey)) return false;
  const prevPhys = map[actionKey];
  if (prevPhys === newPhys) return false;
  if (newPhys !== null) {
    for (const otherKey of Object.keys(map)) {
      if (otherKey !== actionKey && map[otherKey] === newPhys) {
        map[otherKey] = prevPhys === undefined ? null : prevPhys;
        break;
      }
    }
  }
  map[actionKey] = newPhys;
  prof.preset = "custom";
  return true;
}

export function autoSwitchCodeFromName(name) {
  return name === "air_balls" ? 1 : name === "manual" ? 2 : 0;
}

/** 14-bit ControlSettings word tagged 0x4000 (== ControlSettings::encode_reserved). */
export function encodeControlSettingsReserved(prof) {
  const assistCode = (v) => (v === "semi" ? 1 : v === "manual" ? 2 : 0);
  const moveAssistCode = (v) => (v === "low" ? 1 : v === "high" ? 2 : 0);
  const defBit = prof.defendingMode === "legacy" ? 1 : 0;
  const analogBit = prof.analogSprint ? 1 : 0;
  return (
    (0x4000 |
      (assistCode(prof.passAssist) & 0x3) |
      ((assistCode(prof.throughAssist) & 0x3) << 2) |
      ((assistCode(prof.shotAssist) & 0x3) << 4) |
      ((assistCode(prof.crossAssist) & 0x3) << 6) |
      ((defBit & 0x1) << 8) |
      ((moveAssistCode(prof.autoSwitchMoveAssist) & 0x3) << 9) |
      ((analogBit & 0x1) << 11) |
      ((autoSwitchCodeFromName(prof.autoSwitch) & 0x3) << 12)) &
    0xffff
  );
}

// ---------------------------------------------------------------------------
// Layer 2: physical snapshot -> InputFrame bits
// ---------------------------------------------------------------------------
/**
 * Evaluates both context maps into the 16-bit `buttons` word + 7-bit analog
 * sprint. When several pass-family actions share the pressed physical input
 * (Two-Button preset), their bits are NOT OR'd; they are returned in
 * `ctxPassMask` for `resolveContextPass` to pick exactly one.
 */
export function mapPhysicalToSimInput(phys, prof, isAttacking = true) {
  const p = phys.buttons;
  const pressed = (key) => Boolean(key && p[key]);
  const triggerDepth = (key) => {
    if (!key) return 0.0;
    if (key === "rt") return phys.rtValue;
    if (key === "lt") return phys.ltValue;
    return pressed(key) ? 1.0 : 0.0;
  };

  let buttons = 0;
  let ctxPassMask = 0;

  // --- In Possession (Attacking) Map ---
  for (const [action, bit] of Object.entries(ATTACK_ACTION_BITS)) {
    if (action === "sprint") continue;
    const key = prof.attack[action];
    if (!pressed(key)) continue;
    if (PASS_FAMILY_ACTIONS.includes(action)) {
      const sharers = PASS_FAMILY_ACTIONS.filter((a) => prof.attack[a] === key);
      if (sharers.length > 1) {
        ctxPassMask |= bit;
        continue;
      }
    }
    buttons |= bit;
  }

  // --- Out of Possession (Defending) Map ---
  for (const [action, bit] of Object.entries(DEFENSE_ACTION_BITS)) {
    if (action === "sprint") continue;
    if (pressed(prof.defense[action])) buttons |= bit;
  }

  // --- Sprint (Analog or Digital past 0.10 deadzone), resolved strictly from active context ---
  const activeSprintKey = isAttacking ? prof.attack.sprint : prof.defense.sprint;
  const sprintDepth = triggerDepth(activeSprintKey);
  let sprintAnalog = 0;
  if (sprintDepth > 0.1) {
    buttons |= BTN_SPRINT;
    sprintAnalog = prof.analogSprint ? Math.max(1, Math.min(127, Math.round(sprintDepth * 127))) : 127;
  }

  // --- Skill Move Button (Select / L3 / R3; Right Stick also triggers directional skills/knock-on) ---
  if (p.select || p.l3 || p.r3) buttons |= BTN_SKILL;

  // SNES retro pad convenience: L also allows Finesse when shooting since SNES lacks separate RB
  if (phys.isSnesMode && p.lb) {
    buttons |= BTN_FINESSE;
  }

  // Full-magnitude 360° vectors (no 8-way snapping): round to the i8 wire range only.
  const mx = Math.max(-127, Math.min(127, Math.round(phys.lx * 127)));
  const my = Math.max(-127, Math.min(127, Math.round(phys.ly * 127)));
  const ax = Math.max(-127, Math.min(127, Math.round(phys.rx * 127)));
  const ay = Math.max(-127, Math.min(127, Math.round(phys.ry * 127)));

  return {
    mx,
    my,
    ax,
    ay,
    btns: buttons,
    ctxPassMask,
    sprintAnalog,
    south: p.south,
    east: p.east,
    west: p.west,
    north: p.north,
    l1: p.lb,
    r1: p.rb,
    l2: p.lt,
    r2: p.rt,
    ltValue: phys.ltValue,
    rtValue: phys.rtValue,
    select: p.select,
    start: p.start,
    lx: phys.lx,
    ly: phys.ly,
    rx: phys.rx,
    ry: phys.ry,
    phys,
    id: phys.id,
    rawPad: phys.rawPad,
  };
}

/**
 * Two-Button context-sensitive pass choice (pure).
 * ctx: { forward: ball position along own attacking direction (-1 own goal line .. +1 opponent goal line),
 *        wide: |ball y| / half pitch width (0 centre .. 1 touchline),
 *        stickFwd: world stick component toward opponent goal (-1..1),
 *        stickMag: world stick magnitude (0..1) } — any field may be missing.
 */
export function resolveContextPass(mask, ctx = {}) {
  if (!mask) return 0;
  const forward = Number.isFinite(ctx.forward) ? ctx.forward : 0;
  const wide = Number.isFinite(ctx.wide) ? ctx.wide : 0;
  const stickMag = Number.isFinite(ctx.stickMag) ? ctx.stickMag : 0;
  const stickFwd = Number.isFinite(ctx.stickFwd) ? ctx.stickFwd : 0;
  if (mask & BTN_LOB && forward > 0.35 && wide > 0.55) return BTN_LOB;
  if (mask & BTN_THROUGH && forward > -0.15 && stickMag > 0.6 && stickFwd / Math.max(stickMag, 1e-6) > 0.8) {
    return BTN_THROUGH;
  }
  if (mask & BTN_PASS) return BTN_PASS;
  if (mask & BTN_THROUGH) return BTN_THROUGH;
  return BTN_LOB;
}

/** Per-side latch so the resolved bit stays constant for a whole press (power charge). */
export function createContextPassLatch() {
  return { bit: 0 };
}
export function applyContextPassLatch(latch, mask, ctx) {
  if (!mask) {
    latch.bit = 0;
    return 0;
  }
  if (!latch.bit || !(mask & latch.bit)) latch.bit = resolveContextPass(mask, ctx);
  return latch.bit;
}

// ---------------------------------------------------------------------------
// Button prompts / glyphs (action -> physical -> glyph for detected pad family)
// ---------------------------------------------------------------------------
export const GLYPH_TABLES = {
  // Generic / unknown pad: dual Xbox / PlayStation label
  modern: {
    south: "A / ✕", east: "B / ○", west: "X / □", north: "Y / △",
    rt: "RT / R2", rb: "RB / R1", lt: "LT / L2", lb: "LB / L1",
    select: "VIEW / SHARE", start: "MENU / OPTIONS", lstick: "L-STICK", rstick: "R-STICK",
  },
  xbox: {
    south: "A", east: "B", west: "X", north: "Y",
    rt: "RT", rb: "RB", lt: "LT", lb: "LB",
    select: "VIEW", start: "MENU", lstick: "LS", rstick: "RS",
  },
  ps: {
    south: "✕", east: "○", west: "□", north: "△",
    rt: "R2", rb: "R1", lt: "L2", lb: "L1",
    select: "SHARE", start: "OPTIONS", lstick: "L3", rstick: "R3",
  },
  // Nintendo positional labels: bottom = B, right = A, left = Y, top = X
  switch: {
    south: "B", east: "A", west: "Y", north: "X",
    rt: "ZR", rb: "R", lt: "ZL", lb: "L",
    select: "−", start: "+", lstick: "L-STICK", rstick: "R-STICK",
  },
  // SNES: single L/R shoulders double as triggers (see samplePhysicalGamepad)
  snes: {
    south: "B", east: "A", west: "Y", north: "X",
    rt: "R", rb: "R", lt: "L", lb: "L",
    select: "SELECT", start: "START", lstick: "D-PAD", rstick: "—",
  },
  keyboard: {
    south: "Space / J", east: "L / Enter", west: "U / C", north: "K",
    rt: "Shift", rb: "E", lt: "F", lb: "Q",
    select: "Z", start: "Esc", lstick: "WASD", rstick: "—",
  },
  keyboard_p2: {
    south: "Num1 / .", east: "Num2 / /", west: "Num4", north: "Num3",
    rt: "R-Ctrl", rb: "—", lt: "—", lb: ",",
    select: "Num0", start: "—", lstick: "Arrows", rstick: "—",
  },
};
// Back-compat alias for the legacy style name used by formatPhysicalGlyph().
GLYPH_TABLES.psfc = GLYPH_TABLES.modern;

export const PROMPT_FAMILIES = Object.keys(GLYPH_TABLES);

export function formatPhysicalGlyph(physKey, family = "modern") {
  if (physKey === null || physKey === undefined || physKey === "") return "—";
  const tbl = GLYPH_TABLES[family] || GLYPH_TABLES.modern;
  return tbl[physKey] || String(physKey).toUpperCase();
}

/** Pseudo-actions bound to the sticks (not rebindable). */
const STICK_ACTIONS = {
  attack: { dribble: "lstick", skillMove: "rstick" },
  defense: { move: "lstick", manualSwitch: "rstick" },
};

/** action -> physical -> glyph. Returns "—" for an unbound action. */
export function glyphFor(prof, actionKey, context = "attack", family = "modern") {
  const stick = STICK_ACTIONS[context] && STICK_ACTIONS[context][actionKey];
  if (stick) return formatPhysicalGlyph(stick, family);
  const map = prof && prof[context];
  if (!map) return "—";
  return formatPhysicalGlyph(map[actionKey], family);
}

/**
 * Compact prompt list for the HUD: one entry per bound physical input, with
 * every action that shares it (e.g. Two-Button "A: PASS/THROUGH/CROSS").
 */
export function buildPromptEntries(prof, context = "attack", family = "modern") {
  const meta = context === "defense" ? DEFENSE_ACTION_META : ATTACK_ACTION_META;
  const map = (prof && prof[context]) || {};
  const byPhys = new Map();
  for (const item of meta) {
    const phys = map[item.key];
    if (!phys) continue;
    if (!byPhys.has(phys)) byPhys.set(phys, []);
    byPhys.get(phys).push(item.short);
  }
  const out = [];
  for (const [phys, shorts] of byPhys) {
    const shared = shorts.length > 1;
    out.push({
      phys,
      glyph: formatPhysicalGlyph(phys, family),
      label: shared && context === "attack" && shorts.includes("PASS") ? `${shorts.join("/")} (AUTO)` : shorts.join("/"),
    });
  }
  return out;
}
