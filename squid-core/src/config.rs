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
            max_block_size: 512,
            a4_freq: 440.0,
        }
    }
}

pub const DEFAULT_CONFIG: EngineConfig = EngineConfig::default_for_audio();
pub const MAX_BLOCK_SIZE: usize = DEFAULT_CONFIG.max_block_size;
