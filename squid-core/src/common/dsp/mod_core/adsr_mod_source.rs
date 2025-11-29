use core::simd::{
    LaneCount, Mask, Simd, SupportedLaneCount,
    cmp::{SimdPartialEq, SimdPartialOrd},
};

use sleef::Sleef;

const STATE_IDLE: f32 = 0.0;
const STATE_ATTACK: f32 = 1.0;
const STATE_DECAY: f32 = 2.0;
const STATE_SUSTAIN: f32 = 3.0;
const STATE_RELEASE: f32 = 4.0;

#[derive(Clone, Copy)]
pub struct AdsrModSource<const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
{
    current_voltage: Simd<f32, N>,
    state: Simd<f32, N>,

    attack_rate: Simd<f32, N>,
    decay_rate: Simd<f32, N>,
    release_rate: Simd<f32, N>,

    attack_target: Simd<f32, N>,
    sustain_level: Simd<f32, N>,

    epsilon: Simd<f32, N>,
}

impl<const N: usize> AdsrModSource<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    pub fn new() -> Self {
        Self {
            current_voltage: Simd::splat(0.0),
            state: Simd::splat(STATE_IDLE),

            attack_rate: Simd::splat(0.1),
            decay_rate: Simd::splat(0.05),
            release_rate: Simd::splat(0.01),
            attack_target: Simd::splat(1.5),

            sustain_level: Simd::splat(0.7),

            epsilon: Simd::splat(1e-4),
        }
    }

    pub fn note_on(&mut self, mask: Mask<i32, N>) {
        self.state = mask.select(Simd::splat(STATE_ATTACK), self.state);
    }

    pub fn note_off(&mut self, mask: Mask<i32, N>) {
        self.state = mask.select(Simd::splat(STATE_RELEASE), self.state);
    }

    pub fn is_idle(&self) -> bool {
        self.state.as_array()[0] == STATE_IDLE
    }

    pub fn is_active(&self) -> bool {
        !self.is_idle()
    }

    #[inline]
    pub fn process(&mut self) -> Simd<f32, N> {
        let zero = Simd::splat(0.0);
        let one = Simd::splat(1.0);

        let is_attack = self.state.simd_eq(Simd::splat(STATE_ATTACK));
        let is_decay = self.state.simd_eq(Simd::splat(STATE_DECAY));
        let is_release = self.state.simd_eq(Simd::splat(STATE_RELEASE));
        let is_idle = self.state.simd_eq(Simd::splat(STATE_IDLE));

        let target = is_attack.select(
            self.attack_target,
            is_decay.select(
                self.sustain_level,
                is_release.select(zero, is_idle.select(zero, self.sustain_level)),
            ),
        );

        let rate = is_attack.select(
            self.attack_rate,
            is_decay.select(self.decay_rate, is_release.select(self.release_rate, zero)),
        );

        let diff = target - self.current_voltage;
        self.current_voltage += diff * rate;

        let attack_done = is_attack & self.current_voltage.simd_ge(one);
        if attack_done.any() {
            self.current_voltage = attack_done.select(one, self.current_voltage);
            self.state = attack_done.select(Simd::splat(STATE_DECAY), self.state);
        }

        let dist_to_sustain = (self.current_voltage - self.sustain_level).abs();
        let decay_done = is_decay & dist_to_sustain.simd_le(self.epsilon);

        if decay_done.any() {
            self.state = decay_done.select(Simd::splat(STATE_SUSTAIN), self.state);
            self.current_voltage = decay_done.select(self.sustain_level, self.current_voltage);
        }

        let release_done = is_release & self.current_voltage.simd_le(self.epsilon);
        if release_done.any() {
            self.state = release_done.select(Simd::splat(STATE_IDLE), self.state);
            self.current_voltage = release_done.select(zero, self.current_voltage);
        }

        self.current_voltage
    }

    pub fn set_parameters(&mut self, att_rate: f32, dec_rate: f32, rel_rate: f32, sus_lvl: f32) {
        self.attack_rate = Simd::splat(att_rate);
        self.decay_rate = Simd::splat(dec_rate);
        self.release_rate = Simd::splat(rel_rate);
        self.sustain_level = Simd::splat(sus_lvl);
    }
}

pub fn calculate_coefficient(time_ms: f32, sample_rate: f32) -> f32 {
    if time_ms <= 0.0 {
        return 1.0;
    }
    let time_seconds = time_ms / 1000.0;
    let samples = time_seconds * sample_rate;

    1.0 - (-1.0 / (samples * 0.3)).exp()
}
