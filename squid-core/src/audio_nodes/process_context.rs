use std::ops::{Deref, DerefMut};

use crate::{Event, FloatVector};
#[derive(Clone)]
pub struct FixedBuf {
    pub data: FloatVector,
}

impl Default for FixedBuf {
    fn default() -> Self {
        Self {
            data: FloatVector::default(),
        }
    }
}

impl From<FloatVector> for FixedBuf {
    fn from(data: FloatVector) -> Self {
        Self { data }
    }
}

impl Deref for FixedBuf {
    type Target = FloatVector;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for FixedBuf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
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
