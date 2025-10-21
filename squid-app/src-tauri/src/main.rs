// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    f32::consts::TAU,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use squid_app_lib::{app_handle, AppState};
use squid_core::{
    composite_osc::DualOsc,
    lfo_func_mod::FunctionLFO,
    mixing::Mixing,
    primetives::{NoiseOsc, SawOsc, SinOsc, SquareOsc},
    CyclicMod, FixedSpscQueue, ModRate, Modulator, Note, Oscillator, PitchClass, Transport,
    Waveform,
};
use squid_engine::{wav::Wav, LivePlaybackk, OscilloscopeTrigger, TriggerEdge};
use tauri::{AppHandle, Emitter};

fn main() {
    let sample = Wav::from_path("bk.wav").unwrap();
    let mut sample_idx: usize = 0;
    let app_state = Arc::new(FixedSpscQueue::<f32, 512>::new());
    let shared = app_state.clone();

    thread::spawn(move || {
        let mut pb = LivePlaybackk::new();

        let mut sine_oscillator = SinOsc(44100);
        sine_oscillator.set_frequency(110.);
        let mut sine_oscillator2 = SinOsc(44100);
        sine_oscillator2.set_frequency(1300.);

        let mut tr = Transport::new(44100., 120.);
        tr.play();
        struct LFOWaveform;
        impl Waveform for LFOWaveform {
            fn process(&self, phase: f32) -> f32 {
                phase
            }
        }
        let mut lfo = FunctionLFO::new(44100., LFOWaveform);
        lfo.set_rate(ModRate::Hz(0.5));

        let mut trigger_system =
            OscilloscopeTrigger::<512>::new(shared.clone(), 0.0, TriggerEdge::Rising);

        pb.build_stream(Box::new(move |data| {
            let num_frames = data.len() / pb.num_channels as usize;

            for frame_index in 0..num_frames {
                tr.tick();
                lfo.tick(&mut tr);
                let value = if let Some(sample) = sample.samples.get(sample_idx) {
                    sample_idx += 1;
                    *sample
                } else {
                    sample_idx = 0;
                    0.0
                };

                trigger_system.process_sample(value);
                for channel_index in 0..pb.num_channels {
                    let sample_index =
                        frame_index * pb.num_channels as usize + channel_index as usize;
                    data[sample_index] = value;
                }
            }

            let mut res = Vec::with_capacity(512);
            while let Some(v) = shared.pop() {
                res.push(v);
            }

            if let Some(app) = app_handle() {
                app.emit("oscilloscope_waveform", res).unwrap();
            }
        }));

        loop {
            std::thread::sleep(Duration::from_secs(1));
        }
    });

    squid_app_lib::run(app_state);
}
