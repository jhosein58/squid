use crate::Filler;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, StreamConfig};
use squid_core::process_context::FixedBuf;

pub struct LivePlayback {
    pub sample_rate: u32,
    pub num_channels: u16,
    device: cpal::Device,
    config: cpal::StreamConfig,
    stream: cpal::Stream,
}

impl LivePlayback {
    pub fn new<T: FnMut(&mut [&mut FixedBuf]) + Send + 'static>(handle: T) -> Self {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");

        let supported_config = device
            .default_output_config()
            .expect("no default output config");

        let target_buffer_size: u32 = 256;

        let buffer_size = match supported_config.buffer_size() {
            cpal::SupportedBufferSize::Range { min, max } => {
                let v = target_buffer_size.clamp(*min, *max);
                BufferSize::Fixed(v)
            }
            cpal::SupportedBufferSize::Unknown => BufferSize::Fixed(target_buffer_size),
        };

        let config = StreamConfig {
            channels: supported_config.channels(),
            sample_rate: supported_config.sample_rate(),
            buffer_size: buffer_size,
        };

        let sample_rate = config.sample_rate.0;
        let num_channels = config.channels;

        println!("Audio Initialized:");
        println!("  Sample Rate: {}", sample_rate);
        println!("  Requested Buffer: {:?}", buffer_size);

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

        stream.play().unwrap();

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

    pub fn new_raw<T: FnMut(&mut [f32]) + Send + 'static>(mut handle: T) -> Self {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");

        let supported_config = device
            .default_output_config()
            .expect("no default output config");

        let target_buffer_size: u32 = 256;

        let buffer_size = match supported_config.buffer_size() {
            cpal::SupportedBufferSize::Range { min, max } => {
                let v = target_buffer_size.clamp(*min, *max);
                BufferSize::Fixed(v)
            }
            cpal::SupportedBufferSize::Unknown => BufferSize::Fixed(target_buffer_size),
        };

        let config = StreamConfig {
            channels: supported_config.channels(),
            sample_rate: supported_config.sample_rate(),
            buffer_size: buffer_size,
        };

        let sample_rate = config.sample_rate.0;
        let num_channels = config.channels;

        println!("Audio Initialized:");
        println!("  Sample Rate: {}", sample_rate);
        println!("  Requested Buffer: {:?}", buffer_size);

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    handle(data);
                },
                |err| eprintln!("an error occurred on stream: {}", err),
                None,
            )
            .unwrap();

        stream.play().unwrap();

        Self {
            sample_rate,
            num_channels,
            device,
            config,
            stream,
        }
    }
}
