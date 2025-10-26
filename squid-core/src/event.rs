use core::array;

use crate::FixedSpscQueue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event {
    pub timing: u32,
    pub data: EventData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventData {
    NoteOn { note: u8, velocity: u8 },
    NoteOff { note: u8 },

    ControlChange { control: u8, value: u8 },
    PitchBend { value: u16 },

    NoteControlChange { note: u8, control: u8, value: u8 },
    NotePitchBend { note: u8, value: u16 },
    NotePressure { note: u8, pressure: u8 },

    ProgramChange { program: u8 },
}

pub struct EventSequence<const CAPACITY: usize> {
    events: FixedSpscQueue<Event, CAPACITY>,
}

impl<const CAPACITY: usize> EventSequence<CAPACITY> {
    pub fn new() -> Self {
        Self {
            events: FixedSpscQueue::new(),
        }
    }

    pub fn push_event(&mut self, event: Event) {
        let _ = self.events.push(event).is_err();
    }

    pub fn pop_event(&mut self) -> Option<Event> {
        self.events.pop()
    }

    pub fn delete_expired_events(&mut self, current_time: u32) {
        while let Some(event) = self.events.peek() {
            if event.timing > current_time {
                break;
            }
            self.events.pop();
        }
    }
    pub fn peek_event(&self) -> Option<&Event> {
        self.events.peek()
    }
}

pub struct ChannelledEventSequence<const CHANNELS: usize, const CAPACITY: usize> {
    channels: [EventSequence<CAPACITY>; CHANNELS],
}

impl<const CHANNELS: usize, const CAPACITY: usize> ChannelledEventSequence<CHANNELS, CAPACITY> {
    pub fn new() -> Self {
        Self {
            channels: array::from_fn(|_i| EventSequence::<CAPACITY>::new()),
        }
    }

    pub fn push_event(&mut self, channel: usize, event: Event) {
        if let Some(channel) = self.channels.get_mut(channel) {
            channel.push_event(event);
        }
    }

    pub fn pop_event(&mut self, channel: usize) -> Option<Event> {
        self.channels[channel].pop_event()
    }

    pub fn delete_expired_events(&mut self, current_time: u32) {
        for channel in &mut self.channels {
            channel.delete_expired_events(current_time);
        }
    }
    pub fn peek_event(&self, channel: usize) -> Option<&Event> {
        self.channels.get(channel).and_then(|ch| ch.peek_event())
    }
}
