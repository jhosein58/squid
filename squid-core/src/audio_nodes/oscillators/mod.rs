use crate::AudioNode;

pub trait Oscillator: AudioNode + Clone {
    fn configure(&mut self, freq: f32, sample_rate: f32, phase: Option<f32>);
}

pub mod saw_osc;
pub mod sin_osc;
