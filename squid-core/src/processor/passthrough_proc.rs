use crate::processor::Processor;

#[derive(Debug, Default, Clone, Copy)]
pub struct Passthrough;

impl Passthrough {
    pub fn new() -> Self {
        Self
    }
}

impl Processor for Passthrough {
    #[inline]
    fn process(&mut self, input: f32) -> f32 {
        input
    }

    #[inline]
    fn reset(&mut self) {}
}
