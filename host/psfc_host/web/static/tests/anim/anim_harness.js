#!/usr/bin/env node
/*
 * Animation smoothness harness (renderer side).
 *
 * Drives the attract-mode CPU-vs-CPU match headlessly with a VIRTUAL clock
 * (performance.now + requestAnimationFrame are replaced in-page), so every run
 * sees the same frame cadence and the same sim ticks. Per frame and per actor it
 * records root position, root yaw, hip/spine/chest/knee bone rotations, ankle
 * world positions and GK flags, then reports jerk/pop statistics.
 *
 * It also taps wasm.sim_tick to log the packed sim stream (one record per 60 Hz
 * tick) so sim-side jitter (velocity / facing flips) can be separated from
 * renderer-side jitter.
 *
 * Usage:
 *   node anim_harness.js --url http://127.0.0.1:8765/ --label before \
 *        --out docs/anim --seconds 30 --cadences 120,60,40,60j
 *
 *   cadence "60j" = 60 fps with +-4 ms seeded frame-time jitter (real-browser like).
 *   cadence "40"  = throttled 40 fps (forces 1-2 sim steps per frame).
 */
"use strict";

const fs = require("fs");
const path = require("path");
const { chromium } = require("playwright");

function arg(name, def) {
  const i = process.argv.indexOf("--" + name);
  return i >= 0 && i + 1 < process.argv.length ? process.argv[i + 1] : def;
}

const URL = arg("url", "http://127.0.0.1:8765/");
const LABEL = arg("label", "run");
const OUT = path.resolve(arg("out", "docs/anim"));
const SECONDS = Number(arg("seconds", "30"));
const WARMUP_S = Number(arg("warmup", "4"));
const CADENCES = arg("cadences", "120,60,40,60j").split(",");
const SHOTS = arg("shots", "1") !== "0";

fs.mkdirSync(OUT, { recursive: true });

// ---------------------------------------------------------------------------
// In-page init: virtual clock, manual rAF, seeded Math.random
// ---------------------------------------------------------------------------
const INIT_SCRIPT = `
(() => {
  let vt = 1000;
  const realNow = performance.now.bind(performance);
  performance.now = () => vt;
  let queue = [];
  let idSeq = 1;
  window.requestAnimationFrame = (cb) => { const id = idSeq++; queue.push({ id, cb }); return id; };
  window.cancelAnimationFrame = (id) => { queue = queue.filter((q) => q.id !== id); };
  window.__animStep = (ms) => {
    vt += ms;
    const q = queue; queue = [];
    for (const { cb } of q) { try { cb(vt); } catch (e) { console.error("rAF cb error", e && e.stack || e); } }
    return q.length;
  };
  window.__animNow = () => vt;
  window.__animRealNow = realNow;
  let s = 0x9e3779b9 >>> 0;
  Math.random = () => {
    s = (s + 0x6d2b79f5) >>> 0;
    let t = s;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
})();
`;

// ---------------------------------------------------------------------------
// In-page recorder (installed after the engine is up)
// ---------------------------------------------------------------------------
function installRecorder() {
  const diag = window.__psfcDiagnostics;
  const eng = diag.engine3d;
  const wasm = diag.getWasm();
  const B = { hips: 1, spine: 2, chest: 3, kneeL: 11, kneeR: 14, ankleL: 12, ankleR: 15 };
  const BONES = ["hips", "spine", "chest", "kneeL", "kneeR"];

  const rec = {
    frames: 0,
    ticks: [],
    prev: new Map(),
    s: {
      posAcc: [], yawAcc: [], yawVel: [], hipBobAcc: [],
      hipsAcc: [], spineAcc: [], chestAcc: [], kneeAcc: [],
      gkPosAcc: [], gkYawAcc: [], gkHipsAcc: [], gkSpineAcc: [], gkKneeAcc: [],
      skate: [], gkSkate: [], pinSkate: [], movingSkate: [],
    },
    cnt: {
      teleports: 0, slides: 0, posPops: 0, yawPops: 0, bonePops: 0, gkBonePops: 0, skatePops: 0,
      bobPops: 0, samples: 0, plantedSamples: 0,
    },
    headerEvents: [],
    ownerChanges: 0,
    ownerChangePopFrames: 0,
    lastOwner: null,
    ownerWindow: 0,
  };
  window.__animRec = rec;

  // Sim stream: wasm exports are frozen (cannot wrap sim_tick), so read the packed
  // state once per rendered frame and keep it when it changed. At 120 fps every
  // 60 Hz tick is observed (0 or 1 step per frame); the sim analysis uses that run.
  let lastKey = null;
  rec.captureTick = () => {
    const n = wasm.sim_pack_render_state();
    if (!n) return null;
    const p = new Int32Array(wasm.memory.buffer, wasm.wasm_packed_state_ptr(), n);
    const pps = p[6];
    const np = pps * 2;
    const row = new Float32Array(2 + np * 6);
    row[0] = p[3];
    row[1] = p[15] * 32 + p[16];
    let key = p[1] * 7 + p[9] + p[10] * 3;
    for (let k = 0; k < np; k++) {
      const b = 18 + k * 10;
      row[2 + k * 6 + 0] = p[b + 4];
      row[2 + k * 6 + 1] = p[b + 5];
      row[2 + k * 6 + 2] = p[b + 6];
      row[2 + k * 6 + 3] = p[b + 7];
      row[2 + k * 6 + 4] = p[b + 8];
      row[2 + k * 6 + 5] = (p[b + 9] >>> 0) & 0xf;
      key += p[b + 4] * (k + 1) + p[b + 5] * (k + 3);
    }
    if (rec.recording && key !== lastKey) rec.ticks.push(row);
    lastKey = key;
    return row[1];
  };

  // Skip GPU presentation while measuring (animation runs before present);
  // rec.present(true) re-enables it for screenshot frames.
  const origPresent = eng._presentScene.bind(eng);
  rec.presentOn = false;
  eng._presentScene = () => { if (rec.presentOn) origPresent(); };
  rec.renderRaw = origPresent;

  const qPrev = new eng.camera.quaternion.constructor();
  const tmpQ = new eng.camera.quaternion.constructor();
  const V3 = eng.camera.position.constructor;
  const tmpV = new V3();

  function angVel(qNow, qOld, dt, out) {
    // omega = axis * angle of (qNow * qOld^-1) / dt
    tmpQ.copy(qOld).invert().premultiply(qNow);
    if (tmpQ.w < 0) { tmpQ.x = -tmpQ.x; tmpQ.y = -tmpQ.y; tmpQ.z = -tmpQ.z; tmpQ.w = -tmpQ.w; }
    const sinH = Math.hypot(tmpQ.x, tmpQ.y, tmpQ.z);
    const ang = 2 * Math.atan2(sinH, tmpQ.w);
    const k = sinH > 1e-9 ? ang / sinH / dt : 0;
    out[0] = tmpQ.x * k; out[1] = tmpQ.y * k; out[2] = tmpQ.z * k;
    return out;
  }
  const wrap = (a) => { while (a > Math.PI) a -= 2 * Math.PI; while (a < -Math.PI) a += 2 * Math.PI; return a; };

  rec.sample = (dtMs) => {
    const dt = dtMs / 1000;
    rec.frames++;
    const out = { header: null };
    const actors = eng.actors || [];
    // possession owner change bookkeeping (pop check around changes)
    const owner = rec.captureTick();
    if (rec.lastOwner !== null && owner !== rec.lastOwner) { rec.ownerChanges++; rec.ownerWindow = 6; }
    rec.lastOwner = owner;
    let framePop = false;

    for (let n = 0; n < actors.length; n++) {
      const a = actors[n];
      if (!a || !a.root.visible) { rec.prev.delete(n); continue; }
      const bones = a.bones;
      a.root.updateMatrixWorld(true);
      const cur = {
        x: a.root.position.x, z: a.root.position.z, yaw: a.root.rotation.y,
        hipY: bones[B.hips].position.y,
        q: {}, ank: [], jump: a.prevJumpY || 0, pin: [!!(a.footPinL && a.footPinL.active), !!(a.footPinR && a.footPinR.active)],
      };
      for (const bn of BONES) cur.q[bn] = bones[B[bn]].quaternion.clone();
      for (const an of ["ankleL", "ankleR"]) {
        bones[B[an]].getWorldPosition(tmpV);
        cur.ank.push([tmpV.x, tmpV.y, tmpV.z]);
      }
      const pr = rec.prev.get(n);
      if (pr) {
        const vx = (cur.x - pr.x) / dt, vz = (cur.z - pr.z) / dt;
        const yawV = wrap(cur.yaw - pr.yaw) / dt;
        const bobV = (cur.hipY - pr.hipY) / dt;
        const jumpD = Math.hypot(cur.x - pr.x, cur.z - pr.z);
        cur.vx = vx; cur.vz = vz; cur.yawV = yawV; cur.bobV = bobV;
        cur.w = {};
        for (const bn of BONES) cur.w[bn] = angVel(cur.q[bn], pr.q[bn], dt, [0, 0, 0]);
        const tele = jumpD > 3.0;
        if (tele) { rec.cnt.teleports++; cur.tele = true; }
        else if (jumpD / dt > 13.5) { rec.cnt.slides++; (rec.slideLog = rec.slideLog || []).push([rec.frames, n, +jumpD.toFixed(3), a.isGk ? 1 : 0, +(Math.hypot(pr.vx || 0, pr.vz || 0)).toFixed(1)]); }
        if (pr.vx !== undefined && !tele && !pr.tele) {
          rec.cnt.samples++;
          const acc = Math.hypot(vx - pr.vx, vz - pr.vz) / dt;
          const yawAcc = Math.abs(yawV - pr.yawV) / dt;
          const bobAcc = Math.abs(bobV - pr.bobV) / dt;
          const ba = {};
          for (const bn of BONES) {
            const w1 = cur.w[bn], w0 = pr.w[bn];
            ba[bn] = Math.hypot(w1[0] - w0[0], w1[1] - w0[1], w1[2] - w0[2]) / dt;
          }
          const kneeA = Math.max(ba.kneeL, ba.kneeR);
          const S = rec.s;
          if (a.isGk) {
            S.gkPosAcc.push(acc); S.gkYawAcc.push(yawAcc); S.gkHipsAcc.push(ba.hips);
            S.gkSpineAcc.push(ba.spine); S.gkKneeAcc.push(kneeA);
          } else {
            S.posAcc.push(acc); S.yawAcc.push(yawAcc); S.yawVel.push(Math.abs(yawV)); S.hipBobAcc.push(bobAcc);
            S.hipsAcc.push(ba.hips); S.spineAcc.push(ba.spine); S.chestAcc.push(ba.chest); S.kneeAcc.push(kneeA);
          }
          if (acc > 60) { rec.cnt.posPops++; framePop = true; }
          if (yawAcc > 400) { rec.cnt.yawPops++; framePop = true; }
          if (bobAcc > 60) rec.cnt.bobPops++;
          const bmax = Math.max(ba.hips, ba.spine, ba.chest);
          if (bmax > 900) { if (a.isGk) rec.cnt.gkBonePops++; else rec.cnt.bonePops++; framePop = true; }
          // Foot skate: ankle at sole height (planted) on both frames -> horizontal world slip speed
          const floorY = 0.062 * Math.max(0.5, a.bodyScaleGroup.scale.y);
          for (let f = 0; f < 2; f++) {
            const c = cur.ank[f], o = pr.ank[f];
            if (c[1] < floorY + 0.012 && o[1] < floorY + 0.012 && cur.jump < 0.03) {
              rec.cnt.plantedSamples++;
              const slip = Math.hypot(c[0] - o[0], c[2] - o[2]) / dt;
              (a.isGk ? rec.s.gkSkate : rec.s.skate).push(slip);
              if (slip > 2.5) rec.cnt.skatePops++;
              if (slip > 4 && !a.isGk) {
                const ph = (((a.stridePhase || 0) + (f === 1 ? Math.PI : 0)) % (2 * Math.PI) + 2 * Math.PI) % (2 * Math.PI);
                const bk = Math.min(7, Math.floor(ph / (Math.PI / 4)));
                rec.hiSlipPhase = rec.hiSlipPhase || [0, 0, 0, 0, 0, 0, 0, 0];
                rec.hiSlipPhase[bk]++;
                rec.hiSlipSpeed = rec.hiSlipSpeed || [0, 0, 0, 0];
                const spd = Math.hypot(vx, vz);
                rec.hiSlipSpeed[spd < 0.5 ? 0 : spd < 2 ? 1 : spd < 5 ? 2 : 3]++;
              }
              if (!a.isGk && Math.hypot(vx, vz) > 1.0) rec.s.movingSkate.push(slip);
              const pin = f === 0 ? a.footPinL : a.footPinR;
              if (pin && pin.active && pr.pin && pr.pin[f]) rec.s.pinSkate.push(slip);
            }
          }
        }
      }
      if (!a.isGk && !a.isReferee && cur.jump > 0.2 && (!out.header || cur.jump > out.header.jump)) {
        out.header = { n, jump: cur.jump, rising: pr ? cur.jump > pr.jump : false, x: cur.x, z: cur.z };
      }
      rec.prev.set(n, cur);
    }
    if (rec.ownerWindow > 0) { rec.ownerWindow--; if (framePop) rec.ownerChangePopFrames++; }
    return out;
  };

  rec.project = (x, y, z) => {
    tmpV.set(x, y, z).project(eng.camera);
    const c = eng.canvas;
    return { sx: (tmpV.x * 0.5 + 0.5) * c.clientWidth, sy: (-tmpV.y * 0.5 + 0.5) * c.clientHeight, ok: tmpV.z < 1 };
  };

  rec.summary = () => {
    const pct = (arr, p) => {
      if (!arr.length) return null;
      const s = Float64Array.from(arr).sort();
      return s[Math.min(s.length - 1, Math.floor((s.length - 1) * p))];
    };
    const mean = (arr) => (arr.length ? arr.reduce((u, v) => u + v, 0) / arr.length : null);
    const stats = {};
    for (const [k, v] of Object.entries(rec.s)) {
      stats[k] = { n: v.length, mean: mean(v), p50: pct(v, 0.5), p95: pct(v, 0.95), p99: pct(v, 0.99), max: pct(v, 1) };
    }
    return { slideLog: (rec.slideLog || []).slice(0, 40), hiSlipPhase: rec.hiSlipPhase, hiSlipSpeed: rec.hiSlipSpeed, frames: rec.frames, stats, cnt: rec.cnt, ownerChanges: rec.ownerChanges, ownerChangePopFrames: rec.ownerChangePopFrames, tickCount: rec.ticks.length };
  };

  // Sim-stream analysis (independent of renderer).
  rec.simSummary = () => {
    const T = rec.ticks;
    let samples = 0, facingFlip = 0, facingJump = 0, velFlip = 0, velJerk = 0, idleFacingJitter = 0, idleSamples = 0;
    let facingStepAbs = [];
    let velStep = [];
    let posAcc = [];
    let velMismatch = [];
    for (let t = 2; t < T.length; t++) {
      const a = T[t - 2], b = T[t - 1], c = T[t];
      if (a.length !== c.length || a[0] !== c[0]) continue;
      const np = (c.length - 2) / 6;
      for (let k = 0; k < np; k++) {
        const o = 2 + k * 6;
        if (Math.hypot(c[o] - b[o], c[o + 1] - b[o + 1]) > 3000) continue;
        samples++;
        const bam = (u, v) => { let d = v - u; d = ((d + 32768) % 65536 + 65536) % 65536 - 32768; return d; };
        const df1 = bam(a[o + 4], b[o + 4]);
        const df2 = bam(b[o + 4], c[o + 4]);
        facingStepAbs.push(Math.abs(df2) * 360 / 65536);
        if (Math.abs(df2) > 65536 * (30 / 360)) facingJump++;
        if (Math.abs(df1) > 65536 * (3 / 360) && Math.abs(df2) > 65536 * (3 / 360) && Math.sign(df1) !== Math.sign(df2)) facingFlip++;
        const sp = Math.hypot(c[o + 2], c[o + 3]);
        if (sp < 300) {
          idleSamples++;
          if (Math.abs(df2) > 65536 * (4 / 360)) idleFacingJitter++;
        }
        // position second difference (m/s^2) and |reported v - finite-difference v| (m/s)
        posAcc.push(Math.hypot(c[o] - 2 * b[o] + a[o], c[o + 1] - 2 * b[o + 1] + a[o + 1]) * 3.6);
        velMismatch.push(Math.hypot((c[o] - b[o]) * 60 - c[o + 2], (c[o + 1] - b[o + 1]) * 60 - c[o + 3]) / 1000);
        const dvx1 = b[o + 2] - a[o + 2], dvy1 = b[o + 3] - a[o + 3];
        const dvx2 = c[o + 2] - b[o + 2], dvy2 = c[o + 3] - b[o + 3];
        const dv = Math.hypot(dvx2, dvy2);
        velStep.push(dv);
        if (dv > 600) velJerk++; // > 36 m/s^2 in one tick
        const m1 = Math.hypot(dvx1, dvy1);
        if (m1 > 150 && dv > 150 && (dvx1 * dvx2 + dvy1 * dvy2) / (m1 * dv) < -0.7) velFlip++;
      }
    }
    const pct = (arr, p) => {
      if (!arr.length) return null;
      const s = Float64Array.from(arr).sort();
      return s[Math.floor((s.length - 1) * p)];
    };
    return {
      ticks: T.length, samples,
      facingFlipPerK: (facingFlip / samples) * 1000,
      facingJumpGt30degPerK: (facingJump / samples) * 1000,
      idleFacingJitterPct: idleSamples ? (idleFacingJitter / idleSamples) * 100 : null,
      velFlipPerK: (velFlip / samples) * 1000,
      velJerkGt36mss2PerK: (velJerk / samples) * 1000,
      facingStepDeg: { p50: pct(facingStepAbs, 0.5), p95: pct(facingStepAbs, 0.95), p99: pct(facingStepAbs, 0.99) },
      posAccMs2: { p50: pct(posAcc, 0.5), p95: pct(posAcc, 0.95), p99: pct(posAcc, 0.99) },
      velVsFiniteDiffMs: { p50: pct(velMismatch, 0.5), p95: pct(velMismatch, 0.95), p99: pct(velMismatch, 0.99) },
      velStepMmS: { p50: pct(velStep, 0.5), p95: pct(velStep, 0.95), p99: pct(velStep, 0.99) },
    };
  };
  return true;
}

function cadenceSeq(c) {
  // returns a function frame -> dtMs
  if (c.endsWith("j")) {
    const base = 1000 / Number(c.slice(0, -1));
    let s = 12345;
    return () => {
      s = (s * 1103515245 + 12345) & 0x7fffffff;
      return base + ((s / 0x7fffffff) * 2 - 1) * 4.0;
    };
  }
  const base = 1000 / Number(c);
  return () => base;
}

async function runCadence(browser, cadence) {
  const ctx = await browser.newContext({ viewport: { width: 1280, height: 720 }, deviceScaleFactor: 1 });
  const page = await ctx.newPage();
  const errors = [];
  page.on("console", (m) => { if (m.type() === "error") errors.push(m.text()); });
  page.on("pageerror", (e) => errors.push(String(e)));
  await page.addInitScript(INIT_SCRIPT);
  await page.goto(URL, { waitUntil: "load" });

  // Pump frames (real time passes for fetch/wasm) until the attract match is live.
  let ready = false;
  for (let i = 0; i < 600 && !ready; i++) {
    await page.evaluate(() => window.__animStep(16.6667));
    ready = await page.evaluate(() => {
      const d = window.__psfcDiagnostics;
      return !!(d && d.getWasm && d.getWasm() && d.getSnapshot() && d.engine3d && d.engine3d.actors && d.engine3d.actors.length > 0);
    });
    if (!ready) await page.waitForTimeout(50);
  }
  if (!ready) throw new Error("attract match never became ready");
  await page.evaluate(installRecorder);

  const dtFn = cadenceSeq(cadence);
  const warmFrames = Math.round(WARMUP_S * 60);
  for (let i = 0; i < warmFrames; i++) await page.evaluate(() => window.__animStep(16.6667));

  await page.evaluate(() => { window.__animRec.recording = true; });
  const fps = cadence.endsWith("j") ? Number(cadence.slice(0, -1)) : Number(cadence);
  const total = Math.round(SECONDS * fps);
  const shots = [];
  let headerShots = 0;
  let prevHeader = false;
  let evtState = "idle"; // idle -> ascentShot -> peakShot (per header event)
  let lastHeaderFrame = -999;
  const OVERVIEW = new Set([300, 1200]);
  for (let f = 0; f < total; f++) {
    const dt = dtFn();
    const shooting = SHOTS && cadence === "60";
    const present = shooting && ((prevHeader && headerShots < 6) || OVERVIEW.has(f));
    const out = await page.evaluate(([dtMs, present]) => {
      const rec = window.__animRec;
      rec.presentOn = present;
      window.__animStep(dtMs);
      const o = rec.sample(dtMs);
      if (present) {
        // Grab pixels in the same task as the render (no preserveDrawingBuffer needed).
        const src = window.__psfcDiagnostics.engine3d.canvas;
        const c2 = document.createElement("canvas");
        if (o.header) {
          // Close-up: temporarily move the camera beside the jumper, re-render, grab, restore.
          const eng = window.__psfcDiagnostics.engine3d;
          const cam = eng.camera;
          const savedP = cam.position.clone();
          const savedQ = cam.quaternion.clone();
          const savedFov = cam.fov;
          cam.position.set(o.header.x + 3.2, 1.6, o.header.z + 3.2);
          cam.fov = 40;
          cam.updateProjectionMatrix();
          cam.lookAt(o.header.x, 1.2, o.header.z);
          cam.updateMatrixWorld(true);
          rec.renderRaw();
          const W = Math.min(src.width, src.height);
          c2.width = 320; c2.height = 320;
          c2.getContext("2d").drawImage(src, (src.width - W) / 2, 0, W, W, 0, 0, 320, 320);
          cam.position.copy(savedP);
          cam.quaternion.copy(savedQ);
          cam.fov = savedFov;
          cam.updateProjectionMatrix();
        } else {
          c2.width = 480; c2.height = 270;
          c2.getContext("2d").drawImage(src, 0, 0, src.width, src.height, 0, 0, 480, 270);
        }
        o.png = c2.toDataURL("image/png");
      }
      return o;
    }, [dt, present]);
    if (out.header && f - lastHeaderFrame > 40) evtState = "idle";
    if (out.header) lastHeaderFrame = f;
    prevHeader = !!out.header;
    if (out.png) {
      let name = null;
      if (out.header && headerShots < 6) {
        if (evtState === "idle" && out.header.rising) { name = "ascent"; evtState = "ascentShot"; }
        else if (evtState === "ascentShot" && !out.header.rising) { name = "contact_peak"; evtState = "peakShot"; }
        if (name) {
          name = `${LABEL}_header_${headerShots}_${name}_jz${out.header.jump.toFixed(2)}.png`;
          headerShots++;
        }
      } else if (OVERVIEW.has(f)) {
        name = `${LABEL}_overview_f${f}.png`;
      }
      if (name) {
        fs.writeFileSync(path.join(OUT, name), Buffer.from(out.png.split(",")[1], "base64"));
        shots.push(name);
      }
    }
  }
  const summary = await page.evaluate(() => window.__animRec.summary());
  const sim = await page.evaluate(() => window.__animRec.simSummary());
  await ctx.close();
  return { cadence, summary, sim, shots, consoleErrors: errors.slice(0, 20), consoleErrorCount: errors.length };
}

(async () => {
  const browser = await chromium.launch();
  const results = {};
  for (const c of CADENCES) {
    const t0 = Date.now();
    results[c] = await runCadence(browser, c);
    process.stderr.write(`[anim] ${LABEL} cadence ${c} done in ${((Date.now() - t0) / 1000).toFixed(1)}s\n`);
  }
  await browser.close();
  const file = path.join(OUT, `${LABEL}.json`);
  fs.writeFileSync(file, JSON.stringify(results, null, 1));
  // compact table to stdout
  const row = (c, r) => {
    const s = r.summary.stats, k = r.summary.cnt;
    const f = (v) => (v === null || v === undefined ? "-" : v.toFixed(1));
    return [
      c, f(s.posAcc.p95), f(s.posAcc.p99), f(s.yawAcc.p95), f(s.yawAcc.p99), f(s.hipsAcc.p95), f(s.spineAcc.p95), f(s.kneeAcc.p95),
      f(s.hipBobAcc.p95), f(s.skate.mean), f(s.skate.p95), f(s.pinSkate.mean), f(s.movingSkate.mean), f(s.gkPosAcc.p95), f(s.gkHipsAcc.p95), f(s.gkSpineAcc.p95),
      k.posPops, k.yawPops, k.bonePops, k.gkBonePops, k.skatePops, k.slides, k.teleports, r.summary.ownerChangePopFrames + "/" + r.summary.ownerChanges,
      r.consoleErrorCount,
    ].join("\t");
  };
  console.log(["cad", "pos95", "pos99", "yaw95", "yaw99", "hips95", "spine95", "knee95", "bob95", "skateMean", "skate95", "pinSkate", "movSkate", "gkPos95", "gkHips95", "gkSpine95",
    "posPops", "yawPops", "bonePops", "gkBonePops", "skatePops", "slides", "teleports", "ownPop", "consErr"].join("\t"));
  for (const c of CADENCES) console.log(row(c, results[c]));
  console.log("sim-stream:", JSON.stringify(results[CADENCES[0]].sim));
})().catch((e) => {
  console.error(e);
  process.exit(1);
});
