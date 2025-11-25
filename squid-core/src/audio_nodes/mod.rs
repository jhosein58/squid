use crate::process_context::{FixedBuf, ProcessContext};

pub mod process_context;

pub trait AudioNode {
    fn process(&mut self, ctx: &ProcessContext, outputs: &mut [&mut FixedBuf]);
    fn reset(&mut self, sample_rate: f32);
}

pub mod effects;
pub mod modulators;
pub mod oscillators;
pub mod phase_accumulator;
pub mod phase_tracker;
pub mod processors;
pub mod shapers;
pub mod synths;
pub mod voice;
