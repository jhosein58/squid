pub mod gain;
pub mod mixing;

pub trait Waveform {
    fn process(&self, phase: f32) -> f32;
}
