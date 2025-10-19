use crate::frequency::Frequency;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum PitchClass {
    C = 0,
    CSharp = 1,
    D = 2,
    DSharp = 3,
    E = 4,
    F = 5,
    FSharp = 6,
    G = 7,
    GSharp = 8,
    A = 9,
    ASharp = 10,
    B = 11,
}

impl PitchClass {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => PitchClass::C,
            1 => PitchClass::CSharp,
            2 => PitchClass::D,
            3 => PitchClass::DSharp,
            4 => PitchClass::E,
            5 => PitchClass::F,
            6 => PitchClass::FSharp,
            7 => PitchClass::G,
            8 => PitchClass::GSharp,
            9 => PitchClass::A,
            10 => PitchClass::ASharp,
            11 => PitchClass::B,
            _ => panic!("Invalid u8 value for PitchClass: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Note {
    pub pitch_class: PitchClass,
    pub octave: i8,
}

impl Note {
    pub const A4: Note = Note {
        pitch_class: PitchClass::A,
        octave: 4,
    };

    pub fn new(pitch_class: PitchClass, octave: i8) -> Self {
        Self {
            pitch_class,
            octave,
        }
    }

    pub fn to_midi(&self) -> u8 {
        let midi_val = (self.octave as i16 + 1) * 12 + self.pitch_class as i16;
        if midi_val < 0 || midi_val > 127 {
            panic!("Note {:?} is outside the valid MIDI range (0-127)", self);
        }
        midi_val as u8
    }

    pub fn from_midi(midi_note: u8) -> Self {
        if midi_note > 127 {
            panic!("Invalid MIDI note number: {}", midi_note);
        }
        let octave = (midi_note as i8 / 12) - 1;
        let pitch_class_val = midi_note % 12;
        let pitch_class = PitchClass::from_u8(pitch_class_val);
        Self {
            pitch_class,
            octave,
        }
    }

    pub fn to_frequency(&self) -> Frequency {
        Frequency::from_midi(self.to_midi())
    }

    pub fn transpose(&self, semitones: i8) -> Self {
        let current_midi = self.to_midi() as i16;
        let new_midi = current_midi + semitones as i16;
        Note::from_midi(new_midi as u8)
    }
}
