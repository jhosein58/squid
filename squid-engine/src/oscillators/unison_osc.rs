use std::simd::f32x8;

use squid_core::{
    AudioNode, FloatVector, Rand,
    mixing_simd::MixingSimd,
    oscillators::Oscillator,
    process_context::{FixedBuf, ProcessContext},
    shapers::sine_shaper,
};

use crate::Voice;

#[derive(Clone)]
pub struct UnisonOsc<T: Oscillator> {
    freq: f32,
    oscillator: Box<T>,
    voices: Vec<Box<T>>,
    pans: Vec<f32>,
    rng: Rand,
    sample_rate: f32,
    detune_amount: f32,
}

impl<T: Oscillator> UnisonOsc<T> {
    pub fn new(osc: Box<T>) -> Self {
        Self {
            freq: 440.0,
            oscillator: osc,
            voices: vec![],
            pans: vec![],
            rng: Rand::new(0),
            sample_rate: 44100.,
            detune_amount: 0.,
        }
    }

    pub fn set_unison(&mut self, unison: u8) {
        self.voices = vec![self.oscillator.clone(); unison as usize];
    }

    pub fn detune(&mut self, amount: f32) {
        self.detune_amount = amount;
        let n = self.voices.len();
        if n == 0 {
            return;
        }

        self.pans.clear();

        let detune_range_cents = amount;

        for (_, voice) in self.voices.iter_mut().enumerate() {
            let cents = self
                .rng
                .next_range_f32(-detune_range_cents, detune_range_cents);
            let detune_factor = 2f32.powf(cents / 1200.0);
            let freq = self.freq * detune_factor;

            let phase = self.rng.next_range_f32(0.0, std::f32::consts::TAU);
            voice.configure(freq, self.sample_rate, Some(phase));

            let pan = self.rng.next_range_f32(-1.0, 1.0);
            self.pans.push(pan);
        }
    }
}

impl<T: Oscillator> AudioNode for UnisonOsc<T> {
    fn process(&mut self, ctx: &ProcessContext, outputs: &mut [&mut FixedBuf]) {
        let mut l_sum = FixedBuf::default();
        let mut r_sum = FixedBuf::default();
        let mut div = 0;
        let mut tmp_buf = FixedBuf::default();

        for (voice, pan) in self.voices.iter_mut().zip(self.pans.iter()) {
            voice.process(ctx, &mut [&mut tmp_buf]);

            let (tmp_chunks, _) = tmp_buf.data.as_chunks::<8>();
            for (i, chunk) in tmp_chunks.iter().enumerate() {
                let (left, right) = MixingSimd::mono_pan(f32x8::from_array(*chunk), *pan);
                for j in 0..8 {
                    let left = left[j];
                    let right = right[j];
                    l_sum.data[i * 8 + j] += left;
                    r_sum.data[i * 8 + j] += right;
                }
            }
            div += 1;
        }

        if div == 0 {
            return;
        }

        div /= 8;
        for i in 0..l_sum.data.len() {
            l_sum.data[i] /= div as f32;
            r_sum.data[i] /= div as f32;
        }

        outputs[0].data.copy_from_slice(&l_sum.data);
        outputs[1].data.copy_from_slice(&r_sum.data);
    }

    fn reset(&mut self, sample_rate: f32) {}
}

impl<T: Oscillator> Oscillator for UnisonOsc<T> {
    fn configure(&mut self, freq: f32, sample_rate: f32, _: Option<f32>) {
        self.freq = freq;
        self.sample_rate = sample_rate;

        for voice in self.voices.iter_mut() {
            voice.configure(freq, sample_rate, None);
        }
        self.detune(self.detune_amount);
    }
}
