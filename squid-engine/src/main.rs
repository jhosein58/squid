#![feature(portable_simd)]
use std::hint::black_box; // <--- این را اضافه کنید

use std::{
    simd::Simd,
    time::{Duration, Instant},
};

use squid_core::{
    AudioNode, FloatVector, MAX_BLOCK_SIZE,
    oscillators::{Oscillator, saw_osc::SawOsc},
    process_context::ProcessContext,
    vecblock::VecBlock,
};
use squid_engine::{LivePlayback, oscillators::unison_osc::UnisonOsc};

fn main() {
    let mut osc = UnisonOsc::new(Box::new(SawOsc::new()));
    osc.configure(880., 44100., None);
    osc.set_unison(255);
    osc.detune(100.);

    let ctx = ProcessContext::default();

    let _pb = LivePlayback::new(move |out| {
        osc.process(&ctx, out);
    });

    loop {
        std::thread::sleep(Duration::from_secs(100));
    }
}
