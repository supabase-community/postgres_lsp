//! Data structures and helpers for *debouncing* a stream of events: removing
//! duplicate events occurring closely in time.

pub mod buffer;
pub mod thread;

pub use thread::EventDebouncer;
