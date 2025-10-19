use squid_core::{
    CyclicMod, ModRate, Modulator, Note, Oscillator, Processor, Transport, Waveform,
    bitcrusher_proc::BitCrusherProc,
    composite_osc::DualOsc,
    fdelay_proc::FeedbackDelayProc,
    hclip_proc::HardClipProc,
    lfo_func_mod::FunctionLFO,
    primetives::{NoiseOsc, SinOsc},
    saturator_proc::SaturatorProc,
};
use squid_engine::formats::wav::{Wav, WavSpec};

fn main() {
    //let mut pb = LivePlayback::new();

    // let mut sine_oscillator = NoiseOsc::new(44100);
    // sine_oscillator.set_frequency(Note::new(PitchClass::D, 5).to_frequency().into());

    // pb.build_stream(Box::new(move |data| {
    //     let num_frames = data.len() / pb.num_channels as usize;

    //     for frame_index in 0..num_frames {
    //         let value = sine_oscillator.next_sample();

    //         for channel_index in 0..pb.num_channels {
    //             let sample_index = frame_index * pb.num_channels as usize + channel_index as usize;
    //             data[sample_index] = value;
    //         }
    //     }
    // }));

    // loop {
    //     std::thread::sleep(Duration::from_secs(1));
    // }

    // let sr = 44100;
    // let mut osc = DualOsc(SinOsc(sr), NoiseOsc(sr));
    // osc.set_frequency(
    //     Note::new(squid_core::PitchClass::E, 1)
    //         .to_frequency()
    //         .into(),
    // );
    // osc.set_ratio(0.15);

    // let mut res = Wav::new(WavSpec {
    //     audio_format: 1,
    //     num_channels: 1,
    //     sample_rate: sr,
    //     bits_per_sample: 16,
    // });

    // struct W1;
    // impl Waveform for W1 {
    //     fn process(&self, phase: f32) -> f32 {
    //         1.0 - phase
    //     }
    // }

    // struct W2;
    // impl Waveform for W2 {
    //     fn process(&self, phase: f32) -> f32 {
    //         phase
    //     }
    // }

    // let mut env = FunctionLFO::new(sr as f32, W1);
    // let mut lfo = FunctionLFO::new(sr as f32, W2);

    // env.set_rate(ModRate::Hz(4.0));
    // lfo.set_rate(ModRate::Hz(0.12));
    // let mut tr = Transport::new(sr as f32, 60.);
    // tr.play();

    // let mut d = BitCrusherProc::new(16., 20);
    // for _ in 0..44100 * 30 {
    //     tr.tick();
    //     env.tick(&mut tr);
    //     res.samples.push(d.process(osc.next_sample() * env.value()));
    // }
    // res.write_to_path("test.wav").unwrap();

    let sample_rate = 44100;

    let mut sine_osc = SinOsc(sample_rate);
    sine_osc.set_frequency(440.0);

    let mut final_output = Wav(WavSpec::cd_mono());

    for _ in 0..sample_rate * 3 {
        final_output.samples.push(sine_osc.next_sample());
    }

    final_output.write_to_path("output.wav").unwrap();
}
