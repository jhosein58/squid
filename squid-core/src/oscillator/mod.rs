pub mod composite_osc;
pub mod func_osc;
pub mod primetives;

pub trait Oscillator: Send + Sync {
    fn set_frequency(&mut self, frequency: f32);
    fn next_sample(&mut self) -> f32;
    fn reset(&mut self);

    fn get_phase(&self) -> f32;
    fn set_phase(&mut self, phase: f32);
}
