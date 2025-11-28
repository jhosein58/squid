use core::simd::{LaneCount, Simd, SupportedLaneCount, cmp::SimdPartialOrd, num::SimdFloat};

use crate::FloatVector;

#[derive(Clone, Copy)]
pub struct PhaseTracker<const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
{
    prev_phase: Simd<f32, N>,
    is_initialized: bool,
}

impl<const N: usize> PhaseTracker<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    pub fn new() -> Self {
        Self {
            prev_phase: Simd::splat(0.0),
            is_initialized: false,
        }
    }

    #[inline(always)]
    pub fn get_dt(&mut self, current_phase: Simd<f32, N>) -> Simd<f32, N> {
        if !self.is_initialized {
            self.prev_phase = current_phase;
            self.is_initialized = true;
            return Simd::splat(0.0);
        }

        let half = Simd::<f32, N>::splat(0.5);
        let one = Simd::splat(1.0);
        let zero = Simd::splat(0.0);

        let mut delta = current_phase - self.prev_phase;

        let wrap_mask = delta.simd_lt(-half);
        let correction = wrap_mask.select(one, zero);

        delta = delta + correction;

        self.prev_phase = current_phase;

        delta.simd_max(zero) / Simd::splat(FloatVector::LANES as f32)
    }

    pub fn reset(&mut self) {
        self.is_initialized = false;
    }
}
