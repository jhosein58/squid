use core::simd::f32x8;

use sleef::Sleef;

use crate::FloatVector;

pub struct MixingSimd;

impl MixingSimd {
    pub fn new(a: f32x8, b: f32x8) -> f32x8 {
        (a + b) / f32x8::splat(2.)
    }

    pub fn stereo_pan(l: f32x8, r: f32x8, pan: f32) -> (f32x8, f32x8) {
        let pan = pan.clamp(-1.0, 1.0);
        let left_gain = (1.0 - pan) * 0.5;
        let right_gain = (1.0 + pan) * 0.5;

        let left = l * f32x8::splat(left_gain);
        let right = r * f32x8::splat(right_gain);

        (left, right)
    }

    pub fn mono_pan(s: f32x8, pan: f32) -> (f32x8, f32x8) {
        let pan = pan.clamp(-1.0, 1.0);
        let left_gain = (1.0 - pan) * 0.5;
        let right_gain = (1.0 + pan) * 0.5;

        let left = s * f32x8::splat(left_gain);
        let right = s * f32x8::splat(right_gain);

        (left, right)
    }
}
