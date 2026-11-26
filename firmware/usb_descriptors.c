/*
 * Parallel Season FC — Custom RP2040 Controller USB HID Descriptors
 * Exposes a 1000 Hz (`bInterval = 1`) 8-byte standard HID Gamepad report:
 *   4 x int8 axes (X = LX, Y = LY, Z = RX, Rz = RY) + 16 PHYSICAL buttons (W3C standard order,
 *   see main.c PHYS_*) + 16-bit constant sequence counter.
 * Gameplay actions are NOT encoded here; the client's ControlProfile maps physical -> action.
 */

#include "tusb.h"
#include <string.h>

#define PSFC_USB_VID 0x1209 /* pid.codes open-hardware VID */
#define PSFC_USB_PID 0xF5FC /* Parallel Season FC 5v5 Pad */

/* 8-byte HID report: 4 signed int8 axes (LX, LY, RX, RY) + 16 physical buttons + 16-bit sequence */
uint8_t const desc_hid_report[] = {
    0x05, 0x01,       /* Usage Page (Generic Desktop Ctrls) */
    0x09, 0x05,       /* Usage (Game Pad) */
    0xA1, 0x01,       /* Collection (Application) */
    0x09, 0x30,       /*   Usage (X - Left Stick X) */
    0x09, 0x31,       /*   Usage (Y - Left Stick Y) */
    0x09, 0x32,       /*   Usage (Z - Right Stick X) */
    0x09, 0x35,       /*   Usage (Rz - Right Stick Y) */
    0x15, 0x81,       /*   Logical Minimum (-127) */
    0x25, 0x7F,       /*   Logical Maximum (127) */
    0x75, 0x08,       /*   Report Size (8 bits) */
    0x95, 0x04,       /*   Report Count (4 axes) */
    0x81, 0x02,       /*   Input (Data,Var,Abs) */
    0x05, 0x09,       /*   Usage Page (Button) */
    0x19, 0x01,       /*   Usage Minimum (Button 1  = South) */
    0x29, 0x10,       /*   Usage Maximum (Button 16 = D-Pad Right) */
    0x15, 0x00,       /*   Logical Minimum (0) */
    0x25, 0x01,       /*   Logical Maximum (1) */
    0x75, 0x01,       /*   Report Size (1 bit) */
    0x95, 0x10,       /*   Report Count (16 buttons) */
    0x81, 0x02,       /*   Input (Data,Var,Abs) */
    0x75, 0x10,       /*   Report Size (16 bits reserved / sequence) */
    0x95, 0x01,       /*   Report Count (1) */
    0x81, 0x03,       /*   Input (Const,Var,Abs) */
    0xC0              /* End Collection */
};

tusb_desc_device_t const desc_device = {
    .bLength            = sizeof(tusb_desc_device_t),
    .bDescriptorType    = TUSB_DESC_DEVICE,
    .bcdUSB             = 0x0200,
    .bDeviceClass       = 0x00,
    .bDeviceSubClass    = 0x00,
    .bDeviceProtocol    = 0x00,
    .bMaxPacketSize0    = CFG_TUD_ENDPOINT0_SIZE,
    .idVendor           = PSFC_USB_VID,
    .idProduct          = PSFC_USB_PID,
    .bcdDevice          = 0x0200, /* v2: physical-button report (v1 emitted InputFrame gameplay bits) */
    .iManufacturer      = 0x01,
    .iProduct           = 0x02,
    .iSerialNumber      = 0x03,
    .bNumConfigurations = 0x01
};

uint8_t const *tud_descriptor_device_cb(void) {
    return (uint8_t const *)&desc_device;
}

uint8_t const *tud_hid_descriptor_report_cb(uint8_t instance) {
    (void)instance;
    return desc_hid_report;
}

enum {
    ITF_NUM_HID,
    ITF_NUM_TOTAL
};

#define CONFIG_TOTAL_LEN (TUD_CONFIG_DESC_LEN + TUD_HID_DESC_LEN)
#define EPNUM_HID        0x81

/* bInterval = 1 -> 1000 Hz USB Full-Speed polling (<1 ms transport jitter) */
uint8_t const desc_configuration[] = {
    TUD_CONFIG_DESCRIPTOR(1, ITF_NUM_TOTAL, 0, CONFIG_TOTAL_LEN, TUSB_DESC_CONFIG_ATT_REMOTE_WAKEUP, 250),
    TUD_HID_DESCRIPTOR(ITF_NUM_HID, 0, HID_ITF_PROTOCOL_NONE, sizeof(desc_hid_report), EPNUM_HID, CFG_TUD_HID_EP_BUFSIZE, 1)
};

uint8_t const *tud_descriptor_configuration_cb(uint8_t index) {
    (void)index;
    return desc_configuration;
}
