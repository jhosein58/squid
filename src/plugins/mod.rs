pub mod func_osc;

use crate::buffer::AudioBufferML;

pub trait Plugin: Send + Sync + 'static {
    fn name(&self) -> &str;
    fn num_inputs(&self) -> usize;
    fn num_outputs(&self) -> usize;
    fn prepare(&mut self, sample_rate: u32, max_block_size: usize);
    fn next_sample(&mut self) -> f32;
    fn process(&mut self, inputs: &[&AudioBufferML], outputs: &mut [&mut AudioBufferML]);
}
pub type BoxedPlugin = Box<dyn Plugin>;
