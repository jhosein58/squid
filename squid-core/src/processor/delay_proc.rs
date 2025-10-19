use crate::processor::Processor;

pub struct DelayProc<const SIZE: usize> {
    buffer: [f32; SIZE],
    write_index: usize,
    delay_samples: usize,
    sample_rate: f32,
}

impl<const SIZE: usize> DelayProc<SIZE> {
    pub fn new(delay_ms: f32, sample_rate: f32) -> Self {
        let mut delay = DelayProc {
            buffer: [0.0; SIZE],
            write_index: 0,
            delay_samples: 0,
            sample_rate,
        };
        delay.set_delay_ms(delay_ms);
        delay
    }

    pub fn set_delay_ms(&mut self, delay_ms: f32) {
        let new_delay_samples = (delay_ms / 1000.0 * self.sample_rate) as usize;
        self.delay_samples = new_delay_samples.min(SIZE - 1);
    }

    #[inline]
    fn get_read_index(&self) -> usize {
        (self.write_index + SIZE - self.delay_samples) % SIZE
    }
}

impl<const SIZE: usize> Processor for DelayProc<SIZE> {
    #[inline]
    fn process(&mut self, input: f32) -> f32 {
        let read_idx = self.get_read_index();
        let delayed_sample = self.buffer[read_idx];

        self.buffer[self.write_index] = input;

        self.write_index = (self.write_index + 1) % SIZE;

        delayed_sample
    }

    fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_index = 0;
    }
}
