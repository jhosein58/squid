use libm::floor;

#[derive(Debug, Clone, Copy)]
pub struct TransportPosition {
    pub current_sample: u64,
    pub beat: f64,
    pub bar: f64,
    pub phase_in_beat: f64,
    pub phase_in_bar: f64,
}

pub struct Transport {
    sample_rate: f32,
    bpm: f32,
    is_playing: bool,
    current_sample: u64,
    samples_per_beat: f64,
    beats_per_bar: f64,
}

impl Transport {
    pub fn new(sample_rate: f32, bpm: f32) -> Self {
        let mut transport = Self {
            sample_rate,
            bpm: 0.0,
            is_playing: false,
            current_sample: 0,
            samples_per_beat: 0.0,
            beats_per_bar: 4.0,
        };
        transport.set_bpm(bpm);
        transport
    }

    pub fn tick(&mut self) {
        if self.is_playing {
            self.current_sample += 1;
        }
    }

    pub fn play(&mut self) {
        self.is_playing = true;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
    }

    pub fn rewind(&mut self) {
        self.current_sample = 0;
    }

    pub fn set_bpm(&mut self, new_bpm: f32) {
        self.bpm = new_bpm.max(1.0);
        let seconds_per_beat = 60.0 / self.bpm as f64;
        self.samples_per_beat = seconds_per_beat * self.sample_rate as f64;
    }

    pub fn bpm(&self) -> f32 {
        self.bpm
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn get_position(&self) -> TransportPosition {
        if !self.is_playing || self.samples_per_beat == 0.0 {
            return TransportPosition {
                current_sample: self.current_sample,
                beat: 0.0,
                bar: 0.0,
                phase_in_beat: 0.0,
                phase_in_bar: 0.0,
            };
        }

        let current_sample_f64 = self.current_sample as f64;

        let beat = current_sample_f64 / self.samples_per_beat;
        let samples_per_bar = self.samples_per_beat * self.beats_per_bar;
        let bar = current_sample_f64 / samples_per_bar;

        TransportPosition {
            current_sample: self.current_sample,
            beat,
            bar,
            phase_in_beat: floor(beat),
            phase_in_bar: floor(bar),
        }
    }
}
