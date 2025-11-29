use core::simd::Simd;

use sleef::f32x::sin_fast;

use crate::{
    AudioNode, SIMD_LANES,
    oscillators::Oscillator,
    phase_accumulator::PhaseAccumulator,
    process_context::{FixedBuf, ProcessContext},
};

#[derive(Copy, Clone)]
pub struct SinOsc {
    freq: f32,
    sample_rate: f32,
    phasor: PhaseAccumulator<{ SIMD_LANES }>,
}

impl SinOsc {
    pub fn new() -> Self {
        Self {
            freq: 0.,
            sample_rate: 0.,
            phasor: PhaseAccumulator::new(0.),
        }
    }
}

impl AudioNode for SinOsc {
    fn process(&mut self, _: &ProcessContext, outputs: &mut [&mut FixedBuf]) {
        let (left_slice, right_slice) = outputs.split_at_mut(1);
        let mut left_buf = &mut left_slice[0];

        self.phasor
            .process_const(self.freq, self.sample_rate, &mut left_buf);

        let v_tau = Simd::splat(core::f32::consts::TAU);
        left_buf.data.map_in_place(|phase| sin_fast(phase * v_tau));

        right_slice[0].replace(left_buf);
    }

    fn reset(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.phasor = PhaseAccumulator::new(0.);
    }
}

impl Oscillator for SinOsc {
    fn configure(&mut self, freq: f32, sample_rate: f32, phase: Option<f32>) {
        self.freq = freq;
        self.sample_rate = sample_rate;
        if let Some(p) = phase {
            self.phasor = PhaseAccumulator::new(p);
        }
    }
}
