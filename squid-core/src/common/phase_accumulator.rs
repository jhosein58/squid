use core::simd::num::SimdUint;
use core::simd::{LaneCount, SupportedLaneCount};
use core::{array, simd::Simd};

use crate::Fv;

#[derive(Debug, Clone, Copy, Default)]
pub struct PhaseAccumulator<const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
{
    phase_u32: u32,
}

impl<const N: usize> PhaseAccumulator<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    const SCALE: f32 = 4_294_967_296.0;
    const INV_SCALE: f32 = 1.0 / Self::SCALE;

    #[inline]
    pub fn new(initial_phase: f32) -> Self {
        Self {
            phase_u32: (initial_phase * 4_294_967_296.0) as u32,
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        self.phase_u32 = 0;
    }

    #[inline]
    pub fn get_phase(&self) -> f32 {
        self.phase_u32 as f32 * (1.0 / 4_294_967_296.0)
    }

    pub fn next_const(&mut self, base_freq: f32, sample_rate: f32) -> Simd<f32, N> {
        let inc = ((base_freq / sample_rate) * Self::SCALE) as u32;
        let mut phase_u32 = self.phase_u32;
        let indices: [u32; N] = array::from_fn(|i| i as u32);
        let v_offsets = Simd::from_array(indices) * Simd::splat(inc);
        let step = inc.wrapping_mul(N as u32);
        let v_norm = Simd::splat(Self::INV_SCALE);
        let v_base = Simd::splat(phase_u32);

        let v_phase_u32 = v_base + v_offsets;

        phase_u32 = phase_u32.wrapping_add(step);
        self.phase_u32 = phase_u32;

        v_phase_u32.cast::<f32>() * v_norm
    }

    pub fn process_const(&mut self, base_freq: f32, sample_rate: f32, output_buffer: &mut Fv<N>) {
        let inc = ((base_freq / sample_rate) * Self::SCALE) as u32;

        let mut phase_u32 = self.phase_u32;

        let indices: [u32; N] = array::from_fn(|i| i as u32);
        let v_offsets = Simd::from_array(indices) * Simd::splat(inc);
        let step = inc.wrapping_mul(N as u32);
        let v_norm = Simd::splat(Self::INV_SCALE);

        output_buffer.map_in_place(|_| {
            let v_base = Simd::splat(phase_u32);

            let v_phase_u32 = v_base + v_offsets;

            phase_u32 = phase_u32.wrapping_add(step);

            v_phase_u32.cast::<f32>() * v_norm
        });

        self.phase_u32 = phase_u32;
    }

    // pub fn process_mod(
    //     &mut self,
    //     freq_buffer: &[f32],
    //     sample_rate: f32,
    //     output_buffer: &mut [f32],
    // ) {
    //     let mut current_phase = self.phase;
    //     let len = cmp::min(freq_buffer.len(), output_buffer.len());
    //     let inv_sample_rate = 1.0 / sample_rate;

    //     for i in 0..len {
    //         output_buffer[i] = current_phase;
    //         let phase_delta = freq_buffer[i] * inv_sample_rate;
    //         current_phase += phase_delta;
    //         current_phase = current_phase - current_phase.floor();
    //     }

    //     self.phase = current_phase;
    // }

    // pub fn process_const_scalar(
    //     &mut self,
    //     base_freq: f32,
    //     sample_rate: f32,
    //     output_buffer: &mut [f32],
    // ) {
    //     let phase_delta = base_freq / sample_rate;

    //     let mut current_phase = self.phase;

    //     for sample in output_buffer.iter_mut() {
    //         *sample = current_phase;

    //         current_phase += phase_delta;

    //         current_phase = current_phase - current_phase.floor();
    //     }

    //     self.phase = current_phase;
    // }
}
