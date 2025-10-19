use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct LivePlayback {
    pub sample_rate: u32,
    pub num_channels: u16,
    device: cpal::Device,
    config: cpal::StreamConfig,
    stream: Option<cpal::Stream>,
}

impl LivePlayback {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");
        let config = device.default_output_config().unwrap().config();

        let sample_rate = config.sample_rate.0;
        let num_channels = config.channels;

        Self {
            sample_rate,
            num_channels,
            device,
            config,
            stream: None,
        }
    }

    pub fn build_stream(&mut self, mut handle: Box<dyn FnMut(&mut [f32]) + Send>) {
        self.stream = Some(
            self.device
                .build_output_stream(
                    &self.config,
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        (handle)(data);
                    },
                    |err| eprintln!("an error occurred on stream: {}", err),
                    None,
                )
                .unwrap(),
        );
    }

    pub fn play(&mut self) {
        if let Some(ref mut s) = self.stream {
            let _ = s.play();
        }
    }
}
