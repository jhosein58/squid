use crate::{
    AudioNode, MAX_BLOCK_SIZE,
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
        let mut phases = [0.; MAX_BLOCK_SIZE];

        self.phasor
            .process_const(self.freq, self.sample_rate, &mut phases);

        let ctx = ProcessContext {
            sample_rate: self.sample_rate,
            events: &[],
            inputs: &[&FixedBuf { data: phases }],
        };

        self.shaper.process(&ctx, outputs);

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
