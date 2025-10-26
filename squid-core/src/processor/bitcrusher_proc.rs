use libm::{floor, floorf, powf};

use crate::Processor;

pub struct BitCrusherProc {
    bit_depth: f32,
    downsampling_factor: u32,
    quantization_steps: f32,
    sample_counter: u32,
    last_output_sample: f32,
}

impl BitCrusherProc {
    pub fn new(initial_bit_depth: f32, initial_downsampling: u32) -> Self {
        let mut crusher = Self {
            bit_depth: 16.0,
            downsampling_factor: 1,
            quantization_steps: 0.0,
            sample_counter: 0,
            last_output_sample: 0.0,
        };
        crusher.set_bit_depth(initial_bit_depth);
        crusher.set_downsampling(initial_downsampling);
        crusher
    }

    pub fn set_bit_depth(&mut self, bit_depth: f32) -> &mut Self {
        let clamped_bits = bit_depth.max(1.0).min(24.0);
        self.bit_depth = clamped_bits;
        self.quantization_steps = powf(2.0, self.bit_depth) - 1.0;
        self
    }

    pub fn set_downsampling(&mut self, factor: u32) -> &mut Self {
        self.downsampling_factor = factor.max(1);
        self
    }
}

impl Processor for BitCrusherProc {
    fn process(&mut self, input: f32) -> f32 {
        self.sample_counter += 1;

        if self.sample_counter >= self.downsampling_factor {
            self.sample_counter = 0;

            let normalized_input = (input + 1.0) * 0.5;

            let stepped_sample = floorf(normalized_input * self.quantization_steps);

            let quantized_normalized = stepped_sample / self.quantization_steps;

            let final_sample = (quantized_normalized * 2.0) - 1.0;

            self.last_output_sample = final_sample;
        }

        self.last_output_sample
    }

    fn reset(&mut self) {
        self.sample_counter = 0;
        self.last_output_sample = 0.0;
    }
}
