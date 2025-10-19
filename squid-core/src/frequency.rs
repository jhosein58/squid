use libm::powf;

use crate::{config::DEFAULT_CONFIG, note::Note};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Frequency(pub f32);

impl Frequency {
    pub const A4_HERTZ: f32 = DEFAULT_CONFIG.a4_freq;
    pub const A4_MIDI: u8 = 69;

    pub fn from_midi(midi_note: u8) -> Self {
        let exponent = (midi_note as f32 - Self::A4_MIDI as f32) / 12.0;
        Frequency(Self::A4_HERTZ * powf(2., exponent))
    }

    pub fn as_hertz(&self) -> f32 {
        self.0
    }
}

impl From<f32> for Frequency {
    fn from(hertz: f32) -> Self {
        Frequency(hertz)
    }
}
impl From<Frequency> for f32 {
    fn from(freq: Frequency) -> Self {
        freq.0
    }
}
impl From<Note> for Frequency {
    fn from(note: Note) -> Self {
        note.to_frequency()
    }
}
