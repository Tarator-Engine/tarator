//! This crate provides useful functions to the whole Tarator Engine

use std::time::Instant;

/// returns an instant with the current time (the timer)
/// 
/// # Returns
/// - Instant of current time
pub fn start_timer() -> Instant {
    Instant::now()
}

/// logs the elapsed time since the given instant and a message
/// additionally it returns a new instant
pub fn relog_timing(msg: &str, last: Instant) -> Instant {
    println!("{msg}{:?}", last.elapsed());
    start_timer()
}

/// logs the elapsed time since the given instant and a message
pub fn log_timing(msg: &str, last: Instant) {
    println!("{msg}{:?}", last.elapsed());
}