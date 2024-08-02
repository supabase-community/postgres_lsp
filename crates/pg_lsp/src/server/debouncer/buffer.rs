use std::collections::VecDeque;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct Event<T> {
    item: T,
    release_at: Instant,
}

/// Current state of the debouncing buffer returned from [Get::get()]:
///
/// - `Ready(T)` when the event is ready to be delivered after the timeout
///   (moves data out of the buffer)
/// - `Wait(Duration)` indicates how much time is left until `Ready`
/// - `Empty` means the buffer is empty
#[derive(Debug, PartialEq, Eq)]
pub enum State<T> {
    Ready(T),
    Wait(Duration),
    Empty,
}

/// Common interface for getting events out of debouncing buffers.
pub trait Get: Sized {
    type Data;

    /// Attemtps to get the next element out of a buffer. If an element is
    /// [State::Ready] it's removed from the buffer.
    fn get(&mut self) -> State<Self::Data>;
}

/// Debouncing buffer with a common delay for all events. Accepts events via
/// [EventBuffer::put()] which tracks the time of events and de-duplicates them
/// against the current buffer content. Subsequent call to [EventBuffer::get
/// ()] which returns the [State] of the buffer.
pub struct EventBuffer<T> {
    delay: Duration,
    events: VecDeque<Event<T>>,
}

impl<T> EventBuffer<T> {
    pub fn new(delay: Duration) -> EventBuffer<T> {
        EventBuffer {
            delay,
            events: VecDeque::new(),
        }
    }

    pub fn put(&mut self, item: T) {
        let time = Instant::now();
        self.events.retain(|e| e.release_at <= time);
        self.events.push_back(Event {
            item,
            release_at: time + self.delay,
        });
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

impl<T> Get for EventBuffer<T> {
    type Data = T;

    fn get(&mut self) -> State<T> {
        let time = Instant::now();
        match self.events.get(0) {
            None => State::Empty,
            Some(e) if e.release_at > time => State::Wait(e.release_at - time),
            Some(_) => State::Ready(self.events.pop_front().unwrap().item),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn wait() {
        let mut debouncer = EventBuffer::new(Duration::from_millis(20));
        debouncer.put(1);
        assert!(matches!(debouncer.get(), State::Wait(_)));
        sleep(Duration::from_millis(10));
        assert!(matches!(debouncer.get(), State::Wait(_)));
        sleep(Duration::from_millis(10));
        assert!(matches!(debouncer.get(), State::Ready(_)));
    }
}
