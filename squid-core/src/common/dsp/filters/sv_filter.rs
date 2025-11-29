// State-Variable Filter (SVF) implementation.

use core::f32::consts::PI;

use crate::process_context::FixedBuf;

#[derive(Clone, Copy)]
pub struct ScalarSvf {
    ic1eq: f32,
    ic2eq: f32,

    g: f32,
    k: f32,
    a1: f32,
    a2: f32,
    a3: f32,
}

impl ScalarSvf {
    pub fn new() -> Self {
        Self {
            ic1eq: 0.0,
            ic2eq: 0.0,
            g: 0.0,
            k: 0.0,
            a1: 0.0,
            a2: 0.0,
            a3: 0.0,
        }
    }

    pub fn update_coeffs(&mut self, cutoff: f32, q: f32, sample_rate: f32) {
        let cutoff_clamped = cutoff.clamp(10.0, sample_rate / 2.0 - 100.0);
        self.g = (PI * cutoff_clamped / sample_rate).tan();

        let safe_q = q.max(0.5);
        self.k = 1.0 / safe_q;

        self.a1 = 1.0 / (1.0 + self.g * (self.g + self.k));
        self.a2 = self.g * self.a1;
        self.a3 = self.g * self.a2;
    }

    #[inline(always)]
    pub fn process(&mut self, v0: f32) -> (f32, f32, f32) {
        let v1 = self.a1 * self.ic1eq + self.a2 * (v0 - self.ic2eq);
        let v2 = self.ic2eq + self.g * v1;

        let lp = v2;
        let bp = v1;
        let hp = v0 - self.k * v1 - v2;

        self.ic1eq = 2.0 * v1 - self.ic1eq;
        self.ic2eq = 2.0 * v2 - self.ic2eq;

        (lp, hp, bp)
    }

    #[inline(always)]
    pub fn process_block_lp(&mut self, out: &mut FixedBuf) {
        for i in out.iter_mut() {
            let (lp, _, _) = self.process(*i);
            *i = lp;
        }
    }

    #[inline(always)]
    pub fn process_block_hp(&mut self, out: &mut FixedBuf) {
        for i in out.iter_mut() {
            let (_, hp, _) = self.process(*i);
            *i = hp;
        }
    }

    #[inline(always)]
    pub fn process_block_bp(&mut self, out: &mut FixedBuf) {
        for i in out.iter_mut() {
            let (_, _, bp) = self.process(*i);
            *i = bp;
        }
    }
}
