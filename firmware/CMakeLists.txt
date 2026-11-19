cmake_minimum_required(VERSION 3.13)

# Pico SDK import (set PICO_SDK_PATH in environment before building)
include($ENV{PICO_SDK_PATH}/external/pico_sdk_import.cmake)

project(psfc_rp2040_controller C CXX ASM)
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

pico_sdk_init()

add_executable(psfc_controller
    main.c
    usb_descriptors.c
)

target_include_directories(psfc_controller PRIVATE ${CMAKE_CURRENT_LIST_DIR})

target_link_libraries(psfc_controller
    pico_stdlib
    hardware_adc
    hardware_gpio
    hardware_flash
    hardware_sync
    tinyusb_device
    tinyusb_board
)

pico_add_extra_outputs(psfc_controller)
