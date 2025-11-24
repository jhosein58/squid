use std::{array, simd::Simd};

use crate::{
    AudioNode, FloatVector,
    modulators::{Modulator, envlopes::Envelope},
    process_context::{FixedBuf, ProcessContext},
};

#[derive(Clone, Copy)]
enum ArEnvState {
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
        Self {
            attack_increment: 1.0 / (attack_sec * sample_rate),
            release_increment: 1.0 / (release_sec * sample_rate),
            current_sample: 0.0,
            state: ArEnvState::Idle,
        }
    }

    pub fn get_value(&mut self) -> f32 {
        match self.state {
            ArEnvState::Idle => {
                self.current_sample = 0.0;
            }
            ArEnvState::Attack => {
                self.current_sample += self.attack_increment;
                if self.current_sample >= 1.0 {
                    self.current_sample = 1.0;
                    self.state = ArEnvState::Sustain;
                }
            }
            ArEnvState::Sustain => {
                self.current_sample = 1.0;
            }
            ArEnvState::Release => {
                self.current_sample -= self.release_increment;
                if self.current_sample <= 0.0 {
                    self.current_sample = 0.0;
                    self.state = ArEnvState::Idle;
                }
            }
        }
        self.current_sample
    }

    pub fn is_active(&self) -> bool {
        match self.state {
            ArEnvState::Idle => false,
            _ => true,
        }
    }
}

impl AudioNode for ArEnv {
    fn process(&mut self, _: &ProcessContext, outputs: &mut [&mut FixedBuf]) {
        outputs[0].map_in_place(|_| {
            let b: [f32; FloatVector::LANES] = array::from_fn(|_| self.get_value());
            Simd::from_array(b)
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
        self.state = ArEnvState::Release;
    }
}
