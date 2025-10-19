use core::f32::consts::PI;

use libm::sinf;

use crate::{define_oscillator, oscillator::Oscillator};

define_oscillator!(SinOsc, |phase: f32| sinf(phase));

define_oscillator!(SquareOsc, |phase: f32| {
    if phase < PI { 1.0 } else { -1.0 }
});

define_oscillator!(SawOsc, |phase: f32| { 1.0 - (phase / PI) });

define_oscillator!(TriangleOsc, |phase: f32| {
    ((phase / PI) - 1.0).abs() * 2.0 - 1.0
});

//define_oscillator!(RampOsc, |phase: f32| { (phase / PI) - 1.0 });

pub struct NoiseOsc {
    seed: u32,
}

impl NoiseOsc {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }
}

impl Oscillator for NoiseOsc {
    fn next_sample(&mut self) -> f32 {
        self.seed = self
            .seed
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        let random_float = (self.seed as f32) / (u32::MAX as f32);
        random_float * 2.0 - 1.0
    }

    fn set_frequency(&mut self, _frequency: f32) {}

    fn reset(&mut self) {}
}

#[allow(non_snake_case)]
pub fn NoiseOsc(seed: u32) -> NoiseOsc {
    NoiseOsc::new(seed)
}
