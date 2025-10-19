use crate::{dsp::mixing::Mixing, oscillator::Oscillator};

pub struct DualOsc<I: Oscillator, J: Oscillator> {
    a_osc: I,
    b_osc: J,
    ratio: f32,
}

impl<I: Oscillator, J: Oscillator> DualOsc<I, J> {
    pub fn new(a: I, b: J, r: f32) -> Self {
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

impl<I: Oscillator, J: Oscillator> Oscillator for DualOsc<I, J> {
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
#[allow(non_snake_case)]
pub fn DualOsc<I: Oscillator, J: Oscillator>(a: I, b: J) -> DualOsc<I, J> {
    DualOsc::new(a, b, 0.5)
}
