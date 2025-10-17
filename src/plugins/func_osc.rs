use crate::{buffer::AudioBufferML, plugins::Plugin};

type ShapingFunction = Box<dyn Fn(f32) -> f32 + Send + Sync>;

pub struct FunctionOscillator {
    pub frequency: f32,
    phase: f32,
    sample_rate: f32,
    phase_increment: f32,
    shaping_function: ShapingFunction,
}

impl FunctionOscillator {
    pub fn new(frequency: f32, shaping_function: ShapingFunction) -> Self {
        Self {
            frequency,
            shaping_function,
            phase: 0.0,
            sample_rate: 0.0,
            phase_increment: 0.0,
        }
    }
    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq.max(0.0);
        if self.sample_rate > 0.0 {
            self.update_phase_increment();
        }
    }
    fn update_phase_increment(&mut self) {
        self.phase_increment = self.frequency / self.sample_rate;
    }
}
impl Plugin for FunctionOscillator {
    fn name(&self) -> &str {
        "Function Oscillator"
    }

    fn num_inputs(&self) -> usize {
        0
    }

    fn num_outputs(&self) -> usize {
        1
    }
    fn prepare(&mut self, sample_rate: u32, _max_block_size: usize) {
        self.sample_rate = sample_rate as f32;
        self.update_phase_increment();
    }

    fn next_sample(&mut self) -> f32 {
        let value = (self.shaping_function)(self.phase);

        self.phase += self.phase_increment;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        value
    }

    fn process(&mut self, inputs: &[&AudioBufferML], outputs: &mut [&mut AudioBufferML]) {
        assert_eq!(inputs.len(), self.num_inputs());
        assert_eq!(outputs.len(), self.num_outputs());

        let output_buffer = &mut outputs[0];
        let samples = &mut output_buffer.samples;
        let num_channels = output_buffer.num_channels as usize;
        let num_frames = samples.len() / num_channels;

        for frame_index in 0..num_frames {
            let value = self.next_sample();

            for channel_index in 0..num_channels {
                let sample_index = frame_index * num_channels + channel_index;
                samples[sample_index] = value;
            }
        }
    }
}
