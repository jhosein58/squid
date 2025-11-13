use crate::AudioNode;

pub trait Shaper: AudioNode {}

pub mod saw_shaper;
pub mod sine_shaper;
