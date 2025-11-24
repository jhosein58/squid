use crate::AudioNode;

pub trait Processor: AudioNode {}

pub mod gain_proc;
