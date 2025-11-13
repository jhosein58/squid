use core::{f32::consts::TAU, simd::f32x8};

use sleef::Sleef;

use crate::{
    AudioNode,
    process_context::{FixedBuf, ProcessContext},
    shapers::Shaper,
};

#[derive(Copy, Clone)]
pub struct SineShaper;

impl AudioNode for SineShaper {
    fn process(&mut self, ctx: &ProcessContext, outputs: &mut [&mut FixedBuf]) {
        let phase_buf = ctx.inputs[0].data;
        let out_buf = &mut outputs[0].data;

        panic!()
        // out_buf.copy_from_slice(
        //     &Vec8::from_array(&phase_buf)
        //         .in_place::<Mul>(&Vec8::splat(TAU))
        //         .sin()
        //         .to_array(),
        // );
    }

    fn reset(&mut self, _: f32) {}
}

impl Shaper for SineShaper {}
