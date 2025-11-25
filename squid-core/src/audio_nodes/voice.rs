use std::simd::Simd;

use crate::{
    AudioNode, Note, VOICE_GAIN,
    modulators::envlopes::{Envelope, ar_env::ArEnv},
    oscillators::Oscillator,
    process_context::{FixedBuf, ProcessContext},
};

#[derive(Clone, Copy)]
pub struct Voice<T: Oscillator> {
    osc: T,
    env: ArEnv,
    active: bool,
    sample_rate: f32,
    freq: f32,
    note: u8,
}

impl<T> Voice<T>
where
    T: Oscillator,
{
    pub fn new(osc: T, env: ArEnv) -> Self {
        Self {
            osc,
            env,
            active: false,
            sample_rate: 0.,
            freq: 0.,
            note: 0,
        }
    }

    pub fn is_idle(&self) -> bool {
        !self.active //&& !self.env.is_active()
    }

    pub fn is_playing(&self, note: u8) -> bool {
        self.active && self.note == note
    }

    pub fn note_on(&mut self, note: u8, sample_rate: f32) {
        self.note = note;
        self.freq = Note::from_midi(note).to_frequency().into();
        self.sample_rate = sample_rate;
        self.active = true;

        self.osc.configure(self.freq, self.sample_rate, None);

        self.env.trigger();
    }

    pub fn note_off(&mut self) {
        self.active = false;
        self.env.release();
    }
}

impl<T> AudioNode for Voice<T>
where
    T: Oscillator,
{
    fn process(&mut self, ctx: &ProcessContext, outputs: &mut [&mut FixedBuf]) {
        if self.is_idle() {
            return;
        }

        // let mut env_vals = FixedBuf::default();
        // self.env.process(ctx, &mut [&mut env_vals]);

        // self.osc.process(ctx, outputs);

        // let (left_out, right_out) = outputs.split_at_mut(1);

        // left_out[0].zip_map_in_place(&env_vals, |audio_sample, env_val| audio_sample * env_val);

        // if let Some(right) = right_out.first_mut() {
        //     right.zip_map_in_place(&env_vals, |audio_sample, env_val| audio_sample * env_val);
        // }

        self.osc.process(ctx, outputs);
        let g = Simd::splat(VOICE_GAIN);
        outputs[0].data.map_in_place(|c| c * g);
        outputs[1].data.map_in_place(|c| c * g);
    }

    fn reset(&mut self, sample_rate: f32) {
        self.osc.reset(sample_rate);
        self.env.reset(sample_rate);
        self.active = false;
    }
}
