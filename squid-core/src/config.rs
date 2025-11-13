use core::simd::f32x8;

use crate::vecblock::VecBlock;

#[derive(Debug, Clone, Copy)]
pub struct EngineConfig {
    pub sample_rate: u32,
    pub max_block_size: usize,
    pub a4_freq: f32,
}

impl EngineConfig {
    pub const fn default_for_audio() -> Self {
        Self {
            sample_rate: 44100,
            max_block_size: 128,
            a4_freq: 440.0,
        }
    }
}

pub const DEFAULT_CONFIG: EngineConfig = EngineConfig::default_for_audio();
pub const MAX_BLOCK_SIZE: usize = DEFAULT_CONFIG.max_block_size;

pub type Fv<const N: usize> = VecBlock<f32, N, { MAX_BLOCK_SIZE / N }>;
pub type Fv8 = Fv<8>;
pub type FloatVector = Fv8;
