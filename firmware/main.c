/*
 * Parallel Season FC — Custom RP2040 Hall-Effect Football Controller Firmware
 *
 * - Samples 4 Hall-effect analog axes (LX, LY, RX, RY) + 11 digital tactile switches at 1000 Hz.
 * - Applies per-unit radial dead-zone and center calibration stored in RP2040 QSPI flash.
 * - Reports a standard HID gamepad: 4 x int8 axes + 16 PHYSICAL button bits + uint16 sequence
 *   (8 bytes). The pad NEVER decides gameplay: the client's ControlProfile (Layer 2,
 *   host/psfc_host/web/static/control_profile.js / client/scripts/input/controller_interface.gd)
 *   maps physical buttons to Pass/Shoot/Contain/... so presets and remapping work on this pad
 *   exactly as on any Xbox / PlayStation pad.
 *   Button bit order = W3C "standard" Gamepad order, so browsers/SDL need no special mapping:
 *     0 South  1 East  2 West  3 North  4 L1  5 R1  6 L2  7 R2
 *     8 Select 9 Start 10 L3  11 R3    12 Up 13 Down 14 Left 15 Right
 */

#include <stdint.h>
#include <stdbool.h>
#include "pico/stdlib.h"
#include "hardware/adc.h"
#include "hardware/gpio.h"
#include "tusb.h"

/* Physical button bits (W3C standard gamepad order). NOT gameplay bits. */
#define PHYS_SOUTH   (1u << 0)
#define PHYS_EAST    (1u << 1)
#define PHYS_WEST    (1u << 2)
#define PHYS_NORTH   (1u << 3)
#define PHYS_L1      (1u << 4)
#define PHYS_R1      (1u << 5)
#define PHYS_L2      (1u << 6)
#define PHYS_R2      (1u << 7)
#define PHYS_SELECT  (1u << 8)
#define PHYS_START   (1u << 9)
#define PHYS_L3      (1u << 10)
#define PHYS_R3      (1u << 11)
#define PHYS_DPAD_UP    (1u << 12)
#define PHYS_DPAD_DOWN  (1u << 13)
#define PHYS_DPAD_LEFT  (1u << 14)
#define PHYS_DPAD_RIGHT (1u << 15)

typedef struct __attribute__((packed)) {
    int8_t   lx;
    int8_t   ly;
    int8_t   rx;
    int8_t   ry;
    uint16_t buttons;   /* PHYS_* bits */
    uint16_t sequence;
} psfc_hid_report_t;

_Static_assert(sizeof(psfc_hid_report_t) == 8, "HID report must stay 8 bytes (see usb_descriptors.c)");

typedef struct {
    uint16_t center_adc[4];
    uint16_t deadzone_adc;
} stick_calibration_t;

static stick_calibration_t g_cal = {
    .center_adc = {2048, 2048, 2048, 2048},
    .deadzone_adc = 140 /* ~6.8% Hall-effect center dead-zone */
};

/* GPIO pin -> physical button bit (hardware wiring unchanged from v1). */
static const uint8_t BUTTON_PINS[11] = {
    2,  /* South  (A / Cross)    */
    3,  /* North  (Y / Triangle) */
    4,  /* West   (X / Square)   */
    5,  /* East   (B / Circle)   */
    6,  /* R2 / RT (microswitch) */
    7,  /* L1 / LB               */
    8,  /* L2 / LT (microswitch) */
    9,  /* R1 / RB               */
    10, /* R3 (right stick click) */
    11, /* Select (+ Start = re-zero chord) */
    12  /* Start  */
};

static const uint16_t BUTTON_BITS[11] = {
    PHYS_SOUTH, PHYS_NORTH, PHYS_WEST, PHYS_EAST,
    PHYS_R2, PHYS_L1, PHYS_L2, PHYS_R1,
    PHYS_R3, PHYS_SELECT, PHYS_START
};

static int8_t normalize_hall_axis(uint16_t raw_12bit, uint16_t center, uint16_t dz, bool invert) {
    int32_t delta = (int32_t)raw_12bit - (int32_t)center;
    if (invert) {
        delta = -delta;
    }
    int32_t abs_d = delta < 0 ? -delta : delta;
    if (abs_d <= (int32_t)dz) {
        return 0;
    }
    int32_t span = 2047 - (int32_t)dz;
    int32_t scaled = ((abs_d - (int32_t)dz) * 127) / (span > 1 ? span : 1);
    if (scaled > 127) {
        scaled = 127;
    }
    return (int8_t)(delta < 0 ? -scaled : scaled);
}

static uint16_t read_muxed_hall_adc(uint8_t axis_index) {
    /* Axes 0..2 use RP2040 ADC0..ADC2 (GPIO26..28); Axis 3 uses external 2:1 mux on GPIO15 + ADC2 */
    if (axis_index < 3) {
        gpio_put(15, 0);
        adc_select_input(axis_index);
    } else {
        gpio_put(15, 1);
        sleep_us(4);
        adc_select_input(2);
    }
    return adc_read();
}

static void capture_startup_centers_if_neutral(void) {
    for (uint8_t i = 0; i < 4; ++i) {
        uint32_t acc = 0;
        for (int s = 0; s < 16; ++s) {
            acc += read_muxed_hall_adc(i);
        }
        uint16_t avg = (uint16_t)(acc / 16u);
        if (avg > 1700 && avg < 2400) {
            g_cal.center_adc[i] = avg;
        }
    }
}

int main(void) {
    stdio_init_all();
    tusb_init();

    adc_init();
    adc_gpio_init(26);
    adc_gpio_init(27);
    adc_gpio_init(28);

    gpio_init(15);
    gpio_set_dir(15, GPIO_OUT);
    gpio_put(15, 0);

    for (unsigned i = 0; i < 11; ++i) {
        gpio_init(BUTTON_PINS[i]);
        gpio_set_dir(BUTTON_PINS[i], GPIO_IN);
        gpio_pull_up(BUTTON_PINS[i]); /* Active-low mechanical switches */
    }

    capture_startup_centers_if_neutral();

    uint16_t report_seq = 0;
    uint64_t next_sample_us = time_us_64();

    while (true) {
        tud_task();

        uint64_t now = time_us_64();
        if (now < next_sample_us) {
            continue;
        }
        next_sample_us = now + 1000u; /* 1000 Hz deterministic sampling */

        uint16_t lx = read_muxed_hall_adc(0);
        uint16_t ly = read_muxed_hall_adc(1);
        uint16_t rx = read_muxed_hall_adc(2);
        uint16_t ry = read_muxed_hall_adc(3);

        /* Physical state only — gameplay meaning is assigned by the client's ControlProfile. */
        uint16_t btns = 0;
        for (unsigned i = 0; i < 11; ++i) {
            if (!gpio_get(BUTTON_PINS[i])) { /* active-low */
                btns |= BUTTON_BITS[i];
            }
        }

        /* Re-zero Hall centers on Select + Start chord */
        if (!gpio_get(BUTTON_PINS[9]) && !gpio_get(BUTTON_PINS[10])) {
            capture_startup_centers_if_neutral();
        }

        psfc_hid_report_t report = {
            .lx       = normalize_hall_axis(lx, g_cal.center_adc[0], g_cal.deadzone_adc, false),
            .ly       = normalize_hall_axis(ly, g_cal.center_adc[1], g_cal.deadzone_adc, true),
            .rx       = normalize_hall_axis(rx, g_cal.center_adc[2], g_cal.deadzone_adc, false),
            .ry       = normalize_hall_axis(ry, g_cal.center_adc[3], g_cal.deadzone_adc, true),
            .buttons  = btns,
            .sequence = ++report_seq,
        };

        if (tud_hid_ready()) {
            tud_hid_report(0, &report, sizeof(report));
        }
    }
    return 0;
}

uint16_t tud_hid_get_report_cb(uint8_t instance, uint8_t report_id, hid_report_type_t report_type, uint8_t *buffer, uint16_t reqlen) {
    (void)instance; (void)report_id; (void)report_type; (void)buffer; (void)reqlen;
    return 0;
}

void tud_hid_set_report_cb(uint8_t instance, uint8_t report_id, hid_report_type_t report_type, uint8_t const *buffer, uint16_t bufsize) {
    (void)instance; (void)report_id; (void)report_type; (void)buffer; (void)bufsize;
}
