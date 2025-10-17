use squid::{
    core::{
        dsp::mixing::Mixing,
        note::{Note, PitchClass},
        oscillator::{
            Oscillator,
            composite_osc::DualOsc,
            primetives::{NoiseOsc, RampOsc, SawOsc, SinOsc, SquareOsc},
        },
        processor::delay_proc::{DelayProc, Processor},
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

    fn set_and_push(osc: &mut DualOsc, wav: &mut Wav, note: Note, t: f32, delay: &mut DelayProc) {
        osc.set_frequency(note.to_frequency().into());
        for _ in 0..(44100. * t) as usize {
            let sample = osc.next_sample();
            wav.samples
                .push(Mixing::average(delay.process(sample), sample));
        }
    }

    let sr = 44100;
    let mut osc = DualOsc::new(Box::new(SinOsc::new(sr)), Box::new(NoiseOsc::new(sr)), 0.2);
    let mut res = Wav::new(WavSpec {
        audio_format: 1,
        num_channels: 1,
        sample_rate: sr,
        bits_per_sample: 16,
    });

    let mut delay = DelayProc::new(100.0, 200.0, sr as f32);

    let s = Wav::from_path("bk.wav").unwrap();
    for i in 0..44100 * 2 {
        let sample = s.samples.get(i).unwrap_or(&0.0).to_owned();
        res.samples
            .push(Mixing::average(delay.process(sample), sample));
    }

    // set_and_push(
    //     &mut osc,
    //     &mut res,
    //     Note::new(PitchClass::C, 5),
    //     0.2,
    //     &mut delay,
    // );
    // set_and_push(
    //     &mut osc,
    //     &mut res,
    //     Note::new(PitchClass::D, 5),
    //     0.2,
    //     &mut delay,
    // );
    // set_and_push(
    //     &mut osc,
    //     &mut res,
    //     Note::new(PitchClass::E, 5),
    //     0.2,
    //     &mut delay,
    // );
    // set_and_push(
    //     &mut osc,
    //     &mut res,
    //     Note::new(PitchClass::F, 5),
    //     0.2,
    //     &mut delay,
    // );
    // set_and_push(
    //     &mut osc,
    //     &mut res,
    //     Note::new(PitchClass::G, 5),
    //     0.2,
    //     &mut delay,
    // );
    // set_and_push(
    //     &mut osc,
    //     &mut res,
    //     Note::new(PitchClass::A, 5),
    //     0.2,
    //     &mut delay,
    // );
    // set_and_push(
    //     &mut osc,
    //     &mut res,
    //     Note::new(PitchClass::B, 5),
    //     0.2,
    //     &mut delay,
    // );
    // set_and_push(
    //     &mut osc,
    //     &mut res,
    //     Note::new(PitchClass::C, 6),
    //     2.,
    //     &mut delay,
    // );

    res.write_to_path("test.wav").unwrap();
}
