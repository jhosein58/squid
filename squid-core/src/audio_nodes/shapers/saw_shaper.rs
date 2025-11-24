use std::simd::Simd;

use crate::{
    AudioNode,
    process_context::{FixedBuf, ProcessContext},
    shapers::Shaper,
};

#[derive(Copy, Clone)]
pub struct SawShaper;

impl AudioNode for SawShaper {
    fn process(&mut self, ctx: &ProcessContext, outputs: &mut [&mut FixedBuf]) {
        let phase_buf = ctx.inputs[0];
        let out_buf = &mut outputs[0];

        let v_two = Simd::splat(2.);
        let v_one = Simd::splat(1.);

        out_buf.map_from(phase_buf, |c| (c * v_two) - v_one);
    }

    fn reset(&mut self, _: f32) {}
}

impl Shaper for SawShaper {}
