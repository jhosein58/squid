// Xorshift32 random generator for fast DSP/audio use.
//
// Example:
// let rng = Rand::new(123);
// let x = rng.next_f32_bipolar(); // random value in [-1.0, 1.0]

use core::cell::Cell;
use core::simd::{LaneCount, Simd, SupportedLaneCount, num::SimdFloat};

#[derive(Clone)]
pub struct Rand {
    state: Cell<u32>,
}

impl Rand {
    pub fn new(mut seed: u32) -> Self {
        if seed == 0 {
            seed = 0xD8163841;
        }
        Self {
            state: Cell::new(seed),
        }
    }

    #[inline(always)]
    pub fn next_u32(&self) -> u32 {
        let mut x = self.state.get();
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state.set(x);
        x
    }

    #[inline(always)]
    pub fn next_f32(&self) -> f32 {
        let rnd_bits = self.next_u32();

        // Mantissa Trick
        let float_bits = (rnd_bits & 0x007FFFFF) | 0x3F800000;
        let val_1_to_2 = f32::from_bits(float_bits);
        val_1_to_2 - 1.0
    }

    #[inline(always)]
    pub fn next_f32_bipolar(&self) -> f32 {
        let rnd_bits = self.next_u32();
        let float_bits = (rnd_bits & 0x007FFFFF) | 0x3F800000;
        let val_1_to_2 = f32::from_bits(float_bits);
        (val_1_to_2 - 1.5) * 2.0
    }

    #[inline(always)]
    pub fn next_range_u32(&self, min: u32, max: u32) -> u32 {
        debug_assert!(min <= max);

        let range = (max - min) + 1;
        let x = self.next_u32();

        // Lemire's Fast Range
        let product = (x as u64).wrapping_mul(range as u64);
        let offset = (product >> 32) as u32;

        min + offset
    }
    #[inline(always)]
    pub fn next_range_f32(&self, min: f32, max: f32) -> f32 {
        min + (self.next_f32() * (max - min))
    }
}

#[derive(Clone)]
pub struct SimdRand<const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
{
    state: Cell<Simd<u32, N>>,
}

impl<const N: usize> SimdRand<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    pub fn new(seed: u32) -> Self {
        let mut seeds = [0u32; N];
        for i in 0..N {
            let s = seed.wrapping_add((i as u32).wrapping_mul(0x9E3779B9));
            seeds[i] = if s == 0 { 0xD8163841 } else { s };
        }
        Self {
            state: Cell::new(Simd::from_array(seeds)),
        }
    }

    #[inline(always)]
    pub fn next_u32(&self) -> Simd<u32, N> {
        let mut x = self.state.get();
        x ^= x << Simd::splat(13);
        x ^= x >> Simd::splat(17);
        x ^= x << Simd::splat(5);
        self.state.set(x);
        x
    }

    #[inline(always)]
    pub fn next_f32_bipolar(&self) -> Simd<f32, N> {
        let rnd_bits = self.next_u32();
        let mask_mantissa = Simd::splat(0x007FFFFF);
        let exponent_bits = Simd::splat(0x3F800000);

        let float_bits = (rnd_bits & mask_mantissa) | exponent_bits;
        let val_1_to_2 = Simd::<f32, N>::from_bits(float_bits);

        (val_1_to_2 - Simd::splat(1.5)) * Simd::splat(2.0)
    }

    #[inline(always)]
    pub fn next_range_f32(&self, min: Simd<f32, N>, max: Simd<f32, N>) -> Simd<f32, N> {
        let rnd_bits = self.next_u32();

        let mask_mantissa = Simd::splat(0x007FFFFF);
        let exponent_bits = Simd::splat(0x3F800000);

        let float_bits = (rnd_bits & mask_mantissa) | exponent_bits;
        let val_1_to_2 = Simd::<f32, N>::from_bits(float_bits);

        let normalized = val_1_to_2 - Simd::splat(1.0);

        min + (normalized * (max - min))
    }

    #[inline(always)]
    pub fn next_range_f32_splat(&self, min: f32, max: f32) -> Simd<f32, N> {
        self.next_range_f32(Simd::splat(min), Simd::splat(max))
    }
}
