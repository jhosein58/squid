use crate::modulators::Modulator;

pub trait Envelope: Modulator {
    fn trigger(&mut self);
    fn release(&mut self);
}

pub mod ar_env;
