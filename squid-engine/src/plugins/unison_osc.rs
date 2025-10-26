use squid_core::{Oscillator, Plugin, Rand, mixing::Mixing};

use crate::{Mixer, Voice};

pub struct UnisonOsc {
    voices: Vec<(Voice, f32)>,
    rng: Rand,
    detune_amount: f32,
}

impl UnisonOsc {
    pub fn new(constructor: fn(u32) -> Box<dyn Oscillator>, unison: u8) -> Self {
        let voices = (0..unison)
            .map(|_| (Voice::new(constructor(44100)), 0.))
            .collect();
        Self {
            voices,
            rng: Rand::new(100),
            detune_amount: 0.0,
        }
    }
    fn distribution_factor(i: f32, n: f32) -> f32 {
        if n <= 1.0 {
            return 0.0;
        }
        let normalized = i / (n - 1.0);

        normalized * 2.0 - 1.0
    }

    pub fn apply_distribution_factor(&mut self, detune_amount: f32) {
        let mut dist_list = Vec::with_capacity(self.voices.len());

        for i in 0..self.voices.len() {
            dist_list.push(Self::distribution_factor(
                i as f32,
                self.voices.len() as f32,
            ));
        }

        let mut shuffled = Vec::with_capacity(self.voices.len());
        while !dist_list.is_empty() {
            let index = self.rng.next_range_usize(0, dist_list.len() - 1);

            let value = dist_list.remove(index);
            shuffled.push(value);
        }

        for (i, (_, detune)) in self.voices.iter_mut().enumerate() {
            *detune = shuffled[i];
        }

        self.detune_amount = detune_amount;
    }
}

impl Oscillator for UnisonOsc {
    fn set_frequency(&mut self, frequency: f32) {
        for (voice, detune) in &mut self.voices {
            voice.set_frequency(frequency * (1.0 + *detune * self.detune_amount));
            voice.set_phase(self.rng.next_f32_bipolar());
        }
    }
    fn get_phase(&self) -> f32 {
        0.
    }
    fn next_sample(&mut self) -> f32 {
        0.
    }
    fn reset(&mut self) {}
    fn set_phase(&mut self, _: f32) {}
}

impl Plugin for UnisonOsc {
    fn channels(&self) -> u8 {
        2
    }
    fn process(
        &mut self,
        input: &[&[f32]],
        output: &mut [&mut [f32]],
        events: &[squid_core::Event],
    ) {
        let output = &mut output[0];

        for i in 0..output.len() / 2 {
            let mut left = 0.;
            let mut right = 0.;
            let mut divider = 0.;

            let vlen = self.voices.len();

            for (pan, (voice, _)) in self.voices.iter_mut().enumerate() {
                let final_pan = Self::distribution_factor(pan as f32, vlen as f32);

                let final_lr = Mixing::constant_power_pan(voice.next_sample(), final_pan);
                left += final_lr[0];
                right += final_lr[1];

                divider += 1.;
            }

            output[i * 2] = left / divider;
            output[i * 2 + 1] = right / divider;
        }
    }
}
