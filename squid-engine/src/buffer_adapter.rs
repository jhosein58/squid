use std::sync::Arc;

use squid_core::RING_BUFFER_CAPACITY;

use crate::AudioBridge;

pub struct BufferAdapter {
    l_buf: [f32; RING_BUFFER_CAPACITY],
    r_buf: [f32; RING_BUFFER_CAPACITY],
}

impl BufferAdapter {
    pub fn new() -> Self {
        Self {
            l_buf: [0.0; RING_BUFFER_CAPACITY],
            r_buf: [0.0; RING_BUFFER_CAPACITY],
        }
    }

    pub fn fill(&mut self, output: &mut [f32], bridge: &Arc<AudioBridge>) {
        let slice_len = (output.len() / 2).min(RING_BUFFER_CAPACITY);
        self.l_buf.fill(0.);
        self.r_buf.fill(0.);
        let mut l_slice = &mut self.l_buf[..slice_len];
        let mut r_slice = &mut self.r_buf[..slice_len];

        bridge.left_channel.pop_slice(&mut l_slice);
        bridge.right_channel.pop_slice(&mut r_slice);

        for i in 0..output.len() / 2 {
            output[i * 2] = l_slice.get(i).unwrap_or(&0.).tanh();
            output[i * 2 + 1] = r_slice.get(i).unwrap_or(&0.).tanh();
        }
    }
}
