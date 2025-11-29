#![feature(portable_simd)]
use std::simd::{Mask, Simd};

use squid_core::{
    AudioNode,
    dsp::{
        filters::sv_filter::ScalarSvf,
        mod_core::adsr_mod_source::{AdsrModSource, calculate_coefficient},
    },
    oscillators::{Oscillator, saw_osc::SawOsc},
    process_context::ProcessContext,
};
use squid_engine::{LivePlayback, unison_osc::UnisonOsc};

fn main() {
    let ctx = ProcessContext::default();

    let mut osc = SawOsc::new();
    osc.configure(440., 44100., None);

    let mut pb = LivePlayback::init();

    let mut adsr = AdsrModSource::new();
    let sample_rate = 44100.0;

    let attack_coeff = calculate_coefficient(20.0, sample_rate);
    let decay_coeff = calculate_coefficient(1.0, sample_rate);
    let release_coeff = calculate_coefficient(100.0, sample_rate);

    adsr.set_parameters(attack_coeff, decay_coeff, release_coeff, 1.);
    adsr.note_on(Mask::splat(true));

    let mut test = 0;

    pb.start(move |out| {
        osc.process(&ctx, out);

        test += 1;
        if test > 500 {
            adsr.note_off(Mask::splat(true));
        }

        out[0].data.map_in_place(|c| c * adsr.process());
    });

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
