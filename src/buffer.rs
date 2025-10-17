use crate::config::MAX_BLOCK_SIZE;

#[derive(Debug, Clone)]
pub struct AudioBuffer<const MAX_LEN: usize> {
    pub samples: [f32; MAX_LEN],
    pub num_channels: u16,
    pub sample_rate: u32,
    len_frames: usize,
}

impl<const MAX_LEN: usize> AudioBuffer<MAX_LEN> {
    pub fn new(num_channels: u16, sample_rate: u32) -> Self {
        assert!(
            MAX_LEN > 0 && num_channels > 0 && MAX_LEN % num_channels as usize == 0,
            "MAX_LEN must be a non-zero multiple of num_channels"
        );
        Self {
            samples: [0.0; MAX_LEN],
            num_channels,
            sample_rate,
            len_frames: 0,
        }
    }

    pub fn clear(&mut self) {
        self.len_frames = 0;
    }

    pub fn from_interleaved(samples: [f32; MAX_LEN], num_channels: u16, sample_rate: u32) -> Self {
        assert!(
            MAX_LEN > 0 && num_channels > 0 && MAX_LEN % num_channels as usize == 0,
            "MAX_LEN must be a non-zero multiple of num_channels"
        );
        Self {
            samples,
            num_channels,
            sample_rate,
            len_frames: MAX_LEN / num_channels as usize,
        }
    }

    pub fn num_frames(&self) -> usize {
        self.len_frames
    }

    pub fn max_frames(&self) -> usize {
        MAX_LEN / self.num_channels as usize
    }

    pub fn duration_secs(&self) -> f32 {
        if self.sample_rate == 0 {
            0.0
        } else {
            self.num_frames() as f32 / self.sample_rate as f32
        }
    }

    pub fn push_frame(&mut self, frame: &[f32]) {
        assert_eq!(
            frame.len(),
            self.num_channels as usize,
            "Frame length must match number of channels."
        );

        let start_index = self.len_frames * self.num_channels as usize;
        let end_index = start_index + self.num_channels as usize;

        if end_index > MAX_LEN {
            panic!("AudioBuffer overflow: Not enough space to push a new frame.");
        }

        self.samples[start_index..end_index].copy_from_slice(frame);
        self.len_frames += 1;
    }

    pub fn set_len_frames(&mut self, new_len_frames: usize) {
        let max_frames = self.max_frames();
        if new_len_frames > max_frames {
            panic!(
                "New length ({}) exceeds buffer capacity ({})",
                new_len_frames, max_frames
            );
        }
        self.len_frames = new_len_frames;
    }

    pub fn as_slice(&self) -> &[f32] {
        let len_samples = self.len_frames * self.num_channels as usize;
        &self.samples[0..len_samples]
    }

    pub fn as_mut_slice(&mut self) -> &mut [f32] {
        let len_samples = self.len_frames * self.num_channels as usize;
        &mut self.samples[0..len_samples]
    }
}

pub type AudioBufferML = AudioBuffer<MAX_BLOCK_SIZE>;
