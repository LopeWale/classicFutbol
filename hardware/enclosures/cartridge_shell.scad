// Parallel Season FC — Removable Profile Cartridge Shell (`PSFC-CART-V1`)
// Dimensions: 44 mm tall x 32 mm wide x 8.4 mm thick, recessed club crest badge + USB-C tongue aperture

$fn = 40;

cart_w = 32.0;
cart_h = 44.0;
cart_d = 8.4;
wall   = 1.6;

module cartridge_body() {
    difference() {
        hull() {
            for (sx = [-1, 1], sy = [-1, 1]) {
                translate([sx * (cart_w/2 - 3.0), sy * (cart_h/2 - 3.0), 0])
                    cylinder(h = cart_d, r = 3.0, center = true);
            }
        }

        // Top grip ribs
        for (ry = [12, 15, 18]) {
            translate([0, ry, cart_d/2])
                cube([cart_w - 8.0, 1.2, 1.2], center = true);
        }

        // Recessed front club crest & participant label pocket
        translate([0, -2, cart_d/2 - 0.35])
            cube([24.0, 22.0, 0.8], center = true);

        // Internal flash carrier PCB cavity
        translate([0, -3.0, 0])
            cube([cart_w - 2*wall, cart_h - 10.0, cart_d - 2*wall], center = true);

        // Bottom USB-C male plug tongue opening
        translate([0, -cart_h/2 + 2.0, 0])
            cube([8.6, 8.0, 2.8], center = true);

        // Side magnetic detent pockets (3mm x 1.5mm N52 magnets)
        for (sx = [-1, 1]) {
            translate([sx * (cart_w/2 - 0.8), -12.0, 0])
                rotate([0, 90, 0])
                cylinder(h = 1.7, r = 1.55, center = true);
        }
    }
}

cartridge_body();
