use core::{
    f32::consts::TAU,
    simd::{LaneCount, Simd, SupportedLaneCount, cmp::SimdPartialOrd},
};

use sleef::{Sleef, f32x::sin_fast};

use crate::rand::SimdRand;

pub struct ClassicOscillator<const N: usize>;

impl<const N: usize> ClassicOscillator<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    #[inline(always)]
    pub fn sin(phase: Simd<f32, N>) -> Simd<f32, N> {
        sin_fast(phase * Simd::splat(TAU))
    }

    #[inline(always)]
    pub fn saw(phase: Simd<f32, N>) -> Simd<f32, N> {
        // Hope the compiler is smart enough to fuse this into a single FMA instruction
        (phase * Simd::splat(2.)) + Simd::splat(-1.)
    }

    #[inline(always)]
    pub fn ramp(phase: Simd<f32, N>) -> Simd<f32, N> {
        Simd::splat(1.0) - (phase * Simd::splat(2.0))
    }

    pub fn square(phase: Simd<f32, N>) -> Simd<f32, N> {
        let mask = Simd::simd_gt(phase, Simd::splat(0.5));
        mask.select(Simd::splat(1.), Simd::splat(-1.))
    }

    #[inline(always)]
    pub fn pulse(phase: Simd<f32, N>, width: Simd<f32, N>) -> Simd<f32, N> {
        let mask = phase.simd_lt(width);
        mask.select(Simd::splat(1.0), Simd::splat(-1.0))
    }

    #[inline(always)]
    pub fn triangle(phase: Simd<f32, N>) -> Simd<f32, N> {
        let centered = phase - Simd::splat(0.5);
        Simd::splat(1.0) - (centered.abs() * Simd::splat(4.0))
    }

    #[inline(always)]
    pub fn var_triangle(phase: Simd<f32, N>, skew: Simd<f32, N>) -> Simd<f32, N> {
        let epsilon = Simd::splat(1e-5);
        let sk = skew.max(epsilon).min(Simd::splat(1.0) - epsilon);

        let rising = phase / sk;
        let falling = (Simd::splat(1.0) - phase) / (Simd::splat(1.0) - sk);

        let mask = phase.simd_lt(sk);
        let raw = mask.select(rising, falling);

        (raw * Simd::splat(2.0)) - Simd::splat(1.0)
    }

    #[inline(always)]
    pub fn white_noise(rng: &SimdRand<N>) -> Simd<f32, N> {
        rng.next_f32_bipolar()
    }
}
