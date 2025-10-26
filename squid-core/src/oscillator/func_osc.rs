use crate::oscillator::Oscillator;
use core::f32::consts::TAU;
use libm::floorf;

pub struct FuncOscillator<F>
where
    F: Fn(f32) -> f32 + Send + Sync + 'static,
{
    phase: f32,
    phase_increment: f32,
    sample_rate: f32,
    waveform_function: F,
}

impl<F> FuncOscillator<F>
where
    F: Fn(f32) -> f32 + Send + Sync + 'static,
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
        self.phase = self.phase + self.phase_increment;
        self.phase -= floorf(self.phase);
    }
}

impl<F> Oscillator for FuncOscillator<F>
where
    F: Fn(f32) -> f32 + Send + Sync + 'static,
{
    fn set_frequency(&mut self, frequency: f32) {
        let freq = if frequency < 0.0 { 0.0 } else { frequency };
        self.phase_increment = freq / self.sample_rate;
    }

    fn next_sample(&mut self) -> f32 {
        let phase_rads = self.phase * TAU;
        let output = (self.waveform_function)(phase_rads);
        self.update_phase();
        output
    }

    fn reset(&mut self) {
        self.phase = 0.0;
    }

    fn get_phase(&self) -> f32 {
        self.phase
    }

    fn set_phase(&mut self, phase: f32) {
        self.phase = phase - floorf(phase);
    }
}
#[macro_export]
macro_rules! define_oscillator {
    (
        $struct_name:ident,
        $waveform_fn:expr
    ) => {
        pub struct $struct_name<F>
        where
            F: Fn(f32) -> f32 + Send + Sync + 'static,
        {
            oscillator: $crate::oscillator::func_osc::FuncOscillator<F>,
        }

        impl<F> $struct_name<F>
        where
            F: Fn(f32) -> f32 + Send + Sync + 'static,
        {
            pub fn set_frequency(&mut self, frequency: f32) {
                self.oscillator.set_frequency(frequency);
            }

            pub fn next_sample(&mut self) -> f32 {
                self.oscillator.next_sample()
            }

            pub fn reset(&mut self) {
                self.oscillator.reset();
            }

            pub fn get_phase(&self) -> f32 {
                self.oscillator.get_phase()
            }

            pub fn set_phase(&mut self, phase: f32) {
                self.oscillator.set_phase(phase);
            }
        }

        impl<F> $crate::oscillator::Oscillator for $struct_name<F>
        where
            F: Fn(f32) -> f32 + Send + Sync + 'static,
        {
            fn set_frequency(&mut self, frequency: f32) {
                self.oscillator.set_frequency(frequency);
            }

            fn next_sample(&mut self) -> f32 {
                self.oscillator.next_sample()
            }

            fn reset(&mut self) {
                self.oscillator.reset();
            }

            fn get_phase(&self) -> f32 {
                self.oscillator.get_phase()
            }

            fn set_phase(&mut self, phase: f32) {
                self.oscillator.set_phase(phase);
            }
        }

        #[allow(non_snake_case)]
        pub fn $struct_name(
            sample_rate: u32,
        ) -> $struct_name<impl Fn(f32) -> f32 + Send + Sync + 'static> {
            let oscillator =
                $crate::oscillator::func_osc::FuncOscillator::new(sample_rate, $waveform_fn);
            $struct_name { oscillator }
        }
    };
}
