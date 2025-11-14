use std::time::{Duration, Instant};

use squid_core::{
    AudioNode, FloatVector, Fv, Rand,
    effects::gain_fx::GainFx,
    oscillators::{Oscillator, saw_osc::SawOsc, sin_mono_osc::SinOsc},
    process_context::{FixedBuf, ProcessContext},
    vecblock::{VecAdd, VecBlock},
};
use squid_engine::{LivePlayback, oscillators::unison_osc::UnisonOsc};

fn main() {
    let mut sin_osc = UnisonOsc::new(Box::new(SawOsc::new()));
    sin_osc.configure(55., 44100., None);
    sin_osc.set_unison(255);
    sin_osc.detune(10.);

    let mut osc = SawOsc::new();
    osc.configure(1., 44100., None);

    let mut gain = GainFx;

    let mut mod_freq = 0.5;
    let pb = LivePlayback::new(Box::new(move |ctx, out| {
        let mut gains = FixedBuf::default();
        osc.process(ctx, &mut [&mut gains, &mut FixedBuf::default()]);
        mod_freq += 0.001;
        osc.configure(mod_freq, 44100., None);

        for i in gains.data.iter_mut() {
            let mapped = (*i + 1.0) * 0.5;
            *i = mapped * -30.;
        }

        let mut lc = FixedBuf::default();
        let mut rc = FixedBuf::default();

        sin_osc.process(ctx, &mut [&mut lc, &mut rc]);
        let gain_ctx = ProcessContext {
            events: &[],
            sample_rate: 44100.,
            inputs: &[&lc, &rc, &gains],
        };
        gain.process(&gain_ctx, out);
    }));

    //pb.play();

    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}
