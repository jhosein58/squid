use crate::Event;

pub trait Plugin {
    fn channels(&self) -> u8;
    fn process(&mut self, input: &[&[f32]], output: &mut [&mut [f32]], events: &[Event]);
}
