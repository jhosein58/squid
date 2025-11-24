use std::simd::Simd;

use crate::{
    AudioNode,
    oscillators::Oscillator,
    phase_accumulator::PhaseAccumulator,
    process_context::{FixedBuf, ProcessContext},
    shapers::saw_shaper::SawShaper,
};

#[derive(Copy, Clone)]
pub struct SawOsc {
    freq: f32,
    sample_rate: f32,
    phasor: PhaseAccumulator,
    shaper: SawShaper,
}

impl SawOsc {
    pub fn new() -> Self {
        Self {
            freq: 0.,
            sample_rate: 0.,
            phasor: PhaseAccumulator::new(0.),
            shaper: SawShaper,
        }
    }
}

impl AudioNode for SawOsc {
    fn process(&mut self, _: &ProcessContext, outputs: &mut [&mut FixedBuf]) {
        self.phasor
            .process_const(self.freq, self.sample_rate, &mut outputs[0]);

        let v_two = Simd::splat(2.);
        let v_one = Simd::splat(1.);

        outputs[0].map_in_place(|c| (c * v_two) - v_one);

        //outputs[1].data = outputs[0].data.clone();
    }

    fn reset(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.phasor = PhaseAccumulator::new(0.);
    }
}

impl Oscillator for SawOsc {
    fn configure(&mut self, freq: f32, sample_rate: f32, phase: Option<f32>) {
        self.freq = freq;
        self.sample_rate = sample_rate;
        if let Some(p) = phase {
            self.phasor = PhaseAccumulator::new(p);
        }
    }
}
