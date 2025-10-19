use core::f32::consts::TAU;

use libm::expf;

use crate::processor::Processor;

pub struct LowPassFilterProc {
    coefficient: f32,
    last_output: f32,
    sample_rate: f32,
}

impl LowPassFilterProc {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            coefficient: 0.99,
            last_output: 0.0,
            sample_rate,
        }
    }

    pub fn set_cutoff_hz(&mut self, cutoff_hz: f32) {
        let clamped_cutoff = cutoff_hz.max(0.0).min(self.sample_rate / 2.0);

        self.coefficient = expf(-TAU * clamped_cutoff / self.sample_rate);
    }
}

impl Processor for LowPassFilterProc {
    #[inline]
    fn process(&mut self, input: f32) -> f32 {
        let new_output = (1.0 - self.coefficient) * input + self.coefficient * self.last_output;
        self.last_output = new_output;

        new_output
    }

    fn reset(&mut self) {
        self.last_output = 0.0;
    }
}
