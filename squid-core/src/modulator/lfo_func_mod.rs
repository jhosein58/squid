use libm::fmodf;

use crate::{
    dsp::Waveform,
    modulator::{CyclicMod, ModRate, Modulator},
    timing::Transport,
};

pub struct FunctionLFO<W: Waveform> {
    waveform_fn: W,
    rate: ModRate,
    phase: f32,
    sample_rate: f32,
}

impl<W: Waveform> FunctionLFO<W> {
    pub fn new(sample_rate: f32, waveform_fn: W) -> Self {
        Self {
            waveform_fn,
            rate: ModRate::Hz(1.0),
            phase: 0.0,
            sample_rate,
        }
    }
}

impl<W: Waveform> Modulator for FunctionLFO<W> {
    fn tick(&mut self, transport: &Transport) {
        let phase_increment = match self.rate {
            ModRate::Hz(hz) => hz / self.sample_rate,
            ModRate::Beat(beat_division) => {
                let beats_per_second = transport.bpm() / 60.0;
                let cycles_per_second = beats_per_second / beat_division;
                cycles_per_second / self.sample_rate
            }
        };

        self.phase = fmodf(self.phase + phase_increment, 1.0);
    }

    fn value(&self) -> f32 {
        self.waveform_fn.process(self.phase)
    }

    fn reset(&mut self) {
        self.phase = 0.0;
    }
}

impl<W: Waveform> CyclicMod for FunctionLFO<W> {
    fn set_rate(&mut self, rate: ModRate) {
        self.rate = rate;
    }
}
