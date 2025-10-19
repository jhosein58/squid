use crate::Processor;

#[inline]
fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}
pub struct HardClipProc {
    threshold: f32,
}

impl HardClipProc {
    pub fn new(threshold: f32) -> Self {
        Self {
            threshold: threshold.abs().min(1.0),
        }
    }
}

impl Processor for HardClipProc {
    fn process(&mut self, input: f32) -> f32 {
        clamp(input, -self.threshold, self.threshold)
    }

    fn reset(&mut self) {}
}
