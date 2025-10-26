use std::sync::{Arc, Mutex};
use std::vec::Vec;

use squid_core::{
    ChannelledEventSequence, DEFAULT_CONFIG, Event, EventData, Oscillator, Processor, Transport,
    mixing::Mixing,
};

pub fn midi_to_hz(note: u8) -> f32 {
    440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0)
}

pub struct Mixer<const CHANNELS: usize, const CAPACITY: usize> {
    transport: Transport,
    generators: Vec<Arc<Mutex<dyn Oscillator + Send>>>,
    fx_rack: Vec<Vec<Arc<Mutex<dyn Processor + Send>>>>,
    sequencer: ChannelledEventSequence<CHANNELS, CAPACITY>,
    raw_samples_buffer: Vec<f32>,
}

impl<const CHANNELS: usize, const CAPACITY: usize> Mixer<CHANNELS, CAPACITY> {
    pub fn new() -> Self {
        let fx_rack = (0..CHANNELS).map(|_| Vec::new()).collect();

        Self {
            transport: Transport::new(DEFAULT_CONFIG.sample_rate as f32, 120.0),
            generators: Vec::with_capacity(CHANNELS),
            fx_rack,
            sequencer: ChannelledEventSequence::<CHANNELS, CAPACITY>::new(),
            raw_samples_buffer: vec![0.0; CHANNELS],
        }
    }

    pub fn add_generator(&mut self, generator: Arc<Mutex<dyn Oscillator + Send>>) {
        if self.generators.len() < CHANNELS {
            self.generators.push(generator);
            self.raw_samples_buffer.resize(self.generators.len(), 0.0);
        }
    }

    pub fn add_fx(&mut self, channel: usize, fx: Arc<Mutex<dyn Processor + Send>>) {
        if let Some(chain) = self.fx_rack.get_mut(channel) {
            chain.push(fx);
        }
    }

    pub fn get_generator(&self, channel_idx: usize) -> Option<Arc<Mutex<dyn Oscillator + Send>>> {
        self.generators.get(channel_idx).cloned()
    }

    pub fn get_fx(
        &self,
        channel_idx: usize,
        fx_idx: usize,
    ) -> Option<Arc<Mutex<dyn Processor + Send>>> {
        self.fx_rack
            .get(channel_idx)
            .and_then(|chain| chain.get(fx_idx))
            .cloned()
    }

    pub fn get_sequencer_mut(&mut self) -> &mut ChannelledEventSequence<CHANNELS, CAPACITY> {
        &mut self.sequencer
    }

    fn process_events(&mut self) {
        let current_time = self.transport.get_position().current_sample as u32;

        for channel_idx in 0..CHANNELS {
            while let Some(event) = self.sequencer.peek_event(channel_idx) {
                if event.timing == 0 {
                    let event = self.sequencer.pop_event(channel_idx).unwrap();
                    self.apply_event(channel_idx, event);
                } else {
                    break;
                }
            }
        }

        if self.transport.is_playing() {
            for channel_idx in 0..CHANNELS {
                while let Some(event) = self.sequencer.peek_event(channel_idx) {
                    if event.timing <= current_time {
                        let event = self.sequencer.pop_event(channel_idx).unwrap();
                        self.apply_event(channel_idx, event);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn apply_event(&mut self, channel_idx: usize, event: Event) {
        if let Some(generator_arc) = self.generators.get(channel_idx) {
            let mut generator = generator_arc.lock().unwrap();
            match event.data {
                EventData::NoteOn { note, velocity: _ } => {
                    generator.set_frequency(midi_to_hz(note));
                }
                EventData::NoteOff { .. } => {
                    generator.set_frequency(0.0);
                }
                _ => {}
            }
        }
    }

    pub fn render_next_block(&mut self, output_buffer: &mut [f32]) {
        let frame_count = output_buffer.len() / 2;

        for frame in 0..frame_count {
            self.transport.tick();
            self.process_events();

            for (i, generator_arc) in self.generators.iter().enumerate() {
                self.raw_samples_buffer[i] = generator_arc.lock().unwrap().next_sample();
            }

            let mut processed_samples: Vec<f32> = Vec::with_capacity(self.generators.len());
            for (i, raw_sample) in self.raw_samples_buffer.iter().enumerate() {
                let mut output = *raw_sample;
                if let Some(fx_chain) = self.fx_rack.get(i) {
                    for fx_arc in fx_chain.iter() {
                        output = fx_arc.lock().unwrap().process(output);
                    }
                }
                processed_samples.push(output);
            }

            let mixed_sample = Mixing::average_all(&processed_samples);

            let out_idx = frame * 2;
            output_buffer[out_idx] = mixed_sample;
            output_buffer[out_idx + 1] = mixed_sample;
        }
    }
}
