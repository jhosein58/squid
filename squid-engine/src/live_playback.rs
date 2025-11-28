use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use crate::{AudioBridge, BufferAdapter, Filler};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, StreamConfig};
use squid_core::MAX_BLOCK_SIZE;
use squid_core::process_context::FixedBuf;

pub struct LivePlayback {
    pub sample_rate: u32,
    pub num_channels: u16,
    device: cpal::Device,
    config: cpal::StreamConfig,
    stream: Option<cpal::Stream>,
}

impl LivePlayback {
    pub fn init() -> Self {
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

        Self {
            sample_rate,
            num_channels,
            device,
            config,
            stream: None,
        }
    }

    pub fn new_in_callback<T: FnMut(&mut [&mut FixedBuf]) + Send + 'static>(handle: T) -> Self {
        let mut instance = Self::init();

        let mut filler = Filler::new(handle);

        let stream = instance
            .device
            .build_output_stream(
                &instance.config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    filler.fill_stereo(data);
                },
                |err| eprintln!("an error occurred on stream: {}", err),
                None,
            )
            .unwrap();

        instance.stream = Some(stream);
        instance
    }

    pub fn play(&self) {
        if let Err(e) = self.stream.as_ref().unwrap().play() {
            eprintln!("Failed to play stream: {}", e);
        }
    }

    pub fn pause(&self) {
        if let Err(e) = self.stream.as_ref().unwrap().pause() {
            eprintln!("Failed to pause stream: {}", e);
        }
    }

    pub fn start_raw<T: FnMut(&mut [f32]) + Send + 'static>(&mut self, mut handle: T) {
        let stream = self
            .device
            .build_output_stream(
                &self.config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    handle(data);
                },
                |err| eprintln!("an error occurred on stream: {}", err),
                None,
            )
            .unwrap();

        self.stream = Some(stream);
    }

    pub fn start<T: FnMut(&mut [&mut FixedBuf]) + Send + 'static>(&mut self, mut handle: T) {
        let audio_bridge = Arc::new(AudioBridge::new());
        let audio_bridge_clone = audio_bridge.clone();

        let mut l_buf = FixedBuf::default();
        let mut r_buf = FixedBuf::default();

        let is_parked_consumer = Arc::new(AtomicBool::new(false));
        let is_parked_producer = is_parked_consumer.clone();

        let render_thread = thread::spawn(move || {
            loop {
                if audio_bridge_clone.left_channel.has_room_for(MAX_BLOCK_SIZE) {
                    handle(&mut [&mut l_buf, &mut r_buf]);

                    audio_bridge_clone.push_slice(&[&l_buf, &r_buf]);
                } else {
                    is_parked_producer.store(true, Ordering::Release);

                    let can_render_now =
                        audio_bridge_clone.left_channel.has_room_for(MAX_BLOCK_SIZE);

                    if !can_render_now {
                        thread::park();
                    }

                    is_parked_producer.store(false, Ordering::Release);
                }
            }
        });
        let producer_thread_handle = render_thread.thread().clone();

        let mut adapter = BufferAdapter::new();
        self.start_raw(move |out| {
            adapter.fill(out, &audio_bridge);
            if is_parked_consumer.load(Ordering::Relaxed) {
                producer_thread_handle.unpark();
            }
        });
    }
}
