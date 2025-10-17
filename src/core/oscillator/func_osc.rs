use std::f32::consts::TAU;

use crate::core::oscillator::Oscillator;

pub struct FuncOscillator<F>
where
    F: FnMut(f32) -> f32 + Send + Sync + 'static,
{
    phase: f32,
    phase_increment: f32,
    sample_rate: f32,
    waveform_function: F,
}

impl<F> FuncOscillator<F>
where
    F: FnMut(f32) -> f32 + Send + Sync + 'static,
{
    pub fn new(sample_rate: u32, waveform_function: F) -> Self {
        debug_assert!(sample_rate > 0);
        let mut instance = FuncOscillator {
            phase: 0.0,
            phase_increment: 0.0,
            sample_rate: sample_rate as f32,
            waveform_function,
        };
        instance.set_frequency(440.0);
        instance
    }
    fn update_phase(&mut self) {
        self.phase += self.phase_increment;
        if self.phase >= TAU {
            self.phase -= TAU;
        } else if self.phase < 0.0 {
            self.phase += TAU;
        }
    }
}

impl<F> Oscillator for FuncOscillator<F>
where
    F: FnMut(f32) -> f32 + Send + Sync + 'static,
{
    fn set_frequency(&mut self, frequency: f32) {
        self.phase_increment = (frequency * TAU) / self.sample_rate;
    }

    fn next_sample(&mut self) -> f32 {
        let output = (self.waveform_function)(self.phase);
        self.update_phase();

        output
    }

    fn reset(&mut self) {
        self.phase = 0.0;
    }
}
pub type WaveformFn = Box<dyn FnMut(f32) -> f32 + Send + Sync + 'static>;

#[macro_export]
macro_rules! define_oscillator {
    (
        $struct_name:ident,
        $waveform_fn:expr
    ) => {
        pub struct $struct_name {
            oscillator: $crate::core::oscillator::func_osc::FuncOscillator<
                $crate::core::oscillator::func_osc::WaveformFn,
            >,
        }

        impl $struct_name {
            pub fn new(sample_rate: u32) -> Self {
                let waveform_function = $waveform_fn;

                let boxed_function: $crate::core::oscillator::func_osc::WaveformFn =
                    Box::new(waveform_function);

                let oscillator = $crate::core::oscillator::func_osc::FuncOscillator::new(
                    sample_rate,
                    boxed_function,
                );

                Self { oscillator }
            }
        }

        impl $crate::core::oscillator::Oscillator for $struct_name {
            fn set_frequency(&mut self, frequency: f32) {
                self.oscillator.set_frequency(frequency);
            }

            fn next_sample(&mut self) -> f32 {
                self.oscillator.next_sample()
            }

            fn reset(&mut self) {
                self.oscillator.reset();
            }
        }
    };
}
