pub mod bitcrusher_proc;
pub mod chain_proc;
pub mod delay_proc;
pub mod fdelay_proc;
pub mod gain_proc;
pub mod hclip_proc;
pub mod hpass_proc;
pub mod lpass_proc;
pub mod passthrough_proc;
pub mod pitchs_proc;
pub mod saturator_proc;

pub trait Processor {
    fn process(&mut self, input: f32) -> f32;
    fn reset(&mut self);
}
