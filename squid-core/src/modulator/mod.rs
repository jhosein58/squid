use crate::timing::Transport;

pub mod env_adsr_mod;
pub mod lfo_func_mod;
pub mod lfo_mod;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModRate {
    Hz(f32),
    Beat(f32),
}

pub trait Modulator {
    fn tick(&mut self, transport: &Transport);
    fn value(&self) -> f32;
    fn reset(&mut self);
}

pub trait OneShotMod: Modulator {
    fn trigger(&mut self);
}

pub trait CyclicMod: Modulator {
    fn set_rate(&mut self, rate: ModRate);
}

pub trait EnvelopeMod: Modulator {
    fn release(&mut self);
    fn is_active(&self) -> bool;
}
