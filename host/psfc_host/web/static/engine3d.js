// ClassicFutbol — EA FIFA 2010–2012 Broadcast Presentation, SkinnedMesh Athletes & Smooth IK Engine
// Architecture:
// 1. Continuous GPU Vertex-Skinned Athletes (THREE.SkinnedMesh + 16-bone THREE.Skeleton)
//    with realistic 1 : 7.56 adult footballer proportions & FIFA 11 Personality+ somatotypes
//    (eliminating rigid mannequin ball-joint seams at shoulders, elbows, hips, shorts & knees).
// 2. Modern WebGL Stack:
//    - maath/easing (damp, damp3, dampAngle, dampQ) for frame-rate-independent SmoothDamp
//      camera framing and shortest-arc quaternion skeletal blending (zero pose popping).
//    - RoomEnvironment + PMREMGenerator physically-based stadium image-based lighting (IBL).
//    - pmndrs/postprocessing single-pass EffectComposer (SMAA + Floodlight Bloom + Vignette + ACES).
//    - simplex-noise multi-octave stadium turf grain/wear & organic non-robotic idle motion.
// 3. Sub-Tick State Interpolation (prevPacked -> currPacked by alpha) + non-skating run-through
//    pass/shot windup + flush-sole 3D cleats + Tango 12 thermally-bonded match ball spin.
// 4. Zoomed-out EA FIFA 10–12 Tele Broadcast & Co-Op gantry cameras + live user zoom control.

import * as THREE from "./vendor/three.module.js";
import {
  BloomEffect,
  EffectComposer,
  EffectPass,
  RenderPass,
  RoomEnvironment,
  SMAAEffect,
  ToneMappingEffect,
  ToneMappingMode,
  VignetteEffect,
  createNoise2D,
  damp,
  damp3,
  dampAngle,
  dampQ,
} from "./vendor/webgl-extras.module.js";

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

// FIFA 11 "Personality+" Likeness & Somatotype Builds ("lean" | "athletic" | "power")
const PLAYER_LIKENESS = {
  "mun-gk-1": { skin: 5, hair: 0, style: "fade", beard: true, build: "power" }, // Onana
  "mun-def-1": { skin: 1, hair: 1, style: "crop", beard: true, build: "athletic" }, // Martinez
  "mun-def-2": { skin: 1, hair: 1, style: "fade", beard: true, build: "athletic" },
  "mun-def-3": { skin: 0, hair: 2, style: "swept", beard: false, build: "power" },
  "mun-def-4": { skin: 1, hair: 2, style: "crop", beard: true, build: "athletic" },
  "mun-mid-1": { skin: 1, hair: 1, style: "crop", beard: true, build: "lean" }, // Bruno
  "mun-mid-2": { skin: 4, hair: 0, style: "curls", beard: false, build: "athletic" }, // Mainoo
  "mun-mid-3": { skin: 2, hair: 0, style: "crop", beard: false, build: "power" }, // Casemiro
  "mun-fwd-1": { skin: 4, hair: 0, style: "fade", beard: true, build: "lean" }, // Rashford
  "mun-fwd-2": { skin: 0, hair: 4, style: "swept", beard: false, build: "lean" }, // Garnacho
  "mun-fwd-3": { skin: 0, hair: 4, style: "swept", beard: false, build: "power" }, // Hojlund

  "ars-gk-1": { skin: 1, hair: 1, style: "fade", beard: true, build: "athletic" }, // Raya
  "ars-def-1": { skin: 4, hair: 0, style: "buzz", beard: false, build: "power" }, // Saliba
  "ars-def-2": { skin: 3, hair: 0, style: "crop", beard: true, build: "power" }, // Gabriel
  "ars-def-3": { skin: 1, hair: 2, style: "swept", beard: true, build: "athletic" }, // White
  "ars-def-4": { skin: 3, hair: 0, style: "curls", beard: false, build: "athletic" }, // Timber
  "ars-mid-1": { skin: 0, hair: 4, style: "swept", beard: true, build: "lean" }, // Odegaard
  "ars-mid-2": { skin: 0, hair: 2, style: "swept", beard: false, build: "power" }, // Rice
  "ars-mid-3": { skin: 4, hair: 0, style: "fade", beard: true, build: "power" }, // Partey
  "ars-fwd-1": { skin: 5, hair: 0, style: "curls", beard: false, build: "lean" }, // Saka
  "ars-fwd-2": { skin: 0, hair: 2, style: "swept", beard: false, build: "athletic" }, // Havertz
  "ars-fwd-3": { skin: 1, hair: 1, style: "crop", beard: false, build: "lean" }, // Martinelli

  "liv-gk-1": { skin: 1, hair: 2, style: "swept", beard: true, build: "power" }, // Alisson
  "liv-def-1": { skin: 3, hair: 0, style: "bun", beard: true, build: "power" }, // Van Dijk
  "liv-def-2": { skin: 4, hair: 0, style: "curls", beard: false, build: "athletic" }, // Trent
  "liv-def-3": { skin: 4, hair: 0, style: "fade", beard: true, build: "power" }, // Konate
  "liv-def-4": { skin: 0, hair: 2, style: "crop", beard: false, build: "lean" }, // Robertson
  "liv-mid-1": { skin: 1, hair: 1, style: "swept", beard: true, build: "athletic" }, // Mac Allister
  "liv-mid-2": { skin: 4, hair: 0, style: "curls", beard: false, build: "athletic" }, // Gravenberch
  "liv-mid-3": { skin: 0, hair: 1, style: "swept", beard: true, build: "athletic" }, // Szoboszlai
  "liv-fwd-1": { skin: 2, hair: 0, style: "curls", beard: true, build: "lean" }, // Salah
  "liv-fwd-2": { skin: 2, hair: 0, style: "fade", beard: false, build: "lean" }, // Diaz
  "liv-fwd-3": { skin: 1, hair: 1, style: "bun", beard: true, build: "power" }, // Nunez

  "che-gk-1": { skin: 2, hair: 0, style: "fade", beard: true, build: "power" },
  "che-def-1": { skin: 3, hair: 0, style: "curls", beard: false, build: "athletic" },
  "che-def-2": { skin: 4, hair: 0, style: "curls", beard: true, build: "power" },
  "che-def-3": { skin: 1, hair: 1, style: "curls", beard: true, build: "athletic" },
  "che-def-4": { skin: 5, hair: 0, style: "fade", beard: false, build: "power" },
  "che-mid-1": { skin: 0, hair: 3, style: "crop", beard: false, build: "lean" }, // Palmer
  "che-mid-2": { skin: 4, hair: 0, style: "fade", beard: false, build: "power" }, // Caicedo
  "che-mid-3": { skin: 1, hair: 1, style: "fade", beard: true, build: "athletic" },
  "che-fwd-1": { skin: 5, hair: 0, style: "fade", beard: false, build: "athletic" },
  "che-fwd-2": { skin: 4, hair: 0, style: "curls", beard: true, build: "lean" },
  "che-fwd-3": { skin: 1, hair: 2, style: "swept", beard: false, build: "lean" },
};

const SOMATOTYPE_SCALES = {
  lean: { w: 0.96, h: 0.995, d: 0.95 },
  athletic: { w: 1.0, h: 1.01, d: 1.0 },
  power: { w: 1.055, h: 1.035, d: 1.045 },
};

// Deterministic seeded 2D Simplex Noise instance for turf micro-texture & organic idle motion
let _noiseSeed = 0x2011f1fa;
const seededNoise2D = createNoise2D(() => {
  _noiseSeed = (Math.imul(_noiseSeed, 1664525) + 1013904223) >>> 0;
  return _noiseSeed / 4294967296;
});

// Track shared singleton GPU resources so scene rebuilds never dispose them
const RETAINED_GPU = new WeakSet();
function retainGpu(res) {
  if (res) RETAINED_GPU.add(res);
  return res;
}

function disposeObjectTree(root) {
  if (!root) return;
  const disposedSkeletons = new Set();
  root.traverse((obj) => {
    if (obj.isSkinnedMesh && obj.skeleton && !disposedSkeletons.has(obj.skeleton)) {
      disposedSkeletons.add(obj.skeleton);
      obj.skeleton.dispose();
    }
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

function smoothstep(edge0, edge1, x) {
  const t = Math.max(0, Math.min(1, (x - edge0) / (edge1 - edge0)));
  return t * t * (3 - 2 * t);
}

function wrapAngleRad(a) {
  let r = a;
  while (r > Math.PI) r -= Math.PI * 2;
  while (r < -Math.PI) r += Math.PI * 2;
  return r;
}

function lerpAngleRad(a, b, t) {
  return a + wrapAngleRad(b - a) * t;
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
// 1. SKELETON TOPOLOGY & ANALYTICAL TWO-BONE IK WITH QUATERNION SMOOTHDAMP
// ============================================================================

// 1 : 7.56 Real Adult Footballer Anthropometry (Total stature ~1.785m before somatotype scale)
const SOLE_HEIGHT = 0.062;
const LOWER_LEG_LEN = 0.425; // Ankle at y = 0.062 -> Knee at y = 0.487
const UPPER_LEG_LEN = 0.435; // Knee at y = 0.487 -> Hip socket at y = 0.922
const HIPS_BIND_Y = 0.945;
const HIP_SOCKET_L = new THREE.Vector3(0.096, -0.023, 0);
const HIP_SOCKET_R = new THREE.Vector3(-0.096, -0.023, 0);

// Bone indices in the 16-bone humanoid skeleton
const B_ROOT = 0;
const B_HIPS = 1;
const B_SPINE = 2;
const B_CHEST = 3;
const B_NECK = 4;
const B_HEAD = 5;
const B_SH_L = 6;
const B_EL_L = 7;
const B_SH_R = 8;
const B_EL_R = 9;
const B_HIP_L = 10;
const B_KNEE_L = 11;
const B_ANKLE_L = 12;
const B_HIP_R = 13;
const B_KNEE_R = 14;
const B_ANKLE_R = 15;

// Reusable scratch math objects (zero GC churn per frame)
const _vDown = new THREE.Vector3(0, -1, 0);
const _vHipPos = new THREE.Vector3();
const _vTargetOff = new THREE.Vector3();
const _vToTarget = new THREE.Vector3();
const _vPole = new THREE.Vector3();
const _vKneePos = new THREE.Vector3();
const _vEffTarget = new THREE.Vector3();
const _vThighDir = new THREE.Vector3();
const _vShinDir = new THREE.Vector3();
const _qPelvisInv = new THREE.Quaternion();
const _qHipGoal = new THREE.Quaternion();
const _qHipInv = new THREE.Quaternion();
const _qKneeGoal = new THREE.Quaternion();
const _qKneeInv = new THREE.Quaternion();
const _qAnkleGoal = new THREE.Quaternion();
const _qFootPitch = new THREE.Quaternion();
const _qTmp = new THREE.Quaternion();
const _eulerTmp = new THREE.Euler();

/**
 * Solves Analytical Law-of-Cosines Two-Bone Leg IK and applies shortest-arc
 * critically-damped quaternion smoothing (`maath.dampQ`) to eliminate pose snaps.
 */
function solveAndDampLegIK(
  hipsBone,
  hipBone,
  kneeBone,
  ankleBone,
  hipSocketLocal,
  targetPos,
  poleDir,
  footPitchRad,
  smoothTime,
  dt
) {
  _vHipPos.copy(hipSocketLocal).applyQuaternion(hipsBone.quaternion).add(hipsBone.position);

  _vTargetOff.subVectors(targetPos, _vHipPos);
  const rawDist = _vTargetOff.length();
  const maxReach = UPPER_LEG_LEN + LOWER_LEG_LEN - 0.004;
  const clampedDist = Math.max(0.2, Math.min(rawDist, maxReach));

  if (rawDist > 1e-5) {
    _vToTarget.copy(_vTargetOff).divideScalar(rawDist);
  } else {
    _vToTarget.set(0, -1, 0);
  }

  // Orthogonalize knee pole vector (+Z forward) against hip->ankle axis
  _vPole.copy(poleDir).addScaledVector(_vToTarget, -poleDir.dot(_vToTarget));
  if (_vPole.lengthSq() < 1e-6) {
    _vPole.set(0, 0, 1).addScaledVector(_vToTarget, -_vToTarget.z);
  }
  _vPole.normalize();

  const cosAlpha = Math.max(
    -1,
    Math.min(
      1,
      (clampedDist * clampedDist + UPPER_LEG_LEN * UPPER_LEG_LEN - LOWER_LEG_LEN * LOWER_LEG_LEN) /
        (2 * UPPER_LEG_LEN * clampedDist)
    )
  );
  const sinAlpha = Math.sqrt(Math.max(0, 1 - cosAlpha * cosAlpha));

  _vKneePos
    .copy(_vHipPos)
    .addScaledVector(_vToTarget, UPPER_LEG_LEN * cosAlpha)
    .addScaledVector(_vPole, UPPER_LEG_LEN * sinAlpha);
  _vEffTarget.copy(_vHipPos).addScaledVector(_vToTarget, clampedDist);

  _qPelvisInv.copy(hipsBone.quaternion).invert();
  _vThighDir.subVectors(_vKneePos, _vHipPos).normalize().applyQuaternion(_qPelvisInv);
  _qHipGoal.setFromUnitVectors(_vDown, _vThighDir);

  _qHipInv.copy(_qHipGoal).invert();
  _vShinDir.subVectors(_vEffTarget, _vKneePos).normalize().applyQuaternion(_qPelvisInv).applyQuaternion(_qHipInv);
  _qKneeGoal.setFromUnitVectors(_vDown, _vShinDir);

  _qKneeInv.copy(_qKneeGoal).invert();
  _eulerTmp.set(footPitchRad, 0, 0);
  _qFootPitch.setFromEuler(_eulerTmp);
  _qAnkleGoal.copy(_qKneeInv).multiply(_qHipInv).multiply(_qPelvisInv).multiply(_qFootPitch);

  if (smoothTime <= 0.001) {
    hipBone.quaternion.copy(_qHipGoal);
    kneeBone.quaternion.copy(_qKneeGoal);
    ankleBone.quaternion.copy(_qAnkleGoal);
  } else {
    dampQ(hipBone.quaternion, _qHipGoal, smoothTime, dt);
    dampQ(kneeBone.quaternion, _qKneeGoal, smoothTime, dt);
    dampQ(ankleBone.quaternion, _qAnkleGoal, smoothTime, dt);
  }
}

function dampBoneEuler(bone, rx, ry, rz, smoothTime, dt) {
  _eulerTmp.set(rx, ry, rz);
  _qTmp.setFromEuler(_eulerTmp);
  if (smoothTime <= 0.001) {
    bone.quaternion.copy(_qTmp);
  } else {
    dampQ(bone.quaternion, _qTmp, smoothTime, dt);
  }
}

// ============================================================================
// 2. PROCEDURAL SKINNED-MESH HUMAN ANATOMY BUILDERS (1 : 7.56 FIFA 10–12 RATIO)
// ============================================================================

/**
 * Continuous 11-ring lofted V-taper jersey torso bound across Hips, Spine, Chest & Shoulders.
 * Authored in bind-pose rig space (y = 0.915 waist tuck -> y = 1.485 collar).
 */
function createSkinnedTorsoGeometry() {
  const rings = [
    [0.915, 0.146, 0.094, 0.0],
    [0.965, 0.142, 0.092, 0.001], // Lean athletic waist
    [1.035, 0.148, 0.097, 0.003],
    [1.115, 0.160, 0.104, 0.006], // Oblique taper
    [1.195, 0.175, 0.112, 0.009], // Lower latissimus flare
    [1.275, 0.188, 0.118, 0.011], // Pectoral fullness
    [1.345, 0.196, 0.114, 0.006], // Broad athletic chest/shoulder line
    [1.405, 0.184, 0.102, 0.0],
    [1.448, 0.132, 0.082, -0.003], // Sloped trapezius
    [1.482, 0.074, 0.066, -0.004], // Neck collar opening
  ];

  const radialSegs = 28;
  const positions = [];
  const uvs = [];
  const skinIndices = [];
  const skinWeights = [];
  const frontIndices = [];
  const backIndices = [];

  for (let r = 0; r < rings.length; r++) {
    const [y, rx, rz, zShift] = rings[r];
    const vCoord = Math.min(1, Math.max(0, (y - 0.915) / (1.482 - 0.915)));

    // Smooth 3-bone spinal weighting (Hips -> Spine -> Chest)
    let wHips = 0;
    let wSpine = 0;
    let wChest = 0;
    if (y < 1.085) {
      const t = smoothstep(0.915, 1.085, y);
      wHips = 1 - t;
      wSpine = t;
    } else if (y < 1.29) {
      const t = smoothstep(1.085, 1.29, y);
      wSpine = 1 - t;
      wChest = t;
    } else {
      wChest = 1.0;
    }

    for (let s = 0; s <= radialSegs; s++) {
      const theta = (s / radialSegs) * Math.PI * 2;
      const cosT = Math.cos(theta);
      const sinT = Math.sin(theta);

      const x = cosT * rx;
      let z = sinT * rz + zShift;
      if (sinT > 0.25 && y > 1.20 && y < 1.38) {
        z += 0.007 * Math.sin(((y - 1.20) / 0.18) * Math.PI) * (Math.abs(cosT) < 0.82 ? 1 : 0.4);
      }
      positions.push(x, y, z);

      const uFront = 0.5 + (x / 0.205) * 0.46;
      const uBack = 0.5 - (x / 0.205) * 0.46;
      uvs.push(sinT >= 0 ? uFront : uBack, vCoord);

      // Blend lateral shoulder vertices subtly with ShoulderL/R so jersey moves with arm swing
      let shIdx = 0;
      let wSh = 0;
      if (y > 1.30 && Math.abs(x) > 0.12) {
        shIdx = x >= 0 ? B_SH_L : B_SH_R;
        wSh = 0.22 * smoothstep(0.12, 0.195, Math.abs(x)) * smoothstep(1.30, 1.41, y);
      }
      const rem = 1.0 - wSh;
      skinIndices.push(B_HIPS, B_SPINE, B_CHEST, shIdx);
      skinWeights.push(wHips * rem, wSpine * rem, wChest * rem, wSh);
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
  geo.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
  geo.setAttribute("uv", new THREE.Float32BufferAttribute(uvs, 2));
  geo.setAttribute("skinIndex", new THREE.Uint16BufferAttribute(skinIndices, 4));
  geo.setAttribute("skinWeight", new THREE.Float32BufferAttribute(skinWeights, 4));
  geo.setIndex([...frontIndices, ...backIndices]);
  geo.clearGroups();
  geo.addGroup(0, frontIndices.length, 0);
  geo.addGroup(frontIndices.length, backIndices.length, 1);
  geo.computeVertexNormals();
  return geo;
}

/**
 * Helper to append a closed bottom-to-top skinned cylindrical limb loft with material groups.
 */
function appendSkinnedTube(acc, rings, radialSegs, weightFn) {
  const baseVertex = acc.positions.length / 3;
  for (let r = 0; r < rings.length; r++) {
    const { cx, y, cz, rx, rz } = rings[r];
    const [bi0, bw0, bi1, bw1] = weightFn(y);
    for (let s = 0; s <= radialSegs; s++) {
      const theta = (s / radialSegs) * Math.PI * 2;
      const x = cx + Math.cos(theta) * rx;
      const z = cz + Math.sin(theta) * rz;
      acc.positions.push(x, y, z);
      acc.uvs.push(s / radialSegs, r / Math.max(1, rings.length - 1));
      acc.skinIndices.push(bi0, bi1, 0, 0);
      acc.skinWeights.push(bw0, bw1, 0, 0);
    }
  }

  const stride = radialSegs + 1;
  for (let r = 0; r < rings.length - 1; r++) {
    const matSlot = rings[r].mat;
    const target = acc.groupIndices[matSlot];
    for (let s = 0; s < radialSegs; s++) {
      const a = baseVertex + r * stride + s;
      const b = a + 1;
      const c = baseVertex + (r + 1) * stride + s;
      const d = c + 1;
      // Bottom-to-top increasing y -> (a, d, b), (a, c, d) yields outward normals!
      target.push(a, d, b, a, c, d);
    }
  }
}

function finalizeGroupedSkinnedGeometry(acc) {
  const geo = new THREE.BufferGeometry();
  const flatIndices = [];
  geo.clearGroups();
  for (let m = 0; m < acc.groupIndices.length; m++) {
    const arr = acc.groupIndices[m];
    if (arr.length > 0) {
      geo.addGroup(flatIndices.length, arr.length, m);
      for (let i = 0; i < arr.length; i++) flatIndices.push(arr[i]);
    }
  }
  geo.setAttribute("position", new THREE.Float32BufferAttribute(acc.positions, 3));
  geo.setAttribute("uv", new THREE.Float32BufferAttribute(acc.uvs, 2));
  geo.setAttribute("skinIndex", new THREE.Uint16BufferAttribute(acc.skinIndices, 4));
  geo.setAttribute("skinWeight", new THREE.Float32BufferAttribute(acc.skinWeights, 4));
  geo.setIndex(flatIndices);
  geo.computeVertexNormals();
  return geo;
}

/**
 * Continuous Skinned Arms (Left + Right) seamlessly weighted across Chest -> Shoulder -> Elbow.
 * Material groups: 0 = sleeveMat, 1 = trimMat (cuff), 2 = skinMat (or GK long sleeve).
 */
function createSkinnedArmsGeometry(isGk) {
  const acc = {
    positions: [],
    uvs: [],
    skinIndices: [],
    skinWeights: [],
    groupIndices: [[], [], []],
  };
  const forearmMat = isGk ? 0 : 2;

  for (const sideSign of [1, -1]) {
    const cx = sideSign * 0.188;
    const bSh = sideSign > 0 ? B_SH_L : B_SH_R;
    const bEl = sideSign > 0 ? B_EL_L : B_EL_R;

    // Ordered strictly bottom-to-top (wrist y = 0.91 -> shoulder dome y = 1.458)
    const rings = [
      { cx, y: 0.912, cz: 0, rx: 0.002, rz: 0.002, mat: forearmMat },
      { cx, y: 0.922, cz: 0, rx: 0.036, rz: 0.033, mat: forearmMat }, // Wrist
      { cx, y: 1.020, cz: 0, rx: 0.046, rz: 0.044, mat: forearmMat },
      { cx, y: 1.115, cz: 0, rx: 0.052, rz: 0.050, mat: forearmMat }, // Brachioradialis swell
      { cx, y: 1.155, cz: -0.002, rx: 0.051, rz: 0.051, mat: forearmMat }, // Seamless skinned elbow!
      { cx, y: 1.198, cz: 0, rx: 0.054, rz: 0.053, mat: 1 }, // Jersey sleeve cuff trim
      { cx, y: 1.222, cz: 0, rx: 0.059, rz: 0.058, mat: 0 },
      { cx, y: 1.310, cz: 0, rx: 0.064, rz: 0.063, mat: 0 }, // Bicep / tricep sleeve
      { cx: sideSign * 0.182, y: 1.395, cz: 0, rx: 0.067, rz: 0.065, mat: 0 }, // Deltoid contour
      { cx: sideSign * 0.156, y: 1.442, cz: 0, rx: 0.052, rz: 0.052, mat: 0 }, // Acromion blend into chest!
      { cx: sideSign * 0.142, y: 1.454, cz: 0, rx: 0.002, rz: 0.002, mat: 0 },
    ];

    appendSkinnedTube(acc, rings, 14, (y) => {
      if (y < 1.115) {
        return [bEl, 1.0, bSh, 0.0];
      }
      if (y < 1.195) {
        // Smooth skinning across elbow flexion crease (y = 1.155)
        const t = smoothstep(1.115, 1.195, y);
        return [bEl, 1.0 - t, bSh, t];
      }
      if (y < 1.385) {
        return [bSh, 1.0, B_CHEST, 0.0];
      }
      // Smooth acromion skinning into thoracic chest so shoulder never detaches!
      const t = smoothstep(1.385, 1.454, y) * 0.55;
      return [bSh, 1.0 - t, B_CHEST, t];
    });
  }

  return finalizeGroupedSkinnedGeometry(acc);
}

/**
 * Continuous Skinned Lower Body: Pelvis Shorts + Flared Short Legs + Muscular Quads +
 * Seamless Skinned Knees + Sock Turnover Bands + Gastrocnemius Calves & Achilles Socks!
 * Material groups: 0 = shortsMat, 1 = skinMat, 2 = sockBandMat, 3 = sockMat.
 */
function createSkinnedLowerBodyGeometry() {
  const acc = {
    positions: [],
    uvs: [],
    skinIndices: [],
    skinWeights: [],
    groupIndices: [[], [], [], []],
  };

  // 1. Sculpted Pelvis / Glute Shorts Block (y = 0.835 -> 0.965)
  const pelvisRings = [
    { cx: 0, y: 0.835, cz: -0.004, rx: 0.002, rz: 0.002, mat: 0 },
    { cx: 0, y: 0.845, cz: -0.004, rx: 0.148, rz: 0.096, mat: 0 },
    { cx: 0, y: 0.895, cz: -0.006, rx: 0.162, rz: 0.108, mat: 0 },
    { cx: 0, y: 0.938, cz: -0.002, rx: 0.152, rz: 0.099, mat: 0 },
    { cx: 0, y: 0.962, cz: 0, rx: 0.144, rz: 0.093, mat: 0 },
    { cx: 0, y: 0.966, cz: 0, rx: 0.002, rz: 0.002, mat: 0 },
  ];
  appendSkinnedTube(acc, pelvisRings, 20, (y) => {
    const t = smoothstep(0.92, 0.966, y) * 0.25;
    return [B_HIPS, 1.0 - t, B_SPINE, t];
  });

  // 2. Left & Right Continuous Skinned Legs (Ankle y = 0.062 -> Hip y = 0.938)
  for (const sideSign of [1, -1]) {
    const cx = sideSign * 0.096;
    const bHip = sideSign > 0 ? B_HIP_L : B_HIP_R;
    const bKnee = sideSign > 0 ? B_KNEE_L : B_KNEE_R;

    // A. Continuous Skin + Sock Leg Tube (Ankle y = 0.062 -> Upper Thigh y = 0.785 inside shorts)
    const legRings = [
      { cx, y: 0.060, cz: 0, rx: 0.002, rz: 0.002, mat: 3 },
      { cx, y: 0.065, cz: 0, rx: 0.046, rz: 0.049, mat: 3 }, // Ankle collar into cleat
      { cx, y: 0.140, cz: -0.003, rx: 0.045, rz: 0.048, mat: 3 }, // Tapered Achilles tendon
      { cx, y: 0.235, cz: -0.006, rx: 0.058, rz: 0.062, mat: 3 },
      { cx, y: 0.340, cz: -0.010, rx: 0.072, rz: 0.078, mat: 3 }, // Gastrocnemius calf belly!
      { cx, y: 0.412, cz: -0.004, rx: 0.066, rz: 0.069, mat: 2 }, // Sock turnover band start
      { cx, y: 0.452, cz: 0, rx: 0.064, rz: 0.066, mat: 1 }, // Below patella skin
      { cx, y: 0.487, cz: 0.005, rx: 0.065, rz: 0.068, mat: 1 }, // Seamless skinned knee joint!
      { cx, y: 0.535, cz: 0.004, rx: 0.071, rz: 0.074, mat: 1 }, // Lower vastus medialis
      { cx, y: 0.625, cz: 0.004, rx: 0.081, rz: 0.085, mat: 1 }, // Sculpted quadriceps sweep
      { cx, y: 0.735, cz: 0, rx: 0.086, rz: 0.089, mat: 1 },
      { cx, y: 0.775, cz: 0, rx: 0.002, rz: 0.002, mat: 1 },
    ];

    appendSkinnedTube(acc, legRings, 16, (y) => {
      if (y < 0.440) {
        return [bKnee, 1.0, bHip, 0.0];
      }
      if (y < 0.535) {
        // Smooth vertex skinning across knee flexion (y = 0.487) -> zero ball-joint seam!
        const t = smoothstep(0.440, 0.535, y);
        return [bKnee, 1.0 - t, bHip, t];
      }
      return [bHip, 1.0, B_HIPS, 0.0];
    });

    // B. Closed Solid Flared Shorts Leg (Hem y = 0.700 -> Hip y = 0.938)
    const shortLegRings = [
      { cx, y: 0.702, cz: 0, rx: 0.002, rz: 0.002, mat: 0 },
      { cx, y: 0.700, cz: 0, rx: 0.082, rz: 0.085, mat: 0 }, // Clean inner hem tuck against quad
      { cx, y: 0.704, cz: 0, rx: 0.102, rz: 0.106, mat: 0 }, // Athletic fabric flare hem
      { cx, y: 0.815, cz: -0.002, rx: 0.097, rz: 0.101, mat: 0 },
      { cx, y: 0.925, cz: -0.003, rx: 0.092, rz: 0.096, mat: 0 },
      { cx, y: 0.935, cz: 0, rx: 0.002, rz: 0.002, mat: 0 },
    ];

    appendSkinnedTube(acc, shortLegRings, 16, (y) => {
      const pelvisBlend = smoothstep(0.84, 0.935, y) * 0.42;
      return [bHip, 1.0 - pelvisBlend, B_HIPS, pelvisBlend];
    });
  }

  return finalizeGroupedSkinnedGeometry(acc);
}

/**
 * Flush-sole 3D Football Cleat (Codex PR #6 fix: origin at ankle y = 0, sole resting at y = -SOLE_HEIGHT
 * so when ankle bone sits at y = SOLE_HEIGHT = 0.062, the cleat sole & studs rest flush at y = 0.000!).
 */
function createSculptedCleatGeometry() {
  const shape = new THREE.Shape();
  shape.moveTo(-0.042, -0.078);
  shape.quadraticCurveTo(-0.051, 0.02, -0.044, 0.11);
  shape.quadraticCurveTo(-0.032, 0.176, 0.0, 0.186);
  shape.quadraticCurveTo(0.032, 0.176, 0.044, 0.11);
  shape.quadraticCurveTo(0.051, 0.02, 0.042, -0.078);
  shape.quadraticCurveTo(0.0, -0.09, -0.042, -0.078);

  const g = new THREE.ExtrudeGeometry(shape, {
    depth: 0.048,
    bevelEnabled: true,
    bevelSegments: 3,
    steps: 1,
    bevelSize: 0.009,
    bevelThickness: 0.01,
  });
  g.rotateX(Math.PI * 0.5); // After +90 deg X rotation, y spans [-0.058, +0.010]
  const pos = g.attributes.position;
  for (let i = 0; i < pos.count; i++) {
    const y = pos.getY(i);
    const z = pos.getZ(i);
    if (y > -0.028 && z > 0.025) {
      pos.setY(i, y - (z - 0.025) * 0.21);
    }
  }
  g.translate(0, 0.003, 0.012); // Bottom of sole at y = -0.055, studs reach y = -0.062 = -SOLE_HEIGHT!
  g.computeVertexNormals();
  return g;
}

/**
 * Proportionally scaled 1 : 7.56 Human Cranium, Cheekbones, Brow Ridge & Jawline.
 * Uses phiStart = -PI/2 so u = 0.5 maps directly to +Z (front face) and UV seam hides at -Z!
 */
function createSculptedHeadGeometry() {
  const g = new THREE.SphereGeometry(0.106, 28, 22, -Math.PI * 0.5, Math.PI * 2);
  const pos = g.attributes.position;
  for (let i = 0; i < pos.count; i++) {
    let x = pos.getX(i);
    let y = pos.getY(i);
    let z = pos.getZ(i);

    y *= 1.15; // Cranial height ~0.242m -> 1 : 7.5 adult athlete ratio!
    x *= 0.89;

    if (y < 0) {
      const t = Math.min(1, -y / 0.122);
      x *= 1.0 - t * 0.29;
      if (z > 0) {
        z *= 1.0 - t * 0.12;
        if (t > 0.65 && Math.abs(x) < 0.036) {
          z += 0.009 * (t - 0.65);
        }
      } else {
        z *= 1.0 - t * 0.26;
      }
    }

    if (z > 0.045 && y > 0.004 && y < 0.044) {
      z += 0.007 * Math.cos(((y - 0.024) / 0.02) * Math.PI * 0.5);
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

  const cheek = ctx.createRadialGradient(128, 132, 8, 128, 132, 68);
  cheek.addColorStop(0, "rgba(220, 95, 75, 0.11)");
  cheek.addColorStop(1, "rgba(220, 95, 75, 0)");
  ctx.fillStyle = cheek;
  ctx.fillRect(60, 70, 136, 120);

  if (hasBeard) {
    ctx.fillStyle = hairHex;
    ctx.globalAlpha = 0.36;
    ctx.beginPath();
    ctx.ellipse(128, 174, 34, 28, 0, 0.1, Math.PI - 0.1);
    ctx.fill();
    ctx.globalAlpha = 0.45;
    ctx.fillRect(114, 150, 28, 5);
    ctx.globalAlpha = 1.0;
  }

  ctx.fillStyle = hairHex;
  ctx.beginPath();
  ctx.roundRect(105, 108, 15, 4.0, 2);
  ctx.roundRect(136, 108, 15, 4.0, 2);
  ctx.fill();

  for (const ex of [112, 144]) {
    ctx.fillStyle = "#f8fafc";
    ctx.beginPath();
    ctx.ellipse(ex, 120, 5.8, 3.8, 0, 0, Math.PI * 2);
    ctx.fill();

    ctx.fillStyle = "#291d12";
    ctx.beginPath();
    ctx.arc(ex, 120, 3.2, 0, Math.PI * 2);
    ctx.fill();

    ctx.fillStyle = "#090d16";
    ctx.beginPath();
    ctx.arc(ex, 120, 1.7, 0, Math.PI * 2);
    ctx.fill();

    ctx.fillStyle = "#ffffff";
    ctx.beginPath();
    ctx.arc(ex - 1.0, 118.8, 0.9, 0, Math.PI * 2);
    ctx.fill();
  }

  ctx.fillStyle = "rgba(0, 0, 0, 0.16)";
  ctx.beginPath();
  ctx.ellipse(128, 141, 4.8, 2.2, 0, 0, Math.PI * 2);
  ctx.fill();

  ctx.strokeStyle = "rgba(65, 25, 20, 0.48)";
  ctx.lineWidth = 2.4;
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
  const skinnedTorso = retainGpu(createSkinnedTorsoGeometry());
  const skinnedArmsOutfield = retainGpu(createSkinnedArmsGeometry(false));
  const skinnedArmsGk = retainGpu(createSkinnedArmsGeometry(true));
  const skinnedLowerBody = retainGpu(createSkinnedLowerBodyGeometry());

  const cleat = retainGpu(createSculptedCleatGeometry());
  const bootStripe = retainGpu(new THREE.BoxGeometry(0.092, 0.022, 0.084));
  const stud = retainGpu(new THREE.CylinderGeometry(0.007, 0.005, 0.014, 6));

  const hand = retainGpu(new THREE.SphereGeometry(0.04, 10, 8));
  hand.scale(0.84, 1.18, 0.66);
  const gkGlove = retainGpu(new THREE.SphereGeometry(0.06, 12, 10));
  gkGlove.scale(0.94, 1.28, 0.62);

  const neck = retainGpu(new THREE.CylinderGeometry(0.052, 0.064, 0.095, 12));
  const collar = retainGpu(new THREE.TorusGeometry(0.073, 0.015, 8, 18));
  collar.rotateX(Math.PI * 0.5);
  const head = retainGpu(createSculptedHeadGeometry());
  const nose = retainGpu(new THREE.SphereGeometry(0.016, 8, 8));
  nose.scale(0.76, 1.05, 1.15);
  const ear = retainGpu(new THREE.SphereGeometry(0.021, 8, 6));
  ear.scale(0.45, 1.05, 0.68);

  const ring = retainGpu(new THREE.RingGeometry(0.56, 0.72, 36));
  ring.rotateX(-Math.PI * 0.5);
  const chargeArc = retainGpu(new THREE.RingGeometry(0.74, 0.88, 32));
  chargeArc.rotateX(-Math.PI * 0.5);
  const aimArrow = retainGpu(new THREE.ConeGeometry(0.15, 0.30, 4));
  aimArrow.rotateX(Math.PI * 0.5);
  const cursorCone = retainGpu(new THREE.ConeGeometry(0.14, 0.24, 4));
  cursorCone.rotateX(Math.PI);
  const shadowDisc = retainGpu(new THREE.CircleGeometry(0.48, 20));
  shadowDisc.rotateX(-Math.PI * 0.5);

  return {
    skinnedTorso,
    skinnedArmsOutfield,
    skinnedArmsGk,
    skinnedLowerBody,
    cleat,
    bootStripe,
    stud,
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

// ============================================================================
// 3. STADIUM, SIMPLEX-NOISE TURF & TANGO 12 MATCH BALL TEXTURE BUILDERS
// ============================================================================

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

  // Classic EA FIFA 10–12 lush emerald lawn-mower stripes
  const stripeCount = pitchLen > 80 ? 18 : 12;
  const stripeM = pitchLen / stripeCount;
  for (let i = 0; i < stripeCount; i++) {
    const x0 = pxX(-halfL + i * stripeM);
    const x1 = pxX(-halfL + (i + 1) * stripeM);
    ctx.fillStyle = i % 2 === 0 ? "#1f803d" : "#269448";
    ctx.fillRect(x0, pTop, x1 - x0 + 0.8, pH);
  }

  const crossRows = pitchWid > 50 ? 10 : 6;
  const rowH = pH / crossRows;
  for (let r = 0; r < crossRows; r++) {
    if (r % 2 === 0) {
      ctx.fillStyle = "rgba(255, 255, 255, 0.028)";
      ctx.fillRect(pLeft, pTop + r * rowH, pW, rowH);
    }
  }

  // Multi-octave Simplex Noise organic turf variation (eliminates flat digital green!)
  const cell = 16;
  for (let py = 0; py < H; py += cell) {
    for (let px = 0; px < W; px += cell) {
      const n1 = seededNoise2D(px * 0.006, py * 0.006);
      const n2 = seededNoise2D(px * 0.024 + 19.3, py * 0.024 - 7.1) * 0.5;
      const v = n1 + n2;
      if (Math.abs(v) > 0.15) {
        ctx.fillStyle =
          v > 0
            ? `rgba(187, 247, 208, ${(v * 0.032).toFixed(3)})`
            : `rgba(5, 46, 22, ${(-v * 0.045).toFixed(3)})`;
        ctx.fillRect(px, py, cell, cell);
      }
    }
  }

  // Goal-mouth & center-circle turf wear
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

  const linePx = Math.max(4.4, scaleX * 0.135);
  ctx.strokeStyle = "rgba(248, 250, 252, 0.95)";
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
  ctx.strokeStyle = "rgba(241, 245, 249, 0.78)";
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

/**
 * 2010–2012 Era Thermally-Bonded Match Ball Texture (Tango 12 / Jabulani inspired).
 * Bold curved triad rings + pentagon anchors make topspin, backspin, and curl readable from Tele cam!
 */
function createMatchBallTexture() {
  const W = 512;
  const H = 256;
  const c = document.createElement("canvas");
  c.width = W;
  c.height = H;
  const ctx = c.getContext("2d");

  ctx.fillStyle = "#f8fafc";
  ctx.fillRect(0, 0, W, H);

  // High-contrast 2010–2012 Tango/Jabulani thermal triad arcs so ball rotation pops visually
  const triads = [
    [0.125, 0.28],
    [0.375, 0.24],
    [0.625, 0.28],
    [0.875, 0.24],
    [0.25, 0.52],
    [0.5, 0.5],
    [0.75, 0.52],
    [0.0, 0.5],
    [0.125, 0.76],
    [0.375, 0.74],
    [0.625, 0.76],
    [0.875, 0.74],
  ];

  // Gold & crimson championship accent rings
  ctx.strokeStyle = "#d97706";
  ctx.lineWidth = 5;
  for (let i = 0; i < triads.length; i += 2) {
    ctx.beginPath();
    ctx.arc(triads[i][0] * W, triads[i][1] * H, 36, 0, Math.PI * 2);
    ctx.stroke();
  }

  ctx.strokeStyle = "rgba(15, 23, 42, 0.88)";
  ctx.lineWidth = 6.5;
  for (let i = 0; i < triads.length; i++) {
    for (let j = i + 1; j < triads.length; j++) {
      const dx = (triads[i][0] - triads[j][0]) * W;
      const dy = (triads[i][1] - triads[j][1]) * H;
      if (Math.hypot(dx, dy) < 142) {
        ctx.beginPath();
        ctx.moveTo(triads[i][0] * W, triads[i][1] * H);
        ctx.quadraticCurveTo(
          ((triads[i][0] + triads[j][0]) * 0.5) * W,
          ((triads[i][1] + triads[j][1]) * 0.5) * H - 10,
          triads[j][0] * W,
          triads[j][1] * H
        );
        ctx.stroke();
      }
    }
  }

  for (const [u, v] of triads) {
    const cx = u * W;
    const cy = v * H;
    ctx.fillStyle = "#0f172a";
    ctx.beginPath();
    for (let k = 0; k < 5; k++) {
      const ang = -Math.PI * 0.5 + (k * Math.PI * 2) / 5;
      const px = cx + Math.cos(ang) * 22;
      const py = cy + Math.sin(ang) * 20;
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
    { bg: "#0f172a", accent: "#facc15", text: "CLASSIC FUTBOL 12", sub: "60HZ DETERMINISTIC ENGINE" },
    { bg: awayColor || "#1d4ed8", accent: "#ffffff", text: (awayName || "AWAY").toUpperCase(), sub: awayBanner },
    { bg: "#064e3b", accent: "#34d399", text: "5V5 ARENA & STADIUM", sub: "EVERY RESULT MATTERS" },
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

    const shade = ctx.createLinearGradient(0, 0, 256, 0);
    shade.addColorStop(0, "rgba(0,0,0,0.22)");
    shade.addColorStop(0.18, "rgba(0,0,0,0)");
    shade.addColorStop(0.5, "rgba(255,255,255,0.06)");
    shade.addColorStop(0.82, "rgba(0,0,0,0)");
    shade.addColorStop(1, "rgba(0,0,0,0.22)");
    ctx.fillStyle = shade;
    ctx.fillRect(0, 0, 256, 256);

    ctx.fillStyle = kit.trim;
    if (!isBack) {
      ctx.beginPath();
      ctx.moveTo(86, 0);
      ctx.lineTo(128, 36);
      ctx.lineTo(170, 0);
      ctx.lineTo(156, 0);
      ctx.lineTo(128, 22);
      ctx.lineTo(100, 0);
      ctx.closePath();
      ctx.fill();

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
    roughness: 0.78,
    metalness: 0.04,
  });
  const group = new THREE.Group();

  // Scaled proportionally for the 1 : 7.56 head (radius 0.106)
  if (style === "buzz") {
    const cap = new THREE.SphereGeometry(0.109, 16, 12, 0, Math.PI * 2, 0, Math.PI * 0.56);
    cap.scale(0.9, 1.14, 0.99);
    const m = new THREE.Mesh(cap, mat);
    m.position.y = 0.01;
    group.add(m);
  } else if (style === "curls") {
    const cap = new THREE.SphereGeometry(0.114, 16, 12, 0, Math.PI * 2, 0, Math.PI * 0.62);
    cap.scale(0.93, 1.15, 1.01);
    const m = new THREE.Mesh(cap, mat);
    m.position.set(0, 0.017, -0.006);
    group.add(m);
    for (let i = 0; i < 10; i++) {
      const puff = new THREE.Mesh(new THREE.SphereGeometry(0.034, 8, 8), mat);
      const ang = (i / 10) * Math.PI * 2;
      puff.position.set(Math.cos(ang) * 0.066, 0.09 + (i % 2) * 0.014, Math.sin(ang) * 0.066);
      group.add(puff);
    }
  } else if (style === "swept") {
    const cap = new THREE.SphereGeometry(0.113, 16, 12, 0, Math.PI * 2, 0, Math.PI * 0.62);
    cap.scale(0.92, 1.15, 1.03);
    const m = new THREE.Mesh(cap, mat);
    m.position.set(0, 0.016, -0.006);
    group.add(m);
    const wave = new THREE.Mesh(new THREE.SphereGeometry(0.058, 12, 10), mat);
    wave.scale.set(1.25, 0.55, 0.95);
    wave.position.set(0.01, 0.108, 0.036);
    wave.rotation.x = -0.22;
    group.add(wave);
  } else if (style === "bun") {
    const cap = new THREE.SphereGeometry(0.111, 16, 12, 0, Math.PI * 2, 0, Math.PI * 0.6);
    cap.scale(0.91, 1.15, 1.0);
    group.add(new THREE.Mesh(cap, mat));
    const bun = new THREE.Mesh(new THREE.SphereGeometry(0.038, 10, 10), mat);
    bun.position.set(0, 0.066, -0.11);
    group.add(bun);
  } else if (style === "fade") {
    const crown = new THREE.SphereGeometry(0.102, 16, 10, 0, Math.PI * 2, 0, Math.PI * 0.48);
    crown.scale(0.91, 1.08, 0.98);
    const m = new THREE.Mesh(crown, mat);
    m.position.set(0, 0.028, 0);
    group.add(m);
  } else {
    const cap = new THREE.SphereGeometry(0.112, 16, 12, 0, Math.PI * 2, 0, Math.PI * 0.58);
    cap.scale(0.91, 1.15, 1.0);
    const m = new THREE.Mesh(cap, mat);
    m.position.y = 0.012;
    group.add(m);
  }
  return group;
}

function resolveLikeness(footballerId, fallbackSeed, isGk) {
  if (footballerId && PLAYER_LIKENESS[footballerId]) {
    const p = PLAYER_LIKENESS[footballerId];
    return {
      skinHex: SKIN_TONES[p.skin % SKIN_TONES.length],
      hairHex: HAIR_COLORS[p.hair % HAIR_COLORS.length],
      style: p.style,
      beard: Boolean(p.beard),
      build: p.build || (isGk ? "power" : "athletic"),
    };
  }
  const h = hashStr(footballerId || String(fallbackSeed));
  const styles = ["crop", "fade", "curls", "swept", "buzz"];
  const builds = ["lean", "athletic", "power"];
  return {
    skinHex: SKIN_TONES[h % SKIN_TONES.length],
    hairHex: HAIR_COLORS[(h >>> 3) % HAIR_COLORS.length],
    style: styles[(h >>> 6) % styles.length],
    beard: (h & 1) === 1,
    build: isGk ? "power" : builds[(h >>> 9) % builds.length],
  };
}

// ============================================================================
// 4. SKINNED-MESH FOOTBALLER GENERATOR & SMOOTH FIFA 10–12 IK ANIMATOR
// ============================================================================

function createFootballerActor(spec) {
  const { kit, shirtNumber, surname, footballerId, clubShort, isGk, side, scale = 1.06 } = spec;
  const likeness = resolveLikeness(footballerId, shirtNumber + side * 31, isGk);
  const soma = SOMATOTYPE_SCALES[likeness.build] || SOMATOTYPE_SCALES.athletic;

  const skinMat = new THREE.MeshStandardMaterial({ color: likeness.skinHex, roughness: 0.46 });
  const faceTex = createFaceSkinTexture(likeness.skinHex, likeness.hairHex, likeness.beard);
  const headFaceMat = new THREE.MeshStandardMaterial({ map: faceTex, roughness: 0.46 });

  const sleeveMat = new THREE.MeshStandardMaterial({ color: kit.sleeves, roughness: 0.58 });
  const trimMat = new THREE.MeshStandardMaterial({ color: kit.trim, roughness: 0.52 });
  const shortsMat = new THREE.MeshStandardMaterial({ color: kit.shorts, roughness: 0.62 });
  const sockMat = new THREE.MeshStandardMaterial({ color: kit.socks, roughness: 0.68 });
  const sockBandMat = new THREE.MeshStandardMaterial({ color: kit.sockBand, roughness: 0.62 });
  const bootMat = new THREE.MeshStandardMaterial({ color: kit.boots, roughness: 0.24, metalness: 0.18 });
  const bootAccentMat = new THREE.MeshStandardMaterial({ color: kit.bootAccent, roughness: 0.28 });
  const studMat = new THREE.MeshStandardMaterial({ color: "#e2e8f0", metalness: 0.55, roughness: 0.28 });

  const jerseyTex = createJerseyTextures(kit, shirtNumber, surname, clubShort);
  const torsoMats = [
    new THREE.MeshStandardMaterial({ map: jerseyTex.front, roughness: 0.58 }),
    new THREE.MeshStandardMaterial({ map: jerseyTex.back, roughness: 0.58 }),
  ];

  const root = new THREE.Group();
  const bodyScaleGroup = new THREE.Group();
  root.add(bodyScaleGroup);

  // --- Construct 16-Bone Hierarchical Skeleton in Bind Pose ---
  const bones = Array.from({ length: 16 }, () => new THREE.Bone());
  bones[B_ROOT].position.set(0, 0, 0);
  bones[B_HIPS].position.set(0, HIPS_BIND_Y, 0);
  bones[B_SPINE].position.set(0, 0.14, 0); // y = 1.085
  bones[B_CHEST].position.set(0, 0.20, 0); // y = 1.285
  bones[B_NECK].position.set(0, 0.195, 0); // y = 1.480
  bones[B_HEAD].position.set(0, 0.075, 0); // y = 1.555

  bones[B_SH_L].position.set(0.188, 0.125, 0); // y = 1.410
  bones[B_EL_L].position.set(0, -0.255, 0); // y = 1.155
  bones[B_SH_R].position.set(-0.188, 0.125, 0);
  bones[B_EL_R].position.set(0, -0.255, 0);

  bones[B_HIP_L].position.copy(HIP_SOCKET_L); // (+0.096, 0.922, 0)
  bones[B_KNEE_L].position.set(0, -UPPER_LEG_LEN, 0); // y = 0.487
  bones[B_ANKLE_L].position.set(0, -LOWER_LEG_LEN, 0); // y = 0.062
  bones[B_HIP_R].position.copy(HIP_SOCKET_R); // (-0.096, 0.922, 0)
  bones[B_KNEE_R].position.set(0, -UPPER_LEG_LEN, 0);
  bones[B_ANKLE_R].position.set(0, -LOWER_LEG_LEN, 0);

  bones[B_ROOT].add(bones[B_HIPS]);
  bones[B_HIPS].add(bones[B_SPINE], bones[B_HIP_L], bones[B_HIP_R]);
  bones[B_SPINE].add(bones[B_CHEST]);
  bones[B_CHEST].add(bones[B_NECK], bones[B_SH_L], bones[B_SH_R]);
  bones[B_NECK].add(bones[B_HEAD]);
  bones[B_SH_L].add(bones[B_EL_L]);
  bones[B_SH_R].add(bones[B_EL_R]);
  bones[B_HIP_L].add(bones[B_KNEE_L]);
  bones[B_KNEE_L].add(bones[B_ANKLE_L]);
  bones[B_HIP_R].add(bones[B_KNEE_R]);
  bones[B_KNEE_R].add(bones[B_ANKLE_R]);

  bodyScaleGroup.add(bones[B_ROOT]);
  bodyScaleGroup.updateMatrixWorld(true);

  const skeleton = new THREE.Skeleton(bones);
  const identityBind = new THREE.Matrix4();

  // 1. Continuous Skinned Jersey Torso
  const torsoMesh = new THREE.SkinnedMesh(SHARED_GEO.skinnedTorso, torsoMats);
  torsoMesh.castShadow = true;
  torsoMesh.receiveShadow = true;
  torsoMesh.frustumCulled = false;
  torsoMesh.bind(skeleton, identityBind);
  bodyScaleGroup.add(torsoMesh);

  // 2. Continuous Skinned Sleeves + Arms (Zero Shoulder/Elbow Ball Seams!)
  const armsMesh = new THREE.SkinnedMesh(
    isGk ? SHARED_GEO.skinnedArmsGk : SHARED_GEO.skinnedArmsOutfield,
    [sleeveMat, trimMat, skinMat]
  );
  armsMesh.castShadow = true;
  armsMesh.frustumCulled = false;
  armsMesh.bind(skeleton, identityBind);
  bodyScaleGroup.add(armsMesh);

  // 3. Continuous Skinned Shorts + Quads + Knees + Gastrocnemius Calves
  const lowerBodyMesh = new THREE.SkinnedMesh(SHARED_GEO.skinnedLowerBody, [
    shortsMat,
    skinMat,
    sockBandMat,
    sockMat,
  ]);
  lowerBodyMesh.castShadow = true;
  lowerBodyMesh.receiveShadow = true;
  lowerBodyMesh.frustumCulled = false;
  lowerBodyMesh.bind(skeleton, identityBind);
  bodyScaleGroup.add(lowerBodyMesh);

  // Apply FIFA 11 Personality+ Somatotype & Format Scale AFTER skeleton bind!
  bodyScaleGroup.scale.set(soma.w * scale, soma.h * scale, soma.d * scale);

  // Collar, Neck & Sculpted 1 : 7.56 Head attached to Neck/Head Bones
  const collar = new THREE.Mesh(SHARED_GEO.collar, trimMat);
  collar.position.set(0, 0.002, -0.003);
  bones[B_NECK].add(collar);

  const neckMesh = new THREE.Mesh(SHARED_GEO.neck, skinMat);
  neckMesh.position.set(0, 0.038, -0.003);
  bones[B_NECK].add(neckMesh);

  const headMesh = new THREE.Mesh(SHARED_GEO.head, headFaceMat);
  headMesh.position.set(0, 0.088, 0);
  headMesh.castShadow = true;
  bones[B_HEAD].add(headMesh);

  const nose = new THREE.Mesh(SHARED_GEO.nose, skinMat);
  nose.position.set(0, 0.084, 0.104);
  bones[B_HEAD].add(nose);

  for (const sgn of [-1, 1]) {
    const ear = new THREE.Mesh(SHARED_GEO.ear, skinMat);
    ear.position.set(sgn * 0.095, 0.086, -0.004);
    bones[B_HEAD].add(ear);
  }

  const hair = buildHairMesh(likeness.style, likeness.hairHex);
  hair.position.set(0, 0.088, 0);
  bones[B_HEAD].add(hair);

  // Hands / Goalkeeper Gloves attached to Forearm Bones
  const gloveMat = isGk ? new THREE.MeshStandardMaterial({ color: "#f8fafc", roughness: 0.38 }) : null;
  for (const elBone of [bones[B_EL_L], bones[B_EL_R]]) {
    const handMesh = isGk
      ? new THREE.Mesh(SHARED_GEO.gkGlove, gloveMat)
      : new THREE.Mesh(SHARED_GEO.hand, skinMat);
    handMesh.position.set(0, -0.242, 0.01);
    handMesh.castShadow = true;
    elBone.add(handMesh);
  }

  // Flush-Sole 3D Cleats attached to Ankle Bones (Codex PR #6 fix: sole & studs land cleanly at y = 0.000!)
  for (const ankleBone of [bones[B_ANKLE_L], bones[B_ANKLE_R]]) {
    const cleat = new THREE.Mesh(SHARED_GEO.cleat, bootMat);
    cleat.position.set(0, 0, 0.018);
    cleat.castShadow = true;
    ankleBone.add(cleat);

    const stripe = new THREE.Mesh(SHARED_GEO.bootStripe, bootAccentMat);
    stripe.position.set(0, -0.025, 0.03);
    ankleBone.add(stripe);

    for (const [sx, sz] of [
      [-0.024, -0.045],
      [0.024, -0.045],
      [-0.026, 0.082],
      [0.026, 0.082],
    ]) {
      const st = new THREE.Mesh(SHARED_GEO.stud, studMat);
      st.position.set(sx, -0.055, sz); // Bottom of 0.014m stud reaches -0.062 = -SOLE_HEIGHT -> world y = 0!
      ankleBone.add(st);
    }
  }

  // Ground Contact Shadow & Minimalist Broadcast Floor Rings
  const contactShadow = new THREE.Mesh(
    SHARED_GEO.shadowDisc,
    new THREE.MeshBasicMaterial({
      color: "#020617",
      transparent: true,
      opacity: 0.32,
      depthWrite: false,
    })
  );
  contactShadow.position.y = 0.014;
  root.add(contactShadow);

  const ringColor = side === 0 ? "#fbbf24" : "#38bdf8";
  const ringMat = new THREE.MeshBasicMaterial({
    color: ringColor,
    transparent: true,
    opacity: 0.9,
    side: THREE.DoubleSide,
    depthWrite: false,
  });
  const ctrlRing = new THREE.Mesh(SHARED_GEO.ring, ringMat);
  ctrlRing.position.y = 0.022;
  ctrlRing.visible = false;
  root.add(ctrlRing);

  const aimWedge = new THREE.Mesh(SHARED_GEO.aimArrow, ringMat);
  aimWedge.position.set(0, 0.026, 0.84);
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
  chargeRing.position.y = 0.028;
  chargeRing.visible = false;
  root.add(chargeRing);

  const overheadCursor = new THREE.Mesh(
    SHARED_GEO.cursorCone,
    new THREE.MeshBasicMaterial({ color: ringColor })
  );
  overheadCursor.position.set(0, 2.18, 0);
  overheadCursor.visible = false;
  root.add(overheadCursor);

  const nameplateTex = createNameplateSpriteTexture(surname, shirtNumber, true, side);
  const nameplate = new THREE.Sprite(
    new THREE.SpriteMaterial({ map: nameplateTex, transparent: true, depthTest: false })
  );
  nameplate.scale.set(2.25, 0.55, 1);
  nameplate.position.set(0, 2.52, 0);
  nameplate.visible = false;
  root.add(nameplate);

  const phaseSeed = (shirtNumber * 1.37 + side * 2.1) % (Math.PI * 2);

  return {
    root,
    bones,
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
    phaseSeed,
    stridePhase: phaseSeed,
    prevAction: 0,
    kickSwingTimer: 0,
    smoothYaw: side === 0 ? Math.PI * 0.5 : -Math.PI * 0.5,
    smoothAccelFwd: 0,
    smoothAccelLat: 0,
    smoothHipY: HIPS_BIND_Y - 0.03,
    footGoalL: new THREE.Vector3(0.105, SOLE_HEIGHT, 0.04),
    footGoalR: new THREE.Vector3(-0.105, SOLE_HEIGHT, -0.04),
  };
}

const _poleL = new THREE.Vector3(0.06, 0.18, 1.0).normalize();
const _poleR = new THREE.Vector3(-0.06, 0.18, 1.0).normalize();

/**
 * Continuous CMU-Mocap-Tuned Walk -> Jog -> Sprint Foot Trajectory.
 * Smoothly blends from standing ready stance (speed = 0) through walk (< 1.8 m/s)
 * to aerial sprint (> 2.2 m/s) with zero threshold jumps!
 */
function evaluateContinuousFootTarget(phase, sideSign, speed, speedNorm, isGk, outVec) {
  const locoWeight = smoothstep(0.04, 0.72, speed);
  const idleX = sideSign * (isGk ? 0.165 : 0.104);
  const idleZ = sideSign > 0 ? 0.042 : -0.042;

  if (locoWeight <= 0.001) {
    outVec.set(idleX, SOLE_HEIGHT, idleZ);
    return 0.0;
  }

  const p = ((phase % (Math.PI * 2)) + Math.PI * 2) % (Math.PI * 2);
  const runX = sideSign * (isGk ? 0.135 : 0.098);
  const halfSpan = Math.min(0.56, 0.12 + speed * 0.054) * locoWeight;
  const runBlend = smoothstep(1.6, 3.8, speed); // 0 = walk, 1 = dynamic run/sprint

  let gx = idleX + (runX - idleX) * locoWeight;
  let gy = SOLE_HEIGHT;
  let gz = 0;
  let pitch = 0;

  if (p < Math.PI) {
    // STANCE: Cleat locked onto turf sweeping front (+halfSpan) -> rear (-halfSpan)
    const u = p / Math.PI;
    gz = halfSpan * (1.0 - 2.0 * u);
    const heelPitch = -0.24 * (0.45 + 0.55 * runBlend);
    const toeOffPitch = (0.34 + 0.24 * runBlend) * locoWeight;
    pitch =
      u < 0.22
        ? heelPitch * (1 - u / 0.22)
        : u > 0.64
          ? ((u - 0.64) / 0.36) * toeOffPitch
          : 0.0;
    const toeLiftY = u > 0.72 ? ((u - 0.72) / 0.28) * 0.042 * locoWeight : 0.0;
    gy = SOLE_HEIGHT + toeLiftY;
  } else {
    // SWING RECOVERY: Low toe-clearance arc in walk -> high heel-fold recovery in sprint!
    const w = (p - Math.PI) / Math.PI;
    gz = -halfSpan + 2.0 * halfSpan * (w - Math.sin(w * Math.PI * 2) * 0.055);
    const peakLift = (0.055 + speedNorm * 0.27 * (0.35 + 0.65 * runBlend)) * locoWeight;
    gy = SOLE_HEIGHT + Math.sin(w * Math.PI) * peakLift;
    const heelFold = Math.sin(Math.min(1, w * 1.32) * Math.PI);
    pitch = (0.28 + 0.34 * runBlend) * heelFold * locoWeight - (w > 0.76 ? ((w - 0.76) / 0.24) * 0.24 * locoWeight : 0);
  }

  outVec.set(gx, gy, idleZ * (1 - locoWeight) + gz);
  return pitch;
}

const _tmpFootL = new THREE.Vector3();
const _tmpFootR = new THREE.Vector3();

function animateFootballerActor(actor, state, dt, nowSec, particleSystem) {
  const {
    wx,
    wy,
    vx,
    vy,
    axTick,
    ayTick,
    facingRad,
    action,
    charge,
    isControlled,
    isBallCarrier,
    ballX,
    ballY,
    celebrating,
  } = state;

  actor.root.position.set(wx, 0, wy);

  // Frame-rate-independent shortest-arc facing yaw via maath.dampAngle
  const targetYaw = Math.PI * 0.5 - facingRad;
  const prevYaw = actor.smoothYaw;
  dampAngle(actor, "smoothYaw", targetYaw, 0.062, dt);
  const yawRate = wrapAngleRad(actor.smoothYaw - prevYaw) / Math.max(0.004, dt);

  const spinExtra = action === 7 ? (nowSec * 14.0) % (Math.PI * 2) : 0;
  actor.root.rotation.y = actor.smoothYaw + spinExtra;

  // Smooth tick-derived world acceleration in athlete local coordinates (zero rAF dt differentiation jitter!)
  const fwdX = Math.sin(actor.smoothYaw);
  const fwdZ = Math.cos(actor.smoothYaw);
  const leftX = Math.cos(actor.smoothYaw);
  const leftZ = -Math.sin(actor.smoothYaw);

  const rawAccelFwd = Math.max(-20, Math.min(20, axTick * fwdX + ayTick * fwdZ));
  const rawAccelLat = Math.max(-20, Math.min(20, axTick * leftX + ayTick * leftZ));
  damp(actor, "smoothAccelFwd", rawAccelFwd, 0.085, dt);
  damp(actor, "smoothAccelLat", rawAccelLat, 0.085, dt);

  const speed = Math.hypot(vx, vy);
  const speedNorm = Math.min(1.25, speed / 6.8);
  const locoWeight = smoothstep(0.05, 0.75, speed);

  // Cleat turf divot spray on sharp cuts or slide tackles
  if (particleSystem && (Math.hypot(axTick, ayTick) > 17.5 || action === 3) && Math.random() < dt * 18) {
    particleSystem.spawnBurst(wx, 0.04, wy, -vx * 0.22, -vy * 0.22, action === 3 ? 4 : 2);
  }

  // Trigger continuous proximal-to-distal kick whip follow-through on release from Charge (1)
  if (actor.prevAction === 1 && action !== 1) {
    actor.kickSwingTimer = 0.26; // 260ms smooth follow-through
    if (particleSystem) {
      particleSystem.spawnBurst(wx + fwdX * 0.34, 0.05, wy + fwdZ * 0.34, fwdX * 2.1, fwdZ * 2.1, 4);
    }
  }
  actor.prevAction = action;
  if (actor.kickSwingTimer > 0) {
    actor.kickSwingTimer = Math.max(0, actor.kickSwingTimer - dt);
  }

  // HUD Rings & Nameplate
  actor.ctrlRing.visible = isControlled;
  actor.aimWedge.visible = isControlled;
  actor.overheadCursor.visible = isControlled;
  if (isControlled) {
    actor.overheadCursor.position.y = 2.16 + Math.sin(nowSec * 7.5) * 0.05;
    actor.overheadCursor.rotation.y = nowSec * 3.2;
  }
  actor.nameplate.visible = Boolean(isControlled || isBallCarrier);

  if (charge > 0.02) {
    actor.chargeRing.visible = true;
    const sRing = 0.85 + charge * 0.34;
    actor.chargeRing.scale.set(sRing, 1, sRing);
    actor.chargeRingMat.color.setHex(
      charge > 0.78 ? 0xef4444 : charge > 0.45 ? 0xfbbf24 : 0x22c55e
    );
  } else {
    actor.chargeRing.visible = false;
  }

  const bones = actor.bones;

  // Smooth Head Tracking toward Ball + Turn Anticipation
  const toBallAng = wrapAngleRad(Math.atan2(ballX - wx, ballY - wy) - actor.smoothYaw);
  const idleNoise = seededNoise2D(nowSec * 0.55, actor.phaseSeed * 3.1) * 0.06 * (1 - locoWeight);
  const headYawGoal = Math.max(-0.66, Math.min(0.66, toBallAng * 0.65 + yawRate * 0.045)) + idleNoise;
  dampBoneEuler(bones[B_HEAD], 0.04 - speedNorm * 0.06, headYawGoal, 0, 0.075, dt);
  dampBoneEuler(bones[B_NECK], speedNorm * 0.05, headYawGoal * 0.25, 0, 0.075, dt);

  // Special Full-Body States (blended smoothly via dampQ so entry & exit never snap!)
  if (action === 3) {
    // Slide Tackle
    damp(actor, "smoothHipY", 0.30, 0.055, dt);
    bones[B_HIPS].position.set(0, actor.smoothHipY, 0);
    dampBoneEuler(bones[B_HIPS], -1.16, 0, 0, 0.055, dt);
    dampBoneEuler(bones[B_SPINE], 0.22, -0.12, 0, 0.055, dt);
    dampBoneEuler(bones[B_CHEST], 0.18, -0.1, 0, 0.055, dt);
    dampBoneEuler(bones[B_HIP_R], -0.92, 0, -0.05, 0.05, dt);
    dampBoneEuler(bones[B_KNEE_R], 0.14, 0, 0, 0.05, dt);
    dampBoneEuler(bones[B_ANKLE_R], 0.35, 0, 0, 0.05, dt);
    dampBoneEuler(bones[B_HIP_L], 0.32, 0, 0.18, 0.05, dt);
    dampBoneEuler(bones[B_KNEE_L], 1.48, 0, 0, 0.05, dt);
    dampBoneEuler(bones[B_ANKLE_L], 0.2, 0, 0, 0.05, dt);
    dampBoneEuler(bones[B_SH_L], -0.52, 0, 0.62, 0.055, dt);
    dampBoneEuler(bones[B_SH_R], 0.35, 0, -0.68, 0.055, dt);
    dampBoneEuler(bones[B_EL_L], -0.62, 0, 0, 0.055, dt);
    dampBoneEuler(bones[B_EL_R], -0.52, 0, 0, 0.055, dt);
    actor.contactShadow.scale.set(1.5, 1.4, 1);
    return;
  }

  if (action === 4) {
    // Goalkeeper Full-Stretch Dive
    const lateral = vx * Math.sin(facingRad) - vy * Math.cos(facingRad);
    const diveSign = lateral >= 0 ? -1 : 1;
    damp(actor, "smoothHipY", 0.58, 0.05, dt);
    bones[B_HIPS].position.set(0, actor.smoothHipY, 0);
    dampBoneEuler(bones[B_HIPS], 0, 0, diveSign * 1.25, 0.05, dt);
    dampBoneEuler(bones[B_SPINE], 0.05, 0, diveSign * 0.12, 0.05, dt);
    dampBoneEuler(bones[B_CHEST], 0.05, 0, diveSign * 0.12, 0.05, dt);
    dampBoneEuler(bones[B_SH_L], -2.58, 0, 0.22, 0.05, dt);
    dampBoneEuler(bones[B_SH_R], -2.58, 0, -0.22, 0.05, dt);
    dampBoneEuler(bones[B_EL_L], -0.18, 0, 0, 0.05, dt);
    dampBoneEuler(bones[B_EL_R], -0.18, 0, 0, 0.05, dt);
    dampBoneEuler(bones[B_HIP_L], -0.14, 0, 0.16, 0.05, dt);
    dampBoneEuler(bones[B_HIP_R], -0.1, 0, -0.16, 0.05, dt);
    dampBoneEuler(bones[B_KNEE_L], 0.4, 0, 0, 0.05, dt);
    dampBoneEuler(bones[B_KNEE_R], 0.24, 0, 0, 0.05, dt);
    dampBoneEuler(bones[B_ANKLE_L], 0.22, 0, 0, 0.05, dt);
    dampBoneEuler(bones[B_ANKLE_R], 0.22, 0, 0, 0.05, dt);
    actor.contactShadow.scale.set(1.6, 1.2, 1);
    return;
  }

  actor.contactShadow.scale.set(1, 1, 1);

  // Distance-Locked Stride Phase Integration (Zero Foot-Skating!)
  const strideLenMeters = 1.08 + speed * 0.23;
  if (speed > 0.05) {
    actor.stridePhase = (actor.stridePhase + ((speed * dt) / strideLenMeters) * Math.PI * 2) % (Math.PI * 2);
  }

  const s = Math.sin(actor.stridePhase);

  // Biomechanical CoM Lean, Centripetal Banking & Multi-Segment Thoracic Counter-Twist
  const leanPitch = Math.max(-0.2, Math.min(0.32, actor.smoothAccelFwd * 0.015 + speedNorm * 0.11));
  const leanRoll = Math.max(-0.24, Math.min(0.24, -actor.smoothAccelLat * 0.014 - yawRate * 0.032));

  let baseHipY = actor.isGk && speed < 0.4 ? 0.84 : HIPS_BIND_Y - 0.025 - speedNorm * 0.028;
  if (action === 2) baseHipY = 0.80;
  const idleBreathe = seededNoise2D(actor.phaseSeed, nowSec * 0.9) * 0.008 * (1 - locoWeight);
  const runBounce = -Math.cos(actor.stridePhase * 2) * 0.034 * locoWeight * Math.min(1, speedNorm + 0.18);
  damp(actor, "smoothHipY", baseHipY + runBounce + idleBreathe, 0.045, dt);
  bones[B_HIPS].position.set(0, actor.smoothHipY, 0);

  const pelvisYaw = -s * 0.12 * locoWeight * Math.min(1, speedNorm + 0.2);
  const pelvisHipDrop = s * 0.048 * locoWeight * Math.min(1, speedNorm + 0.2);
  dampBoneEuler(bones[B_HIPS], leanPitch * 0.36, pelvisYaw, leanRoll + pelvisHipDrop, 0.05, dt);

  // Non-Skating Kick Windup Blend: player keeps running naturally during early charge,
  // cocking the striking leg ONLY in the final strike plant window (charge > 0.52) or when stationary!
  const stationaryFactor = 1.0 - smoothstep(0.3, 2.2, speed);
  const chargeCockBlend =
    action === 1 || charge > 0.03
      ? Math.min(1, smoothstep(0.52, 0.92, charge) + stationaryFactor * charge * 0.85)
      : 0;
  const swingProgress = actor.kickSwingTimer > 0 ? 1.0 - actor.kickSwingTimer / 0.26 : -1;
  const whipArc = swingProgress >= 0 ? Math.sin(swingProgress * Math.PI) : 0;

  let spinePitch = leanPitch * 0.34 + (actor.isGk && speed < 0.4 ? 0.12 : 0.03);
  let spineYaw = s * 0.11 * locoWeight * Math.min(1, speedNorm + 0.25);
  let spineRoll = -pelvisHipDrop * 0.45;

  if (chargeCockBlend > 0.01) {
    spineYaw -= 0.20 * chargeCockBlend;
    spineRoll += 0.08 * chargeCockBlend;
  } else if (whipArc > 0.01) {
    spinePitch -= 0.10 * whipArc;
    spineYaw += 0.18 * whipArc;
  }

  dampBoneEuler(bones[B_SPINE], spinePitch, spineYaw, spineRoll, 0.05, dt);
  dampBoneEuler(bones[B_CHEST], spinePitch * 0.95, spineYaw * 1.15, spineRoll * 0.65, 0.05, dt);

  // Evaluate Continuous Locomotion Foot Targets
  let pitchL = evaluateContinuousFootTarget(actor.stridePhase, 1, speed, speedNorm, actor.isGk, _tmpFootL);
  let pitchR = evaluateContinuousFootTarget(
    actor.stridePhase + Math.PI,
    -1,
    speed,
    speedNorm,
    actor.isGk,
    _tmpFootR
  );

  if (action === 2) {
    // Standing Tactical Tackle Lunge
    _tmpFootR.set(-0.10, SOLE_HEIGHT, 0.54);
    pitchR = -0.18;
    _tmpFootL.set(0.11, SOLE_HEIGHT + 0.02, -0.36);
    pitchL = 0.42;
  } else if (chargeCockBlend > 0.01) {
    // Smoothly blend striking leg into instep backswing while support leg continues natural plant!
    const cockR = _vEffTarget.set(-0.11, SOLE_HEIGHT + 0.34 * chargeCockBlend, -0.36 * chargeCockBlend);
    _tmpFootR.lerp(cockR, chargeCockBlend);
    pitchR = pitchR * (1 - chargeCockBlend) + 0.62 * chargeCockBlend;
    if (stationaryFactor > 0.4) {
      _tmpFootL.set(0.125, SOLE_HEIGHT, 0.12 * chargeCockBlend);
      pitchL = 0;
    }
  } else if (whipArc > 0.01) {
    // Proximal-to-distal instep whip through ball impact & cross-body follow-through
    const strikeR = _vEffTarget.set(
      -0.09 + whipArc * 0.08,
      SOLE_HEIGHT + whipArc * 0.38,
      -0.12 + whipArc * 0.66
    );
    _tmpFootR.lerp(strikeR, Math.min(1, whipArc * 1.35));
    pitchR = -0.32 * whipArc;
  }

  damp3(actor.footGoalL, _tmpFootL, 0.022, dt);
  damp3(actor.footGoalR, _tmpFootR, 0.022, dt);

  solveAndDampLegIK(
    bones[B_HIPS],
    bones[B_HIP_L],
    bones[B_KNEE_L],
    bones[B_ANKLE_L],
    HIP_SOCKET_L,
    actor.footGoalL,
    _poleL,
    pitchL,
    0.032,
    dt
  );
  solveAndDampLegIK(
    bones[B_HIPS],
    bones[B_HIP_R],
    bones[B_KNEE_R],
    bones[B_ANKLE_R],
    HIP_SOCKET_R,
    actor.footGoalR,
    _poleR,
    pitchR,
    0.032,
    dt
  );

  // Continuous Quaternion-Damped Upper-Limb Counter-Balance Kinematics
  if (celebrating) {
    dampBoneEuler(bones[B_SH_L], -2.35 + s * 0.22, 0, 0.44, 0.055, dt);
    dampBoneEuler(bones[B_SH_R], -2.35 - s * 0.22, 0, -0.44, 0.055, dt);
    dampBoneEuler(bones[B_EL_L], -0.26, 0, 0, 0.055, dt);
    dampBoneEuler(bones[B_EL_R], -0.26, 0, 0, 0.055, dt);
  } else if (chargeCockBlend > 0.25) {
    const w = chargeCockBlend;
    dampBoneEuler(bones[B_SH_L], -0.58 * w, 0.22 * w, 0.68 * w, 0.05, dt);
    dampBoneEuler(bones[B_EL_L], -0.45, 0, 0, 0.05, dt);
    dampBoneEuler(bones[B_SH_R], 0.35 * w, 0, -0.28, 0.05, dt);
    dampBoneEuler(bones[B_EL_R], -0.82, 0, 0, 0.05, dt);
  } else if (whipArc > 0.1) {
    dampBoneEuler(bones[B_SH_L], 0.45 * whipArc, 0, 0.5, 0.045, dt);
    dampBoneEuler(bones[B_SH_R], -0.65 * whipArc, 0, -0.34, 0.045, dt);
    dampBoneEuler(bones[B_EL_L], -0.7, 0, 0, 0.045, dt);
    dampBoneEuler(bones[B_EL_R], -0.8, 0, 0, 0.045, dt);
  } else {
    // Seamless continuous blend from idle/GK ready arms into athletic sprint pump (zero speed > 0.25 snap!)
    const idleArmPitch = actor.isGk ? -0.52 : 0.04 + idleBreathe * 2.5;
    const idleArmAbd = actor.isGk ? 0.38 : 0.14;
    const idleElbow = actor.isGk ? -0.92 : -0.46;

    const amp = Math.min(1.06, 0.28 + speedNorm * 0.74);
    const runShPitchL = s * 0.82 * amp;
    const runShPitchR = -s * 0.82 * amp;
    const runAbd = 0.13 + speedNorm * 0.06;
    const runElL = -0.74 - speedNorm * 0.48 - Math.max(0, -s) * 0.22;
    const runElR = -0.74 - speedNorm * 0.48 - Math.max(0, s) * 0.22;

    dampBoneEuler(
      bones[B_SH_L],
      idleArmPitch * (1 - locoWeight) + runShPitchL * locoWeight,
      0,
      idleArmAbd * (1 - locoWeight) + runAbd * locoWeight,
      0.045,
      dt
    );
    dampBoneEuler(
      bones[B_SH_R],
      -idleArmPitch * (1 - locoWeight) + runShPitchR * locoWeight,
      0,
      -(idleArmAbd * (1 - locoWeight) + runAbd * locoWeight),
      0.045,
      dt
    );
    dampBoneEuler(
      bones[B_EL_L],
      idleElbow * (1 - locoWeight) + runElL * locoWeight,
      0,
      -0.1 * locoWeight,
      0.045,
      dt
    );
    dampBoneEuler(
      bones[B_EL_R],
      idleElbow * (1 - locoWeight) + runElR * locoWeight,
      0,
      0.1 * locoWeight,
      0.045,
      dt
    );
  }
}

// ============================================================================
// 5. TURF CLEAT DIVOT PARTICLE POOL & STADIUM BUILDER
// ============================================================================

class TurfParticleSystem {
  constructor(scene) {
    this.capacity = 140;
    const geo = retainGpu(new THREE.BoxGeometry(0.052, 0.052, 0.052));
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
      p.life = 0.36 + (k % 3) * 0.08;
      p.x = x + (((this.cursor * 17) % 11) - 5) * 0.03;
      p.y = y;
      p.z = z + (((this.cursor * 31) % 11) - 5) * 0.03;
      p.vx = baseVx + (((this.cursor * 13) % 9) - 4) * 0.35;
      p.vy = 1.55 + (k % 3) * 0.6;
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

function buildGoalAssembly(goalX, sign, goalHW, goalH, netTex, accentHex) {
  const group = new THREE.Group();
  const postMat = new THREE.MeshStandardMaterial({
    color: "#ffffff",
    roughness: 0.2,
    metalness: 0.28,
  });
  const stanchionMat = new THREE.MeshStandardMaterial({
    color: accentHex || "#334155",
    roughness: 0.42,
    metalness: 0.38,
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
  const goalH = (fmt.goal_height_mm / 2000) * 2;

  const kits = resolveMatchKits(
    snapshot.home.club_id,
    snapshot.away.club_id,
    snapshot.home.primary_color,
    snapshot.away.primary_color
  );

  const trackFloor = new THREE.Mesh(
    new THREE.PlaneGeometry(pitchLen + 76, pitchWid + 76),
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
    new THREE.MeshStandardMaterial({ map: pitchTex, roughness: 0.84, metalness: 0.02 })
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
        new THREE.SphereGeometry(0.1, 10, 8),
        new THREE.MeshStandardMaterial({ color: SKIN_TONES[(s + sideIdx * 2) % SKIN_TONES.length] })
      );
      subHead.position.set(-1.5 + s * 1.0, 1.12, -0.1);
      dugout.add(subHead);
    }
    stadium.add(dugout);
  }

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
    emissiveIntensity: 0.24,
    roughness: 0.45,
  });

  const adDistZ = halfW + 4.25;
  const adDistX = halfL + 4.65;
  const adH = 1.05;

  const northAd = new THREE.Mesh(new THREE.BoxGeometry(adDistX * 2, adH, 0.18), adMat);
  northAd.position.set(0, adH * 0.5, -adDistZ);
  northAd.castShadow = true;
  stadium.add(northAd);

  const southAd = new THREE.Mesh(new THREE.BoxGeometry(adDistX * 2, adH * 0.65, 0.18), adMat);
  southAd.position.set(0, adH * 0.32, adDistZ + 1.8);
  stadium.add(southAd);

  for (const xSign of [-1, 1]) {
    const endAd = new THREE.Mesh(new THREE.BoxGeometry(0.18, adH, adDistZ * 2), adMat);
    endAd.position.set(xSign * adDistX, adH * 0.5, 0);
    endAd.castShadow = true;
    stadium.add(endAd);
  }

  const concreteMat = new THREE.MeshStandardMaterial({ color: "#1e293b", roughness: 0.88 });
  const riserDarkMat = new THREE.MeshStandardMaterial({ color: "#0f172a", roughness: 0.92 });
  const stairMat = new THREE.MeshStandardMaterial({ color: "#ca8a04", roughness: 0.7 });
  const roofMat = new THREE.MeshStandardMaterial({ color: "#0b1324", roughness: 0.65, metalness: 0.3 });
  const fasciaMatHome = new THREE.MeshStandardMaterial({
    color: kits.homeOutfield.shirt,
    emissive: kits.homeOutfield.shirt,
    emissiveIntensity: 0.2,
    roughness: 0.5,
  });
  const fasciaMatAway = new THREE.MeshStandardMaterial({
    color: kits.awayOutfield.shirt,
    emissive: kits.awayOutfield.shirt,
    emissiveIntensity: 0.2,
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
  // Push south gantry stand far back so zoomed-out Tele Broadcast camera never clips seats!
  buildStand(standSpanX, adDistZ + 28.0, Math.PI, 4, false, false);

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

  // Emissive Floodlight Towers (pick up postprocessing BloomEffect!)
  const pylonMat = new THREE.MeshStandardMaterial({ color: "#475569", metalness: 0.6, roughness: 0.4 });
  const bulbMat = new THREE.MeshStandardMaterial({
    color: "#fffbeb",
    emissive: "#fef3c7",
    emissiveIntensity: 2.6,
  });
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
    halo.scale.set(15, 15, 1);
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
// 6. PUBLIC ENGINE CLASS (WITH POSTPROCESSING, PBR IBL & ZOOMED-OUT GANTRY CAM)
// ============================================================================

export const CAMERA_PRESETS = ["tele", "wide", "dynamic", "pro"];
export const CAMERA_LABELS = {
  tele: "TELE BROADCAST",
  wide: "CO-OP TACTICAL",
  dynamic: "DYNAMIC ACTION",
  pro: "END-TO-END PRO",
};

export class ClassicFifa3DEngine {
  constructor(canvas) {
    this.canvas = canvas;
    this.renderer = new THREE.WebGLRenderer({
      canvas,
      antialias: false, // SMAAEffect handles crisp subpixel morphological AA in single uber-pass
      powerPreference: "high-performance",
    });
    this.renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
    this.renderer.outputColorSpace = THREE.SRGBColorSpace;
    this.renderer.shadowMap.enabled = true;
    this.renderer.shadowMap.type = THREE.PCFShadowMap; // Soft PCF in Three.js r182+

    this.scene = new THREE.Scene();
    this.scene.fog = new THREE.FogExp2("#0c192c", 0.0032);

    // Physically-Based Image-Based Lighting (RoomEnvironment + PMREMGenerator)
    const pmrem = new THREE.PMREMGenerator(this.renderer);
    const roomEnv = new RoomEnvironment();
    this.envRT = pmrem.fromScene(roomEnv, 0.04);
    retainGpu(this.envRT.texture);
    this.scene.environment = this.envRT.texture;
    this.scene.environmentIntensity = 0.46;
    roomEnv.dispose();
    pmrem.dispose();

    this.camera = new THREE.PerspectiveCamera(30.5, 16 / 9, 0.4, 460);
    this.cameraMode = "tele";
    this.viewMode = "attract";
    this.userZoom = 1.0; // Adjustable [0.72 .. 1.45] via Mouse Wheel, - / =, or HUD Zoom buttons

    this.camPos = new THREE.Vector3(0, 21.5, 36.5);
    this.camLook = new THREE.Vector3(0, 0.55, -1.2);
    this.smoothPlayFocus = new THREE.Vector3(0, 0, 0);

    // pmndrs/postprocessing single merged uber-shader pipeline (with safe fallback)
    this.composer = null;
    try {
      this.renderer.toneMapping = THREE.NoToneMapping;
      const composer = new EffectComposer(this.renderer, {
        frameBufferType: THREE.HalfFloatType,
      });
      composer.addPass(new RenderPass(this.scene, this.camera));
      const smaa = new SMAAEffect();
      const bloom = new BloomEffect({
        intensity: 0.34,
        luminanceThreshold: 0.88,
        luminanceSmoothing: 0.2,
        mipmapBlur: true,
      });
      const vignette = new VignetteEffect({ darkness: 0.24, offset: 0.36 });
      const toneMap = new ToneMappingEffect({ mode: ToneMappingMode.ACES_FILMIC });
      composer.addPass(new EffectPass(this.camera, smaa, bloom, vignette, toneMap));
      this.composer = composer;
    } catch (_err) {
      this.composer = null;
      this.renderer.toneMapping = THREE.ACESFilmicToneMapping;
      this.renderer.toneMappingExposure = 1.08;
    }

    this.glowTex = createGlowSpriteTexture();
    this.maxAniso = this.renderer.capabilities.getMaxAnisotropy();

    const skyDome = new THREE.Mesh(
      retainGpu(new THREE.SphereGeometry(280, 28, 18)),
      retainGpu(new THREE.MeshBasicMaterial({ map: createSkyTexture(), side: THREE.BackSide, fog: false }))
    );
    this.scene.add(skyDome);

    const hemi = new THREE.HemisphereLight(0xdbeafe, 0x15803d, 0.82);
    this.scene.add(hemi);

    this.keyLight = new THREE.DirectionalLight(0xfffbeb, 2.05);
    this.keyLight.position.set(26, 46, 32);
    this.keyLight.castShadow = true;
    this.keyLight.shadow.mapSize.set(2048, 2048);
    this.keyLight.shadow.camera.near = 8;
    this.keyLight.shadow.camera.far = 160;
    this.keyLight.shadow.bias = -0.0004;
    this.scene.add(this.keyLight);

    const rimLight = new THREE.DirectionalLight(0xbae6fd, 0.72);
    rimLight.position.set(-30, 36, -36);
    this.scene.add(rimLight);

    const frontFillLight = new THREE.DirectionalLight(0xffedd5, 0.65);
    frontFillLight.position.set(0, 15, 42);
    this.scene.add(frontFillLight);

    this.ballRadius = 0.22;
    this.ballMesh = new THREE.Mesh(
      retainGpu(new THREE.SphereGeometry(this.ballRadius, 28, 22)),
      retainGpu(
        new THREE.MeshStandardMaterial({
          map: createMatchBallTexture(),
          roughness: 0.24,
          metalness: 0.08,
        })
      )
    );
    this.ballMesh.castShadow = true;
    this.scene.add(this.ballMesh);

    this.ballShadow = new THREE.Mesh(
      retainGpu(new THREE.CircleGeometry(0.28, 18)),
      retainGpu(
        new THREE.MeshBasicMaterial({
          color: "#020617",
          transparent: true,
          opacity: 0.44,
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
    if (this.composer) {
      this.composer.setSize(w, h);
    }
    this.camera.aspect = w / h;
    this.camera.updateProjectionMatrix();
  }

  adjustZoom(delta) {
    this.userZoom = Math.max(0.72, Math.min(1.48, this.userZoom + delta));
    return this.userZoom;
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
    const prevMode = this.viewMode;
    this.viewMode = mode;
    const showMatchActors = mode !== "showcase";
    for (const a of this.actors) {
      a.root.visible = showMatchActors;
    }
    this.ballMesh.visible = true;
    this.ballShadow.visible = true;
    this.showcaseGroup.visible = mode === "showcase";

    if (mode !== prevMode) {
      if (mode === "showcase") {
        this.camPos.set(0, 1.55, 6.65);
        this.camLook.set(0, 0.58, 0.15);
      } else if (mode === "match") {
        const is11 = this.stadiumData && this.stadiumData.pitchLen > 80;
        const z = this.userZoom;
        this.camPos.set(0, (is11 ? 31.5 : 24.8) * z, (is11 ? 52.0 : 41.5) * z);
        this.camLook.set(0, 0.55, -1.2);
      }
    }
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

  _presentScene() {
    if (this.composer) {
      this.composer.render();
    } else {
      this.renderer.render(this.scene, this.camera);
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
    this.smoothPlayFocus.set(0, 0, 0);

    const sx = this.stadiumData.halfL + 8;
    const sz = this.stadiumData.halfW + 10;
    this.keyLight.shadow.camera.left = -sx;
    this.keyLight.shadow.camera.right = sx;
    this.keyLight.shadow.camera.top = sz;
    this.keyLight.shadow.camera.bottom = -sz;
    this.keyLight.shadow.camera.updateProjectionMatrix();

    const pps = snapshot.format.players_per_side;
    const playerScale = pps <= 6 ? 1.08 : 1.01;
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
      scale: 1.02,
    });
    homeCapt.root.position.set(-0.88, 0, 0.15);
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
      scale: 1.02,
    });
    awayCapt.root.position.set(0.88, 0, 0.15);
    awayCapt.smoothYaw = -0.42;
    awayCapt.root.rotation.y = -0.42;

    this.showcaseGroup.add(homeCapt.root);
    this.showcaseGroup.add(awayCapt.root);
    this.showcaseCaptains = [homeCapt, awayCapt];
  }

  renderFrame(currPacked, replayOpts = null, prevPacked = null, alpha = 1.0) {
    const now = performance.now();
    const nowSec = now * 0.001;
    const dt = Math.min(0.05, Math.max(0.002, (now - this.lastTime) / 1000));
    this.lastTime = now;

    this.particles.step(dt);

    if (!this.stadiumData) {
      this._presentScene();
      return;
    }

    const { halfL, halfW, pitchLen, leftGoal, rightGoal, cornerFlags, crowdMesh, crowdPositions } =
      this.stadiumData;

    const waveT = nowSec * 4.0;
    for (let i = 0; i < cornerFlags.length; i++) {
      cornerFlags[i].rotation.y = Math.sin(waveT + i * 1.4) * 0.28;
    }

    for (let g = 0; g < 2; g++) {
      if (this.netRipple[g] > 0.005) {
        this.netRipple[g] *= Math.pow(0.04, dt);
        const holder = g === 0 ? leftGoal.netHolder : rightGoal.netHolder;
        const bulge = Math.sin(nowSec * 35) * this.netRipple[g] * 0.14;
        holder.scale.set(1 + Math.abs(bulge), 1 + bulge * 0.3, 1 + bulge * 0.4);
      }
    }

    if (this.goalExcitement > 0) {
      this.goalExcitement = Math.max(0, this.goalExcitement - dt);
    }
    if (crowdMesh && (this.goalExcitement > 0.01 || Math.floor(now / 50) % 2 === 0)) {
      const dummy = new THREE.Object3D();
      const amp = this.goalExcitement > 0.01 ? 0.24 : 0.032;
      const spd = this.goalExcitement > 0.01 ? 14.0 : 3.4;
      for (let i = 0; i < crowdPositions.length; i += 2) {
        const p = crowdPositions[i];
        dummy.position.set(p.x, p.y + Math.abs(Math.sin(nowSec * spd + p.phase)) * amp, p.z);
        dummy.updateMatrix();
        crowdMesh.setMatrixAt(i, dummy.matrix);
      }
      crowdMesh.instanceMatrix.needsUpdate = true;
    }

    // --- Pre-Match Showcase View ---
    if (this.viewMode === "showcase") {
      this.ballMesh.position.set(0, this.ballRadius, 0.85);
      this.ballMesh.rotation.y += dt * 0.8;
      this.ballShadow.position.set(0, 0.018, 0.85);

      if (this.showcaseCaptains) {
        this.showcaseCaptains.forEach((cap, cIdx) => {
          const sgn = cIdx === 0 ? 1 : -1;
          const breathe = seededNoise2D(cap.phaseSeed, nowSec * 0.8) * 0.01;
          const bones = cap.bones;
          bones[B_HIPS].position.set(0, HIPS_BIND_Y - 0.02 + breathe, 0);
          dampBoneEuler(bones[B_HIPS], 0.02, -sgn * 0.06, sgn * 0.02, 0.08, dt);
          dampBoneEuler(bones[B_SPINE], 0.02, -sgn * 0.05, -sgn * 0.015, 0.08, dt);
          dampBoneEuler(bones[B_CHEST], 0.02, -sgn * 0.05, -sgn * 0.015, 0.08, dt);

          solveAndDampLegIK(
            bones[B_HIPS],
            bones[B_HIP_L],
            bones[B_KNEE_L],
            bones[B_ANKLE_L],
            HIP_SOCKET_L,
            _tmpFootL.set(0.118, SOLE_HEIGHT, 0.05 * sgn),
            _poleL,
            0.0,
            0.06,
            dt
          );
          solveAndDampLegIK(
            bones[B_HIPS],
            bones[B_HIP_R],
            bones[B_KNEE_R],
            bones[B_ANKLE_R],
            HIP_SOCKET_R,
            _tmpFootR.set(-0.118, SOLE_HEIGHT, -0.04 * sgn),
            _poleR,
            0.0,
            0.06,
            dt
          );

          dampBoneEuler(bones[B_SH_L], 0.08, 0, 0.18, 0.08, dt);
          dampBoneEuler(bones[B_SH_R], -0.06, 0, -0.18, 0.08, dt);
          dampBoneEuler(bones[B_EL_L], -0.52, 0, -0.1, 0.08, dt);
          dampBoneEuler(bones[B_EL_R], -0.48, 0, 0.1, 0.08, dt);
          dampBoneEuler(
            bones[B_HEAD],
            0.03,
            Math.sin(nowSec * 1.2 + cap.phaseSeed) * 0.08 - sgn * 0.08,
            0,
            0.08,
            dt
          );
        });
      }

      const sway = Math.sin(nowSec * 0.65) * 0.35;
      this.camera.fov = 30.5;
      this.camera.updateProjectionMatrix();
      damp3(this.camPos, [sway, 1.55, 6.65], 0.14, dt);
      damp3(this.camLook, [0, 0.58, 0.15], 0.14, dt);
      this.camera.position.copy(this.camPos);
      this.camera.lookAt(this.camLook);
      this._presentScene();
      return;
    }

    // --- Sub-Tick Interpolated Live / Attract / Replay State ---
    let ballX = 0;
    let ballY = 0;
    let ballZ = 0;
    let ballVx = 0;
    let ballVy = 0;

    if (currPacked) {
      const p0 = prevPacked && prevPacked.length === currPacked.length ? prevPacked : currPacked;
      const tInterp = Math.max(0, Math.min(1, alpha));

      const phase = currPacked[3];
      const pps = currPacked[6];
      const homeCtrl = currPacked[7];
      const awayCtrl = currPacked[8];

      const bx0 = p0[9] / 1000;
      const by0 = p0[10] / 1000;
      const bz0 = p0[11] / 1000;
      const bx1 = currPacked[9] / 1000;
      const by1 = currPacked[10] / 1000;
      const bz1 = currPacked[11] / 1000;
      const ballTeleported = Math.hypot(bx1 - bx0, by1 - by0) > 6.5;

      ballX = ballTeleported ? bx1 : bx0 + (bx1 - bx0) * tInterp;
      ballY = ballTeleported ? by1 : by0 + (by1 - by0) * tInterp;
      ballZ = ballTeleported ? bz1 : bz0 + (bz1 - bz0) * tInterp;
      ballVx = (p0[12] + (currPacked[12] - p0[12]) * tInterp) / 1000;
      ballVy = (p0[13] + (currPacked[13] - p0[13]) * tInterp) / 1000;

      const ownerSide = currPacked[15];
      const ownerIdx = currPacked[16];

      if (this.prevBallZ > 0.28 && ballZ <= 0.06) {
        this.particles.spawnBurst(ballX, 0.04, ballY, ballVx * 0.15, ballVy * 0.15, 4);
      }
      this.prevBallZ = ballZ;

      this.ballMesh.position.set(ballX, this.ballRadius + Math.max(0, ballZ), ballY);
      const bSpeed = Math.hypot(ballVx, ballVy);
      if (bSpeed > 0.04) {
        const rollAxis = _vTargetOff.set(ballVy, 0, -ballVx).normalize();
        this.ballMesh.rotateOnWorldAxis(rollAxis, (bSpeed * dt) / this.ballRadius);
      }

      this.ballShadow.position.set(ballX, 0.018, ballY);
      const shadowShrink = Math.max(0.4, 1.0 - ballZ * 0.12);
      this.ballShadow.scale.set(shadowShrink, shadowShrink, 1);

      const showHudIndicators = this.viewMode === "match" && !(replayOpts && replayOpts.active);

      for (let n = 0; n < Math.min(pps * 2, this.actors.length); n++) {
        const b = 18 + n * 10;
        const side = currPacked[b + 0];
        const idx = currPacked[b + 1];

        const wx0 = p0[b + 4] / 1000;
        const wy0 = p0[b + 5] / 1000;
        const wx1 = currPacked[b + 4] / 1000;
        const wy1 = currPacked[b + 5] / 1000;
        const teleported = Math.hypot(wx1 - wx0, wy1 - wy0) > 4.0;

        const wx = teleported ? wx1 : wx0 + (wx1 - wx0) * tInterp;
        const wy = teleported ? wy1 : wy0 + (wy1 - wy0) * tInterp;

        const vx0 = p0[b + 6] / 1000;
        const vy0 = p0[b + 7] / 1000;
        const vx1 = currPacked[b + 6] / 1000;
        const vy1 = currPacked[b + 7] / 1000;
        const vx = teleported ? vx1 : vx0 + (vx1 - vx0) * tInterp;
        const vy = teleported ? vy1 : vy0 + (vy1 - vy0) * tInterp;

        // Clean 60Hz tick-step acceleration (immune to variable browser rAF dt!)
        const axTick = teleported ? 0 : (vx1 - vx0) * 60;
        const ayTick = teleported ? 0 : (vy1 - vy0) * 60;

        const f0 = (p0[b + 8] * Math.PI * 2) / 65536;
        const f1 = (currPacked[b + 8] * Math.PI * 2) / 65536;
        const facingRad = teleported ? f1 : lerpAngleRad(f0, f1, tInterp);

        const flags = currPacked[b + 9];
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
            axTick,
            ayTick,
            facingRad,
            action,
            charge,
            isControlled,
            isBallCarrier,
            ballX,
            ballY,
            celebrating,
          },
          dt,
          nowSec,
          replayOpts && replayOpts.active ? null : this.particles
        );
      }
    }

    // --- EA FIFA 10–12 Zoomed-Out Gantry Camera Controller (with maath.damp3) ---
    // Velocity-predicted play focus damped smoothly so ping-pong passes flow across screen without camera whip
    const focusGoalX = ballX + Math.max(-2.8, Math.min(2.8, ballVx * 0.16));
    const focusGoalZ = ballY + Math.max(-1.8, Math.min(1.8, ballVy * 0.12));
    damp3(this.smoothPlayFocus, [focusGoalX, 0, focusGoalZ], 0.24, dt);

    const fx = this.smoothPlayFocus.x;
    const fz = this.smoothPlayFocus.z;
    const zScale = this.userZoom;

    let targetPos;
    let targetLook;
    let desiredFov = 30.5;
    let camSmoothTime = 0.22;

    if (replayOpts && replayOpts.active) {
      camSmoothTime = 0.12;
      desiredFov = 29;
      if (replayOpts.camMode === "broadcast") {
        targetPos = [ballX * 0.55, 16.5 * zScale, (halfW + 19.0) * zScale];
        targetLook = [ballX * 0.75, 0.75, ballY * 0.35];
      } else if (replayOpts.camMode === "closeup") {
        const ang = replayOpts.orbitAngle;
        targetPos = [ballX + Math.sin(ang) * 6.4, 2.35, ballY + Math.cos(ang) * 6.4];
        targetLook = [ballX, 1.0, ballY];
      } else {
        const r = (pitchLen > 80 ? 19.5 : 13.5) * zScale;
        const ang = replayOpts.orbitAngle;
        targetPos = [ballX + Math.sin(ang) * r, 5.8 * zScale, ballY + Math.cos(ang) * r];
        targetLook = [ballX, 0.85, ballY];
      }
    } else if (this.viewMode === "attract" || this.viewMode === "squad") {
      this.attractAngle += dt * 0.13;
      desiredFov = 31;
      const radX = halfL * 0.52;
      const eyeX = Math.sin(this.attractAngle) * radX;
      const eyeZ = halfW + 17.5 + Math.cos(this.attractAngle * 0.7) * 4.0;
      targetPos = [eyeX, 14.8, eyeZ];
      targetLook = [fx * 0.4, 0.8, fz * 0.2 - 1.2];
    } else {
      const is11 = pitchLen > 80;
      const clampX = halfL * (is11 ? 0.54 : 0.36);
      const leadX = Math.max(-clampX, Math.min(clampX, fx * (is11 ? 0.62 : 0.44)));

      if (this.cameraMode === "dynamic") {
        desiredFov = 30;
        const heightY = (is11 ? 15.8 : 13.2) * zScale;
        const distZ = (is11 ? 29.5 : 23.8) * zScale;
        targetPos = [leadX, heightY, fz * 0.32 + distZ];
        targetLook = [fx * 0.74, 0.72, fz * 0.35 - 0.8];
      } else if (this.cameraMode === "pro") {
        desiredFov = 35;
        targetPos = [fx - (is11 ? 24.5 : 18.8) * zScale, (is11 ? 13.5 : 10.6) * zScale, fz * 0.58];
        targetLook = [fx + 9.5, 0.8, fz * 0.4];
      } else if (this.cameraMode === "wide") {
        // Classic FIFA Co-Op Tactical (full goal-to-goal tactical overview!)
        desiredFov = 31.5;
        const heightY = (is11 ? 37.0 : 29.2) * zScale;
        const distZ = (is11 ? 59.5 : 48.0) * zScale;
        targetPos = [leadX * 0.38, heightY, distZ];
        targetLook = [leadX * 0.42, 0.45, -1.0];
      } else {
        // Default EA FIFA 10–12 Tele Broadcast Gantry (frames both penalty arcs & passing lanes!)
        desiredFov = 30.5;
        const heightY = (is11 ? 31.5 : 24.8) * zScale;
        const distZ = (is11 ? 52.0 : 41.5) * zScale;
        targetPos = [leadX * 0.85, heightY, fz * 0.18 + distZ];
        targetLook = [leadX * 0.9, 0.52, fz * 0.18 - 1.15];
      }
    }

    if (Math.abs(this.camera.fov - desiredFov) > 0.02) {
      damp(this.camera, "fov", desiredFov, 0.16, dt);
      this.camera.updateProjectionMatrix();
    }

    damp3(this.camPos, targetPos, camSmoothTime, dt);
    damp3(this.camLook, targetLook, camSmoothTime, dt);
    this.camera.position.copy(this.camPos);
    this.camera.lookAt(this.camLook);

    this._presentScene();
  }
}
