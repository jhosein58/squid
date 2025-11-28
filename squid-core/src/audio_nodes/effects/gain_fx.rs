use crate::{
    AudioNode,
    dsp::microprocessors::gain::Gain,
    process_context::{FixedBuf, ProcessContext},
};

#[derive(Clone, Copy)]
pub struct GainFx;

impl AudioNode for GainFx {
    fn process(&mut self, ctx: &ProcessContext, outputs: &mut [&mut FixedBuf]) {
        let input_l = ctx.inputs[0];
        let input_r = ctx.inputs[1];
        let gains = ctx.inputs[2];

        outputs[0].zip_map_from(input_l, gains, |l, g| Gain::apply(g, l));
        outputs[1].zip_map_from(input_r, gains, |r, g| Gain::apply(g, r));
    }

    fn reset(&mut self, _: f32) {}
}
