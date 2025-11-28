use squid_core::{
    AudioNode,
    oscillators::{Oscillator, saw_osc::SawOsc},
    process_context::ProcessContext,
};
use squid_engine::LivePlayback;

fn main() {
    let ctx = ProcessContext::default();

    let mut osc = SawOsc::new();
    osc.configure(440., 44100., None);

    let mut pb = LivePlayback::init();
    pb.start(move |out| {
        osc.process(&ctx, out);
    });

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
