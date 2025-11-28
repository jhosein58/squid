// use std::{array, simd::Simd};

// use crate::{
//     AudioNode,
//     modulators::{Modulator, envlopes::Envelope},
//     process_context::{FixedBuf, ProcessContext},
// };

// #[derive(Clone, Copy, PartialEq)]
// enum ArEnvState {
//     Idle,
//     Attack,
//     Sustain,
//     Release,
// }

// #[derive(Clone, Copy)]
// pub struct ArEnv {
//     attack_increment: f32,
//     release_increment: f32,
//     current_sample: f32,
//     state: ArEnvState,
// }

// impl ArEnv {
//     pub fn new(attack_sec: f32, release_sec: f32, sample_rate: f32) -> Self {
//         let attack_inc = if attack_sec > 0.001 {
//             1.0 / (attack_sec * sample_rate)
//         } else {
//             1.0
//         };
//         let release_inc = if release_sec > 0.001 {
//             1.0 / (release_sec * sample_rate)
//         } else {
//             1.0
//         };

//         Self {
//             attack_increment: attack_inc,
//             release_increment: release_inc,
//             current_sample: 0.0,
//             state: ArEnvState::Idle,
//         }
//     }

//     #[inline]
//     fn next_sample(&mut self) -> f32 {
//         match self.state {
//             ArEnvState::Idle => 0.0,
//             ArEnvState::Sustain => 1.0,
//             ArEnvState::Attack => {
//                 self.current_sample += self.attack_increment;
//                 if self.current_sample >= 1.0 {
//                     self.current_sample = 1.0;
//                     self.state = ArEnvState::Sustain;
//                 }
//                 self.current_sample
//             }
//             ArEnvState::Release => {
//                 self.current_sample -= self.release_increment;
//                 if self.current_sample <= 0.0 {
//                     self.current_sample = 0.0;
//                     self.state = ArEnvState::Idle;
//                 }
//                 self.current_sample
//             }
//         }
//     }

//     pub fn is_active(&self) -> bool {
//         self.state != ArEnvState::Idle
//     }
// }

// impl AudioNode for ArEnv {
//     fn process(&mut self, _: &ProcessContext, outputs: &mut [&mut FixedBuf]) {
//         match self.state {
//             ArEnvState::Idle => {
//                 outputs[0].fill(0.);
//                 return;
//             }
//             ArEnvState::Sustain => {
//                 outputs[0].fill(1.0);
//                 return;
//             }
//             _ => {}
//         }

//         outputs[0].map_in_place(|_| {
//             let chunk = array::from_fn(|_| self.next_sample());
//             Simd::from_array(chunk)
//         });
//     }

//     fn reset(&mut self, _: f32) {
//         self.current_sample = 0.0;
//         self.state = ArEnvState::Idle;
//     }
// }

// impl Modulator for ArEnv {}

// impl Envelope for ArEnv {
//     fn trigger(&mut self) {
//         self.state = ArEnvState::Attack;
//     }

//     fn release(&mut self) {
//         if self.state != ArEnvState::Idle {
//             self.state = ArEnvState::Release;
//         }
//     }
// }

use core::{array, simd::Simd};

use crate::{
    AudioNode,
    modulators::{Modulator, envlopes::Envelope},
    process_context::{FixedBuf, ProcessContext},
};

#[derive(Clone, Copy, PartialEq)]
pub enum ArEnvState {
    Idle,
    Attack,
    Sustain,
    Release,
}

#[derive(Clone, Copy)]
pub struct ArEnv {
    attack_increment: f32,
    release_increment: f32,
    current_sample: f32,
    state: ArEnvState,
}

impl ArEnv {
    pub fn new(attack_sec: f32, release_sec: f32, sample_rate: f32) -> Self {
        let attack_inc = if attack_sec > 0.001 {
            1.0 / (attack_sec * sample_rate)
        } else {
            1.0
        };
        let release_inc = if release_sec > 0.001 {
            1.0 / (release_sec * sample_rate)
        } else {
            1.0
        };

        Self {
            attack_increment: attack_inc,
            release_increment: release_inc,
            current_sample: 0.0,
            state: ArEnvState::Idle,
        }
    }
    #[inline]
    fn next_sample(&mut self) -> f32 {
        match self.state {
            ArEnvState::Idle => 0.0,

            ArEnvState::Sustain => {
                self.state = ArEnvState::Release;
                1.0
            }

            ArEnvState::Attack => {
                self.current_sample += self.attack_increment;
                if self.current_sample >= 1.0 {
                    self.current_sample = 1.0;
                    self.state = ArEnvState::Release;
                }
                self.current_sample
            }

            ArEnvState::Release => {
                self.current_sample -= self.release_increment;
                if self.current_sample <= 0.0 {
                    self.current_sample = 0.0;
                    self.state = ArEnvState::Idle;
                }
                self.current_sample
            }
        }
    }

    pub fn is_active(&self) -> bool {
        self.state != ArEnvState::Idle
    }
}

impl AudioNode for ArEnv {
    fn process(&mut self, _: &ProcessContext, outputs: &mut [&mut FixedBuf]) {
        match self.state {
            ArEnvState::Idle => {
                outputs[0].fill(0.);
                return;
            }
            ArEnvState::Sustain => {
                outputs[0].fill(1.0);
                return;
            }
            _ => {}
        }

        outputs[0].map_in_place(|_| {
            let chunk = array::from_fn(|_| self.next_sample());
            Simd::from_array(chunk)
        });
    }

    fn reset(&mut self, _: f32) {
        self.current_sample = 0.0;
        self.state = ArEnvState::Idle;
    }
}

impl Modulator for ArEnv {}

impl Envelope for ArEnv {
    fn trigger(&mut self) {
        self.state = ArEnvState::Attack;
    }

    fn release(&mut self) {
        if self.state != ArEnvState::Idle {
            self.state = ArEnvState::Release;
        }
    }
}
