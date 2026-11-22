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
  "man-city": {
    home: {
      shirt: "#6cabdd",
      sleeves: "#6cabdd",
      trim: "#ffffff",
      shorts: "#ffffff",
      shortsStripe: "#6cabdd",
      socks: "#6cabdd",
      sockBand: "#ffffff",
      numberColor: "#1e293b",
      numberOutline: "#ffffff",
      pattern: "shoulder_stripes",
      boots: "#0f172a",
      bootAccent: "#38bdf8",
    },
    away: {
      shirt: "#111827",
      sleeves: "#111827",
      trim: "#facc15",
      shorts: "#111827",
      shortsStripe: "#ef4444",
      socks: "#111827",
      sockBand: "#facc15",
      numberColor: "#facc15",
      numberOutline: "#000000",
      pattern: "pinstripes",
      boots: "#facc15",
      bootAccent: "#111827",
    },
    gk: {
      shirt: "#22c55e",
      sleeves: "#16a34a",
      trim: "#ffffff",
      shorts: "#14532d",
      shortsStripe: "#4ade80",
      socks: "#22c55e",
      sockBand: "#ffffff",
      numberColor: "#ffffff",
      numberOutline: "#14532d",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#22c55e",
    },
    banner: "ETIHAD STADIUM · SOUTH STAND",
  },
  brighton: {
    home: {
      shirt: "#0057b8",
      sleeves: "#0057b8",
      trim: "#ffffff",
      shorts: "#ffffff",
      shortsStripe: "#0057b8",
      socks: "#0057b8",
      sockBand: "#ffffff",
      numberColor: "#facc15",
      numberOutline: "#0f172a",
      pattern: "vertical_stripes",
      boots: "#0f172a",
      bootAccent: "#38bdf8",
    },
    away: {
      shirt: "#facc15",
      sleeves: "#facc15",
      trim: "#0f172a",
      shorts: "#0f172a",
      shortsStripe: "#facc15",
      socks: "#facc15",
      sockBand: "#0f172a",
      numberColor: "#0f172a",
      numberOutline: "#ffffff",
      pattern: "pinstripes",
      boots: "#0f172a",
      bootAccent: "#facc15",
    },
    gk: {
      shirt: "#ec4899",
      sleeves: "#db2777",
      trim: "#ffffff",
      shorts: "#831843",
      shortsStripe: "#f472b6",
      socks: "#ec4899",
      sockBand: "#ffffff",
      numberColor: "#ffffff",
      numberOutline: "#831843",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#f472b6",
    },
    banner: "AMEX STADIUM · NORTH STAND",
  },
  bournemouth: {
    home: {
      shirt: "#da291c",
      sleeves: "#111827",
      trim: "#111827",
      shorts: "#111827",
      shortsStripe: "#da291c",
      socks: "#111827",
      sockBand: "#da291c",
      numberColor: "#ffffff",
      numberOutline: "#000000",
      pattern: "vertical_stripes",
      boots: "#111827",
      bootAccent: "#ef4444",
    },
    away: {
      shirt: "#f8fafc",
      sleeves: "#f8fafc",
      trim: "#7c3aed",
      shorts: "#312e81",
      shortsStripe: "#f8fafc",
      socks: "#f8fafc",
      sockBand: "#7c3aed",
      numberColor: "#1e1b4b",
      numberOutline: "#ffffff",
      pattern: "chevron_subtle",
      boots: "#312e81",
      bootAccent: "#ffffff",
    },
    gk: {
      shirt: "#10b981",
      sleeves: "#059669",
      trim: "#ffffff",
      shorts: "#064e3b",
      shortsStripe: "#34d399",
      socks: "#10b981",
      sockBand: "#ffffff",
      numberColor: "#ffffff",
      numberOutline: "#064e3b",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#10b981",
    },
    banner: "VITALITY STADIUM · DEAN COURT",
  },
  leeds: {
    home: {
      shirt: "#f8fafc",
      sleeves: "#f8fafc",
      trim: "#1d428a",
      shorts: "#f8fafc",
      shortsStripe: "#ffcd00",
      socks: "#f8fafc",
      sockBand: "#1d428a",
      numberColor: "#1d428a",
      numberOutline: "#ffffff",
      pattern: "shoulder_stripes",
      boots: "#0f172a",
      bootAccent: "#ffcd00",
    },
    away: {
      shirt: "#1d428a",
      sleeves: "#1d428a",
      trim: "#ffcd00",
      shorts: "#1d428a",
      shortsStripe: "#ffcd00",
      socks: "#1d428a",
      sockBand: "#ffcd00",
      numberColor: "#ffcd00",
      numberOutline: "#0f172a",
      pattern: "pinstripes",
      boots: "#ffcd00",
      bootAccent: "#1d428a",
    },
    gk: {
      shirt: "#f97316",
      sleeves: "#ea580c",
      trim: "#ffffff",
      shorts: "#7c2d12",
      shortsStripe: "#fb923c",
      socks: "#f97316",
      sockBand: "#ffffff",
      numberColor: "#ffffff",
      numberOutline: "#7c2d12",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#fb923c",
    },
    banner: "ELLAND ROAD · KOP STAND",
  },
  everton: {
    home: {
      shirt: "#003399",
      sleeves: "#003399",
      trim: "#ffffff",
      shorts: "#ffffff",
      shortsStripe: "#003399",
      socks: "#ffffff",
      sockBand: "#003399",
      numberColor: "#ffffff",
      numberOutline: "#0f172a",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#3b82f6",
    },
    away: {
      shirt: "#fef08a",
      sleeves: "#fef08a",
      trim: "#1e3a8a",
      shorts: "#1e3a8a",
      shortsStripe: "#fef08a",
      socks: "#fef08a",
      sockBand: "#1e3a8a",
      numberColor: "#1e3a8a",
      numberOutline: "#ffffff",
      pattern: "chevron_subtle",
      boots: "#1e3a8a",
      bootAccent: "#fef08a",
    },
    gk: {
      shirt: "#22c55e",
      sleeves: "#15803d",
      trim: "#ffffff",
      shorts: "#14532d",
      shortsStripe: "#4ade80",
      socks: "#22c55e",
      sockBand: "#ffffff",
      numberColor: "#ffffff",
      numberOutline: "#14532d",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#4ade80",
    },
    banner: "BRAMLEY-MOORE DOCK · MERSEYSIDE",
  },
  tottenham: {
    home: {
      shirt: "#f8fafc",
      sleeves: "#132257",
      trim: "#132257",
      shorts: "#132257",
      shortsStripe: "#ffffff",
      socks: "#f8fafc",
      sockBand: "#132257",
      numberColor: "#132257",
      numberOutline: "#ffffff",
      pattern: "solid",
      boots: "#132257",
      bootAccent: "#ffffff",
    },
    away: {
      shirt: "#0f172a",
      sleeves: "#0f172a",
      trim: "#38bdf8",
      shorts: "#0f172a",
      shortsStripe: "#38bdf8",
      socks: "#0f172a",
      sockBand: "#38bdf8",
      numberColor: "#38bdf8",
      numberOutline: "#000000",
      pattern: "pinstripes",
      boots: "#38bdf8",
      bootAccent: "#0f172a",
    },
    gk: {
      shirt: "#eab308",
      sleeves: "#ca8a04",
      trim: "#132257",
      shorts: "#713f12",
      shortsStripe: "#fde047",
      socks: "#eab308",
      sockBand: "#132257",
      numberColor: "#132257",
      numberOutline: "#ffffff",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#fde047",
    },
    banner: "NORTH LONDON · SOUTH STAND",
  },
  brentford: {
    home: {
      shirt: "#e30613",
      sleeves: "#ffffff",
      trim: "#ffffff",
      shorts: "#111827",
      shortsStripe: "#e30613",
      socks: "#e30613",
      sockBand: "#ffffff",
      numberColor: "#ffffff",
      numberOutline: "#111827",
      pattern: "vertical_stripes",
      boots: "#111827",
      bootAccent: "#ffffff",
    },
    away: {
      shirt: "#38bdf8",
      sleeves: "#38bdf8",
      trim: "#0f172a",
      shorts: "#0f172a",
      shortsStripe: "#38bdf8",
      socks: "#38bdf8",
      sockBand: "#0f172a",
      numberColor: "#0f172a",
      numberOutline: "#ffffff",
      pattern: "pinstripes",
      boots: "#0f172a",
      bootAccent: "#38bdf8",
    },
    gk: {
      shirt: "#a855f7",
      sleeves: "#7e22ce",
      trim: "#ffffff",
      shorts: "#3b0764",
      shortsStripe: "#c084fc",
      socks: "#a855f7",
      sockBand: "#ffffff",
      numberColor: "#ffffff",
      numberOutline: "#3b0764",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#c084fc",
    },
    banner: "GTECH COMMUNITY STADIUM · WEST LONDON",
  },
  "aston-villa": {
    home: {
      shirt: "#670e36",
      sleeves: "#95bfe5",
      trim: "#95bfe5",
      shorts: "#ffffff",
      shortsStripe: "#670e36",
      socks: "#95bfe5",
      sockBand: "#670e36",
      numberColor: "#ffffff",
      numberOutline: "#0f172a",
      pattern: "arsenal_sleeves",
      boots: "#111827",
      bootAccent: "#95bfe5",
    },
    away: {
      shirt: "#f8fafc",
      sleeves: "#f8fafc",
      trim: "#670e36",
      shorts: "#670e36",
      shortsStripe: "#95bfe5",
      socks: "#f8fafc",
      sockBand: "#670e36",
      numberColor: "#670e36",
      numberOutline: "#ffffff",
      pattern: "pinstripes",
      boots: "#670e36",
      bootAccent: "#ffffff",
    },
    gk: {
      shirt: "#10b981",
      sleeves: "#059669",
      trim: "#ffffff",
      shorts: "#064e3b",
      shortsStripe: "#34d399",
      socks: "#10b981",
      sockBand: "#ffffff",
      numberColor: "#ffffff",
      numberOutline: "#064e3b",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#34d399",
    },
    banner: "VILLA PARK · HOLTE END",
  },
  newcastle: {
    home: {
      shirt: "#18181b",
      sleeves: "#18181b",
      trim: "#ffffff",
      shorts: "#18181b",
      shortsStripe: "#ffffff",
      socks: "#18181b",
      sockBand: "#ffffff",
      numberColor: "#ef4444",
      numberOutline: "#ffffff",
      pattern: "vertical_stripes",
      boots: "#111827",
      bootAccent: "#ffffff",
    },
    away: {
      shirt: "#047857",
      sleeves: "#047857",
      trim: "#ffffff",
      shorts: "#ffffff",
      shortsStripe: "#047857",
      socks: "#047857",
      sockBand: "#ffffff",
      numberColor: "#ffffff",
      numberOutline: "#064e3b",
      pattern: "shoulder_stripes",
      boots: "#064e3b",
      bootAccent: "#ffffff",
    },
    gk: {
      shirt: "#38bdf8",
      sleeves: "#0284c7",
      trim: "#ffffff",
      shorts: "#0c4a6e",
      shortsStripe: "#38bdf8",
      socks: "#38bdf8",
      sockBand: "#ffffff",
      numberColor: "#0f172a",
      numberOutline: "#ffffff",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#38bdf8",
    },
    banner: "ST JAMES' PARK · GALLOWGATE END",
  },
  coventry: {
    home: {
      shirt: "#0593d0",
      sleeves: "#0593d0",
      trim: "#ffffff",
      shorts: "#0593d0",
      shortsStripe: "#ffffff",
      socks: "#ffffff",
      sockBand: "#0593d0",
      numberColor: "#ffffff",
      numberOutline: "#0f172a",
      pattern: "chevron_subtle",
      boots: "#0f172a",
      bootAccent: "#38bdf8",
    },
    away: {
      shirt: "#1e1b4b",
      sleeves: "#1e1b4b",
      trim: "#38bdf8",
      shorts: "#1e1b4b",
      shortsStripe: "#38bdf8",
      socks: "#1e1b4b",
      sockBand: "#38bdf8",
      numberColor: "#ffffff",
      numberOutline: "#000000",
      pattern: "pinstripes",
      boots: "#38bdf8",
      bootAccent: "#1e1b4b",
    },
    gk: {
      shirt: "#f59e0b",
      sleeves: "#d97706",
      trim: "#111827",
      shorts: "#78350f",
      shortsStripe: "#fbbf24",
      socks: "#f59e0b",
      sockBand: "#111827",
      numberColor: "#111827",
      numberOutline: "#ffffff",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#fbbf24",
    },
    banner: "CBS ARENA · SKY BLUE ARMY",
  },
  hull: {
    home: {
      shirt: "#f5a12d",
      sleeves: "#111827",
      trim: "#111827",
      shorts: "#111827",
      shortsStripe: "#f5a12d",
      socks: "#f5a12d",
      sockBand: "#111827",
      numberColor: "#ffffff",
      numberOutline: "#111827",
      pattern: "vertical_stripes",
      boots: "#111827",
      bootAccent: "#f5a12d",
    },
    away: {
      shirt: "#f8fafc",
      sleeves: "#f8fafc",
      trim: "#f5a12d",
      shorts: "#111827",
      shortsStripe: "#f5a12d",
      socks: "#f8fafc",
      sockBand: "#f5a12d",
      numberColor: "#111827",
      numberOutline: "#ffffff",
      pattern: "pinstripes",
      boots: "#111827",
      bootAccent: "#f5a12d",
    },
    gk: {
      shirt: "#14b8a6",
      sleeves: "#0d9488",
      trim: "#ffffff",
      shorts: "#134e4a",
      shortsStripe: "#2dd4bf",
      socks: "#14b8a6",
      sockBand: "#ffffff",
      numberColor: "#ffffff",
      numberOutline: "#134e4a",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#2dd4bf",
    },
    banner: "MKM STADIUM · THE TIGERS",
  },
  fulham: {
    home: {
      shirt: "#f8fafc",
      sleeves: "#f8fafc",
      trim: "#111827",
      shorts: "#111827",
      shortsStripe: "#ef4444",
      socks: "#f8fafc",
      sockBand: "#111827",
      numberColor: "#111827",
      numberOutline: "#ffffff",
      pattern: "shoulder_stripes",
      boots: "#111827",
      bootAccent: "#ef4444",
    },
    away: {
      shirt: "#dc2626",
      sleeves: "#111827",
      trim: "#111827",
      shorts: "#dc2626",
      shortsStripe: "#111827",
      socks: "#111827",
      sockBand: "#dc2626",
      numberColor: "#ffffff",
      numberOutline: "#111827",
      pattern: "vertical_stripes",
      boots: "#111827",
      bootAccent: "#dc2626",
    },
    gk: {
      shirt: "#eab308",
      sleeves: "#ca8a04",
      trim: "#111827",
      shorts: "#713f12",
      shortsStripe: "#fde047",
      socks: "#eab308",
      sockBand: "#111827",
      numberColor: "#111827",
      numberOutline: "#ffffff",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#fde047",
    },
    banner: "CRAVEN COTTAGE · HAMMERSMITH END",
  },
  ipswich: {
    home: {
      shirt: "#3a64a3",
      sleeves: "#3a64a3",
      trim: "#ffffff",
      shorts: "#ffffff",
      shortsStripe: "#3a64a3",
      socks: "#3a64a3",
      sockBand: "#ffffff",
      numberColor: "#ffffff",
      numberOutline: "#0f172a",
      pattern: "pinstripes",
      boots: "#111827",
      bootAccent: "#ffffff",
    },
    away: {
      shirt: "#7f1d1d",
      sleeves: "#7f1d1d",
      trim: "#fbbf24",
      shorts: "#7f1d1d",
      shortsStripe: "#fbbf24",
      socks: "#7f1d1d",
      sockBand: "#fbbf24",
      numberColor: "#fbbf24",
      numberOutline: "#000000",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#fbbf24",
    },
    gk: {
      shirt: "#22c55e",
      sleeves: "#16a34a",
      trim: "#ffffff",
      shorts: "#14532d",
      shortsStripe: "#4ade80",
      socks: "#22c55e",
      sockBand: "#ffffff",
      numberColor: "#ffffff",
      numberOutline: "#14532d",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#4ade80",
    },
    banner: "PORTMAN ROAD · SIR BOBBY ROBSON STAND",
  },
  "nottm-forest": {
    home: {
      shirt: "#dd0000",
      sleeves: "#dd0000",
      trim: "#ffffff",
      shorts: "#ffffff",
      shortsStripe: "#dd0000",
      socks: "#dd0000",
      sockBand: "#ffffff",
      numberColor: "#ffffff",
      numberOutline: "#450a0a",
      pattern: "shoulder_stripes",
      boots: "#111827",
      bootAccent: "#ffffff",
    },
    away: {
      shirt: "#1e3a8a",
      sleeves: "#1e3a8a",
      trim: "#38bdf8",
      shorts: "#1e3a8a",
      shortsStripe: "#38bdf8",
      socks: "#1e3a8a",
      sockBand: "#38bdf8",
      numberColor: "#ffffff",
      numberOutline: "#0f172a",
      pattern: "chevron_subtle",
      boots: "#38bdf8",
      bootAccent: "#1e3a8a",
    },
    gk: {
      shirt: "#eab308",
      sleeves: "#ca8a04",
      trim: "#111827",
      shorts: "#713f12",
      shortsStripe: "#fde047",
      socks: "#eab308",
      sockBand: "#111827",
      numberColor: "#111827",
      numberOutline: "#ffffff",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#fde047",
    },
    banner: "THE CITY GROUND · TRENT END",
  },
  sunderland: {
    home: {
      shirt: "#eb172b",
      sleeves: "#ffffff",
      trim: "#ffffff",
      shorts: "#111827",
      shortsStripe: "#eb172b",
      socks: "#111827",
      sockBand: "#eb172b",
      numberColor: "#111827",
      numberOutline: "#ffffff",
      pattern: "vertical_stripes",
      boots: "#111827",
      bootAccent: "#eb172b",
    },
    away: {
      shirt: "#f8fafc",
      sleeves: "#f8fafc",
      trim: "#0284c7",
      shorts: "#0c4a6e",
      shortsStripe: "#f8fafc",
      socks: "#f8fafc",
      sockBand: "#0284c7",
      numberColor: "#0c4a6e",
      numberOutline: "#ffffff",
      pattern: "chevron_subtle",
      boots: "#0c4a6e",
      bootAccent: "#ffffff",
    },
    gk: {
      shirt: "#10b981",
      sleeves: "#059669",
      trim: "#ffffff",
      shorts: "#064e3b",
      shortsStripe: "#34d399",
      socks: "#10b981",
      sockBand: "#ffffff",
      numberColor: "#ffffff",
      numberOutline: "#064e3b",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#34d399",
    },
    banner: "STADIUM OF LIGHT · ROKER END",
  },
  "crystal-palace": {
    home: {
      shirt: "#1b458f",
      sleeves: "#c4122e",
      trim: "#c4122e",
      shorts: "#1b458f",
      shortsStripe: "#c4122e",
      socks: "#1b458f",
      sockBand: "#c4122e",
      numberColor: "#ffffff",
      numberOutline: "#0f172a",
      pattern: "vertical_stripes",
      boots: "#111827",
      bootAccent: "#c4122e",
    },
    away: {
      shirt: "#facc15",
      sleeves: "#facc15",
      trim: "#1b458f",
      shorts: "#1b458f",
      shortsStripe: "#facc15",
      socks: "#facc15",
      sockBand: "#1b458f",
      numberColor: "#1b458f",
      numberOutline: "#ffffff",
      pattern: "pinstripes",
      boots: "#1b458f",
      bootAccent: "#facc15",
    },
    gk: {
      shirt: "#22c55e",
      sleeves: "#16a34a",
      trim: "#ffffff",
      shorts: "#14532d",
      shortsStripe: "#4ade80",
      socks: "#22c55e",
      sockBand: "#ffffff",
      numberColor: "#ffffff",
      numberOutline: "#14532d",
      pattern: "solid",
      boots: "#111827",
      bootAccent: "#4ade80",
    },
    banner: "SELHURST PARK · HOLMESDALE ROAD",
  },
};

const SKIN_TONES = ["#f4cfb0", "#e8b98e", "#d19968", "#9e663d", "#6a3f23", "#4c2b17"];
const HAIR_COLORS = ["#141416", "#2b1d12", "#4a321f", "#7c5329", "#d4a959", "#1f242d"];

// FIFA 11 "Personality+" Likeness & Somatotype Builds ("lean" | "athletic" | "power") — 2026–27 + Historical Squads
const PLAYER_LIKENESS = {
  // Manchester United (2026–27 Active)
  "mun-gk-2": { skin: 0, hair: 3, style: "swept", beard: false, build: "power" }, // S. Lammens #1
  "mun-def-1": { skin: 1, hair: 1, style: "crop", beard: true, build: "athletic" }, // L. Martinez #6
  "mun-def-2": { skin: 1, hair: 1, style: "fade", beard: true, build: "athletic" }, // D. Dalot #2
  "mun-def-3": { skin: 0, hair: 3, style: "swept", beard: false, build: "power" }, // M. de Ligt #4
  "mun-def-5": { skin: 4, hair: 0, style: "curls", beard: false, build: "athletic" }, // L. Yoro #15
  "mun-mid-1": { skin: 4, hair: 0, style: "curls", beard: false, build: "athletic" }, // K. Mainoo #37
  "mun-mid-2": { skin: 1, hair: 1, style: "crop", beard: true, build: "lean" }, // B. Fernandes #8
  "mun-mid-4": { skin: 0, hair: 2, style: "crop", beard: false, build: "athletic" }, // Y. Tielemans #18
  "mun-fwd-4": { skin: 2, hair: 0, style: "fade", beard: true, build: "athletic" }, // M. Cunha #10
  "mun-fwd-5": { skin: 5, hair: 0, style: "buzz", beard: true, build: "power" }, // B. Mbeumo #19
  "mun-fwd-6": { skin: 0, hair: 2, style: "crop", beard: false, build: "power" }, // B. Sesko #30

  // Arsenal (2026–27 Active)
  "ars-gk-1": { skin: 1, hair: 1, style: "fade", beard: true, build: "athletic" }, // D. Raya #1
  "ars-def-1": { skin: 4, hair: 0, style: "buzz", beard: false, build: "power" }, // W. Saliba #2
  "ars-def-3": { skin: 3, hair: 0, style: "crop", beard: true, build: "power" }, // Gabriel #6
  "ars-def-5": { skin: 0, hair: 1, style: "headband", beard: false, build: "athletic" }, // R. Calafiori #33
  "ars-def-4": { skin: 3, hair: 0, style: "curls", beard: false, build: "athletic" }, // J. Timber #12
  "ars-mid-1": { skin: 0, hair: 2, style: "swept", beard: false, build: "power" }, // D. Rice #41
  "ars-mid-2": { skin: 0, hair: 4, style: "swept", beard: true, build: "lean" }, // M. Odegaard #8
  "ars-mid-4": { skin: 1, hair: 4, style: "crop", beard: true, build: "athletic" }, // B. Guimaraes #39
  "ars-fwd-1": { skin: 5, hair: 0, style: "curls", beard: false, build: "lean" }, // B. Saka #7
  "ars-fwd-4": { skin: 4, hair: 0, style: "curls", beard: true, build: "lean" }, // E. Eze #10
  "ars-fwd-5": { skin: 0, hair: 3, style: "swept", beard: false, build: "power" }, // V. Gyokeres #14

  // Liverpool (2026–27 Active)
  "liv-gk-1": { skin: 1, hair: 2, style: "swept", beard: true, build: "power" }, // Alisson #1
  "liv-def-1": { skin: 3, hair: 0, style: "bun", beard: true, build: "power" }, // V. van Dijk #4
  "liv-def-5": { skin: 5, hair: 0, style: "curls", beard: false, build: "lean" }, // J. Frimpong #30
  "liv-def-6": { skin: 3, hair: 0, style: "fade", beard: true, build: "power" }, // R. Araujo #33
  "liv-def-7": { skin: 0, hair: 2, style: "crop", beard: false, build: "athletic" }, // M. Kerkez #6
  "liv-mid-4": { skin: 0, hair: 3, style: "swept", beard: false, build: "lean" }, // F. Wirtz #7
  "liv-mid-2": { skin: 0, hair: 1, style: "headband", beard: true, build: "athletic" }, // D. Szoboszlai #8
  "liv-mid-1": { skin: 0, hair: 2, style: "crop", beard: true, build: "athletic" }, // A. Mac Allister #10
  "liv-fwd-4": { skin: 4, hair: 0, style: "curls", beard: true, build: "lean" }, // A. Isak #9
  "liv-fwd-5": { skin: 4, hair: 0, style: "curls", beard: false, build: "lean" }, // B. Barcola #29
  "liv-fwd-6": { skin: 4, hair: 0, style: "curls", beard: false, build: "lean" }, // H. Ekitike #22

  // Chelsea (2026–27 Active)
  "che-gk-2": { skin: 1, hair: 1, style: "fade", beard: false, build: "power" }, // E. Martinez #1
  "che-def-1": { skin: 3, hair: 0, style: "curls", beard: false, build: "athletic" }, // L. Colwill #6
  "che-def-2": { skin: 4, hair: 0, style: "curls", beard: true, build: "power" }, // R. James #24
  "che-def-3": { skin: 4, hair: 0, style: "fade", beard: true, build: "athletic" }, // W. Fofana #3
  "che-def-5": { skin: 4, hair: 0, style: "curls", beard: false, build: "athletic" }, // J. Hato #21
  "che-mid-1": { skin: 4, hair: 0, style: "fade", beard: false, build: "power" }, // M. Caicedo #25
  "che-mid-2": { skin: 0, hair: 3, style: "crop", beard: false, build: "lean" }, // C. Palmer #10
  "che-mid-4": { skin: 2, hair: 1, style: "fade", beard: true, build: "power" }, // M. Rogers #17
  "che-fwd-4": { skin: 2, hair: 0, style: "fade", beard: true, build: "athletic" }, // Joao Pedro #9
  "che-fwd-2": { skin: 1, hair: 1, style: "crop", beard: true, build: "lean" }, // P. Neto #7
  "che-fwd-5": { skin: 3, hair: 0, style: "curls", beard: false, build: "lean" }, // Estevao #41

  // Manchester City & Premier League Stars (2026–27 Active + Cross-Club Transfers)
  "mci-gk-1": { skin: 1, hair: 1, style: "crop", beard: true, build: "power" }, // G. Donnarumma #1
  "mci-def-1": { skin: 1, hair: 1, style: "crop", beard: true, build: "power" }, // R. Dias #3
  "che-mid-3": { skin: 1, hair: 0, style: "fade", beard: false, build: "athletic" }, // E. Fernandez #17 (Man City)
  "mci-mid-2": { skin: 1, hair: 1, style: "curls", beard: true, build: "athletic" }, // R. Cherki #10
  "mci-fwd-1": { skin: 0, hair: 4, style: "bun", beard: false, build: "power" }, // E. Haaland #9
  "mci-mid-3": { skin: 0, hair: 1, style: "fade", beard: false, build: "lean" }, // P. Foden #47
  "mci-fwd-2": { skin: 5, hair: 0, style: "curls", beard: false, build: "athletic" }, // J. Doku #11
  "mci-fwd-3": { skin: 5, hair: 0, style: "fade", beard: true, build: "power" }, // A. Semenyo #42
  "bha-mid-1": { skin: 0, hair: 2, style: "swept", beard: true, build: "athletic" }, // P. Gross #13
  "bha-fwd-1": { skin: 1, hair: 0, style: "swept", beard: false, build: "lean" }, // K. Mitoma #7
  "eve-gk-1": { skin: 0, hair: 3, style: "swept", beard: false, build: "athletic" }, // J. Pickford #1
  "eve-mid-2": { skin: 0, hair: 1, style: "headband", beard: true, build: "athletic" }, // J. Grealish #10
  "eve-fwd-1": { skin: 5, hair: 0, style: "fade", beard: false, build: "power" }, // T. Barry #11
  "tot-def-1": { skin: 0, hair: 2, style: "swept", beard: false, build: "power" }, // M. van de Ven #37
  "tot-mid-1": { skin: 0, hair: 1, style: "swept", beard: false, build: "athletic" }, // S. Tonali #16
  "tot-mid-2": { skin: 3, hair: 0, style: "curls", beard: false, build: "lean" }, // X. Simons #7
  "tot-fwd-1": { skin: 2, hair: 0, style: "curls", beard: true, build: "athletic" }, // O. Marmoush #22
  "avl-mid-1": { skin: 5, hair: 0, style: "curls", beard: false, build: "power" }, // A. Onana #24
  "che-fwd-1": { skin: 5, hair: 0, style: "fade", beard: false, build: "athletic" }, // N. Jackson #11 (Aston Villa)
  "mun-fwd-2": { skin: 0, hair: 4, style: "swept", beard: false, build: "lean" }, // A. Garnacho #17 (Aston Villa)
  "lee-fwd-1": { skin: 2, hair: 0, style: "bun", beard: true, build: "power" }, // D. Calvert-Lewin #9
  "sun-mid-1": { skin: 0, hair: 1, style: "crop", beard: false, build: "power" }, // G. Xhaka #34

  // Retained Historical Pre-2026/27 Identities (for legacy worlds & frozen replays)
  "mun-gk-1": { skin: 5, hair: 0, style: "curls", beard: true, build: "power" }, // A. Onana
  "mun-fwd-1": { skin: 4, hair: 0, style: "fade", beard: false, build: "athletic" }, // M. Rashford
  "mun-def-4": { skin: 0, hair: 2, style: "crop", beard: false, build: "athletic" }, // L. Shaw
  "mun-mid-3": { skin: 2, hair: 1, style: "crop", beard: false, build: "power" }, // Casemiro
  "mun-fwd-3": { skin: 0, hair: 4, style: "swept", beard: false, build: "power" }, // R. Hojlund
  "ars-def-2": { skin: 1, hair: 1, style: "crop", beard: true, build: "athletic" }, // B. White
  "ars-mid-3": { skin: 5, hair: 0, style: "crop", beard: true, build: "power" }, // T. Partey
  "ars-fwd-2": { skin: 1, hair: 1, style: "crop", beard: false, build: "lean" }, // G. Martinelli
  "ars-fwd-3": { skin: 0, hair: 2, style: "swept", beard: false, build: "lean" }, // K. Havertz
  "liv-fwd-1": { skin: 2, hair: 0, style: "curls", beard: true, build: "athletic" }, // M. Salah
  "liv-def-2": { skin: 3, hair: 0, style: "curls", beard: false, build: "athletic" }, // T. Alexander-Arnold
  "liv-def-3": { skin: 5, hair: 0, style: "fade", beard: true, build: "power" }, // I. Konate
  "liv-def-4": { skin: 0, hair: 2, style: "crop", beard: false, build: "lean" }, // A. Robertson
  "liv-mid-3": { skin: 4, hair: 0, style: "curls", beard: false, build: "athletic" }, // R. Gravenberch
  "liv-fwd-2": { skin: 2, hair: 0, style: "fade", beard: false, build: "lean" }, // L. Diaz
  "liv-fwd-3": { skin: 1, hair: 1, style: "crop", beard: true, build: "athletic" }, // D. Jota
  "che-gk-1": { skin: 3, hair: 0, style: "fade", beard: true, build: "power" }, // R. Sanchez
  "che-def-4": { skin: 1, hair: 1, style: "curls", beard: true, build: "lean" }, // M. Cucurella
  "che-fwd-3": { skin: 4, hair: 0, style: "fade", beard: true, build: "athletic" }, // C. Nkunku
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
      for (const key of ["map", "emissiveMap", "alphaMap", "roughnessMap", "bumpMap", "normalMap"]) {
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

function parseHexSrgb(hex) {
  const s = String(hex || "#cc0000").trim().replace(/^#/, "");
  if (s.length === 3) {
    return [
      parseInt(s[0] + s[0], 16) / 255,
      parseInt(s[1] + s[1], 16) / 255,
      parseInt(s[2] + s[2], 16) / 255,
    ];
  }
  const n = parseInt(s.padEnd(6, "0").slice(0, 6), 16);
  return [((n >> 16) & 255) / 255, ((n >> 8) & 255) / 255, (n & 255) / 255];
}

function srgbPerceptualDist(hexA, hexB) {
  const [r1, g1, b1] = parseHexSrgb(hexA);
  const [r2, g2, b2] = parseHexSrgb(hexB);
  const dr = r1 - r2;
  const dg = g1 - g2;
  const db = b1 - b2;
  // Redmean-weighted sRGB distance normalized to [0, 1] (immune to Three.js linear-RGB gamma crush!)
  const rMean = (r1 + r2) * 0.5;
  const wR = 2 + rMean;
  const wG = 4.0;
  const wB = 3 - rMean;
  return Math.sqrt(wR * dr * dr + wG * dg * dg + wB * db * db) / 3.0;
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
    srgbPerceptualDist(homeOut.shirt, aEntry.home.shirt) < 0.36;
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
const HIP_SOCKET_L = new THREE.Vector3(0.089, -0.023, 0);
const HIP_SOCKET_R = new THREE.Vector3(-0.089, -0.023, 0);

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
  dt,
  footYawRad = 0,
  footRollRad = 0
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
  _eulerTmp.set(footPitchRad, footYawRad, footRollRad);
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
 * Continuous 12-ring lofted V-taper jersey torso with untucked lower hem drape (`y = 0.854 -> 1.482`)
 * bound smoothly across Hips, Spine, Chest & Shoulders.
 */
function createSkinnedTorsoGeometry() {
  const rings = [
    [0.902, 0.134, 0.088, -0.001], // Subtle inner underside lip so untucked jersey hem has real thickness!
    [0.906, 0.148, 0.098, -0.001], // Untucked jersey hem overhanging shorts waist (`IMG_0347`, `IMG_0354`)
    [0.938, 0.145, 0.095, 0.0],
    [0.975, 0.141, 0.092, 0.001], // Lean athletic waist taper
    [1.038, 0.147, 0.097, 0.003],
    [1.115, 0.158, 0.104, 0.006], // Oblique taper
    [1.195, 0.173, 0.111, 0.009], // Lower latissimus flare
    [1.275, 0.186, 0.117, 0.011], // Pectoral fullness
    [1.345, 0.193, 0.113, 0.006], // Broad athletic chest/shoulder line
    [1.405, 0.180, 0.100, 0.0],
    [1.448, 0.128, 0.080, -0.003], // Sloped trapezius
    [1.482, 0.071, 0.064, -0.004], // Neck collar opening
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
    const vCoord = Math.min(1, Math.max(0, (y - 0.904) / (1.482 - 0.904)));

    // Smooth 3-bone spinal weighting (Hips -> Spine -> Chest)
    let wHips = 0;
    let wSpine = 0;
    let wChest = 0;
    if (y < 1.085) {
      const t = smoothstep(0.885, 1.085, y);
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
function appendSkinnedTube(acc, rings, radialSegs, weightFn, uvMode = "wrap") {
  const baseVertex = acc.positions.length / 3;
  for (let r = 0; r < rings.length; r++) {
    const { cx, y, cz, rx, rz } = rings[r];
    const [bi0, bw0, bi1, bw1] = weightFn(y);
    const isShortsUv =
      uvMode === "shortsLeft" || uvMode === "shortsRight" || uvMode === "pelvis";
    const vCoord = isShortsUv
      ? Math.max(0, Math.min(1, (y - 0.556) / (0.926 - 0.556)))
      : r / Math.max(1, rings.length - 1);
    for (let s = 0; s <= radialSegs; s++) {
      const f = s / radialSegs;
      const theta = f * Math.PI * 2;
      const x = cx + Math.cos(theta) * rx;
      const z = cz + Math.sin(theta) * rz;
      let u = f;
      if (uvMode === "shortsLeft") {
        // u = 0 strictly on outer +X hip seam; u = 1 on inner -X groin inseam (continuous, zero winding flip!)
        u = 1.0 - Math.abs(f * 2.0 - 1.0);
      } else if (uvMode === "shortsRight") {
        // u = 0 strictly on outer -X hip seam; u = 1 on inner +X groin inseam!
        u = Math.abs(f * 2.0 - 1.0);
      } else if (uvMode === "pelvis") {
        // Pure clean shorts fabric across the entire pelvis & crotch bridge (zero stripe bleed!)
        u = 0.36 + 0.54 * (1.0 - Math.abs(f * 2.0 - 1.0));
      }
      acc.positions.push(x, y, z);
      acc.uvs.push(u, vCoord);
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
  geo.setAttribute("skinWeight", new THREE.Float32BufferAttribute(skinWeightsFromAcc(acc), 4));
  geo.setIndex(flatIndices);
  geo.computeVertexNormals();
  return geo;
}

function skinWeightsFromAcc(acc) {
  return acc.skinWeights;
}

/**
 * Continuous Skinned Arms (Left + Right) with sleeker athletic forearms and open sleeve cuff overhang.
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
    const cx = sideSign * 0.186;
    const bSh = sideSign > 0 ? B_SH_L : B_SH_R;
    const bEl = sideSign > 0 ? B_EL_L : B_EL_R;

    // Ordered strictly bottom-to-top (wrist y = 0.916 -> shoulder dome y = 1.454)
    const rings = [
      { cx, y: 0.914, cz: 0, rx: 0.002, rz: 0.002, mat: forearmMat },
      { cx, y: 0.920, cz: 0, rx: 0.030, rz: 0.025, mat: forearmMat }, // Anatomical wrist matching sculpted hand!
      { cx, y: 1.015, cz: 0, rx: 0.039, rz: 0.036, mat: forearmMat },
      { cx, y: 1.110, cz: 0, rx: 0.045, rz: 0.043, mat: forearmMat }, // Brachioradialis muscle taper
      { cx, y: 1.155, cz: -0.002, rx: 0.046, rz: 0.045, mat: forearmMat }, // Seamless skinned elbow!
      { cx, y: 1.192, cz: 0, rx: 0.047, rz: 0.046, mat: forearmMat },
      { cx, y: 1.196, cz: 0, rx: 0.055, rz: 0.054, mat: 1 }, // Open jersey sleeve cuff overhang!
      { cx, y: 1.218, cz: 0, rx: 0.058, rz: 0.057, mat: 0 },
      { cx, y: 1.310, cz: 0, rx: 0.062, rz: 0.060, mat: 0 }, // Bicep / tricep sleeve
      { cx: sideSign * 0.180, y: 1.395, cz: 0, rx: 0.065, rz: 0.062, mat: 0 }, // Deltoid contour
      { cx: sideSign * 0.154, y: 1.442, cz: 0, rx: 0.050, rz: 0.050, mat: 0 }, // Acromion blend into chest!
      { cx: sideSign * 0.140, y: 1.454, cz: 0, rx: 0.002, rz: 0.002, mat: 0 },
    ];

    appendSkinnedTube(acc, rings, 14, (y) => {
      if (y < 1.115) {
        return [bEl, 1.0, bSh, 0.0];
      }
      if (y < 1.192) {
        const t = smoothstep(1.115, 1.192, y);
        return [bEl, 1.0 - t, bSh, t];
      }
      if (y < 1.385) {
        return [bSh, 1.0, B_CHEST, 0.0];
      }
      const t = smoothstep(1.385, 1.454, y) * 0.55;
      return [bSh, 1.0 - t, B_CHEST, t];
    });
  }

  return finalizeGroupedSkinnedGeometry(acc);
}

/**
 * Continuous Skinned Lower Body: Trim Pelvis under Untucked Jersey + Tailored Shorts +
 * Toned Quadriceps + Seamless Skinned Knees + Sock Turnover Bands + Sculpted Calves & Achilles!
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

  // 1. Tailored Pelvis & Crotch Inseam Block (`y = 0.812 -> 0.926`) tucked cleanly under waist hem!
  const pelvisRings = [
    { cx: 0, y: 0.810, cz: -0.003, rx: 0.002, rz: 0.002, mat: 0 },
    { cx: 0, y: 0.816, cz: -0.003, rx: 0.114, rz: 0.078, mat: 0 },
    { cx: 0, y: 0.860, cz: -0.003, rx: 0.128, rz: 0.086, mat: 0 },
    { cx: 0, y: 0.902, cz: -0.002, rx: 0.132, rz: 0.089, mat: 0 },
    { cx: 0, y: 0.920, cz: -0.002, rx: 0.128, rz: 0.086, mat: 0 },
    { cx: 0, y: 0.926, cz: 0, rx: 0.002, rz: 0.002, mat: 0 },
  ];
  appendSkinnedTube(
    acc,
    pelvisRings,
    20,
    (y) => {
      const t = smoothstep(0.89, 0.926, y) * 0.22;
      return [B_HIPS, 1.0 - t, B_SPINE, t];
    },
    "pelvis"
  );

  // 2. Left & Right Continuous Skinned Legs (Ankle y = 0.062 -> Hip y = 0.922)
  for (const sideSign of [1, -1]) {
    const cx = sideSign * 0.089;
    const bHip = sideSign > 0 ? B_HIP_L : B_HIP_R;
    const bKnee = sideSign > 0 ? B_KNEE_L : B_KNEE_R;

    // A. Continuous Skin + Sock Leg Tube (Ankle y = 0.062 -> Upper Thigh y = 0.775 inside shorts)
    const legRings = [
      { cx, y: 0.060, cz: 0, rx: 0.002, rz: 0.002, mat: 3 },
      { cx, y: 0.065, cz: 0, rx: 0.041, rz: 0.045, mat: 3 }, // Sleek ankle collar into cleat
      { cx, y: 0.145, cz: -0.003, rx: 0.040, rz: 0.044, mat: 3 }, // Tapered Achilles tendon
      { cx, y: 0.240, cz: -0.006, rx: 0.053, rz: 0.057, mat: 3 },
      { cx, y: 0.345, cz: -0.009, rx: 0.065, rz: 0.070, mat: 3 }, // High gastrocnemius calf belly
      { cx, y: 0.415, cz: -0.004, rx: 0.059, rz: 0.062, mat: 2 }, // Sock turnover band
      { cx, y: 0.452, cz: 0, rx: 0.057, rz: 0.060, mat: 1 }, // Below patella skin
      { cx, y: 0.487, cz: 0.005, rx: 0.058, rz: 0.062, mat: 1 }, // Seamless skinned knee joint!
      { cx, y: 0.540, cz: 0.004, rx: 0.064, rz: 0.068, mat: 1 }, // Vastus medialis teardrop
      { cx, y: 0.630, cz: 0.004, rx: 0.075, rz: 0.079, mat: 1 }, // Sculpted quadriceps sweep
      { cx, y: 0.735, cz: 0, rx: 0.080, rz: 0.084, mat: 1 },
      { cx, y: 0.775, cz: 0, rx: 0.002, rz: 0.002, mat: 1 },
    ];

    appendSkinnedTube(acc, legRings, 16, (y) => {
      if (y < 0.440) {
        return [bKnee, 1.0, bHip, 0.0];
      }
      if (y < 0.535) {
        const t = smoothstep(0.440, 0.535, y);
        return [bKnee, 1.0 - t, bHip, t];
      }
      return [bHip, 1.0, B_HIPS, 0.0];
    });

    // B. Authentic FIFA 10–12 Knee-Length Loose Football Shorts (`y = 0.556 -> 0.922`, `IMG_0347` & `IMG_0363`!)
    const shortLegRings = [
      { cx, y: 0.560, cz: 0.004, rx: 0.002, rz: 0.002, mat: 0 },
      { cx, y: 0.556, cz: 0.004, rx: 0.068, rz: 0.072, mat: 0 }, // Inner shadow hem tuck above kneecap
      { cx, y: 0.560, cz: 0.004, rx: 0.095, rz: 0.100, mat: 0 }, // Loose flared athletic shorts opening!
      { cx, y: 0.655, cz: 0.002, rx: 0.094, rz: 0.098, mat: 0 },
      { cx, y: 0.785, cz: -0.001, rx: 0.092, rz: 0.095, mat: 0 },
      { cx, y: 0.910, cz: -0.002, rx: 0.087, rz: 0.090, mat: 0 },
      { cx, y: 0.922, cz: 0, rx: 0.002, rz: 0.002, mat: 0 },
    ];

    appendSkinnedTube(
      acc,
      shortLegRings,
      16,
      (y) => {
        if (y < 0.605) {
          const kneeFollow = (1.0 - smoothstep(0.556, 0.605, y)) * 0.16;
          return [bHip, 1.0 - kneeFollow, bKnee, kneeFollow];
        }
        const pelvisBlend = smoothstep(0.83, 0.922, y) * 0.42;
        return [bHip, 1.0 - pelvisBlend, B_HIPS, pelvisBlend];
      },
      sideSign > 0 ? "shortsLeft" : "shortsRight"
    );
  }

  return finalizeGroupedSkinnedGeometry(acc);
}

/**
 * Merges an array of static sub-geometries into a single draw-call BufferGeometry.
 */
function mergeStaticGeometries(geos) {
  const positions = [];
  const normals = [];
  for (const g of geos) {
    const nonIdx = g.index ? g.toNonIndexed() : g;
    const p = nonIdx.attributes.position;
    const n = nonIdx.attributes.normal;
    for (let i = 0; i < p.count; i++) {
      positions.push(p.getX(i), p.getY(i), p.getZ(i));
      normals.push(n ? n.getX(i) : 0, n ? n.getY(i) : 1, n ? n.getZ(i) : 0);
    }
    if (nonIdx !== g) nonIdx.dispose();
    g.dispose();
  }
  const out = new THREE.BufferGeometry();
  out.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
  out.setAttribute("normal", new THREE.Float32BufferAttribute(normals, 3));
  return out;
}

/**
 * Sculpted Anatomical 3D Hand (metacarpal palm + angled medial thumb + curled athletic fingers)
 * replacing bare SphereGeometry "potato hands".
 */
function createSculptedHandGeometry(sideSign) {
  const parts = [];
  // 1. Flattened metacarpal palm tapering seamlessly from wrist (y = 0) to knuckles (y = -0.062)
  const palm = new THREE.BoxGeometry(0.052, 0.066, 0.024, 2, 2, 2);
  palm.translate(0, -0.032, 0.002);
  parts.push(palm);

  // 2. Four slightly curled fingers in natural running carriage
  for (let f = 0; f < 4; f++) {
    const fx = (f - 1.5) * 0.0125;
    const lenScale = f === 1 || f === 2 ? 1.06 : f === 0 ? 0.96 : 0.88;
    const prox = new THREE.CapsuleGeometry(0.0058, 0.026 * lenScale, 3, 6);
    prox.rotateX(0.32);
    prox.translate(fx, -0.074, 0.008);
    parts.push(prox);

    const dist = new THREE.CapsuleGeometry(0.0050, 0.020 * lenScale, 3, 6);
    dist.rotateX(0.78);
    dist.translate(fx, -0.092 * lenScale, 0.020);
    parts.push(dist);
  }

  // 3. Opposable medial thumb angled forward (`-sideSign` points toward body midline!)
  const thumb = new THREE.CapsuleGeometry(0.0068, 0.030, 3, 6);
  thumb.rotateZ(sideSign * 0.58);
  thumb.rotateX(0.36);
  thumb.translate(-sideSign * 0.028, -0.038, 0.015);
  parts.push(thumb);

  const merged = mergeStaticGeometries(parts);
  merged.computeVertexNormals();
  return merged;
}

/**
 * Flush-sole 3D Football Cleat (Codex PR #6 fix: origin at ankle y = 0, sole resting at y = -SOLE_HEIGHT
 * so when ankle bone sits at y = SOLE_HEIGHT = 0.062, the cleat sole & studs rest flush at y = 0.000!).
 */
function createSculptedCleatGeometry() {
  const shape = new THREE.Shape();
  shape.moveTo(-0.040, -0.076);
  shape.quadraticCurveTo(-0.048, 0.02, -0.042, 0.108);
  shape.quadraticCurveTo(-0.030, 0.174, 0.0, 0.184);
  shape.quadraticCurveTo(0.030, 0.174, 0.042, 0.108);
  shape.quadraticCurveTo(0.048, 0.02, 0.040, -0.076);
  shape.quadraticCurveTo(0.0, -0.088, -0.040, -0.076);

  const g = new THREE.ExtrudeGeometry(shape, {
    depth: 0.046,
    bevelEnabled: true,
    bevelSegments: 3,
    steps: 1,
    bevelSize: 0.0085,
    bevelThickness: 0.0095,
  });
  g.rotateX(Math.PI * 0.5);
  const pos = g.attributes.position;
  for (let i = 0; i < pos.count; i++) {
    const y = pos.getY(i);
    const z = pos.getZ(i);
    if (y > -0.028 && z > 0.025) {
      pos.setY(i, y - (z - 0.025) * 0.21);
    }
  }
  g.translate(0, 0.003, 0.012);
  g.computeVertexNormals();
  return g;
}

/**
 * Proportionally scaled 1 : 7.56 Human Cranium, Cheekbones, Brow Ridge & Jawline.
 * Uses phiStart = -PI/2 so u = 0.5 maps directly to +Z (front face) and UV seam hides at -Z!
 */
function createSculptedHeadGeometry() {
  const g = new THREE.SphereGeometry(0.103, 30, 24, -Math.PI * 0.5, Math.PI * 2);
  const pos = g.attributes.position;
  for (let i = 0; i < pos.count; i++) {
    let x = pos.getX(i);
    let y = pos.getY(i);
    let z = pos.getZ(i);

    y *= 1.15; // Cranial height ~0.237m -> true 1 : 7.55 adult athlete ratio!
    x *= 0.87;

    if (y < 0) {
      const t = Math.min(1, -y / 0.118);
      x *= 1.0 - t * 0.30; // Defined athletic jawline taper
      if (z > 0) {
        z *= 1.0 - t * 0.12;
        if (t > 0.65 && Math.abs(x) < 0.034) {
          z += 0.009 * (t - 0.65); // Chin projection
        }
      } else {
        z *= 1.0 - t * 0.28; // Sub-occipital nape tuck
      }
    }

    // Smooth supraorbital brow ridge + zygomatic cheekbone fullness (zero faceted eye-socket dents!)
    if (z > 0.042 && y > 0.014 && y < 0.044) {
      z += 0.0062 * Math.cos(((y - 0.029) / 0.015) * Math.PI * 0.5);
    }
    if (z > 0.022 && y > -0.024 && y < 0.012 && Math.abs(x) > 0.032) {
      x *= 1.028;
      z += 0.0025;
    }

    pos.setXYZ(i, x, y, z);
  }
  g.computeVertexNormals();
  return g;
}

function createFaceSkinTexture(skinHex, hairHex, hasBeard, faceSeed = 0, maxAnisotropy = 4) {
  const W = 512;
  const H = 512;
  const c = document.createElement("canvas");
  c.width = W;
  c.height = H;
  const ctx = c.getContext("2d");

  ctx.fillStyle = skinHex;
  ctx.fillRect(0, 0, W, H);

  // Subsurface cheek/nose warmth + forehead highlight + jawline ambient contour (`IMG_0363`, `IMG_0365`)
  const foreheadHi = ctx.createRadialGradient(256, 198, 8, 256, 198, 72);
  foreheadHi.addColorStop(0, "rgba(255, 245, 235, 0.14)");
  foreheadHi.addColorStop(1, "rgba(255, 245, 235, 0)");
  ctx.fillStyle = foreheadHi;
  ctx.fillRect(160, 130, 192, 110);

  for (const cx of [214, 298]) {
    const cheek = ctx.createRadialGradient(cx, 272, 4, cx, 272, 44);
    cheek.addColorStop(0, "rgba(195, 72, 56, 0.12)");
    cheek.addColorStop(1, "rgba(195, 72, 56, 0)");
    ctx.fillStyle = cheek;
    ctx.beginPath();
    ctx.arc(cx, 272, 44, 0, Math.PI * 2);
    ctx.fill();
  }

  const jawShade = ctx.createLinearGradient(0, 332, 0, 512);
  jawShade.addColorStop(0, "rgba(18, 10, 6, 0)");
  jawShade.addColorStop(1, "rgba(18, 10, 6, 0.25)");
  ctx.fillStyle = jawShade;
  ctx.fillRect(0, 332, W, 180);

  // Baked hairline / temple / nape fade so 3D hair blends organically into skin
  ctx.fillStyle = hairHex;
  ctx.globalAlpha = 0.30;
  ctx.fillRect(0, 0, W, 152);
  ctx.fillRect(0, 152, 146, 188);
  ctx.fillRect(366, 152, 146, 188);
  ctx.globalAlpha = 1.0;

  // 4 Distinct Adult Pro Facial-Hair Styles (`IMG_0363`, `IMG_0365`)
  const beardVariant = hasBeard ? 1 + (faceSeed % 3) : faceSeed % 4 === 0 ? 1 : 0; // 0=clean, 1=stubble, 2=goatee, 3=fullBeard
  if (beardVariant > 0) {
    ctx.save();
    const beardAlpha = beardVariant === 1 ? 0.22 : beardVariant === 2 ? 0.44 : 0.56;
    const beardRx = beardVariant === 2 ? 42 : 76;
    const beardGrad = ctx.createRadialGradient(256, 346, 16, 256, 338, 90);
    beardGrad.addColorStop(0, hairHex);
    beardGrad.addColorStop(0.76, hairHex);
    beardGrad.addColorStop(1, "rgba(0,0,0,0)");
    ctx.globalAlpha = beardAlpha;
    ctx.fillStyle = beardGrad;
    ctx.beginPath();
    ctx.ellipse(256, 340, beardRx, 64, 0, 0.04, Math.PI - 0.04);
    ctx.fill();

    // Mustache arch above upper lip
    ctx.globalAlpha = beardVariant === 1 ? 0.26 : 0.58;
    ctx.strokeStyle = hairHex;
    ctx.lineWidth = beardVariant === 1 ? 6 : 9;
    ctx.lineCap = "round";
    ctx.beginPath();
    ctx.moveTo(226, 299);
    ctx.quadraticCurveTo(256, 291, 286, 299);
    ctx.stroke();

    // Chin soul-patch connector
    if (beardVariant >= 2) {
      ctx.fillStyle = hairHex;
      ctx.beginPath();
      ctx.roundRect(249, 318, 14, 18, 4);
      ctx.fill();
    }
    ctx.restore();
  }

  // Crisp 512x512 Anatomical Eyes (`IMG_0363`, `IMG_0365`): Sclera + Colored Iris + Pupil + Catchlight + Lashes!
  const irisColors = ["#2c1d11", "#3f2a18", "#233645", "#4c3822"];
  const irisHex = irisColors[faceSeed % irisColors.length];
  for (const [idx, ex] of [
    [0, 225],
    [1, 287],
  ]) {
    const sgn = idx === 0 ? -1 : 1; // -1 = viewer left, +1 = viewer right
    // Subtle upper-orbital crease shadow (strictly ABOVE eye so sclera stays crisp!)
    const browShadow = ctx.createRadialGradient(ex, 226, 2, ex, 228, 22);
    browShadow.addColorStop(0, "rgba(26, 14, 10, 0.24)");
    browShadow.addColorStop(1, "rgba(26, 14, 10, 0)");
    ctx.fillStyle = browShadow;
    ctx.beginPath();
    ctx.arc(ex, 228, 22, Math.PI, Math.PI * 2);
    ctx.fill();

    // Almond sclera (`#eef2f6` with subtle corner shading)
    ctx.fillStyle = "#eef2f6";
    ctx.beginPath();
    ctx.moveTo(ex - 13.5, 238);
    ctx.quadraticCurveTo(ex, 229.2, ex + 13.5, 238);
    ctx.quadraticCurveTo(ex, 245.4, ex - 13.5, 238);
    ctx.closePath();
    ctx.fill();

    // Colored iris + limbal ring
    ctx.fillStyle = irisHex;
    ctx.beginPath();
    ctx.arc(ex, 237.4, 5.9, 0, Math.PI * 2);
    ctx.fill();
    ctx.strokeStyle = "rgba(10, 12, 18, 0.65)";
    ctx.lineWidth = 1.2;
    ctx.stroke();

    // Deep pupil
    ctx.fillStyle = "#060911";
    ctx.beginPath();
    ctx.arc(ex, 237.4, 3.1, 0, Math.PI * 2);
    ctx.fill();

    // Specular stadium floodlight catchlight dot
    ctx.fillStyle = "rgba(255, 255, 255, 0.85)";
    ctx.beginPath();
    ctx.arc(ex - 1.8, 235.6, 1.45, 0, Math.PI * 2);
    ctx.fill();

    // Crisp upper eyelid lash arch + subtle lower tear-line
    ctx.strokeStyle = "rgba(14, 10, 9, 0.88)";
    ctx.lineWidth = 3.0;
    ctx.lineCap = "round";
    ctx.beginPath();
    ctx.moveTo(ex - 14.2, 238.4);
    ctx.quadraticCurveTo(ex, 228.4, ex + 14.2, 238.4);
    ctx.stroke();

    ctx.strokeStyle = "rgba(56, 30, 22, 0.36)";
    ctx.lineWidth = 1.4;
    ctx.beginPath();
    ctx.moveTo(ex - 11.5, 240.0);
    ctx.quadraticCurveTo(ex, 245.5, ex + 11.5, 240.0);
    ctx.stroke();

    // Expressive competitive arched eyebrows (`IMG_0363`, `IMG_0365`)
    ctx.fillStyle = hairHex;
    ctx.globalAlpha = 0.90;
    ctx.beginPath();
    ctx.moveTo(ex - sgn * 16, 219);
    ctx.quadraticCurveTo(ex + sgn * 2, 211, ex + sgn * 18, 217);
    ctx.lineTo(ex + sgn * 17, 222);
    ctx.quadraticCurveTo(ex + sgn * 2, 217, ex - sgn * 15, 224);
    ctx.closePath();
    ctx.fill();
    ctx.globalAlpha = 1.0;
  }

  // Defined nasal bridge side-contours + nostril wings (`cy = 278`)
  ctx.strokeStyle = "rgba(38, 18, 12, 0.18)";
  ctx.lineWidth = 2.6;
  ctx.beginPath();
  ctx.moveTo(246, 236);
  ctx.quadraticCurveTo(243, 260, 241, 276);
  ctx.moveTo(266, 236);
  ctx.quadraticCurveTo(269, 260, 271, 276);
  ctx.stroke();

  ctx.fillStyle = "rgba(26, 12, 9, 0.34)";
  ctx.beginPath();
  ctx.ellipse(247, 279, 4.6, 2.5, -0.25, 0, Math.PI * 2);
  ctx.ellipse(265, 279, 4.6, 2.5, 0.25, 0, Math.PI * 2);
  ctx.fill();

  // Nasolabial athletic creases + two-tone sculpted lips (`cy = 308`)
  ctx.strokeStyle = "rgba(42, 20, 14, 0.14)";
  ctx.lineWidth = 2.0;
  ctx.beginPath();
  ctx.moveTo(236, 278);
  ctx.quadraticCurveTo(226, 296, 230, 313);
  ctx.moveTo(276, 278);
  ctx.quadraticCurveTo(286, 296, 282, 313);
  ctx.stroke();

  // Upper lip (richer shadow vermilion) & lower lip (subtle sheen)
  ctx.fillStyle = "rgba(126, 54, 48, 0.52)";
  ctx.beginPath();
  ctx.moveTo(234, 308);
  ctx.quadraticCurveTo(248, 301, 256, 303.5);
  ctx.quadraticCurveTo(264, 301, 278, 308);
  ctx.quadraticCurveTo(256, 310, 234, 308);
  ctx.fill();

  ctx.fillStyle = "rgba(154, 72, 64, 0.40)";
  ctx.beginPath();
  ctx.moveTo(236, 309);
  ctx.quadraticCurveTo(256, 318.5, 276, 309);
  ctx.closePath();
  ctx.fill();

  ctx.strokeStyle = "rgba(44, 16, 14, 0.68)";
  ctx.lineWidth = 2.4;
  ctx.beginPath();
  ctx.moveTo(234, 308.5);
  ctx.quadraticCurveTo(256, 311.2, 278, 308.5);
  ctx.stroke();

  const tex = new THREE.CanvasTexture(c);
  tex.colorSpace = THREE.SRGBColorSpace;
  tex.anisotropy = maxAnisotropy;
  return tex;
}

/**
 * Procedural 5-blade manicured 3D stadium grass micro-tuft geometry (`2.4cm` height, `IMG_0351`, `IMG_0364`).
 */
function createGrassTuftGeometry() {
  const positions = [];
  const normals = [];
  const uvs = [];
  const blades = [
    { ox: 0.0, oz: 0.0, yaw: 0.1, lean: 0.009, h: 0.026, w: 0.0046 },
    { ox: -0.016, oz: 0.012, yaw: 1.45, lean: 0.011, h: 0.023, w: 0.0042 },
    { ox: 0.015, oz: -0.013, yaw: -1.75, lean: 0.010, h: 0.025, w: 0.0044 },
    { ox: 0.018, oz: 0.014, yaw: 2.65, lean: 0.011, h: 0.021, w: 0.0040 },
    { ox: -0.014, oz: -0.017, yaw: -0.65, lean: 0.009, h: 0.024, w: 0.0042 },
  ];
  const pushTriBothSides = (ax, ay, az, bx, by, bz, cx, cy, cz, nx, ny, nz) => {
    positions.push(ax, ay, az, bx, by, bz, cx, cy, cz);
    positions.push(ax, ay, az, cx, cy, cz, bx, by, bz);
    for (let k = 0; k < 6; k++) {
      normals.push(nx, ny, nz);
      uvs.push(0.5, 0.5);
    }
  };
  for (const b of blades) {
    const c = Math.cos(b.yaw);
    const s = Math.sin(b.yaw);
    const lx = -b.w * c;
    const lz = -b.w * s;
    const rx = b.w * c;
    const rz = b.w * s;
    const tx = b.ox + s * b.lean;
    const tz = b.oz + c * b.lean;
    const mxL = b.ox + lx * 0.58 + s * b.lean * 0.42;
    const mzL = b.oz + lz * 0.58 + c * b.lean * 0.42;
    const mxR = b.ox + rx * 0.58 + s * b.lean * 0.42;
    const mzR = b.oz + rz * 0.58 + c * b.lean * 0.42;
    const my = b.h * 0.52;
    const nx = s * 0.16;
    const ny = 0.97;
    const nz = c * 0.16;

    pushTriBothSides(b.ox + lx, 0, b.oz + lz, b.ox + rx, 0, b.oz + rz, mxR, my, mzR, nx, ny, nz);
    pushTriBothSides(b.ox + lx, 0, b.oz + lz, mxR, my, mzR, mxL, my, mzL, nx, ny, nz);
    pushTriBothSides(mxL, my, mzL, mxR, my, mzR, tx, b.h, tz, nx, ny, nz);
  }
  const geo = new THREE.BufferGeometry();
  geo.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
  geo.setAttribute("normal", new THREE.Float32BufferAttribute(normals, 3));
  geo.setAttribute("uv", new THREE.Float32BufferAttribute(uvs, 2));
  return geo;
}

/**
 * Singleton Tangent-Space Cloth Weave & Tension Wrinkle Normal Map (`256x256`)
 * Gives SkinnedMesh jerseys, sleeves & shorts realistic EA FIFA 10–12 fabric folds under floodlights!
 */
const SHARED_CLOTH_NORMAL = (() => {
  const S = 256;
  const data = new Uint8Array(S * S * 4);
  const heightAt = (u, v) => {
    // Diagonal athletic chest/underarm & waist tension creases + fine polyester piqué weave
    const fold1 = Math.sin((u * 3.2 + v * 5.8) * Math.PI * 2) * Math.exp(-Math.pow((v - 0.38) * 3.4, 2)) * 0.45;
    const fold2 = Math.sin((u * -3.0 + v * 6.2) * Math.PI * 2) * Math.exp(-Math.pow((v - 0.68) * 3.6, 2)) * 0.42;
    const hemFold = Math.sin(u * 9.0 * Math.PI * 2) * Math.exp(-Math.pow((v - 0.88) * 6.0, 2)) * 0.32;
    const weave = (Math.sin(u * 64 * Math.PI * 2) * Math.cos(v * 64 * Math.PI * 2)) * 0.08;
    const organic = seededNoise2D(u * 10.5 + 3.1, v * 12.5 - 1.7) * 0.22;
    return fold1 + fold2 + hemFold + weave + organic;
  };
  const eps = 1 / S;
  const strength = 2.1;
  for (let y = 0; y < S; y++) {
    const v = y / S;
    for (let x = 0; x < S; x++) {
      const u = x / S;
      const dhdu = (heightAt(u + eps, v) - heightAt(u - eps, v)) * strength;
      const dhdv = (heightAt(u, v + eps) - heightAt(u, v - eps)) * strength;
      let nx = -dhdu;
      let ny = -dhdv;
      let nz = 1.0;
      const invL = 1.0 / Math.hypot(nx, ny, nz);
      nx *= invL;
      ny *= invL;
      nz *= invL;
      const idx = (y * S + x) * 4;
      data[idx] = Math.round((nx * 0.5 + 0.5) * 255);
      data[idx + 1] = Math.round((ny * 0.5 + 0.5) * 255);
      data[idx + 2] = Math.round((nz * 0.5 + 0.5) * 255);
      data[idx + 3] = 255;
    }
  }
  const tex = new THREE.DataTexture(data, S, S, THREE.RGBAFormat);
  tex.wrapS = THREE.RepeatWrapping;
  tex.wrapT = THREE.RepeatWrapping;
  tex.needsUpdate = true;
  return retainGpu(tex);
})();

// --- Singleton Shared Geometries ---

const SHARED_GEO = (() => {
  const skinnedTorso = retainGpu(createSkinnedTorsoGeometry());
  const skinnedArmsOutfield = retainGpu(createSkinnedArmsGeometry(false));
  const skinnedArmsGk = retainGpu(createSkinnedArmsGeometry(true));
  const skinnedLowerBody = retainGpu(createSkinnedLowerBodyGeometry());

  const cleat = retainGpu(createSculptedCleatGeometry());
  const solePlate = retainGpu(new THREE.BoxGeometry(0.068, 0.008, 0.222));
  const bootStripe = retainGpu(new THREE.BoxGeometry(0.088, 0.020, 0.080));
  const stud = retainGpu(new THREE.CylinderGeometry(0.0065, 0.0045, 0.014, 6));

  const handL = retainGpu(createSculptedHandGeometry(1));
  const handR = retainGpu(createSculptedHandGeometry(-1));
  const wristTape = retainGpu(new THREE.CylinderGeometry(0.032, 0.031, 0.030, 12));
  const gkGlove = retainGpu(new THREE.SphereGeometry(0.052, 12, 10));
  gkGlove.scale(0.98, 1.18, 0.48);
  const gkFinger = retainGpu(new THREE.CapsuleGeometry(0.0105, 0.042, 4, 6));
  const gkCuff = retainGpu(new THREE.CylinderGeometry(0.044, 0.041, 0.044, 12));

  const neck = retainGpu(new THREE.CylinderGeometry(0.050, 0.061, 0.095, 12));
  const collar = retainGpu(new THREE.TorusGeometry(0.069, 0.0135, 8, 18));
  collar.rotateX(Math.PI * 0.5);
  const head = retainGpu(createSculptedHeadGeometry());
  const nose = retainGpu(new THREE.SphereGeometry(0.0145, 8, 8));
  nose.scale(0.74, 1.08, 1.12);
  const ear = retainGpu(new THREE.SphereGeometry(0.019, 8, 6));
  ear.scale(0.42, 1.04, 0.66);

  const hairTopPad = retainGpu(new THREE.SphereGeometry(0.068, 12, 8));
  const hairCurl = retainGpu(new THREE.SphereGeometry(0.021, 7, 6));
  const hairWave = retainGpu(new THREE.SphereGeometry(0.056, 12, 10));
  const hairBun = retainGpu(new THREE.SphereGeometry(0.034, 10, 10));
  const headband = retainGpu(new THREE.TorusGeometry(0.096, 0.0095, 8, 24));
  headband.rotateX(Math.PI * 0.46);
  headband.scale(0.92, 1.03, 1.0);
  const gkHeadgear = retainGpu(createTailoredHairCapGeometry(-0.068, 0.012));
  const sNumPlane = retainGpu(new THREE.PlaneGeometry(0.054, 0.050));

  // Sleek FIFA 10–12 Proportioned Pitch Indicators (`IMG_0350.webp`)
  const ring = retainGpu(new THREE.RingGeometry(0.35, 0.45, 36));
  ring.rotateX(-Math.PI * 0.5);
  const chargeArc = retainGpu(new THREE.RingGeometry(0.47, 0.56, 32));
  chargeArc.rotateX(-Math.PI * 0.5);
  const aimArrow = retainGpu(new THREE.ConeGeometry(0.095, 0.20, 4));
  aimArrow.rotateX(Math.PI * 0.5);
  const cursorCone = retainGpu(new THREE.ConeGeometry(0.092, 0.17, 4));
  cursorCone.rotateX(Math.PI);
  const shadowDisc = retainGpu(new THREE.CircleGeometry(0.36, 20));
  shadowDisc.rotateX(-Math.PI * 0.5);
  const grassTuft = retainGpu(createGrassTuftGeometry());

  return {
    skinnedTorso,
    skinnedArmsOutfield,
    skinnedArmsGk,
    skinnedLowerBody,
    cleat,
    solePlate,
    bootStripe,
    stud,
    handL,
    handR,
    wristTape,
    gkGlove,
    gkFinger,
    gkCuff,
    neck,
    collar,
    head,
    nose,
    ear,
    hairTopPad,
    hairCurl,
    hairWave,
    hairBun,
    headband,
    gkHeadgear,
    sNumPlane,
    ring,
    chargeArc,
    aimArrow,
    cursorCone,
    shadowDisc,
    grassTuft,
  };
})();

// ============================================================================
// 3. WEATHER SKIES, PBR TURF MAPS, STADIUM JUMBOTRON & KIT TEXTURE BUILDERS
// ============================================================================

function createSkyTexture(mode = "night") {
  const c = document.createElement("canvas");
  c.width = 64;
  c.height = 512;
  const g = c.getContext("2d");
  const grad = g.createLinearGradient(0, 0, 0, 512);
  if (mode === "day") {
    grad.addColorStop(0.0, "#1d4ed8");
    grad.addColorStop(0.28, "#3b82f6");
    grad.addColorStop(0.52, "#7dd3fc");
    grad.addColorStop(0.72, "#bae6fd");
    grad.addColorStop(1.0, "#dbeafe");
  } else if (mode === "rain") {
    grad.addColorStop(0.0, "#050b16");
    grad.addColorStop(0.32, "#0f1e33");
    grad.addColorStop(0.56, "#1e3048");
    grad.addColorStop(0.76, "#283c54");
    grad.addColorStop(1.0, "#12212b");
  } else {
    grad.addColorStop(0.0, "#030712");
    grad.addColorStop(0.32, "#0a162c");
    grad.addColorStop(0.54, "#152c4e");
    grad.addColorStop(0.72, "#1e3a5f");
    grad.addColorStop(1.0, "#0b1914");
  }
  g.fillStyle = grad;
  g.fillRect(0, 0, 64, 512);
  const tex = new THREE.CanvasTexture(c);
  tex.colorSpace = THREE.SRGBColorSpace;
  return retainGpu(tex);
}

const SKY_TEXTURES = {
  night: createSkyTexture("night"),
  day: createSkyTexture("day"),
  rain: createSkyTexture("rain"),
};

/**
 * Generates coordinated PBR Turf `map`, `normalMap`, and `roughnessMap` (`IMG_0350`, `IMG_0354`, `IMG_0366`):
 * - Authentic Premier League botanical olive-emerald cross-mowed checkerboard grass
 * - Tangent-space micro-blade normal relief
 * - Directional mower-stripe & worn-box roughness map (enables realistic wet-pitch floodlight sheen!)
 */
function createPitchPbrMaps(pitchLen, pitchWid, fmt, maxAnisotropy) {
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

  // Roughness canvas (`#000000` = mirror wet, `#ffffff` = matte dry)
  const rCanvas = document.createElement("canvas");
  rCanvas.width = 512;
  rCanvas.height = 320;
  const rCtx = rCanvas.getContext("2d");
  rCtx.fillStyle = "#d4d4d4";
  rCtx.fillRect(0, 0, 512, 320);

  const pxX = (wx) => ((wx + totalW * 0.5) / totalW) * W;
  const pxZ = (wz) => ((wz + totalH * 0.5) / totalH) * H;
  const scaleX = W / totalW;
  const scaleZ = H / totalH;

  // Outer pitch apron in rich deep stadium turf olive
  ctx.fillStyle = "#1f4916";
  ctx.fillRect(0, 0, W, H);

  const halfL = pitchLen * 0.5;
  const halfW = pitchWid * 0.5;
  const pLeft = pxX(-halfL);
  const pTop = pxZ(-halfW);
  const pW = pitchLen * scaleX;
  const pH = pitchWid * scaleZ;

  // Classic EA FIFA 10–12 rich botanical olive-emerald vertical mower stripes (`IMG_0350.webp`, `IMG_0354.jpeg`!)
  const stripeCount = pitchLen > 80 ? 18 : 12;
  const stripeM = pitchLen / stripeCount;
  for (let i = 0; i < stripeCount; i++) {
    const x0 = pxX(-halfL + i * stripeM);
    const x1 = pxX(-halfL + (i + 1) * stripeM);
    ctx.fillStyle = i % 2 === 0 ? "#275b1b" : "#316c23";
    ctx.fillRect(x0, pTop, x1 - x0 + 0.8, pH);

    // Alternating blade-lay roughness in PBR roughnessMap
    rCtx.fillStyle = i % 2 === 0 ? "#bcbcbc" : "#d8d8d8";
    rCtx.fillRect(x0 * 0.25, pTop * 0.25, (x1 - x0) * 0.25 + 0.5, pH * 0.25);
  }

  // Subtle horizontal cross-cut checkerboard bands (`IMG_0350.webp`!)
  const crossRows = pitchWid > 50 ? 10 : 6;
  const rowH = pH / crossRows;
  for (let r = 0; r < crossRows; r++) {
    ctx.fillStyle = r % 2 === 0 ? "rgba(245, 250, 195, 0.038)" : "rgba(8, 26, 6, 0.042)";
    ctx.fillRect(pLeft, pTop + r * rowH, pW, rowH);
  }

  // Authentic Stadium Roof Canopy Shadow Falloff along far/near touchlines (`IMG_0350.webp`, `IMG_0351.jpeg`!)
  const northRoofGrad = ctx.createLinearGradient(0, 0, 0, pTop + pH * 0.22);
  northRoofGrad.addColorStop(0.0, "rgba(4, 14, 6, 0.42)");
  northRoofGrad.addColorStop(0.55, "rgba(6, 18, 8, 0.16)");
  northRoofGrad.addColorStop(1.0, "rgba(6, 18, 8, 0)");
  ctx.fillStyle = northRoofGrad;
  ctx.fillRect(0, 0, W, pTop + pH * 0.22);

  const southRoofGrad = ctx.createLinearGradient(0, H, 0, pTop + pH * 0.84);
  southRoofGrad.addColorStop(0.0, "rgba(4, 14, 6, 0.34)");
  southRoofGrad.addColorStop(1.0, "rgba(6, 18, 8, 0)");
  ctx.fillStyle = southRoofGrad;
  ctx.fillRect(0, pTop + pH * 0.84, W, H - (pTop + pH * 0.84));

  // Multi-octave Simplex Noise organic turf grain & fine grass blade stippling
  const cell = 12;
  for (let py = 0; py < H; py += cell) {
    for (let px = 0; px < W; px += cell) {
      const n1 = seededNoise2D(px * 0.007, py * 0.007);
      const n2 = seededNoise2D(px * 0.028 + 19.3, py * 0.028 - 7.1) * 0.55;
      const v = n1 + n2;
      if (Math.abs(v) > 0.12) {
        ctx.fillStyle =
          v > 0
            ? `rgba(198, 232, 148, ${(v * 0.034).toFixed(3)})`
            : `rgba(10, 32, 8, ${(-v * 0.055).toFixed(3)})`;
        ctx.fillRect(px, py, cell, cell);
      }
    }
  }

  // Goal-mouth, six-yard & center-spot earth/wear patches (lower roughness when damp!)
  for (const [wx, wz, radM, alpha] of [
    [-halfL + 1.4, 0, 3.8, 0.22],
    [halfL - 1.4, 0, 3.8, 0.22],
    [0, 0, 2.8, 0.10],
  ]) {
    const gx = pxX(wx);
    const gy = pxZ(wz);
    const rPx = scaleX * radM;
    const wear = ctx.createRadialGradient(gx, gy, 3, gx, gy, rPx);
    wear.addColorStop(0, `rgba(112, 86, 46, ${alpha})`);
    wear.addColorStop(0.55, `rgba(96, 76, 40, ${(alpha * 0.45).toFixed(3)})`);
    wear.addColorStop(1, "rgba(96, 76, 40, 0)");
    ctx.fillStyle = wear;
    ctx.beginPath();
    ctx.arc(gx, gy, rPx, 0, Math.PI * 2);
    ctx.fill();

    const rWear = rCtx.createRadialGradient(gx * 0.25, gy * 0.25, 2, gx * 0.25, gy * 0.25, rPx * 0.25);
    rWear.addColorStop(0, "rgba(110, 110, 110, 0.65)");
    rWear.addColorStop(1, "rgba(110, 110, 110, 0)");
    rCtx.fillStyle = rWear;
    rCtx.beginPath();
    rCtx.arc(gx * 0.25, gy * 0.25, rPx * 0.25, 0, Math.PI * 2);
    rCtx.fill();
  }

  const linePx = Math.max(4.4, scaleX * 0.135);
  ctx.strokeStyle = "rgba(248, 250, 252, 0.94)";
  ctx.fillStyle = "rgba(248, 250, 252, 0.95)";
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

  // Tileable high-frequency Tangent-Space Grass Blade Normal Map (`256x256` repeated across turf)
  const NS = 256;
  const nData = new Uint8Array(NS * NS * 4);
  for (let y = 0; y < NS; y++) {
    for (let x = 0; x < NS; x++) {
      const g1 = seededNoise2D(x * 0.32, y * 0.32);
      const g2 = seededNoise2D(x * 0.78 + 41.2, y * 0.78 - 13.7);
      let nx = g1 * 0.36 + g2 * 0.22;
      let ny = g2 * 0.36 - g1 * 0.18;
      let nz = 1.0;
      const inv = 1.0 / Math.hypot(nx, ny, nz);
      const idx = (y * NS + x) * 4;
      nData[idx] = Math.round(((nx * inv) * 0.5 + 0.5) * 255);
      nData[idx + 1] = Math.round(((ny * inv) * 0.5 + 0.5) * 255);
      nData[idx + 2] = Math.round(((nz * inv) * 0.5 + 0.5) * 255);
      nData[idx + 3] = 255;
    }
  }
  const normalMap = new THREE.DataTexture(nData, NS, NS, THREE.RGBAFormat);
  normalMap.wrapS = THREE.RepeatWrapping;
  normalMap.wrapT = THREE.RepeatWrapping;
  normalMap.repeat.set(Math.round(totalW * 0.85), Math.round(totalH * 0.85));
  normalMap.needsUpdate = true;

  const roughnessMap = new THREE.CanvasTexture(rCanvas);
  roughnessMap.anisotropy = maxAnisotropy || 4;

  const tex = new THREE.CanvasTexture(c);
  tex.colorSpace = THREE.SRGBColorSpace;
  tex.anisotropy = maxAnisotropy || 4;
  return { tex, normalMap, roughnessMap, totalW, totalH };
}

function createGoalNetTexture(maxAnisotropy = 4) {
  const S = 256;
  const c = document.createElement("canvas");
  c.width = S;
  c.height = S;
  const ctx = c.getContext("2d");
  ctx.clearRect(0, 0, S, S);

  // Crisp braided white diamond/square FIFA stadium goal net (`IMG_0350.webp`, `IMG_0351.jpeg`, `IMG_0356.jpeg`)
  ctx.strokeStyle = "rgba(255, 255, 255, 0.95)";
  ctx.lineWidth = 3.4;
  const step = 28;
  for (let i = -S; i <= S * 2; i += step) {
    ctx.beginPath();
    ctx.moveTo(i, 0);
    ctx.lineTo(i + S, S);
    ctx.stroke();
    ctx.beginPath();
    ctx.moveTo(i + S, 0);
    ctx.lineTo(i, S);
    ctx.stroke();
  }
  // Subtle knot highlight at diamond intersections
  ctx.fillStyle = "rgba(226, 232, 240, 0.9)";
  for (let y = 0; y <= S; y += step * 0.5) {
    for (let x = 0; x <= S; x += step) {
      const ox = (Math.round(y / (step * 0.5)) % 2) * (step * 0.5);
      ctx.fillRect(x + ox - 1.5, y - 1.5, 3, 3);
    }
  }

  const tex = new THREE.CanvasTexture(c);
  tex.colorSpace = THREE.SRGBColorSpace;
  tex.wrapS = THREE.RepeatWrapping;
  tex.wrapT = THREE.RepeatWrapping;
  tex.repeat.set(12, 5.5);
  tex.anisotropy = maxAnisotropy;
  return tex;
}

/**
 * Clean Thermally-Bonded Pro Size 5 Match Ball Texture (`IMG_0345`, `IMG_0358`, `IMG_0363`, `IMG_0366`).
 * Crisp pearl-white leather with non-overlapping geometric Obsidian/Gold/Crimson star-pentagon panels!
 */
function createMatchBallTexture() {
  const W = 512;
  const H = 256;
  const c = document.createElement("canvas");
  c.width = W;
  c.height = H;
  const ctx = c.getContext("2d");

  // Pearl-white micro-textured polyurethane match ball leather
  ctx.fillStyle = "#f8fafc";
  ctx.fillRect(0, 0, W, H);

  // Clean icosahedral thermal-bond panel seam lines (subtle, never messy!)
  const centers = [
    [0.125, 0.26],
    [0.375, 0.26],
    [0.625, 0.26],
    [0.875, 0.26],
    [0.0, 0.5],
    [0.25, 0.5],
    [0.5, 0.5],
    [0.75, 0.5],
    [1.0, 0.5],
    [0.125, 0.74],
    [0.375, 0.74],
    [0.625, 0.74],
    [0.875, 0.74],
  ];

  ctx.strokeStyle = "rgba(100, 116, 139, 0.30)";
  ctx.lineWidth = 2.0;
  for (let i = 0; i < centers.length; i++) {
    for (let j = i + 1; j < centers.length; j++) {
      const dx = (centers[i][0] - centers[j][0]) * W;
      const dy = (centers[i][1] - centers[j][1]) * H;
      const d = Math.hypot(dx, dy);
      if (d > 40 && d < 136) {
        ctx.beginPath();
        ctx.moveTo(centers[i][0] * W, centers[i][1] * H);
        ctx.lineTo(centers[j][0] * W, centers[j][1] * H);
        ctx.stroke();
      }
    }
  }

  // Crisp non-overlapping geometric 5-point Pro Star / Pentagon panels (`IMG_0363`, `IMG_0366`)
  for (let idx = 0; idx < centers.length; idx++) {
    const [u, v] = centers[idx];
    const cx = u * W;
    const cy = v * H;
    const latStretch = 1.0 / Math.max(0.55, Math.sin(v * Math.PI));

    // Outer crimson/gold aero ring
    ctx.strokeStyle = idx % 2 === 0 ? "#dc2626" : "#eab308";
    ctx.lineWidth = 4.2;
    ctx.beginPath();
    ctx.ellipse(cx, cy, 27 * Math.min(1.25, latStretch), 25, 0, 0, Math.PI * 2);
    ctx.stroke();

    // Crisp Obsidian 5-point star-pentagon core
    ctx.fillStyle = "#0f172a";
    ctx.beginPath();
    for (let k = 0; k < 10; k++) {
      const ang = -Math.PI * 0.5 + (k * Math.PI) / 5;
      const rad = k % 2 === 0 ? 22 : 12.5;
      const px = cx + Math.cos(ang) * rad * Math.min(1.22, latStretch);
      const py = cy + Math.sin(ang) * rad * 0.94;
      if (k === 0) ctx.moveTo(px, py);
      else ctx.lineTo(px, py);
    }
    ctx.closePath();
    ctx.fill();

    // Crisp silver-gold inner pentagon badge
    ctx.strokeStyle = "#facc15";
    ctx.lineWidth = 1.8;
    ctx.beginPath();
    for (let k = 0; k < 5; k++) {
      const ang = -Math.PI * 0.5 + (k * Math.PI * 2) / 5;
      const px = cx + Math.cos(ang) * 9.5 * Math.min(1.2, latStretch);
      const py = cy + Math.sin(ang) * 9.0;
      if (k === 0) ctx.moveTo(px, py);
      else ctx.lineTo(px, py);
    }
    ctx.closePath();
    ctx.stroke();
  }

  const tex = new THREE.CanvasTexture(c);
  tex.colorSpace = THREE.SRGBColorSpace;
  tex.anisotropy = 4;
  return retainGpu(tex);
}

function createAdboardTexture(homeName, awayName, homeColor, awayColor, homeBanner, awayBanner) {
  const W = 2048;
  const H = 128;
  const c = document.createElement("canvas");
  c.width = W;
  c.height = H;
  const ctx = c.getContext("2d");

  // Authentic EA FIFA 10–12 high-contrast pitchside sponsor & club boards (`IMG_0351`, `IMG_0354`, `IMG_0359`)
  const panels = [
    { bg: "#f8fafc", fg: "#0f172a", accent: "#1d4ed8", text: "PARALLEL SEASON FC", sub: "OFFICIAL CHAMPIONSHIP" },
    { bg: homeColor || "#c8102e", fg: "#ffffff", accent: "#facc15", text: (homeName || "HOME").toUpperCase(), sub: homeBanner },
    { bg: "#fde047", fg: "#0f172a", accent: "#15803d", text: "RESPECT · FAIR PLAY", sub: "EVERY MATCH MATTERS" },
    { bg: "#0f172a", fg: "#ffffff", accent: "#38bdf8", text: "CLASSIC FUTBOL 3D", sub: "60HZ DETERMINISTIC SIM" },
    { bg: awayColor || "#1d4ed8", fg: "#ffffff", accent: "#ffffff", text: (awayName || "AWAY").toUpperCase(), sub: awayBanner },
    { bg: "#f8fafc", fg: "#111827", accent: "#dc2626", text: "5V5 & 11V11 STADIUM", sub: "VERIFIED CARTRIDGE SYNC" },
  ];

  const pw = W / panels.length;
  panels.forEach((p, idx) => {
    const x = idx * pw;
    ctx.fillStyle = p.bg;
    ctx.fillRect(x, 0, pw, H);
    ctx.fillStyle = p.accent;
    ctx.fillRect(x, 0, pw, 9);
    ctx.fillRect(x, H - 7, pw, 7);
    ctx.strokeStyle = "rgba(15,23,42,0.22)";
    ctx.lineWidth = 3;
    ctx.strokeRect(x + 2, 2, pw - 4, H - 4);

    ctx.fillStyle = p.fg;
    ctx.font = "900 34px 'Arial Narrow', Impact, sans-serif";
    ctx.textAlign = "center";
    ctx.fillText(p.text, x + pw * 0.5, 64);

    ctx.fillStyle = p.fg === "#ffffff" ? p.accent : "#334155";
    ctx.font = "800 16px Inter, Arial, sans-serif";
    ctx.fillText(p.sub, x + pw * 0.5, 98);
  });

  const tex = new THREE.CanvasTexture(c);
  tex.colorSpace = THREE.SRGBColorSpace;
  tex.wrapS = THREE.RepeatWrapping;
  return tex;
}

/**
 * Live 3D Stadium Jumbotron Scoreboard (`IMG_0359.jpeg`, `IMG_0347.jpeg`).
 * Mounted above the stadium bowl and updated dynamically whenever score or clock minute changes!
 */
function createJumbotronScoreboard(homeShort, awayShort, homeHex, awayHex) {
  const c = document.createElement("canvas");
  c.width = 512;
  c.height = 192;
  const ctx = c.getContext("2d");
  const tex = new THREE.CanvasTexture(c);
  tex.colorSpace = THREE.SRGBColorSpace;

  let lastKey = "";
  const update = (hScore, aScore, clockStr, phaseStr) => {
    const key = `${hScore}:${aScore}:${clockStr}:${phaseStr}`;
    if (key === lastKey) return;
    lastKey = key;

    ctx.fillStyle = "#050b16";
    ctx.fillRect(0, 0, 512, 192);

    ctx.strokeStyle = "#38bdf8";
    ctx.lineWidth = 5;
    ctx.strokeRect(6, 6, 500, 180);

    ctx.fillStyle = "#0f172a";
    ctx.fillRect(10, 10, 492, 36);
    ctx.fillStyle = "#facc15";
    ctx.font = "800 18px Inter, Arial, sans-serif";
    ctx.textAlign = "center";
    ctx.fillText(`PARALLEL SEASON FC  ·  ${phaseStr || "LIVE"}`, 256, 34);

    // Club color blocks
    ctx.fillStyle = homeHex || "#c8102e";
    ctx.fillRect(24, 62, 138, 72);
    ctx.fillStyle = awayHex || "#1d4ed8";
    ctx.fillRect(350, 62, 138, 72);

    ctx.fillStyle = "#ffffff";
    ctx.font = "900 44px 'Arial Narrow', Impact, sans-serif";
    ctx.fillText((homeShort || "HOM").slice(0, 3), 93, 113);
    ctx.fillText((awayShort || "AWY").slice(0, 3), 419, 113);

    // Center LED Score
    ctx.fillStyle = "#111827";
    ctx.fillRect(172, 56, 168, 84);
    ctx.fillStyle = "#4ade80";
    ctx.font = "900 58px 'Arial Narrow', Impact, sans-serif";
    ctx.fillText(`${hScore} - ${aScore}`, 256, 118);

    // Clock bar
    ctx.fillStyle = "#cbd5e1";
    ctx.font = "800 24px Inter, monospace";
    ctx.fillText(clockStr || "00:00", 256, 172);

    tex.needsUpdate = true;
  };

  update(0, 0, "00:00", "KICK OFF");
  return { tex, update };
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

function createShortsTexture(kit) {
  const c = document.createElement("canvas");
  c.width = 128;
  c.height = 128;
  const ctx = c.getContext("2d");
  ctx.fillStyle = kit.shorts || "#ffffff";
  ctx.fillRect(0, 0, 128, 128);

  // Lateral contrast piping strictly at u = 0 (`shortsLeft` / `shortsRight` maps u = 0 to outer lateral hip ONLY; u = 1 is clean inner groin!)
  ctx.fillStyle = kit.shortsStripe || kit.trim || "#ef4444";
  ctx.fillRect(0, 0, 6, 128);
  ctx.fillRect(9, 0, 4, 128);

  // Subtle ambient-occlusion drop shadow at the top of the shorts (`v -> 1`, canvas y = 0..24) cast by untucked jersey hem!
  const hemAo = ctx.createLinearGradient(0, 0, 0, 26);
  hemAo.addColorStop(0, "rgba(8, 12, 22, 0.30)");
  hemAo.addColorStop(0.55, "rgba(8, 12, 22, 0.11)");
  hemAo.addColorStop(1, "rgba(8, 12, 22, 0)");
  ctx.fillStyle = hemAo;
  ctx.fillRect(0, 0, 128, 26);

  // Subtle bottom shorts hem stitch (`v -> 0`, canvas y = 122..128)
  ctx.fillStyle = "rgba(15, 23, 42, 0.12)";
  ctx.fillRect(0, 123, 128, 5);

  const tex = new THREE.CanvasTexture(c);
  tex.colorSpace = THREE.SRGBColorSpace;
  return tex;
}

function createShortsNumberTexture(kit, shirtNumber) {
  const c = document.createElement("canvas");
  c.width = 64;
  c.height = 64;
  const ctx = c.getContext("2d");
  ctx.clearRect(0, 0, 64, 64);
  const numStr = String(shirtNumber || "");
  if (numStr) {
    ctx.font = "900 42px 'Arial Narrow', Impact, sans-serif";
    ctx.textAlign = "center";
    ctx.textBaseline = "middle";
    ctx.lineWidth = 4;
    ctx.strokeStyle = kit.shorts || "#ffffff";
    ctx.strokeText(numStr, 32, 34);
    ctx.fillStyle = kit.shortsStripe || kit.numberOutline || "#0f172a";
    ctx.fillText(numStr, 32, 34);
  }
  const tex = new THREE.CanvasTexture(c);
  tex.colorSpace = THREE.SRGBColorSpace;
  return tex;
}

function createGkGloveTexture(accentHex, side = 0) {
  const c = document.createElement("canvas");
  c.width = 256;
  c.height = 256;
  const ctx = c.getContext("2d");
  // High-visibility Neon-Volt/Obsidian (`IMG_0364`, `IMG_0366`) & Crisp White/Obsidian (`IMG_0358`, `IMG_0365`) pro latex GK gloves!
  const hiVis = side === 0 ? "#d9f99d" : "#f8fafc";
  const punchAccent = side === 0 ? "#eab308" : accentHex || "#38bdf8";

  ctx.fillStyle = hiVis;
  ctx.fillRect(0, 0, 256, 256);

  // Obsidian segmented backhand punch-zone ribs & wrist strap (`IMG_0358`, `IMG_0364`)
  ctx.fillStyle = "#0f172a";
  ctx.fillRect(0, 0, 256, 44); // Wrap-around neoprene wrist strap
  ctx.fillRect(18, 64, 220, 24);
  ctx.fillRect(24, 102, 208, 20);
  ctx.fillStyle = punchAccent;
  ctx.fillRect(0, 44, 256, 12);
  ctx.fillRect(28, 130, 200, 14);

  // Bright grip-latex palm zone
  ctx.fillStyle = "#ffffff";
  ctx.fillRect(22, 156, 212, 86);

  const tex = new THREE.CanvasTexture(c);
  tex.colorSpace = THREE.SRGBColorSpace;
  return tex;
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
      ctx.fillStyle = "rgba(148, 163, 184, 0.18)";
      for (let x = 20; x < 240; x += 24) ctx.fillRect(x, 0, 3.5, 256);
    } else if (kit.pattern === "vertical_stripes") {
      ctx.fillStyle = kit.trim || "#ffffff";
      for (let x = 16; x < 240; x += 44) {
        ctx.fillRect(x, 0, 20, 256);
      }
      if (isBack) {
        ctx.fillStyle = kit.shirt;
        ctx.fillRect(46, 56, 164, 152);
      }
    } else if (kit.pattern === "quarters" && !isBack) {
      ctx.fillStyle = kit.trim || "#047857";
      ctx.globalAlpha = 0.88;
      ctx.fillRect(0, 0, 128, 128);
      ctx.fillRect(128, 128, 128, 128);
      ctx.globalAlpha = 1.0;
    } else if (kit.pattern === "shoulder_stripes") {
      ctx.fillStyle = kit.trim;
      for (let s = 0; s < 3; s++) {
        ctx.fillRect(22 + s * 11, 0, 6, 48);
        ctx.fillRect(206 + s * 11, 0, 6, 48);
      }
    } else if (kit.pattern === "chevron_subtle") {
      ctx.strokeStyle = "rgba(15, 23, 42, 0.14)";
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
    shade.addColorStop(0.5, "rgba(255,255,255,0.07)");
    shade.addColorStop(0.82, "rgba(0,0,0,0)");
    shade.addColorStop(1, "rgba(0,0,0,0.22)");
    ctx.fillStyle = shade;
    ctx.fillRect(0, 0, 256, 256);

    // Crisp bottom untucked jersey hem stitch & subtle underside fold shadow (`v = 0` -> canvas y = 246..256)
    ctx.fillStyle = "rgba(10, 15, 28, 0.24)";
    ctx.fillRect(0, 248, 256, 8);
    ctx.fillStyle = kit.trim || "#ffffff";
    ctx.globalAlpha = 0.45;
    ctx.fillRect(0, 245, 256, 3);
    ctx.globalAlpha = 1.0;

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

/**
 * Builds an anatomically contoured cranial hair shell that sits cleanly above the brows in front (`z > 0.03`),
 * arches around the ears at the sides, and wraps ALL the way down to the occipital nape (`y = napeDrop`) in back!
 */
function createTailoredHairCapGeometry(napeDrop = -0.078, crownBoost = 0.008) {
  const g = new THREE.SphereGeometry(0.1065, 24, 18);
  const pos = g.attributes.position;
  for (let i = 0; i < pos.count; i++) {
    let x = pos.getX(i);
    let y = pos.getY(i);
    let z = pos.getZ(i);

    y = y * 1.15 + (y > 0.02 ? crownBoost * (y / 0.12) : 0);
    x *= 0.885;
    if (y < 0) {
      const t = Math.min(1, -y / 0.118);
      x *= 1.0 - t * 0.27;
      if (z < 0) z *= 1.0 - t * 0.24;
    }

    // Anatomical hairline contour from forehead (+Z) -> temple/ear arch -> occipital nape (-Z)
    let minHairY;
    if (z > 0.036) {
      minHairY = 0.045; // Clean forehead hairline above eyebrows
    } else if (z > -0.022) {
      // Ear arch & sideburn contour
      const earArch = Math.abs(z + 0.004) < 0.022 ? 0.022 : 0.008;
      const frontBlend = smoothstep(-0.022, 0.036, z);
      minHairY = earArch * (1 - frontBlend) + 0.045 * frontBlend;
    } else {
      const backBlend = smoothstep(-0.022, -0.062, z);
      minHairY = 0.018 * (1 - backBlend) + napeDrop * backBlend;
    }

    if (y < minHairY) {
      // Tuck sub-hairline cap vertices cleanly inside the opaque skull so only the sculpted hairline shows!
      y = minHairY;
      x *= 0.68;
      z *= 0.68;
    }

    pos.setXYZ(i, x, y, z);
  }
  g.computeVertexNormals();
  return g;
}

const HAIR_CAP_FULL = retainGpu(createTailoredHairCapGeometry(-0.078, 0.009));
const HAIR_CAP_FADE = retainGpu(createTailoredHairCapGeometry(-0.044, 0.005));

function buildHairMesh(style, hairColorHex, accentHex = "#f8fafc") {
  const mat = new THREE.MeshStandardMaterial({
    color: hairColorHex,
    roughness: 0.84,
    metalness: 0.02,
  });
  const group = new THREE.Group();

  if (style === "headgear") {
    // Iconic Petr Cech padded GK scrum-cap headgear (`IMG_0364.jpeg`, `IMG_0366.webp`)
    const gearMat = new THREE.MeshStandardMaterial({
      color: "#111827",
      roughness: 0.52,
      metalness: 0.08,
    });
    const shell = new THREE.Mesh(SHARED_GEO.gkHeadgear, gearMat);
    shell.scale.set(1.035, 1.025, 1.035);
    group.add(shell);
    const browPad = new THREE.Mesh(SHARED_GEO.headband, gearMat);
    browPad.position.set(0, 0.044, 0.004);
    browPad.scale.set(1.04, 1.04, 1.35);
    group.add(browPad);
    return group;
  }

  if (style === "buzz" || style === "fade") {
    group.add(new THREE.Mesh(HAIR_CAP_FADE, mat));
    if (style === "fade") {
      const topPad = new THREE.Mesh(SHARED_GEO.hairTopPad, mat);
      topPad.scale.set(0.95, 0.44, 1.08);
      topPad.position.set(0, 0.106, 0.006);
      group.add(topPad);
    }
  } else if (style === "curls") {
    group.add(new THREE.Mesh(HAIR_CAP_FULL, mat));
    const rings = [
      { r: 0.032, y: 0.116, n: 6, zOff: 0.006 },
      { r: 0.064, y: 0.096, n: 10, zOff: -0.004 },
      { r: 0.078, y: 0.062, n: 8, zOff: -0.018, backOnly: true },
    ];
    for (const row of rings) {
      for (let i = 0; i < row.n; i++) {
        const ang = (i / row.n) * Math.PI * 2 + row.y * 7.0;
        const cz = Math.sin(ang) * row.r + row.zOff;
        if (row.backOnly && cz > 0.028) continue;
        const puff = new THREE.Mesh(SHARED_GEO.hairCurl, mat);
        puff.position.set(
          Math.cos(ang) * row.r * 0.92,
          row.y + ((i % 2) - 0.5) * 0.006,
          cz
        );
        group.add(puff);
      }
    }
  } else if (style === "swept" || style === "headband") {
    group.add(new THREE.Mesh(HAIR_CAP_FULL, mat));
    const wave = new THREE.Mesh(SHARED_GEO.hairWave, mat);
    wave.scale.set(1.18, 0.46, 1.02);
    wave.position.set(0.008, 0.108, 0.020);
    wave.rotation.x = -0.14;
    wave.rotation.z = -0.08;
    group.add(wave);
    if (style === "headband") {
      // Athletic footballer hairband (`IMG_0365.jpeg` #3)
      const bandMat = new THREE.MeshStandardMaterial({
        color: accentHex || "#f8fafc",
        roughness: 0.48,
      });
      const band = new THREE.Mesh(SHARED_GEO.headband, bandMat);
      band.position.set(0, 0.052, 0.003);
      group.add(band);
    }
  } else if (style === "bun") {
    group.add(new THREE.Mesh(HAIR_CAP_FULL, mat));
    const bun = new THREE.Mesh(SHARED_GEO.hairBun, mat);
    bun.position.set(0, 0.054, -0.106);
    group.add(bun);
  } else {
    group.add(new THREE.Mesh(HAIR_CAP_FULL, mat));
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
  const styles = ["crop", "fade", "curls", "swept", "buzz", "headband"];
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
  const {
    kit,
    shirtNumber,
    surname,
    footballerId,
    clubShort,
    isGk,
    isReferee = false,
    side,
    scale = 1.06,
  } = spec;
  const faceSeed = hashStr(footballerId || String((shirtNumber || 9) + side * 31));
  const likeness = resolveLikeness(footballerId, (shirtNumber || 9) + side * 31, isGk);
  const soma = isReferee
    ? SOMATOTYPE_SCALES.athletic
    : SOMATOTYPE_SCALES[likeness.build] || SOMATOTYPE_SCALES.athletic;

  const clothNormScale = new THREE.Vector2(0.46, 0.46);
  const skinMat = new THREE.MeshStandardMaterial({ color: likeness.skinHex, roughness: 0.60 });
  const faceTex = createFaceSkinTexture(likeness.skinHex, likeness.hairHex, likeness.beard, faceSeed, 4);
  const headFaceMat = new THREE.MeshStandardMaterial({ map: faceTex, roughness: 0.62 });

  const sleeveMat = new THREE.MeshStandardMaterial({
    color: kit.sleeves,
    normalMap: SHARED_CLOTH_NORMAL,
    normalScale: clothNormScale,
    roughness: 0.58,
  });
  const trimMat = new THREE.MeshStandardMaterial({ color: kit.trim, roughness: 0.52 });
  const shortsTex = createShortsTexture(kit);
  const shortsMat = new THREE.MeshStandardMaterial({
    color: "#ffffff",
    map: shortsTex,
    normalMap: SHARED_CLOTH_NORMAL,
    normalScale: clothNormScale,
    roughness: 0.62,
  });
  const sockMat = new THREE.MeshStandardMaterial({ color: kit.socks, roughness: 0.68 });
  const sockBandMat = new THREE.MeshStandardMaterial({ color: kit.sockBand, roughness: 0.62 });

  // Authentic 2010–2012 Pro Football Boot Colorways (`IMG_0363.jpeg`, `IMG_0347.jpeg` — F50 Volt, Predator Black/Crimson, Vapor Blue/White, Total90 White/Red!)
  const bootPalette = isReferee
    ? { upper: "#111827", accent: "#f8fafc" }
    : [
        { upper: "#f8fafc", accent: "#dc2626" }, // Classic Pearl White / Crimson (`IMG_0347`)
        { upper: "#eab308", accent: "#0f172a" }, // Electric Volt Yellow / Obsidian (`IMG_0363`)
        { upper: "#1d4ed8", accent: "#f8fafc" }, // Royal Vapor Blue / Silver (`IMG_0363`)
        { upper: "#111827", accent: "#facc15" }, // Predator Black / Gold
        { upper: "#dc2626", accent: "#f8fafc" }, // Crimson / White
      ][faceSeed % 5];

  const bootCanvas = document.createElement("canvas");
  bootCanvas.width = 128;
  bootCanvas.height = 128;
  const bCtx = bootCanvas.getContext("2d");
  bCtx.fillStyle = bootPalette.upper;
  bCtx.fillRect(0, 0, 128, 128);
  // Dark carbon outsole rim & heel counter
  bCtx.fillStyle = "#0f172a";
  bCtx.fillRect(0, 0, 128, 28);
  bCtx.fillStyle = bootPalette.accent;
  bCtx.fillRect(0, 28, 128, 11);
  // Central tongue & speed-lace panel
  bCtx.fillStyle = "#0f172a";
  bCtx.fillRect(48, 40, 32, 52);
  bCtx.strokeStyle = bootPalette.accent;
  bCtx.lineWidth = 3.5;
  for (let ly = 46; ly <= 84; ly += 9) {
    bCtx.beginPath();
    bCtx.moveTo(46, ly);
    bCtx.lineTo(82, ly);
    bCtx.stroke();
  }
  const bootTex = new THREE.CanvasTexture(bootCanvas);
  bootTex.colorSpace = THREE.SRGBColorSpace;

  const bootMat = new THREE.MeshStandardMaterial({ map: bootTex, roughness: 0.26, metalness: 0.14 });
  const bootAccentMat = new THREE.MeshStandardMaterial({ color: bootPalette.accent, roughness: 0.30 });
  const soleMat = new THREE.MeshStandardMaterial({ color: "#090d16", roughness: 0.24, metalness: 0.36 });
  const studMat = new THREE.MeshStandardMaterial({ color: "#e2e8f0", metalness: 0.64, roughness: 0.22 });

  const jerseyTex = createJerseyTextures(
    kit,
    isReferee ? "" : shirtNumber,
    isReferee ? "" : surname,
    isReferee ? "REF" : clubShort
  );
  const torsoMats = [
    new THREE.MeshStandardMaterial({
      map: jerseyTex.front,
      normalMap: SHARED_CLOTH_NORMAL,
      normalScale: clothNormScale,
      roughness: 0.58,
    }),
    new THREE.MeshStandardMaterial({
      map: jerseyTex.back,
      normalMap: SHARED_CLOTH_NORMAL,
      normalScale: clothNormScale,
      roughness: 0.58,
    }),
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
  bones[B_HEAD].position.set(0, 0.054, 0); // Athletic neck height (eliminates giraffe-neck!)

  bones[B_SH_L].position.set(0.186, 0.125, 0); // y = 1.410
  bones[B_EL_L].position.set(0, -0.255, 0); // y = 1.155
  bones[B_SH_R].position.set(-0.186, 0.125, 0);
  bones[B_EL_R].position.set(0, -0.255, 0);

  bones[B_HIP_L].position.copy(HIP_SOCKET_L); // (+0.089, 0.922, 0)
  bones[B_KNEE_L].position.set(0, -UPPER_LEG_LEN, 0); // y = 0.487
  bones[B_ANKLE_L].position.set(0, -LOWER_LEG_LEN, 0); // y = 0.062
  bones[B_HIP_R].position.copy(HIP_SOCKET_R); // (-0.089, 0.922, 0)
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

  // 1. Continuous Skinned Jersey Torso (with untucked hem & cloth tension normalMap)
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

  // 3. Continuous Skinned Shorts + Quads + Knees + Calves
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

  // Crisp Upright Front-Left Thigh Squad Number Patch (`IMG_0347.jpeg`, `IMG_0363.jpeg`!)
  if (!isReferee && shirtNumber) {
    const sNumTex = createShortsNumberTexture(kit, shirtNumber);
    const sNumPatch = new THREE.Mesh(
      SHARED_GEO.sNumPlane,
      new THREE.MeshStandardMaterial({
        map: sNumTex,
        transparent: true,
        alphaTest: 0.08,
        roughness: 0.62,
        depthWrite: false,
      })
    );
    sNumPatch.position.set(0.012, -0.258, 0.098);
    sNumPatch.rotation.x = -0.05;
    sNumPatch.rotation.y = 0.14;
    bones[B_HIP_L].add(sNumPatch);
  }

  // Apply FIFA 11 Personality+ Somatotype & Format Scale AFTER skeleton bind!
  bodyScaleGroup.scale.set(soma.w * scale, soma.h * scale, soma.d * scale);

  // Collar, Neck & Sculpted 1 : 7.56 Head attached to Neck/Head Bones
  const collar = new THREE.Mesh(SHARED_GEO.collar, trimMat);
  collar.position.set(0, 0.002, -0.003);
  bones[B_NECK].add(collar);

  const neckMesh = new THREE.Mesh(SHARED_GEO.neck, skinMat);
  neckMesh.position.set(0, 0.028, -0.003);
  bones[B_NECK].add(neckMesh);

  const headMesh = new THREE.Mesh(SHARED_GEO.head, headFaceMat);
  headMesh.position.set(0, 0.086, 0);
  headMesh.castShadow = true;
  bones[B_HEAD].add(headMesh);

  const nose = new THREE.Mesh(SHARED_GEO.nose, skinMat);
  nose.position.set(0, 0.082, 0.098);
  bones[B_HEAD].add(nose);

  for (const sgn of [-1, 1]) {
    const ear = new THREE.Mesh(SHARED_GEO.ear, skinMat);
    ear.position.set(sgn * 0.090, 0.084, -0.004);
    bones[B_HEAD].add(ear);
  }

  const hair = buildHairMesh(likeness.style, likeness.hairHex, kit.trim);
  hair.position.set(0, 0.086, 0);
  bones[B_HEAD].add(hair);

  // Sculpted Anatomical Hands / Articulated Spread-Finger GK Latex Gloves (`IMG_0358`, `IMG_0364`, `IMG_0366`)
  const gloveTex = isGk ? createGkGloveTexture(kit.bootAccent || "#facc15", side) : null;
  const gloveMat = isGk ? new THREE.MeshStandardMaterial({ map: gloveTex, roughness: 0.34 }) : null;
  const tapeMat =
    !isGk && !isReferee && (shirtNumber === 7 || shirtNumber === 10 || shirtNumber === 5 || shirtNumber % 3 === 1)
      ? new THREE.MeshStandardMaterial({ color: "#f8fafc", roughness: 0.58 })
      : null;

  for (const [armIdx, elBone] of [
    [1, bones[B_EL_L]],
    [-1, bones[B_EL_R]],
  ]) {
    if (isGk) {
      const gloveGroup = new THREE.Group();
      gloveGroup.position.set(0, -0.236, 0.006);
      gloveGroup.scale.setScalar(1.14);
      const cuff = new THREE.Mesh(SHARED_GEO.gkCuff, gloveMat);
      cuff.position.set(0, 0.036, 0);
      gloveGroup.add(cuff);
      const palm = new THREE.Mesh(SHARED_GEO.gkGlove, gloveMat);
      palm.castShadow = true;
      gloveGroup.add(palm);
      for (let f = 0; f < 4; f++) {
        const fx = (f - 1.5) * 0.021;
        const finger = new THREE.Mesh(SHARED_GEO.gkFinger, gloveMat);
        finger.position.set(fx, -0.058, 0.006);
        finger.rotation.z = (f - 1.5) * -0.15; // Spread-finger shot-stopper ready hand (`IMG_0358`, `IMG_0364`)!
        finger.rotation.x = 0.10;
        gloveGroup.add(finger);
      }
      const thumb = new THREE.Mesh(SHARED_GEO.gkFinger, gloveMat);
      thumb.position.set(armIdx * -0.046, -0.024, 0.022);
      thumb.rotation.z = armIdx * 0.68;
      thumb.rotation.x = 0.22;
      gloveGroup.add(thumb);
      elBone.add(gloveGroup);
    } else {
      if (tapeMat) {
        const tape = new THREE.Mesh(SHARED_GEO.wristTape, tapeMat);
        tape.position.set(0, -0.218, 0.001);
        elBone.add(tape);
      }
      const handMesh = new THREE.Mesh(armIdx > 0 ? SHARED_GEO.handL : SHARED_GEO.handR, skinMat);
      handMesh.position.set(0, -0.234, 0.002);
      handMesh.castShadow = true;
      elBone.add(handMesh);
    }
  }

  // Flush-Sole 3D Cleats with Carbon Soleplate & 6 Metallic Studs (`IMG_0345`, `IMG_0354`, `IMG_0363`)
  for (const ankleBone of [bones[B_ANKLE_L], bones[B_ANKLE_R]]) {
    const cleat = new THREE.Mesh(SHARED_GEO.cleat, bootMat);
    cleat.position.set(0, 0, 0.018);
    cleat.castShadow = true;
    ankleBone.add(cleat);

    const sole = new THREE.Mesh(SHARED_GEO.solePlate, soleMat);
    sole.position.set(0, -0.050, 0.036);
    ankleBone.add(sole);

    const stripe = new THREE.Mesh(SHARED_GEO.bootStripe, bootAccentMat);
    stripe.position.set(0, -0.025, 0.03);
    ankleBone.add(stripe);

    for (const [sx, sz] of [
      [-0.022, -0.048],
      [0.022, -0.048],
      [-0.024, 0.042],
      [0.024, 0.042],
      [-0.020, 0.106],
      [0.020, 0.106],
    ]) {
      const st = new THREE.Mesh(SHARED_GEO.stud, studMat);
      st.position.set(sx, -0.055, sz);
      ankleBone.add(st);
    }
  }

  // Ground Contact Shadow & Minimalist EA FIFA 10–12 Broadcast Floor Indicators (`IMG_0350.webp`)
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
    opacity: 0.88,
    side: THREE.DoubleSide,
    depthWrite: false,
  });
  const ctrlRing = new THREE.Mesh(SHARED_GEO.ring, ringMat);
  ctrlRing.position.y = 0.022;
  ctrlRing.visible = false;
  root.add(ctrlRing);

  const aimWedge = new THREE.Mesh(SHARED_GEO.aimArrow, ringMat);
  aimWedge.position.set(0, 0.025, 0.53);
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
  overheadCursor.position.set(0, 2.02, 0);
  overheadCursor.visible = false;
  root.add(overheadCursor);

  const nameplateTex = createNameplateSpriteTexture(surname, shirtNumber, true, side);
  const nameplate = new THREE.Sprite(
    new THREE.SpriteMaterial({ map: nameplateTex, transparent: true, depthTest: false })
  );
  nameplate.scale.set(1.55, 0.38, 1);
  nameplate.position.set(0, 2.28, 0);
  nameplate.visible = false;
  root.add(nameplate);

  const phaseSeed = ((shirtNumber || 1) * 1.37 + Math.max(0, side) * 2.1) % (Math.PI * 2);

  return {
    root,
    bodyScaleGroup,
    bones,
    skinMat,
    shortsMat,
    sockMat,
    baseSocksColor: new THREE.Color(kit.socks),
    dirtLevel: 0,
    sweatLevel: 0,
    ctrlRing,
    aimWedge,
    chargeRing,
    chargeRingMat,
    overheadCursor,
    nameplate,
    contactShadow,
    isGk,
    isReferee,
    footballerId,
    shirtNumber,
    side,
    scale,
    phaseSeed,
    stridePhase: phaseSeed,
    prevAction: 0,
    kickSwingTimer: 0,
    smoothYaw: side === 0 ? Math.PI * 0.5 : -Math.PI * 0.5,
    smoothAccelFwd: 0,
    smoothAccelLat: 0,
    smoothBankRoll: 0,
    smoothHipY: HIPS_BIND_Y - 0.03,
    smoothJumpY: 0,
    footGoalL: new THREE.Vector3(0.095, SOLE_HEIGHT, 0.04),
    footGoalR: new THREE.Vector3(-0.095, SOLE_HEIGHT, -0.04),
  };
}

const _poleL = new THREE.Vector3(0.05, 0.18, 1.0).normalize();
const _poleR = new THREE.Vector3(-0.05, 0.18, 1.0).normalize();
const _mudTint = new THREE.Color("#5c5332");
const _whiteTint = new THREE.Color("#ffffff");

/**
 * Continuous CMU-Mocap-Tuned Walk -> Jog -> Sprint Foot Trajectory.
 * Smoothly blends from standing ready stance (speed = 0) through walk (< 1.8 m/s)
 * to aerial sprint (> 2.2 m/s) with zero threshold jumps!
 */
function evaluateContinuousFootTarget(phase, sideSign, speed, speedNorm, isGk, outVec) {
  const locoWeight = smoothstep(0.04, 0.72, speed);
  const idleX = sideSign * (isGk ? 0.155 : 0.094);
  const idleZ = sideSign > 0 ? 0.040 : -0.040;

  if (locoWeight <= 0.001) {
    outVec.set(idleX, SOLE_HEIGHT, idleZ);
    return 0.0;
  }

  const p = ((phase % (Math.PI * 2)) + Math.PI * 2) % (Math.PI * 2);
  const runX = sideSign * (isGk ? 0.128 : 0.088);
  const halfSpan = Math.min(0.56, 0.12 + speed * 0.055) * locoWeight;
  const runBlend = smoothstep(1.6, 3.8, speed);

  let gx = idleX + (runX - idleX) * locoWeight;
  let gy = SOLE_HEIGHT;
  let gz = 0;
  let pitch = 0;

  if (p < Math.PI) {
    const u = p / Math.PI;
    gz = halfSpan * (1.0 - 2.0 * u);
    const heelPitch = -0.25 * (0.45 + 0.55 * runBlend);
    const toeOffPitch = (0.36 + 0.28 * runBlend) * locoWeight;
    pitch =
      u < 0.22
        ? heelPitch * (1 - u / 0.22)
        : u > 0.62
          ? ((u - 0.62) / 0.38) * toeOffPitch
          : 0.0;
    const toeLiftY = u > 0.70 ? ((u - 0.70) / 0.30) * 0.048 * locoWeight : 0.0;
    gy = SOLE_HEIGHT + toeLiftY;
  } else {
    const w = (p - Math.PI) / Math.PI;
    gz = -halfSpan + 2.0 * halfSpan * (w - Math.sin(w * Math.PI * 2) * 0.055);
    // High hamstring back-swing fold so carbon soleplate & studs flash behind sprinters (`IMG_0345`, `IMG_0354`)!
    const peakLift = (0.062 + speedNorm * 0.34 * (0.35 + 0.65 * runBlend)) * locoWeight;
    gy = SOLE_HEIGHT + Math.sin(w * Math.PI) * peakLift;
    const heelFold = Math.sin(Math.min(1, w * 1.38) * Math.PI);
    pitch =
      (0.34 + 0.64 * runBlend) * heelFold * locoWeight -
      (w > 0.76 ? ((w - 0.76) / 0.24) * 0.26 * locoWeight : 0);
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
    aimRad = null,
    action,
    charge,
    jumpZ = 0,
    inWall = false,
    refSignal = 0,
    isControlled,
    ballX,
    ballY,
    ballZ = 0,
    oppDist = 99,
    oppWorldDx = 0,
    oppWorldDy = 0,
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

  // Smooth tick-derived world acceleration in athlete local coordinates
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

  // Physical Opponent Jostling Proximity (`IMG_0363.jpeg`, `IMG_0357.jpeg` — shoulder lean + shielding arm!)
  const jostleWeight = !actor.isReferee && !actor.isGk ? 1.0 - smoothstep(0.72, 1.45, oppDist) : 0;
  const oppLocalLeft = oppWorldDx * leftX + oppWorldDy * leftZ; // > 0 opponent on athlete's left, < 0 on right
  const oppSideSign = oppLocalLeft >= 0 ? 1 : -1;

  // Whole-body centripetal lateral bank when cutting/turning + leaning into shoulder-to-shoulder jostles!
  const goalBankRoll =
    action === 3 || action === 4 || action === 5
      ? 0
      : Math.max(
          -0.16,
          Math.min(
            0.16,
            (-actor.smoothAccelLat * 0.0085 - yawRate * 0.019) * locoWeight -
              oppSideSign * jostleWeight * 0.065
          )
        );
  damp(actor, "smoothBankRoll", goalBankRoll, 0.075, dt);
  actor.bodyScaleGroup.rotation.z = actor.smoothBankRoll;

  // Dynamic Match Sweat & Progressive Turf-Dirt Smudges (`IMG_0366.webp`, `IMG_0357.jpeg`)
  if (!actor.isReferee) {
    if (speed > 3.2 && actor.sweatLevel < 1.0) {
      actor.sweatLevel = Math.min(1.0, actor.sweatLevel + dt * 0.015);
      actor.skinMat.roughness = 0.60 - actor.sweatLevel * 0.16;
    }
    if ((action === 3 || action === 5 || action === 4) && actor.dirtLevel < 0.36) {
      actor.dirtLevel = Math.min(0.36, actor.dirtLevel + dt * 0.28);
      actor.shortsMat.color.copy(_whiteTint).lerp(_mudTint, actor.dirtLevel);
      actor.sockMat.color.copy(actor.baseSocksColor).lerp(_mudTint, actor.dirtLevel * 0.75);
    }
  }

  // Cleat turf divot / wet spray on sharp cuts, standing lunge tackles, slide tackles, low GK dives, or falls (`IMG_0365`, `IMG_0366`)
  if (
    particleSystem &&
    (Math.hypot(axTick, ayTick) > 16.5 ||
      action === 2 ||
      action === 3 ||
      action === 5 ||
      (action === 4 && jumpZ < 0.24)) &&
    Math.random() < dt * (action === 3 || action === 4 ? 28 : 18)
  ) {
    particleSystem.spawnBurst(
      wx,
      0.04,
      wy,
      -vx * 0.25,
      -vy * 0.25,
      action === 3 || action === 4 ? 6 : 3
    );
  }

  // Trigger continuous proximal-to-distal kick whip follow-through on release from Charge (1)
  if (actor.prevAction === 1 && action !== 1) {
    actor.kickSwingTimer = 0.26;
    if (particleSystem) {
      particleSystem.spawnBurst(wx + fwdX * 0.32, 0.04, wy + fwdZ * 0.32, fwdX * 2.1, fwdZ * 2.1, 4);
    }
  }
  actor.prevAction = action;
  if (actor.kickSwingTimer > 0) {
    actor.kickSwingTimer = Math.max(0, actor.kickSwingTimer - dt);
  }

  // Clean EA FIFA 10–12 Pitch Indicators (`IMG_0350.webp` — zero giant floating black billboards!)
  actor.ctrlRing.visible = Boolean(isControlled);
  actor.aimWedge.visible = Boolean(isControlled);
  actor.overheadCursor.visible = Boolean(isControlled);
  if (isControlled) {
    actor.overheadCursor.position.y = 2.02 + actor.smoothJumpY + Math.sin(nowSec * 7.0) * 0.035;
    actor.overheadCursor.rotation.y = nowSec * 3.0;
    const relAimYaw =
      aimRad !== null && charge > 0.02
        ? wrapAngleRad(Math.PI * 0.5 - aimRad - actor.root.rotation.y)
        : 0;
    const rAim = 0.53 + (charge > 0.02 ? charge * 0.18 : 0);
    actor.aimWedge.position.set(Math.sin(relAimYaw) * rAim, 0.025, Math.cos(relAimYaw) * rAim);
    actor.aimWedge.rotation.y = relAimYaw;
    const wScale = charge > 0.02 ? 1.0 + charge * 0.32 : 1.0;
    const lScale = charge > 0.02 ? 1.0 + charge * 0.55 : 1.0;
    actor.aimWedge.scale.set(wScale, 1.0, lScale);
  }
  actor.nameplate.visible = false;

  if (isControlled && charge > 0.02) {
    actor.chargeRing.visible = true;
    const sRing = 0.88 + charge * 0.28;
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
  const idleNoise = seededNoise2D(nowSec * 0.55, actor.phaseSeed * 3.1) * 0.05 * (1 - locoWeight);
  const headYawGoal = Math.max(-0.66, Math.min(0.66, toBallAng * 0.65 + yawRate * 0.045)) + idleNoise;
  dampBoneEuler(bones[B_HEAD], 0.04 - speedNorm * 0.08 - actor.smoothJumpY * 0.25, headYawGoal, 0, 0.075, dt);
  dampBoneEuler(bones[B_NECK], speedNorm * 0.06, headYawGoal * 0.25, 0, 0.075, dt);

  // Special Full-Body States (blended smoothly via dampQ so entry & exit never snap!)
  if (action === 3) {
    damp(actor, "smoothJumpY", 0, 0.05, dt);
    actor.bodyScaleGroup.position.y = actor.smoothJumpY;
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

  if (action === 5) {
    damp(actor, "smoothJumpY", 0, 0.05, dt);
    actor.bodyScaleGroup.position.y = actor.smoothJumpY;
    damp(actor, "smoothHipY", 0.38, 0.06, dt);
    bones[B_HIPS].position.set(0, actor.smoothHipY, 0);
    dampBoneEuler(bones[B_HIPS], 0.92, 0.14, 0.26, 0.06, dt);
    dampBoneEuler(bones[B_SPINE], 0.24, -0.08, -0.1, 0.06, dt);
    dampBoneEuler(bones[B_CHEST], 0.18, -0.06, 0, 0.06, dt);
    dampBoneEuler(bones[B_HIP_L], -0.68, 0, 0.14, 0.06, dt);
    dampBoneEuler(bones[B_KNEE_L], 1.38, 0, 0, 0.06, dt);
    dampBoneEuler(bones[B_ANKLE_L], 0.28, 0, 0, 0.06, dt);
    dampBoneEuler(bones[B_HIP_R], 0.24, 0, -0.18, 0.06, dt);
    dampBoneEuler(bones[B_KNEE_R], 0.95, 0, 0, 0.06, dt);
    dampBoneEuler(bones[B_ANKLE_R], 0.32, 0, 0, 0.06, dt);
    dampBoneEuler(bones[B_SH_L], -1.12, 0, 0.35, 0.06, dt);
    dampBoneEuler(bones[B_EL_L], -0.32, 0, 0, 0.06, dt);
    dampBoneEuler(bones[B_SH_R], -0.85, 0, -0.45, 0.06, dt);
    dampBoneEuler(bones[B_EL_R], -0.48, 0, 0, 0.06, dt);
    actor.contactShadow.scale.set(1.45, 1.35, 1);
    return;
  }

  if (action === 4) {
    // Dynamic-height GK Save Biomechanics:
    // - Airborne full-stretch horizontal layout dive (`jumpZ > 0.25m`, `IMG_0356.jpeg`, `IMG_0364.jpeg`)
    // - Low turf reaction lunge / wet-grass smother block (`jumpZ <= 0.25m`, `IMG_0358.webp`, `IMG_0366.webp`)
    const lateral = vx * Math.sin(facingRad) - vy * Math.cos(facingRad);
    const diveSign = lateral >= 0 ? -1 : 1;
    const targetDiveLift = 0.10 + jumpZ * 0.96;
    const highDive = smoothstep(0.16, 0.55, targetDiveLift);
    damp(actor, "smoothJumpY", targetDiveLift, 0.042, dt);
    actor.bodyScaleGroup.position.y = actor.smoothJumpY;
    damp(actor, "smoothHipY", 0.44 + highDive * 0.18, 0.045, dt);
    bones[B_HIPS].position.set(0, actor.smoothHipY, 0);
    dampBoneEuler(bones[B_HIPS], 0.04 * (1 - highDive), 0, diveSign * (1.28 + highDive * 0.18), 0.045, dt);
    dampBoneEuler(bones[B_SPINE], 0.03, 0, diveSign * 0.16, 0.045, dt);
    dampBoneEuler(bones[B_CHEST], 0.02, 0, diveSign * 0.16, 0.045, dt);
    // Asymmetrical full-extension lead-glove fingertip reach (`IMG_0356`, `IMG_0364`)!
    const lIsLead = diveSign < 0;
    dampBoneEuler(bones[B_SH_L], lIsLead ? -2.96 : -2.48, 0, 0.22, 0.045, dt);
    dampBoneEuler(bones[B_SH_R], !lIsLead ? -2.96 : -2.48, 0, -0.22, 0.045, dt);
    dampBoneEuler(bones[B_EL_L], lIsLead ? -0.06 : -0.24, 0, 0, 0.045, dt);
    dampBoneEuler(bones[B_EL_R], !lIsLead ? -0.06 : -0.24, 0, 0, 0.045, dt);
    dampBoneEuler(bones[B_HIP_L], -0.18, 0, 0.16, 0.05, dt);
    dampBoneEuler(bones[B_HIP_R], -0.10, 0, -0.16, 0.05, dt);
    dampBoneEuler(bones[B_KNEE_L], diveSign > 0 ? 0.68 - highDive * 0.22 : 0.18, 0, 0, 0.05, dt);
    dampBoneEuler(bones[B_KNEE_R], diveSign < 0 ? 0.68 - highDive * 0.22 : 0.18, 0, 0, 0.05, dt);
    dampBoneEuler(bones[B_ANKLE_L], 0.28, 0, 0, 0.05, dt);
    dampBoneEuler(bones[B_ANKLE_R], 0.28, 0, 0, 0.05, dt);
    const dShadow = Math.max(0.65, 1.55 - targetDiveLift * 0.65);
    actor.contactShadow.scale.set(dShadow * 1.15, dShadow * 0.82, 1);
    return;
  }

  // Smooth vertical jump elevation for GK High-Cross Plucks/Punches (`IMG_0365`) & Outfield Headers (`IMG_0348`)!
  damp(actor, "smoothJumpY", jumpZ, 0.042, dt);
  actor.bodyScaleGroup.position.y = actor.smoothJumpY;
  const airborneBlend = smoothstep(0.05, 0.34, actor.smoothJumpY);
  const shadowScale = Math.max(0.48, 1.0 - actor.smoothJumpY * 0.55);
  actor.contactShadow.scale.set(shadowScale, shadowScale, 1);

  if (airborneBlend > 0.28) {
    damp(actor, "smoothHipY", HIPS_BIND_Y - 0.02, 0.045, dt);
    bones[B_HIPS].position.set(0, actor.smoothHipY, 0);
    if (actor.isGk) {
      // Towering Goalkeeper High-Cross Leap (`IMG_0365.jpeg` Oblak: twin gloves skyward + protective raised knee!)
      dampBoneEuler(bones[B_HIPS], -0.08 * airborneBlend, 0, 0, 0.045, dt);
      dampBoneEuler(bones[B_SPINE], -0.10 * airborneBlend, 0, 0, 0.045, dt);
      dampBoneEuler(bones[B_CHEST], -0.10 * airborneBlend, 0, 0, 0.045, dt);
      dampBoneEuler(bones[B_HIP_L], 0.14 * airborneBlend, 0, 0.06, 0.045, dt);
      dampBoneEuler(bones[B_KNEE_L], 0.22 * airborneBlend, 0, 0, 0.045, dt);
      dampBoneEuler(bones[B_ANKLE_L], 0.34, 0, 0, 0.045, dt);
      dampBoneEuler(bones[B_HIP_R], -0.94 * airborneBlend, 0, -0.08, 0.045, dt);
      dampBoneEuler(bones[B_KNEE_R], 1.62 * airborneBlend, 0, 0, 0.045, dt);
      dampBoneEuler(bones[B_ANKLE_R], 0.28, 0, 0, 0.045, dt);
      dampBoneEuler(bones[B_SH_L], -2.84 * airborneBlend, 0, 0.20, 0.045, dt);
      dampBoneEuler(bones[B_SH_R], -2.80 * airborneBlend, 0, -0.20, 0.045, dt);
      dampBoneEuler(bones[B_EL_L], -0.16, 0, 0, 0.045, dt);
      dampBoneEuler(bones[B_EL_R], -0.18, 0, 0, 0.045, dt);
    } else {
      dampBoneEuler(bones[B_HIPS], -0.14 * airborneBlend, 0, 0, 0.05, dt);
      dampBoneEuler(bones[B_SPINE], -0.12 * airborneBlend, 0, 0, 0.05, dt);
      dampBoneEuler(bones[B_CHEST], -0.10 * airborneBlend, 0, 0, 0.05, dt);
      dampBoneEuler(bones[B_HIP_L], 0.18 * airborneBlend, 0, 0.08, 0.05, dt);
      dampBoneEuler(bones[B_KNEE_L], 0.96 * airborneBlend, 0, 0, 0.05, dt);
      dampBoneEuler(bones[B_ANKLE_L], 0.34, 0, 0, 0.05, dt);
      dampBoneEuler(bones[B_HIP_R], 0.28 * airborneBlend, 0, -0.08, 0.05, dt);
      dampBoneEuler(bones[B_KNEE_R], 1.12 * airborneBlend, 0, 0, 0.05, dt);
      dampBoneEuler(bones[B_ANKLE_R], 0.36, 0, 0, 0.05, dt);
      dampBoneEuler(bones[B_SH_L], -1.52 * airborneBlend, 0, 0.34, 0.05, dt);
      dampBoneEuler(bones[B_SH_R], -1.42 * airborneBlend, 0, -0.34, 0.05, dt);
      dampBoneEuler(bones[B_EL_L], -0.48, 0, 0, 0.05, dt);
      dampBoneEuler(bones[B_EL_R], -0.52, 0, 0, 0.05, dt);
    }
    return;
  }

  // Distance-Locked Stride Phase Integration (Zero Foot-Skating!)
  const strideLenMeters = 1.06 + speed * 0.22;
  if (speed > 0.05) {
    actor.stridePhase = (actor.stridePhase + ((speed * dt) / strideLenMeters) * Math.PI * 2) % (Math.PI * 2);
  }

  const s = Math.sin(actor.stridePhase);

  // Biomechanical CoM Sprint Lean, Centripetal Banking & Multi-Segment Thoracic Counter-Twist
  const leanPitch = Math.max(-0.18, Math.min(0.36, actor.smoothAccelFwd * 0.016 + speedNorm * 0.165));
  const leanRoll = Math.max(-0.22, Math.min(0.22, -actor.smoothAccelLat * 0.011 - yawRate * 0.024));

  let baseHipY = actor.isGk && speed < 0.4 ? 0.84 : inWall ? 0.895 : HIPS_BIND_Y - 0.028 - speedNorm * 0.032;
  if (action === 2) baseHipY = 0.65; // Deep split-leg standing tackle lunge (`IMG_0363.jpeg`)!
  const idleBreathe = seededNoise2D(actor.phaseSeed, nowSec * 0.9) * 0.007 * (1 - locoWeight);
  const runBounce = -Math.cos(actor.stridePhase * 2) * 0.035 * locoWeight * Math.min(1, speedNorm + 0.18);
  damp(actor, "smoothHipY", baseHipY + (action === 2 ? 0 : runBounce) + idleBreathe, 0.045, dt);
  bones[B_HIPS].position.set(0, actor.smoothHipY, 0);

  const pelvisYaw = action === 2 ? 0.24 : -s * 0.135 * locoWeight * Math.min(1, speedNorm + 0.22);
  const pelvisHipDrop = s * 0.050 * locoWeight * Math.min(1, speedNorm + 0.2);
  dampBoneEuler(
    bones[B_HIPS],
    action === 2 ? 0.22 : leanPitch * 0.40,
    pelvisYaw,
    leanRoll * 0.65 + pelvisHipDrop,
    0.05,
    dt
  );

  const stationaryFactor = 1.0 - smoothstep(0.3, 2.2, speed);
  const chargeCockBlend =
    action === 1 || charge > 0.03
      ? Math.min(1, smoothstep(0.52, 0.92, charge) + stationaryFactor * charge * 0.85)
      : 0;
  const swingProgress = actor.kickSwingTimer > 0 ? 1.0 - actor.kickSwingTimer / 0.26 : -1;
  const whipArc = swingProgress >= 0 ? Math.sin(swingProgress * Math.PI) : 0;

  let spinePitch = leanPitch * 0.42 + (actor.isGk && speed < 0.4 ? 0.12 : inWall ? 0.07 : 0.035);
  let spineYaw = s * 0.125 * locoWeight * Math.min(1, speedNorm + 0.25);
  let spineRoll = -pelvisHipDrop * 0.45;

  if (action === 2) {
    spinePitch += 0.22;
    spineYaw = -0.22;
    spineRoll = -0.10;
  } else if (chargeCockBlend > 0.01) {
    spineYaw -= 0.20 * chargeCockBlend;
    spineRoll += 0.08 * chargeCockBlend;
  } else if (whipArc > 0.01) {
    spinePitch -= 0.10 * whipArc;
    spineYaw += 0.18 * whipArc;
  }

  dampBoneEuler(bones[B_SPINE], spinePitch, spineYaw, spineRoll, 0.05, dt);
  dampBoneEuler(bones[B_CHEST], spinePitch * 0.98, spineYaw * 1.18, spineRoll * 0.65, 0.05, dt);

  // Evaluate Continuous Locomotion Foot Targets + Ball-Contact Inside-Foot Posture (`IMG_0347.jpeg`!)
  let pitchL = evaluateContinuousFootTarget(actor.stridePhase, 1, speed, speedNorm, actor.isGk, _tmpFootL);
  let pitchR = evaluateContinuousFootTarget(
    actor.stridePhase + Math.PI,
    -1,
    speed,
    speedNorm,
    actor.isGk,
    _tmpFootR
  );
  let yawL = 0;
  let yawR = 0;
  let rollL = -actor.smoothBankRoll * 0.35;
  let rollR = -actor.smoothBankRoll * 0.35;

  // Local ball offset relative to athlete root (`+X` = athlete left, `+Z` = athlete forward)
  const bDx = ballX - wx;
  const bDz = ballY - wy;
  const ballDistHoriz = Math.hypot(bDx, bDz);
  const ballLocalX = bDx * leftX + bDz * leftZ;
  const ballLocalZ = bDx * fwdX + bDz * fwdZ;
  const touchProx =
    !actor.isGk && !actor.isReferee && ballZ < 0.42 && ballLocalZ > -0.15
      ? 1.0 - smoothstep(0.32, 0.86, ballDistHoriz)
      : 0;

  if (inWall && speed < 0.3) {
    _tmpFootL.set(0.088, SOLE_HEIGHT, 0.01);
    _tmpFootR.set(-0.088, SOLE_HEIGHT, -0.01);
    pitchL = 0;
    pitchR = 0;
  } else if (action === 2) {
    // Deep split-leg standing tackle lunge (`IMG_0363.jpeg` Valencia defender!)
    _tmpFootR.set(-0.065, SOLE_HEIGHT + 0.015, 0.68);
    pitchR = -0.26;
    yawR = 0.34;
    _tmpFootL.set(0.12, SOLE_HEIGHT + 0.03, -0.50);
    pitchL = 0.58;
  } else if (chargeCockBlend > 0.01) {
    const cockR = _vEffTarget.set(-0.10, SOLE_HEIGHT + 0.34 * chargeCockBlend, -0.36 * chargeCockBlend);
    _tmpFootR.lerp(cockR, chargeCockBlend);
    pitchR = pitchR * (1 - chargeCockBlend) + 0.62 * chargeCockBlend;
    yawR = -0.18 * chargeCockBlend;
    if (stationaryFactor > 0.4) {
      _tmpFootL.set(0.125, SOLE_HEIGHT, 0.14 * chargeCockBlend);
      pitchL = -0.06 * chargeCockBlend;
    }
  } else if (whipArc > 0.01) {
    const strikeR = _vEffTarget.set(
      -0.085 + whipArc * 0.08,
      SOLE_HEIGHT + whipArc * 0.36,
      -0.12 + whipArc * 0.64
    );
    _tmpFootR.lerp(strikeR, Math.min(1, whipArc * 1.35));
    pitchR = -0.34 * whipArc;
    yawR = 0.32 * whipArc; // Follow-through across body (`IMG_0347`)!
  } else if (touchProx > 0.05) {
    // Active Dribble / Cushion Touch: nearer foot reaches subtly toward ball with external inside-of-boot turn (`IMG_0347.jpeg`!)
    const reachZ = Math.max(0.08, Math.min(0.46, ballLocalZ - 0.11));
    if (ballLocalX >= 0.02) {
      const touchGoalL = _vEffTarget.set(
        Math.max(0.05, Math.min(0.22, ballLocalX)),
        SOLE_HEIGHT + 0.035 * touchProx,
        reachZ
      );
      _tmpFootL.lerp(touchGoalL, touchProx * 0.55);
      yawL = -0.38 * touchProx; // Left boot turns outward to cushion with inside instep!
      pitchL = pitchL * (1 - touchProx * 0.5) - 0.10 * touchProx;
    } else {
      const touchGoalR = _vEffTarget.set(
        Math.min(-0.05, Math.max(-0.22, ballLocalX)),
        SOLE_HEIGHT + 0.035 * touchProx,
        reachZ
      );
      _tmpFootR.lerp(touchGoalR, touchProx * 0.55);
      yawR = 0.38 * touchProx; // Right boot turns outward (`IMG_0347.jpeg`!)
      pitchR = pitchR * (1 - touchProx * 0.5) - 0.10 * touchProx;
    }
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
    dt,
    yawL,
    rollL
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
    dt,
    yawR,
    rollR
  );

  // Continuous Quaternion-Damped Upper-Limb Angular-Momentum Counter-Balance & Shielding (`IMG_0347`, `IMG_0363`!)
  if (inWall && speed < 0.35) {
    // Classic FIFA Free-Kick Defensive Wall Shield Pose (`IMG_0352.jpeg`, `IMG_0359.jpeg`!)
    dampBoneEuler(bones[B_SH_L], -0.34, 0.36, -0.22, 0.06, dt);
    dampBoneEuler(bones[B_SH_R], -0.34, -0.36, 0.22, 0.06, dt);
    dampBoneEuler(bones[B_EL_L], -0.46, 0, -0.28, 0.06, dt);
    dampBoneEuler(bones[B_EL_R], -0.46, 0, 0.28, 0.06, dt);
  } else if (action === 2) {
    // Wide counter-balance lunge arms (`IMG_0363.jpeg`)
    dampBoneEuler(bones[B_SH_L], -0.38, 0.22, 0.82, 0.045, dt);
    dampBoneEuler(bones[B_EL_L], -0.48, 0, 0, 0.045, dt);
    dampBoneEuler(bones[B_SH_R], 0.28, -0.18, -0.78, 0.045, dt);
    dampBoneEuler(bones[B_EL_R], -0.56, 0, 0, 0.045, dt);
  } else if (refSignal > 0.05) {
    // Match Referee Advantage / Free-Kick / Whistle Arm Signal (`IMG_0354`, `IMG_0359`)
    dampBoneEuler(bones[B_SH_L], 0.08, 0, 0.14, 0.06, dt);
    dampBoneEuler(bones[B_EL_L], -0.55, 0, 0, 0.06, dt);
    dampBoneEuler(bones[B_SH_R], -2.35 * refSignal, 0, -0.22, 0.06, dt);
    dampBoneEuler(bones[B_EL_R], -0.18, 0, 0, 0.06, dt);
  } else if (celebrating) {
    dampBoneEuler(bones[B_SH_L], -2.35 + s * 0.22, 0, 0.44, 0.055, dt);
    dampBoneEuler(bones[B_SH_R], -2.35 - s * 0.22, 0, -0.44, 0.055, dt);
    dampBoneEuler(bones[B_EL_L], -0.26, 0, 0, 0.055, dt);
    dampBoneEuler(bones[B_EL_R], -0.26, 0, 0, 0.055, dt);
  } else if (chargeCockBlend > 0.25) {
    const w = chargeCockBlend;
    dampBoneEuler(bones[B_SH_L], -0.62 * w, 0.24 * w, 0.76 * w, 0.05, dt);
    dampBoneEuler(bones[B_EL_L], -0.52, 0, 0, 0.05, dt);
    dampBoneEuler(bones[B_SH_R], 0.38 * w, 0, -0.36, 0.05, dt);
    dampBoneEuler(bones[B_EL_R], -0.86, 0, 0, 0.05, dt);
  } else if (whipArc > 0.1) {
    dampBoneEuler(bones[B_SH_L], 0.48 * whipArc, 0, 0.62, 0.045, dt);
    dampBoneEuler(bones[B_SH_R], -0.68 * whipArc, 0, -0.44, 0.045, dt);
    dampBoneEuler(bones[B_EL_L], -0.72, 0, 0, 0.045, dt);
    dampBoneEuler(bones[B_EL_R], -0.82, 0, 0, 0.045, dt);
  } else {
    // Athletic ready stance & counter-balance arm flare (`IMG_0347`, `IMG_0363` — never stiff tin-soldier arms!)
    const cutBalanceFlare =
      Math.min(0.24, Math.abs(actor.smoothAccelLat) * 0.012 + Math.abs(yawRate) * 0.045) +
      touchProx * 0.18;
    const idleArmPitch = actor.isGk ? -0.52 : -0.06 + idleBreathe * 2.5;
    const idleArmAbd = actor.isGk ? 0.40 : 0.22 + cutBalanceFlare * 0.5;
    const idleElbow = actor.isGk ? -0.94 : -0.56;

    const amp = Math.min(1.10, 0.32 + speedNorm * 0.76);
    const runShPitchL = s * 0.86 * amp - 0.08;
    const runShPitchR = -s * 0.86 * amp - 0.08;
    const runAbdBase = 0.25 + speedNorm * 0.11 + cutBalanceFlare;
    const shieldL = oppSideSign > 0 ? jostleWeight * 0.42 : 0;
    const shieldR = oppSideSign < 0 ? jostleWeight * 0.42 : 0;
    const shieldPitchL = oppSideSign > 0 ? -jostleWeight * 0.38 : 0;
    const shieldPitchR = oppSideSign < 0 ? -jostleWeight * 0.38 : 0;

    const runElL = -0.82 - speedNorm * 0.46 - Math.max(0, -s) * 0.24 + shieldL * 0.22;
    const runElR = -0.82 - speedNorm * 0.46 - Math.max(0, s) * 0.24 + shieldR * 0.22;

    dampBoneEuler(
      bones[B_SH_L],
      idleArmPitch * (1 - locoWeight) + runShPitchL * locoWeight + shieldPitchL,
      0.08 * locoWeight,
      idleArmAbd * (1 - locoWeight) + runAbdBase * locoWeight + shieldL,
      0.045,
      dt
    );
    dampBoneEuler(
      bones[B_SH_R],
      -idleArmPitch * (1 - locoWeight) + runShPitchR * locoWeight + shieldPitchR,
      -0.08 * locoWeight,
      -(idleArmAbd * (1 - locoWeight) + runAbdBase * locoWeight + shieldR),
      0.045,
      dt
    );
    dampBoneEuler(
      bones[B_EL_L],
      idleElbow * (1 - locoWeight) + runElL * locoWeight,
      0,
      -0.14 * locoWeight,
      0.045,
      dt
    );
    dampBoneEuler(
      bones[B_EL_R],
      idleElbow * (1 - locoWeight) + runElR * locoWeight,
      0,
      0.14 * locoWeight,
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
    this.capacity = 180;
    const geo = retainGpu(new THREE.BoxGeometry(0.048, 0.048, 0.048));
    this.mat = retainGpu(
      new THREE.MeshLambertMaterial({
        color: "#36782b",
        transparent: true,
        opacity: 0.92,
      })
    );
    this.mesh = new THREE.InstancedMesh(geo, this.mat, this.capacity);
    this.mesh.instanceMatrix.setUsage(THREE.DynamicDrawUsage);
    this.mesh.frustumCulled = false;
    scene.add(this.mesh);
    this.wetMode = false;

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

  setWetMode(wet) {
    this.wetMode = Boolean(wet);
    // In rain mode (`IMG_0366.webp`), cleats & ball kick up fine white-cyan water droplet spray!
    this.mat.color.set(this.wetMode ? "#bae6fd" : "#36782b");
    this.mat.opacity = this.wetMode ? 0.72 : 0.92;
  }

  spawnBurst(x, y, z, baseVx, baseVz, count = 3) {
    const mult = this.wetMode ? 1.5 : 1.0;
    const n = Math.min(8, Math.round(count * mult));
    for (let k = 0; k < n; k++) {
      const p = this.particles[this.cursor];
      this.cursor = (this.cursor + 1) % this.capacity;
      p.life = (this.wetMode ? 0.30 : 0.36) + (k % 3) * 0.07;
      p.x = x + (((this.cursor * 17) % 11) - 5) * 0.032;
      p.y = y;
      p.z = z + (((this.cursor * 31) % 11) - 5) * 0.032;
      p.vx = baseVx + (((this.cursor * 13) % 9) - 4) * (this.wetMode ? 0.48 : 0.35);
      p.vy = (this.wetMode ? 1.9 : 1.55) + (k % 3) * 0.58;
      p.vz = baseVz + (((this.cursor * 23) % 9) - 4) * (this.wetMode ? 0.48 : 0.35);
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
        this.dummy.scale.setScalar(Math.min(1, p.life * 2.8) * (this.wetMode ? 0.78 : 1.0));
      }
      this.dummy.updateMatrix();
      this.mesh.setMatrixAt(i, this.dummy.matrix);
    }
    if (anyActive) {
      this.mesh.instanceMatrix.needsUpdate = true;
    }
  }
}

/**
 * Real-time 3D Falling Stadium Rain Streak System (`IMG_0366.webp`).
 * Renders 1,400 wind-slanted raindrop streaks inside a volume following the active play focus.
 */
class RainSystem {
  constructor(scene) {
    this.count = 1400;
    this.spanX = 68;
    this.spanY = 30;
    this.spanZ = 52;
    const positions = new Float32Array(this.count * 6);
    this.seeds = new Float32Array(this.count * 4);
    let rSeed = 0x77a1beef;
    const rand = () => {
      rSeed = (Math.imul(rSeed, 1664525) + 1013904223) >>> 0;
      return rSeed / 4294967296;
    };

    for (let i = 0; i < this.count; i++) {
      this.seeds[i * 4 + 0] = (rand() - 0.5) * this.spanX;
      this.seeds[i * 4 + 1] = rand() * this.spanY;
      this.seeds[i * 4 + 2] = (rand() - 0.5) * this.spanZ;
      this.seeds[i * 4 + 3] = 19.0 + rand() * 8.0; // fall speed m/s
    }

    const geo = retainGpu(new THREE.BufferGeometry());
    geo.setAttribute("position", new THREE.BufferAttribute(positions, 3));
    const mat = retainGpu(
      new THREE.LineBasicMaterial({
        color: "#dbeafe",
        transparent: true,
        opacity: 0.34,
        depthWrite: false,
      })
    );
    this.lines = new THREE.LineSegments(geo, mat);
    this.lines.frustumCulled = false;
    this.lines.visible = false;
    scene.add(this.lines);
  }

  setEnabled(enabled) {
    this.lines.visible = Boolean(enabled);
  }

  step(dt, focusX, focusZ) {
    if (!this.lines.visible) return;
    const attr = this.lines.geometry.attributes.position;
    const arr = attr.array;
    const windX = 2.4;
    const windZ = 0.9;
    const streakT = 0.036;

    for (let i = 0; i < this.count; i++) {
      const b4 = i * 4;
      let y = this.seeds[b4 + 1] - this.seeds[b4 + 3] * dt;
      if (y < 0.05) {
        y += this.spanY;
      }
      this.seeds[b4 + 1] = y;

      const x = focusX + this.seeds[b4 + 0];
      const z = focusZ + this.seeds[b4 + 2];
      const b6 = i * 6;
      arr[b6 + 0] = x;
      arr[b6 + 1] = y;
      arr[b6 + 2] = z;
      arr[b6 + 3] = x - windX * streakT;
      arr[b6 + 4] = y + this.seeds[b4 + 3] * streakT;
      arr[b6 + 5] = z - windZ * streakT;
    }
    attr.needsUpdate = true;
  }
}

/**
 * GPU-Instanced 3D Shaded Grass Blade Tuft Field (`IMG_0364.jpeg`, `IMG_0366.webp`).
 * Combines pitch-wide 3D grass tufts with a dense rolling detail patch centered around `(ballX, ballY)`
 * so close-up cameras, set pieces, and instant replays display genuine 3D grass blades around cleats!
 */
class GrassBladeField {
  constructor(halfL, halfW) {
    this.staticCount = 1300;
    this.detailCount = 1100;
    this.total = this.staticCount + this.detailCount;
    this.halfL = halfL;
    this.halfW = halfW;

    const mat = new THREE.MeshStandardMaterial({
      color: "#ffffff",
      roughness: 0.72,
      metalness: 0.02,
      side: THREE.FrontSide,
    });
    this.mesh = new THREE.InstancedMesh(SHARED_GEO.grassTuft, mat, this.total);
    this.mesh.instanceMatrix.setUsage(THREE.DynamicDrawUsage);
    this.mesh.receiveShadow = true;
    this.mesh.frustumCulled = false;

    this.detailOffsets = new Float32Array(this.detailCount * 4);
    const dummy = new THREE.Object3D();
    const cLight = new THREE.Color("#43822c");
    const cDark = new THREE.Color("#2d631d");
    const cTmp = new THREE.Color();

    let gSeed = 0x51a72010;
    const rand = () => {
      gSeed = (Math.imul(gSeed, 1664525) + 1013904223) >>> 0;
      return gSeed / 4294967296;
    };

    for (let i = 0; i < this.staticCount; i++) {
      const x = (rand() * 2 - 1) * (halfL - 0.3);
      const z = (rand() * 2 - 1) * (halfW - 0.3);
      dummy.position.set(x, 0.001, z);
      dummy.rotation.set(0, rand() * Math.PI * 2, 0);
      const sc = 0.85 + rand() * 0.45;
      dummy.scale.set(sc, sc, sc);
      dummy.updateMatrix();
      this.mesh.setMatrixAt(i, dummy.matrix);
      cTmp.copy(cDark).lerp(cLight, rand());
      this.mesh.setColorAt(i, cTmp);
    }

    for (let j = 0; j < this.detailCount; j++) {
      const r = Math.sqrt(rand()) * 7.6;
      const ang = rand() * Math.PI * 2;
      this.detailOffsets[j * 4 + 0] = Math.cos(ang) * r;
      this.detailOffsets[j * 4 + 1] = Math.sin(ang) * r;
      this.detailOffsets[j * 4 + 2] = rand() * Math.PI * 2;
      this.detailOffsets[j * 4 + 3] = 0.82 + rand() * 0.42;
      const idx = this.staticCount + j;
      dummy.position.set(this.detailOffsets[j * 4], 0.001, this.detailOffsets[j * 4 + 1]);
      dummy.rotation.set(0, this.detailOffsets[j * 4 + 2], 0);
      dummy.scale.setScalar(this.detailOffsets[j * 4 + 3]);
      dummy.updateMatrix();
      this.mesh.setMatrixAt(idx, dummy.matrix);
      cTmp.copy(cDark).lerp(cLight, rand());
      this.mesh.setColorAt(idx, cTmp);
    }
    this.mesh.instanceMatrix.needsUpdate = true;
    if (this.mesh.instanceColor) this.mesh.instanceColor.needsUpdate = true;
    this.dummy = dummy;
    this.lastPatchX = 999;
    this.lastPatchZ = 999;
  }

  updateFocus(focusX, focusZ, nowSec) {
    // Snap rolling detail patch on a coarse 1.5m grid + gentle wind sway so blades never crawl across grass!
    const snapX = Math.round(focusX / 1.5) * 1.5;
    const snapZ = Math.round(focusZ / 1.5) * 1.5;
    const sway = Math.sin(nowSec * 2.4) * 0.06;
    const dummy = this.dummy;
    for (let j = 0; j < this.detailCount; j++) {
      const b = j * 4;
      const wx = Math.max(-this.halfL, Math.min(this.halfL, snapX + this.detailOffsets[b + 0]));
      const wz = Math.max(-this.halfW, Math.min(this.halfW, snapZ + this.detailOffsets[b + 1]));
      dummy.position.set(wx, 0.001, wz);
      dummy.rotation.set(sway, this.detailOffsets[b + 2], sway * 0.5);
      dummy.scale.setScalar(this.detailOffsets[b + 3]);
      dummy.updateMatrix();
      this.mesh.setMatrixAt(this.staticCount + j, dummy.matrix);
    }
    this.mesh.instanceMatrix.needsUpdate = true;
  }
}

/**
 * Procedural Multi-Row Stadium Crowd Tapestry Texture (`IMG_0354`, `IMG_0358`, `IMG_0359`).
 * Eliminates dark empty concrete gaps between 3D instanced supporters so stands read as a packed stadium bowl!
 */
function createCrowdTierTexture(homeHex, awayHex) {
  const W = 512;
  const H = 128;
  const c = document.createElement("canvas");
  c.width = W;
  c.height = H;
  const ctx = c.getContext("2d");

  ctx.fillStyle = "#111827";
  ctx.fillRect(0, 0, W, H);

  // Subtle concrete terrace riser lip at top & bottom
  ctx.fillStyle = "#263346";
  ctx.fillRect(0, 0, W, 7);
  ctx.fillStyle = "#0b1120";
  ctx.fillRect(0, H - 10, W, 10);

  const jacketColors = [
    "#1e293b",
    "#0f172a",
    "#273549",
    "#334155",
    homeHex || "#991b1b",
    awayHex || "#1e3a8a",
    "#1f2937",
  ];
  let s = 0x44c19021;
  const rnd = () => {
    s = (Math.imul(s, 1664525) + 1013904223) >>> 0;
    return s / 4294967296;
  };

  for (const rowY of [44, 86, 118]) {
    for (let x = 6; x < W; x += 13) {
      const jx = x + (rnd() - 0.5) * 4;
      const col = jacketColors[Math.floor(rnd() * jacketColors.length)];
      ctx.fillStyle = col;
      ctx.globalAlpha = 0.44;
      ctx.beginPath();
      ctx.ellipse(jx, rowY, 7.2, 12.5, 0, Math.PI, Math.PI * 2);
      ctx.fill();

      ctx.fillStyle = SKIN_TONES[Math.floor(rnd() * SKIN_TONES.length)];
      ctx.globalAlpha = 0.48;
      ctx.beginPath();
      ctx.arc(jx, rowY - 13.5, 3.4, 0, Math.PI * 2);
      ctx.fill();
    }
  }
  ctx.globalAlpha = 1.0;

  const tex = new THREE.CanvasTexture(c);
  tex.colorSpace = THREE.SRGBColorSpace;
  tex.wrapS = THREE.RepeatWrapping;
  tex.wrapT = THREE.RepeatWrapping;
  tex.repeat.set(10, 1);
  return tex;
}

/**
 * Builds an authentic box-stanchion Goal Assembly (`IMG_0350`, `IMG_0351`, `IMG_0356`)
 * with crisp white braided nylon net panels, diagonal tension cables & localized vertex impact pocket deformation!
 */
function buildGoalAssembly(goalX, sign, goalHW, goalH, netTex, accentHex) {
  const group = new THREE.Group();
  const postMat = new THREE.MeshStandardMaterial({
    color: "#ffffff",
    roughness: 0.16,
    metalness: 0.28,
  });
  const stanchionMat = new THREE.MeshStandardMaterial({
    color: accentHex || "#334155",
    roughness: 0.42,
    metalness: 0.38,
  });
  const cableMat = new THREE.MeshStandardMaterial({
    color: "#cbd5e1",
    roughness: 0.35,
    metalness: 0.5,
  });

  const postR = 0.068;
  const depth = 1.90;
  const backX = goalX + sign * depth;

  const postGeo = new THREE.CylinderGeometry(postR, postR, goalH, 14);
  for (const zSign of [-1, 1]) {
    const post = new THREE.Mesh(postGeo, postMat);
    post.position.set(goalX, goalH * 0.5, zSign * goalHW);
    post.castShadow = true;
    group.add(post);

    // Classic FIFA rear stanchion poles, ground frame & diagonal tension wires (`IMG_0350`, `IMG_0351`)
    const rearX = backX + sign * 0.28;
    const rearZ = zSign * (goalHW + 0.16);
    const rearPost = new THREE.Mesh(new THREE.CylinderGeometry(0.036, 0.036, goalH * 1.04, 8), stanchionMat);
    rearPost.position.set(rearX, goalH * 0.52, rearZ);
    group.add(rearPost);

    const topBar = new THREE.Mesh(new THREE.CylinderGeometry(0.026, 0.026, depth * 0.96, 8), postMat);
    topBar.rotation.z = Math.PI * 0.5;
    topBar.position.set(goalX + sign * depth * 0.48, goalH * 0.99, zSign * goalHW);
    group.add(topBar);

    const groundSide = new THREE.Mesh(new THREE.CylinderGeometry(0.030, 0.030, depth, 8), postMat);
    groundSide.rotation.z = Math.PI * 0.5;
    groundSide.position.set(goalX + sign * depth * 0.5, 0.03, zSign * goalHW);
    group.add(groundSide);

    // Diagonal tension cable from top net corner to top of rear stanchion pole (`IMG_0350`, `IMG_0351`)
    const cableLen = Math.hypot(0.36, 0.16);
    const cable = new THREE.Mesh(new THREE.CylinderGeometry(0.012, 0.012, cableLen, 6), cableMat);
    cable.rotation.z = Math.PI * 0.5;
    cable.rotation.y = -sign * zSign * 0.42;
    cable.position.set(backX + sign * 0.10, goalH * 1.01, zSign * (goalHW + 0.08));
    group.add(cable);
  }

  const groundBack = new THREE.Mesh(new THREE.CylinderGeometry(0.032, 0.032, goalHW * 2, 8), postMat);
  groundBack.rotation.x = Math.PI * 0.5;
  groundBack.position.set(backX, 0.03, 0);
  group.add(groundBack);

  const crossbar = new THREE.Mesh(new THREE.CylinderGeometry(postR, postR, goalHW * 2 + postR * 2, 14), postMat);
  crossbar.rotation.x = Math.PI * 0.5;
  crossbar.position.set(goalX, goalH, 0);
  crossbar.castShadow = true;
  group.add(crossbar);

  const netMat = new THREE.MeshStandardMaterial({
    map: netTex,
    transparent: true,
    alphaTest: 0.08,
    opacity: 0.94,
    emissive: "#e2e8f0",
    emissiveIntensity: 0.10,
    side: THREE.DoubleSide,
    roughness: 0.72,
    depthWrite: false,
  });

  const netHolder = new THREE.Group();
  netHolder.position.set(goalX, 0, 0);
  group.add(netHolder);

  const roofGeo = new THREE.PlaneGeometry(depth * 0.90, goalHW * 2, 10, 14);
  const roofNet = new THREE.Mesh(roofGeo, netMat);
  roofNet.rotation.x = -Math.PI * 0.5;
  roofNet.position.set(sign * depth * 0.45, goalH - 0.02, 0);
  netHolder.add(roofNet);

  const backSlopeLen = Math.hypot(depth * 0.14, goalH);
  const backGeo = new THREE.PlaneGeometry(goalHW * 2, backSlopeLen, 18, 12);
  const backBase = new Float32Array(backGeo.attributes.position.array);
  const backNet = new THREE.Mesh(backGeo, netMat);
  backNet.rotation.y = sign > 0 ? -Math.PI * 0.5 : Math.PI * 0.5;
  backNet.rotation.x = sign * 0.12;
  backNet.position.set(sign * depth * 0.93, goalH * 0.5, 0);
  netHolder.add(backNet);

  for (const zSign of [-1, 1]) {
    const sideNet = new THREE.Mesh(new THREE.PlaneGeometry(depth, goalH, 10, 8), netMat);
    sideNet.position.set(sign * depth * 0.5, goalH * 0.5, zSign * goalHW);
    netHolder.add(sideNet);
  }

  // Deforms `backNet` vertices into a localized 3D pocket around `(impactY, impactZ)`!
  const deformNet = (strength, impactZ, impactY, phaseRad) => {
    const pos = backGeo.attributes.position;
    const arr = pos.array;
    if (strength <= 0.004) {
      arr.set(backBase);
      pos.needsUpdate = true;
      return;
    }
    const localU = sign > 0 ? impactZ : -impactZ;
    const localV = impactY - goalH * 0.5;
    const osc = (0.62 + 0.38 * Math.sin(phaseRad)) * strength;
    for (let i = 0; i < pos.count; i++) {
      const bx = backBase[i * 3 + 0];
      const by = backBase[i * 3 + 1];
      const bz = backBase[i * 3 + 2];
      // Pin outer frame border so net stays attached to crossbar, posts & ground!
      const edgeU = 1.0 - Math.min(1, Math.pow(Math.abs(bx) / Math.max(0.5, goalHW), 3));
      const edgeV = 1.0 - Math.min(1, Math.pow(Math.abs(by) / Math.max(0.5, backSlopeLen * 0.5), 3));
      const d2 = (bx - localU) * (bx - localU) + (by - localV) * (by - localV);
      const pocket = Math.exp(-d2 / 1.35) * 0.72 * osc * edgeU * edgeV;
      arr[i * 3 + 0] = bx;
      arr[i * 3 + 1] = by - pocket * 0.12;
      arr[i * 3 + 2] = bz - pocket; // pushes outward behind goal!
    }
    pos.needsUpdate = true;
  };

  return { group, netHolder, deformNet, goalHW, goalH };
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
    new THREE.PlaneGeometry(pitchLen + 92, pitchWid + 92),
    new THREE.MeshStandardMaterial({ color: "#141c27", roughness: 0.92 })
  );
  trackFloor.rotation.x = -Math.PI * 0.5;
  trackFloor.position.y = -0.02;
  trackFloor.receiveShadow = true;
  stadium.add(trackFloor);

  const trackBorder = new THREE.Mesh(
    new THREE.PlaneGeometry(pitchLen + 17.5, pitchWid + 16.0),
    new THREE.MeshStandardMaterial({ color: "#223d1e", roughness: 0.9 })
  );
  trackBorder.rotation.x = -Math.PI * 0.5;
  trackBorder.position.y = -0.01;
  trackBorder.receiveShadow = true;
  stadium.add(trackBorder);

  const { tex: pitchTex, normalMap: pitchNorm, roughnessMap: pitchRough, totalW, totalH } =
    createPitchPbrMaps(pitchLen, pitchWid, fmt, maxAnisotropy);
  const pitchMat = new THREE.MeshStandardMaterial({
    map: pitchTex,
    normalMap: pitchNorm,
    normalScale: new THREE.Vector2(0.38, 0.38),
    roughnessMap: pitchRough,
    roughness: 0.82,
    metalness: 0.02,
  });
  const pitchMesh = new THREE.Mesh(new THREE.PlaneGeometry(totalW, totalH), pitchMat);
  pitchMesh.rotation.x = -Math.PI * 0.5;
  pitchMesh.receiveShadow = true;
  stadium.add(pitchMesh);

  // 3D Instanced Manicured Grass Micro-Blades (`IMG_0351`, `IMG_0364`, `IMG_0366`)
  const grassField = new GrassBladeField(halfL, halfW);
  stadium.add(grassField.mesh);

  const netTex = createGoalNetTexture(maxAnisotropy || 4);
  const leftGoal = buildGoalAssembly(-halfL, -1, goalHW, goalH, netTex, kits.homeOutfield.shirt);
  const rightGoal = buildGoalAssembly(halfL, 1, goalHW, goalH, netTex, kits.awayOutfield.shirt);
  stadium.add(leftGoal.group);
  stadium.add(rightGoal.group);

  // Pitchside Broadcast TV & Photographer Camera Tripods behind bylines (`IMG_0351.jpeg`!)
  const camBodyMat = new THREE.MeshStandardMaterial({ color: "#0f172a", roughness: 0.35, metalness: 0.55 });
  const tripodLegMat = new THREE.MeshStandardMaterial({ color: "#475569", roughness: 0.4, metalness: 0.65 });
  for (const [cx, cz, yaw] of [
    [-halfL - 2.35, halfW * 0.58, Math.PI * 0.42],
    [-halfL - 2.35, -halfW * 0.58, Math.PI * 0.58],
    [halfL + 2.35, halfW * 0.58, -Math.PI * 0.42],
    [halfL + 2.35, -halfW * 0.58, -Math.PI * 0.58],
  ]) {
    const camRig = new THREE.Group();
    camRig.position.set(cx, 0, cz);
    camRig.rotation.y = yaw;
    for (const [lx, lz, rx, rz] of [
      [-0.22, 0.18, 0.22, 0.24],
      [0.22, 0.18, 0.22, -0.24],
      [0, -0.24, -0.26, 0],
    ]) {
      const leg = new THREE.Mesh(new THREE.CylinderGeometry(0.018, 0.014, 0.98, 6), tripodLegMat);
      leg.position.set(lx, 0.46, lz);
      leg.rotation.set(rx, 0, rz);
      camRig.add(leg);
    }
    const camBox = new THREE.Mesh(new THREE.BoxGeometry(0.32, 0.24, 0.54), camBodyMat);
    camBox.position.set(0, 1.02, 0.06);
    camBox.castShadow = true;
    camRig.add(camBox);
    const lens = new THREE.Mesh(new THREE.CylinderGeometry(0.085, 0.075, 0.22, 10), camBodyMat);
    lens.rotation.x = Math.PI * 0.5;
    lens.position.set(0, 1.02, 0.38);
    camRig.add(lens);
    stadium.add(camRig);
  }

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
    emissiveIntensity: 0.16,
    roughness: 0.45,
  });

  const adDistZ = halfW + 4.25;
  const adDistX = halfL + 4.65;
  const adH = 1.05;

  const northAd = new THREE.Mesh(new THREE.BoxGeometry(adDistX * 2, adH, 0.18), adMat);
  northAd.position.set(0, adH * 0.5, -adDistZ);
  northAd.castShadow = true;
  stadium.add(northAd);

  const southAd = new THREE.Mesh(new THREE.BoxGeometry(adDistX * 2, adH * 0.78, 0.18), adMat);
  southAd.position.set(0, adH * 0.39, adDistZ + 0.85);
  stadium.add(southAd);

  for (const xSign of [-1, 1]) {
    const endAd = new THREE.Mesh(new THREE.BoxGeometry(0.18, adH, adDistZ * 2), adMat);
    endAd.position.set(xSign * adDistX, adH * 0.5, 0);
    endAd.castShadow = true;
    stadium.add(endAd);
  }

  const crowdTierTex = createCrowdTierTexture(kits.homeOutfield.shirt, kits.awayOutfield.shirt);
  const tierCrowdMat = new THREE.MeshStandardMaterial({
    map: crowdTierTex,
    roughness: 0.84,
  });
  const riserDarkMat = new THREE.MeshStandardMaterial({ color: "#0f172a", roughness: 0.90 });
  const stairMat = new THREE.MeshStandardMaterial({ color: "#64748b", roughness: 0.82 });
  // Translucent Sky-Blue Glass & White Steel Truss Cantilevered Roof Canopy (`IMG_0348.jpeg`, `IMG_0364.jpeg`!)
  const roofGlassMat = new THREE.MeshStandardMaterial({
    color: "#64748b",
    transparent: true,
    opacity: 0.62,
    roughness: 0.22,
    metalness: 0.28,
  });
  const trussMat = new THREE.MeshStandardMaterial({ color: "#cbd5e1", roughness: 0.42, metalness: 0.45 });
  const fasciaMatHome = new THREE.MeshStandardMaterial({
    color: kits.homeOutfield.shirt,
    emissive: kits.homeOutfield.shirt,
    emissiveIntensity: 0.18,
    roughness: 0.50,
  });
  const fasciaMatAway = new THREE.MeshStandardMaterial({
    color: kits.awayOutfield.shirt,
    emissive: kits.awayOutfield.shirt,
    emissiveIntensity: 0.18,
    roughness: 0.50,
  });
  // Illuminated Mid-Tier LED Stadium Ribbon (`IMG_0359.jpeg`!)
  const midRibbonMat = new THREE.MeshStandardMaterial({
    color: "#f59e0b",
    emissive: "#d97706",
    emissiveIntensity: 0.42,
    roughness: 0.40,
  });

  // Upper South Stand & South-Corner group: visible in 360° field/replay/pro/set-piece cameras,
  // automatically hidden ONLY when high exterior gantry cameras (`TELE`, `CO-OP`, `DYNAMIC`) sit behind the +Z stand!
  const southStandUpper = new THREE.Group();
  stadium.add(southStandUpper);

  const crowdPositions = [];
  const crowdColors = [];
  const crowdHeadColors = [];
  const southCrowdPositions = [];
  const southCrowdColors = [];
  const southCrowdHeadColors = [];

  // Muted broadcast-authentic supporter jackets & club shirts (`IMG_0354`, `IMG_0358`, `IMG_0359` — zero yellow clash-kit Lego blocks!)
  const homeSupporterCol = new THREE.Color(
    snapshot.home.primary_color || kits.homeOutfield.shirt
  ).multiplyScalar(0.50);
  const awaySupporterCol = new THREE.Color(
    snapshot.away.primary_color || kits.awayOutfield.shirt
  ).multiplyScalar(0.50);
  const palette = [
    homeSupporterCol,
    new THREE.Color("#1e293b"),
    awaySupporterCol,
    new THREE.Color("#0f172a"),
    new THREE.Color("#273549"),
    new THREE.Color("#334155"),
    new THREE.Color("#475569"),
    new THREE.Color("#64748b").multiplyScalar(0.75),
  ];
  const headPalette = [
    new THREE.Color("#cfa07a"),
    new THREE.Color("#b5835d"),
    new THREE.Color("#855737"),
    new THREE.Color("#4a3121"),
    new THREE.Color("#241b16"),
    new THREE.Color("#dca982"),
  ];

  let cSeed = 0x99fa2026;
  const cRand = () => {
    cSeed = (Math.imul(cSeed, 1664525) + 1013904223) >>> 0;
    return cSeed / 4294967296;
  };
  const _yAxis = new THREE.Vector3(0, 1, 0);

  function buildStand(length, startDist, rotY, tiers, addRoof, isHomeBias, isSouth = false, originXZ = null) {
    const ox = originXZ ? originXZ[0] : 0;
    const oz = originXZ ? originXZ[1] : 0;

    const standLower = new THREE.Group();
    standLower.position.set(ox, 0, oz);
    standLower.rotation.y = rotY;
    stadium.add(standLower);

    let standUpper = standLower;
    if (isSouth) {
      standUpper = new THREE.Group();
      standUpper.position.set(ox, 0, oz);
      standUpper.rotation.y = rotY;
      southStandUpper.add(standUpper);
    }

    const stepDepth = 1.04;
    const stepRise = 0.62;

    const wall = new THREE.Mesh(new THREE.BoxGeometry(length, 1.55, 0.6), riserDarkMat);
    wall.position.set(0, 0.775, -startDist);
    standLower.add(wall);

    const ribbon = new THREE.Mesh(
      new THREE.BoxGeometry(length, 0.26, 0.66),
      isHomeBias ? fasciaMatHome : fasciaMatAway
    );
    ribbon.position.set(0, 1.44, -startDist);
    standLower.add(ribbon);

    for (let t = 0; t < tiers; t++) {
      const targetGrp = isSouth && t >= 2 ? standUpper : standLower;
      const targetPosArr = isSouth && t >= 2 ? southCrowdPositions : crowdPositions;
      const targetColArr = isSouth && t >= 2 ? southCrowdColors : crowdColors;
      const targetHeadArr = isSouth && t >= 2 ? southCrowdHeadColors : crowdHeadColors;

      const y = 1.55 + t * stepRise;
      const z = -startDist - 0.58 - t * stepDepth;
      const tierMesh = new THREE.Mesh(
        new THREE.BoxGeometry(length, stepRise + 0.16, stepDepth + 0.12),
        tierCrowdMat
      );
      tierMesh.position.set(0, y, z);
      tierMesh.receiveShadow = true;
      targetGrp.add(tierMesh);

      if (t === 5) {
        // Mid-Bowl Illuminated Fascia Ribbon (`IMG_0359.jpeg`!)
        const midRib = new THREE.Mesh(new THREE.BoxGeometry(length + 0.2, 0.26, 0.22), midRibbonMat);
        midRib.position.set(0, y + 0.24, z + stepDepth * 0.54);
        targetGrp.add(midRib);
      }

      const rowShadowShade = Math.max(0.32, 0.90 - (t / Math.max(1, tiers)) * 0.54);
      const spacing = 0.41;
      const count = Math.floor((length - 1.4) / spacing);
      for (let i = 0; i < count; i++) {
        const lx = -((count - 1) * spacing) * 0.5 + i * spacing;
        if (length > 22 && (i % 28 === 14 || i % 28 === 15)) {
          if (i % 28 === 14) {
            const stair = new THREE.Mesh(
              new THREE.BoxGeometry(0.84, stepRise + 0.18, stepDepth + 0.14),
              stairMat
            );
            stair.position.set(lx + spacing * 0.5, y + 0.02, z + 0.03);
            targetGrp.add(stair);
          }
          continue;
        }
        const jitterX = lx + (cRand() - 0.5) * 0.12;
        const ly = y + stepRise * 0.56;
        const lz = z + 0.06;

        const v = new THREE.Vector3(jitterX, ly, lz).applyAxisAngle(_yAxis, rotY);
        v.x += ox;
        v.z += oz;
        targetPosArr.push({ x: v.x, y: v.y, z: v.z, rotY, phase: cRand() * Math.PI * 2 });

        const r = cRand();
        const baseCol =
          r < 0.28
            ? isHomeBias
              ? palette[0]
              : palette[2]
            : r < 0.36
              ? isHomeBias
                ? palette[2]
                : palette[0]
              : palette[1 + Math.floor(cRand() * 7)];
        targetColArr.push(baseCol.clone().multiplyScalar(rowShadowShade * (0.82 + cRand() * 0.26)));

        const hCol = headPalette[Math.floor(cRand() * headPalette.length)];
        targetHeadArr.push(hCol.clone().multiplyScalar(rowShadowShade * (0.78 + cRand() * 0.22)));
      }
    }

    const backZ = -startDist - 0.58 - tiers * stepDepth;
    const topY = 1.55 + tiers * stepRise;
    const backWall = new THREE.Mesh(new THREE.BoxGeometry(length + 3.0, topY + 4.2, 1.0), riserDarkMat);
    backWall.position.set(0, (topY + 4.2) * 0.5, backZ - 0.5);
    standUpper.add(backWall);

    if (addRoof) {
      const roofDepth = tiers * stepDepth + 3.8;
      const canopyZ = backZ + roofDepth * 0.5 - 0.6;
      const canopy = new THREE.Mesh(new THREE.BoxGeometry(length + 3.2, 0.22, roofDepth), roofGlassMat);
      canopy.position.set(0, topY + 4.15, canopyZ);
      canopy.rotation.x = 0.06;
      standUpper.add(canopy);

      const ribCount = Math.max(3, Math.floor(length / 8.5));
      for (let rb = 0; rb <= ribCount; rb++) {
        const rx = -length * 0.5 + (rb / ribCount) * length;
        const rib = new THREE.Mesh(new THREE.BoxGeometry(0.28, 0.36, roofDepth + 0.2), trussMat);
        rib.position.set(rx, topY + 4.15, canopyZ);
        rib.rotation.x = 0.06;
        standUpper.add(rib);
      }

      const fascia = new THREE.Mesh(new THREE.BoxGeometry(length + 3.2, 0.52, 0.3), trussMat);
      fascia.position.set(0, topY + 3.88, backZ + roofDepth - 0.65);
      standUpper.add(fascia);
    }
  }

  // 4 Main Touchline & Goal-End Stands + 4 Angled 45° Corner Wedges -> 100% Enclosed 360° Bowl (`IMG_0345`, `IMG_0354`, `IMG_0359`)!
  const standSpanX = (adDistX + 4.0) * 2;
  const standSpanZ = (adDistZ + 3.6) * 2;
  buildStand(standSpanX, adDistZ + 2.4, 0, 11, true, true, false); // North Stand
  buildStand(standSpanZ, adDistX + 2.8, Math.PI * 0.5, 11, true, true, false); // West Home End
  buildStand(standSpanZ, adDistX + 2.8, -Math.PI * 0.5, 11, true, false, false); // East Away End
  buildStand(standSpanX, adDistZ + 2.8, Math.PI, 11, true, false, true); // Full Roofed South Stand!

  const cornerSpan = 18.5;
  buildStand(cornerSpan, 0, Math.PI * 0.25, 11, true, true, false, [-(adDistX + 2.8), -(adDistZ + 2.5)]); // NW
  buildStand(cornerSpan, 0, -Math.PI * 0.25, 11, true, false, false, [adDistX + 2.8, -(adDistZ + 2.5)]); // NE
  buildStand(cornerSpan, 0, Math.PI * 0.75, 11, true, true, true, [-(adDistX + 2.8), adDistZ + 2.7]); // SW
  buildStand(cornerSpan, 0, -Math.PI * 0.75, 11, true, false, true, [adDistX + 2.8, adDistZ + 2.7]); // SE

  // Live 3D Stadium Jumbotron Scoreboards mounted on North & Goal-End Roof Fascias (`IMG_0359.jpeg`!)
  const jumbotron = createJumbotronScoreboard(
    snapshot.home.short_name,
    snapshot.away.short_name,
    kits.homeOutfield.shirt,
    kits.awayOutfield.shirt
  );
  const jumboScreenMat = new THREE.MeshStandardMaterial({
    map: jumbotron.tex,
    emissive: "#ffffff",
    emissiveMap: jumbotron.tex,
    emissiveIntensity: 0.75,
    roughness: 0.3,
  });
  const jumboFrameMat = new THREE.MeshStandardMaterial({ color: "#090f1c", roughness: 0.5, metalness: 0.4 });
  for (const [jx, jy, jz, jYaw] of [
    [0, 9.8, -(adDistZ + 11.4), 0],
    [-(adDistX + 10.6), 8.25, 0, Math.PI * 0.5],
    [adDistX + 10.6, 8.25, 0, -Math.PI * 0.5],
  ]) {
    const jGroup = new THREE.Group();
    jGroup.position.set(jx, jy, jz);
    jGroup.rotation.y = jYaw;
    const housing = new THREE.Mesh(new THREE.BoxGeometry(9.6, 3.65, 0.55), jumboFrameMat);
    jGroup.add(housing);
    const screen = new THREE.Mesh(new THREE.PlaneGeometry(9.1, 3.32), jumboScreenMat);
    screen.position.z = 0.3;
    jGroup.add(screen);
    stadium.add(jGroup);
  }

  // Two-Part Human Supporter Silhouettes: Muted Jacket Torsos + Natural Skin/Hair Heads (zero single-color Lego pegs!)
  const supTorsoGeo = new THREE.BoxGeometry(0.38, 0.42, 0.20);
  supTorsoGeo.translate(0, 0.18, 0);
  const supHeadGeo = new THREE.SphereGeometry(0.088, 6, 5);
  supHeadGeo.translate(0, 0.47, 0);

  const crowdMat = new THREE.MeshLambertMaterial({ color: "#ffffff" });
  const crowdHeadMat = new THREE.MeshLambertMaterial({ color: "#ffffff" });
  const crowdMesh = new THREE.InstancedMesh(supTorsoGeo, crowdMat, crowdPositions.length);
  const crowdHeadMesh = new THREE.InstancedMesh(supHeadGeo, crowdHeadMat, crowdPositions.length);
  const dummy = new THREE.Object3D();
  for (let i = 0; i < crowdPositions.length; i++) {
    const p = crowdPositions[i];
    dummy.position.set(p.x, p.y, p.z);
    dummy.rotation.set(0, p.rotY || 0, 0);
    dummy.updateMatrix();
    crowdMesh.setMatrixAt(i, dummy.matrix);
    crowdMesh.setColorAt(i, crowdColors[i]);
    crowdHeadMesh.setMatrixAt(i, dummy.matrix);
    crowdHeadMesh.setColorAt(i, crowdHeadColors[i]);
  }
  crowdMesh.instanceMatrix.needsUpdate = true;
  if (crowdMesh.instanceColor) crowdMesh.instanceColor.needsUpdate = true;
  crowdHeadMesh.instanceMatrix.needsUpdate = true;
  if (crowdHeadMesh.instanceColor) crowdHeadMesh.instanceColor.needsUpdate = true;
  stadium.add(crowdMesh);
  stadium.add(crowdHeadMesh);

  if (southCrowdPositions.length > 0) {
    const southCrowdMesh = new THREE.InstancedMesh(
      supTorsoGeo.clone(),
      new THREE.MeshLambertMaterial({ color: "#ffffff" }),
      southCrowdPositions.length
    );
    const southCrowdHeadMesh = new THREE.InstancedMesh(
      supHeadGeo.clone(),
      new THREE.MeshLambertMaterial({ color: "#ffffff" }),
      southCrowdPositions.length
    );
    for (let i = 0; i < southCrowdPositions.length; i++) {
      const p = southCrowdPositions[i];
      dummy.position.set(p.x, p.y, p.z);
      dummy.rotation.set(0, p.rotY || 0, 0);
      dummy.updateMatrix();
      southCrowdMesh.setMatrixAt(i, dummy.matrix);
      southCrowdMesh.setColorAt(i, southCrowdColors[i]);
      southCrowdHeadMesh.setMatrixAt(i, dummy.matrix);
      southCrowdHeadMesh.setColorAt(i, southCrowdHeadColors[i]);
    }
    southCrowdMesh.instanceMatrix.needsUpdate = true;
    if (southCrowdMesh.instanceColor) southCrowdMesh.instanceColor.needsUpdate = true;
    southCrowdHeadMesh.instanceMatrix.needsUpdate = true;
    if (southCrowdHeadMesh.instanceColor) southCrowdHeadMesh.instanceColor.needsUpdate = true;
    southStandUpper.add(southCrowdMesh);
    southStandUpper.add(southCrowdHeadMesh);
  }

  // Emissive Floodlight Towers (dynamically toggled between Night/Rain vs Sunny Day!)
  const pylonMat = new THREE.MeshStandardMaterial({ color: "#475569", metalness: 0.6, roughness: 0.4 });
  const bulbMat = new THREE.MeshStandardMaterial({
    color: "#fffbeb",
    emissive: "#fef3c7",
    emissiveIntensity: 2.6,
  });
  const floodlightHalos = [];
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
    floodlightHalos.push(halo);

    stadium.add(tower);
  }

  return {
    stadium,
    southStandUpper,
    kits,
    halfL,
    halfW,
    pitchLen,
    pitchWid,
    pitchMat,
    grassField,
    jumbotron,
    bulbMat,
    floodlightHalos,
    leftGoal,
    rightGoal,
    cornerFlags,
    crowdMesh,
    crowdHeadMesh,
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

export const WEATHER_PRESETS = ["night", "day", "rain"];
export const WEATHER_LABELS = {
  night: "FLOODLIT NIGHT",
  day: "SUNNY STADIUM DAY",
  rain: "RAINY STORM",
};

const PHASE_BANNER_TEXT = {
  0: "KICK OFF",
  1: "LIVE MATCH",
  2: "THROW-IN",
  3: "CORNER KICK",
  4: "GOAL KICK",
  5: "FREE KICK",
  6: "PENALTY KICK",
  10: "GOAL!",
  11: "HALF-TIME",
  12: "FULL-TIME",
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
    this.scene.fog = new THREE.FogExp2("#0c192c", 0.003);

    // Physically-Based Image-Based Lighting (subtle sky sheen without bleaching stadium shadows!)
    const pmrem = new THREE.PMREMGenerator(this.renderer);
    const roomEnv = new RoomEnvironment();
    this.envRT = pmrem.fromScene(roomEnv, 0.04);
    retainGpu(this.envRT.texture);
    this.scene.environment = this.envRT.texture;
    this.scene.environmentIntensity = 0.16;
    roomEnv.dispose();
    pmrem.dispose();

    this.camera = new THREE.PerspectiveCamera(28.5, 16 / 9, 0.4, 460);
    this.cameraMode = "tele";
    this.weatherMode = "night";
    this.viewMode = "attract";
    this.userZoom = 1.0; // Adjustable [0.72 .. 1.48] via Mouse Wheel, - / =, or HUD Zoom buttons

    this.camPos = new THREE.Vector3(0, 23.2, 31.2);
    this.camLook = new THREE.Vector3(0, 0.45, 0.85);
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
        intensity: 0.28,
        luminanceThreshold: 0.90,
        luminanceSmoothing: 0.2,
        mipmapBlur: true,
      });
      const vignette = new VignetteEffect({ darkness: 0.28, offset: 0.34 });
      const toneMap = new ToneMappingEffect({ mode: ToneMappingMode.ACES_FILMIC });
      composer.addPass(new EffectPass(this.camera, smaa, bloom, vignette, toneMap));
      this.composer = composer;
    } catch (_err) {
      this.composer = null;
      this.renderer.toneMapping = THREE.ACESFilmicToneMapping;
      this.renderer.toneMappingExposure = 1.04;
    }

    this.glowTex = createGlowSpriteTexture();
    this.maxAniso = this.renderer.capabilities.getMaxAnisotropy();

    this.skyMat = retainGpu(
      new THREE.MeshBasicMaterial({ map: SKY_TEXTURES.night, side: THREE.BackSide, fog: false })
    );
    this.skyDome = new THREE.Mesh(retainGpu(new THREE.SphereGeometry(280, 28, 18)), this.skyMat);
    this.scene.add(this.skyDome);

    this.hemiLight = new THREE.HemisphereLight(0xdbeafe, 0x123810, 0.40);
    this.scene.add(this.hemiLight);

    this.keyLightOffset = new THREE.Vector3(19, 28, 18);
    this.keyLight = new THREE.DirectionalLight(0xfffbeb, 3.05);
    this.keyLight.position.copy(this.keyLightOffset);
    this.keyLight.castShadow = true;
    this.keyLight.shadow.mapSize.set(2048, 2048);
    this.keyLight.shadow.camera.near = 4;
    this.keyLight.shadow.camera.far = 96;
    this.keyLight.shadow.camera.left = -26;
    this.keyLight.shadow.camera.right = 26;
    this.keyLight.shadow.camera.top = 24;
    this.keyLight.shadow.camera.bottom = -24;
    this.keyLight.shadow.bias = -0.00035;
    this.scene.add(this.keyLight);
    this.scene.add(this.keyLight.target);

    this.rimLight = new THREE.DirectionalLight(0xbae6fd, 0.32);
    this.rimLight.position.set(-28, 32, -32);
    this.scene.add(this.rimLight);

    this.frontFillLight = new THREE.DirectionalLight(0xffedd5, 0.24);
    this.frontFillLight.position.set(0, 14, 42);
    this.scene.add(this.frontFillLight);

    // FIFA Law 2 Regulation Size 5 Football: ~11.5cm radius (~23cm diameter), replacing oversized 0.22m radius!
    this.ballRadius = 0.115;
    this.ballMesh = new THREE.Mesh(
      retainGpu(new THREE.SphereGeometry(this.ballRadius, 28, 22)),
      retainGpu(
        new THREE.MeshStandardMaterial({
          map: createMatchBallTexture(),
          roughness: 0.26,
          metalness: 0.06,
        })
      )
    );
    this.ballMesh.castShadow = true;
    this.scene.add(this.ballMesh);

    this.ballShadowMat = retainGpu(
      new THREE.MeshBasicMaterial({
        color: "#020617",
        transparent: true,
        opacity: 0.48,
        depthWrite: false,
      })
    );
    this.ballShadow = new THREE.Mesh(
      retainGpu(new THREE.CircleGeometry(0.145, 18)),
      this.ballShadowMat
    );
    this.ballShadow.rotation.x = -Math.PI * 0.5;
    this.ballShadow.position.y = 0.016;
    this.scene.add(this.ballShadow);

    this.particles = new TurfParticleSystem(this.scene);
    this.rain = new RainSystem(this.scene);

    this.stadiumData = null;
    this.lastSnapshot = null;
    this.actors = [];
    this.refereeActor = null;
    this.refPos = new THREE.Vector2(-3.6, -5.2);
    this.refVel = new THREE.Vector2(0, 0);

    this.showcaseGroup = new THREE.Group();
    this.showcaseGroup.visible = false;
    this.scene.add(this.showcaseGroup);
    this.showcaseCaptains = null;

    this.netRipple = [0, 0];
    this.netImpact = [
      { strength: 0, z: 0, y: 1.2 },
      { strength: 0, z: 0, y: 1.2 },
    ];
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

  cycleWeatherPreset() {
    const idx = WEATHER_PRESETS.indexOf(this.weatherMode);
    return this.setWeatherPreset(WEATHER_PRESETS[(idx + 1) % WEATHER_PRESETS.length]);
  }

  setWeatherPreset(mode) {
    if (WEATHER_PRESETS.includes(mode)) {
      this.weatherMode = mode;
    }
    const m = this.weatherMode;
    this.skyMat.map = SKY_TEXTURES[m] || SKY_TEXTURES.night;
    this.skyMat.needsUpdate = true;

    if (m === "day") {
      this.scene.fog.color.set("#93c5fd");
      this.scene.fog.density = 0.0011;
      this.scene.environmentIntensity = 0.20;
      this.hemiLight.color.set("#e0f2fe");
      this.hemiLight.groundColor.set("#1c4416");
      this.hemiLight.intensity = 0.50;
      this.keyLight.color.set("#fffdf0");
      this.keyLight.intensity = 3.45;
      this.keyLightOffset.set(22, 29, 17);
      this.rimLight.color.set("#e0f2fe");
      this.rimLight.intensity = 0.26;
      this.frontFillLight.color.set("#fff7ed");
      this.frontFillLight.intensity = 0.22;
      this.rain.setEnabled(false);
      this.particles.setWetMode(false);
      this.ballMesh.material.roughness = 0.28;
    } else if (m === "rain") {
      this.scene.fog.color.set("#0b1626");
      this.scene.fog.density = 0.0040;
      this.scene.environmentIntensity = 0.18;
      this.hemiLight.color.set("#cbd5e1");
      this.hemiLight.groundColor.set("#0f2e14");
      this.hemiLight.intensity = 0.42;
      this.keyLight.color.set("#e0f2fe");
      this.keyLight.intensity = 2.95;
      this.keyLightOffset.set(17, 27, 19);
      this.rimLight.color.set("#93c5fd");
      this.rimLight.intensity = 0.46;
      this.frontFillLight.color.set("#dbeafe");
      this.frontFillLight.intensity = 0.28;
      this.rain.setEnabled(true);
      this.particles.setWetMode(true);
      this.ballMesh.material.roughness = 0.12;
    } else {
      this.scene.fog.color.set("#081220");
      this.scene.fog.density = 0.0024;
      this.scene.environmentIntensity = 0.16;
      this.hemiLight.color.set("#dbeafe");
      this.hemiLight.groundColor.set("#123810");
      this.hemiLight.intensity = 0.40;
      this.keyLight.color.set("#fffbeb");
      this.keyLight.intensity = 3.05;
      this.keyLightOffset.set(19, 28, 18);
      this.rimLight.color.set("#bae6fd");
      this.rimLight.intensity = 0.32;
      this.frontFillLight.color.set("#ffedd5");
      this.frontFillLight.intensity = 0.24;
      this.rain.setEnabled(false);
      this.particles.setWetMode(false);
      this.ballMesh.material.roughness = 0.24;
    }

    if (this.stadiumData) {
      const { pitchMat, grassField, bulbMat, floodlightHalos } = this.stadiumData;
      if (bulbMat) {
        bulbMat.emissiveIntensity = m === "day" ? 0.08 : m === "rain" ? 2.85 : 2.5;
      }
      if (floodlightHalos) {
        for (const h of floodlightHalos) {
          h.visible = m !== "day";
        }
      }
      if (pitchMat) {
        pitchMat.roughness = m === "rain" ? 0.38 : m === "day" ? 0.86 : 0.82;
        pitchMat.metalness = m === "rain" ? 0.08 : 0.01;
      }
      if (grassField && grassField.mesh && grassField.mesh.material) {
        grassField.mesh.material.roughness = m === "rain" ? 0.36 : 0.76;
      }
    }

    return this.weatherMode;
  }

  setViewMode(mode) {
    const prevMode = this.viewMode;
    this.viewMode = mode;
    const showMatchActors = mode !== "showcase";
    for (const a of this.actors) {
      a.root.visible = showMatchActors;
    }
    if (this.refereeActor) {
      this.refereeActor.root.visible = showMatchActors;
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
        this.camPos.set(0, (is11 ? 29.5 : 23.2) * z, (is11 ? 39.5 : 31.2) * z);
        this.camLook.set(0, 0.45, 0.85);
      }
    }
  }

  triggerGoalEffect(scoringSide, impactZ = 0, impactY = 1.25) {
    this.goalExcitement = 3.5;
    const clampedY = Math.max(0.35, Math.min(2.2, impactY));
    if (scoringSide === 0) {
      this.netRipple[1] = 1.0;
      this.netImpact[1] = { strength: 1.0, z: impactZ, y: clampedY };
    } else if (scoringSide === 1) {
      this.netRipple[0] = 1.0;
      this.netImpact[0] = { strength: 1.0, z: impactZ, y: clampedY };
    } else {
      this.netRipple[0] = 0.85;
      this.netRipple[1] = 0.85;
      this.netImpact[0] = { strength: 0.85, z: impactZ, y: clampedY };
      this.netImpact[1] = { strength: 0.85, z: impactZ, y: clampedY };
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
    this.lastSnapshot = snapshot;
    if (this.stadiumData) {
      this.scene.remove(this.stadiumData.stadium);
      disposeObjectTree(this.stadiumData.stadium);
    }
    for (const a of this.actors) {
      this.scene.remove(a.root);
      disposeObjectTree(a.root);
    }
    this.actors = [];
    if (this.refereeActor) {
      this.scene.remove(this.refereeActor.root);
      disposeObjectTree(this.refereeActor.root);
      this.refereeActor = null;
    }

    this.stadiumData = buildStadiumWorld(snapshot, this.maxAniso, this.glowTex);
    this.scene.add(this.stadiumData.stadium);
    this.smoothPlayFocus.set(0, 0, 0);

    const is11 = this.stadiumData.pitchLen > 80;
    const sx = is11 ? 30 : 25;
    const sz = is11 ? 26 : 22;
    this.keyLight.shadow.camera.left = -sx;
    this.keyLight.shadow.camera.right = sx;
    this.keyLight.shadow.camera.top = sz;
    this.keyLight.shadow.camera.bottom = -sz;
    this.keyLight.shadow.camera.updateProjectionMatrix();

    const pps = snapshot.format.players_per_side;
    const playerScale = pps <= 6 ? 1.0 : 0.97;
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

    // 3D On-Pitch Match Official (`IMG_0354.jpeg`): high-contrast yellow or electric-cyan referee kit
    const awayShirtLc = String(kits.awayOutfield.shirt || "").toLowerCase();
    const homeShirtLc = String(kits.homeOutfield.shirt || "").toLowerCase();
    const yellowClash = [awayShirtLc, homeShirtLc].some(
      (c) => c.includes("ea") || c.includes("fa") || c.includes("fd") || c.includes("eb") || c.includes("f5")
    );
    const refShirtHex = yellowClash ? "#06b6d4" : "#facc15";
    const refKit = {
      shirt: refShirtHex,
      sleeves: refShirtHex,
      trim: "#0f172a",
      shorts: "#0f172a",
      shortsStripe: refShirtHex,
      socks: "#0f172a",
      sockBand: refShirtHex,
      boots: "#0f172a",
      bootAccent: "#f8fafc",
      numberColor: "#0f172a",
      numberOutline: "#f8fafc",
      pattern: "solid",
    };
    this.refereeActor = createFootballerActor({
      kit: refKit,
      shirtNumber: 0,
      surname: "REFEREE",
      footballerId: "official-referee-1",
      clubShort: "REF",
      isGk: false,
      isReferee: true,
      side: -1,
      scale: playerScale * 0.99,
    });
    this.refereeActor.root.visible = this.viewMode !== "showcase";
    this.refPos.set(-3.6, -5.2);
    this.refVel.set(0, 0);
    this.scene.add(this.refereeActor.root);

    this.setWeatherPreset(this.weatherMode);
  }

  /**
   * Hot-swaps a single 3D player mesh during a live deterministic in-match substitution!
   */
  replacePlayerActor(side, idx, member, team) {
    if (!this.stadiumData || !member) return;
    const pps = this.lastSnapshot ? this.lastSnapshot.format.players_per_side : this.actors.length >> 1;
    const slot = side * pps + idx;
    const prev = this.actors[slot];
    if (!prev) return;

    const prevX = prev.root.position.x;
    const prevZ = prev.root.position.z;
    const prevYaw = prev.smoothYaw;

    this.scene.remove(prev.root);
    disposeObjectTree(prev.root);

    const kits = this.stadiumData.kits;
    const isGk = idx === 0 || member.role === "Goalkeeper";
    const kit = isGk
      ? side === 0
        ? kits.homeGk
        : kits.awayGk
      : side === 0
        ? kits.homeOutfield
        : kits.awayOutfield;
    const playerScale = pps <= 6 ? 1.0 : 0.97;

    const nextActor = createFootballerActor({
      kit,
      shirtNumber: member.shirt_number,
      surname: extractSurname(member.name),
      footballerId: member.footballer_id,
      clubShort: team ? team.short_name : "FC",
      isGk,
      side,
      scale: playerScale,
    });
    nextActor.root.position.set(prevX, 0, prevZ);
    nextActor.smoothYaw = prevYaw;
    nextActor.root.rotation.y = prevYaw;
    nextActor.root.visible = this.viewMode !== "showcase";
    this.scene.add(nextActor.root);
    this.actors[slot] = nextActor;
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

    const {
      halfL,
      halfW,
      pitchLen,
      southStandUpper,
      grassField,
      jumbotron,
      leftGoal,
      rightGoal,
      cornerFlags,
      crowdMesh,
      crowdHeadMesh,
      crowdPositions,
    } = this.stadiumData;

    const waveT = nowSec * 4.0;
    for (let i = 0; i < cornerFlags.length; i++) {
      cornerFlags[i].rotation.y = Math.sin(waveT + i * 1.4) * 0.28;
    }

    // --- Pre-Match Showcase View ---
    if (this.viewMode === "showcase") {
      if (southStandUpper) southStandUpper.visible = true;
      this.rain.step(dt, 0, 0);
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
    let phase = 1;
    let ownerSide = -1;
    let pps = 5;
    let gk0X = -halfL;
    let gk1X = halfL;

    if (currPacked) {
      const p0 = prevPacked && prevPacked.length === currPacked.length ? prevPacked : currPacked;
      const tInterp = Math.max(0, Math.min(1, alpha));

      const clockTicks = currPacked[1];
      phase = currPacked[3];
      const hScore = currPacked[4];
      const aScore = currPacked[5];
      pps = currPacked[6];
      const homeCtrl = currPacked[7];
      const awayCtrl = currPacked[8];

      if (jumbotron) {
        const totSec = Math.floor(clockTicks / 60);
        const mm = String(Math.floor(totSec / 60)).padStart(2, "0");
        const ss = String(totSec % 60).padStart(2, "0");
        jumbotron.update(hScore, aScore, `${mm}:${ss}`, PHASE_BANNER_TEXT[phase] || "LIVE");
      }

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

      ownerSide = currPacked[15];
      const ownerIdx = currPacked[16];

      if (pps > 0 && currPacked.length >= 18 + pps * 20) {
        gk0X = currPacked[18 + 4] / 1000;
        gk1X = currPacked[18 + pps * 10 + 4] / 1000;
      }

      if (this.prevBallZ > 0.28 && ballZ <= 0.06) {
        this.particles.spawnBurst(ballX, 0.04, ballY, ballVx * 0.15, ballVy * 0.15, 5);
      }
      this.prevBallZ = ballZ;

      // Detect physical ball-into-net contact to deform localized 3D goal mesh pocket!
      if (Math.abs(ballX) >= halfL - 0.08 && Math.abs(ballY) < leftGoal.goalHW && ballZ < leftGoal.goalH) {
        const gIdx = ballX > 0 ? 1 : 0;
        const hitPow = Math.min(1.15, 0.45 + Math.hypot(ballVx, ballVy) * 0.04);
        if (hitPow > this.netImpact[gIdx].strength * 0.75) {
          this.netImpact[gIdx] = {
            strength: hitPow,
            z: ballY,
            y: Math.max(0.32, Math.min(leftGoal.goalH - 0.2, ballZ + this.ballRadius)),
          };
          this.netRipple[gIdx] = Math.max(this.netRipple[gIdx], hitPow);
        }
      }

      const spinBase = 18 + pps * 20;
      const hasSpinWords = currPacked.length >= spinBase + 2;
      const spinSide = hasSpinWords ? currPacked[spinBase] / 1000 : 0;
      const spinTop = hasSpinWords ? currPacked[spinBase + 1] / 1000 : 0;
      const hasAimWords = currPacked.length >= spinBase + 4;
      const homeAimBam = hasAimWords ? currPacked[spinBase + 2] : -1;
      const awayAimBam = hasAimWords ? currPacked[spinBase + 3] : -1;

      this.ballMesh.position.set(ballX, this.ballRadius + Math.max(0, ballZ), ballY);
      const bSpeed = Math.hypot(ballVx, ballVy);
      if (ballZ <= 0.06) {
        if (bSpeed > 0.03) {
          const rollAxis = _vTargetOff.set(ballVy, 0, -ballVx).normalize();
          this.ballMesh.rotateOnWorldAxis(rollAxis, (bSpeed * dt) / this.ballRadius);
        }
      } else {
        if (bSpeed > 0.05) {
          const pitchAxis = _vTargetOff.set(ballVy, 0, -ballVx).normalize();
          const pitchOmega = spinTop * 8.5 + bSpeed * 1.4;
          this.ballMesh.rotateOnWorldAxis(pitchAxis, pitchOmega * dt);
        }
        if (Math.abs(spinSide) > 0.03) {
          const yawAxis = _vPole.set(0, 1, 0);
          this.ballMesh.rotateOnWorldAxis(yawAxis, spinSide * 4.2 * dt);
        }
      }

      this.ballShadow.position.set(ballX, 0.016, ballY);
      const shadowScale = Math.max(0.45, 1.0 + Math.min(2.2, Math.max(0, ballZ)) * 0.22);
      this.ballShadow.scale.set(shadowScale, shadowScale, 1);
      if (this.ballShadowMat) {
        this.ballShadowMat.opacity = Math.max(0.10, 0.46 - Math.max(0, ballZ) * 0.11);
      }

      const showHudIndicators = this.viewMode === "match" && !(replayOpts && replayOpts.active);
      const wallTargetDist = pitchLen > 80 ? 9.15 : 4.6;

      const actorCount = Math.min(pps * 2, this.actors.length);
      for (let n = 0; n < actorCount; n++) {
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

        const flags0 = p0[b + 9] >>> 0;
        const flags = currPacked[b + 9] >>> 0;
        const action = flags & 0xf;
        const charge = ((flags >>> 15) & 0x7ff) / 1000;
        const sentOff = ((flags >>> 26) & 1) === 1;
        const jz0 = ((flags0 >>> 28) & 0xf) * 0.06;
        const jz1 = ((flags >>> 28) & 0xf) * 0.06;
        const jumpZ = teleported ? jz1 : jz0 + (jz1 - jz0) * tInterp;

        const actor = this.actors[n];
        if (sentOff) {
          actor.root.visible = false;
          continue;
        }
        actor.root.visible = true;

        // Nearest-opponent vector for physical shoulder-to-shoulder jostling & shielding arm (`IMG_0363.jpeg`!)
        let oppDist = 99;
        let oppWorldDx = 0;
        let oppWorldDy = 0;
        for (let m = 0; m < actorCount; m++) {
          const mb = 18 + m * 10;
          if (currPacked[mb] === side || ((currPacked[mb + 9] >>> 26) & 1) === 1) continue;
          const ox = currPacked[mb + 4] / 1000;
          const oy = currPacked[mb + 5] / 1000;
          const d = Math.hypot(ox - wx, oy - wy);
          if (d < oppDist) {
            oppDist = d;
            oppWorldDx = ox - wx;
            oppWorldDy = oy - wy;
          }
        }

        const isControlled = showHudIndicators && (side === 0 ? idx === homeCtrl : idx === awayCtrl);
        const rawAimBam = side === 0 ? homeAimBam : awayAimBam;
        const aimRad = isControlled && rawAimBam >= 0 ? (rawAimBam * Math.PI * 2) / 65536 : null;
        const isBallCarrier = showHudIndicators && ownerSide === side && ownerIdx === idx;
        const celebrating = phase === 10 && ownerSide === side;
        const inWall =
          phase === 5 &&
          ownerSide >= 0 &&
          side !== ownerSide &&
          idx > 0 &&
          Math.abs(Math.hypot(wx - ballX, wy - ballY) - wallTargetDist) < 1.45;

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
            aimRad,
            action,
            charge,
            jumpZ,
            inWall,
            isControlled,
            isBallCarrier,
            ballX,
            ballY,
            ballZ,
            oppDist,
            oppWorldDx,
            oppWorldDy,
            celebrating,
          },
          dt,
          nowSec,
          replayOpts && replayOpts.active ? null : this.particles
        );
        if (phase === 3 || phase === 5 || phase === 6) {
          actor.nameplate.visible = false;
        }
      }

      // Animate 3D On-Pitch Referee (`IMG_0354.jpeg`): smooth diagonal officiating run + restart whistle signal
      if (this.refereeActor) {
        this.refereeActor.root.visible = true;
        let goalRefX = Math.max(-halfL + 3.0, Math.min(halfL - 3.0, ballX * 0.64 - 3.8));
        let goalRefY = Math.max(-halfW + 2.2, Math.min(halfW - 2.2, ballY * 0.48 - 5.4));
        const dBall = Math.hypot(goalRefX - ballX, goalRefY - ballY);
        if (dBall < 3.4) {
          goalRefY = ballY >= 0 ? Math.max(-halfW + 2.0, ballY - 4.8) : Math.min(halfW - 2.0, ballY + 4.8);
        }
        const prevRx = this.refPos.x;
        const prevRy = this.refPos.y;
        const stepBlend = 1.0 - Math.exp(-dt * 2.8);
        const nextRx = prevRx + (goalRefX - prevRx) * stepBlend;
        const nextRy = prevRy + (goalRefY - prevRy) * stepBlend;
        const rvx = (nextRx - prevRx) / Math.max(0.004, dt);
        const rvy = (nextRy - prevRy) / Math.max(0.004, dt);
        const rax = (rvx - this.refVel.x) / Math.max(0.01, dt);
        const ray = (rvy - this.refVel.y) / Math.max(0.01, dt);
        this.refPos.set(nextRx, nextRy);
        this.refVel.set(rvx, rvy);

        const refFacing = Math.atan2(-(ballY - nextRy), ballX - nextRx);
        const refSignal = phase === 2 || phase === 3 || phase === 4 || phase === 5 || phase === 6 ? 1 : 0;
        animateFootballerActor(
          this.refereeActor,
          {
            wx: nextRx,
            wy: nextRy,
            vx: rvx,
            vy: rvy,
            axTick: rax,
            ayTick: ray,
            facingRad: refFacing,
            action: 0,
            charge: 0,
            jumpZ: 0,
            inWall: false,
            refSignal,
            isControlled: false,
            isBallCarrier: false,
            ballX,
            ballY,
            ballZ,
            celebrating: false,
          },
          dt,
          nowSec,
          null
        );
      }
    }

    this.rain.step(dt, ballX, ballY);
    if (grassField) {
      grassField.updateFocus(ballX, ballY, nowSec);
    }

    // 3D Goal-Net Vertex Pocket Deformation + Stanchion Ripple
    for (let g = 0; g < 2; g++) {
      const goalObj = g === 0 ? leftGoal : rightGoal;
      const imp = this.netImpact[g];
      if (imp.strength > 0.004) {
        imp.strength *= Math.pow(0.05, dt);
        goalObj.deformNet(imp.strength, imp.z, imp.y, nowSec * 28.0);
      } else if (imp.strength > 0) {
        imp.strength = 0;
        goalObj.deformNet(0, 0, 1.2, 0);
      }
      if (this.netRipple[g] > 0.005) {
        this.netRipple[g] *= Math.pow(0.04, dt);
        const bulge = Math.sin(nowSec * 32) * this.netRipple[g] * 0.08;
        goalObj.netHolder.scale.set(1 + Math.abs(bulge), 1 + bulge * 0.25, 1 + bulge * 0.25);
      }
    }

    if (this.goalExcitement > 0) {
      this.goalExcitement = Math.max(0, this.goalExcitement - dt);
    }
    if (crowdMesh && (this.goalExcitement > 0.01 || Math.floor(now / 50) % 2 === 0)) {
      const dummy = new THREE.Object3D();
      const amp = this.goalExcitement > 0.01 ? 0.24 : 0.028;
      const spd = this.goalExcitement > 0.01 ? 14.0 : 3.2;
      for (let i = 0; i < crowdPositions.length; i += 2) {
        const p = crowdPositions[i];
        dummy.position.set(p.x, p.y + Math.abs(Math.sin(nowSec * spd + p.phase)) * amp, p.z);
        dummy.rotation.set(0, p.rotY || 0, 0);
        dummy.updateMatrix();
        crowdMesh.setMatrixAt(i, dummy.matrix);
        if (crowdHeadMesh) crowdHeadMesh.setMatrixAt(i, dummy.matrix);
      }
      crowdMesh.instanceMatrix.needsUpdate = true;
      if (crowdHeadMesh) crowdHeadMesh.instanceMatrix.needsUpdate = true;
    }

    // --- EA FIFA 10–12 Broadcast Gantry & Dynamic Set-Piece Camera Controller (`IMG_0350.webp`!) ---
    const focusGoalX = ballX + Math.max(-2.8, Math.min(2.8, ballVx * 0.16));
    const focusGoalZ = ballY + Math.max(-1.8, Math.min(1.8, ballVy * 0.12));
    damp3(this.smoothPlayFocus, [focusGoalX, 0, focusGoalZ], 0.22, dt);

    const fx = this.smoothPlayFocus.x;
    const fz = this.smoothPlayFocus.z;
    const zScale = this.userZoom;

    // Track `2048x2048` shadow frustum around play focus for razor-sharp diagonal stadium shadows (`IMG_0350.webp`)!
    this.keyLight.target.position.set(fx, 0, fz);
    this.keyLight.position.set(
      fx + this.keyLightOffset.x,
      this.keyLightOffset.y,
      fz + this.keyLightOffset.z
    );

    let targetPos;
    let targetLook;
    let desiredFov = 28.5;
    let camSmoothTime = 0.22;
    let setPieceCamKey = 0;

    const maxBowlX = halfL + 6.0;
    const maxBowlZNorth = halfW + 5.4;
    const maxBowlZSouth = halfW + 5.6;

    if (replayOpts && replayOpts.active) {
      setPieceCamKey =
        replayOpts.camMode === "closeup" ? 101 : replayOpts.camMode === "orbit" ? 102 : 100;
      camSmoothTime = 0.12;
      desiredFov = 28.5;
      if (replayOpts.camMode === "broadcast") {
        targetPos = [ballX * 0.55, 17.5 * zScale, (halfW + 14.5) * zScale];
        targetLook = [ballX * 0.75, 0.55, ballY * 0.35 + 0.6];
      } else if (replayOpts.camMode === "closeup") {
        const ang = replayOpts.orbitAngle;
        targetPos = [ballX + Math.sin(ang) * 5.6, 2.05, ballY + Math.cos(ang) * 5.6];
        targetLook = [ballX, 0.86, ballY];
      } else {
        const r = (pitchLen > 80 ? 19.5 : 13.5) * zScale;
        const ang = replayOpts.orbitAngle;
        targetPos = [
          Math.max(-maxBowlX, Math.min(maxBowlX, ballX + Math.sin(ang) * r)),
          Math.min(9.6, 5.6 * zScale),
          Math.max(-maxBowlZNorth, Math.min(maxBowlZSouth, ballY + Math.cos(ang) * r)),
        ];
        targetLook = [ballX, 0.85, ballY];
      }
    } else if (this.viewMode === "attract" || this.viewMode === "squad") {
      this.attractAngle += dt * 0.13;
      desiredFov = 29.5;
      const radX = halfL * 0.52;
      const eyeX = Math.sin(this.attractAngle) * radX;
      const eyeZ = halfW + 14.2 + Math.cos(this.attractAngle * 0.7) * 3.5;
      targetPos = [eyeX, 15.2, eyeZ];
      targetLook = [fx * 0.4, 0.55, fz * 0.2 + 0.4];
    } else {
      const is11 = pitchLen > 80;
      const targetGoalSign =
        ownerSide === 0 ? Math.sign(gk1X || 1) : ownerSide === 1 ? Math.sign(gk0X || -1) : Math.sign(ballX || 1);
      const targetGoalX = targetGoalSign * halfL;
      const distToAttGoal = Math.hypot(targetGoalX - ballX, ballY);

      if (phase === 3 && Math.abs(ballX) > halfL * 0.72 && Math.abs(ballY) > halfW * 0.65) {
        // Authentic Behind-the-Corner-Taker Set-Piece Camera (`IMG_0351.jpeg`!)
        setPieceCamKey = 3;
        const penSpotX = Math.sign(ballX) * (halfL - (is11 ? 11.0 : 6.2));
        const dx = penSpotX - ballX;
        const dz = -ballY;
        const inv = 1.0 / Math.max(0.1, Math.hypot(dx, dz));
        const ux = dx * inv;
        const uz = dz * inv;
        desiredFov = 34;
        camSmoothTime = 0.14;
        targetPos = [
          Math.max(-maxBowlX, Math.min(maxBowlX, ballX - ux * 6.2 * zScale)),
          3.35 * zScale,
          Math.max(-maxBowlZNorth, Math.min(maxBowlZSouth, ballY - uz * 6.2 * zScale)),
        ];
        targetLook = [ballX + ux * 11.5, 1.05, ballY + uz * 11.5];
      } else if (phase === 6 || (phase === 5 && distToAttGoal < halfL * 0.92)) {
        // Authentic Behind-the-Free-Kick / Penalty-Taker Camera (`IMG_0352.jpeg`, `IMG_0359.jpeg`!)
        setPieceCamKey = phase;
        const dx = targetGoalX - ballX;
        const dz = -ballY;
        const inv = 1.0 / Math.max(0.1, Math.hypot(dx, dz));
        const ux = dx * inv;
        const uz = dz * inv;
        const backDist = (phase === 6 ? 5.8 : 6.8) * zScale;
        const camH = (phase === 6 ? 2.25 : 2.55) * zScale;
        desiredFov = 33.5;
        camSmoothTime = 0.14;
        targetPos = [
          Math.max(-maxBowlX, Math.min(maxBowlX, ballX - ux * backDist)),
          camH,
          Math.max(-maxBowlZNorth, Math.min(maxBowlZSouth, ballY - uz * backDist)),
        ];
        targetLook = [targetGoalX, 1.38, 0];
      } else {
        const clampX = halfL * (is11 ? 0.72 : 0.66);
        const leadX = Math.max(-clampX, Math.min(clampX, fx * (is11 ? 0.80 : 0.78)));

        if (this.cameraMode === "dynamic") {
          desiredFov = 29;
          const heightY = (is11 ? 15.8 : 13.4) * zScale;
          const distZ = (is11 ? 25.5 : 20.5) * zScale;
          targetPos = [leadX, heightY, fz * 0.35 + distZ];
          targetLook = [fx * 0.76, 0.55, fz * 0.35 + 0.45];
        } else if (this.cameraMode === "pro") {
          // Authentic EA FIFA 10–12 "Be A Pro" Behind-the-Runner Horizon Camera (`IMG_0345.jpeg`!),
          // clamped pitch-side clear of the West Stand wall (`-(halfL + 7.15)`) with smooth end-line elevation!
          desiredFov = 36;
          const rawProX = fx - (is11 ? 15.2 : 11.8) * zScale;
          const minProX = -maxBowlX;
          const proX = Math.max(minProX, Math.min(maxBowlX, rawProX));
          const westWallBlend = Math.min(1, Math.max(0, (minProX - rawProX) / 10.0));
          const proY = Math.min(9.6, (is11 ? 4.85 : 3.95) * zScale + westWallBlend * 2.15);
          const proZ = Math.max(-(halfW + 4.8), Math.min(halfW + 4.8, fz * 0.68));
          targetPos = [proX, proY, proZ];
          targetLook = [
            Math.max(proX + 5.5, fx + (is11 ? 16.0 : 12.0)),
            1.32 - westWallBlend * 0.28,
            fz * 0.42,
          ];
        } else if (this.cameraMode === "wide") {
          // Classic FIFA Co-Op Tactical (full goal-to-goal tactical overview!)
          desiredFov = 30;
          const heightY = (is11 ? 35.5 : 28.2) * zScale;
          const distZ = (is11 ? 47.5 : 37.8) * zScale;
          targetPos = [leadX * 0.40, heightY, distZ];
          targetLook = [leadX * 0.44, 0.40, 0.55];
        } else {
          // Authentic EA FIFA 10–12 Tele Broadcast Gantry (`IMG_0350.webp` — pitch fills 92% of screen, crisp diagonal shadows, full box + 3D net in frame!)
          desiredFov = 28.5;
          const heightY = (is11 ? 29.5 : 23.2) * zScale;
          const distZ = (is11 ? 39.5 : 31.2) * zScale;
          targetPos = [leadX * 0.90, heightY, fz * 0.24 + distZ];
          targetLook = [leadX * 0.95, 0.42, fz * 0.26 + 0.85];
        }
      }
    }

    if (setPieceCamKey !== (this.prevSetPieceCamKey || 0)) {
      this.prevSetPieceCamKey = setPieceCamKey;
      this.camPos.set(targetPos[0], targetPos[1], targetPos[2]);
      this.camLook.set(targetLook[0], targetLook[1], targetLook[2]);
      this.camera.fov = desiredFov;
      this.camera.updateProjectionMatrix();
    } else {
      if (Math.abs(this.camera.fov - desiredFov) > 0.02) {
        damp(this.camera, "fov", desiredFov, 0.16, dt);
        this.camera.updateProjectionMatrix();
      }
      damp3(this.camPos, targetPos, camSmoothTime, dt);
      damp3(this.camLook, targetLook, camSmoothTime, dt);
    }

    this.camera.position.copy(this.camPos);
    this.camera.lookAt(this.camLook);

    if (southStandUpper) {
      const isSouthGantryCam =
        this.viewMode === "attract" ||
        this.viewMode === "squad" ||
        (replayOpts && replayOpts.active && replayOpts.camMode === "broadcast") ||
        (this.viewMode === "match" &&
          !(replayOpts && replayOpts.active) &&
          setPieceCamKey === 0 &&
          this.cameraMode !== "pro") ||
        this.camPos.z > halfW + 4.2;
      southStandUpper.visible = !isSouthGantryCam;
    }

    this._presentScene();
  }
}
