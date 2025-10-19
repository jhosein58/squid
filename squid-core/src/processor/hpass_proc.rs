use crate::processor::{Processor, lpass_proc::LowPassFilterProc};

pub struct HighPassFilterProc {
    low_pass: LowPassFilterProc,
}

impl HighPassFilterProc {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            low_pass: LowPassFilterProc::new(sample_rate),
        }
    }

    pub fn set_cutoff_hz(&mut self, cutoff_hz: f32) {
        self.low_pass.set_cutoff_hz(cutoff_hz);
    }
}

impl Processor for HighPassFilterProc {
    #[inline]
    fn process(&mut self, input: f32) -> f32 {
        let low_passed_signal = self.low_pass.process(input);
        let high_passed_signal = input - low_passed_signal;

        high_passed_signal
    }

    fn reset(&mut self) {
        self.low_pass.reset();
    }
}
