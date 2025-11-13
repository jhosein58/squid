use std::time::Duration;

use squid_core::{
    AudioNode, FloatVector, Fv, Rand,
    oscillators::{Oscillator, saw_osc::SawOsc, sin_mono_osc::SinOsc},
    vecblock::{VecAdd, VecBlock},
};
use squid_engine::{LivePlayback, oscillators::unison_osc::UnisonOsc};

fn main() {
    // let mut sin_osc = UnisonOsc::new(Box::new(SawOsc::new()));
    // sin_osc.configure(440., 44100., None);
    // sin_osc.set_unison(32);
    // sin_osc.detune(100.);

    // let pb = LivePlayback::new(Box::new(move |ctx, out| {
    //     for _ in 0..1 {
    //         sin_osc.process(ctx, out);
    //     }
    // }));

    // //pb.play();

    // loop {
    //     std::thread::sleep(Duration::from_secs(1));
    // }

    let a = FloatVector::splat(1.);
    let b = FloatVector::splat(1.);
    let c = a + b;
    dbg!(c.to_array());
}
