use core::simd::f32x8;
use std::simd::{LaneCount, SupportedLaneCount};

use crate::vecblock::VecBlock;

#[derive(Debug, Clone, Copy)]
pub struct EngineConfig {
    pub sample_rate: u32,
    pub max_block_size: usize,
    pub sound_card_buffer_size: usize,
    pub target_latency_frames: usize,
    pub ring_buffer_capacity: usize,

    pub a4_freq: f32,
    pub lut_resolution: usize,
    pub silence_db: f32,
    pub voice_gain: f32,
}

impl EngineConfig {
    pub const fn default_for_audio() -> Self {
        Self {
            sample_rate: 44100,

            max_block_size: 64,
            sound_card_buffer_size: 256,
            target_latency_frames: 256 * 2,
            ring_buffer_capacity: 2048,

            a4_freq: 440.0,
            lut_resolution: 1024,
            silence_db: -96.0,
            voice_gain: 0.25,
        }
    }
}

pub const DEFAULT_CONFIG: EngineConfig = EngineConfig::default_for_audio();
pub const MAX_BLOCK_SIZE: usize = DEFAULT_CONFIG.max_block_size;
pub const LUT_RESOLUTION: usize = DEFAULT_CONFIG.lut_resolution;
pub const SILENCE_DB: f32 = DEFAULT_CONFIG.silence_db;
pub const VOICE_GAIN: f32 = DEFAULT_CONFIG.voice_gain;
pub const TARGET_LATENCY_FRAMES: usize = DEFAULT_CONFIG.target_latency_frames;
pub const RING_BUFFER_CAPACITY: usize = DEFAULT_CONFIG.ring_buffer_capacity;

pub type Fv<const N: usize> = VecBlock<f32, N, { MAX_BLOCK_SIZE / N }>;
pub type Fv8 = Fv<8>;
pub type Fv16 = Fv<16>;
pub type FloatVector = Fv8;

impl<const N: usize> Default for Fv<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    fn default() -> Self {
        Self::splat(0.)
    }
}
