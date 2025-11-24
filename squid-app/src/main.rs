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
    Event, EventData, FixedSpscQueue, FloatVector, Note, PitchClass, Plugin, Rand,
    modulators::envlopes::ar_env::ArEnv,
    oscillators::{saw_osc::SawOsc, sin_osc::SinOsc},
    process_context::ProcessContext,
    synths::poly_synth::PolySynth,
};
use squid_engine::{
    LivePlayback, OscilloscopeTrigger, StreamContext, TriggerEdge,
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
        let shared = Arc::new(FixedSpscQueue::<f32, 128>::new());

        let mut trigger_system = OscilloscopeTrigger::<128>::new(
            shared.clone(),
            0.0,
            TriggerEdge::Rising,
            0.5,
            10.,
            44100.,
        );

        let mut osc = UnisonOsc::new(Box::new(SawOsc::new()));
        osc.set_unison(16);
        osc.detune(4.);

        let mut synth = PolySynth::new(osc, ArEnv::new(0.05, 8., 44100.));

        let pd = LivePlayback::new(move |out| {
            let mut e = Vec::new();
            while let Some(v) = shared_ctx.events.pop() {
                e.push(v);
            }
            let ctx = &ProcessContext {
                inputs: &[],
                events: &e,
                sample_rate: 44100.,
            };

            synth.process(ctx, out);

            for (l, r) in out[0].data.iter().zip(out[1].data.iter()) {
                trigger_system.process_sample((*l + *r) / 2.);
            }

            let mut res = Vec::with_capacity(512);
            while let Some(v) = shared.pop() {
                res.push(v);
            }

            let mut g = last_waveform.lock().unwrap();

            let mut ol = FloatVector::from_array(&out[0].data);
            let or = FloatVector::from_array(&out[1].data);

            ol.zip_map_in_place(&or, |l, r| (l + r) / Simd::splat(2.));
            *g = ol.to_array().to_vec();
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
