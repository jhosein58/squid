use crate::{
    AudioNode, Note,
    oscillators::Oscillator,
    process_context::{FixedBuf, ProcessContext},
};

#[derive(Clone, Copy)]
pub struct Voice<T: Oscillator> {
    osc: T,
    active: bool,
    sample_rate: f32,
    freq: f32,
    note: u8,
}

impl<T> Voice<T>
where
    T: Oscillator,
{
    pub fn new(mut osc: T) -> Self {
        Self {
            osc,
            active: false,
            sample_rate: 0.,
            freq: 0.,
            note: 0,
        }
    }

    pub fn is_idle(&self) -> bool {
        !self.active
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
    }

    pub fn note_off(&mut self) {
        self.active = false;
    }
}

impl<T> AudioNode for Voice<T>
where
    T: Oscillator,
{
    fn process(&mut self, ctx: &ProcessContext, outputs: &mut [&mut FixedBuf]) {
        if self.active {
            self.osc.process(ctx, outputs);
        }
    }

    fn reset(&mut self, sample_rate: f32) {
        self.osc.reset(sample_rate);
    }
}
