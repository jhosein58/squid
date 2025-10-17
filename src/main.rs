use squid::{
    core::{
        note::{Note, PitchClass},
        oscillator::{
            Oscillator,
            primetives::{NoiseOsc, RampOsc, SawOsc, SinOsc, SquareOsc},
        },
    },
    formats::wav::{Wav, WavSpec},
    live_playback::LivePlayback,
};
use std::time::Duration;

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

    let sr = 44100;
    let mut osc = SawOsc::new(sr);
    osc.set_frequency(Note::new(PitchClass::A, 4).to_frequency().into());

    let mut res = Wav::new(WavSpec {
        audio_format: 1,
        num_channels: 1,
        sample_rate: sr,
        bits_per_sample: 16,
    });

    for _ in 0..sr * 3 {
        res.samples.push(osc.next_sample());
    }

    res.write_to_path("Saw.wav").unwrap();
}
