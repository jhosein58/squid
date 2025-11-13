use core::simd::f32x8;

use crate::{
    AudioNode,
    process_context::{FixedBuf, ProcessContext},
    shapers::Shaper,
};

#[derive(Copy, Clone)]
pub struct SawShaper;

impl AudioNode for SawShaper {
    fn process(&mut self, ctx: &ProcessContext, outputs: &mut [&mut FixedBuf]) {
        let phase_buf = ctx.inputs[0].data;
        let out_buf = &mut outputs[0].data;

        let len = phase_buf.len().min(out_buf.len());

        let safe_phase_buf = &phase_buf[..len];
        let safe_out_buf = &mut out_buf[..len];

        let (phase_chunks, phase_rem) = safe_phase_buf.as_chunks::<8>();
        let (out_chunks, out_rem) = safe_out_buf.as_chunks_mut::<8>();

        for (phase_chunk, out_chunk) in phase_chunks.iter().zip(out_chunks.iter_mut()) {
            let phase_simd = f32x8::from_slice(phase_chunk);

            let result_simd = phase_simd * f32x8::splat(2.0) - f32x8::splat(1.0);
            out_chunk.copy_from_slice(&result_simd.to_array());
        }

        for (phase_sample, out_sample) in phase_rem.iter().zip(out_rem.iter_mut()) {
            *out_sample = *phase_sample * 2.0 - 1.0;
        }
    }

    fn reset(&mut self, _: f32) {}
}

impl Shaper for SawShaper {}
