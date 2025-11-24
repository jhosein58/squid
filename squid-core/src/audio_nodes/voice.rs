use std::simd::Simd;

use crate::{
    AudioNode, Note, SILENCE_DB,
    effects::gain_fx::GainFx,
    modulators::envlopes::{Envelope, ar_env::ArEnv},
    oscillators::Oscillator,
    process_context::{FixedBuf, ProcessContext},
};

#[derive(Clone, Copy)]
pub struct Voice<T: Oscillator> {
    osc: T,
    env: ArEnv,
    gain: GainFx,
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
            gain: GainFx,
            active: false,
            sample_rate: 0.,
            freq: 0.,
            note: 0,
        }
    }

    pub fn is_idle(&self) -> bool {
        !self.active // && !self.env.is_active()
    }

    pub fn is_playing(&self, note: u8) -> bool {
        self.active && self.note == note // && self.env.is_active()
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
        if self.active {
            //|| self.env.is_active() {
            // let mut env_mod_signal = FixedBuf::default();
            // self.env.process(ctx, &mut [&mut env_mod_signal]);

            // let v_silence = Simd::splat(SILENCE_DB);
            // let v_one = Simd::splat(1.0);

            // env_mod_signal.map_in_place(|c| v_silence * (v_one - c));

            // let mut lc = FixedBuf::default();
            // let mut lr = FixedBuf::default();
            // self.osc.process(ctx, &mut [&mut lc, &mut lr]);

            // let gain_ctx = ProcessContext {
            //     sample_rate: self.sample_rate,
            //     events: &[],
            //     inputs: &[&lc, &lr, &env_mod_signal.into()],
            // };
            // self.gain.process(&gain_ctx, outputs);
            self.osc.process(ctx, outputs);
        }
    }

    fn reset(&mut self, sample_rate: f32) {
        self.osc.reset(sample_rate);
    }
}
