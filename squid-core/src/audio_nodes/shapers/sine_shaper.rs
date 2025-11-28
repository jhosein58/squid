use core::f32::consts::TAU;
use core::simd::Simd;

use sleef::f32x::sin_fast;

use crate::{
    AudioNode,
    process_context::{FixedBuf, ProcessContext},
    shapers::Shaper,
};

#[derive(Copy, Clone)]
pub struct SineShaper;

impl AudioNode for SineShaper {
    fn process(&mut self, ctx: &ProcessContext, outputs: &mut [&mut FixedBuf]) {
        let phase_buf = ctx.inputs[0];
        let out_buf = &mut outputs[0];

        let v_tau = Simd::splat(TAU);
        out_buf.map_from(phase_buf, |c| sin_fast(c * v_tau));
    }

    fn reset(&mut self, _: f32) {}
}

impl Shaper for SineShaper {}
