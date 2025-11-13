// use std::array;

// use squid_core::{Event, EventData, Note, Oscillator, Plugin, primetives::SinOsc};

// use crate::Voice;

// pub struct UnisonSynth {
//     voices: [(bool, u8, UnisonOsc); 16],
// }

// impl UnisonSynth {
//     pub fn new(constructor: fn(u32) -> Box<dyn Oscillator>) -> Self {
//         let mut i = Self {
//             voices: array::from_fn(|_| (false, 0, UnisonOsc::new(constructor, 10))),
//         };
//         for v in i.voices.iter_mut() {
//             v.2.apply_distribution_factor(0.5);
//         }
//         i
//     }

//     fn note_on(&mut self, note: u8) {
//         for v in self.voices.iter() {
//             if v.1 == note && v.0 {
//                 return;
//             }
//         }
//         for v in self.voices.iter_mut() {
//             if !v.0 {
//                 v.0 = true;
//                 v.1 = note;

//                 v.2.set_frequency(Note::from_midi(note).to_frequency().into());
//                 break;
//             }
//         }
//     }

//     fn note_off(&mut self, note: u8) {
//         for v in self.voices.iter_mut() {
//             if v.1 == note {
//                 v.0 = false;
//                 break;
//             }
//         }
//     }

//     fn process_events(&mut self, events: &[Event]) {
//         for event in events {
//             match event.data {
//                 EventData::NoteOn { note, velocity } => self.note_on(note),
//                 EventData::NoteOff { note } => self.note_off(note),
//                 _ => {}
//             }
//         }
//     }
// }

// impl Plugin for UnisonSynth {
//     fn channels(&self) -> u8 {
//         2
//     }
//     fn process(&mut self, _: &[&[f32]], output: &mut [&mut [f32]], events: &[Event]) {
//         self.process_events(events);
//         let output_buf = &mut output[0];
//         for (_, _, osc) in self.voices.iter_mut() {
//             osc.process(&[], output, &[]);
//         }
//     }
// }
