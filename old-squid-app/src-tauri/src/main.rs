// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    cell::RefCell,
    f32::consts::TAU,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use squid_app_lib::{app_handle, AppState};
use squid_core::{
    bitcrusher_proc::BitCrusherProc,
    composite_osc::DualOsc,
    fdelay_proc::FeedbackDelayProc,
    lfo_func_mod::FunctionLFO,
    mixing::Mixing,
    primetives::{NoiseOsc, SawOsc, SinOsc, SquareOsc},
    CyclicMod, Event, EventData, FixedSpscQueue, ModRate, Modulator, Note, Oscillator, PitchClass,
    Plugin, Transport, Waveform,
};
use squid_engine::{
    oscilloscope_trigger, sin_synth::SinSynth, unison_osc::UnisonOsc, unison_synth::UnisonSynth,
    wav::Wav, LivePlaybackk, Mixer, OscilloscopeTrigger, StreamContext, TriggerEdge,
};
use tauri::{AppHandle, Emitter};

fn main() {
    let app_state = Arc::new(StreamContext::new());
    let shared = app_state.clone();

    thread::spawn(move || {
        let mut pb = LivePlaybackk::new();

        let mut mixer = Mixer::<1, 128>::new();
        let osc = Arc::new(Mutex::new(DualOsc(SinOsc(44100), SawOsc(44100))));

        let mut temp_delay = FeedbackDelayProc::<10000>::new(44100.);
        temp_delay.set_delay_ms(500.);
        temp_delay.set_feedback(0.8);
        temp_delay.set_mix(0.2);
        let delay = Arc::new(Mutex::new(temp_delay));

        let tmp_bit = BitCrusherProc::new(16., 10);
        let bitcrusher = Arc::new(Mutex::new(tmp_bit));

        mixer.add_generator(osc.clone());

        mixer.add_fx(0, bitcrusher.clone());
        mixer.add_fx(0, delay.clone());

        let note_on_event = Event {
            timing: 0,
            data: EventData::NoteOff { note: 0 },
        };
        mixer.get_sequencer_mut().push_event(0, note_on_event);

        let mut trigger_system = OscilloscopeTrigger::<512>::new(
            shared.waveform.clone(),
            0.0,
            TriggerEdge::Rising,
            0.5,
            10.,
            44100.,
        );

        let mut synth = UnisonOsc::new(|s| Box::new(SawOsc(s)), 255);
        synth.apply_distribution_factor(0.3);
        synth.set_frequency(Note::new(PitchClass::A, 4).to_frequency().into());

        pb.build_stream(Box::new(move |data| {
            let mut events = Vec::new();
            while let Some(event) = shared.events.pop() {
                events.push(event);
                //mixer.get_sequencer_mut().push_event(0, event);
            }

            synth.process(&[], &mut [data], events.as_slice());

            for d in data.iter_mut() {
                *d *= 8.;
            }

            // mixer.render_next_block(data);

            // let mut g = osc.lock().unwrap();
            // g.set_ratio(shared.f1.load(std::sync::atomic::Ordering::Relaxed) as f32 / 100.);

            // let mut gd = delay.lock().unwrap();
            // gd.set_delay_ms(
            //     (shared.f2.load(std::sync::atomic::Ordering::Relaxed) as f32) * 5. + 3.,
            // );

            // let mut gb = bitcrusher.lock().unwrap();
            // gb.set_downsampling((shared.f3.load(std::sync::atomic::Ordering::Relaxed)) as u32);

            for v in data {
                trigger_system.process_sample(*v);
            }

            let mut res = Vec::with_capacity(512);
            while let Some(v) = shared.waveform.pop() {
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
