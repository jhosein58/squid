#![feature(portable_simd)]

use std::{
    fs,
    simd::Simd,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use macroquad::prelude::*;
use mlua::{Function, Lua, Result};
use squid_app::api::RuntimeApi;
use squid_core::{
    Event, EventData, FixedSpscQueue, FloatVector, MAX_BLOCK_SIZE, Note, PitchClass, Plugin, Rand,
    TARGET_LATENCY_FRAMES,
    modulators::envlopes::ar_env::ArEnv,
    oscillators::{saw_osc::SawOsc, sin_osc::SinOsc},
    process_context::{FixedBuf, ProcessContext},
    synths::poly_synth::PolySynth,
};
use squid_engine::{
    AudioBridge, BufferAdapter, LivePlayback, OscilloscopeTrigger, StreamContext, TriggerEdge,
    oscillators::unison_osc::UnisonOsc, sin_synth::SinSynth,
};

use squid_core::AudioNode;
use squid_core::oscillators::Oscillator;

fn window_conf() -> Conf {
    Conf {
        window_title: "Squid".to_string(),
        window_resizable: true,
        fullscreen: false,

        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    let ctx = Arc::new(StreamContext::new());
    let shared_ctx = ctx.clone();

    let last_waveform: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
    let last_waveform_c = last_waveform.clone();

    thread::spawn(move || {
        let shared = Arc::new(FixedSpscQueue::<f32, 2048>::new());

        let audio_bridge = Arc::new(AudioBridge::new());
        let audio_bridge_c = audio_bridge.clone();

        let mut l_buf = FixedBuf::default();
        let mut r_buf = FixedBuf::default();

        let mut osc = UnisonOsc::new(Box::new(SawOsc::new()));
        osc.set_unison(12);
        osc.detune(4.);

        let mut synth = PolySynth::new(osc, ArEnv::new(0.1, 0.5, 44100.));

        thread::spawn(move || {
            loop {
                while audio_bridge_c.left_channel.len() >= TARGET_LATENCY_FRAMES
                    || !audio_bridge_c.left_channel.has_room_for(MAX_BLOCK_SIZE)
                {
                    std::thread::sleep(std::time::Duration::from_micros(500));
                }

                let mut e = Vec::new();
                while let Some(v) = shared_ctx.events.pop() {
                    e.push(v);
                }
                let ctx = &ProcessContext {
                    inputs: &[],
                    events: &e,
                    sample_rate: 44100.,
                };

                synth.process(&ctx, &mut [&mut l_buf, &mut r_buf]);

                audio_bridge_c.push_slice(&[&l_buf, &r_buf]);
            }
        });

        let mut adapter = BufferAdapter::new();
        let _pd = LivePlayback::new_raw(move |out| {
            adapter.fill(out, &audio_bridge);

            for i in 0..out.len() / 2 {
                let _ = shared.push((out[i * 2] + out[i * 2 + 1]) / 2.);
            }

            if shared.len() >= 500 {
                let mut res = Vec::with_capacity(2048);
                while let Some(v) = shared.pop() {
                    res.push(v);
                }

                let mut g = last_waveform.lock().unwrap();
                *g = res;
            }
        });

        loop {
            thread::sleep(Duration::from_millis(10));
        }
    });

    let lua = Lua::new();
    let mut runtime = RuntimeApi::new();
    runtime.add_api_to_lua(&lua, ctx.clone());

    let code = fs::read_to_string("gui/main.lua")?;
    lua.load(code).exec()?;
    loop {
        clear_background(BLACK);

        if let Ok(update) = lua.globals().get::<Function>("update") {
            update.call::<()>(())?;
        }

        if let Ok(update) = lua.globals().get::<Function>("waveform") {
            let g = last_waveform_c.lock().unwrap();

            update.call::<()>(g.clone())?;
        }

        next_frame().await;
    }
}
