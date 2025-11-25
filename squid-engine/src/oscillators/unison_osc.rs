use std::simd::Simd;

use squid_core::{
    AudioNode, Rand,
    mixing_simd::MixingSimd,
    oscillators::Oscillator,
    process_context::{FixedBuf, ProcessContext},
};

#[derive(Clone)]
pub struct UnisonOsc<T: Oscillator> {
    freq: f32,
    oscillator: Box<T>,
    voices: Vec<Box<T>>,
    pans: Vec<f32>,
    rng: Rand,
    sample_rate: f32,
    detune_amount: f32,
    l_sum: FixedBuf,
    r_sum: FixedBuf,
    tmp_buf: FixedBuf,
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
            l_sum: FixedBuf::default(),
            r_sum: FixedBuf::default(),
            tmp_buf: FixedBuf::default(),
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
        let v_zero = Simd::splat(0.);
        self.l_sum.map_in_place(|c| v_zero);
        self.r_sum.map_in_place(|c| v_zero);

        let mut div = 0.;

        for (voice, pan) in self.voices.iter_mut().zip(self.pans.iter()) {
            voice.process(ctx, &mut [&mut self.tmp_buf]);

            self.l_sum.zip_map_in_place(&self.tmp_buf, |l, t| {
                l + (MixingSimd::mono_pan_left(t, *pan))
            });
            self.r_sum.zip_map_in_place(&self.tmp_buf, |r, t| {
                r + (MixingSimd::mono_pan_right(t, *pan))
            });

            div += 1.;
        }

        if div == 0. {
            return;
        }

        div /= 8.;
        // let v_div = Simd::splat(div);

        outputs[0].map_from(&self.l_sum, |c| c);
        outputs[1].map_from(&self.r_sum, |c| c);
    }

    fn reset(&mut self, _: f32) {}
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
