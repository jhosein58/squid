pub mod gain;
pub mod mixing;
pub mod mixing_simd;
pub mod vecblock;

pub trait Waveform {
    fn process(&self, phase: f32) -> f32;
}
