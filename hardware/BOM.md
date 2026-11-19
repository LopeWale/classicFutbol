# Parallel Season FC — Hardware Bill of Materials (Phase 3 Prototype)

Each player unit consists of a **Custom Hall-Effect Football Controller** with an integrated top-slot **USB-C Cartridge Dock** and a pocketable **Removable Profile Cartridge (`PSFC-CART-V1`)**.

## 1. Custom Controller + Integrated Cartridge Dock (Per Pad)

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
