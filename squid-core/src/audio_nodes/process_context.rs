use crate::{Event, MAX_BLOCK_SIZE};

pub struct FixedBuf {
    pub data: [f32; MAX_BLOCK_SIZE],
}

impl Default for FixedBuf {
    fn default() -> Self {
        Self {
            data: [0.; MAX_BLOCK_SIZE],
        }
    }
}

pub struct ProcessContext<'a> {
    pub sample_rate: f32,
    pub events: &'a [Event],
    pub inputs: &'a [&'a FixedBuf],
}

impl<'a> ProcessContext<'a> {
    pub fn new(sample_rate: f32, events: &'a [Event], inputs: &'a [&'a FixedBuf]) -> Self {
        Self {
            sample_rate,
            events,
            inputs,
        }
    }
}
impl<'a> Default for ProcessContext<'a> {
    fn default() -> Self {
        Self::new(44100., &[], &[])
    }
}
