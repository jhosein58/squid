use squid_core::{
    MAX_BLOCK_SIZE,
    process_context::{FixedBuf, ProcessContext},
};

pub struct Filler {
    handler: Box<dyn for<'a, 'b> FnMut(&'a ProcessContext<'b>, &mut [&mut FixedBuf]) + Send>,
    left_channel: Vec<f32>,
    right_channel: Vec<f32>,
}

impl Filler {
    pub fn new(
        handler: Box<dyn for<'a, 'b> FnMut(&'a ProcessContext<'b>, &mut [&mut FixedBuf]) + Send>,
    ) -> Self {
        Filler {
            handler,
            left_channel: vec![],
            right_channel: vec![],
        }
    }

    pub fn fill_stereo(&mut self, buf: &mut [f32]) {
        debug_assert!(buf.len() % 2 == 0);

        let mut write_idx = 0;

        if !self.left_channel.is_empty() {
            let frames = self.left_channel.len().min(buf.len() / 2);
            for i in 0..frames {
                buf[i * 2] = self.left_channel[i];
                buf[i * 2 + 1] = self.right_channel[i];
            }
            write_idx = frames * 2;

            if frames == self.left_channel.len() {
                self.left_channel.clear();
                self.right_channel.clear();
            } else {
                self.left_channel.drain(0..frames);
                self.right_channel.drain(0..frames);
                return;
            }
        }

        let mut l_channel = FixedBuf {
            data: [0.0; MAX_BLOCK_SIZE],
        };
        let mut r_channel = FixedBuf {
            data: [0.0; MAX_BLOCK_SIZE],
        };

        const BLOCK_SAMPLES_STEREO: usize = MAX_BLOCK_SIZE * 2;

        let available_samples = buf.len() - write_idx;
        let full_blocks = available_samples / BLOCK_SAMPLES_STEREO;

        for _ in 0..full_blocks {
            (self.handler)(
                &ProcessContext {
                    sample_rate: 44100.0,
                    events: &[],
                    inputs: &[],
                },
                &mut [&mut l_channel, &mut r_channel],
            );

            for i in 0..MAX_BLOCK_SIZE {
                buf[write_idx + i * 2] = l_channel.data[i];
                buf[write_idx + i * 2 + 1] = r_channel.data[i];
            }
            write_idx += BLOCK_SAMPLES_STEREO;
        }

        let remaining_samples = buf.len() - write_idx;
        let remaining_frames = remaining_samples / 2;

        if remaining_frames > 0 {
            (self.handler)(
                &ProcessContext {
                    sample_rate: 44100.0,
                    events: &[],
                    inputs: &[],
                },
                &mut [&mut l_channel, &mut r_channel],
            );

            for i in 0..remaining_frames {
                let l = l_channel.data[i];
                let r = r_channel.data[i];
                buf[write_idx + i * 2] = l;
                buf[write_idx + i * 2 + 1] = r;
            }

            for i in remaining_frames..MAX_BLOCK_SIZE {
                self.left_channel.push(l_channel.data[i]);
                self.right_channel.push(r_channel.data[i]);
            }
        }
    }
}
