use core::cmp;
use std::{
    array,
    simd::{Simd, StdFloat},
};

use crate::FloatVector;

#[derive(Debug, Clone, Copy, Default)]
pub struct PhaseAccumulator {
    phase: f32,
}

impl PhaseAccumulator {
    #[inline]
    pub fn new(initial_phase: f32) -> Self {
        PhaseAccumulator {
            phase: initial_phase - initial_phase.floor(),
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        self.phase = 0.0;
    }

    #[inline]
    pub fn get_phase(&self) -> f32 {
        self.phase
    }

    pub fn process_const(
        &mut self,
        base_freq: f32,
        sample_rate: f32,
        output_buffer: &mut FloatVector,
    ) {
        //self.process_const_scalar(base_freq, sample_rate, output_buffer);

        let phase_delta = base_freq / sample_rate;

        let indices: [f32; FloatVector::LANES] = array::from_fn(|i| i as f32);

        let v_indices = Simd::from_array(indices);

        let v_offsets = v_indices * Simd::splat(phase_delta);
        let step_per_vector = (FloatVector::LANES as f32) * phase_delta;

        let mut current_base_phase = self.phase;

        output_buffer.map_in_place(|_| {
            let v_base = Simd::splat(current_base_phase);
            let v_phase = v_base + v_offsets;

            current_base_phase += step_per_vector;

            current_base_phase -= current_base_phase.floor();

            v_phase - v_phase.floor()
        });

        self.phase = current_base_phase;
    }

    pub fn process_mod(
        &mut self,
        freq_buffer: &[f32],
        sample_rate: f32,
        output_buffer: &mut [f32],
    ) {
        let mut current_phase = self.phase;
        let len = cmp::min(freq_buffer.len(), output_buffer.len());
        let inv_sample_rate = 1.0 / sample_rate;

        for i in 0..len {
            output_buffer[i] = current_phase;
            let phase_delta = freq_buffer[i] * inv_sample_rate;
            current_phase += phase_delta;
            current_phase = current_phase - current_phase.floor();
        }

        self.phase = current_phase;
    }

    pub fn process_const_scalar(
        &mut self,
        base_freq: f32,
        sample_rate: f32,
        output_buffer: &mut [f32],
    ) {
        let phase_delta = base_freq / sample_rate;

        let mut current_phase = self.phase;

        for sample in output_buffer.iter_mut() {
            *sample = current_phase;

            current_phase += phase_delta;

            current_phase = current_phase - current_phase.floor();
        }

        self.phase = current_phase;
    }
}
