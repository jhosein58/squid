// use crate::{
//     AudioNode, FloatVector,
//     microprocessors::gain::Gain,
//     process_context::{FixedBuf, ProcessContext},
//     processors::Processor,
// };

// pub struct GrainProc {
//     _db: Option<FloatVector>,
// }

// impl GrainProc {
//     pub fn new() -> Self {
//         GrainProc { _db: None }
//     }
// }q

// impl AudioNode for GrainProc {
//     fn process(&mut self, ctx: &ProcessContext, outputs: &mut [&mut FixedBuf]) {
//         let input = ctx.inputs[0].data;
//         let gains = ctx.inputs[1].data;
//         let out_buf = &mut outputs[0].data;

//         let gain = Gain::from_db(FloatVector::from_array(&gains));

//         out_buf.copy_from_slice(&gain.apply(FloatVector::from_array(&input)).to_array());
//     }
//     fn reset(&mut self, _: f32) {}
// }

// impl Processor for GrainProc {}
