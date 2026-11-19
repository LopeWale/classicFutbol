# Parallel Season FC — Custom RP2040 Controller Firmware

Open-hardware firmware for the **Parallel Season FC** custom gamepad (`hardware/BOM.md`), engineered around 5v5 / 11v11 precision football input:

- **1000 Hz USB Full-Speed HID Polling (`bInterval = 1`)**: Sub-millisecond input latency into Godot (`ControllerInterface`) and Browser WebHID/Gamepad APIs.
- **Zero-Drift Hall-Effect Thumbsticks**: 12-bit oversampled ADC reads with startup neutral calibration and on-pad `Select + Start` re-zeroing.
- **Integrated USB-C Cartridge Bay**: Built-in 2-port USB 2.0 hub (`SL2.1A`) inside the controller body exposes both the RP2040 HID endpoint and the top-loading `PSFC-CART-V1` profile cartridge over a single braided USB-C cable.
- **Direct Wire Alignment with `psfc-sim`**: Emits an 8-byte packed report (`move_x`, `move_y`, `aim_x`, `aim_y`, `buttons`, `sequence`) matching [`InputFrame`](../sim/psfc-sim/src/input.rs) bit-for-bit, including `BTN_SKILL` (`1 << 10`) on `R3`.

## Building `.uf2` Firmware

```bash
export PICO_SDK_PATH=/path/to/pico-sdk
cmake -B build -S .
cmake --build build -j
# Hold BOOTSEL while plugging in the RP2040 board, then copy build/psfc_controller.uf2 to RPI-RP2
```
