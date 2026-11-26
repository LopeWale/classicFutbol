// Node test for the pure ControlProfile module (no test runner needed):
//   node host/psfc_host/web/static/tests/control_profile.test.mjs
// Optional: pass a directory to write exported canonical profiles for schema validation:
//   node .../control_profile.test.mjs /tmp/out   (then validate with shared/schemas/control_profile.schema.json)
import assert from "node:assert/strict";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import * as cp from "../control_profile.js";

let passed = 0;
const test = (name, fn) => {
  fn();
  passed++;
  console.log(`ok - ${name}`);
};

const physSnapshot = (over = {}) => ({
  lx: 0,
  ly: 0,
  rx: 0,
  ry: 0,
  ltValue: 0,
  rtValue: 0,
  isSnesMode: false,
  id: "test",
  rawPad: null,
  ...over,
  buttons: {
    south: false, east: false, west: false, north: false,
    lb: false, rb: false, lt: false, rt: false,
    select: false, start: false, l3: false, r3: false,
    ...(over.buttons || {}),
  },
});

test("Classic preset matches the FIFA 10–12 table exactly", () => {
  const c = cp.CONTROL_PRESETS.classic;
  assert.deepEqual(c.attack, {
    pass: "south", shoot: "east", cross: "west", throughBall: "north",
    sprint: "rt", finesse: "rb", paceControl: "lt", chipRun: "lb",
  });
  assert.deepEqual(c.defense, {
    contain: "south", standingTackle: "east", slidingTackle: "west", rushGk: "north",
    sprint: "rt", teammateContain: "rb", jockey: "lt", switchPlayer: "lb",
  });
});

test("Classic: each physical input emits attack bit | defense bit (dual map)", () => {
  const prof = cp.createDefaultControlProfile("classic");
  const expect = {
    south: cp.BTN_PASS | cp.BTN_CONTAIN,
    east: cp.BTN_SHOOT | cp.BTN_TACKLE,
    west: cp.BTN_LOB | cp.BTN_SLIDE,
    north: cp.BTN_THROUGH | cp.BTN_GK_RUSH,
    rb: cp.BTN_FINESSE | cp.BTN_TEAMMATE_CONTAIN,
    lb: cp.BTN_CALL_RUN | cp.BTN_SWITCH,
  };
  for (const [k, bits] of Object.entries(expect)) {
    const out = cp.mapPhysicalToSimInput(physSnapshot({ buttons: { [k]: true } }), prof, true);
    assert.equal(out.btns, bits, k);
    assert.equal(out.ctxPassMask, 0);
  }
  const lt = cp.mapPhysicalToSimInput(physSnapshot({ ltValue: 0.8, buttons: { lt: true } }), prof, true);
  assert.equal(lt.btns, cp.BTN_PACE_CONTROL | cp.BTN_JOCKEY);
});

test("360° stick magnitude preserved (no 8-way snap)", () => {
  const prof = cp.createDefaultControlProfile("classic");
  const a = 0.3; // 30° off-axis, partial magnitude
  const out = cp.mapPhysicalToSimInput(physSnapshot({ lx: 0.55 * Math.cos(a), ly: 0.55 * Math.sin(a) }), prof);
  assert.equal(out.mx, Math.round(0.55 * Math.cos(a) * 127));
  assert.equal(out.my, Math.round(0.55 * Math.sin(a) * 127));
  assert.ok(Math.abs(Math.hypot(out.mx, out.my) / 127 - 0.55) < 0.01);
});

test("Analog sprint: depth scales 1..127; off => 127 past 0.10 deadzone; <=0.10 => no sprint", () => {
  const on = cp.createDefaultControlProfile("classic");
  const off = { ...on, analogSprint: false };
  const half = physSnapshot({ rtValue: 0.5, buttons: { rt: true } });
  assert.equal(cp.mapPhysicalToSimInput(half, on).sprintAnalog, 64);
  assert.ok(cp.mapPhysicalToSimInput(half, on).btns & cp.BTN_SPRINT);
  assert.equal(cp.mapPhysicalToSimInput(half, off).sprintAnalog, 127);
  const light = physSnapshot({ rtValue: 0.11, buttons: { rt: true } });
  assert.equal(cp.mapPhysicalToSimInput(light, off).sprintAnalog, 127);
  const dz = physSnapshot({ rtValue: 0.1 });
  assert.equal(cp.mapPhysicalToSimInput(dz, off).btns & cp.BTN_SPRINT, 0);
  assert.equal(cp.mapPhysicalToSimInput(dz, off).sprintAnalog, 0);
});

test("Two-Button: South resolves to exactly one pass bit; East = shoot | tackle | contain", () => {
  const prof = cp.createDefaultControlProfile("two_button");
  assert.equal(prof.defendingMode, "legacy");
  const south = cp.mapPhysicalToSimInput(physSnapshot({ buttons: { south: true } }), prof, true);
  assert.equal(south.ctxPassMask, cp.BTN_PASS | cp.BTN_THROUGH | cp.BTN_LOB);
  assert.equal(south.btns, cp.BTN_SWITCH); // defense half of South
  const east = cp.mapPhysicalToSimInput(physSnapshot({ buttons: { east: true } }), prof, true);
  assert.equal(east.btns, cp.BTN_SHOOT | cp.BTN_TACKLE | cp.BTN_CONTAIN);
  for (const k of ["west", "north", "rb", "lb"]) {
    const o = cp.mapPhysicalToSimInput(physSnapshot({ buttons: { [k]: true } }), prof, true);
    assert.equal(o.btns, 0, k);
  }
  const m = south.ctxPassMask;
  assert.equal(cp.resolveContextPass(m, { forward: 0.1, wide: 0.1, stickFwd: 0.2, stickMag: 0.3 }), cp.BTN_PASS);
  assert.equal(cp.resolveContextPass(m, { forward: 0.2, wide: 0.2, stickFwd: 0.95, stickMag: 1 }), cp.BTN_THROUGH);
  assert.equal(cp.resolveContextPass(m, { forward: 0.7, wide: 0.8, stickFwd: 0.95, stickMag: 1 }), cp.BTN_LOB);
  assert.equal(cp.resolveContextPass(m, { forward: -0.6, wide: 0.1, stickFwd: 1, stickMag: 1 }), cp.BTN_PASS);
  assert.equal(cp.resolveContextPass(m, {}), cp.BTN_PASS);
  // Latch keeps the press-edge choice for the whole hold (power charge)
  const latch = cp.createContextPassLatch();
  assert.equal(cp.applyContextPassLatch(latch, m, { forward: 0.2, stickFwd: 1, stickMag: 1 }), cp.BTN_THROUGH);
  assert.equal(cp.applyContextPassLatch(latch, m, { forward: 0.2, stickFwd: 0, stickMag: 0 }), cp.BTN_THROUGH);
  assert.equal(cp.applyContextPassLatch(latch, 0, {}), 0);
  assert.equal(cp.applyContextPassLatch(latch, m, {}), cp.BTN_PASS);
});

test("Canonical serialization round-trips (snake_case on disk, camelCase in memory)", () => {
  for (const name of ["classic", "alternate", "two_button"]) {
    const prof = cp.createDefaultControlProfile(name);
    const canon = cp.profileToCanonical(prof);
    assert.ok("through_ball" in canon.attack && "switch_player" in canon.defense);
    assert.ok(!("throughBall" in canon.attack));
    assert.deepEqual(cp.sanitizeControlProfile(JSON.parse(JSON.stringify(canon))), prof);
    assert.deepEqual(cp.sanitizeControlProfile(prof), prof); // legacy camelCase still loads
  }
  const junk = cp.sanitizeControlProfile({ preset: "nope", attack: { pass: "z" }, pass_assist: "x" });
  assert.equal(junk.preset, "classic");
  assert.equal(junk.attack.pass, "south");
  assert.equal(junk.passAssist, "assisted");
});

test("Rebind: collision auto-swap, unbind with null, preset becomes custom", () => {
  const prof = cp.createDefaultControlProfile("classic");
  assert.ok(cp.rebindProfileAction(prof, "attack", "shoot", "west"));
  assert.equal(prof.attack.shoot, "west");
  assert.equal(prof.attack.cross, "east");
  assert.equal(prof.preset, "custom");
  assert.ok(cp.rebindProfileAction(prof, "defense", "rushGk", null));
  assert.equal(prof.defense.rushGk, null);
  assert.ok(!cp.rebindProfileAction(prof, "attack", "shoot", "bogus"));
});

test("Settings word matches ControlSettings::encode_reserved bit layout", () => {
  const prof = cp.createDefaultControlProfile("classic");
  Object.assign(prof, {
    passAssist: "semi", throughAssist: "manual", shotAssist: "assisted", crossAssist: "semi",
    defendingMode: "legacy", autoSwitchMoveAssist: "high", analogSprint: false, autoSwitch: "manual",
  });
  const w = cp.encodeControlSettingsReserved(prof);
  assert.equal(w, 0x4000 | 1 | (2 << 2) | (0 << 4) | (1 << 6) | (1 << 8) | (2 << 9) | (0 << 11) | (2 << 12));
  assert.equal(w & 0xc000, 0x4000);
});

test("glyphFor resolves action -> physical -> pad-family glyph and follows rebinds", () => {
  const prof = cp.createDefaultControlProfile("classic");
  assert.equal(cp.glyphFor(prof, "pass", "attack", "xbox"), "A");
  assert.equal(cp.glyphFor(prof, "pass", "attack", "ps"), "✕");
  assert.equal(cp.glyphFor(prof, "pass", "attack", "switch"), "B");
  assert.equal(cp.glyphFor(prof, "pass", "attack", "snes"), "B");
  assert.equal(cp.glyphFor(prof, "pass", "attack", "keyboard"), "Space / J");
  assert.equal(cp.glyphFor(prof, "switchPlayer", "defense", "ps"), "L1");
  assert.equal(cp.glyphFor(prof, "dribble", "attack", "xbox"), "LS");
  cp.rebindProfileAction(prof, "attack", "pass", "north");
  assert.equal(cp.glyphFor(prof, "pass", "attack", "xbox"), "Y");
  const two = cp.createDefaultControlProfile("two_button");
  assert.equal(cp.glyphFor(two, "finesse", "attack", "xbox"), "—");
  const entries = cp.buildPromptEntries(two, "attack", "xbox");
  assert.deepEqual(entries[0], { phys: "south", glyph: "A", label: "PASS/CROSS/THROUGH (AUTO)" });
  assert.equal(cp.buildPromptEntries(two, "defense", "ps").find((e) => e.phys === "east").label, "CONTAIN/TACKLE");
});

const outDir = process.argv[2];
if (outDir) {
  mkdirSync(outDir, { recursive: true });
  for (const name of ["classic", "alternate", "two_button"]) {
    writeFileSync(join(outDir, `${name}.json`), JSON.stringify(cp.profileToCanonical(cp.createDefaultControlProfile(name)), null, 2));
  }
  const custom = cp.createDefaultControlProfile("classic");
  cp.rebindProfileAction(custom, "defense", "rushGk", null);
  writeFileSync(join(outDir, "custom.json"), JSON.stringify(cp.profileToCanonical(custom), null, 2));
}

console.log(`\n${passed} tests passed`);
