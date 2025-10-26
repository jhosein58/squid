use squid_core::{Note, Oscillator};

pub mod sin_synth;
pub mod unison_osc;
pub mod unison_synth;

pub struct Voice {
    active: bool,
    osc: Box<dyn Oscillator>,
    note: u8,
    velocity: f32,
}

impl Voice {
    pub fn new(osc: Box<dyn Oscillator>) -> Self {
        Self {
            active: false,
            osc,
            note: 60,
            velocity: 1.0,
        }
    }

    pub fn is_free(&self) -> bool {
        !self.active
    }
    pub fn is_playing(&self) -> bool {
        self.active
    }
    pub fn set_note(&mut self, note: u8) {
        self.note = note;
    }

    pub fn note(&self) -> u8 {
        self.note
    }

    pub fn activate(&mut self, note: u8) {
        self.active = true;
        self.note = note;
        self.set_frequency(Note::from_midi(note).to_frequency().into());
    }
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.osc.set_frequency(frequency);
    }

    pub fn set_phase(&mut self, phase: f32) {
        self.osc.set_phase(phase);
    }

    pub fn phase(&self) -> f32 {
        self.osc.get_phase()
    }

    pub fn next_sample(&mut self) -> f32 {
        self.osc.next_sample()
    }

    pub fn reset(&mut self) {
        self.osc.reset();
    }
}
