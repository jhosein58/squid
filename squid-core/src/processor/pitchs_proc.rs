use core::f32::consts::PI;

use libm::{cosf, floorf, powf, sinf};

use crate::processor::Processor;

const SEMITONE_RATIO: f32 = 1.0594635;

pub struct PitchShifterProc<const SIZE: usize> {
    shift_semitones: f32,
    rate: f32,
    buffer: [f32; SIZE],
    write_index: usize,
    main_read_head: f32,
    crossfade_read_head: f32,
    fade_pos: f32,
    fade_len_samples: f32,
}

impl<const SIZE: usize> PitchShifterProc<SIZE> {
    pub fn new(crossfade_ms: f32, sample_rate: f32) -> Self {
        assert!(SIZE > 0, "Buffer size for PitchShifterProc cannot be zero.");

        let fade_len_samples = (crossfade_ms / 1000.0 * sample_rate).max(1.0);

        Self {
            shift_semitones: 0.0,
            rate: 1.0,
            buffer: [0.0; SIZE],
            write_index: 0,
            main_read_head: 0.0,
            crossfade_read_head: 0.0,
            fade_pos: 1.0,
            fade_len_samples,
        }
    }

    pub fn set_shift_semitones(&mut self, semitones: f32) {
        self.shift_semitones = semitones;

        self.rate = powf(SEMITONE_RATIO, semitones);
    }
}

impl<const SIZE: usize> Processor for PitchShifterProc<SIZE> {
    #[inline]
    fn process(&mut self, input: f32) -> f32 {
        self.buffer[self.write_index] = input;

        let read_from_buffer = |read_head: f32, buffer: &[f32; SIZE]| -> f32 {
            let a = read_head;
            let n = SIZE as f32;

            let read_head_wrapped = a - n * floorf(a / n);

            let index_floor = floorf(read_head_wrapped);
            let fract = read_head_wrapped - index_floor;

            let i0 = index_floor as usize;
            let i1 = (i0 + 1) % SIZE;

            let s0 = buffer[i0];
            let s1 = buffer[i1];

            s0 + (s1 - s0) * fract
        };

        let main_sample = read_from_buffer(self.main_read_head, &self.buffer);
        let crossfade_sample = read_from_buffer(self.crossfade_read_head, &self.buffer);

        let fade_angle = self.fade_pos * PI * 0.5;
        let main_gain = cosf(fade_angle);
        let crossfade_gain = sinf(fade_angle);

        let output = main_sample * main_gain + crossfade_sample * crossfade_gain;

        self.main_read_head += self.rate;
        self.crossfade_read_head += self.rate;
        self.write_index = (self.write_index + 1) % SIZE;

        if self.fade_pos < 1.0 {
            self.fade_pos += 1.0 / self.fade_len_samples;
            if self.fade_pos >= 1.0 {
                self.fade_pos = 1.0;
                self.main_read_head = self.crossfade_read_head;
            }
        }
        let a = self.main_read_head;
        let n = SIZE as f32;

        let read_head_wrapped = a - n * floorf(a / n);

        let distance_to_writer = self.write_index as f32 - read_head_wrapped;

        let needs_hop = if self.rate > 1.0 {
            distance_to_writer < self.fade_len_samples
        } else {
            distance_to_writer > SIZE as f32 - self.fade_len_samples
        };

        if needs_hop && self.fade_pos >= 1.0 {
            self.fade_pos = 0.0;
            self.crossfade_read_head = self.main_read_head + (SIZE as f32 * 0.5);
        }

        output
    }

    fn reset(&mut self) {
        self.write_index = 0;
        self.buffer.fill(0.0);
        self.main_read_head = 0.0;
        self.crossfade_read_head = 0.0;
        self.fade_pos = 1.0;
    }
}
