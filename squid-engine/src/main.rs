#![feature(portable_simd)]

use std::hint::black_box;

use std::sync::Arc;
use std::thread;
use std::{
    simd::Simd,
    time::{Duration, Instant},
};

use squid_core::process_context::FixedBuf;
use squid_core::{
    AudioNode, FloatVector, MAX_BLOCK_SIZE,
    oscillators::{Oscillator, saw_osc::SawOsc},
    process_context::ProcessContext,
    vecblock::VecBlock,
};
use squid_core::{FixedSpscQueue, Rand, TARGET_LATENCY_FRAMES};
use squid_engine::{AudioBridge, BufferAdapter, audio_bridge};
use squid_engine::{LivePlayback, oscillators::unison_osc::UnisonOsc};

fn main() {
    let mut osc = UnisonOsc::new(Box::new(SawOsc::new()));
    osc.configure(440., 44100., None);
    osc.set_unison(255);
    osc.detune(10.);

    let mut osc2 = SawOsc::new();
    osc2.configure(440., 44100., None);

    let ctx = ProcessContext::default();

    let audio_bridge = Arc::new(AudioBridge::new());
    let audio_bridge_clone = audio_bridge.clone();

    let mut l_buf = FixedBuf::default();
    let mut r_buf = FixedBuf::default();

    let mut fake_l_buf = FixedBuf::default();
    let mut fake_r_buf = FixedBuf::default();

    thread::spawn(move || {
        loop {
            while audio_bridge_clone.left_channel.len() >= TARGET_LATENCY_FRAMES
                || !audio_bridge_clone.left_channel.has_room_for(MAX_BLOCK_SIZE)
            {
                std::thread::sleep(std::time::Duration::from_micros(500));
            }

            for _ in 0..22 {
                osc.process(&ctx, &mut [&mut fake_l_buf, &mut fake_r_buf]);
            }

            osc2.process(&ctx, &mut [&mut l_buf, &mut r_buf]);

            audio_bridge_clone.push_slice(&[&l_buf, &r_buf]);
        }
    });

    let mut adapter = BufferAdapter::new();
    let _pb = LivePlayback::new_raw(move |out| {
        adapter.fill(out, &audio_bridge);
    });

    loop {
        std::thread::sleep(Duration::from_secs(100));
    }
}
