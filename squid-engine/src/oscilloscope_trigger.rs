use squid_core::FixedSpscQueue;
use std::sync::Arc;

#[derive(PartialEq, Copy, Clone, Debug)]
enum TriggerState {
    Armed,
    Triggered,
    Holdoff,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TriggerEdge {
    Rising,
    Falling,
}
pub struct OscilloscopeTrigger<const SIZE: usize> {
    queue: Arc<FixedSpscQueue<f32, SIZE>>,
    history_buffer: Vec<f32>,
    write_pos: usize,
    state: TriggerState,
    trigger_level: f32,
    trigger_edge: TriggerEdge,
    hysteresis: f32,
    pre_trigger_samples: usize,
    post_trigger_samples: usize,
    holdoff_samples: usize,
    samples_to_collect: usize,
    holdoff_counter: usize,
    last_sample_value: f32,
    // phase tracking fields:
    freq_estimate: f32,
    sample_rate: f32,
    phase_accum: f32,
    phase_window: f32,
}

impl<const SIZE: usize> OscilloscopeTrigger<SIZE> {
    pub fn new(
        queue: Arc<FixedSpscQueue<f32, SIZE>>,
        trigger_level: f32,
        trigger_edge: TriggerEdge,
        pre_trigger_percent: f32,
        holdoff_ms: f32,
        sample_rate: f32,
    ) -> Self {
        let history_size = SIZE * 2;
        let pre_trigger_samples = (SIZE as f32 * pre_trigger_percent.clamp(0.0, 1.0)) as usize;
        let post_trigger_samples = SIZE - pre_trigger_samples;
        let holdoff_samples = (holdoff_ms / 1000.0 * sample_rate).max(1.0) as usize;

        Self {
            queue,
            history_buffer: vec![0.0; history_size],
            write_pos: 0,
            state: TriggerState::Armed,
            trigger_level,
            trigger_edge,
            hysteresis: 0.01,
            pre_trigger_samples,
            post_trigger_samples,
            holdoff_samples,
            samples_to_collect: 0,
            holdoff_counter: 0,
            last_sample_value: 0.0,
            freq_estimate: 440.0,
            sample_rate,
            phase_accum: 0.0,
            phase_window: 0.03,
        }
    }

    pub fn set_frequency(&mut self, freq: f32) {
        self.freq_estimate = freq;
    }

    pub fn process_sample(&mut self, sample: f32) {
        self.history_buffer[self.write_pos] = sample;
        self.write_pos = (self.write_pos + 1) % self.history_buffer.len();

        self.phase_accum += self.freq_estimate / self.sample_rate;
        if self.phase_accum >= 1.0 {
            self.phase_accum -= 1.0;
        }

        match self.state {
            TriggerState::Armed => {
                let edge_hit = match self.trigger_edge {
                    TriggerEdge::Rising => {
                        self.last_sample_value < (self.trigger_level - self.hysteresis)
                            && sample >= (self.trigger_level + self.hysteresis)
                    }
                    TriggerEdge::Falling => {
                        self.last_sample_value > (self.trigger_level + self.hysteresis)
                            && sample <= (self.trigger_level - self.hysteresis)
                    }
                };

                if edge_hit && (self.phase_accum < self.phase_window) {
                    self.state = TriggerState::Triggered;
                    self.samples_to_collect = self.post_trigger_samples.saturating_sub(1);
                }
            }

            TriggerState::Triggered => {
                if self.samples_to_collect > 0 {
                    self.samples_to_collect -= 1;
                } else {
                    self.capture_and_send_frame();
                    self.state = TriggerState::Holdoff;
                    self.holdoff_counter = self.holdoff_samples;
                }
            }

            TriggerState::Holdoff => {
                if self.holdoff_counter > 0 {
                    self.holdoff_counter -= 1;
                } else {
                    self.state = TriggerState::Armed;
                }
            }
        }

        self.last_sample_value = sample;
    }

    fn capture_and_send_frame(&self) {
        let mut frame_buffer = [0.0f32; SIZE];
        let history_len = self.history_buffer.len();

        let start_index = (self.write_pos + history_len - self.post_trigger_samples) % history_len;

        for i in 0..SIZE {
            let history_index = (start_index + i) % history_len;
            frame_buffer[i] = self.history_buffer[history_index];
        }

        self.queue.clear();
        for &s in frame_buffer.iter() {
            let _ = self.queue.as_ref().push(s);
        }
    }
}
