use crate::processor::Processor;

impl<P1, P2> Processor for (P1, P2)
where
    P1: Processor,
    P2: Processor,
{
    #[inline]
    fn process(&mut self, input: f32) -> f32 {
        let signal = self.0.process(input);
        self.1.process(signal)
    }

    fn reset(&mut self) {
        self.0.reset();
        self.1.reset();
    }
}

impl<P1, P2, P3> Processor for (P1, P2, P3)
where
    P1: Processor,
    P2: Processor,
    P3: Processor,
{
    #[inline]
    fn process(&mut self, input: f32) -> f32 {
        let signal = self.0.process(input);
        let signal = self.1.process(signal);
        self.2.process(signal)
    }

    fn reset(&mut self) {
        self.0.reset();
        self.1.reset();
        self.2.reset();
    }
}
