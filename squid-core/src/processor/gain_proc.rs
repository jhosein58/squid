// use crate::processor::Processor;

// #[derive(Debug, Clone, Copy)]
// // pub struct GainProc {
// //     gain: Gain,
// // }

// impl GainProc {
//     // pub fn new(gain: Gain) -> Self {
//     //     Self { gain }
//     // }

//     // pub fn from_db(db: f32) -> Self {
//     //     Self {
//     //         gain: Gain::from_db(db),
//     //     }
//     // }

//     // pub fn from_amplitude(amplitude: f32) -> Self {
//     //     Self {
//     //         gain: Gain::from_amplitude(amplitude),
//     //     }
//     // }

//     pub fn set_gain(&mut self, new_gain: Gain) {
//         self.gain = new_gain;
//     }
// }

// impl Processor for GainProc {
//     #[inline]
//     fn process(&mut self, input: f32) -> f32 {
//         input * self.gain.as_amplitude()
//     }

//     #[inline]
//     fn reset(&mut self) {}
// }
