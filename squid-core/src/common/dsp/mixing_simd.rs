use core::simd::f32x8;
use core::simd::{LaneCount, Simd, SupportedLaneCount};

pub struct MixingSimd;

impl MixingSimd {
    pub fn new<const N: usize>(a: Simd<f32, N>, b: Simd<f32, N>) -> Simd<f32, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        (a + b) / Simd::splat(2.)
    }

    pub fn stereo_pan(l: f32x8, r: f32x8, pan: f32) -> (f32x8, f32x8) {
        let pan = pan.clamp(-1.0, 1.0);
        let left_gain = (1.0 - pan) * 0.5;
        let right_gain = (1.0 + pan) * 0.5;

        let left = l * f32x8::splat(left_gain);
        let right = r * f32x8::splat(right_gain);

        (left, right)
    }

    pub fn mono_pan_both(s: f32x8, pan: f32) -> (f32x8, f32x8) {
        let pan = pan.clamp(-1.0, 1.0);
        let left_gain = (1.0 - pan) * 0.5;
        let right_gain = (1.0 + pan) * 0.5;

        let left = s * f32x8::splat(left_gain);
        let right = s * f32x8::splat(right_gain);

        (left, right)
    }

    pub fn mono_pan_left<const N: usize>(s: Simd<f32, N>, pan: f32) -> Simd<f32, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        let pan = pan.clamp(-1.0, 1.0);
        let left_gain = (1.0 - pan) * 0.5;

        s * Simd::splat(left_gain)
    }

    pub fn mono_pan_right<const N: usize>(s: Simd<f32, N>, pan: f32) -> Simd<f32, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        let pan = pan.clamp(-1.0, 1.0);
        let right_gain = (1.0 + pan) * 0.5;

        s * Simd::splat(right_gain)
    }
}
