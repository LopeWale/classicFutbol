// Classic FIFA-Inspired 3D WebGL Match & Showcase Engine (Three.js)
// Renders a full 4-sided floodlit 3D stadium, mowed turf pitch, Telstar 3D ball,
// and 22-joint articulated 3D footballers with procedural skeletal animations,
// bespoke club kits, squad numbers, surnames, and multiple broadcast camera modes.

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

// Curated likeness presets for the official 44-player Premier League squads
const PLAYER_LIKENESS = {
  "mun-gk-1": { skin: 5, hair: 0, style: "fade" }, // Onana
  "mun-def-1": { skin: 1, hair: 1, style: "crop" }, // Martinez
  "mun-def-2": { skin: 1, hair: 1, style: "fade" }, // Dalot
  "mun-def-3": { skin: 0, hair: 2, style: "swept" }, // Maguire
  "mun-def-4": { skin: 1, hair: 2, style: "crop" }, // Mazraoui
  "mun-mid-1": { skin: 1, hair: 1, style: "crop" }, // Bruno
  "mun-mid-2": { skin: 4, hair: 0, style: "curls" }, // Mainoo
  "mun-mid-3": { skin: 2, hair: 0, style: "crop" }, // Casemiro
  "mun-fwd-1": { skin: 4, hair: 0, style: "fade" }, // Rashford
  "mun-fwd-2": { skin: 0, hair: 4, style: "swept" }, // Garnacho
  "mun-fwd-3": { skin: 0, hair: 4, style: "swept" }, // Hojlund

  "ars-gk-1": { skin: 1, hair: 1, style: "fade" }, // Raya
  "ars-def-1": { skin: 4, hair: 0, style: "buzz" }, // Saliba
  "ars-def-2": { skin: 3, hair: 0, style: "crop" }, // Gabriel
  "ars-def-3": { skin: 1, hair: 2, style: "swept" }, // White
  "ars-def-4": { skin: 3, hair: 0, style: "curls" }, // Timber
  "ars-mid-1": { skin: 0, hair: 4, style: "swept" }, // Odegaard
  "ars-mid-2": { skin: 0, hair: 2, style: "swept" }, // Rice
  "ars-mid-3": { skin: 4, hair: 0, style: "fade" }, // Partey
  "ars-fwd-1": { skin: 5, hair: 0, style: "curls" }, // Saka
  "ars-fwd-2": { skin: 0, hair: 2, style: "swept" }, // Havertz
  "ars-fwd-3": { skin: 1, hair: 1, style: "crop" }, // Martinelli

  "liv-gk-1": { skin: 1, hair: 2, style: "swept" }, // Alisson
  "liv-def-1": { skin: 3, hair: 0, style: "bun" }, // Van Dijk
  "liv-def-2": { skin: 4, hair: 0, style: "curls" }, // Trent
  "liv-def-3": { skin: 4, hair: 0, style: "fade" }, // Konate
  "liv-def-4": { skin: 0, hair: 2, style: "crop" }, // Robertson
  "liv-mid-1": { skin: 1, hair: 1, style: "swept" }, // Mac Allister
  "liv-mid-2": { skin: 4, hair: 0, style: "curls" }, // Gravenberch
  "liv-mid-3": { skin: 0, hair: 1, style: "swept" }, // Szoboszlai
  "liv-fwd-1": { skin: 2, hair: 0, style: "curls" }, // Salah
  "liv-fwd-2": { skin: 2, hair: 0, style: "fade" }, // Diaz
  "liv-fwd-3": { skin: 1, hair: 1, style: "bun" }, // Nunez

  "che-gk-1": { skin: 2, hair: 0, style: "fade" }, // Sanchez
  "che-def-1": { skin: 3, hair: 0, style: "curls" }, // Colwill
  "che-def-2": { skin: 4, hair: 0, style: "curls" }, // James
  "che-def-3": { skin: 1, hair: 1, style: "curls" }, // Cucurella
  "che-def-4": { skin: 5, hair: 0, style: "fade" }, // Fofana
  "che-mid-1": { skin: 0, hair: 3, style: "crop" }, // Palmer
  "che-mid-2": { skin: 4, hair: 0, style: "fade" }, // Caicedo
  "che-mid-3": { skin: 1, hair: 1, style: "fade" }, // Enzo
  "che-fwd-1": { skin: 5, hair: 0, style: "fade" }, // Jackson
  "che-fwd-2": { skin: 4, hair: 0, style: "curls" }, // Nkunku
  "che-fwd-3": { skin: 1, hair: 2, style: "swept" }, // Neto
};

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

// --- Procedural Texture Builders ---

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
  return tex;
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

  // Outer pitch apron grass
  ctx.fillStyle = "#14532d";
  ctx.fillRect(0, 0, W, H);

  const halfL = pitchLen * 0.5;
  const halfW = pitchWid * 0.5;
  const pLeft = pxX(-halfL);
  const pTop = pxZ(-halfW);
  const pW = pitchLen * scaleX;
  const pH = pitchWid * scaleZ;

  // Lush alternating vertical mowed grass stripes
  const stripeCount = pitchLen > 80 ? 18 : 12;
  const stripeM = pitchLen / stripeCount;
  for (let i = 0; i < stripeCount; i++) {
    const x0 = pxX(-halfL + i * stripeM);
    const x1 = pxX(-halfL + (i + 1) * stripeM);
    ctx.fillStyle = i % 2 === 0 ? "#1d7c3a" : "#248f44";
    ctx.fillRect(x0, pTop, x1 - x0 + 0.8, pH);
  }

  // Subtle cross-cut horizontal mower sheen
  const crossRows = pitchWid > 50 ? 10 : 6;
  const rowH = pH / crossRows;
  for (let r = 0; r < crossRows; r++) {
    if (r % 2 === 0) {
      ctx.fillStyle = "rgba(255, 255, 255, 0.025)";
      ctx.fillRect(pLeft, pTop + r * rowH, pW, rowH);
    }
  }

  // Fine organic grass blade micro-texture (deterministic PRNG)
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

  // Subtle goalmouth wear patches
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

  // Crisp chalk-white pitch markings
  const linePx = Math.max(4.2, scaleX * 0.13);
  ctx.strokeStyle = "rgba(248, 250, 252, 0.94)";
  ctx.fillStyle = "rgba(248, 250, 252, 0.96)";
  ctx.lineWidth = linePx;
  ctx.lineJoin = "miter";

  // Touchlines & goal lines
  ctx.strokeRect(pLeft, pTop, pW, pH);

  // Halfway line
  ctx.beginPath();
  ctx.moveTo(pxX(0), pTop);
  ctx.lineTo(pxX(0), pTop + pH);
  ctx.stroke();

  // Center circle & spot
  const circleR = (fmt.center_circle_radius_mm || 6000) / 1000;
  ctx.beginPath();
  ctx.ellipse(pxX(0), pxZ(0), circleR * scaleX, circleR * scaleZ, 0, 0, Math.PI * 2);
  ctx.stroke();

  ctx.beginPath();
  ctx.ellipse(pxX(0), pxZ(0), linePx * 1.45, linePx * 1.45, 0, 0, Math.PI * 2);
  ctx.fill();

  // Penalty boxes, 6-yard goal areas, penalty spots, and D-arcs
  const boxD = (fmt.penalty_box_depth_mm || 9000) / 1000;
  const boxHW = (fmt.penalty_box_width_mm || 18000) / 2000;
  const goalHW = (fmt.goal_width_mm || 4000) / 2000;
  const sixD = Math.min(boxD * 0.42, 5.5);
  const sixHW = goalHW + (pitchLen > 80 ? 5.5 : 2.2);

  for (const sign of [-1, 1]) {
    const goalLineX = sign * halfL;
    const boxEdgeX = goalLineX - sign * boxD;
    const xMin = Math.min(pxX(goalLineX), pxX(boxEdgeX));
    const xSpan = Math.abs(pxX(goalLineX) - pxX(boxEdgeX));
    ctx.strokeRect(xMin, pxZ(-boxHW), xSpan, boxHW * 2 * scaleZ);

    // Goal area (keeper box)
    const sixEdgeX = goalLineX - sign * sixD;
    const sxMin = Math.min(pxX(goalLineX), pxX(sixEdgeX));
    const sxSpan = Math.abs(pxX(goalLineX) - pxX(sixEdgeX));
    ctx.strokeRect(sxMin, pxZ(-sixHW), sxSpan, sixHW * 2 * scaleZ);

    // Penalty spot
    const penDist = boxD * 0.68;
    const penX = goalLineX - sign * penDist;
    ctx.beginPath();
    ctx.ellipse(pxX(penX), pxZ(0), linePx * 1.25, linePx * 1.25, 0, 0, Math.PI * 2);
    ctx.fill();

    // Penalty D-arc outside penalty box
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

  // 4 Corner arcs
  const cornerR = 1.0;
  const corners = [
    [-halfL, -halfW, 0, Math.PI * 0.5],
    [halfL, -halfW, Math.PI * 0.5, Math.PI],
    [halfL, halfW, Math.PI, Math.PI * 1.5],
    [-halfL, halfW, Math.PI * 1.5, Math.PI * 2],
  ];
  for (const [cx, cz, a0, a1] of corners) {
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

  // Technical area dashed coaching boxes on far touchline
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
  ctx.strokeStyle = "rgba(241, 245, 249, 0.72)";
  ctx.lineWidth = 2.1;
  const step = 16;
  for (let i = -128; i <= 256; i += step) {
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

  // Classic FIFA Telstar black pentagon spots + subtle panel seams
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
  return tex;
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
  return new THREE.CanvasTexture(c);
}

function createJerseyTextures(kit, shirtNumber, surname, clubShort) {
  const makeSide = (isBack) => {
    const c = document.createElement("canvas");
    c.width = 256;
    c.height = 256;
    const ctx = c.getContext("2d");

    ctx.fillStyle = kit.shirt;
    ctx.fillRect(0, 0, 256, 256);

    // Subtle fabric weave & pattern
    if (kit.pattern === "pinstripes") {
      ctx.fillStyle = "rgba(255,255,255,0.12)";
      for (let x = 20; x < 240; x += 24) {
        ctx.fillRect(x, 0, 4, 256);
      }
    } else if (kit.pattern === "shoulder_stripes") {
      ctx.fillStyle = kit.trim;
      for (let s = 0; s < 3; s++) {
        ctx.fillRect(18 + s * 11, 0, 6, 46);
        ctx.fillRect(216 + s * 11, 0, 6, 46);
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

    // Side seam shading
    const shade = ctx.createLinearGradient(0, 0, 256, 0);
    shade.addColorStop(0, "rgba(0,0,0,0.22)");
    shade.addColorStop(0.16, "rgba(0,0,0,0)");
    shade.addColorStop(0.84, "rgba(0,0,0,0)");
    shade.addColorStop(1, "rgba(0,0,0,0.22)");
    ctx.fillStyle = shade;
    ctx.fillRect(0, 0, 256, 256);

    // Collar trim
    ctx.fillStyle = kit.trim;
    if (isBack) {
      ctx.fillRect(72, 0, 112, 16);
    } else {
      ctx.beginPath();
      ctx.moveTo(76, 0);
      ctx.lineTo(128, 38);
      ctx.lineTo(180, 0);
      ctx.lineTo(164, 0);
      ctx.lineTo(128, 24);
      ctx.lineTo(92, 0);
      ctx.closePath();
      ctx.fill();

      // Club crest shield on left chest (viewer right)
      ctx.fillStyle = kit.trim;
      ctx.beginPath();
      ctx.arc(172, 68, 16, 0, Math.PI * 2);
      ctx.fill();
      ctx.fillStyle = kit.shirt;
      ctx.font = "900 11px Arial, sans-serif";
      ctx.textAlign = "center";
      ctx.fillText((clubShort || "FC").slice(0, 3), 172, 72);

      // Kit maker mark on right chest
      ctx.fillStyle = kit.trim;
      ctx.fillRect(74, 62, 20, 9);

      // Classic chest sponsor bar
      ctx.fillStyle = kit.numberColor;
      ctx.font = "900 24px 'Arial Narrow', Impact, sans-serif";
      ctx.textAlign = "center";
      ctx.fillText("PARALLEL", 128, 134);
      ctx.font = "700 13px Arial, sans-serif";
      ctx.fillText("SEASON FC", 128, 152);
    }

    if (isBack) {
      const cleanName = (surname || "").slice(0, 11);
      if (cleanName) {
        ctx.font = "900 28px 'Arial Narrow', Impact, sans-serif";
        ctx.textAlign = "center";
        ctx.lineWidth = 5;
        ctx.strokeStyle = kit.numberOutline;
        ctx.strokeText(cleanName, 128, 62);
        ctx.fillStyle = kit.numberColor;
        ctx.fillText(cleanName, 128, 62);
      }

      const numStr = String(shirtNumber || 9);
      ctx.font = "900 118px 'Arial Narrow', Impact, sans-serif";
      ctx.textAlign = "center";
      ctx.lineWidth = 9;
      ctx.strokeStyle = kit.numberOutline;
      ctx.strokeText(numStr, 128, 178);
      ctx.fillStyle = kit.numberColor;
      ctx.fillText(numStr, 128, 178);
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

// --- Shared Geometries (Built once for fast zero-hitch actor instantiation) ---

const SHARED_GEO = (() => {
  const torso = new THREE.BoxGeometry(0.41, 0.52, 0.24, 2, 2, 2);
  const shoulders = new THREE.CapsuleGeometry(0.115, 0.26, 6, 12);
  shoulders.rotateZ(Math.PI * 0.5);

  const pelvis = new THREE.BoxGeometry(0.37, 0.21, 0.23);
  const neck = new THREE.CylinderGeometry(0.068, 0.078, 0.1, 10);
  const head = new THREE.SphereGeometry(0.142, 16, 14);
  head.scale(0.92, 1.08, 0.98);

  const nose = new THREE.SphereGeometry(0.021, 8, 8);
  nose.scale(0.85, 1.0, 1.15);
  const eye = new THREE.SphereGeometry(0.016, 8, 6);
  const ear = new THREE.SphereGeometry(0.028, 8, 6);
  ear.scale(0.5, 1.0, 0.7);

  const sleeve = new THREE.CylinderGeometry(0.078, 0.068, 0.24, 10);
  const sleeveCuff = new THREE.CylinderGeometry(0.07, 0.07, 0.032, 10);
  const forearm = new THREE.CapsuleGeometry(0.052, 0.19, 6, 10);
  const hand = new THREE.SphereGeometry(0.052, 10, 8);
  const gkGlove = new THREE.BoxGeometry(0.095, 0.13, 0.055);

  const thighShort = new THREE.CylinderGeometry(0.105, 0.096, 0.25, 12);
  const thighSkin = new THREE.CylinderGeometry(0.088, 0.076, 0.16, 12);
  const calfSock = new THREE.CylinderGeometry(0.076, 0.056, 0.34, 12);
  const sockBand = new THREE.CylinderGeometry(0.079, 0.078, 0.05, 12);
  const boot = new THREE.BoxGeometry(0.115, 0.085, 0.26);
  const bootStripe = new THREE.BoxGeometry(0.12, 0.03, 0.11);

  const ring = new THREE.RingGeometry(0.58, 0.74, 36);
  ring.rotateX(-Math.PI * 0.5);
  const chargeArc = new THREE.RingGeometry(0.76, 0.9, 32);
  chargeArc.rotateX(-Math.PI * 0.5);

  const aimArrow = new THREE.ConeGeometry(0.16, 0.32, 4);
  aimArrow.rotateX(Math.PI * 0.5);

  const cursorCone = new THREE.ConeGeometry(0.16, 0.28, 4);
  cursorCone.rotateX(Math.PI); // Point downward at controlled footballer's head!

  const shadowDisc = new THREE.CircleGeometry(0.52, 20);
  shadowDisc.rotateX(-Math.PI * 0.5);

  return {
    torso,
    shoulders,
    pelvis,
    neck,
    head,
    nose,
    eye,
    ear,
    sleeve,
    sleeveCuff,
    forearm,
    hand,
    gkGlove,
    thighShort,
    thighSkin,
    calfSock,
    sockBand,
    boot,
    bootStripe,
    ring,
    chargeArc,
    aimArrow,
    cursorCone,
    shadowDisc,
  };
})();

function buildHairMesh(style, hairColorHex) {
  const mat = new THREE.MeshStandardMaterial({
    color: hairColorHex,
    roughness: 0.82,
    metalness: 0.04,
  });
  const group = new THREE.Group();

  if (style === "buzz") {
    const cap = new THREE.SphereGeometry(0.144, 14, 10, 0, Math.PI * 2, 0, Math.PI * 0.56);
    cap.scale(0.93, 1.07, 0.99);
    const m = new THREE.Mesh(cap, mat);
    m.position.y = 0.01;
    group.add(m);
  } else if (style === "curls") {
    const cap = new THREE.SphereGeometry(0.152, 14, 12, 0, Math.PI * 2, 0, Math.PI * 0.62);
    const m = new THREE.Mesh(cap, mat);
    m.position.set(0, 0.025, -0.01);
    group.add(m);
    for (let i = 0; i < 9; i++) {
      const puff = new THREE.Mesh(new THREE.SphereGeometry(0.048, 8, 8), mat);
      const ang = (i / 9) * Math.PI * 2;
      puff.position.set(Math.cos(ang) * 0.09, 0.11 + (i % 2) * 0.02, Math.sin(ang) * 0.09);
      group.add(puff);
    }
  } else if (style === "swept") {
    const cap = new THREE.SphereGeometry(0.149, 14, 12, 0, Math.PI * 2, 0, Math.PI * 0.62);
    cap.scale(0.95, 1.08, 1.04);
    const m = new THREE.Mesh(cap, mat);
    m.position.set(0, 0.02, -0.008);
    group.add(m);
    const quiff = new THREE.Mesh(new THREE.BoxGeometry(0.16, 0.055, 0.12), mat);
    quiff.position.set(0.01, 0.135, 0.055);
    quiff.rotation.x = -0.25;
    quiff.rotation.z = -0.12;
    group.add(quiff);
  } else if (style === "bun") {
    const cap = new THREE.SphereGeometry(0.147, 14, 12, 0, Math.PI * 2, 0, Math.PI * 0.6);
    cap.scale(0.94, 1.08, 1.0);
    group.add(new THREE.Mesh(cap, mat));
    const bun = new THREE.Mesh(new THREE.SphereGeometry(0.052, 10, 10), mat);
    bun.position.set(0, 0.09, -0.145);
    group.add(bun);
  } else if (style === "fade") {
    const top = new THREE.CylinderGeometry(0.115, 0.128, 0.075, 14);
    const m = new THREE.Mesh(top, mat);
    m.position.set(0, 0.122, 0);
    group.add(m);
  } else {
    // Classic crop
    const cap = new THREE.SphereGeometry(0.148, 14, 12, 0, Math.PI * 2, 0, Math.PI * 0.58);
    cap.scale(0.94, 1.08, 1.0);
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
    };
  }
  const h = hashStr(footballerId || String(fallbackSeed));
  const styles = ["crop", "fade", "curls", "swept", "buzz"];
  return {
    skinHex: SKIN_TONES[h % SKIN_TONES.length],
    hairHex: HAIR_COLORS[(h >>> 3) % HAIR_COLORS.length],
    style: styles[(h >>> 6) % styles.length],
  };
}

// --- Articulated 3D Footballer Rig & Procedural Animator ---

function createFootballerActor(spec) {
  const { kit, shirtNumber, surname, footballerId, clubShort, isGk, side, scale = 1.12 } = spec;
  const likeness = resolveLikeness(footballerId, shirtNumber + side * 31);

  const skinMat = new THREE.MeshStandardMaterial({ color: likeness.skinHex, roughness: 0.52 });
  const shirtPlainMat = new THREE.MeshStandardMaterial({ color: kit.shirt, roughness: 0.68 });
  const sleeveMat = new THREE.MeshStandardMaterial({ color: kit.sleeves, roughness: 0.68 });
  const trimMat = new THREE.MeshStandardMaterial({ color: kit.trim, roughness: 0.55 });
  const shortsMat = new THREE.MeshStandardMaterial({ color: kit.shorts, roughness: 0.68 });
  const sockMat = new THREE.MeshStandardMaterial({ color: kit.socks, roughness: 0.72 });
  const sockBandMat = new THREE.MeshStandardMaterial({ color: kit.sockBand, roughness: 0.65 });
  const bootMat = new THREE.MeshStandardMaterial({ color: kit.boots, roughness: 0.34, metalness: 0.12 });
  const bootAccentMat = new THREE.MeshStandardMaterial({ color: kit.bootAccent, roughness: 0.35 });
  const darkEyeMat = new THREE.MeshBasicMaterial({ color: "#111827" });

  const jerseyTex = createJerseyTextures(kit, shirtNumber, surname, clubShort);
  const torsoMaterials = [
    shirtPlainMat, // +x right side
    shirtPlainMat, // -x left side
    shirtPlainMat, // +y top
    shirtPlainMat, // -y bottom
    new THREE.MeshStandardMaterial({ map: jerseyTex.front, roughness: 0.66 }), // +z chest front!
    new THREE.MeshStandardMaterial({ map: jerseyTex.back, roughness: 0.66 }), // -z jersey back!
  ];

  const root = new THREE.Group();
  const bodyScaleGroup = new THREE.Group();
  bodyScaleGroup.scale.setScalar(scale);
  root.add(bodyScaleGroup);

  // Soft ground contact shadow disc directly beneath boots
  const contactShadow = new THREE.Mesh(
    SHARED_GEO.shadowDisc,
    new THREE.MeshBasicMaterial({
      color: "#020617",
      transparent: true,
      opacity: 0.34,
      depthWrite: false,
    })
  );
  contactShadow.position.y = 0.015;
  root.add(contactShadow);

  // Classic FIFA On-Pitch Player Indicator Ring + Aim Wedge + Overhead Floating Cursor
  const ringColor = side === 0 ? "#fbbf24" : "#38bdf8";
  const ringMat = new THREE.MeshBasicMaterial({
    color: ringColor,
    transparent: true,
    opacity: 0.92,
    side: THREE.DoubleSide,
    depthWrite: false,
  });
  const ctrlRing = new THREE.Mesh(SHARED_GEO.ring, ringMat);
  ctrlRing.position.y = 0.025;
  ctrlRing.visible = false;
  root.add(ctrlRing);

  const aimWedge = new THREE.Mesh(SHARED_GEO.aimArrow, ringMat);
  aimWedge.position.set(0, 0.03, 0.86);
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

  const cursorMat = new THREE.MeshBasicMaterial({ color: ringColor });
  const overheadCursor = new THREE.Mesh(SHARED_GEO.cursorCone, cursorMat);
  overheadCursor.position.set(0, 2.38, 0);
  overheadCursor.visible = false;
  root.add(overheadCursor);

  // Crisp floating nameplate sprite (shown when controlled or carrying ball)
  const nameplateTex = createNameplateSpriteTexture(surname, shirtNumber, true, side);
  const nameplateMat = new THREE.SpriteMaterial({
    map: nameplateTex,
    transparent: true,
    depthTest: false,
  });
  const nameplate = new THREE.Sprite(nameplateMat);
  nameplate.scale.set(2.35, 0.58, 1);
  nameplate.position.set(0, 2.74, 0);
  nameplate.visible = false;
  root.add(nameplate);

  // Pelvis / Hips root (0.93m off ground in neutral standing pose)
  const pelvis = new THREE.Group();
  pelvis.position.set(0, 0.93, 0);
  bodyScaleGroup.add(pelvis);

  const shortsWaist = new THREE.Mesh(SHARED_GEO.pelvis, shortsMat);
  shortsWaist.castShadow = true;
  pelvis.add(shortsWaist);

  // Spine & Upper Body
  const spine = new THREE.Group();
  spine.position.set(0, 0.09, 0);
  pelvis.add(spine);

  const torso = new THREE.Mesh(SHARED_GEO.torso, torsoMaterials);
  torso.position.set(0, 0.26, 0);
  torso.castShadow = true;
  torso.receiveShadow = true;
  spine.add(torso);

  const collarRing = new THREE.Mesh(
    new THREE.CylinderGeometry(0.098, 0.112, 0.042, 12),
    trimMat
  );
  collarRing.position.set(0, 0.525, 0.005);
  spine.add(collarRing);

  const shoulderBar = new THREE.Mesh(SHARED_GEO.shoulders, sleeveMat);
  shoulderBar.position.set(0, 0.48, 0);
  shoulderBar.castShadow = true;
  spine.add(shoulderBar);

  // Neck + Sculpted Head + Face + Hair
  const neck = new THREE.Mesh(SHARED_GEO.neck, skinMat);
  neck.position.set(0, 0.56, 0);
  spine.add(neck);

  const headPivot = new THREE.Group();
  headPivot.position.set(0, 0.62, 0);
  spine.add(headPivot);

  const headMesh = new THREE.Mesh(SHARED_GEO.head, skinMat);
  headMesh.position.set(0, 0.1, 0);
  headMesh.castShadow = true;
  headPivot.add(headMesh);

  const nose = new THREE.Mesh(SHARED_GEO.nose, skinMat);
  nose.position.set(0, 0.095, 0.14);
  headPivot.add(nose);

  for (const sgn of [-1, 1]) {
    const eye = new THREE.Mesh(SHARED_GEO.eye, darkEyeMat);
    eye.position.set(sgn * 0.046, 0.118, 0.126);
    headPivot.add(eye);

    const ear = new THREE.Mesh(SHARED_GEO.ear, skinMat);
    ear.position.set(sgn * 0.132, 0.1, 0);
    headPivot.add(ear);
  }

  const hair = buildHairMesh(likeness.style, likeness.hairHex);
  hair.position.set(0, 0.1, 0);
  headPivot.add(hair);

  // Articulated Left & Right Arms (Shoulder -> Sleeve -> Elbow -> Forearm -> Hand / GK Glove)
  function buildArm(sideSign) {
    const shoulder = new THREE.Group();
    shoulder.position.set(sideSign * 0.255, 0.47, 0);
    spine.add(shoulder);

    const sleeve = new THREE.Mesh(SHARED_GEO.sleeve, sleeveMat);
    sleeve.position.set(0, -0.11, 0);
    sleeve.castShadow = true;
    shoulder.add(sleeve);

    const cuff = new THREE.Mesh(SHARED_GEO.sleeveCuff, trimMat);
    cuff.position.set(0, -0.225, 0);
    shoulder.add(cuff);

    const elbow = new THREE.Group();
    elbow.position.set(0, -0.24, 0);
    shoulder.add(elbow);

    const forearm = new THREE.Mesh(SHARED_GEO.forearm, isGk ? sleeveMat : skinMat);
    forearm.position.set(0, -0.12, 0);
    forearm.castShadow = true;
    elbow.add(forearm);

    const handMesh = isGk
      ? new THREE.Mesh(
          SHARED_GEO.gkGlove,
          new THREE.MeshStandardMaterial({ color: "#f8fafc", roughness: 0.45 })
        )
      : new THREE.Mesh(SHARED_GEO.hand, skinMat);
    handMesh.position.set(0, -0.25, 0.01);
    handMesh.castShadow = true;
    elbow.add(handMesh);

    return { shoulder, elbow };
  }

  const armL = buildArm(1);
  const armR = buildArm(-1);

  // Articulated Left & Right Legs (Hip -> Shorts/Thigh -> Knee -> Sock/Calf -> Football Boot)
  function buildLeg(sideSign) {
    const hip = new THREE.Group();
    hip.position.set(sideSign * 0.105, -0.05, 0);
    pelvis.add(hip);

    const shortLeg = new THREE.Mesh(SHARED_GEO.thighShort, shortsMat);
    shortLeg.position.set(0, -0.12, 0);
    shortLeg.castShadow = true;
    hip.add(shortLeg);

    const thighSkin = new THREE.Mesh(SHARED_GEO.thighSkin, skinMat);
    thighSkin.position.set(0, -0.31, 0);
    thighSkin.castShadow = true;
    hip.add(thighSkin);

    const knee = new THREE.Group();
    knee.position.set(0, -0.4, 0);
    hip.add(knee);

    const calf = new THREE.Mesh(SHARED_GEO.calfSock, sockMat);
    calf.position.set(0, -0.19, 0);
    calf.castShadow = true;
    knee.add(calf);

    const band = new THREE.Mesh(SHARED_GEO.sockBand, sockBandMat);
    band.position.set(0, -0.045, 0);
    knee.add(band);

    const boot = new THREE.Mesh(SHARED_GEO.boot, bootMat);
    boot.position.set(0, -0.4, 0.045);
    boot.castShadow = true;
    knee.add(boot);

    const stripe = new THREE.Mesh(SHARED_GEO.bootStripe, bootAccentMat);
    stripe.position.set(0, -0.39, 0.035);
    knee.add(stripe);

    return { hip, knee };
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
    stridePhase: (shirtNumber * 1.37) % (Math.PI * 2),
    prevAction: 0,
    kickSwingTicks: 0,
    smoothYaw: side === 0 ? Math.PI * 0.5 : -Math.PI * 0.5,
  };
}

function animateFootballerActor(actor, state, dt) {
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

  // Convert sim facingRad (0 = +X, pi/2 = +Z) into right-handed Three.js Y rotation (0 = +Z)
  const targetYaw = Math.PI * 0.5 - facingRad;
  let dyaw = targetYaw - actor.smoothYaw;
  while (dyaw > Math.PI) dyaw -= Math.PI * 2;
  while (dyaw < -Math.PI) dyaw += Math.PI * 2;
  actor.smoothYaw += dyaw * Math.min(1, dt * 18);

  const spinExtra = action === 7 ? (tick * 0.42) % (Math.PI * 2) : 0;
  actor.root.rotation.y = actor.smoothYaw + spinExtra;

  // Trigger kick follow-through swing when transitioning from Charging (1) to Cooldown (6) or Normal
  if (actor.prevAction === 1 && action !== 1) {
    actor.kickSwingTicks = 14;
  }
  actor.prevAction = action;
  if (actor.kickSwingTicks > 0) actor.kickSwingTicks--;

  const speed = Math.hypot(vx, vy);
  const speedNorm = Math.min(1.35, speed / 6.8);

  // Indicators & Nameplate visibility
  actor.ctrlRing.visible = isControlled;
  actor.aimWedge.visible = isControlled;
  actor.overheadCursor.visible = isControlled;
  if (isControlled) {
    actor.overheadCursor.position.y = 2.42 + Math.sin(tick * 0.14) * 0.07;
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

  // Reset baseline transforms before applying state-specific pose
  actor.pelvis.position.set(0, 0.93, 0);
  actor.pelvis.rotation.set(0, 0, -dyaw * 0.35);
  actor.spine.rotation.set(0, 0, 0);
  actor.armL.shoulder.rotation.set(0, 0, 0.14);
  actor.armR.shoulder.rotation.set(0, 0, -0.14);
  actor.armL.elbow.rotation.set(-0.35, 0, 0);
  actor.armR.elbow.rotation.set(-0.35, 0, 0);
  actor.legL.hip.rotation.set(0, 0, 0.03);
  actor.legR.hip.rotation.set(0, 0, -0.03);
  actor.legL.knee.rotation.set(0.08, 0, 0);
  actor.legR.knee.rotation.set(0.08, 0, 0);

  // Subtle head tracking toward the ball
  const toBallAng = Math.atan2(ballX - wx, ballY - wy) - actor.smoothYaw;
  let headYaw = toBallAng;
  while (headYaw > Math.PI) headYaw -= Math.PI * 2;
  while (headYaw < -Math.PI) headYaw += Math.PI * 2;
  actor.headPivot.rotation.y = Math.max(-0.62, Math.min(0.62, headYaw * 0.65));
  actor.headPivot.rotation.x = 0.06;

  // 1. Sliding Tackle Pose (action === 3)
  if (action === 3) {
    actor.pelvis.position.y = 0.28;
    actor.pelvis.rotation.x = -1.18;
    actor.spine.rotation.x = 0.32;
    actor.legR.hip.rotation.x = 0.95; // Extended slide leg forward
    actor.legR.knee.rotation.x = 0.08;
    actor.legL.hip.rotation.x = -0.35; // Tucked trail leg
    actor.legL.knee.rotation.x = 1.45;
    actor.armL.shoulder.rotation.set(-0.6, 0, 0.65);
    actor.armR.shoulder.rotation.set(-0.4, 0, -0.75);
    actor.contactShadow.scale.set(1.5, 1.5, 1);
    return;
  }

  // 2. Goalkeeper Airborne Dive Pose (action === 4)
  if (action === 4) {
    const lateral = vx * Math.sin(facingRad) - vy * Math.cos(facingRad);
    const diveSign = lateral >= 0 ? -1 : 1;
    actor.pelvis.position.y = 0.56;
    actor.pelvis.rotation.z = diveSign * 1.26;
    actor.spine.rotation.z = diveSign * 0.18;
    actor.armL.shoulder.rotation.set(-2.55, 0, 0.22);
    actor.armR.shoulder.rotation.set(-2.55, 0, -0.22);
    actor.armL.elbow.rotation.x = -0.18;
    actor.armR.elbow.rotation.x = -0.18;
    actor.legL.hip.rotation.z = 0.18;
    actor.legR.hip.rotation.z = -0.18;
    actor.legL.knee.rotation.x = 0.35;
    actor.legR.knee.rotation.x = 0.22;
    actor.contactShadow.scale.set(1.65, 1.2, 1);
    return;
  }

  actor.contactShadow.scale.set(1, 1, 1);

  // 3. Standing Lunge Tackle (action === 2)
  if (action === 2) {
    actor.pelvis.position.y = 0.78;
    actor.spine.rotation.x = 0.36;
    actor.legR.hip.rotation.x = -0.88;
    actor.legR.knee.rotation.x = 0.68;
    actor.legL.hip.rotation.x = 0.65;
    actor.legL.knee.rotation.x = 0.45;
    actor.armL.shoulder.rotation.set(-0.75, 0, 0.42);
    actor.armR.shoulder.rotation.set(0.55, 0, -0.38);
    return;
  }

  // 4. Stumbled / Off-balance (action === 5)
  if (action === 5) {
    actor.pelvis.position.y = 0.84;
    actor.spine.rotation.x = 0.48;
    actor.spine.rotation.z = Math.sin(tick * 0.4) * 0.25;
    actor.armL.shoulder.rotation.set(-0.9, 0, 0.75);
    actor.armR.shoulder.rotation.set(-0.9, 0, -0.75);
    return;
  }

  // 5. Goal Celebration Arms-Aloft Sprint!
  if (celebrating) {
    actor.stridePhase += dt * 11.5;
    const s = Math.sin(actor.stridePhase);
    actor.pelvis.position.y = 0.94 + Math.abs(Math.cos(actor.stridePhase)) * 0.08;
    actor.legL.hip.rotation.x = -s * 0.72;
    actor.legR.hip.rotation.x = s * 0.72;
    actor.legL.knee.rotation.x = Math.max(0.1, s * 0.95);
    actor.legR.knee.rotation.x = Math.max(0.1, -s * 0.95);
    actor.armL.shoulder.rotation.set(-2.35 + s * 0.2, 0, 0.45);
    actor.armR.shoulder.rotation.set(-2.35 - s * 0.2, 0, -0.45);
    actor.armL.elbow.rotation.x = -0.25;
    actor.armR.elbow.rotation.x = -0.25;
    return;
  }

  // 6. Locomotion (Idle Ready Stance vs Jog / Sprint Cycle)
  if (speed > 0.28) {
    actor.stridePhase += dt * (4.8 + speed * 1.65);
    const s = Math.sin(actor.stridePhase);
    const c = Math.cos(actor.stridePhase);
    const amp = Math.min(1.05, 0.35 + speedNorm * 0.68);

    // Double-frequency vertical flight bounce + forward sprint torso lean
    actor.pelvis.position.y = 0.91 + Math.abs(c) * 0.075 * amp;
    actor.spine.rotation.x = 0.08 + speedNorm * 0.24;
    actor.spine.rotation.y = s * 0.12 * amp;

    // In Three.js right-handed +Z forward: negative hip.rotation.x swings thigh FORWARD (+Z)
    actor.legL.hip.rotation.x = -s * 0.85 * amp;
    actor.legR.hip.rotation.x = s * 0.85 * amp;

    // Knee bends sharply during backward recovery swing
    actor.legL.knee.rotation.x = 0.14 + Math.max(0, s) * 1.22 * amp;
    actor.legR.knee.rotation.x = 0.14 + Math.max(0, -s) * 1.22 * amp;

    // Opposite arm pump with athletic bent elbows
    actor.armL.shoulder.rotation.x = s * 0.78 * amp;
    actor.armR.shoulder.rotation.x = -s * 0.78 * amp;
    actor.armL.elbow.rotation.x = -0.75 - speedNorm * 0.45;
    actor.armR.elbow.rotation.x = -0.75 - speedNorm * 0.45;
  } else if (actor.isGk) {
    // Goalkeeper athletic wide ready crouch
    const breathe = Math.sin(tick * 0.08 + actor.stridePhase) * 0.02;
    actor.pelvis.position.y = 0.81 + breathe;
    actor.spine.rotation.x = 0.26;
    actor.legL.hip.rotation.set(-0.28, 0, 0.22);
    actor.legR.hip.rotation.set(-0.28, 0, -0.22);
    actor.legL.knee.rotation.x = 0.56;
    actor.legR.knee.rotation.x = 0.56;
    actor.armL.shoulder.rotation.set(-0.58, 0, 0.42);
    actor.armR.shoulder.rotation.set(-0.58, 0, -0.42);
    actor.armL.elbow.rotation.x = -0.92;
    actor.armR.elbow.rotation.x = -0.92;
  } else {
    // Outfield athletic ready stance
    const breathe = Math.sin(tick * 0.06 + actor.stridePhase) * 0.015;
    actor.pelvis.position.y = 0.91 + breathe;
    actor.spine.rotation.x = 0.08;
    actor.legL.hip.rotation.set(-0.1, 0, 0.06);
    actor.legR.hip.rotation.set(0.06, 0, -0.06);
    actor.legL.knee.rotation.x = 0.2;
    actor.legR.knee.rotation.x = 0.16;
    actor.armL.shoulder.rotation.set(0.06, 0, 0.16);
    actor.armR.shoulder.rotation.set(-0.06, 0, -0.16);
    actor.armL.elbow.rotation.x = -0.48;
    actor.armR.elbow.rotation.x = -0.48;
  }

  // 7. Layered Upper/Lower Kick Wind-Up (Charging) or Follow-Through!
  if (action === 1 || charge > 0.03) {
    const w = Math.min(1, Math.max(0.25, charge));
    actor.legR.hip.rotation.x = 0.95 * w; // Cock right kicking leg back
    actor.legR.knee.rotation.x = 1.25 * w;
    actor.legL.hip.rotation.x = -0.32 * w; // Plant left leg firmly
    actor.legL.knee.rotation.x = 0.28;
    actor.spine.rotation.y = -0.32 * w;
    actor.armL.shoulder.rotation.set(-0.55 * w, 0, 0.68 * w); // Balance arm wide
  } else if (actor.kickSwingTicks > 0) {
    const k = actor.kickSwingTicks / 14;
    actor.legR.hip.rotation.x = -1.12 * k; // Whip right boot forward through the ball!
    actor.legR.knee.rotation.x = 0.08;
    actor.spine.rotation.x = -0.14 * k;
    actor.spine.rotation.y = 0.28 * k;
    actor.armL.shoulder.rotation.set(0.45 * k, 0, 0.45);
    actor.armR.shoulder.rotation.set(-0.65 * k, 0, -0.35);
  }
}

// --- Stadium & Grandstand Builder ---

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

  // Left & Right Goalposts + Crossbar
  const postGeo = new THREE.CylinderGeometry(postR, postR, goalH, 14);
  for (const zSign of [-1, 1]) {
    const post = new THREE.Mesh(postGeo, postMat);
    post.position.set(goalX, goalH * 0.5, zSign * goalHW);
    post.castShadow = true;
    group.add(post);

    // Rear stanchion & ground bar
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

  // Woven 3D Goal Nets (Back, Roof, Left Side, Right Side)
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
  const goalH = fmt.goal_height_mm / 1000;

  const kits = resolveMatchKits(
    snapshot.home.club_id,
    snapshot.away.club_id,
    snapshot.home.primary_color,
    snapshot.away.primary_color
  );

  // 1. Stadium Track / Outer Floor
  const trackFloor = new THREE.Mesh(
    new THREE.PlaneGeometry(pitchLen + 70, pitchWid + 70),
    new THREE.MeshStandardMaterial({ color: "#141c27", roughness: 0.92 })
  );
  trackFloor.rotation.x = -Math.PI * 0.5;
  trackFloor.position.y = -0.02;
  trackFloor.receiveShadow = true;
  stadium.add(trackFloor);

  // Classic clay/brick stadium perimeter track border
  const trackBorder = new THREE.Mesh(
    new THREE.PlaneGeometry(pitchLen + 17.5, pitchWid + 16.0),
    new THREE.MeshStandardMaterial({ color: "#3b231d", roughness: 0.88 })
  );
  trackBorder.rotation.x = -Math.PI * 0.5;
  trackBorder.position.y = -0.01;
  trackBorder.receiveShadow = true;
  stadium.add(trackBorder);

  // 2. High-DPI Mowed Grass Pitch Plane
  const { tex: pitchTex, totalW, totalH } = createPitchTexture(pitchLen, pitchWid, fmt, maxAnisotropy);
  const pitchMesh = new THREE.Mesh(
    new THREE.PlaneGeometry(totalW, totalH),
    new THREE.MeshStandardMaterial({
      map: pitchTex,
      roughness: 0.86,
      metalness: 0.02,
    })
  );
  pitchMesh.rotation.x = -Math.PI * 0.5;
  pitchMesh.receiveShadow = true;
  stadium.add(pitchMesh);

  // 3. Goals & Corner Flags
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

  // 4. Touchline Team Dugouts & Coaching Benches (North Touchline)
  const shelterMat = new THREE.MeshStandardMaterial({
    color: "#0f172a",
    roughness: 0.4,
    metalness: 0.4,
  });
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

    // 4 Seated Substitutes inside the dugout + 1 Manager standing in technical area
    for (let s = 0; s < 4; s++) {
      const subTorso = new THREE.Mesh(
        new THREE.BoxGeometry(0.34, 0.48, 0.22),
        new THREE.MeshStandardMaterial({ color: benchKit.shirt, roughness: 0.7 })
      );
      subTorso.position.set(-1.5 + s * 1.0, 0.78, -0.1);
      dugout.add(subTorso);
      const subHead = new THREE.Mesh(
        new THREE.SphereGeometry(0.12, 10, 8),
        new THREE.MeshStandardMaterial({ color: SKIN_TONES[(s + sideIdx * 2) % SKIN_TONES.length] })
      );
      subHead.position.set(-1.5 + s * 1.0, 1.14, -0.1);
      dugout.add(subHead);
    }
    stadium.add(dugout);
  }

  // 5. Illuminated 4-Sided LED Perimeter Advertising Boards
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

  // Far (North) & Near (South — lower profile so broadcast camera is unobstructed)
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

  // 6. Four-Sided Tiered Classic FIFA Stadium Bowl + Cantilevered Roof + Animated 3D Crowd!
  const concreteMat = new THREE.MeshStandardMaterial({ color: "#1e293b", roughness: 0.88 });
  const riserDarkMat = new THREE.MeshStandardMaterial({ color: "#0f172a", roughness: 0.92 });
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

    // Front concrete wall
    const wall = new THREE.Mesh(new THREE.BoxGeometry(length, 1.65, 0.6), riserDarkMat);
    wall.position.set(0, 0.825, -startDist);
    stand.add(wall);

    // Illuminated supporter ribbon along wall top
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

      // Populate spectators densely along this tier + yellow stadium aisle steps
      const spacing = 0.52;
      const count = Math.floor((length - 2.0) / spacing);
      for (let i = 0; i < count; i++) {
        const lx = -((count - 1) * spacing) * 0.5 + i * spacing;
        // Every 24 seats leave a stadium stairway aisle
        if (i % 24 === 12 || i % 24 === 13) {
          if (i % 24 === 12) {
            const stair = new THREE.Mesh(
              new THREE.BoxGeometry(0.95, stepRise + 0.18, stepDepth + 0.14),
              new THREE.MeshStandardMaterial({ color: "#ca8a04", roughness: 0.7 })
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

        let col;
        const r = cRand();
        if (r < 0.44) col = isHomeBias ? palette[0] : palette[2];
        else if (r < 0.62) col = isHomeBias ? palette[2] : palette[0];
        else col = palette[3 + Math.floor(cRand() * 4)];
        crowdColors.push(col);
      }
    }

    // Rear stadium wall & cantilevered roof canopy
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

  // North Stand (far side — primary broadcast backdrop!), West & East Goal Ends, and South Stand
  buildStand(standSpanX, adDistZ + 2.4, 0, 11, true, true);
  buildStand(standSpanZ, adDistX + 2.8, Math.PI * 0.5, 10, true, true);
  buildStand(standSpanZ, adDistX + 2.8, -Math.PI * 0.5, 10, true, false);
  buildStand(standSpanX, adDistZ + 6.5, Math.PI, 5, false, false);

  // Instanced 3D Crowd Mesh (single draw call for 4,500+ 3D spectators!)
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

  // 7. Four Towering Corner Floodlight Pylons + Glowing Bulb Banks
  const pylonMat = new THREE.MeshStandardMaterial({ color: "#475569", metalness: 0.6, roughness: 0.4 });
  const bulbMat = new THREE.MeshBasicMaterial({ color: "#fffbeb" });
  const towerH = pitchLen > 80 ? 34 : 26;
  const corners = [
    [-adDistX - 6.5, -adDistZ - 6.5],
    [adDistX + 6.5, -adDistZ - 6.5],
    [-adDistX - 6.5, adDistZ + 6.5],
    [adDistX + 6.5, adDistZ + 6.5],
  ];
  for (const [tx, tz] of corners) {
    const tower = new THREE.Group();
    tower.position.set(tx, 0, tz);
    const mast = new THREE.Mesh(new THREE.CylinderGeometry(0.32, 0.58, towerH, 8), pylonMat);
    mast.position.y = towerH * 0.5;
    tower.add(mast);

    const headFrame = new THREE.Mesh(new THREE.BoxGeometry(4.4, 2.4, 0.65), pylonMat);
    headFrame.position.y = towerH + 0.8;
    tower.lookAt(0, 0, 0);
    // Re-orient upright after horizontal yaw lookAt
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

// --- Public Engine Class ---

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

    this.camera = new THREE.PerspectiveCamera(30, 16 / 9, 0.4, 420);
    this.cameraMode = "tele"; // "tele" | "dynamic" | "pro" | "wide"
    this.viewMode = "attract"; // "attract" | "match" | "showcase" | "squad" | "replay"

    this.camPos = new THREE.Vector3(0, 17, 34);
    this.camLook = new THREE.Vector3(0, 0.8, -1.5);

    this.glowTex = createGlowSpriteTexture();
    this.maxAniso = this.renderer.capabilities.getMaxAnisotropy();

    // Sky dome
    const skyDome = new THREE.Mesh(
      new THREE.SphereGeometry(260, 28, 18),
      new THREE.MeshBasicMaterial({ map: createSkyTexture(), side: THREE.BackSide, fog: false })
    );
    this.scene.add(skyDome);

    // Rich Stadium Floodlight Rig
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

    // 3D Telstar Ball + Contact Ground Shadow + Power Shot Trail
    this.ballRadius = 0.22;
    this.ballMesh = new THREE.Mesh(
      new THREE.SphereGeometry(this.ballRadius, 26, 20),
      new THREE.MeshStandardMaterial({
        map: createTelstarBallTexture(),
        roughness: 0.32,
        metalness: 0.06,
      })
    );
    this.ballMesh.castShadow = true;
    this.scene.add(this.ballMesh);

    this.ballShadow = new THREE.Mesh(
      new THREE.CircleGeometry(0.3, 18),
      new THREE.MeshBasicMaterial({
        color: "#020617",
        transparent: true,
        opacity: 0.45,
        depthWrite: false,
      })
    );
    this.ballShadow.rotation.x = -Math.PI * 0.5;
    this.ballShadow.position.y = 0.018;
    this.scene.add(this.ballShadow);

    this.stadiumData = null;
    this.actors = [];
    this.showcaseGroup = new THREE.Group();
    this.showcaseGroup.visible = false;
    this.scene.add(this.showcaseGroup);

    this.netRipple = [0, 0];
    this.goalExcitement = 0;
    this.attractAngle = 0;
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
    // Home (0) attacks +X (right goal = 1), Away (1) attacks -X (left goal = 0) in 1H
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
    }
    for (const a of this.actors) {
      this.scene.remove(a.root);
    }
    this.actors = [];

    this.stadiumData = buildStadiumWorld(snapshot, this.maxAniso, this.glowTex);
    this.scene.add(this.stadiumData.stadium);

    // Adjust directional shadow frustum tightly to active pitch size
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

  /**
   * Build 2 hero 3D Captain Mannequins standing face-to-face on the center spot
   * for the Pre-Match Club & Kit Select screen!
   */
  updatePreMatchShowcase(homeRoster, awayRoster) {
    while (this.showcaseGroup.children.length > 0) {
      this.showcaseGroup.remove(this.showcaseGroup.children[0]);
    }
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

    if (!this.stadiumData) {
      this.renderer.render(this.scene, this.camera);
      return;
    }

    const { halfL, halfW, pitchLen, leftGoal, rightGoal, cornerFlags, crowdMesh, crowdPositions } =
      this.stadiumData;

    // Animate fluttering corner flags
    const waveT = now * 0.004;
    for (let i = 0; i < cornerFlags.length; i++) {
      cornerFlags[i].rotation.y = Math.sin(waveT + i * 1.4) * 0.28;
    }

    // Animate goal net ripple decay
    for (let g = 0; g < 2; g++) {
      if (this.netRipple[g] > 0.005) {
        this.netRipple[g] *= Math.pow(0.04, dt);
        const holder = g === 0 ? leftGoal.netHolder : rightGoal.netHolder;
        const bulge = Math.sin(now * 0.035) * this.netRipple[g] * 0.14;
        holder.scale.set(1 + Math.abs(bulge), 1 + bulge * 0.3, 1 + bulge * 0.4);
      }
    }

    // Subtle crowd bounce (amplified during Goal excitement!)
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
      // Pre-Match 3D Kit & Captain Showcase Camera (framed cleanly above the settings dock!)
      this.ballMesh.position.set(0, this.ballRadius, 0.85);
      this.ballMesh.rotation.y += dt * 0.8;
      this.ballShadow.position.set(0, 0.018, 0.85);

      if (this.showcaseCaptains) {
        const idleTick = Math.floor(now / 16);
        this.showcaseCaptains.forEach((cap, cIdx) => {
          const sgn = cIdx === 0 ? 1 : -1;
          cap.pelvis.position.y = 0.91 + Math.sin(now * 0.003 + cap.stridePhase) * 0.014;
          cap.spine.rotation.y = -sgn * 0.08;
          // Confident athletic wide stance + hands-near-hips / ready elbows
          cap.legL.hip.rotation.set(-0.08, 0, 0.09);
          cap.legR.hip.rotation.set(0.06, 0, -0.09);
          cap.legL.knee.rotation.x = 0.16;
          cap.legR.knee.rotation.x = 0.12;
          cap.armL.shoulder.rotation.set(0.14, 0, 0.34);
          cap.armR.shoulder.rotation.set(-0.12, 0, -0.34);
          cap.armL.elbow.rotation.set(-0.82, 0, -0.22);
          cap.armR.elbow.rotation.set(-0.78, 0, 0.22);
          cap.headPivot.rotation.y = Math.sin(idleTick * 0.02 + cap.stridePhase) * 0.1 - sgn * 0.08;
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

      // Position & roll the 3D Telstar ball
      this.ballMesh.position.set(ballX, this.ballRadius + Math.max(0, ballZ), ballY);
      const bSpeed = Math.hypot(ballVx, ballVy);
      if (bSpeed > 0.05) {
        const rollAxis = new THREE.Vector3(ballVy, 0, -ballVx).normalize();
        this.ballMesh.rotateOnWorldAxis(rollAxis, (bSpeed * dt) / this.ballRadius);
      }

      this.ballShadow.position.set(ballX, 0.018, ballY);
      const shadowShrink = Math.max(0.45, 1.0 - ballZ * 0.12);
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
          dt
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
        targetPos = new THREE.Vector3(ballX + Math.sin(ang) * 7.2, 2.9, ballY + Math.cos(ang) * 7.2);
        targetLook = new THREE.Vector3(ballX, 1.02, ballY);
      } else {
        // Orbit 360° Theater Cam
        const r = pitchLen > 80 ? 18.0 : 11.8;
        const ang = replayOpts.orbitAngle;
        targetPos = new THREE.Vector3(ballX + Math.sin(ang) * r, 5.4, ballY + Math.cos(ang) * r);
        targetLook = new THREE.Vector3(ballX, 0.9, ballY);
      }
    } else if (this.viewMode === "attract" || this.viewMode === "squad") {
      // Cinematic sweeping stadium camera behind Title / Main Menu / Team Management
      this.attractAngle += dt * 0.16;
      desiredFov = 32;
      const radX = halfL * 0.55;
      const eyeX = Math.sin(this.attractAngle) * radX;
      const eyeZ = halfW * 0.88 + 11.5 + Math.cos(this.attractAngle * 0.7) * 3.5;
      targetPos = new THREE.Vector3(eyeX, 11.5, eyeZ);
      targetLook = new THREE.Vector3(ballX * 0.45, 0.9, ballY * 0.25 - 1.5);
    } else {
      // Live Match Broadcast Cameras!
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
        // End-to-End Pro Camera looking from behind Home (-X) goal toward Away (+X) goal
        desiredFov = 36;
        targetPos = new THREE.Vector3(ballX - (is11 ? 21 : 15.5), is11 ? 11.5 : 8.6, ballY * 0.65);
        targetLook = new THREE.Vector3(ballX + 9.0, 0.9, ballY * 0.45);
      } else if (this.cameraMode === "wide") {
        desiredFov = 35;
        targetPos = new THREE.Vector3(leadX * 0.5, is11 ? 27.0 : 19.2, is11 ? 43.0 : 30.5);
        targetLook = new THREE.Vector3(leadX * 0.55, 0.5, -1.2);
      } else {
        // Default: Classic FIFA Tele Broadcast Camera!
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
