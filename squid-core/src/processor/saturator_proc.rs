use libm::tanhf;

use crate::Processor;

pub struct SaturatorProc {
    drive: f32,
}

impl SaturatorProc {
    pub fn new(drive: f32) -> Self {
        Self {
            drive: drive.max(1.0),
        }
    }
}

impl Processor for SaturatorProc {
    fn process(&mut self, input: f32) -> f32 {
        tanhf(input * self.drive)
    }

    fn reset(&mut self) {}
}
