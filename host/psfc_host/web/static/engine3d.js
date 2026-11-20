// ClassicFutbol — Biomechanics, Analytical Two-Bone IK & Sculpted 3D WebGL Engine
// Implements:
// 1. Sculpted organic 3D human anatomy (lofted V-taper athletic torso, LatheGeometry
//    gastrocnemius calves & forearms, patella/elbow joint spheres, curved-vamp studded
//    3D cleats, vertex-sculpted cranium/jawline head with procedural facial map).
// 2. Closed-form Critically Damped Springs (Holden exact half-life integrator) for
//    CoM acceleration lean, centripetal banking, hip impact absorption & spine counter-twist.
// 3. Distance-driven foot-planting + Analytical Law-of-Cosines Two-Bone Leg IK
//    (eliminating foot-skating and keeping boots flush with turf during cuts/stances)
//    + proximal-to-distal instep kick whip chain.
// 4. Turf cleat-divot particle system + deterministic GPU resource disposal.

import * as THREE from "./vendor/three.module.js";

const KIT_CATALOGUE = {
  "man-utd": {
    home: {
      shirt: "#c8102e",
      sleeves: "#c8102e",
      trim: "#ffffff",
      shorts: "#ffffff",
      shortsStripe: "#c8102e",
      socks: "#111827",
      sockBand: "#c8102e",
      numberColor: "#ffffff",
      numberOutline: "#111827",
      pattern: "shoulder_stripes",
      boots: "#111827",
      bootAccent: "#f59e0b",
    },
    away: {
      shirt: "#f8fafc",
      sleeves: "#f8fafc",
      trim: "#111827",
      shorts: "#111827",
      shortsStripe: "#c8102e",
      socks: "#f8fafc",
      sockBand: "#111827",
      numberColor: "#111827",
      numberOutline: "#ffffff",
      pattern: "pinstripes",
      boots: "#dc2626",
      bootAccent: "#ffffff",
    },
    gk: {
      shirt: "#10b981",
      sleeves: "#059669",
      trim: "#ecfdf5",
      shorts: "#064e3b",
      shortsStripe: "#34d399",
      socks: "#10b981",
      sockBand: "#ffffff",
      numberColor: "#ffffff",
      numberOutline: "#064e3b",
      pattern: "solid",
      boots: "#f8fafc",
      bootAccent: "#10b981",
    },
    banner: "OLD TRAFFORD · STRETFORD END",
  },
  arsenal: {
    home: {
      shirt: "#db0007",
      sleeves: "#ffffff",
      trim: "#ffffff",
      shorts: "#ffffff",
      shortsStripe: "#db0007",
      socks: "#ffffff",
      sockBand: "#db0007",
      numberColor: "#ffffff",
      numberOutline: "#0f172a",
      pattern: "arsenal_sleeves",
      boots: "#0f172a",
      bootAccent: "#ffffff",
    },
    away: {
      shirt: "#eab308",
      sleeves: "#eab308",
      trim: "#1e293b",
      shorts: "#1e293b",
      shortsStripe: "#eab308",
      socks: "#eab308",
      sockBand: "#1e293b",
      numberColor: "#1e293b",
      numberOutline: "#fef9c3",
      pattern: "chevron_subtle",
      boots: "#1e293b",
      bootAccent: "#eab308",
    },
    gk: {
      shirt: "#38bdf8",
      sleeves: "#0284c7",
      trim: "#0f172a",
      shorts: "#0c4a6e",
      shortsStripe: "#38bdf8",
      socks: "#38bdf8",
      sockBand: "#0f172a",
      numberColor: "#0f172a",
      numberOutline: "#ffffff",
      pattern: "solid",
      boots: "#0f172a",
      bootAccent: "#38bdf8",
    },
    banner: "NORTH LONDON · CLOCK END",
  },
  liverpool: {
    home: {
      shirt: "#b91c1c",
      sleeves: "#b91c1c",
      trim: "#facc15",
      shorts: "#b91c1c",
      shortsStripe: "#facc15",
      socks: "#b91c1c",
      sockBand: "#facc15",
      numberColor: "#ffffff",
      numberOutline: "#450a0a",
      pattern: "pinstripes",
      boots: "#111827",
      bootAccent: "#ef4444",
    },
    away: {
      shirt: "#f1f5f9",
      sleeves: "#047857",
      trim: "#047857",
      shorts: "#0f172a",
      shortsStripe: "#10b981",
      socks: "#f1f5f9",
      sockBand: "#047857",
      numberColor: "#0f172a",
      numberOutline: "#ffffff",
      pattern: "quarters",
      boots: "#047857",
      bootAccent: "#ffffff",
    },
    gk: {
      shirt: "#a855f7",
      sleeves: "#7e22ce",
      trim: "#faf5ff",
      shorts: "#3b0764",
      shortsStripe: "#c084fc",
      socks: "#9333ea",
      sockBand: "#ffffff",
      numberColor: "#ffffff",
      numberOutline: "#3b0764",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#c084fc",
    },
    banner: "ANFIELD ROAD · THE KOP",
  },
  chelsea: {
    home: {
      shirt: "#1d4ed8",
      sleeves: "#1d4ed8",
      trim: "#ffffff",
      shorts: "#1d4ed8",
      shortsStripe: "#ffffff",
      socks: "#ffffff",
      sockBand: "#1d4ed8",
      numberColor: "#ffffff",
      numberOutline: "#0f172a",
      pattern: "shoulder_stripes",
      boots: "#f8fafc",
      bootAccent: "#2563eb",
    },
    away: {
      shirt: "#fefce8",
      sleeves: "#fefce8",
      trim: "#1d4ed8",
      shorts: "#1e3a8a",
      shortsStripe: "#fefce8",
      socks: "#1e3a8a",
      sockBand: "#fefce8",
      numberColor: "#1e3a8a",
      numberOutline: "#ffffff",
      pattern: "pinstripes",
      boots: "#1e3a8a",
      bootAccent: "#facc15",
    },
    gk: {
      shirt: "#f97316",
      sleeves: "#ea580c",
      trim: "#111827",
      shorts: "#111827",
      shortsStripe: "#f97316",
      socks: "#f97316",
      sockBand: "#111827",
      numberColor: "#ffffff",
      numberOutline: "#111827",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#f97316",
    },
    banner: "STAMFORD BRIDGE · SHED END",
  },
};

const SKIN_TONES = ["#f4cfb0", "#e8b98e", "#d19968", "#9e663d", "#6a3f23", "#4c2b17"];
const HAIR_COLORS = ["#141416", "#2b1d12", "#4a321f", "#7c5329", "#d4a959", "#1f242d"];

const PLAYER_LIKENESS = {
  "mun-gk-1": { skin: 5, hair: 0, style: "fade", beard: true },
  "mun-def-1": { skin: 1, hair: 1, style: "crop", beard: true },
  "mun-def-2": { skin: 1, hair: 1, style: "fade", beard: true },
  "mun-def-3": { skin: 0, hair: 2, style: "swept", beard: false },
  "mun-def-4": { skin: 1, hair: 2, style: "crop", beard: true },
  "mun-mid-1": { skin: 1, hair: 1, style: "crop", beard: true }, // Bruno
  "mun-mid-2": { skin: 4, hair: 0, style: "curls", beard: false }, // Mainoo
  "mun-mid-3": { skin: 2, hair: 0, style: "crop", beard: false }, // Casemiro
  "mun-fwd-1": { skin: 4, hair: 0, style: "fade", beard: true }, // Rashford
  "mun-fwd-2": { skin: 0, hair: 4, style: "swept", beard: false }, // Garnacho
  "mun-fwd-3": { skin: 0, hair: 4, style: "swept", beard: false }, // Hojlund

  "ars-gk-1": { skin: 1, hair: 1, style: "fade", beard: true }, // Raya
  "ars-def-1": { skin: 4, hair: 0, style: "buzz", beard: false }, // Saliba
  "ars-def-2": { skin: 3, hair: 0, style: "crop", beard: true }, // Gabriel
  "ars-def-3": { skin: 1, hair: 2, style: "swept", beard: true }, // White
  "ars-def-4": { skin: 3, hair: 0, style: "curls", beard: false }, // Timber
  "ars-mid-1": { skin: 0, hair: 4, style: "swept", beard: true }, // Odegaard
  "ars-mid-2": { skin: 0, hair: 2, style: "swept", beard: false }, // Rice
  "ars-mid-3": { skin: 4, hair: 0, style: "fade", beard: true }, // Partey
  "ars-fwd-1": { skin: 5, hair: 0, style: "curls", beard: false }, // Saka
  "ars-fwd-2": { skin: 0, hair: 2, style: "swept", beard: false }, // Havertz
  "ars-fwd-3": { skin: 1, hair: 1, style: "crop", beard: false }, // Martinelli

  "liv-gk-1": { skin: 1, hair: 2, style: "swept", beard: true }, // Alisson
  "liv-def-1": { skin: 3, hair: 0, style: "bun", beard: true }, // Van Dijk
  "liv-def-2": { skin: 4, hair: 0, style: "curls", beard: false }, // Trent
  "liv-def-3": { skin: 4, hair: 0, style: "fade", beard: true }, // Konate
  "liv-def-4": { skin: 0, hair: 2, style: "crop", beard: false }, // Robertson
  "liv-mid-1": { skin: 1, hair: 1, style: "swept", beard: true }, // Mac Allister
  "liv-mid-2": { skin: 4, hair: 0, style: "curls", beard: false }, // Gravenberch
  "liv-mid-3": { skin: 0, hair: 1, style: "swept", beard: true }, // Szoboszlai
  "liv-fwd-1": { skin: 2, hair: 0, style: "curls", beard: true }, // Salah
  "liv-fwd-2": { skin: 2, hair: 0, style: "fade", beard: false }, // Diaz
  "liv-fwd-3": { skin: 1, hair: 1, style: "bun", beard: true }, // Nunez

  "che-gk-1": { skin: 2, hair: 0, style: "fade", beard: true },
  "che-def-1": { skin: 3, hair: 0, style: "curls", beard: false },
  "che-def-2": { skin: 4, hair: 0, style: "curls", beard: true },
  "che-def-3": { skin: 1, hair: 1, style: "curls", beard: true },
  "che-def-4": { skin: 5, hair: 0, style: "fade", beard: false },
  "che-mid-1": { skin: 0, hair: 3, style: "crop", beard: false }, // Palmer
  "che-mid-2": { skin: 4, hair: 0, style: "fade", beard: false },
  "che-mid-3": { skin: 1, hair: 1, style: "fade", beard: true },
  "che-fwd-1": { skin: 5, hair: 0, style: "fade", beard: false },
  "che-fwd-2": { skin: 4, hair: 0, style: "curls", beard: true },
  "che-fwd-3": { skin: 1, hair: 2, style: "swept", beard: false },
};

// Track intentionally shared singleton geometries & textures so scene disposal never frees them
const RETAINED_GPU = new WeakSet();
function retainGpu(res) {
  if (res) RETAINED_GPU.add(res);
  return res;
}

function disposeObjectTree(root) {
  if (!root) return;
  root.traverse((obj) => {
    if (obj.geometry && !RETAINED_GPU.has(obj.geometry)) {
      obj.geometry.dispose();
    }
    const mats = Array.isArray(obj.material) ? obj.material : obj.material ? [obj.material] : [];
    for (const mat of mats) {
      if (RETAINED_GPU.has(mat)) continue;
      for (const key of ["map", "emissiveMap", "alphaMap", "roughnessMap", "bumpMap"]) {
        if (mat[key] && !RETAINED_GPU.has(mat[key])) {
          mat[key].dispose();
        }
      }
      mat.dispose();
    }
  });
}

function hashStr(s) {
  let h = 2166136261 >>> 0;
  const str = String(s || "");
  for (let i = 0; i < str.length; i++) {
    h ^= str.charCodeAt(i);
    h = Math.imul(h, 16777619) >>> 0;
  }
  return h >>> 0;
}

function isReddish(hex) {
  const c = new THREE.Color(hex || "#cc0000");
  return c.r > 0.55 && c.g < 0.28 && c.b < 0.32;
}

function isBluish(hex) {
  const c = new THREE.Color(hex || "#1d4ed8");
  return c.b > 0.5 && c.r < 0.3;
}

export function resolveMatchKits(homeClubId, awayClubId, homePrimaryFallback, awayPrimaryFallback) {
  const hEntry = KIT_CATALOGUE[homeClubId] || {
    home: {
      shirt: homePrimaryFallback || "#c8102e",
      sleeves: homePrimaryFallback || "#c8102e",
      trim: "#ffffff",
      shorts: "#ffffff",
      shortsStripe: homePrimaryFallback || "#c8102e",
      socks: "#111827",
      sockBand: "#ffffff",
      numberColor: "#ffffff",
      numberOutline: "#0f172a",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#ffffff",
    },
    gk: KIT_CATALOGUE["man-utd"].gk,
  };
  const aEntry = KIT_CATALOGUE[awayClubId] || {
    home: {
      shirt: awayPrimaryFallback || "#1d4ed8",
      sleeves: awayPrimaryFallback || "#1d4ed8",
      trim: "#ffffff",
      shorts: "#ffffff",
      shortsStripe: awayPrimaryFallback || "#1d4ed8",
      socks: "#1e293b",
      sockBand: "#ffffff",
      numberColor: "#ffffff",
      numberOutline: "#0f172a",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#38bdf8",
    },
    away: KIT_CATALOGUE.arsenal.away,
    gk: KIT_CATALOGUE.arsenal.gk,
  };

  const homeOut = hEntry.home;
  const clash =
    homeClubId === awayClubId ||
    (isReddish(homeOut.shirt) && isReddish(aEntry.home.shirt)) ||
    (isBluish(homeOut.shirt) && isBluish(aEntry.home.shirt));
  const awayOut = clash && aEntry.away ? aEntry.away : aEntry.home;

  return {
    homeOutfield: homeOut,
    awayOutfield: awayOut,
    homeGk: hEntry.gk,
    awayGk: aEntry.gk,
    homeBanner: hEntry.banner || "HOME SUPPORTERS END",
    awayBanner: aEntry.banner || "AWAY TRAVELLING KOP",
  };
}

function extractSurname(fullName) {
  if (!fullName) return "";
  const parts = String(fullName).trim().split(/\s+/);
  return parts[parts.length - 1].toUpperCase();
}

// ============================================================================
// 1. CLOSED-FORM CRITICALLY DAMPED SPRING & ANALYTICAL TWO-BONE IK MATH
// ============================================================================

/**
 * Closed-form Critically Damped Spring (Holden "Spring-It-On" exact half-life solver).
 * Preserves C1 velocity continuity across sudden target changes with zero overshoot.
 */
function dampSpring(state, keyVal, keyVel, goal, halfLife, dt) {
  const y = 1.38629436112 / Math.max(0.005, halfLife);
  const j0 = state[keyVal] - goal;
  const j1 = state[keyVel] + j0 * y;
  const yd = y * dt;
  const eydt = 1.0 / (1.0 + yd + 0.48 * yd * yd + 0.235 * yd * yd * yd);
  state[keyVal] = eydt * (j0 + j1 * dt) + goal;
  state[keyVel] = eydt * (state[keyVel] - j1 * yd);
  return state[keyVal];
}

// Reusable scratch math objects (zero per-frame GC allocation in IK loop!)
const _vDown = new THREE.Vector3(0, -1, 0);
const _vTargetOff = new THREE.Vector3();
const _vToTarget = new THREE.Vector3();
const _vPole = new THREE.Vector3();
const _vKnee = new THREE.Vector3();
const _vThighDir = new THREE.Vector3();
const _vShinDir = new THREE.Vector3();
const _qPelvisInv = new THREE.Quaternion();
const _qHipInv = new THREE.Quaternion();
const _qKneeInv = new THREE.Quaternion();
const _qChainInv = new THREE.Quaternion();
const _qFootPitch = new THREE.Quaternion();
const _eulerTmp = new THREE.Euler();

/**
 * Analytical Law-of-Cosines Two-Bone Leg IK Solver in `bodyScaleGroup` space.
 * Rotates `leg.hip`, `leg.knee`, and `leg.ankle` so the ankle lands on `targetPos`
 * with the knee bending anatomically along `poleDir` (+Z forward) and foot pitched by `footPitchRad`.
 */
function solveLegTwoBoneIK(pelvis, leg, hipSocketLocal, targetPos, poleDir, footPitchRad, upperLen, lowerLen) {
  // Compute world-like hip socket position in bodyScaleGroup space
  const hipPos = _vKnee.copy(hipSocketLocal).applyQuaternion(pelvis.quaternion).add(pelvis.position).clone();

  _vTargetOff.subVectors(targetPos, hipPos);
  const rawDist = _vTargetOff.length();
  const maxReach = upperLen + lowerLen - 0.004;
  const clampedDist = Math.max(0.18, Math.min(rawDist, maxReach));

  if (rawDist > 1e-5) {
    _vToTarget.copy(_vTargetOff).divideScalar(rawDist);
  } else {
    _vToTarget.set(0, -1, 0);
  }

  // Orthogonalize knee pole vector against target axis so knee ALWAYS bends forward (+Z)
  _vPole.copy(poleDir).addScaledVector(_vToTarget, -poleDir.dot(_vToTarget));
  if (_vPole.lengthSq() < 1e-6) {
    _vPole.set(0, 0, 1).addScaledVector(_vToTarget, -_vToTarget.z);
  }
  _vPole.normalize();

  // Law of Cosines for interior angle alpha between thigh and target axis
  const cosAlpha = Math.max(
    -1,
    Math.min(1, (clampedDist * clampedDist + upperLen * upperLen - lowerLen * lowerLen) / (2 * upperLen * clampedDist))
  );
  const sinAlpha = Math.sqrt(Math.max(0, 1 - cosAlpha * cosAlpha));

  // Exact 3D knee joint position in bodyScaleGroup space
  const kneePos = new THREE.Vector3()
    .copy(hipPos)
    .addScaledVector(_vToTarget, upperLen * cosAlpha)
    .addScaledVector(_vPole, upperLen * sinAlpha);

  const effTarget = new THREE.Vector3().copy(hipPos).addScaledVector(_vToTarget, clampedDist);

  // Convert thigh vector into pelvis-local space -> hip.quaternion
  _qPelvisInv.copy(pelvis.quaternion).invert();
  _vThighDir.subVectors(kneePos, hipPos).normalize().applyQuaternion(_qPelvisInv);
  leg.hip.quaternion.setFromUnitVectors(_vDown, _vThighDir);

  // Convert shin vector into hip-local space -> knee.quaternion
  _qHipInv.copy(leg.hip.quaternion).invert();
  _vShinDir.subVectors(effTarget, kneePos).normalize().applyQuaternion(_qPelvisInv).applyQuaternion(_qHipInv);
  leg.knee.quaternion.setFromUnitVectors(_vDown, _vShinDir);

  // Orient ankle so boot sole aligns cleanly with ground + desired dorsiflexion/plantarflexion pitch!
  _qKneeInv.copy(leg.knee.quaternion).invert();
  _qChainInv.copy(_qKneeInv).multiply(_qHipInv).multiply(_qPelvisInv);
  _eulerTmp.set(footPitchRad, 0, 0);
  _qFootPitch.setFromEuler(_eulerTmp);
  leg.ankle.quaternion.copy(_qChainInv).multiply(_qFootPitch);
}

// ============================================================================
// 2. SCULPTED ORGANIC 3D HUMAN ANATOMY GEOMETRY BUILDERS
// ============================================================================

/**
 * Multi-ring lofted athletic V-taper torso with rounded pectorals, obliques,
 * latissimus flare, sloped trapezius shoulders, and front/back planar UV unwrap.
 */
function createSculptedTorsoGeometry() {
  // [y, halfWidthX, halfDepthZ, zCenterShift] from lower waist (0.0) to collar base (0.53)
  const rings = [
    [0.0, 0.162, 0.102, 0.0],
    [0.07, 0.158, 0.100, 0.002],
    [0.15, 0.168, 0.108, 0.004],
    [0.24, 0.186, 0.118, 0.008],
    [0.33, 0.205, 0.128, 0.012], // Pectoral / latissimus peak
    [0.41, 0.218, 0.124, 0.006], // Broad athletic chest/shoulder line
    [0.47, 0.204, 0.110, 0.0],
    [0.51, 0.145, 0.088, -0.003], // Trapezius slope
    [0.535, 0.086, 0.074, -0.004], // Neck collar ring
  ];

  const radialSegs = 28;
  const positions = [];
  const uvs = [];
  const frontIndices = [];
  const backIndices = [];

  for (let r = 0; r < rings.length; r++) {
    const [y, rx, rz, zShift] = rings[r];
    const vCoord = Math.min(1, Math.max(0, y / 0.535));
    for (let s = 0; s <= radialSegs; s++) {
      const theta = (s / radialSegs) * Math.PI * 2;
      const cosT = Math.cos(theta);
      const sinT = Math.sin(theta);

      // Subtle superellipse rounding + spinal groove & pectoral contour
      let x = cosT * rx;
      let z = sinT * rz + zShift;
      if (sinT > 0.3 && y > 0.26 && y < 0.44) {
        // Subtle pectoral muscle fullness
        z += 0.008 * Math.sin(((y - 0.26) / 0.18) * Math.PI) * Math.abs(cosT < 0.8 ? 1 : 0.4);
      }
      positions.push(x, y, z);

      // Planar front/back UV mapping so crest, sponsor, surname & big squad number stay razor-crisp!
      const uNorm = 0.5 + (x / 0.225) * 0.46;
      const uBack = 0.5 - (x / 0.225) * 0.46;
      uvs.push(sinT >= 0 ? uNorm : uBack, vCoord);
    }
  }

  const stride = radialSegs + 1;
  for (let r = 0; r < rings.length - 1; r++) {
    for (let s = 0; s < radialSegs; s++) {
      const a = r * stride + s;
      const b = a + 1;
      const c = (r + 1) * stride + s;
      const d = c + 1;
      const midTheta = ((s + 0.5) / radialSegs) * Math.PI * 2;
      const targetArr = Math.sin(midTheta) >= 0 ? frontIndices : backIndices;
      targetArr.push(a, d, b, a, c, d);
    }
  }

  const geo = new THREE.BufferGeometry();
  const allIndices = [...frontIndices, ...backIndices];
  geo.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
  geo.setAttribute("uv", new THREE.Float32BufferAttribute(uvs, 2));
  geo.setIndex(allIndices);
  geo.clearGroups();
  geo.addGroup(0, frontIndices.length, 0); // Material 0: Chest Front
  geo.addGroup(frontIndices.length, backIndices.length, 1); // Material 1: Jersey Back (Surname + #)
  geo.computeVertexNormals();
  return geo;
}

/**
 * Sculpted anatomical pelvis/hips shorts block (smooth glute/hip lathe scaled elliptically).
 */
function createSculptedPelvisGeometry() {
  const pts = [
    new THREE.Vector2(0.001, -0.11),
    new THREE.Vector2(0.145, -0.105),
    new THREE.Vector2(0.178, -0.05),
    new THREE.Vector2(0.184, 0.02),
    new THREE.Vector2(0.166, 0.095),
    new THREE.Vector2(0.001, 0.10),
  ];
  const g = new THREE.LatheGeometry(pts, 20);
  g.scale(1.0, 1.0, 0.66);
  g.computeVertexNormals();
  return g;
}

/**
 * Sculpted gastrocnemius calf + Achilles tendon LatheGeometry (origin at knee = 0, ankle at y = -0.40).
 * Points ordered bottom (-y) to top (+y) so LatheGeometry normals face outward!
 */
function createMuscularCalfGeometry() {
  const pts = [
    new THREE.Vector2(0.001, -0.405),
    new THREE.Vector2(0.054, -0.40),
    new THREE.Vector2(0.052, -0.35), // Tapered Achilles / ankle
    new THREE.Vector2(0.065, -0.26),
    new THREE.Vector2(0.081, -0.17),
    new THREE.Vector2(0.085, -0.105), // Gastrocnemius muscle belly bulge!
    new THREE.Vector2(0.073, -0.02),
    new THREE.Vector2(0.001, 0.005),
  ];
  const g = new THREE.LatheGeometry(pts, 18);
  g.scale(0.95, 1.0, 1.06);
  g.computeVertexNormals();
  return g;
}

/**
 * Closed solid flared athletic shorts leg (ordered bottom-to-top so all outer & hem normals face outward).
 */
function createFlaredShortLegGeometry() {
  const pts = [
    new THREE.Vector2(0.001, -0.215),
    new THREE.Vector2(0.086, -0.218), // Inner hem tuck against quad
    new THREE.Vector2(0.112, -0.222), // Natural fabric flare hem
    new THREE.Vector2(0.105, -0.09),
    new THREE.Vector2(0.098, 0.025),
    new THREE.Vector2(0.001, 0.03),
  ];
  const g = new THREE.LatheGeometry(pts, 18);
  g.scale(0.98, 1.0, 1.04);
  g.computeVertexNormals();
  return g;
}

/**
 * Sculpted quadriceps thigh emerging below shorts hem down to knee (ordered bottom-to-top).
 */
function createMuscularQuadGeometry() {
  const pts = [
    new THREE.Vector2(0.001, -0.408),
    new THREE.Vector2(0.074, -0.39),
    new THREE.Vector2(0.089, -0.29), // Vastus lateralis / rectus femoris sweep
    new THREE.Vector2(0.092, -0.18),
    new THREE.Vector2(0.001, -0.17),
  ];
  const g = new THREE.LatheGeometry(pts, 16);
  g.scale(0.97, 1.0, 1.04);
  g.computeVertexNormals();
  return g;
}

/**
 * Sculpted brachioradialis forearm tapering naturally from wrist (-0.23) up to elbow (0).
 */
function createForearmGeometry() {
  const pts = [
    new THREE.Vector2(0.001, -0.235),
    new THREE.Vector2(0.042, -0.215),
    new THREE.Vector2(0.054, -0.10),
    new THREE.Vector2(0.057, -0.025),
    new THREE.Vector2(0.001, 0.012),
  ];
  const g = new THREE.LatheGeometry(pts, 14);
  g.computeVertexNormals();
  return g;
}

/**
 * Curved-vamp 3D Football Cleat (origin at ankle = (0,0,0), sole resting at y = -0.065).
 */
function createSculptedCleatGeometry() {
  const shape = new THREE.Shape();
  shape.moveTo(-0.046, -0.085);
  shape.quadraticCurveTo(-0.056, 0.02, -0.048, 0.115);
  shape.quadraticCurveTo(-0.036, 0.185, 0.0, 0.195); // Rounded anatomical toe box!
  shape.quadraticCurveTo(0.036, 0.185, 0.048, 0.115);
  shape.quadraticCurveTo(0.056, 0.02, 0.046, -0.085);
  shape.quadraticCurveTo(0.0, -0.098, -0.046, -0.085);

  const g = new THREE.ExtrudeGeometry(shape, {
    depth: 0.058,
    bevelEnabled: true,
    bevelSegments: 3,
    steps: 1,
    bevelSize: 0.01,
    bevelThickness: 0.012,
  });
  g.rotateX(Math.PI * 0.5);
  // Slope vamp downward toward toe for sleek athletic cleat silhouette
  const pos = g.attributes.position;
  for (let i = 0; i < pos.count; i++) {
    const y = pos.getY(i);
    const z = pos.getZ(i);
    if (y > -0.03 && z > 0.03) {
      pos.setY(i, y - (z - 0.03) * 0.22);
    }
  }
  g.translate(0, -0.002, 0.01);
  g.computeVertexNormals();
  return g;
}

/**
 * Vertex-sculpted Human Cranium, Cheekbones, Brow Ridge & Tapered Jawline/Chin.
 * Uses phiStart = -PI/2 so u = 0.5 maps directly to +Z (front face) and UV seam hides at -Z!
 */
function createSculptedHeadGeometry() {
  const g = new THREE.SphereGeometry(0.138, 28, 22, -Math.PI * 0.5, Math.PI * 2);
  const pos = g.attributes.position;
  for (let i = 0; i < pos.count; i++) {
    let x = pos.getX(i);
    let y = pos.getY(i);
    let z = pos.getZ(i);

    // Elongate cranium height slightly
    y *= 1.12;
    x *= 0.91;

    // Taper lower cheeks & jawline toward a defined human chin!
    if (y < 0) {
      const t = Math.min(1, -y / 0.155);
      x *= 1.0 - t * 0.28;
      if (z > 0) {
        z *= 1.0 - t * 0.12;
        // Subtle chin prominence at bottom front
        if (t > 0.65 && Math.abs(x) < 0.045) {
          z += 0.012 * (t - 0.65);
        }
      } else {
        // Tuck base of skull above occipital neck insertion
        z *= 1.0 - t * 0.25;
      }
    }

    // Subtle brow ridge & eye-socket contour on front face (+Z)
    if (z > 0.06 && y > 0.005 && y < 0.055) {
      z += 0.009 * Math.cos(((y - 0.03) / 0.025) * Math.PI * 0.5);
    }

    pos.setXYZ(i, x, y, z);
  }
  g.computeVertexNormals();
  return g;
}

function createFaceSkinTexture(skinHex, hairHex, hasBeard) {
  const W = 256;
  const H = 256;
  const c = document.createElement("canvas");
  c.width = W;
  c.height = H;
  const ctx = c.getContext("2d");

  ctx.fillStyle = skinHex;
  ctx.fillRect(0, 0, W, H);

  // Warm subsurface cheek/forehead vitality gradient
  const cheek = ctx.createRadialGradient(128, 132, 8, 128, 132, 68);
  cheek.addColorStop(0, "rgba(220, 95, 75, 0.11)");
  cheek.addColorStop(1, "rgba(220, 95, 75, 0)");
  ctx.fillStyle = cheek;
  ctx.fillRect(60, 70, 136, 120);

  // Optional athletic stubble / short beard along jawline & upper lip
  if (hasBeard) {
    ctx.fillStyle = hairHex;
    ctx.globalAlpha = 0.36;
    ctx.beginPath();
    ctx.ellipse(128, 174, 34, 28, 0, 0.1, Math.PI - 0.1);
    ctx.fill();
    ctx.globalAlpha = 0.45;
    ctx.fillRect(114, 150, 28, 5); // Mustache line
    ctx.globalAlpha = 1.0;
  }

  // Eyebrows (centered around u=0.5 -> x=128, v=0.55 -> y=114)
  ctx.fillStyle = hairHex;
  ctx.beginPath();
  ctx.roundRect(104, 108, 16, 4.2, 2);
  ctx.roundRect(136, 108, 16, 4.2, 2);
  ctx.fill();

  // Crisp Sclera + Iris + Pupil + Specular Catchlight
  for (const ex of [112, 144]) {
    ctx.fillStyle = "#f8fafc";
    ctx.beginPath();
    ctx.ellipse(ex, 120, 6.2, 4.0, 0, 0, Math.PI * 2);
    ctx.fill();

    ctx.fillStyle = "#291d12";
    ctx.beginPath();
    ctx.arc(ex, 120, 3.4, 0, Math.PI * 2);
    ctx.fill();

    ctx.fillStyle = "#090d16";
    ctx.beginPath();
    ctx.arc(ex, 120, 1.8, 0, Math.PI * 2);
    ctx.fill();

    ctx.fillStyle = "#ffffff";
    ctx.beginPath();
    ctx.arc(ex - 1.1, 118.8, 0.95, 0, Math.PI * 2);
    ctx.fill();
  }

  // Subtle nose bridge shadow & mouth line
  ctx.fillStyle = "rgba(0, 0, 0, 0.16)";
  ctx.beginPath();
  ctx.ellipse(128, 141, 5.2, 2.4, 0, 0, Math.PI * 2);
  ctx.fill();

  ctx.strokeStyle = "rgba(65, 25, 20, 0.48)";
  ctx.lineWidth = 2.5;
  ctx.beginPath();
  ctx.moveTo(118, 158);
  ctx.quadraticCurveTo(128, 161, 138, 158);
  ctx.stroke();

  const tex = new THREE.CanvasTexture(c);
  tex.colorSpace = THREE.SRGBColorSpace;
  return tex;
}

// --- Singleton Shared Geometries ---

const SHARED_GEO = (() => {
  const torso = retainGpu(createSculptedTorsoGeometry());
  const pelvis = retainGpu(createSculptedPelvisGeometry());
  const shortLeg = retainGpu(createFlaredShortLegGeometry());
  const quad = retainGpu(createMuscularQuadGeometry());
  const patella = retainGpu(new THREE.SphereGeometry(0.075, 12, 10));
  const calfSock = retainGpu(createMuscularCalfGeometry());
  const sockBand = retainGpu(new THREE.CylinderGeometry(0.081, 0.080, 0.048, 14));
  const cleat = retainGpu(createSculptedCleatGeometry());
  const bootStripe = retainGpu(new THREE.BoxGeometry(0.104, 0.026, 0.095));
  const stud = retainGpu(new THREE.CylinderGeometry(0.008, 0.006, 0.016, 6));

  const deltoid = retainGpu(new THREE.SphereGeometry(0.076, 14, 12));
  deltoid.scale(0.92, 1.06, 0.94);
  const sleeve = retainGpu(new THREE.CylinderGeometry(0.071, 0.060, 0.215, 14));
  const sleeveCuff = retainGpu(new THREE.CylinderGeometry(0.063, 0.062, 0.028, 14));
  const elbowSphere = retainGpu(new THREE.SphereGeometry(0.055, 10, 8));
  const forearm = retainGpu(createForearmGeometry());
  const hand = retainGpu(new THREE.SphereGeometry(0.046, 10, 8));
  hand.scale(0.85, 1.18, 0.68);
  const gkGlove = retainGpu(new THREE.SphereGeometry(0.068, 12, 10));
  gkGlove.scale(0.95, 1.28, 0.62);

  const neck = retainGpu(new THREE.CylinderGeometry(0.064, 0.076, 0.105, 12));
  const collar = retainGpu(new THREE.TorusGeometry(0.086, 0.018, 8, 18));
  collar.rotateX(Math.PI * 0.5);
  const head = retainGpu(createSculptedHeadGeometry());
  const nose = retainGpu(new THREE.SphereGeometry(0.02, 8, 8));
  nose.scale(0.78, 1.05, 1.15);
  const ear = retainGpu(new THREE.SphereGeometry(0.026, 8, 6));
  ear.scale(0.45, 1.05, 0.68);

  const ring = retainGpu(new THREE.RingGeometry(0.58, 0.74, 36));
  ring.rotateX(-Math.PI * 0.5);
  const chargeArc = retainGpu(new THREE.RingGeometry(0.76, 0.9, 32));
  chargeArc.rotateX(-Math.PI * 0.5);
  const aimArrow = retainGpu(new THREE.ConeGeometry(0.16, 0.32, 4));
  aimArrow.rotateX(Math.PI * 0.5);
  const cursorCone = retainGpu(new THREE.ConeGeometry(0.15, 0.26, 4));
  cursorCone.rotateX(Math.PI);
  const shadowDisc = retainGpu(new THREE.CircleGeometry(0.52, 20));
  shadowDisc.rotateX(-Math.PI * 0.5);

  return {
    torso,
    pelvis,
    shortLeg,
    quad,
    patella,
    calfSock,
    sockBand,
    cleat,
    bootStripe,
    stud,
    deltoid,
    sleeve,
    sleeveCuff,
    elbowSphere,
    forearm,
    hand,
    gkGlove,
    neck,
    collar,
    head,
    nose,
    ear,
    ring,
    chargeArc,
    aimArrow,
    cursorCone,
    shadowDisc,
  };
})();

// --- Stadium & Kit Texture Builders ---

function createSkyTexture() {
  const c = document.createElement("canvas");
  c.width = 64;
  c.height = 512;
  const g = c.getContext("2d");
  const grad = g.createLinearGradient(0, 0, 0, 512);
  grad.addColorStop(0.0, "#030712");
  grad.addColorStop(0.32, "#0a162c");
  grad.addColorStop(0.54, "#152c4e");
  grad.addColorStop(0.72, "#1e3a5f");
  grad.addColorStop(1.0, "#0b1914");
  g.fillStyle = grad;
  g.fillRect(0, 0, 64, 512);
  const tex = new THREE.CanvasTexture(c);
  tex.colorSpace = THREE.SRGBColorSpace;
  return retainGpu(tex);
}

function createPitchTexture(pitchLen, pitchWid, fmt, maxAnisotropy) {
  const marginX = 5.5;
  const marginZ = 5.0;
  const totalW = pitchLen + marginX * 2;
  const totalH = pitchWid + marginZ * 2;

  const W = 2048;
  const H = 1280;
  const c = document.createElement("canvas");
  c.width = W;
  c.height = H;
  const ctx = c.getContext("2d");

  const pxX = (wx) => ((wx + totalW * 0.5) / totalW) * W;
  const pxZ = (wz) => ((wz + totalH * 0.5) / totalH) * H;
  const scaleX = W / totalW;
  const scaleZ = H / totalH;

  ctx.fillStyle = "#14532d";
  ctx.fillRect(0, 0, W, H);

  const halfL = pitchLen * 0.5;
  const halfW = pitchWid * 0.5;
  const pLeft = pxX(-halfL);
  const pTop = pxZ(-halfW);
  const pW = pitchLen * scaleX;
  const pH = pitchWid * scaleZ;

  const stripeCount = pitchLen > 80 ? 18 : 12;
  const stripeM = pitchLen / stripeCount;
  for (let i = 0; i < stripeCount; i++) {
    const x0 = pxX(-halfL + i * stripeM);
    const x1 = pxX(-halfL + (i + 1) * stripeM);
    ctx.fillStyle = i % 2 === 0 ? "#1d7c3a" : "#248f44";
    ctx.fillRect(x0, pTop, x1 - x0 + 0.8, pH);
  }

  const crossRows = pitchWid > 50 ? 10 : 6;
  const rowH = pH / crossRows;
  for (let r = 0; r < crossRows; r++) {
    if (r % 2 === 0) {
      ctx.fillStyle = "rgba(255, 255, 255, 0.025)";
      ctx.fillRect(pLeft, pTop + r * rowH, pW, rowH);
    }
  }

  let seed = 0x51f499;
  const rand = () => {
    seed = (Math.imul(seed, 1664525) + 1013904223) >>> 0;
    return seed / 4294967296;
  };
  for (let i = 0; i < 28000; i++) {
    const gx = rand() * W;
    const gy = rand() * H;
    ctx.fillStyle = rand() > 0.5 ? "rgba(255,255,255,0.032)" : "rgba(6,32,14,0.055)";
    ctx.fillRect(gx, gy, 2, 2);
  }

  for (const sign of [-1, 1]) {
    const gx = pxX(sign * (halfL - 1.2));
    const gy = pxZ(0);
    const wear = ctx.createRadialGradient(gx, gy, 4, gx, gy, scaleX * 3.4);
    wear.addColorStop(0, "rgba(120, 92, 48, 0.16)");
    wear.addColorStop(1, "rgba(120, 92, 48, 0)");
    ctx.fillStyle = wear;
    ctx.beginPath();
    ctx.arc(gx, gy, scaleX * 3.4, 0, Math.PI * 2);
    ctx.fill();
  }

  const linePx = Math.max(4.2, scaleX * 0.13);
  ctx.strokeStyle = "rgba(248, 250, 252, 0.94)";
  ctx.fillStyle = "rgba(248, 250, 252, 0.96)";
  ctx.lineWidth = linePx;
  ctx.lineJoin = "miter";

  ctx.strokeRect(pLeft, pTop, pW, pH);
  ctx.beginPath();
  ctx.moveTo(pxX(0), pTop);
  ctx.lineTo(pxX(0), pTop + pH);
  ctx.stroke();

  const circleR = (fmt.center_circle_radius_mm || 6000) / 1000;
  ctx.beginPath();
  ctx.ellipse(pxX(0), pxZ(0), circleR * scaleX, circleR * scaleZ, 0, 0, Math.PI * 2);
  ctx.stroke();
  ctx.beginPath();
  ctx.ellipse(pxX(0), pxZ(0), linePx * 1.45, linePx * 1.45, 0, 0, Math.PI * 2);
  ctx.fill();

  const boxD = (fmt.penalty_box_depth_mm || 9000) / 1000;
  const boxHW = (fmt.penalty_box_width_mm || 18000) / 2000;
  const goalHW = (fmt.goal_width_mm || 4000) / 2000;
  const sixD = Math.min(boxD * 0.42, 5.5);
  const sixHW = goalHW + (pitchLen > 80 ? 5.5 : 2.2);

  for (const sign of [-1, 1]) {
    const goalLineX = sign * halfL;
    const boxEdgeX = goalLineX - sign * boxD;
    ctx.strokeRect(
      Math.min(pxX(goalLineX), pxX(boxEdgeX)),
      pxZ(-boxHW),
      Math.abs(pxX(goalLineX) - pxX(boxEdgeX)),
      boxHW * 2 * scaleZ
    );

    const sixEdgeX = goalLineX - sign * sixD;
    ctx.strokeRect(
      Math.min(pxX(goalLineX), pxX(sixEdgeX)),
      pxZ(-sixHW),
      Math.abs(pxX(goalLineX) - pxX(sixEdgeX)),
      sixHW * 2 * scaleZ
    );

    const penDist = boxD * 0.68;
    const penX = goalLineX - sign * penDist;
    ctx.beginPath();
    ctx.ellipse(pxX(penX), pxZ(0), linePx * 1.25, linePx * 1.25, 0, 0, Math.PI * 2);
    ctx.fill();

    const arcR = boxD * 0.52;
    const dxBox = boxD - penDist;
    if (arcR > dxBox) {
      const maxAng = Math.acos(dxBox / arcR);
      ctx.beginPath();
      for (let s = 0; s <= 32; s++) {
        const t = -maxAng + (s / 32) * (maxAng * 2);
        const wx = penX - sign * Math.cos(t) * arcR;
        const wz = Math.sin(t) * arcR;
        if (s === 0) ctx.moveTo(pxX(wx), pxZ(wz));
        else ctx.lineTo(pxX(wx), pxZ(wz));
      }
      ctx.stroke();
    }
  }

  const cornerR = 1.0;
  for (const [cx, cz, a0, a1] of [
    [-halfL, -halfW, 0, Math.PI * 0.5],
    [halfL, -halfW, Math.PI * 0.5, Math.PI],
    [halfL, halfW, Math.PI, Math.PI * 1.5],
    [-halfL, halfW, Math.PI * 1.5, Math.PI * 2],
  ]) {
    ctx.beginPath();
    for (let s = 0; s <= 12; s++) {
      const a = a0 + (s / 12) * (a1 - a0);
      const wx = cx + Math.cos(a) * cornerR;
      const wz = cz + Math.sin(a) * cornerR;
      if (s === 0) ctx.moveTo(pxX(wx), pxZ(wz));
      else ctx.lineTo(pxX(wx), pxZ(wz));
    }
    ctx.stroke();
  }

  ctx.strokeStyle = "rgba(248, 250, 252, 0.45)";
  ctx.lineWidth = linePx * 0.55;
  ctx.setLineDash([8, 6]);
  for (const benchCenterX of [-pitchLen * 0.2, pitchLen * 0.2]) {
    const bw = pitchLen * 0.14;
    ctx.strokeRect(pxX(benchCenterX - bw * 0.5), pxZ(-halfW - 2.2), bw * scaleX, 1.9 * scaleZ);
  }
  ctx.setLineDash([]);

  const tex = new THREE.CanvasTexture(c);
  tex.colorSpace = THREE.SRGBColorSpace;
  tex.anisotropy = maxAnisotropy || 4;
  return { tex, totalW, totalH };
}

function createGoalNetTexture() {
  const c = document.createElement("canvas");
  c.width = 128;
  c.height = 128;
  const ctx = c.getContext("2d");
  ctx.clearRect(0, 0, 128, 128);
  ctx.strokeStyle = "rgba(241, 245, 249, 0.75)";
  ctx.lineWidth = 2.1;
  for (let i = -128; i <= 256; i += 16) {
    ctx.beginPath();
    ctx.moveTo(i, 0);
    ctx.lineTo(i + 128, 128);
    ctx.stroke();
    ctx.beginPath();
    ctx.moveTo(i + 128, 0);
    ctx.lineTo(i, 128);
    ctx.stroke();
  }
  const tex = new THREE.CanvasTexture(c);
  tex.wrapS = THREE.RepeatWrapping;
  tex.wrapT = THREE.RepeatWrapping;
  tex.repeat.set(8, 4);
  return tex;
}

function createTelstarBallTexture() {
  const W = 512;
  const H = 256;
  const c = document.createElement("canvas");
  c.width = W;
  c.height = H;
  const ctx = c.getContext("2d");

  ctx.fillStyle = "#f8fafc";
  ctx.fillRect(0, 0, W, H);

  const spots = [
    [0.12, 0.26],
    [0.37, 0.22],
    [0.62, 0.26],
    [0.87, 0.22],
    [0.24, 0.52],
    [0.5, 0.5],
    [0.75, 0.52],
    [0.99, 0.5],
    [0.12, 0.78],
    [0.37, 0.76],
    [0.62, 0.78],
    [0.87, 0.76],
  ];

  ctx.strokeStyle = "rgba(100, 116, 139, 0.45)";
  ctx.lineWidth = 3;
  for (let i = 0; i < spots.length; i++) {
    for (let j = i + 1; j < spots.length; j++) {
      const dx = (spots[i][0] - spots[j][0]) * W;
      const dy = (spots[i][1] - spots[j][1]) * H;
      if (Math.hypot(dx, dy) < 145) {
        ctx.beginPath();
        ctx.moveTo(spots[i][0] * W, spots[i][1] * H);
        ctx.lineTo(spots[j][0] * W, spots[j][1] * H);
        ctx.stroke();
      }
    }
  }

  for (const [u, v] of spots) {
    const cx = u * W;
    const cy = v * H;
    const r = 24;
    ctx.fillStyle = "#0f172a";
    ctx.beginPath();
    for (let k = 0; k < 5; k++) {
      const ang = -Math.PI * 0.5 + (k * Math.PI * 2) / 5;
      const px = cx + Math.cos(ang) * r * 1.15;
      const py = cy + Math.sin(ang) * r;
      if (k === 0) ctx.moveTo(px, py);
      else ctx.lineTo(px, py);
    }
    ctx.closePath();
    ctx.fill();
  }

  const tex = new THREE.CanvasTexture(c);
  tex.colorSpace = THREE.SRGBColorSpace;
  return retainGpu(tex);
}

function createAdboardTexture(homeName, awayName, homeColor, awayColor, homeBanner, awayBanner) {
  const W = 2048;
  const H = 128;
  const c = document.createElement("canvas");
  c.width = W;
  c.height = H;
  const ctx = c.getContext("2d");

  const panels = [
    { bg: "#09111f", accent: "#22c55e", text: "PARALLEL SEASON FC", sub: "OFFICIAL WORLD HOST" },
    { bg: homeColor || "#c8102e", accent: "#ffffff", text: (homeName || "HOME").toUpperCase(), sub: homeBanner },
    { bg: "#0f172a", accent: "#facc15", text: "CLASSIC FUTBOL 99", sub: "60HZ DETERMINISTIC ENGINE" },
    { bg: awayColor || "#1d4ed8", accent: "#ffffff", text: (awayName || "AWAY").toUpperCase(), sub: awayBanner },
    { bg: "#064e3b", accent: "#34d399", text: "5V5 STREET & STADIUM", sub: "EVERY RESULT MATTERS" },
    { bg: "#1e1b4b", accent: "#38bdf8", text: "PROFILE CARTRIDGE", sub: "VERIFIED REPLAY LOGS" },
  ];

  const pw = W / panels.length;
  panels.forEach((p, idx) => {
    const x = idx * pw;
    ctx.fillStyle = p.bg;
    ctx.fillRect(x, 0, pw, H);
    ctx.fillStyle = p.accent;
    ctx.fillRect(x, 0, pw, 8);
    ctx.fillRect(x, H - 6, pw, 6);
    ctx.strokeStyle = "rgba(255,255,255,0.22)";
    ctx.lineWidth = 3;
    ctx.strokeRect(x + 2, 2, pw - 4, H - 4);

    ctx.fillStyle = "#ffffff";
    ctx.font = "900 34px 'Arial Narrow', Impact, sans-serif";
    ctx.textAlign = "center";
    ctx.fillText(p.text, x + pw * 0.5, 64);

    ctx.fillStyle = p.accent;
    ctx.font = "700 17px Inter, Arial, sans-serif";
    ctx.fillText(p.sub, x + pw * 0.5, 98);
  });

  const tex = new THREE.CanvasTexture(c);
  tex.colorSpace = THREE.SRGBColorSpace;
  tex.wrapS = THREE.RepeatWrapping;
  return tex;
}

function createGlowSpriteTexture() {
  const c = document.createElement("canvas");
  c.width = 128;
  c.height = 128;
  const ctx = c.getContext("2d");
  const g = ctx.createRadialGradient(64, 64, 2, 64, 64, 64);
  g.addColorStop(0.0, "rgba(255, 253, 240, 0.95)");
  g.addColorStop(0.25, "rgba(224, 242, 254, 0.45)");
  g.addColorStop(0.6, "rgba(56, 189, 248, 0.12)");
  g.addColorStop(1.0, "rgba(0, 0, 0, 0)");
  ctx.fillStyle = g;
  ctx.fillRect(0, 0, 128, 128);
  return retainGpu(new THREE.CanvasTexture(c));
}

function createJerseyTextures(kit, shirtNumber, surname, clubShort) {
  const makeSide = (isBack) => {
    const c = document.createElement("canvas");
    c.width = 256;
    c.height = 256;
    const ctx = c.getContext("2d");

    ctx.fillStyle = kit.shirt;
    ctx.fillRect(0, 0, 256, 256);

    if (kit.pattern === "pinstripes") {
      ctx.fillStyle = "rgba(255,255,255,0.12)";
      for (let x = 20; x < 240; x += 24) ctx.fillRect(x, 0, 4, 256);
    } else if (kit.pattern === "shoulder_stripes") {
      ctx.fillStyle = kit.trim;
      for (let s = 0; s < 3; s++) {
        ctx.fillRect(22 + s * 11, 0, 6, 48);
        ctx.fillRect(206 + s * 11, 0, 6, 48);
      }
    } else if (kit.pattern === "chevron_subtle") {
      ctx.strokeStyle = "rgba(15, 23, 42, 0.12)";
      ctx.lineWidth = 14;
      for (let y = 40; y < 240; y += 44) {
        ctx.beginPath();
        ctx.moveTo(20, y);
        ctx.lineTo(128, y + 28);
        ctx.lineTo(236, y);
        ctx.stroke();
      }
    }

    // Subtle fabric fold & oblique contour shading
    const shade = ctx.createLinearGradient(0, 0, 256, 0);
    shade.addColorStop(0, "rgba(0,0,0,0.24)");
    shade.addColorStop(0.18, "rgba(0,0,0,0)");
    shade.addColorStop(0.5, "rgba(255,255,255,0.05)");
    shade.addColorStop(0.82, "rgba(0,0,0,0)");
    shade.addColorStop(1, "rgba(0,0,0,0.24)");
    ctx.fillStyle = shade;
    ctx.fillRect(0, 0, 256, 256);

    ctx.fillStyle = kit.trim;
    if (!isBack) {
      // V-neck collar detail
      ctx.beginPath();
      ctx.moveTo(86, 0);
      ctx.lineTo(128, 36);
      ctx.lineTo(170, 0);
      ctx.lineTo(156, 0);
      ctx.lineTo(128, 22);
      ctx.lineTo(100, 0);
      ctx.closePath();
      ctx.fill();

      // Club crest shield on left chest
      ctx.beginPath();
      ctx.arc(166, 70, 15, 0, Math.PI * 2);
      ctx.fill();
      ctx.fillStyle = kit.shirt;
      ctx.font = "900 10px Arial, sans-serif";
      ctx.textAlign = "center";
      ctx.fillText((clubShort || "FC").slice(0, 3), 166, 74);

      ctx.fillStyle = kit.trim;
      ctx.fillRect(76, 65, 18, 8);

      ctx.fillStyle = kit.numberColor;
      ctx.font = "900 22px 'Arial Narrow', Impact, sans-serif";
      ctx.textAlign = "center";
      ctx.fillText("PARALLEL", 128, 128);
      ctx.font = "700 12px Arial, sans-serif";
      ctx.fillText("SEASON FC", 128, 145);
    } else {
      const cleanName = (surname || "").slice(0, 11);
      if (cleanName) {
        ctx.font = "900 26px 'Arial Narrow', Impact, sans-serif";
        ctx.textAlign = "center";
        ctx.lineWidth = 5;
        ctx.strokeStyle = kit.numberOutline;
        ctx.strokeText(cleanName, 128, 62);
        ctx.fillStyle = kit.numberColor;
        ctx.fillText(cleanName, 128, 62);
      }

      const numStr = String(shirtNumber || 9);
      ctx.font = "900 112px 'Arial Narrow', Impact, sans-serif";
      ctx.textAlign = "center";
      ctx.lineWidth = 8;
      ctx.strokeStyle = kit.numberOutline;
      ctx.strokeText(numStr, 128, 174);
      ctx.fillStyle = kit.numberColor;
      ctx.fillText(numStr, 128, 174);
    }

    const tex = new THREE.CanvasTexture(c);
    tex.colorSpace = THREE.SRGBColorSpace;
    return tex;
  };

  return {
    front: makeSide(false),
    back: makeSide(true),
  };
}

function createNameplateSpriteTexture(surname, shirtNumber, isControlled, side) {
  const c = document.createElement("canvas");
  c.width = 256;
  c.height = 64;
  const ctx = c.getContext("2d");
  ctx.clearRect(0, 0, 256, 64);

  const label = `${shirtNumber} ${(surname || "PLAYER").slice(0, 11)}`;
  ctx.font = "800 24px Inter, system-ui, sans-serif";
  const tw = Math.min(220, ctx.measureText(label).width + 28);
  const x = (256 - tw) * 0.5;

  ctx.fillStyle = "rgba(7, 12, 22, 0.84)";
  ctx.strokeStyle = isControlled
    ? side === 0
      ? "#fbbf24"
      : "#38bdf8"
    : "rgba(226, 232, 240, 0.45)";
  ctx.lineWidth = 3;
  ctx.beginPath();
  ctx.roundRect(x, 10, tw, 40, 8);
  ctx.fill();
  ctx.stroke();

  ctx.fillStyle = "#f8fafc";
  ctx.textAlign = "center";
  ctx.fillText(label, 128, 38);

  const tex = new THREE.CanvasTexture(c);
  tex.colorSpace = THREE.SRGBColorSpace;
  return tex;
}

function buildHairMesh(style, hairColorHex) {
  const mat = new THREE.MeshStandardMaterial({
    color: hairColorHex,
    roughness: 0.8,
    metalness: 0.04,
  });
  const group = new THREE.Group();

  if (style === "buzz") {
    const cap = new THREE.SphereGeometry(0.141, 16, 12, 0, Math.PI * 2, 0, Math.PI * 0.56);
    cap.scale(0.91, 1.11, 0.99);
    const m = new THREE.Mesh(cap, mat);
    m.position.y = 0.012;
    group.add(m);
  } else if (style === "curls") {
    const cap = new THREE.SphereGeometry(0.148, 16, 12, 0, Math.PI * 2, 0, Math.PI * 0.62);
    cap.scale(0.94, 1.12, 1.01);
    const m = new THREE.Mesh(cap, mat);
    m.position.set(0, 0.022, -0.008);
    group.add(m);
    for (let i = 0; i < 10; i++) {
      const puff = new THREE.Mesh(new THREE.SphereGeometry(0.044, 8, 8), mat);
      const ang = (i / 10) * Math.PI * 2;
      puff.position.set(Math.cos(ang) * 0.086, 0.115 + (i % 2) * 0.018, Math.sin(ang) * 0.086);
      group.add(puff);
    }
  } else if (style === "swept") {
    const cap = new THREE.SphereGeometry(0.146, 16, 12, 0, Math.PI * 2, 0, Math.PI * 0.62);
    cap.scale(0.93, 1.12, 1.03);
    const m = new THREE.Mesh(cap, mat);
    m.position.set(0, 0.02, -0.008);
    group.add(m);
    const wave = new THREE.Mesh(new THREE.SphereGeometry(0.075, 12, 10), mat);
    wave.scale.set(1.25, 0.55, 0.95);
    wave.position.set(0.012, 0.138, 0.048);
    wave.rotation.x = -0.22;
    group.add(wave);
  } else if (style === "bun") {
    const cap = new THREE.SphereGeometry(0.144, 16, 12, 0, Math.PI * 2, 0, Math.PI * 0.6);
    cap.scale(0.92, 1.12, 1.0);
    group.add(new THREE.Mesh(cap, mat));
    const bun = new THREE.Mesh(new THREE.SphereGeometry(0.048, 10, 10), mat);
    bun.position.set(0, 0.085, -0.142);
    group.add(bun);
  } else if (style === "fade") {
    const crown = new THREE.SphereGeometry(0.132, 16, 10, 0, Math.PI * 2, 0, Math.PI * 0.48);
    crown.scale(0.92, 1.05, 0.98);
    const m = new THREE.Mesh(crown, mat);
    m.position.set(0, 0.036, 0);
    group.add(m);
  } else {
    const cap = new THREE.SphereGeometry(0.145, 16, 12, 0, Math.PI * 2, 0, Math.PI * 0.58);
    cap.scale(0.92, 1.12, 1.0);
    const m = new THREE.Mesh(cap, mat);
    m.position.y = 0.016;
    group.add(m);
  }
  return group;
}

function resolveLikeness(footballerId, fallbackSeed) {
  if (footballerId && PLAYER_LIKENESS[footballerId]) {
    const p = PLAYER_LIKENESS[footballerId];
    return {
      skinHex: SKIN_TONES[p.skin % SKIN_TONES.length],
      hairHex: HAIR_COLORS[p.hair % HAIR_COLORS.length],
      style: p.style,
      beard: Boolean(p.beard),
    };
  }
  const h = hashStr(footballerId || String(fallbackSeed));
  const styles = ["crop", "fade", "curls", "swept", "buzz"];
  return {
    skinHex: SKIN_TONES[h % SKIN_TONES.length],
    hairHex: HAIR_COLORS[(h >>> 3) % HAIR_COLORS.length],
    style: styles[(h >>> 6) % styles.length],
    beard: (h & 1) === 1,
  };
}

// ============================================================================
// 3. SCULPTED ARTICULATED FOOTBALLER ACTOR & BIOMECHANICAL IK ANIMATOR
// ============================================================================

const UPPER_LEG_LEN = 0.41;
const LOWER_LEG_LEN = 0.40;
const SOLE_HEIGHT = 0.065;
const HIP_SOCKET_L = new THREE.Vector3(0.104, -0.04, 0);
const HIP_SOCKET_R = new THREE.Vector3(-0.104, -0.04, 0);

function createFootballerActor(spec) {
  const { kit, shirtNumber, surname, footballerId, clubShort, isGk, side, scale = 1.12 } = spec;
  const likeness = resolveLikeness(footballerId, shirtNumber + side * 31);

  const skinMat = new THREE.MeshStandardMaterial({ color: likeness.skinHex, roughness: 0.48 });
  const faceTex = createFaceSkinTexture(likeness.skinHex, likeness.hairHex, likeness.beard);
  const headFaceMat = new THREE.MeshStandardMaterial({ map: faceTex, roughness: 0.48 });

  const sleeveMat = new THREE.MeshStandardMaterial({ color: kit.sleeves, roughness: 0.66 });
  const trimMat = new THREE.MeshStandardMaterial({ color: kit.trim, roughness: 0.55 });
  const shortsMat = new THREE.MeshStandardMaterial({ color: kit.shorts, roughness: 0.68 });
  const sockMat = new THREE.MeshStandardMaterial({ color: kit.socks, roughness: 0.7 });
  const sockBandMat = new THREE.MeshStandardMaterial({ color: kit.sockBand, roughness: 0.65 });
  const bootMat = new THREE.MeshStandardMaterial({ color: kit.boots, roughness: 0.3, metalness: 0.16 });
  const bootAccentMat = new THREE.MeshStandardMaterial({ color: kit.bootAccent, roughness: 0.32 });
  const studMat = new THREE.MeshStandardMaterial({ color: "#e2e8f0", metalness: 0.5, roughness: 0.3 });

  const jerseyTex = createJerseyTextures(kit, shirtNumber, surname, clubShort);
  const torsoMats = [
    new THREE.MeshStandardMaterial({ map: jerseyTex.front, roughness: 0.65 }),
    new THREE.MeshStandardMaterial({ map: jerseyTex.back, roughness: 0.65 }),
  ];

  const root = new THREE.Group();
  const bodyScaleGroup = new THREE.Group();
  bodyScaleGroup.scale.setScalar(scale);
  root.add(bodyScaleGroup);

  const contactShadow = new THREE.Mesh(
    SHARED_GEO.shadowDisc,
    new THREE.MeshBasicMaterial({
      color: "#020617",
      transparent: true,
      opacity: 0.34,
      depthWrite: false,
    })
  );
  contactShadow.position.y = 0.014;
  root.add(contactShadow);

  const ringColor = side === 0 ? "#fbbf24" : "#38bdf8";
  const ringMat = new THREE.MeshBasicMaterial({
    color: ringColor,
    transparent: true,
    opacity: 0.92,
    side: THREE.DoubleSide,
    depthWrite: false,
  });
  const ctrlRing = new THREE.Mesh(SHARED_GEO.ring, ringMat);
  ctrlRing.position.y = 0.024;
  ctrlRing.visible = false;
  root.add(ctrlRing);

  const aimWedge = new THREE.Mesh(SHARED_GEO.aimArrow, ringMat);
  aimWedge.position.set(0, 0.028, 0.86);
  aimWedge.visible = false;
  root.add(aimWedge);

  const chargeRingMat = new THREE.MeshBasicMaterial({
    color: "#22c55e",
    transparent: true,
    opacity: 0.9,
    side: THREE.DoubleSide,
    depthWrite: false,
  });
  const chargeRing = new THREE.Mesh(SHARED_GEO.chargeArc, chargeRingMat);
  chargeRing.position.y = 0.03;
  chargeRing.visible = false;
  root.add(chargeRing);

  const overheadCursor = new THREE.Mesh(
    SHARED_GEO.cursorCone,
    new THREE.MeshBasicMaterial({ color: ringColor })
  );
  overheadCursor.position.set(0, 2.38, 0);
  overheadCursor.visible = false;
  root.add(overheadCursor);

  const nameplateTex = createNameplateSpriteTexture(surname, shirtNumber, true, side);
  const nameplate = new THREE.Sprite(
    new THREE.SpriteMaterial({ map: nameplateTex, transparent: true, depthTest: false })
  );
  nameplate.scale.set(2.35, 0.58, 1);
  nameplate.position.set(0, 2.74, 0);
  nameplate.visible = false;
  root.add(nameplate);

  // Pelvis / Center of Mass Root
  const pelvis = new THREE.Group();
  pelvis.position.set(0, 0.895, 0);
  bodyScaleGroup.add(pelvis);

  const pelvisMesh = new THREE.Mesh(SHARED_GEO.pelvis, shortsMat);
  pelvisMesh.castShadow = true;
  pelvis.add(pelvisMesh);

  // Thoracic Spine & Sculpted V-Taper Athletic Torso
  const spine = new THREE.Group();
  spine.position.set(0, 0.065, 0);
  pelvis.add(spine);

  const torso = new THREE.Mesh(SHARED_GEO.torso, torsoMats);
  torso.castShadow = true;
  torso.receiveShadow = true;
  spine.add(torso);

  const collar = new THREE.Mesh(SHARED_GEO.collar, trimMat);
  collar.position.set(0, 0.525, -0.002);
  spine.add(collar);

  const neck = new THREE.Mesh(SHARED_GEO.neck, skinMat);
  neck.position.set(0, 0.57, -0.004);
  spine.add(neck);

  // Sculpted Head + Facial Texture + Nose + Ears + Hair
  const headPivot = new THREE.Group();
  headPivot.position.set(0, 0.625, 0);
  spine.add(headPivot);

  const headMesh = new THREE.Mesh(SHARED_GEO.head, headFaceMat);
  headMesh.position.set(0, 0.105, 0);
  headMesh.castShadow = true;
  headPivot.add(headMesh);

  const nose = new THREE.Mesh(SHARED_GEO.nose, skinMat);
  nose.position.set(0, 0.1, 0.136);
  headPivot.add(nose);

  for (const sgn of [-1, 1]) {
    const ear = new THREE.Mesh(SHARED_GEO.ear, skinMat);
    ear.position.set(sgn * 0.125, 0.102, -0.005);
    headPivot.add(ear);
  }

  const hair = buildHairMesh(likeness.style, likeness.hairHex);
  hair.position.set(0, 0.105, 0);
  headPivot.add(hair);

  // Sculpted Articulated Arms (Deltoid -> Tapered Sleeve -> Elbow Sphere -> Tapered Forearm -> Hand/GK Glove)
  function buildArm(sideSign) {
    const shoulder = new THREE.Group();
    shoulder.position.set(sideSign * 0.202, 0.446, 0);
    spine.add(shoulder);

    const delt = new THREE.Mesh(SHARED_GEO.deltoid, sleeveMat);
    delt.castShadow = true;
    shoulder.add(delt);

    const sleeve = new THREE.Mesh(SHARED_GEO.sleeve, sleeveMat);
    sleeve.position.set(0, -0.112, 0);
    sleeve.castShadow = true;
    shoulder.add(sleeve);

    const cuff = new THREE.Mesh(SHARED_GEO.sleeveCuff, trimMat);
    cuff.position.set(0, -0.214, 0);
    shoulder.add(cuff);

    const elbow = new THREE.Group();
    elbow.position.set(0, -0.228, 0);
    shoulder.add(elbow);

    const elbowCap = new THREE.Mesh(SHARED_GEO.elbowSphere, isGk ? sleeveMat : skinMat);
    elbow.add(elbowCap);

    const forearm = new THREE.Mesh(SHARED_GEO.forearm, isGk ? sleeveMat : skinMat);
    forearm.castShadow = true;
    elbow.add(forearm);

    const handMesh = isGk
      ? new THREE.Mesh(
          SHARED_GEO.gkGlove,
          new THREE.MeshStandardMaterial({ color: "#f8fafc", roughness: 0.4 })
        )
      : new THREE.Mesh(SHARED_GEO.hand, skinMat);
    handMesh.position.set(0, -0.25, 0.012);
    handMesh.castShadow = true;
    elbow.add(handMesh);

    return { shoulder, elbow };
  }

  const armL = buildArm(1);
  const armR = buildArm(-1);

  // Sculpted Articulated Legs Driven by Analytical Two-Bone IK!
  function buildLeg(sideSign) {
    const hip = new THREE.Group();
    hip.position.copy(sideSign > 0 ? HIP_SOCKET_L : HIP_SOCKET_R);
    pelvis.add(hip);

    const shortLeg = new THREE.Mesh(SHARED_GEO.shortLeg, shortsMat);
    shortLeg.castShadow = true;
    hip.add(shortLeg);

    const quad = new THREE.Mesh(SHARED_GEO.quad, skinMat);
    quad.castShadow = true;
    hip.add(quad);

    const knee = new THREE.Group();
    knee.position.set(0, -UPPER_LEG_LEN, 0);
    hip.add(knee);

    // Seamless spherical patella cap prevents any cylinder notch at deep 115-deg sprint knee flexion!
    const patella = new THREE.Mesh(SHARED_GEO.patella, skinMat);
    patella.position.set(0, 0, 0.008);
    knee.add(patella);

    const calf = new THREE.Mesh(SHARED_GEO.calfSock, sockMat);
    calf.castShadow = true;
    knee.add(calf);

    const band = new THREE.Mesh(SHARED_GEO.sockBand, sockBandMat);
    band.position.set(0, -0.042, 0);
    knee.add(band);

    const ankle = new THREE.Group();
    ankle.position.set(0, -LOWER_LEG_LEN, 0);
    knee.add(ankle);

    const cleat = new THREE.Mesh(SHARED_GEO.cleat, bootMat);
    cleat.position.set(0, -SOLE_HEIGHT, 0.022);
    cleat.castShadow = true;
    ankle.add(cleat);

    const stripe = new THREE.Mesh(SHARED_GEO.bootStripe, bootAccentMat);
    stripe.position.set(0, -0.036, 0.035);
    ankle.add(stripe);

    // Molded turf studs underneath soleplate
    for (const [sx, sz] of [
      [-0.028, -0.05],
      [0.028, -0.05],
      [-0.03, 0.09],
      [0.03, 0.09],
    ]) {
      const st = new THREE.Mesh(SHARED_GEO.stud, studMat);
      st.position.set(sx, -SOLE_HEIGHT - 0.006, sz);
      ankle.add(st);
    }

    return { hip, knee, ankle };
  }

  const legL = buildLeg(1);
  const legR = buildLeg(-1);

  return {
    root,
    pelvis,
    spine,
    headPivot,
    armL,
    armR,
    legL,
    legR,
    ctrlRing,
    aimWedge,
    chargeRing,
    chargeRingMat,
    overheadCursor,
    nameplate,
    contactShadow,
    isGk,
    side,
    scale,
    stridePhase: (shirtNumber * 1.37) % (Math.PI * 2),
    prevAction: 0,
    kickSwingTicks: 0,
    smoothYaw: side === 0 ? Math.PI * 0.5 : -Math.PI * 0.5,
    prevVx: 0,
    prevVy: 0,
    // Critically damped spring states
    springs: {
      leanPitch: 0,
      leanPitchVel: 0,
      leanRoll: 0,
      leanRollVel: 0,
      pelvisY: 0.895,
      pelvisYVel: 0,
      spineTwist: 0,
      spineTwistVel: 0,
      headYaw: 0,
      headYawVel: 0,
    },
  };
}

function computeCycloidFootTarget(phase, sideSign, speed, speedNorm, isGk) {
  // Stance occupies [0, pi], Swing recovery occupies [pi, 2*pi]
  const p = ((phase % (Math.PI * 2)) + Math.PI * 2) % (Math.PI * 2);
  const lateralX = sideSign * (isGk ? 0.14 : 0.108);

  if (speed < 0.22) {
    // Ground-locked athletic ready stance!
    return {
      x: sideSign * (isGk ? 0.18 : 0.115),
      y: SOLE_HEIGHT,
      z: sideSign > 0 ? 0.05 : -0.05,
      pitch: 0.0,
    };
  }

  // Biomechanical stride half-span scales with speed (0.18m slow jog -> 0.52m full sprint)
  const halfSpan = Math.min(0.54, 0.16 + speed * 0.048);

  if (p < Math.PI) {
    // STANCE PHASE: Foot planted on turf (y = SOLE_HEIGHT) sweeping from front (+halfSpan) to rear (-halfSpan)
    const u = p / Math.PI; // 0 = heel strike, 0.5 = mid-stance, 1.0 = toe-off!
    const z = halfSpan * (1.0 - 2.0 * u);
    // Heel-strike dorsiflexion (-0.24) -> flat (0) -> explosive plantarflexion (+0.52) at toe-off!
    const pitch = u < 0.22 ? -0.24 * (1 - u / 0.22) : u > 0.65 ? ((u - 0.65) / 0.35) * 0.52 : 0.0;
    const toeLiftY = u > 0.72 ? ((u - 0.72) / 0.28) * 0.045 : 0.0;
    return { x: lateralX, y: SOLE_HEIGHT + toeLiftY, z, pitch };
  }

  // SWING RECOVERY PHASE: High-heel sprint recovery arc from rear (-halfSpan) to front (+halfSpan)
  const w = (p - Math.PI) / Math.PI; // 0 -> 1
  const z = -halfSpan + 2.0 * halfSpan * (w - Math.sin(w * Math.PI * 2) * 0.06);
  // Peak heel lift scales from 0.10m jog up to 0.34m full sprint!
  const peakLift = 0.09 + speedNorm * 0.24;
  const liftProfile = Math.sin(w * Math.PI);
  const heelFold = Math.sin(Math.min(1, w * 1.35) * Math.PI);
  const y = SOLE_HEIGHT + liftProfile * peakLift;
  const pitch = 0.55 * heelFold - (w > 0.75 ? ((w - 0.75) / 0.25) * 0.28 : 0);
  return { x: lateralX, y, z, pitch };
}

function animateFootballerActor(actor, state, dt, particleSystem) {
  const {
    wx,
    wy,
    vx,
    vy,
    facingRad,
    action,
    charge,
    isControlled,
    isBallCarrier,
    ballX,
    ballY,
    celebrating,
    tick,
  } = state;

  actor.root.position.set(wx, 0, wy);

  const targetYaw = Math.PI * 0.5 - facingRad;
  let dyaw = targetYaw - actor.smoothYaw;
  while (dyaw > Math.PI) dyaw -= Math.PI * 2;
  while (dyaw < -Math.PI) dyaw += Math.PI * 2;
  actor.smoothYaw += dyaw * Math.min(1, dt * 18);

  const spinExtra = action === 7 ? (tick * 0.42) % (Math.PI * 2) : 0;
  actor.root.rotation.y = actor.smoothYaw + spinExtra;

  // Compute physical world acceleration vector a = dv/dt -> transform into athlete local frame!
  const axWorld = (vx - actor.prevVx) / Math.max(0.008, dt);
  const ayWorld = (vy - actor.prevVy) / Math.max(0.008, dt);
  actor.prevVx = vx;
  actor.prevVy = vy;

  const fwdX = Math.sin(actor.smoothYaw);
  const fwdZ = Math.cos(actor.smoothYaw);
  const leftX = Math.cos(actor.smoothYaw);
  const leftZ = -Math.sin(actor.smoothYaw);

  const accelFwd = Math.max(-22, Math.min(22, axWorld * fwdX + ayWorld * fwdZ));
  const accelLat = Math.max(-22, Math.min(22, axWorld * leftX + ayWorld * leftZ));
  const speed = Math.hypot(vx, vy);
  const speedNorm = Math.min(1.3, speed / 6.8);

  // Emit grass/turf cleat divots on hard cuts or slides!
  if (particleSystem && (Math.hypot(axWorld, ayWorld) > 16.5 || action === 3) && tick % 3 === 0) {
    particleSystem.spawnBurst(wx, 0.04, wy, -vx * 0.22, -vy * 0.22, action === 3 ? 5 : 2);
  }

  // Trigger proximal-to-distal kick whip follow-through on release from Charge (1)
  if (actor.prevAction === 1 && action !== 1) {
    actor.kickSwingTicks = 16;
    if (particleSystem) {
      particleSystem.spawnBurst(wx + fwdX * 0.35, 0.05, wy + fwdZ * 0.35, fwdX * 2.2, fwdZ * 2.2, 4);
    }
  }
  actor.prevAction = action;
  if (actor.kickSwingTicks > 0) actor.kickSwingTicks--;

  // HUD indicators
  actor.ctrlRing.visible = isControlled;
  actor.aimWedge.visible = isControlled;
  actor.overheadCursor.visible = isControlled;
  if (isControlled) {
    actor.overheadCursor.position.y = 2.42 + Math.sin(tick * 0.14) * 0.06;
    actor.overheadCursor.rotation.y = tick * 0.06;
  }
  actor.nameplate.visible = Boolean(isControlled || isBallCarrier);

  if (charge > 0.02) {
    actor.chargeRing.visible = true;
    const s = 0.85 + charge * 0.35;
    actor.chargeRing.scale.set(s, 1, s);
    actor.chargeRingMat.color.setHex(
      charge > 0.78 ? 0xef4444 : charge > 0.45 ? 0xfbbf24 : 0x22c55e
    );
  } else {
    actor.chargeRing.visible = false;
  }

  // Smooth head tracking toward the ball via Critically Damped Spring
  const toBallAng = Math.atan2(ballX - wx, ballY - wy) - actor.smoothYaw;
  let relBallYaw = toBallAng;
  while (relBallYaw > Math.PI) relBallYaw -= Math.PI * 2;
  while (relBallYaw < -Math.PI) relBallYaw += Math.PI * 2;
  const clampedHeadGoal = Math.max(-0.68, Math.min(0.68, relBallYaw * 0.7));
  const headYaw = dampSpring(actor.springs, "headYaw", "headYawVel", clampedHeadGoal, 0.09, dt);
  actor.headPivot.rotation.set(0.05, headYaw, 0);

  // Special Full-Body States: Slide Tackle (3) & Goalkeeper Dive (4)
  if (action === 3) {
    actor.pelvis.position.set(0, 0.26, 0);
    actor.pelvis.rotation.set(-1.18, 0, 0);
    actor.spine.rotation.set(0.34, -0.18, 0);
    actor.legR.hip.rotation.set(-0.92, 0, -0.05);
    actor.legR.knee.rotation.set(0.12, 0, 0);
    actor.legR.ankle.rotation.set(0.35, 0, 0);
    actor.legL.hip.rotation.set(0.32, 0, 0.18);
    actor.legL.knee.rotation.set(1.52, 0, 0);
    actor.legL.ankle.rotation.set(0.2, 0, 0);
    actor.armL.shoulder.rotation.set(-0.55, 0, 0.68);
    actor.armR.shoulder.rotation.set(0.35, 0, -0.75);
    actor.armL.elbow.rotation.set(-0.65, 0, 0);
    actor.armR.elbow.rotation.set(-0.55, 0, 0);
    actor.contactShadow.scale.set(1.55, 1.45, 1);
    return;
  }

  if (action === 4) {
    const lateral = vx * Math.sin(facingRad) - vy * Math.cos(facingRad);
    const diveSign = lateral >= 0 ? -1 : 1;
    actor.pelvis.position.set(0, 0.56, 0);
    actor.pelvis.rotation.set(0, 0, diveSign * 1.28);
    actor.spine.rotation.set(0.08, 0, diveSign * 0.2);
    actor.armL.shoulder.rotation.set(-2.62, 0, 0.24);
    actor.armR.shoulder.rotation.set(-2.62, 0, -0.24);
    actor.armL.elbow.rotation.set(-0.2, 0, 0);
    actor.armR.elbow.rotation.set(-0.2, 0, 0);
    actor.legL.hip.rotation.set(-0.15, 0, 0.18);
    actor.legR.hip.rotation.set(-0.1, 0, -0.18);
    actor.legL.knee.rotation.set(0.42, 0, 0);
    actor.legR.knee.rotation.set(0.25, 0, 0);
    actor.legL.ankle.rotation.set(0.25, 0, 0);
    actor.legR.ankle.rotation.set(0.25, 0, 0);
    actor.contactShadow.scale.set(1.65, 1.2, 1);
    return;
  }

  actor.contactShadow.scale.set(1, 1, 1);

  // Distance-Locked Stride Phase Integration (Eliminates 100% of Foot-Skating!)
  const strideLenMeters = 1.05 + speed * 0.21;
  if (speed > 0.2) {
    actor.stridePhase = (actor.stridePhase + ((speed * dt) / strideLenMeters) * Math.PI * 2) % (Math.PI * 2);
  }

  const s = Math.sin(actor.stridePhase);
  const c = Math.cos(actor.stridePhase);

  // Biomechanical CoM Lean & Centripetal Banking via Critically Damped Springs!
  const goalPitch = Math.max(-0.24, Math.min(0.36, accelFwd * 0.017 + speedNorm * 0.12));
  const goalRoll = Math.max(-0.28, Math.min(0.28, -accelLat * 0.016 - dyaw * 0.45));
  const leanPitch = dampSpring(actor.springs, "leanPitch", "leanPitchVel", goalPitch, 0.085, dt);
  const leanRoll = dampSpring(actor.springs, "leanRoll", "leanRollVel", goalRoll, 0.085, dt);

  // Pelvis vertical spring-mass bounce (dips at mid-stance absorption, peaks in flight!)
  let baseHipH = actor.isGk && speed < 0.4 ? 0.78 : 0.885 - speedNorm * 0.025;
  if (action === 2) baseHipH = 0.76; // Standing lunge tackle drop
  const bounceOffset = speed > 0.25 ? -Math.cos(actor.stridePhase * 2) * 0.038 * Math.min(1, speedNorm + 0.2) : Math.sin(tick * 0.07 + actor.stridePhase) * 0.01;
  const pelvisY = dampSpring(actor.springs, "pelvisY", "pelvisYVel", baseHipH + bounceOffset, 0.055, dt);

  // Counter-rotating pelvis yaw & thoracic spine twist
  const pelvisYaw = speed > 0.25 ? -s * 0.13 * Math.min(1, speedNorm + 0.25) : 0;
  const pelvisHipDrop = speed > 0.25 ? s * 0.055 * Math.min(1, speedNorm + 0.2) : 0;
  actor.pelvis.position.set(0, pelvisY, 0);
  actor.pelvis.rotation.set(leanPitch * 0.42, pelvisYaw, leanRoll + pelvisHipDrop);

  const goalTwist = speed > 0.25 ? s * 0.22 * Math.min(1, speedNorm + 0.25) : 0;
  const spineTwist = dampSpring(actor.springs, "spineTwist", "spineTwistVel", goalTwist, 0.065, dt);
  actor.spine.rotation.set(
    leanPitch * 0.68 + (actor.isGk && speed < 0.4 ? 0.24 : 0.06),
    spineTwist,
    -pelvisHipDrop * 0.75
  );

  // Compute Left & Right Foot Targets & Solve Analytical Two-Bone Leg IK!
  let footL = computeCycloidFootTarget(actor.stridePhase, 1, speed, speedNorm, actor.isGk);
  let footR = computeCycloidFootTarget(actor.stridePhase + Math.PI, -1, speed, speedNorm, actor.isGk);

  // Standing Lunge Tackle override (right foot lunging forward onto turf)
  if (action === 2) {
    footR = { x: -0.11, y: SOLE_HEIGHT, z: 0.56, pitch: -0.18 };
    footL = { x: 0.12, y: SOLE_HEIGHT + 0.03, z: -0.38, pitch: 0.45 };
  }

  // Proximal-to-Distal Instep Kick Wind-Up & Whip Chain!
  if (action === 1 || charge > 0.03) {
    const w = Math.min(1, Math.max(0.28, charge));
    // Left support foot plants firmly beside ball; right kicking leg hyper-extends back with deep knee flexion!
    footL = { x: 0.14, y: SOLE_HEIGHT, z: 0.16 * w, pitch: 0.0 };
    footR = { x: -0.12, y: SOLE_HEIGHT + 0.36 * w, z: -0.38 * w, pitch: 0.65 * w };
    actor.spine.rotation.y = -0.34 * w;
    actor.spine.rotation.z = 0.14 * w; // Slight lateral tilt away from kicking leg (classic Beckham/Roberto Carlos lean!)
  } else if (actor.kickSwingTicks > 0) {
    const u = 1.0 - actor.kickSwingTicks / 16.0; // 0 = start of downswing -> 1 = end of follow-through
    // Phase 1 (0..0.35): Thigh drives forward while shank whips through ball impact
    // Phase 2 (0.35..1.0): High cross-body follow-through arc & smooth return to turf
    const whipArc = Math.sin(u * Math.PI);
    footL = { x: 0.14, y: SOLE_HEIGHT, z: 0.12 * (1 - u), pitch: 0.0 };
    footR = {
      x: -0.11 + whipArc * 0.09, // Slight cross-body follow-through!
      y: SOLE_HEIGHT + whipArc * 0.42,
      z: -0.15 + whipArc * 0.68,
      pitch: -0.35 * whipArc,
    };
    actor.spine.rotation.x -= 0.18 * whipArc;
    actor.spine.rotation.y += 0.32 * whipArc;
  }

  const poleL = new THREE.Vector3(0.07, 0.18, 1.0).normalize();
  const poleR = new THREE.Vector3(-0.07, 0.18, 1.0).normalize();

  solveLegTwoBoneIK(
    actor.pelvis,
    actor.legL,
    HIP_SOCKET_L,
    new THREE.Vector3(footL.x, footL.y, footL.z),
    poleL,
    footL.pitch,
    UPPER_LEG_LEN,
    LOWER_LEG_LEN
  );
  solveLegTwoBoneIK(
    actor.pelvis,
    actor.legR,
    HIP_SOCKET_R,
    new THREE.Vector3(footR.x, footR.y, footR.z),
    poleR,
    footR.pitch,
    UPPER_LEG_LEN,
    LOWER_LEG_LEN
  );

  // Upper-Limb Counter-Balance & Sprint Arm Pump Kinematics
  if (celebrating) {
    actor.armL.shoulder.rotation.set(-2.35 + s * 0.22, 0, 0.48);
    actor.armR.shoulder.rotation.set(-2.35 - s * 0.22, 0, -0.48);
    actor.armL.elbow.rotation.set(-0.28, 0, 0);
    actor.armR.elbow.rotation.set(-0.28, 0, 0);
  } else if (action === 1 || charge > 0.03) {
    const w = Math.min(1, Math.max(0.28, charge));
    // Non-kicking left arm abducts wide & forward to counter kicking-leg angular momentum!
    actor.armL.shoulder.rotation.set(-0.62 * w, 0.25 * w, 0.78 * w);
    actor.armL.elbow.rotation.set(-0.42, 0, 0);
    actor.armR.shoulder.rotation.set(0.38 * w, 0, -0.32);
    actor.armR.elbow.rotation.set(-0.85, 0, 0);
  } else if (actor.kickSwingTicks > 0) {
    const whip = Math.sin((1.0 - actor.kickSwingTicks / 16.0) * Math.PI);
    actor.armL.shoulder.rotation.set(0.48 * whip, 0, 0.55);
    actor.armR.shoulder.rotation.set(-0.68 * whip, 0, -0.38);
    actor.armL.elbow.rotation.set(-0.72, 0, 0);
    actor.armR.elbow.rotation.set(-0.82, 0, 0);
  } else if (speed > 0.25) {
    const amp = Math.min(1.08, 0.32 + speedNorm * 0.72);
    // In local +Z forward: negative shoulder.rotation.x swings arm FORWARD (+Z)
    actor.armL.shoulder.rotation.set(s * 0.82 * amp, 0, 0.14 + speedNorm * 0.06);
    actor.armR.shoulder.rotation.set(-s * 0.82 * amp, 0, -0.14 - speedNorm * 0.06);
    // Elbows flex tighter (~95-110 deg) during sprint drive!
    const elbowFlexL = -0.78 - speedNorm * 0.48 - Math.max(0, -s) * 0.22;
    const elbowFlexR = -0.78 - speedNorm * 0.48 - Math.max(0, s) * 0.22;
    actor.armL.elbow.rotation.set(elbowFlexL, 0, -0.12);
    actor.armR.elbow.rotation.set(elbowFlexR, 0, 0.12);
  } else if (actor.isGk) {
    actor.armL.shoulder.rotation.set(-0.55, 0, 0.44);
    actor.armR.shoulder.rotation.set(-0.55, 0, -0.44);
    actor.armL.elbow.rotation.set(-0.95, 0, -0.15);
    actor.armR.elbow.rotation.set(-0.95, 0, 0.15);
  } else {
    const br = Math.sin(tick * 0.06 + actor.stridePhase) * 0.025;
    actor.armL.shoulder.rotation.set(0.05 + br, 0, 0.17);
    actor.armR.shoulder.rotation.set(-0.05 - br, 0, -0.17);
    actor.armL.elbow.rotation.set(-0.52, 0, -0.08);
    actor.armR.elbow.rotation.set(-0.52, 0, 0.08);
  }
}

// ============================================================================
// 4. TURF CLEAT DIVOT & GRASS SPRAY PARTICLE POOL (Zero Per-Frame Allocation)
// ============================================================================

class TurfParticleSystem {
  constructor(scene) {
    this.capacity = 140;
    const geo = retainGpu(new THREE.BoxGeometry(0.055, 0.055, 0.055));
    const mat = retainGpu(new THREE.MeshLambertMaterial({ color: "#2e8b47" }));
    this.mesh = new THREE.InstancedMesh(geo, mat, this.capacity);
    this.mesh.instanceMatrix.setUsage(THREE.DynamicDrawUsage);
    this.mesh.frustumCulled = false;
    scene.add(this.mesh);

    this.cursor = 0;
    this.particles = Array.from({ length: this.capacity }, () => ({
      life: 0,
      x: 0,
      y: -10,
      z: 0,
      vx: 0,
      vy: 0,
      vz: 0,
    }));
    this.dummy = new THREE.Object3D();
    for (let i = 0; i < this.capacity; i++) {
      this.dummy.position.set(0, -10, 0);
      this.dummy.scale.setScalar(0.001);
      this.dummy.updateMatrix();
      this.mesh.setMatrixAt(i, this.dummy.matrix);
    }
  }

  spawnBurst(x, y, z, baseVx, baseVz, count = 3) {
    for (let k = 0; k < count; k++) {
      const p = this.particles[this.cursor];
      this.cursor = (this.cursor + 1) % this.capacity;
      p.life = 0.38 + (k % 3) * 0.08;
      p.x = x + (((this.cursor * 17) % 11) - 5) * 0.03;
      p.y = y;
      p.z = z + (((this.cursor * 31) % 11) - 5) * 0.03;
      p.vx = baseVx + (((this.cursor * 13) % 9) - 4) * 0.35;
      p.vy = 1.6 + (k % 3) * 0.65;
      p.vz = baseVz + (((this.cursor * 23) % 9) - 4) * 0.35;
    }
  }

  step(dt) {
    let anyActive = false;
    for (let i = 0; i < this.capacity; i++) {
      const p = this.particles[i];
      if (p.life <= 0) continue;
      anyActive = true;
      p.life -= dt;
      if (p.life <= 0) {
        this.dummy.position.set(0, -10, 0);
        this.dummy.scale.setScalar(0.001);
      } else {
        p.vy -= 9.81 * dt;
        p.x += p.vx * dt;
        p.y = Math.max(0.02, p.y + p.vy * dt);
        p.z += p.vz * dt;
        this.dummy.position.set(p.x, p.y, p.z);
        this.dummy.rotation.set(p.life * 8, p.life * 5, 0);
        this.dummy.scale.setScalar(Math.min(1, p.life * 2.8));
      }
      this.dummy.updateMatrix();
      this.mesh.setMatrixAt(i, this.dummy.matrix);
    }
    if (anyActive) {
      this.mesh.instanceMatrix.needsUpdate = true;
    }
  }
}

// ============================================================================
// 5. STADIUM & GRANDSTAND BUILDER
// ============================================================================

function buildGoalAssembly(goalX, sign, goalHW, goalH, netTex, accentHex) {
  const group = new THREE.Group();
  const postMat = new THREE.MeshStandardMaterial({
    color: "#ffffff",
    roughness: 0.22,
    metalness: 0.25,
  });
  const stanchionMat = new THREE.MeshStandardMaterial({
    color: accentHex || "#334155",
    roughness: 0.45,
    metalness: 0.35,
  });

  const postR = 0.068;
  const depth = 1.75;
  const backX = goalX + sign * depth;

  const postGeo = new THREE.CylinderGeometry(postR, postR, goalH, 14);
  for (const zSign of [-1, 1]) {
    const post = new THREE.Mesh(postGeo, postMat);
    post.position.set(goalX, goalH * 0.5, zSign * goalHW);
    post.castShadow = true;
    group.add(post);

    const rearPost = new THREE.Mesh(new THREE.CylinderGeometry(0.035, 0.035, goalH * 0.92, 8), stanchionMat);
    rearPost.position.set(backX, goalH * 0.46, zSign * goalHW);
    group.add(rearPost);

    const topBar = new THREE.Mesh(new THREE.CylinderGeometry(0.03, 0.03, depth * 0.8, 8), stanchionMat);
    topBar.rotation.z = Math.PI * 0.5;
    topBar.position.set(goalX + sign * depth * 0.4, goalH * 0.96, zSign * goalHW);
    group.add(topBar);
  }

  const crossbar = new THREE.Mesh(new THREE.CylinderGeometry(postR, postR, goalHW * 2 + postR * 2, 14), postMat);
  crossbar.rotation.x = Math.PI * 0.5;
  crossbar.position.set(goalX, goalH, 0);
  crossbar.castShadow = true;
  group.add(crossbar);

  const netMat = new THREE.MeshStandardMaterial({
    map: netTex,
    transparent: true,
    opacity: 0.86,
    side: THREE.DoubleSide,
    roughness: 0.85,
    depthWrite: false,
  });

  const netHolder = new THREE.Group();
  netHolder.position.set(goalX, 0, 0);
  group.add(netHolder);

  const roofNet = new THREE.Mesh(new THREE.PlaneGeometry(depth * 0.82, goalHW * 2), netMat);
  roofNet.rotation.x = -Math.PI * 0.5;
  roofNet.position.set(sign * depth * 0.41, goalH - 0.02, 0);
  netHolder.add(roofNet);

  const backSlopeLen = Math.hypot(depth * 0.18, goalH);
  const backNet = new THREE.Mesh(new THREE.PlaneGeometry(goalHW * 2, backSlopeLen), netMat);
  backNet.rotation.y = sign > 0 ? -Math.PI * 0.5 : Math.PI * 0.5;
  backNet.rotation.x = sign * 0.16;
  backNet.position.set(sign * depth * 0.9, goalH * 0.5, 0);
  netHolder.add(backNet);

  for (const zSign of [-1, 1]) {
    const sideNet = new THREE.Mesh(new THREE.PlaneGeometry(depth, goalH), netMat);
    sideNet.position.set(sign * depth * 0.5, goalH * 0.5, zSign * goalHW);
    netHolder.add(sideNet);
  }

  return { group, netHolder };
}

function buildStadiumWorld(snapshot, maxAnisotropy, glowTex) {
  const stadium = new THREE.Group();
  const fmt = snapshot.format;
  const pitchLen = fmt.pitch_length_mm / 1000;
  const pitchWid = fmt.pitch_width_mm / 1000;
  const halfL = pitchLen * 0.5;
  const halfW = pitchWid * 0.5;
  const goalHW = fmt.goal_width_mm / 2000;
  const goalH = fmt.goal_height_mm / 2000 * 2;

  const kits = resolveMatchKits(
    snapshot.home.club_id,
    snapshot.away.club_id,
    snapshot.home.primary_color,
    snapshot.away.primary_color
  );

  const trackFloor = new THREE.Mesh(
    new THREE.PlaneGeometry(pitchLen + 70, pitchWid + 70),
    new THREE.MeshStandardMaterial({ color: "#141c27", roughness: 0.92 })
  );
  trackFloor.rotation.x = -Math.PI * 0.5;
  trackFloor.position.y = -0.02;
  trackFloor.receiveShadow = true;
  stadium.add(trackFloor);

  const trackBorder = new THREE.Mesh(
    new THREE.PlaneGeometry(pitchLen + 17.5, pitchWid + 16.0),
    new THREE.MeshStandardMaterial({ color: "#3b231d", roughness: 0.88 })
  );
  trackBorder.rotation.x = -Math.PI * 0.5;
  trackBorder.position.y = -0.01;
  trackBorder.receiveShadow = true;
  stadium.add(trackBorder);

  const { tex: pitchTex, totalW, totalH } = createPitchTexture(pitchLen, pitchWid, fmt, maxAnisotropy);
  const pitchMesh = new THREE.Mesh(
    new THREE.PlaneGeometry(totalW, totalH),
    new THREE.MeshStandardMaterial({ map: pitchTex, roughness: 0.86, metalness: 0.02 })
  );
  pitchMesh.rotation.x = -Math.PI * 0.5;
  pitchMesh.receiveShadow = true;
  stadium.add(pitchMesh);

  const netTex = createGoalNetTexture();
  const leftGoal = buildGoalAssembly(-halfL, -1, goalHW, goalH, netTex, kits.homeOutfield.shirt);
  const rightGoal = buildGoalAssembly(halfL, 1, goalHW, goalH, netTex, kits.awayOutfield.shirt);
  stadium.add(leftGoal.group);
  stadium.add(rightGoal.group);

  const flagPoleGeo = new THREE.CylinderGeometry(0.022, 0.022, 1.45, 8);
  const flagPoleMat = new THREE.MeshStandardMaterial({ color: "#f8fafc", roughness: 0.4 });
  const flagClothGeo = new THREE.PlaneGeometry(0.42, 0.28);
  const cornerFlags = [];
  for (const [cx, cz, col] of [
    [-halfL, -halfW, kits.homeOutfield.shirt],
    [-halfL, halfW, kits.homeOutfield.shirt],
    [halfL, -halfW, kits.awayOutfield.shirt],
    [halfL, halfW, kits.awayOutfield.shirt],
  ]) {
    const pole = new THREE.Mesh(flagPoleGeo, flagPoleMat);
    pole.position.set(cx, 0.725, cz);
    pole.castShadow = true;
    stadium.add(pole);

    const cloth = new THREE.Mesh(
      flagClothGeo,
      new THREE.MeshStandardMaterial({ color: col, side: THREE.DoubleSide, roughness: 0.6 })
    );
    cloth.position.set(cx + 0.21, 1.3, cz);
    stadium.add(cloth);
    cornerFlags.push(cloth);
  }

  // Dugouts
  const shelterMat = new THREE.MeshStandardMaterial({ color: "#0f172a", roughness: 0.4, metalness: 0.4 });
  const glassMat = new THREE.MeshStandardMaterial({
    color: "#94a3b8",
    transparent: true,
    opacity: 0.36,
    roughness: 0.15,
  });
  for (const [sideIdx, bx, benchKit] of [
    [0, -pitchLen * 0.2, kits.homeOutfield],
    [1, pitchLen * 0.2, kits.awayOutfield],
  ]) {
    const dugout = new THREE.Group();
    dugout.position.set(bx, 0, -halfW - 3.05);
    const frame = new THREE.Mesh(new THREE.BoxGeometry(5.2, 1.85, 1.25), glassMat);
    frame.position.y = 0.925;
    dugout.add(frame);

    const roof = new THREE.Mesh(new THREE.BoxGeometry(5.4, 0.14, 1.45), shelterMat);
    roof.position.y = 1.9;
    dugout.add(roof);

    const seatBar = new THREE.Mesh(
      new THREE.BoxGeometry(4.8, 0.45, 0.55),
      new THREE.MeshStandardMaterial({ color: benchKit.shirt, roughness: 0.7 })
    );
    seatBar.position.set(0, 0.35, -0.2);
    dugout.add(seatBar);

    for (let s = 0; s < 4; s++) {
      const subTorso = new THREE.Mesh(
        new THREE.CapsuleGeometry(0.14, 0.28, 4, 8),
        new THREE.MeshStandardMaterial({ color: benchKit.shirt, roughness: 0.7 })
      );
      subTorso.position.set(-1.5 + s * 1.0, 0.78, -0.1);
      dugout.add(subTorso);
      const subHead = new THREE.Mesh(
        new THREE.SphereGeometry(0.115, 10, 8),
        new THREE.MeshStandardMaterial({ color: SKIN_TONES[(s + sideIdx * 2) % SKIN_TONES.length] })
      );
      subHead.position.set(-1.5 + s * 1.0, 1.13, -0.1);
      dugout.add(subHead);
    }
    stadium.add(dugout);
  }

  // Adboards
  const adTex = createAdboardTexture(
    snapshot.home.club_name,
    snapshot.away.club_name,
    kits.homeOutfield.shirt,
    kits.awayOutfield.shirt,
    kits.homeBanner,
    kits.awayBanner
  );
  const adMat = new THREE.MeshStandardMaterial({
    map: adTex,
    emissive: "#ffffff",
    emissiveMap: adTex,
    emissiveIntensity: 0.26,
    roughness: 0.45,
  });

  const adDistZ = halfW + 4.25;
  const adDistX = halfL + 4.65;
  const adH = 1.05;

  const northAd = new THREE.Mesh(new THREE.BoxGeometry(adDistX * 2, adH, 0.18), adMat);
  northAd.position.set(0, adH * 0.5, -adDistZ);
  northAd.castShadow = true;
  stadium.add(northAd);

  const southAd = new THREE.Mesh(new THREE.BoxGeometry(adDistX * 2, adH * 0.7, 0.18), adMat);
  southAd.position.set(0, adH * 0.35, adDistZ + 1.2);
  stadium.add(southAd);

  for (const xSign of [-1, 1]) {
    const endAd = new THREE.Mesh(new THREE.BoxGeometry(0.18, adH, adDistZ * 2), adMat);
    endAd.position.set(xSign * adDistX, adH * 0.5, 0);
    endAd.castShadow = true;
    stadium.add(endAd);
  }

  // Grandstands + Crowd
  const concreteMat = new THREE.MeshStandardMaterial({ color: "#1e293b", roughness: 0.88 });
  const riserDarkMat = new THREE.MeshStandardMaterial({ color: "#0f172a", roughness: 0.92 });
  const stairMat = new THREE.MeshStandardMaterial({ color: "#ca8a04", roughness: 0.7 });
  const roofMat = new THREE.MeshStandardMaterial({ color: "#0b1324", roughness: 0.65, metalness: 0.3 });
  const fasciaMatHome = new THREE.MeshStandardMaterial({
    color: kits.homeOutfield.shirt,
    emissive: kits.homeOutfield.shirt,
    emissiveIntensity: 0.22,
    roughness: 0.5,
  });
  const fasciaMatAway = new THREE.MeshStandardMaterial({
    color: kits.awayOutfield.shirt,
    emissive: kits.awayOutfield.shirt,
    emissiveIntensity: 0.22,
    roughness: 0.5,
  });

  const crowdPositions = [];
  const crowdColors = [];
  const palette = [
    new THREE.Color(kits.homeOutfield.shirt),
    new THREE.Color(kits.homeOutfield.shirt),
    new THREE.Color(kits.awayOutfield.shirt),
    new THREE.Color("#f8fafc"),
    new THREE.Color("#334155"),
    new THREE.Color("#fbbf24"),
    new THREE.Color("#94a3b8"),
  ];

  let cSeed = 0x99fa2026;
  const cRand = () => {
    cSeed = (Math.imul(cSeed, 1664525) + 1013904223) >>> 0;
    return cSeed / 4294967296;
  };

  function buildStand(length, startDist, rotY, tiers, addRoof, isHomeBias) {
    const stand = new THREE.Group();
    stand.rotation.y = rotY;

    const stepDepth = 1.15;
    const stepRise = 0.68;

    const wall = new THREE.Mesh(new THREE.BoxGeometry(length, 1.65, 0.6), riserDarkMat);
    wall.position.set(0, 0.825, -startDist);
    stand.add(wall);

    const ribbon = new THREE.Mesh(
      new THREE.BoxGeometry(length, 0.28, 0.66),
      isHomeBias ? fasciaMatHome : fasciaMatAway
    );
    ribbon.position.set(0, 1.55, -startDist);
    stand.add(ribbon);

    for (let t = 0; t < tiers; t++) {
      const y = 1.65 + t * stepRise;
      const z = -startDist - 0.6 - t * stepDepth;
      const tierMesh = new THREE.Mesh(new THREE.BoxGeometry(length, stepRise + 0.15, stepDepth + 0.1), concreteMat);
      tierMesh.position.set(0, y, z);
      tierMesh.receiveShadow = true;
      stand.add(tierMesh);

      const spacing = 0.52;
      const count = Math.floor((length - 2.0) / spacing);
      for (let i = 0; i < count; i++) {
        const lx = -((count - 1) * spacing) * 0.5 + i * spacing;
        if (i % 24 === 12 || i % 24 === 13) {
          if (i % 24 === 12) {
            const stair = new THREE.Mesh(
              new THREE.BoxGeometry(0.95, stepRise + 0.18, stepDepth + 0.14),
              stairMat
            );
            stair.position.set(lx + spacing * 0.5, y + 0.02, z + 0.02);
            stand.add(stair);
          }
          continue;
        }
        const jitterX = lx + (cRand() - 0.5) * 0.12;
        const ly = y + stepRise * 0.58;
        const lz = z + 0.08;

        const v = new THREE.Vector3(jitterX, ly, lz).applyAxisAngle(new THREE.Vector3(0, 1, 0), rotY);
        crowdPositions.push({ x: v.x, y: v.y, z: v.z, phase: cRand() * Math.PI * 2 });

        const r = cRand();
        const col =
          r < 0.44
            ? isHomeBias
              ? palette[0]
              : palette[2]
            : r < 0.62
              ? isHomeBias
                ? palette[2]
                : palette[0]
              : palette[3 + Math.floor(cRand() * 4)];
        crowdColors.push(col);
      }
    }

    const backZ = -startDist - 0.6 - tiers * stepDepth;
    const topY = 1.65 + tiers * stepRise;
    const backWall = new THREE.Mesh(new THREE.BoxGeometry(length + 3.0, topY + 4.2, 1.0), riserDarkMat);
    backWall.position.set(0, (topY + 4.2) * 0.5, backZ - 0.5);
    stand.add(backWall);

    if (addRoof) {
      const roofDepth = tiers * stepDepth + 3.8;
      const canopy = new THREE.Mesh(new THREE.BoxGeometry(length + 3.2, 0.45, roofDepth), roofMat);
      canopy.position.set(0, topY + 4.1, backZ + roofDepth * 0.5 - 0.6);
      canopy.rotation.x = 0.06;
      stand.add(canopy);

      const fascia = new THREE.Mesh(
        new THREE.BoxGeometry(length + 3.2, 0.52, 0.25),
        new THREE.MeshStandardMaterial({ color: "#e2e8f0", roughness: 0.4 })
      );
      fascia.position.set(0, topY + 3.9, backZ + roofDepth - 0.65);
      stand.add(fascia);
    }

    stadium.add(stand);
  }

  const standSpanX = (adDistX + 8.5) * 2;
  const standSpanZ = (adDistZ + 7.5) * 2;
  buildStand(standSpanX, adDistZ + 2.4, 0, 11, true, true);
  buildStand(standSpanZ, adDistX + 2.8, Math.PI * 0.5, 10, true, true);
  buildStand(standSpanZ, adDistX + 2.8, -Math.PI * 0.5, 10, true, false);
  buildStand(standSpanX, adDistZ + 6.5, Math.PI, 5, false, false);

  const personGeo = new THREE.CapsuleGeometry(0.21, 0.38, 4, 6);
  const crowdMat = new THREE.MeshLambertMaterial({ color: "#ffffff" });
  const crowdMesh = new THREE.InstancedMesh(personGeo, crowdMat, crowdPositions.length);
  const dummy = new THREE.Object3D();
  for (let i = 0; i < crowdPositions.length; i++) {
    const p = crowdPositions[i];
    dummy.position.set(p.x, p.y, p.z);
    dummy.updateMatrix();
    crowdMesh.setMatrixAt(i, dummy.matrix);
    crowdMesh.setColorAt(i, crowdColors[i]);
  }
  crowdMesh.instanceMatrix.needsUpdate = true;
  if (crowdMesh.instanceColor) crowdMesh.instanceColor.needsUpdate = true;
  stadium.add(crowdMesh);

  // Floodlight Towers
  const pylonMat = new THREE.MeshStandardMaterial({ color: "#475569", metalness: 0.6, roughness: 0.4 });
  const bulbMat = new THREE.MeshBasicMaterial({ color: "#fffbeb" });
  const towerH = pitchLen > 80 ? 34 : 26;
  for (const [tx, tz] of [
    [-adDistX - 6.5, -adDistZ - 6.5],
    [adDistX + 6.5, -adDistZ - 6.5],
    [-adDistX - 6.5, adDistZ + 6.5],
    [adDistX + 6.5, adDistZ + 6.5],
  ]) {
    const tower = new THREE.Group();
    tower.position.set(tx, 0, tz);
    const mast = new THREE.Mesh(new THREE.CylinderGeometry(0.32, 0.58, towerH, 8), pylonMat);
    mast.position.y = towerH * 0.5;
    tower.add(mast);

    const headFrame = new THREE.Mesh(new THREE.BoxGeometry(4.4, 2.4, 0.65), pylonMat);
    headFrame.position.y = towerH + 0.8;
    const yaw = Math.atan2(-tx, -tz);
    tower.rotation.set(0, yaw, 0);
    headFrame.rotation.x = 0.42;

    const bulbPanel = new THREE.Mesh(new THREE.PlaneGeometry(4.0, 2.0), bulbMat);
    bulbPanel.position.z = 0.36;
    headFrame.add(bulbPanel);
    tower.add(headFrame);

    const halo = new THREE.Sprite(
      new THREE.SpriteMaterial({
        map: glowTex,
        transparent: true,
        blending: THREE.AdditiveBlending,
        depthWrite: false,
      })
    );
    halo.position.set(0, towerH + 0.8, 0);
    halo.scale.set(16, 16, 1);
    tower.add(halo);

    stadium.add(tower);
  }

  return {
    stadium,
    kits,
    halfL,
    halfW,
    pitchLen,
    pitchWid,
    leftGoal,
    rightGoal,
    cornerFlags,
    crowdMesh,
    crowdPositions,
  };
}

// ============================================================================
// 6. PUBLIC ENGINE CLASS
// ============================================================================

export const CAMERA_PRESETS = ["tele", "dynamic", "pro", "wide"];
export const CAMERA_LABELS = {
  tele: "TELE BROADCAST",
  dynamic: "DYNAMIC CLOSE",
  pro: "END-TO-END PRO",
  wide: "WIDE TACTICAL",
};

export class ClassicFifa3DEngine {
  constructor(canvas) {
    this.canvas = canvas;
    this.renderer = new THREE.WebGLRenderer({
      canvas,
      antialias: true,
      powerPreference: "high-performance",
    });
    this.renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
    this.renderer.outputColorSpace = THREE.SRGBColorSpace;
    this.renderer.toneMapping = THREE.ACESFilmicToneMapping;
    this.renderer.toneMappingExposure = 1.12;
    this.renderer.shadowMap.enabled = true;
    this.renderer.shadowMap.type = THREE.PCFSoftShadowMap;

    this.scene = new THREE.Scene();
    this.scene.fog = new THREE.FogExp2("#0c192c", 0.0042);

    this.camera = new THREE.PerspectiveCamera(32, 16 / 9, 0.4, 420);
    this.cameraMode = "tele";
    this.viewMode = "attract";

    this.camPos = new THREE.Vector3(0, 15.2, 26.8);
    this.camLook = new THREE.Vector3(0, 0.72, -1.5);

    this.glowTex = createGlowSpriteTexture();
    this.maxAniso = this.renderer.capabilities.getMaxAnisotropy();

    const skyDome = new THREE.Mesh(
      retainGpu(new THREE.SphereGeometry(260, 28, 18)),
      retainGpu(new THREE.MeshBasicMaterial({ map: createSkyTexture(), side: THREE.BackSide, fog: false }))
    );
    this.scene.add(skyDome);

    const hemi = new THREE.HemisphereLight(0xdbeafe, 0x15803d, 0.98);
    this.scene.add(hemi);

    this.keyLight = new THREE.DirectionalLight(0xfffbeb, 2.2);
    this.keyLight.position.set(24, 44, 30);
    this.keyLight.castShadow = true;
    this.keyLight.shadow.mapSize.set(2048, 2048);
    this.keyLight.shadow.camera.near = 8;
    this.keyLight.shadow.camera.far = 140;
    this.keyLight.shadow.bias = -0.0004;
    this.scene.add(this.keyLight);

    const rimLight = new THREE.DirectionalLight(0xbae6fd, 0.88);
    rimLight.position.set(-28, 34, -34);
    this.scene.add(rimLight);

    const frontFillLight = new THREE.DirectionalLight(0xffedd5, 0.85);
    frontFillLight.position.set(0, 14, 38);
    this.scene.add(frontFillLight);

    this.ballRadius = 0.22;
    this.ballMesh = new THREE.Mesh(
      retainGpu(new THREE.SphereGeometry(this.ballRadius, 26, 20)),
      retainGpu(
        new THREE.MeshStandardMaterial({
          map: createTelstarBallTexture(),
          roughness: 0.3,
          metalness: 0.06,
        })
      )
    );
    this.ballMesh.castShadow = true;
    this.scene.add(this.ballMesh);

    this.ballShadow = new THREE.Mesh(
      retainGpu(new THREE.CircleGeometry(0.3, 18)),
      retainGpu(
        new THREE.MeshBasicMaterial({
          color: "#020617",
          transparent: true,
          opacity: 0.45,
          depthWrite: false,
        })
      )
    );
    this.ballShadow.rotation.x = -Math.PI * 0.5;
    this.ballShadow.position.y = 0.018;
    this.scene.add(this.ballShadow);

    this.particles = new TurfParticleSystem(this.scene);

    this.stadiumData = null;
    this.actors = [];
    this.showcaseGroup = new THREE.Group();
    this.showcaseGroup.visible = false;
    this.scene.add(this.showcaseGroup);
    this.showcaseCaptains = null;

    this.netRipple = [0, 0];
    this.goalExcitement = 0;
    this.attractAngle = 0;
    this.prevBallZ = 0;
    this.lastTime = performance.now();

    this.resize();
    window.addEventListener("resize", () => this.resize());
  }

  resize() {
    const w = Math.max(320, window.innerWidth);
    const h = Math.max(240, window.innerHeight);
    this.renderer.setSize(w, h, false);
    this.camera.aspect = w / h;
    this.camera.updateProjectionMatrix();
  }

  cycleCameraPreset() {
    const idx = CAMERA_PRESETS.indexOf(this.cameraMode);
    this.cameraMode = CAMERA_PRESETS[(idx + 1) % CAMERA_PRESETS.length];
    return this.cameraMode;
  }

  setCameraPreset(mode) {
    if (CAMERA_PRESETS.includes(mode)) {
      this.cameraMode = mode;
    }
    return this.cameraMode;
  }

  setViewMode(mode) {
    this.viewMode = mode;
    const showMatchActors = mode !== "showcase";
    for (const a of this.actors) {
      a.root.visible = showMatchActors;
    }
    this.ballMesh.visible = true;
    this.ballShadow.visible = true;
    this.showcaseGroup.visible = mode === "showcase";
  }

  triggerGoalEffect(scoringSide) {
    this.goalExcitement = 3.5;
    if (scoringSide === 0) this.netRipple[1] = 1.0;
    else if (scoringSide === 1) this.netRipple[0] = 1.0;
    else {
      this.netRipple[0] = 0.8;
      this.netRipple[1] = 0.8;
    }
  }

  loadSnapshotScene(snapshot) {
    if (this.stadiumData) {
      this.scene.remove(this.stadiumData.stadium);
      disposeObjectTree(this.stadiumData.stadium);
    }
    for (const a of this.actors) {
      this.scene.remove(a.root);
      disposeObjectTree(a.root);
    }
    this.actors = [];

    this.stadiumData = buildStadiumWorld(snapshot, this.maxAniso, this.glowTex);
    this.scene.add(this.stadiumData.stadium);

    const sx = this.stadiumData.halfL + 8;
    const sz = this.stadiumData.halfW + 10;
    this.keyLight.shadow.camera.left = -sx;
    this.keyLight.shadow.camera.right = sx;
    this.keyLight.shadow.camera.top = sz;
    this.keyLight.shadow.camera.bottom = -sz;
    this.keyLight.shadow.camera.updateProjectionMatrix();

    const pps = snapshot.format.players_per_side;
    const playerScale = pps <= 6 ? 1.14 : 1.04;
    const kits = this.stadiumData.kits;

    for (let side = 0; side < 2; side++) {
      const team = side === 0 ? snapshot.home : snapshot.away;
      const starters = team.starters || [];
      for (let idx = 0; idx < pps; idx++) {
        const m = starters[idx] || {
          footballer_id: `${team.club_id}-${idx}`,
          name: `Player ${idx + 1}`,
          shirt_number: idx + 1,
          role: idx === 0 ? "Goalkeeper" : "Midfielder",
        };
        const isGk = idx === 0 || m.role === "Goalkeeper";
        const kit = isGk
          ? side === 0
            ? kits.homeGk
            : kits.awayGk
          : side === 0
            ? kits.homeOutfield
            : kits.awayOutfield;

        const actor = createFootballerActor({
          kit,
          shirtNumber: m.shirt_number,
          surname: extractSurname(m.name),
          footballerId: m.footballer_id,
          clubShort: team.short_name,
          isGk,
          side,
          scale: playerScale,
        });
        actor.root.visible = this.viewMode !== "showcase";
        this.scene.add(actor.root);
        this.actors.push(actor);
      }
    }
  }

  updatePreMatchShowcase(homeRoster, awayRoster) {
    while (this.showcaseGroup.children.length > 0) {
      const child = this.showcaseGroup.children[0];
      this.showcaseGroup.remove(child);
      disposeObjectTree(child);
    }
    this.showcaseCaptains = null;
    if (!homeRoster || !awayRoster) return;

    const kits = resolveMatchKits(
      homeRoster.club_id,
      awayRoster.club_id,
      homeRoster.primary_color,
      awayRoster.primary_color
    );

    const pickStar = (roster) => {
      const sq = roster.squad || [];
      const out = sq.filter((p) => p.selected_5v5 && p.role !== "Goalkeeper");
      return out[out.length - 1] || sq[1] || sq[0];
    };

    const hStar = pickStar(homeRoster);
    const aStar = pickStar(awayRoster);

    const homeCapt = createFootballerActor({
      kit: kits.homeOutfield,
      shirtNumber: hStar ? hStar.shirt_number : 10,
      surname: extractSurname(hStar ? hStar.name : "CAPTAIN"),
      footballerId: hStar ? hStar.footballer_id : "mun-fwd-1",
      clubShort: homeRoster.short_name,
      isGk: false,
      side: 0,
      scale: 1.24,
    });
    homeCapt.root.position.set(-1.35, 0, 0.25);
    homeCapt.smoothYaw = 0.42;
    homeCapt.root.rotation.y = 0.42;

    const awayCapt = createFootballerActor({
      kit: kits.awayOutfield,
      shirtNumber: aStar ? aStar.shirt_number : 7,
      surname: extractSurname(aStar ? aStar.name : "CAPTAIN"),
      footballerId: aStar ? aStar.footballer_id : "ars-fwd-1",
      clubShort: awayRoster.short_name,
      isGk: false,
      side: 1,
      scale: 1.24,
    });
    awayCapt.root.position.set(1.35, 0, 0.25);
    awayCapt.smoothYaw = -0.42;
    awayCapt.root.rotation.y = -0.42;

    this.showcaseGroup.add(homeCapt.root);
    this.showcaseGroup.add(awayCapt.root);
    this.showcaseCaptains = [homeCapt, awayCapt];
  }

  renderFrame(packed, replayOpts = null) {
    const now = performance.now();
    const dt = Math.min(0.05, Math.max(0.004, (now - this.lastTime) / 1000));
    this.lastTime = now;

    this.particles.step(dt);

    if (!this.stadiumData) {
      this.renderer.render(this.scene, this.camera);
      return;
    }

    const { halfL, halfW, pitchLen, leftGoal, rightGoal, cornerFlags, crowdMesh, crowdPositions } =
      this.stadiumData;

    const waveT = now * 0.004;
    for (let i = 0; i < cornerFlags.length; i++) {
      cornerFlags[i].rotation.y = Math.sin(waveT + i * 1.4) * 0.28;
    }

    for (let g = 0; g < 2; g++) {
      if (this.netRipple[g] > 0.005) {
        this.netRipple[g] *= Math.pow(0.04, dt);
        const holder = g === 0 ? leftGoal.netHolder : rightGoal.netHolder;
        const bulge = Math.sin(now * 0.035) * this.netRipple[g] * 0.14;
        holder.scale.set(1 + Math.abs(bulge), 1 + bulge * 0.3, 1 + bulge * 0.4);
      }
    }

    if (this.goalExcitement > 0) {
      this.goalExcitement = Math.max(0, this.goalExcitement - dt);
    }
    if (crowdMesh && (this.goalExcitement > 0.01 || Math.floor(now / 50) % 2 === 0)) {
      const dummy = new THREE.Object3D();
      const amp = this.goalExcitement > 0.01 ? 0.24 : 0.035;
      const spd = this.goalExcitement > 0.01 ? 0.014 : 0.0035;
      for (let i = 0; i < crowdPositions.length; i += 2) {
        const p = crowdPositions[i];
        dummy.position.set(p.x, p.y + Math.abs(Math.sin(now * spd + p.phase)) * amp, p.z);
        dummy.updateMatrix();
        crowdMesh.setMatrixAt(i, dummy.matrix);
      }
      crowdMesh.instanceMatrix.needsUpdate = true;
    }

    let ballX = 0;
    let ballY = 0;
    let ballZ = 0;

    if (this.viewMode === "showcase") {
      this.ballMesh.position.set(0, this.ballRadius, 0.85);
      this.ballMesh.rotation.y += dt * 0.8;
      this.ballShadow.position.set(0, 0.018, 0.85);

      if (this.showcaseCaptains) {
        const idleTick = Math.floor(now / 16);
        const poleL = new THREE.Vector3(0.08, 0.2, 1.0).normalize();
        const poleR = new THREE.Vector3(-0.08, 0.2, 1.0).normalize();

        this.showcaseCaptains.forEach((cap, cIdx) => {
          const sgn = cIdx === 0 ? 1 : -1;
          const breathe = Math.sin(now * 0.003 + cap.stridePhase) * 0.012;
          cap.pelvis.position.set(0, 0.885 + breathe, 0);
          cap.pelvis.rotation.set(0.03, -sgn * 0.06, sgn * 0.025);
          cap.spine.rotation.set(0.04, -sgn * 0.08, -sgn * 0.02);

          // Ground-locked athletic stance solved with Two-Bone IK so cleats sit flush on grass!
          solveLegTwoBoneIK(
            cap.pelvis,
            cap.legL,
            HIP_SOCKET_L,
            new THREE.Vector3(0.135, SOLE_HEIGHT, 0.06 * sgn),
            poleL,
            0.0,
            UPPER_LEG_LEN,
            LOWER_LEG_LEN
          );
          solveLegTwoBoneIK(
            cap.pelvis,
            cap.legR,
            HIP_SOCKET_R,
            new THREE.Vector3(-0.135, SOLE_HEIGHT, -0.05 * sgn),
            poleR,
            0.0,
            UPPER_LEG_LEN,
            LOWER_LEG_LEN
          );

          cap.armL.shoulder.rotation.set(0.1, 0, 0.24);
          cap.armR.shoulder.rotation.set(-0.08, 0, -0.24);
          cap.armL.elbow.rotation.set(-0.62, 0, -0.14);
          cap.armR.elbow.rotation.set(-0.58, 0, 0.14);
          cap.headPivot.rotation.set(0.04, Math.sin(idleTick * 0.02 + cap.stridePhase) * 0.09 - sgn * 0.08, 0);
        });
      }

      const sway = Math.sin(now * 0.0007) * 0.45;
      this.camera.fov = 31;
      this.camera.updateProjectionMatrix();
      this.camPos.lerp(new THREE.Vector3(sway, 1.48, 6.25), Math.min(1, dt * 8));
      this.camLook.lerp(new THREE.Vector3(0, 0.62, 0.15), Math.min(1, dt * 8));
      this.camera.position.copy(this.camPos);
      this.camera.lookAt(this.camLook);
      this.renderer.render(this.scene, this.camera);
      return;
    }

    if (packed) {
      const tick = packed[0];
      const phase = packed[3];
      const pps = packed[6];
      const homeCtrl = packed[7];
      const awayCtrl = packed[8];
      ballX = packed[9] / 1000;
      ballY = packed[10] / 1000;
      ballZ = packed[11] / 1000;
      const ballVx = packed[12] / 1000;
      const ballVy = packed[13] / 1000;
      const ownerSide = packed[15];
      const ownerIdx = packed[16];

      // Spawn subtle turf spray when an aerial ball bounces onto the grass
      if (this.prevBallZ > 0.28 && ballZ <= 0.06) {
        this.particles.spawnBurst(ballX, 0.04, ballY, ballVx * 0.15, ballVy * 0.15, 4);
      }
      this.prevBallZ = ballZ;

      this.ballMesh.position.set(ballX, this.ballRadius + Math.max(0, ballZ), ballY);
      const bSpeed = Math.hypot(ballVx, ballVy);
      if (bSpeed > 0.05) {
        const rollAxis = new THREE.Vector3(ballVy, 0, -ballVx).normalize();
        this.ballMesh.rotateOnWorldAxis(rollAxis, (bSpeed * dt) / this.ballRadius);
      }

      this.ballShadow.position.set(ballX, 0.018, ballY);
      const shadowShrink = Math.max(0.42, 1.0 - ballZ * 0.12);
      this.ballShadow.scale.set(shadowShrink, shadowShrink, 1);

      const showHudIndicators = this.viewMode === "match" && !(replayOpts && replayOpts.active);

      for (let n = 0; n < Math.min(pps * 2, this.actors.length); n++) {
        const b = 18 + n * 10;
        const side = packed[b + 0];
        const idx = packed[b + 1];
        const wx = packed[b + 4] / 1000;
        const wy = packed[b + 5] / 1000;
        const vx = packed[b + 6] / 1000;
        const vy = packed[b + 7] / 1000;
        const facingRad = (packed[b + 8] * Math.PI * 2) / 65536;
        const flags = packed[b + 9];
        const action = flags & 0xf;
        const charge = ((flags >> 15) & 0x7ff) / 1000;
        const sentOff = ((flags >> 26) & 1) === 1;

        const actor = this.actors[n];
        if (sentOff) {
          actor.root.visible = false;
          continue;
        }
        actor.root.visible = true;

        const isControlled = showHudIndicators && (side === 0 ? idx === homeCtrl : idx === awayCtrl);
        const isBallCarrier = showHudIndicators && ownerSide === side && ownerIdx === idx;
        const celebrating = phase === 10 && ownerSide === side;

        animateFootballerActor(
          actor,
          {
            wx,
            wy,
            vx,
            vy,
            facingRad,
            action,
            charge,
            isControlled,
            isBallCarrier,
            ballX,
            ballY,
            celebrating,
            tick,
          },
          dt,
          replayOpts && replayOpts.active ? null : this.particles
        );
      }
    }

    // --- Camera Controller ---
    let targetPos;
    let targetLook;
    let desiredFov = 32;

    if (replayOpts && replayOpts.active) {
      desiredFov = 29;
      if (replayOpts.camMode === "broadcast") {
        targetPos = new THREE.Vector3(ballX * 0.65, 13.5, halfW + 14.0);
        targetLook = new THREE.Vector3(ballX * 0.85, 0.85, ballY * 0.45);
      } else if (replayOpts.camMode === "closeup") {
        const ang = replayOpts.orbitAngle;
        targetPos = new THREE.Vector3(ballX + Math.sin(ang) * 6.8, 2.55, ballY + Math.cos(ang) * 6.8);
        targetLook = new THREE.Vector3(ballX, 1.0, ballY);
      } else {
        const r = pitchLen > 80 ? 18.0 : 11.5;
        const ang = replayOpts.orbitAngle;
        targetPos = new THREE.Vector3(ballX + Math.sin(ang) * r, 5.2, ballY + Math.cos(ang) * r);
        targetLook = new THREE.Vector3(ballX, 0.9, ballY);
      }
    } else if (this.viewMode === "attract" || this.viewMode === "squad") {
      this.attractAngle += dt * 0.16;
      desiredFov = 32;
      const radX = halfL * 0.55;
      const eyeX = Math.sin(this.attractAngle) * radX;
      const eyeZ = halfW * 0.88 + 11.5 + Math.cos(this.attractAngle * 0.7) * 3.5;
      targetPos = new THREE.Vector3(eyeX, 11.5, eyeZ);
      targetLook = new THREE.Vector3(ballX * 0.45, 0.9, ballY * 0.25 - 1.5);
    } else {
      const is11 = pitchLen > 80;
      const clampX = halfL * 0.56;
      const leadX = Math.max(-clampX, Math.min(clampX, ballX * 0.76));

      if (this.cameraMode === "dynamic") {
        desiredFov = 29;
        const distZ = is11 ? 23.5 : 18.2;
        const heightY = is11 ? 12.2 : 9.6;
        targetPos = new THREE.Vector3(leadX, heightY, ballY * 0.42 + distZ);
        targetLook = new THREE.Vector3(ballX * 0.86, 0.85, ballY * 0.45 - 0.8);
      } else if (this.cameraMode === "pro") {
        desiredFov = 36;
        targetPos = new THREE.Vector3(ballX - (is11 ? 21 : 15.5), is11 ? 11.5 : 8.6, ballY * 0.65);
        targetLook = new THREE.Vector3(ballX + 9.0, 0.9, ballY * 0.45);
      } else if (this.cameraMode === "wide") {
        desiredFov = 35;
        targetPos = new THREE.Vector3(leadX * 0.5, is11 ? 27.0 : 19.2, is11 ? 43.0 : 30.5);
        targetLook = new THREE.Vector3(leadX * 0.55, 0.5, -1.2);
      } else {
        desiredFov = 33.5;
        const heightY = is11 ? 21.0 : 15.2;
        const distZ = is11 ? 35.5 : 26.8;
        targetPos = new THREE.Vector3(leadX, heightY, ballY * 0.3 + distZ);
        targetLook = new THREE.Vector3(leadX * 1.04, 0.72, ballY * 0.28 - 1.5);
      }
    }

    if (Math.abs(this.camera.fov - desiredFov) > 0.05) {
      this.camera.fov += (desiredFov - this.camera.fov) * Math.min(1, dt * 8);
      this.camera.updateProjectionMatrix();
    }

    const smooth = replayOpts && replayOpts.active ? Math.min(1, dt * 12) : Math.min(1, dt * 7.5);
    this.camPos.lerp(targetPos, smooth);
    this.camLook.lerp(targetLook, smooth);
    this.camera.position.copy(this.camPos);
    this.camera.lookAt(this.camLook);

    this.renderer.render(this.scene, this.camera);
  }
}
