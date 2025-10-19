use core::f32::consts::FRAC_PI_2;

use libm::{cosf, sinf, tanhf};

use crate::dsp::gain::Gain;

pub struct Mixing;

impl Mixing {
    pub fn average(a: f32, b: f32) -> f32 {
        (a + b) / 2.0
    }

    pub fn crossfade(a: f32, b: f32, ratio: f32) -> f32 {
        let phase = ratio.clamp(0.0, 1.0) * FRAC_PI_2;
        (a * cosf(phase)) + (b * sinf(phase))
    }

    pub fn average_all(signals: &[f32]) -> f32 {
        signals.iter().sum::<f32>() / signals.len() as f32
    }
    pub fn weighted_sum(signals: &[f32], gains: &[Gain]) -> f32 {
        signals
            .iter()
            .zip(gains.iter())
            .map(|(signal, gain)| signal * gain.as_amplitude())
            .sum()
    }
    pub fn saturate(signal: f32, drive: f32) -> f32 {
        tanhf(signal * drive)
    }

    pub fn weighted_sum_saturated(signals: &[f32], gains: &[Gain], drive: f32) -> f32 {
        let sum = Self::weighted_sum(signals, gains);
        Self::saturate(sum, drive)
    }
}
