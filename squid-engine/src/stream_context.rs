use std::sync::{Arc, atomic::AtomicU8};

use squid_core::{Event, FixedSpscQueue};

pub struct StreamContext {
    pub waveform: Arc<FixedSpscQueue<f32, 512>>,
    pub events: Arc<FixedSpscQueue<Event, 128>>,
    pub f1: Arc<AtomicU8>,
    pub f2: Arc<AtomicU8>,
    pub f3: Arc<AtomicU8>,
}

impl StreamContext {
    pub fn new() -> Self {
        Self {
            waveform: Arc::new(FixedSpscQueue::new()),
            events: Arc::new(FixedSpscQueue::new()),
            f1: Arc::new(AtomicU8::new(0)),
            f2: Arc::new(AtomicU8::new(0)),
            f3: Arc::new(AtomicU8::new(0)),
        }
    }
}
