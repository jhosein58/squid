use libm::{powf, roundf};

use crate::Processor;

pub struct BitCrusherProc {
    bit_depth: f32,
    downsampling: u32,
    step: f32,
    counter: u32,
    last_sample: f32,
}

impl BitCrusherProc {
    pub fn new(bit_depth: f32, downsampling: u32) -> Self {
        let mut crusher = Self {
            bit_depth: 1.0,
            downsampling: 1,
            step: 0.0,
            counter: 0,
            last_sample: 0.0,
        };
        crusher.set_bit_depth(bit_depth);
        crusher.set_downsampling(downsampling);
        crusher
    }

    pub fn set_bit_depth(&mut self, bit_depth: f32) {
        self.bit_depth = bit_depth.max(1.0).min(32.0);
        self.step = 2.0 / powf(2.0_f32, self.bit_depth);
    }

    pub fn set_downsampling(&mut self, downsampling: u32) {
        self.downsampling = downsampling.max(1);
    }
}

impl Processor for BitCrusherProc {
    fn process(&mut self, input: f32) -> f32 {
        if self.counter < self.downsampling {
            self.counter += 1;
            return self.last_sample;
        }
        self.counter = 1;

        let crushed_sample = self.step * roundf(input / self.step);

        self.last_sample = crushed_sample;
        self.last_sample
    }

    fn reset(&mut self) {
        self.counter = 0;
        self.last_sample = 0.0;
    }
}
