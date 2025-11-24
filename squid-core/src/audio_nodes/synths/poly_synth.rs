use core::array;

use crate::{
    AudioNode, Event, EventData,
    modulators::envlopes::ar_env::ArEnv,
    oscillators::Oscillator,
    process_context::{FixedBuf, ProcessContext},
    voice::Voice,
};

const VOICE_COUNT: usize = 16;

pub struct PolySynth<T: Oscillator> {
    voices: [Voice<T>; VOICE_COUNT],
}

impl<T: Oscillator> PolySynth<T> {
    pub fn new(osc: T, env: ArEnv) -> Self {
        Self {
            voices: array::from_fn(|_| Voice::new(osc.clone(), env)),
        }
    }

    fn process_events(&mut self, events: &[Event]) {
        for event in events {
            match event.data {
                EventData::NoteOn { note, velocity } => {
                    if let Some(voice) = self.voices.iter_mut().find(|v| v.is_idle()) {
                        voice.note_on(note, 44100.);
                    }
                }
                EventData::NoteOff { note } => {
                    if let Some(voice) = self.voices.iter_mut().find(|v| v.is_playing(note)) {
                        voice.note_off();
                    }
                }
                _ => {}
            }
        }
    }
}

impl<T: Oscillator> AudioNode for PolySynth<T> {
    fn process(&mut self, ctx: &ProcessContext, outputs: &mut [&mut FixedBuf]) {
        self.process_events(ctx.events);

        let dummy_out = &mut [&mut FixedBuf::default(), &mut FixedBuf::default()];
        let sum_out = &mut [&mut FixedBuf::default(), &mut FixedBuf::default()];
        let mut div = 0;

        for voice in &mut self.voices {
            dummy_out[0].data.fill(0.0);
            dummy_out[1].data.fill(0.0);
            voice.process(ctx, dummy_out);

            for i in 0..dummy_out[0].data.len() {
                sum_out[0].data[i] += dummy_out[0].data[i];
                sum_out[1].data[i] += dummy_out[1].data[i];
            }

            if !voice.is_idle() {
                div += 1;
            }
        }

        for i in 0..sum_out[0].data.len() {
            sum_out[0].data[i] /= div as f32;
            sum_out[1].data[i] /= div as f32;
        }

        for i in 0..sum_out[0].data.len() {
            outputs[0].data[i] = sum_out[0].data[i];
            outputs[1].data[i] = sum_out[1].data[i];
        }
    }

    fn reset(&mut self, sample_rate: f32) {
        for voice in &mut self.voices {
            voice.reset(sample_rate);
        }
    }
}
