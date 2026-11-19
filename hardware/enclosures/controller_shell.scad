// Parallel Season FC — Parametric Controller Shell with Top USB-C Cartridge Dock
// Units: millimeters (render with OpenSCAD >= 2021.01)

$fn = 48;

shell_width  = 152.0;
shell_height = 96.0;
shell_depth  = 28.0;
wall_thick   = 2.6;

cart_bay_w   = 33.2; // 32.0 mm cartridge + 1.2 mm clearance
cart_bay_d   = 9.2;  // 8.4 mm cartridge + 0.8 mm clearance
cart_bay_h   = 18.0;

module rounded_brick(w, h, d, r) {
    hull() {
        for (sx = [-1, 1], sy = [-1, 1]) {
            translate([sx * (w/2 - r), sy * (h/2 - r), 0])
                cylinder(h = d, r = r, center = true);
        }
    }
}

module controller_top_shell() {
    difference() {
        union() {
            rounded_brick(shell_width, shell_height, shell_depth / 2, 18.0);
            // Ergonomic palm grips
            for (sx = [-1, 1]) {
                translate([sx * 56, -28, -2])
                    rotate([0, 0, sx * -14])
                    rounded_brick(36, 56, shell_depth / 2, 14.0);
            }
        }

        // Hollow interior cavity
        translate([0, 0, -wall_thick])
            rounded_brick(shell_width - 2*wall_thick, shell_height - 2*wall_thick, shell_depth / 2, 16.0);

        // Top-loading PSFC-CART-V1 Cartridge Bay
        translate([0, shell_height/2 - cart_bay_h/2 + 1.0, 0])
            cube([cart_bay_w, cart_bay_h, cart_bay_d], center = true);

        // Left & Right Hall-effect thumbstick wells
        translate([-42, 8, 0]) cylinder(h = 20, r = 13.5, center = true);
        translate([ 24, -14, 0]) cylinder(h = 20, r = 13.5, center = true);

        // Diamond 4-button cluster (Pass / Shoot / Through / Lob)
        for (offset = [[0, -11], [11, 0], [0, 11], [-11, 0]]) {
            translate([44 + offset[0], 10 + offset[1], 0])
                cylinder(h = 20, r = 5.4, center = true);
        }

        // Start & Select apertures
        translate([-9, 14, 0]) cylinder(h = 20, r = 3.2, center = true);
        translate([ 9, 14, 0]) cylinder(h = 20, r = 3.2, center = true);
    }
}

controller_top_shell();
