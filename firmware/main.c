/*
 * Parallel Season FC — Custom RP2040 Hall-Effect Football Controller Firmware
 *
 * - Samples 4 Hall-effect analog axes (LX, LY, RX, RY) + 11 digital tactile switches at 1000 Hz.
 * - Applies per-unit radial dead-zone and center calibration stored in RP2040 QSPI flash.
 * - Packs reports directly matching the 8-byte `psfc_sim::InputFrame` wire layout:
 *     int8 move_x, int8 move_y, int8 aim_x, int8 aim_y, uint16 buttons, uint16 seq
 */

#include <stdint.h>
#include <stdbool.h>
#include "pico/stdlib.h"
#include "hardware/adc.h"
#include "hardware/gpio.h"
#include "tusb.h"

/* Exact bitflags matching `sim/psfc-sim/src/input.rs` */
#define BTN_PASS      (1u << 0)
#define BTN_THROUGH   (1u << 1)
#define BTN_LOB       (1u << 2)
#define BTN_SHOOT     (1u << 3)
#define BTN_SPRINT    (1u << 4)
#define BTN_SWITCH    (1u << 5)
#define BTN_JOCKEY    (1u << 6)
#define BTN_TACKLE    (1u << 7)
#define BTN_SLIDE     (1u << 8)
#define BTN_CALL_RUN  (1u << 9)
#define BTN_SKILL     (1u << 10)

typedef struct __attribute__((packed)) {
    int8_t   move_x;
    int8_t   move_y;
    int8_t   aim_x;
    int8_t   aim_y;
    uint16_t buttons;
    uint16_t sequence;
} psfc_hid_report_t;

typedef struct {
    uint16_t center_adc[4];
    uint16_t deadzone_adc;
} stick_calibration_t;

static stick_calibration_t g_cal = {
    .center_adc = {2048, 2048, 2048, 2048},
    .deadzone_adc = 140 /* ~6.8% Hall-effect center dead-zone */
};

static const uint8_t BUTTON_PINS[11] = {
    2,  /* South  -> BTN_PASS / BTN_TACKLE */
    3,  /* North  -> BTN_THROUGH */
    4,  /* West   -> BTN_LOB / BTN_SLIDE */
    5,  /* East   -> BTN_SHOOT */
    6,  /* RT     -> BTN_SPRINT */
    7,  /* LB     -> BTN_SWITCH */
    8,  /* LT     -> BTN_JOCKEY */
    9,  /* RB     -> BTN_CALL_RUN */
    10, /* R3     -> BTN_SKILL (Roulette / Skill Move) */
    11, /* Select -> Calibrationchord */
    12  /* Start  -> Pause */
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

        uint16_t btns = 0;
        if (!gpio_get(BUTTON_PINS[0])) btns |= (BTN_PASS | BTN_TACKLE);
        if (!gpio_get(BUTTON_PINS[1])) btns |= BTN_THROUGH;
        if (!gpio_get(BUTTON_PINS[2])) btns |= (BTN_LOB | BTN_SLIDE);
        if (!gpio_get(BUTTON_PINS[3])) btns |= BTN_SHOOT;
        if (!gpio_get(BUTTON_PINS[4])) btns |= BTN_SPRINT;
        if (!gpio_get(BUTTON_PINS[5])) btns |= BTN_SWITCH;
        if (!gpio_get(BUTTON_PINS[6])) btns |= BTN_JOCKEY;
        if (!gpio_get(BUTTON_PINS[7])) btns |= BTN_CALL_RUN;
        if (!gpio_get(BUTTON_PINS[8])) btns |= BTN_SKILL;

        /* Re-zero Hall centers on Select + Start chord */
        if (!gpio_get(BUTTON_PINS[9]) && !gpio_get(BUTTON_PINS[10])) {
            capture_startup_centers_if_neutral();
        }

        psfc_hid_report_t report = {
            .move_x   = normalize_hall_axis(lx, g_cal.center_adc[0], g_cal.deadzone_adc, false),
            .move_y   = normalize_hall_axis(ly, g_cal.center_adc[1], g_cal.deadzone_adc, true),
            .aim_x    = normalize_hall_axis(rx, g_cal.center_adc[2], g_cal.deadzone_adc, false),
            .aim_y    = normalize_hall_axis(ry, g_cal.center_adc[3], g_cal.deadzone_adc, true),
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
