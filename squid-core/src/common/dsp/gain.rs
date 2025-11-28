// use libm::{log2f, log10f, powf};

// #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
// pub struct Gain {
//     amplitude: f32,
// }

// impl Gain {
//     pub const UNITY: Gain = Gain { amplitude: 1.0 };

//     pub const SILENCE: Gain = Gain { amplitude: 0.0 };

//     pub fn from_amplitude(amplitude: f32) -> Self {
//         Self { amplitude }
//     }

//     pub fn from_db(db: f32) -> Self {
//         if db <= -96.0 {
//             return Self::SILENCE;
//         }

//         let amplitude = powf(10.0, db / 20.0);
//         Self { amplitude }
//     }

//     pub fn from_multiplier(multiplier: f32) -> Self {
//         Self {
//             amplitude: multiplier.max(0.0),
//         }
//     }

//     pub fn from_perceptual(factor: f32) -> Self {
//         if factor <= 0.0 {
//             return Self::SILENCE;
//         }

//         let db_change = 10.0 * log2f(factor);

//         Self::from_db(db_change)
//     }

//     pub fn as_amplitude(&self) -> f32 {
//         self.amplitude
//     }

//     pub fn as_db(&self) -> f32 {
//         if self.amplitude <= 0.0 {
//             return -f32::INFINITY;
//         }
//         20.0 * log10f(self.amplitude)
//     }

//     pub fn as_perceptual(&self) -> f32 {
//         if self.amplitude <= 0.0 {
//             return 0.0;
//         }

//         let db = self.as_db();

//         powf(2., db / 10.)
//     }
// }

// impl From<f32> for Gain {
//     fn from(multiplier: f32) -> Self {
//         Gain::from_multiplier(multiplier)
//     }
// }
