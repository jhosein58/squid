use std::{
    fs,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use macroquad::prelude::*;
use mlua::{Function, Lua, Result};
use squid_app::api::RuntimeApi;
use squid_core::{
    Event, EventData, FixedSpscQueue, Note, Oscillator, PitchClass, Plugin, primetives::SawOsc,
};
use squid_engine::{
    LivePlaybackk, OscilloscopeTrigger, StreamContext, TriggerEdge, sin_synth::SinSynth,
    unison_osc::UnisonOsc,
};

#[macroquad::main("Lua + Macroquad Demo")]
async fn main() -> Result<()> {
    let ctx = Arc::new(StreamContext::new());
    let shared_ctx = ctx.clone();

    let last_waveform = Arc::new(Mutex::new(Vec::new()));
    let last_waveform_c = last_waveform.clone();

    thread::spawn(move || {
        let mut pb = LivePlaybackk::new();
        let shared = Arc::new(FixedSpscQueue::<f32, 512>::new());

        let mut trigger_system = OscilloscopeTrigger::<512>::new(
            shared.clone(),
            0.0,
            TriggerEdge::Rising,
            0.5,
            10.,
            44100.,
        );

        let mut synth = SinSynth(44100);
        // let mut synth = UnisonOsc::new(|s| Box::new(SawOsc(s)), 255);
        // synth.apply_distribution_factor(0.05);

        // synth.set_frequency(Note::new(PitchClass::A, 4).to_frequency().into());

        pb.build_stream(Box::new(move |data| {
            let mut e = Vec::new();
            while let Some(v) = shared_ctx.events.pop() {
                e.push(v);
            }
            synth.process(&[], &mut [data], &e);

            // for d in data.iter_mut() {
            //     *d *= 12.;
            // }

            for v in data.iter_mut() {
                trigger_system.process_sample(*v);
            }

            let mut res = Vec::with_capacity(512);
            while let Some(v) = shared.pop() {
                res.push(v);
            }

            let mut g = last_waveform.lock().unwrap();
            *g = res;
        }));

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
