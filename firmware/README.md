# Parallel Season FC — Custom RP2040 Controller Firmware

Open-hardware firmware for the **Parallel Season FC** custom gamepad (`hardware/BOM.md`), engineered around 5v5 / 11v11 precision football input:

- **1000 Hz USB Full-Speed HID Polling (`bInterval = 1`)**: Sub-millisecond input latency into Godot (`ControllerInterface`) and Browser WebHID/Gamepad APIs.
- **Zero-Drift Hall-Effect Thumbsticks**: 12-bit oversampled ADC reads with startup neutral calibration and on-pad `Select + Start` re-zeroing.
- **Integrated USB-C Cartridge Bay**: Built-in 2-port USB 2.0 hub (`SL2.1A`) inside the controller body exposes both the RP2040 HID endpoint and the top-loading `PSFC-CART-V1` profile cartridge over a single braided USB-C cable.
- **Physical buttons only (firmware v2)**: the pad reports *which buttons are pressed*, never what they do. Pass / Shoot / Contain / Switch… are assigned by the client's ControlProfile (Classic / Alternate / Two-Button / Custom, see [`control_profile.schema.json`](../shared/schemas/control_profile.schema.json)), so remapping, presets and on-screen button prompts work on this pad exactly like on an Xbox or PlayStation pad.

## HID report (8 bytes, no report ID) — v2

| Byte | Type | Field |
| --- | --- | --- |
| 0 | `int8` | Left stick X (HID usage X) |
| 1 | `int8` | Left stick Y (usage Y, up = negative) |
| 2 | `int8` | Right stick X (usage Z) |
| 3 | `int8` | Right stick Y (usage Rz) |
| 4..5 | `uint16` LE | 16 physical buttons (HID Button 1..16) |
| 6..7 | `uint16` LE | Sequence counter (constant input) |

Button bits follow the W3C "standard" gamepad order, so `navigator.getGamepads()` button *i* is bit *i*:

| Bit | Button | Bit | Button |
| --- | --- | --- | --- |
| 0 | South (A / ✕) | 8 | Select |
| 1 | East (B / ○) | 9 | Start |
| 2 | West (X / □) | 10 | L3 (not fitted) |
| 3 | North (Y / △) | 11 | R3 |
| 4 | L1 / LB | 12 | D-Pad Up (not fitted) |
| 5 | R1 / RB | 13 | D-Pad Down (not fitted) |
| 6 | L2 / LT (digital microswitch) | 14 | D-Pad Left (not fitted) |
| 7 | R2 / RT (digital microswitch) | 15 | D-Pad Right (not fitted) |

USB IDs are unchanged: VID `0x1209` (pid.codes) / PID `0xF5FC`. `bcdDevice` is `0x0200` for the v2 report.

### Host-side recognition

- **Browser (Gamepad API)**: `samplePhysicalGamepad()` in `host/psfc_host/web/static/app.js` detects `1209`/`f5fc` in the pad id (`isPsfcPadId`) and reads it with the standard layout even though the browser reports `mapping === ""` (it also skips the SNES / DirectInput / hat-switch heuristics for this pad, and falls back to `axes[5]` for Rz if the browser places it there).
- **Browser (WebHID)**: `decodePsfcHidReport()` decodes the 8-byte report directly when the paired device is `1209:f5fc`.
- **Godot**: `ControllerInterface._register_psfc_mapping()` registers an SDL mapping (`a:b0,b:b1,x:b2,y:b3,leftshoulder:b4,rightshoulder:b5,lefttrigger:b6,righttrigger:b7,back:b8,start:b9,leftstick:b10,rightstick:b11,dpup:b12,…,leftx:a0,lefty:a1,rightx:a2,righty:a3`) using the device's real GUID when the pad is not already known.

## Migration from v1

v1 firmware wrote `psfc_sim::InputFrame` gameplay bits straight into the report (e.g. South = `BTN_PASS | BTN_TACKLE`), bypassing the client's remapping layer. Pads still running v1 will be mis-read by the v2-aware client (their bits are interpreted as physical buttons); reflash them with this firmware.

## Verify on hardware (not verifiable in CI — no pico-sdk here)

The v2 change only touches button packing and comments; the HID descriptor bytes, ADC and timing code are unchanged. On first flash, check:

1. `chrome://gamepad` (or https://hardwaretester.com/gamepad): each face button lights the expected index 0–3, LB/RB = 4/5, LT/RT = 6/7, Select/Start = 8/9, R3 = 11.
2. Left stick up gives a negative `axes[1]`; right stick shows on `axes[2]`/`axes[3]` (some OSes put Rz on `axes[5]`; the web client handles both).
3. The Controllers screen visualizer lights the matching buttons and the Classic preset passes with South.

## Known build gaps (pre-existing, unchanged)

- There is no `tusb_config.h` in this directory and no `tud_descriptor_string_cb()` although the device descriptor references string indices 1–3; both are needed for a real TinyUSB build.
- `stdio_init_all()` together with `tinyusb_device` may conflict with `pico_stdio_usb`; disable USB stdio (`pico_enable_stdio_usb(psfc_controller 0)`) if the build complains.

## Building `.uf2` Firmware

```bash
export PICO_SDK_PATH=/path/to/pico-sdk
cmake -B build -S .
cmake --build build -j
# Hold BOOTSEL while plugging in the RP2040 board, then copy build/psfc_controller.uf2 to RPI-RP2
```

A host-side syntax check (no SDK) is possible with stub headers for `pico/stdlib.h`, `hardware/{adc,gpio}.h` and `tusb.h`: `gcc -std=c11 -Wall -Wextra -fsyntax-only -I<stubs> main.c usb_descriptors.c`.
