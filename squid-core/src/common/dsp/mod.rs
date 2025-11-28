pub mod approx;
pub mod gain;
pub mod microprocessors;
pub mod mixing;
pub mod mixing_simd;
pub mod osc_core;
pub mod polyblep;
pub mod vecblock;

pub trait Waveform {
    fn process(&self, phase: f32) -> f32;
}
