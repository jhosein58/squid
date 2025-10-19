// use crate::core::{modulator::Modulator, timing::Transport};

// fn apply_curve(progress: f32, curve: EnvelopeCurve) -> f32 {
//     match curve {
//         EnvelopeCurve::Linear => progress,
//         EnvelopeCurve::Exponential(exponent) => {
//             if exponent > 0.0 {
//                 progress.max(0.0).powf(exponent)
//             } else {
//                 progress
//             }
//         }
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum EnvelopeCurve {
//     Linear,
//     Exponential(f32),
// }

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum EnvelopeStage {
//     Idle,
//     Attack,
//     Decay,
//     Sustain,
//     Release,
// }

// pub struct Envelope {
//     pub attack_ms: f32,
//     pub decay_ms: f32,
//     pub sustain_level: f32,
//     pub release_ms: f32,

//     pub attack_curve: EnvelopeCurve,
//     pub decay_curve: EnvelopeCurve,
//     pub release_curve: EnvelopeCurve,

//     stage: EnvelopeStage,
//     current_value: f32,
//     time_in_stage: f32,
//     sample_rate: f32,
//     is_held: bool,
//     release_start_value: f32,
// }

// impl Envelope {
//     pub fn new(sample_rate: f32) -> Self {
//         Self {
//             attack_ms: 5.0,
//             decay_ms: 100.0,
//             sustain_level: 0.7,
//             release_ms: 300.0,
//             attack_curve: EnvelopeCurve::Linear,
//             decay_curve: EnvelopeCurve::Linear,
//             release_curve: EnvelopeCurve::Linear,
//             stage: EnvelopeStage::Idle,
//             current_value: 0.0,
//             time_in_stage: 0.0,
//             sample_rate,
//             is_held: false,
//             release_start_value: 0.0,
//         }
//     }

//     pub fn note_on(&mut self) {
//         self.is_held = true;
//         self.time_in_stage = 0.0;
//         self.stage = EnvelopeStage::Attack;
//     }

//     pub fn note_off(&mut self) {
//         self.is_held = false;
//         if self.stage != EnvelopeStage::Idle {
//             self.release_start_value = self.current_value;
//             self.time_in_stage = 0.0;
//             self.stage = EnvelopeStage::Release;
//         }
//     }

//     pub fn get_value(&self) -> f32 {
//         self.current_value
//     }

//     pub fn get_stage(&self) -> EnvelopeStage {
//         self.stage
//     }

//     pub fn is_active(&self) -> bool {
//         self.stage != EnvelopeStage::Idle
//     }

//     pub fn process(&mut self) {
//         let sample_duration_s = 1.0 / self.sample_rate;
//         self.time_in_stage += sample_duration_s;

//         match self.stage {
//             EnvelopeStage::Idle => {
//                 self.current_value = 0.0;
//             }

//             EnvelopeStage::Attack => {
//                 let attack_duration_s = self.attack_ms / 1000.0;
//                 if attack_duration_s <= 0.0 {
//                     self.current_value = 1.0;
//                     self.stage = EnvelopeStage::Decay;
//                     self.time_in_stage = 0.0;
//                     return;
//                 }

//                 let progress = (self.time_in_stage / attack_duration_s).min(1.0);
//                 let curved_progress = apply_curve(progress, self.attack_curve);

//                 self.current_value = curved_progress;

//                 if progress >= 1.0 {
//                     self.stage = EnvelopeStage::Decay;
//                     self.time_in_stage = 0.0;
//                 }
//             }

//             EnvelopeStage::Decay => {
//                 let decay_duration_s = self.decay_ms / 1000.0;
//                 if decay_duration_s <= 0.0 {
//                     self.current_value = self.sustain_level;
//                     self.stage = EnvelopeStage::Sustain;
//                     self.time_in_stage = 0.0;
//                     return;
//                 }

//                 let progress = (self.time_in_stage / decay_duration_s).min(1.0);
//                 let curved_progress = apply_curve(progress, self.decay_curve);

//                 let start_value = 1.0;
//                 let end_value = self.sustain_level;
//                 self.current_value = start_value + (end_value - start_value) * curved_progress;

//                 if progress >= 1.0 {
//                     self.stage = EnvelopeStage::Sustain;
//                     self.time_in_stage = 0.0;
//                 }
//             }

//             EnvelopeStage::Sustain => {
//                 self.current_value = self.sustain_level;
//                 if !self.is_held {
//                     self.release_start_value = self.current_value;
//                     self.stage = EnvelopeStage::Release;
//                     self.time_in_stage = 0.0;
//                 }
//             }

//             EnvelopeStage::Release => {
//                 let release_duration_s = self.release_ms / 1000.0;
//                 if release_duration_s <= 0.0 {
//                     self.current_value = 0.0;
//                     self.stage = EnvelopeStage::Idle;
//                     return;
//                 }

//                 let progress = (self.time_in_stage / release_duration_s).min(1.0);
//                 let curved_progress = apply_curve(progress, self.release_curve);

//                 let start_value = self.release_start_value;
//                 let end_value = 0.0;
//                 self.current_value = start_value + (end_value - start_value) * curved_progress;

//                 if progress >= 1.0 {
//                     self.stage = EnvelopeStage::Idle;
//                     self.current_value = 0.0;
//                 }
//             }
//         }
//     }
// }

// impl Modulator for Envelope {
//     fn tick(&mut self, _transport: &Transport) {
//         self.process();
//     }

//     fn value(&self) -> f32 {
//         self.get_value()
//     }

//     fn reset(&mut self) {
//         self.stage = EnvelopeStage::Idle;
//         self.current_value = 0.0;
//         self.time_in_stage = 0.0;
//         self.is_held = false;
//         self.release_start_value = 0.0;
//     }
// }
