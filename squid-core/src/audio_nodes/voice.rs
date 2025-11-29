use core::simd::{Mask, Simd};

use crate::{
    AudioNode, FloatVector, Note, SIMD_LANES, VOICE_GAIN,
    dsp::mod_core::adsr_mod_source::AdsrModSource,
    modulators::envlopes::{Envelope, ar_env::ArEnv},
    oscillators::Oscillator,
    process_context::{FixedBuf, ProcessContext},
};

#[derive(Clone, Copy)]
pub struct Voice<T: Oscillator> {
    osc: T,
    env: AdsrModSource<{ SIMD_LANES }>,
    active: bool,
    sample_rate: f32,
    freq: f32,
    note: u8,
}

impl<T> Voice<T>
where
    T: Oscillator,
{
    pub fn new(osc: T, env: AdsrModSource<{ SIMD_LANES }>) -> Self {
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
        !self.active && self.env.is_idle()
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

        self.env.note_on(Mask::splat(true));
    }

    pub fn note_off(&mut self) {
        self.active = false;
        self.env.note_off(Mask::splat(true));
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

        self.osc.process(ctx, outputs);

        let mut v_mod = FloatVector::splat(0.);
        v_mod.map_in_place(|_| self.env.process());

        let g = Simd::splat(VOICE_GAIN);
        outputs[0].data.zip_map_in_place(&v_mod, |c, m| c * g * m);
        outputs[1].data.zip_map_in_place(&v_mod, |c, m| c * g * m);
    }

    fn reset(&mut self, sample_rate: f32) {
        self.osc.reset(sample_rate);
        self.active = false;
    }
}
