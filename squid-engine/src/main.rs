#![feature(portable_simd)]

use std::time::Duration;

use squid_core::{
    AudioNode,
    oscillators::{Oscillator, saw_osc::SawOsc, sin_osc::SinOsc},
    process_context::ProcessContext,
};
use squid_engine::LivePlayback;

fn main() {
    let mut osc = SawOsc::new();
    osc.configure(440., 44100., None);

    let mut osc_pool = vec![osc; 13_000];

    let ctx = ProcessContext::default();

    let mut pb = LivePlayback::init();
    pb.start(move |out| {
        for osc in &mut osc_pool {
            osc.process(&ctx, out);
        }
        osc.process(&ctx, out);
    });

    loop {
        std::thread::sleep(Duration::from_secs(100));
    }
}
