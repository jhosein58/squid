use std::sync::Arc;

use squid_core::FixedSpscQueue;

#[derive(PartialEq, Copy, Clone, Debug)]
enum TriggerState {
    Searching,
    Collecting,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TriggerEdge {
    Rising,
    Falling,
}

pub struct OscilloscopeTrigger<const SIZE: usize> {
    queue: Arc<FixedSpscQueue<f32, SIZE>>,
    buffer: Vec<f32>,
    samples_needed: usize,
    samples_collected: usize,
    state: TriggerState,
    trigger_level: f32,
    trigger_edge: TriggerEdge,
    last_sample_value: f32,
}

impl<const SIZE: usize> OscilloscopeTrigger<SIZE> {
    pub fn new(
        queue: Arc<FixedSpscQueue<f32, SIZE>>,
        trigger_level: f32,
        trigger_edge: TriggerEdge,
    ) -> Self {
        Self {
            queue,
            buffer: vec![0.0; SIZE],
            samples_needed: SIZE,
            samples_collected: 0,
            state: TriggerState::Searching,
            trigger_level,
            trigger_edge,
            last_sample_value: 0.0,
        }
    }

    pub fn process_sample(&mut self, sample: f32) {
        match self.state {
            TriggerState::Searching => {
                let triggered = match self.trigger_edge {
                    TriggerEdge::Rising => {
                        self.last_sample_value < self.trigger_level && sample >= self.trigger_level
                    }
                    TriggerEdge::Falling => {
                        self.last_sample_value > self.trigger_level && sample <= self.trigger_level
                    }
                };

                if triggered {
                    self.state = TriggerState::Collecting;
                    self.samples_collected = 0;
                    self.buffer[self.samples_collected] = sample;
                    self.samples_collected += 1;
                }
            }
            TriggerState::Collecting => {
                if self.samples_collected < self.samples_needed {
                    self.buffer[self.samples_collected] = sample;
                    self.samples_collected += 1;
                }

                if self.samples_collected >= self.samples_needed {
                    self.queue.clear();
                    for &s in self.buffer.iter() {
                        let _ = self.queue.as_ref().push(s);
                    }

                    self.state = TriggerState::Searching;
                }
            }
        }
        self.last_sample_value = sample;
    }
}
