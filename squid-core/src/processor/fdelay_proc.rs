use crate::processor::Processor;

pub struct FeedbackDelayProc<const SIZE: usize> {
    buffer: [f32; SIZE],
    write_index: usize,
    delay_samples: usize,
    sample_rate: f32,
    feedback: f32,
    wet_level: f32,
    dry_level: f32,
}

impl<const SIZE: usize> FeedbackDelayProc<SIZE> {
    pub fn new(sample_rate: f32) -> Self {
        assert!(
            SIZE > 0,
            "Buffer size for FeedbackDelayProc cannot be zero."
        );

        FeedbackDelayProc {
            buffer: [0.0; SIZE],
            write_index: 0,
            delay_samples: 0,
            sample_rate,
            feedback: 0.0,
            wet_level: 0.5,
            dry_level: 0.5,
        }
    }

    pub fn set_delay_ms(&mut self, delay_ms: f32) {
        let new_delay_samples = (delay_ms / 1000.0 * self.sample_rate) as usize;
        self.delay_samples = new_delay_samples.min(SIZE - 1);
    }

    pub fn set_feedback(&mut self, feedback_gain: f32) {
        self.feedback = feedback_gain.clamp(0.0, 0.9999);
    }

    pub fn set_mix(&mut self, mix: f32) {
        let m = mix.clamp(0.0, 1.0);
        self.wet_level = m;
        self.dry_level = 1.0 - m;
    }

    #[inline]
    fn get_read_index(&self) -> usize {
        (self.write_index + SIZE - self.delay_samples) % SIZE
    }
}

impl<const SIZE: usize> Processor for FeedbackDelayProc<SIZE> {
    #[inline]
    fn process(&mut self, input: f32) -> f32 {
        let read_idx = self.get_read_index();
        let delayed_sample = self.buffer[read_idx];

        let feedback_signal = input + delayed_sample * self.feedback;

        self.buffer[self.write_index] = feedback_signal;

        self.write_index = (self.write_index + 1) % SIZE;

        let output = (input * self.dry_level) + (delayed_sample * self.wet_level);

        output
    }

    fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_index = 0;
    }
}
