#![allow(dead_code)]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use super::buffer::{EventBuffer, Get, State};

struct DebouncerThread<B> {
    mutex: Arc<Mutex<B>>,
    thread: JoinHandle<()>,
    stopped: Arc<AtomicBool>,
}

impl<B> DebouncerThread<B> {
    fn new<F>(buffer: B, mut f: F) -> Self
    where
        B: Get + Send + 'static,
        F: FnMut(B::Data) + Send + 'static,
    {
        let mutex = Arc::new(Mutex::new(buffer));
        let stopped = Arc::new(AtomicBool::new(false));
        let thread = thread::spawn({
            let mutex = mutex.clone();
            let stopped = stopped.clone();
            move || {
                while !stopped.load(Ordering::Relaxed) {
                    let state = mutex.lock().unwrap().get();
                    match state {
                        State::Empty => thread::park(),
                        State::Wait(duration) => thread::sleep(duration),
                        State::Ready(data) => f(data),
                    }
                }
            }
        });
        Self {
            mutex,
            thread,
            stopped,
        }
    }

    fn stop(self) -> JoinHandle<()> {
        self.stopped.store(true, Ordering::Relaxed);
        self.thread
    }
}

/// Threaded debouncer wrapping [EventBuffer]. Accepts a common delay and a
/// callback function which is going to be called by a background thread with
/// debounced events.
pub struct EventDebouncer<T>(DebouncerThread<EventBuffer<T>>);

impl<T> EventDebouncer<T> {
    pub fn new<F>(delay: Duration, f: F) -> Self
    where
        F: FnMut(T) + Send + 'static,
        T: Send + 'static,
    {
        Self(DebouncerThread::new(EventBuffer::new(delay), f))
    }

    pub fn put(&self, data: T) {
        self.0.mutex.lock().unwrap().put(data);
        self.0.thread.thread().unpark();
    }

    pub fn clear(&self) {
        self.0.mutex.lock().unwrap().clear();
    }

    /// Signals the debouncer thread to quit and returns a
    /// [std::thread::JoinHandle] which can be `.join()`ed in the consumer
    /// thread. The common idiom is: `debouncer.stop().join().unwrap();`
    pub fn stop(self) -> JoinHandle<()> {
        self.0.stop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    #[test]
    fn event_debouncer() {
        let (tx, rx) = channel();
        let debouncer = EventDebouncer::new(Duration::from_millis(10), move |s| {
            tx.send(s).unwrap();
        });
        debouncer.put(String::from("Test1"));
        debouncer.put(String::from("Test2"));
        thread::sleep(Duration::from_millis(20));
        assert!(rx.try_iter().eq([String::from("Test2")]));
    }
}
