pub mod delay_proc;
pub mod gain_proc;
pub mod passthrough_proc;

pub trait Processor {
    fn process(&mut self, input: f32) -> f32;
    fn reset(&mut self);
}
