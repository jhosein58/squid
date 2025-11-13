use crate::Filler;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use squid_core::process_context::{FixedBuf, ProcessContext};

pub struct LivePlayback {
    pub sample_rate: u32,
    pub num_channels: u16,
    device: cpal::Device,
    config: cpal::StreamConfig,
    stream: cpal::Stream,
}

impl LivePlayback {
    pub fn new(
        handle: Box<
            dyn for<'a, 'b> FnMut(&'a ProcessContext<'b>, &mut [&mut FixedBuf]) + Send + 'static,
        >,
    ) -> Self {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");
        let config = device.default_output_config().unwrap().config();

        let sample_rate = config.sample_rate.0;
        let num_channels = config.channels;

        let mut filler = Filler::new(handle);

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    filler.fill_stereo(data);
                },
                |err| eprintln!("an error occurred on stream: {}", err),
                None,
            )
            .unwrap();
        Self {
            sample_rate,
            num_channels,
            device,
            config,
            stream,
        }
    }

    pub fn play(&self) {
        if let Err(e) = self.stream.play() {
            eprintln!("Failed to play stream: {}", e);
        }
    }

    pub fn pause(&self) {
        if let Err(e) = self.stream.pause() {
            eprintln!("Failed to pause stream: {}", e);
        }
    }
}
