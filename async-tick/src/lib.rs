#![no_std]

use core::time::Duration;

/// Nanosecond since `app start time` or `os start time`
pub fn now() -> u64 {
    tick_clock::now()
}

pub fn tick(interval: Duration) {
    tick_clock::tick(interval)
}
