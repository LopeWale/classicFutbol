# Parallel Season FC — Controller Compatibility & Optional DIY Hardware BOM

> **Plug-and-Play Off-the-Shelf Controller Support (No Custom Hardware Required):**
> Any standard USB or Bluetooth gamepad from Amazon works out of the box in **both** the Browser/WASM client and the Godot 4 client, with automatic profile detection, Left-Stick + D-Pad movement, analog triggers, and Right-Stick skill-move flicks:
> - **Xbox Wireless / Wired Controllers** (Xbox Series X|S, Xbox One, GameSir G7 SE, PowerA) — native XInput
> - **8BitDo Ultimate 2C / Ultimate Bluetooth / Pro 2** — XInput (`X` mode) & Switch/DirectInput modes supported
> - **Sony PlayStation DualSense (PS5) & DualShock 4 (PS4)** — USB-C or Bluetooth (`Cross` = Pass, `Circle` = Shoot, `Square` = Lob/Slide, `Triangle` = Through)
> - **Logitech F310 / F710 & Generic USB DirectInput Pads** — both `X` (XInput) and `D` (DirectInput) switch modes auto-mapped
> - **Nintendo Switch Pro Controller** — positional south/east/west/north mapping + optional AB/XY layout selector in Browser UI
>
> The custom RP2040 Hall-effect controller + removable `PSFC-CART-V1` cartridge below is an **optional Phase 3 physical hardware build** for couch tournaments.

Each optional DIY player unit consists of a **Custom Hall-Effect Football Controller** with an integrated top-slot **USB-C Cartridge Dock** and a pocketable **Removable Profile Cartridge (`PSFC-CART-V1`)**.

## 1. Custom Controller + Integrated Cartridge Dock (Optional DIY Per Pad)

| Qty | Component | Part / Spec | Approx. Unit Cost (USD) | Role |
| ---: | --- | --- | ---: | --- |
| 1 | MCU Module | Raspberry Pi Pico / RP2040 Stamp (USB-C) | \$4.00 | 1000 Hz TinyUSB HID gamepad MCU |
| 1 | USB 2.0 Hub IC | `SL2.1A` 4-port HS USB Hub breakout | \$2.20 | Multiplexes RP2040 HID + top Cartridge slot onto 1 host cable |
| 2 | Thumbsticks | K-Silver `JH16` Hall-Effect Dual-Axis Modules + `R3` click | \$7.50 | Zero-drift 360° movement (`LX/LY`) + skill-flick/aim (`RX/RY`) |
| 1 | Analog Mux | `74LVC1G3157` SPDT Analog Switch | \$0.35 | Multiplexes 4th Hall axis onto RP2040 `ADC2` (`GPIO28`) |
| 8 | Face & Shoulder Switches | Kailh Choc V2 Low-Profile Tactile + 6mm硅胶 caps | \$4.80 | Crisp low-travel Pass, Through, Lob, Shoot, LB/RB, Start/Select |
| 2 | Trigger Switches | Omron `D2LS-21` SMD Microswitches + torsion springs | \$1.60 | Instant Sprint (`RT`) & Jockey (`LT`) actuation |
| 1 | Cartridge Slot Receptacle | Vertical 16-pin USB-C Female Receptacle PCB + guide rails | \$1.40 | Top-mounted cartridge bay with 5,000+ insertion cycle rating |
| 1 | Enclosure | 3D-printed PETG/ASA (`controller_shell.scad`) + 4x M3x12 heat-set inserts | \$3.20 | Ergonomic two-piece shell with recessed cartridge guide |
| **Total** | | | **\$25.05** | |

---

## 2. Removable Profile Cartridge (`PSFC-CART-V1`, Per Cartridge)

| Qty | Component | Part / Spec | Approx. Unit Cost (USD) | Role |
| ---: | --- | --- | ---: | --- |
| 1 | Flash Carrier | Industrial SLC/pSLC USB-C 2.0 Flash Module (128 MB – 4 GB) | \$3.40 | Hosts FAT32 volume with `slot_a.psfc` & `slot_b.psfc` |
| 2 | Retention Detents | 3mm x 1.5mm N52 Neodymium Magnets | \$0.30 | Tactile snap-in feel inside the controller's top cartridge bay |
| 1 | Cartridge Shell | Two-tone 3D-printed shell (`cartridge_shell.scad`) + club crest label recess | \$0.80 | Classic tactile cartridge silhouette (`44 mm × 32 mm × 8.4 mm`) |
| **Total** | | | **\$4.50** | |
