use squid_core::{FixedSpscQueue, RING_BUFFER_CAPACITY, process_context::FixedBuf};

pub struct AudioBridge {
    pub left_channel: FixedSpscQueue<f32, { RING_BUFFER_CAPACITY }>,
    pub right_channel: FixedSpscQueue<f32, { RING_BUFFER_CAPACITY }>,
}

impl AudioBridge {
    pub fn new() -> Self {
        AudioBridge {
            left_channel: FixedSpscQueue::new(),
            right_channel: FixedSpscQueue::new(),
        }
    }

    pub fn push_slice(&self, data: &[&FixedBuf]) {
        self.left_channel.push_slice(data[0].as_slice()).unwrap();
        self.right_channel.push_slice(data[1].as_slice()).unwrap();
    }
}
