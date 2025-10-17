pub trait Processor {
    fn process(&mut self, input: f32) -> f32;
    fn reset(&mut self);
}

pub struct DelayProc {
    buffer: Vec<f32>,
    write_index: usize,
    delay_samples: usize,
    sample_rate: f32,
}

impl DelayProc {
    pub fn new(max_delay_ms: f32, delay_ms: f32, sample_rate: f32) -> Self {
        let max_buffer_size = (max_delay_ms / 1000.0 * sample_rate).ceil() as usize;

        let mut delay = DelayProc {
            buffer: vec![0.0; max_buffer_size],
            write_index: 0,
            delay_samples: 0,
            sample_rate,
        };

        delay.set_delay_ms(delay_ms);

        delay
    }

    pub fn set_delay_ms(&mut self, delay_ms: f32) {
        let new_delay_samples = (delay_ms / 1000.0 * self.sample_rate) as usize;

        self.delay_samples = new_delay_samples.min(self.buffer.len() - 1);
    }

    #[inline]
    fn get_read_index(&self) -> usize {
        (self.write_index + self.buffer.len() - self.delay_samples) % self.buffer.len()
    }
}

impl Processor for DelayProc {
    #[inline]
    fn process(&mut self, input: f32) -> f32 {
        let read_idx = self.get_read_index();
        let delayed_sample = self.buffer[read_idx];

        self.buffer[self.write_index] = input;

        self.write_index = (self.write_index + 1) % self.buffer.len();

        delayed_sample
    }

    fn reset(&mut self) {
        for sample in self.buffer.iter_mut() {
            *sample = 0.0;
        }

        self.write_index = 0;
    }
}
