pub struct Rand {
    state: u32,
}

impl Rand {
    pub fn new(mut seed: u32) -> Self {
        if seed == 0 {
            seed = 1;
        }
        Self { state: seed }
    }

    pub fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    #[inline]
    pub fn next_f32(&mut self) -> f32 {
        self.next_u32() as f32 / 4294967296.0
    }

    #[inline]
    pub fn next_f32_bipolar(&mut self) -> f32 {
        self.next_f32() * 2.0 - 1.0
    }

    pub fn next_range_u32(&mut self, min: u32, max: u32) -> u32 {
        assert!(min <= max, "min must be less than or equal to max");

        let range_size = (max - min) + 1;

        if range_size == 0 {
            return self.next_u32();
        }

        (self.next_u32() % range_size) + min
    }

    pub fn next_range_usize(&mut self, min: usize, max: usize) -> usize {
        assert!(min <= max, "min must be less than or equal to max");
        let range_size = (max - min) + 1;

        (self.next_u32() as usize % range_size) + min
    }
    pub fn next_range_f32(&mut self, min: f32, max: f32) -> f32 {
        assert!(min < max, "min must be less than max");

        let random_zero_to_one = self.next_f32();

        random_zero_to_one * (max - min) + min
    }
}
