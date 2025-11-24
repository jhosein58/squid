use core::f32::consts::PI;

use crate::LUT_RESOLUTION;

pub const fn sin_const(x: f32) -> f32 {
    let mut y = x % (2.0 * PI);
    if y > PI {
        y -= 2.0 * PI;
    }
    if y < -PI {
        y += 2.0 * PI;
    }

    let y2 = y * y;
    let y3 = y2 * y;
    let y5 = y3 * y2;
    let y7 = y5 * y2;

    y - y3 * (1.0 / 6.0) + y5 * (1.0 / 120.0) - y7 * (1.0 / 5040.0)
}

pub static SIN_TABLE: [f32; LUT_RESOLUTION] = {
    let mut table = [0.0; LUT_RESOLUTION];

    let mut i = 0;
    while i < LUT_RESOLUTION {
        table[i] = sin_const(i as f32 * 2.0 * PI / LUT_RESOLUTION as f32);
        i += 1;
    }
    table
};
