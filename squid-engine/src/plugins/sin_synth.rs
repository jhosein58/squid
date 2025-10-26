use std::array;

use squid_core::{Event, EventData, Note, Oscillator, Plugin, primetives::SinOsc};

use crate::Voice;

pub struct SinSynth {
    voices: [Voice; 16],
}

#[allow(non_snake_case)]
pub fn SinSynth(sample_rate: u32) -> SinSynth {
    SinSynth::new(sample_rate)
}

impl SinSynth {
    pub fn new(sample_rate: u32) -> Self {
        SinSynth {
            voices: array::from_fn(|_| Voice::new(Box::new(SinOsc(sample_rate)))),
        }
    }

    fn note_on(&mut self, note: u8) {
        for v in self.voices.iter() {
            if v.note() == note && v.is_playing() {
                return;
            }
        }
        for v in self.voices.iter_mut() {
            if v.is_free() {
                v.activate(note);
                break;
            }
        }
    }

    fn note_off(&mut self, note: u8) {
        for v in self.voices.iter_mut() {
            if v.note() == note {
                v.deactivate();
                break;
            }
        }
    }

    fn process_events(&mut self, events: &[Event]) {
        for event in events {
            match event.data {
                EventData::NoteOn { note, velocity } => self.note_on(note),
                EventData::NoteOff { note } => self.note_off(note),
                _ => {}
            }
        }
    }

    fn process_next_sample(&mut self) -> f32 {
        let mut sample = 0.0;
        let mut divider = 0.0;
        for v in self.voices.iter_mut() {
            if v.is_playing() {
                sample += v.next_sample();
                divider += 1.0;
            }
        }
        sample / divider
    }
}

impl Plugin for SinSynth {
    fn channels(&self) -> u8 {
        2
    }
    fn process(&mut self, _: &[&[f32]], output: &mut [&mut [f32]], events: &[Event]) {
        self.process_events(events);
        let output_buf = &mut output[0];
        for i in 0..output_buf.len() / 2 {
            let sample = self.process_next_sample();
            output_buf[i * 2] = sample;
            output_buf[i * 2 + 1] = sample;
        }
    }
}
