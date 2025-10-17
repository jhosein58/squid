use crate::core::{dsp::mixing::Mixing, oscillator::Oscillator};

pub struct DualOsc {
    a_osc: Box<dyn Oscillator>,
    b_osc: Box<dyn Oscillator>,
    ratio: f32,
}

impl DualOsc {
    pub fn new(a: Box<dyn Oscillator>, b: Box<dyn Oscillator>, r: f32) -> Self {
        Self {
            a_osc: a,
            b_osc: b,
            ratio: r,
        }
    }

    pub fn set_ratio(&mut self, r: f32) {
        self.ratio = r;
    }
}

impl Oscillator for DualOsc {
    fn set_frequency(&mut self, frequency: f32) {
        self.a_osc.set_frequency(frequency);
        self.b_osc.set_frequency(frequency);
    }

    fn next_sample(&mut self) -> f32 {
        Mixing::crossfade(
            self.a_osc.next_sample(),
            self.b_osc.next_sample(),
            self.ratio,
        )
    }

    fn reset(&mut self) {
        self.a_osc.reset();
        self.b_osc.reset();
    }
}
