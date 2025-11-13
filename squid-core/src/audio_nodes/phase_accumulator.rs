use core::cmp;
use core::simd::f32x8;

use sleef::Sleef;

const LANES: usize = 8;
type FloatVector = f32x8;

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
    pub fn process_const(&mut self, base_freq: f32, sample_rate: f32, output_buffer: &mut [f32]) {
        let phase_delta = base_freq / sample_rate;
        let phase_delta_vec = FloatVector::splat(phase_delta);

        let lane_indices = FloatVector::from_array([0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
        let increments = lane_indices * phase_delta_vec;

        let mut current_phase = self.phase;

        let (chunks, remainder) = output_buffer.as_chunks_mut::<LANES>();

        for chunk in chunks {
            let start_phase_vec = FloatVector::splat(current_phase);

            let phase_vec = start_phase_vec + increments;

            let wrapped_phase_vec = phase_vec - phase_vec.floor();

            *chunk = wrapped_phase_vec.to_array();

            current_phase += LANES as f32 * phase_delta;
            current_phase = current_phase - current_phase.floor();
        }

        for sample in remainder.iter_mut() {
            *sample = current_phase;
            current_phase += phase_delta;
            current_phase = current_phase - current_phase.floor();
        }

        self.phase = current_phase;
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
