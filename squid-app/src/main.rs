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
    Event, EventData, FixedSpscQueue, FloatVector, MAX_BLOCK_SIZE, Note, PitchClass, Plugin,
    dsp::{
        filters::sv_filter::ScalarSvf,
        mod_core::adsr_mod_source::{AdsrModSource, calculate_coefficient},
    },
    modulators::envlopes::ar_env::ArEnv,
    oscillators::{saw_osc::SawOsc, sin_osc::SinOsc},
    process_context::{FixedBuf, ProcessContext},
    synths::poly_synth::PolySynth,
};
use squid_engine::{
    AudioBridge, BufferAdapter, LivePlayback, StreamContext, oscillators::unison_osc::UnisonOsc,
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

    let shared = Arc::new(FixedSpscQueue::<f32, 2048>::new());

    let audio_bridge = Arc::new(AudioBridge::new());
    let audio_bridge_c = audio_bridge.clone();

    let mut l_buf = FixedBuf::default();
    let mut r_buf = FixedBuf::default();

    let mut osc = UnisonOsc::new(Box::new(SawOsc::new()));
    osc.set_unison(12);
    osc.detune(4.);

    let mut adsr = AdsrModSource::new();
    let sample_rate = 44100.0;

    let attack_coeff = calculate_coefficient(25.0, sample_rate);
    let decay_coeff = calculate_coefficient(10.0, sample_rate);
    let release_coeff = calculate_coefficient(300.0, sample_rate);

    adsr.set_parameters(attack_coeff, decay_coeff, release_coeff, 0.8);
    let mut synth = PolySynth::new(osc, adsr);

    let mut f1 = ScalarSvf::new();
    let mut f2 = ScalarSvf::new();

    let mut c = 10.;
    let mut t = 0;

    let mut pd = LivePlayback::init();
    pd.start(move |out| {
        f1.update_coeffs(c, 0.7, 44100.);
        f2.update_coeffs(c, 0.7, 44100.);
        let mut e = Vec::new();
        while let Some(v) = shared_ctx.events.pop() {
            e.push(v);
        }
        let ctx = &ProcessContext {
            inputs: &[],
            events: &e,
            sample_rate: 44100.,
        };

        synth.process(&ctx, out);
        out[0].data.map_in_place(|c| c * Simd::splat(0.5));
        out[1].data.map_in_place(|c| c * Simd::splat(0.5));
        f1.process_block_lp(out[0]);
        f2.process_block_lp(out[1]);
        if t > 1500 {
            c += 2.;
        }
        t += 1;

        for (i, j) in out[0].iter().zip(out[1].iter()) {
            let _ = shared.push((i + j) / 3.);
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
