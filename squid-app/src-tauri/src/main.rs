// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    f32::consts::TAU,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use squid_app_lib::AppState;
use squid_core::{
    composite_osc::DualOsc,
    lfo_func_mod::FunctionLFO,
    primetives::{NoiseOsc, SawOsc, SinOsc, SquareOsc},
    CyclicMod, ModRate, Modulator, Note, Oscillator, PitchClass, Transport, Waveform,
};
use squid_engine::LivePlaybackk;

fn main() {
    let app_state = Arc::new(Mutex::new(AppState { freq: Vec::new() }));
    let shared = app_state.clone();

    thread::spawn(move || {
        let mut pb = LivePlaybackk::new();

        let mut sine_oscillator = DualOsc(DualOsc(SquareOsc(44100), NoiseOsc(300)), SawOsc(44100));
        sine_oscillator.set_frequency(Note::new(PitchClass::D, 4).to_frequency().into());

        let mut tr = Transport::new(44100., 120.);

        struct LFOWaveform;
        impl Waveform for LFOWaveform {
            fn process(&self, phase: f32) -> f32 {
                1. - phase
            }
        }
        let mut lfo = FunctionLFO::new(44100., LFOWaveform);
        lfo.set_rate(ModRate::Hz(2.));

        pb.build_stream(Box::new(move |data| {
            let num_frames = data.len() / pb.num_channels as usize;

            for frame_index in 0..num_frames {
                tr.tick();
                lfo.tick(&mut tr);
                let value = sine_oscillator.next_sample() * lfo.value();

                for channel_index in 0..pb.num_channels {
                    let sample_index =
                        frame_index * pb.num_channels as usize + channel_index as usize;
                    data[sample_index] = value;
                }
            }

            let mut guard = shared.lock().unwrap();
            (*guard).freq = data.to_vec();
        }));

        loop {
            std::thread::sleep(Duration::from_secs(1));
        }
    });

    squid_app_lib::run(app_state);
}
