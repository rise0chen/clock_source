#![no_std]

use core::sync::atomic::{AtomicU64, Ordering::SeqCst};
use core::time::Duration;

static TIME: AtomicU64 = AtomicU64::new(0);

/// Nanosecond since `app start time` or `os start time`
pub fn now() -> u64 {
    TIME.load(SeqCst)
}
clock_source::register_clock_source!(now);

pub fn tick(interval: Duration) {
    TIME.fetch_add(interval.as_nanos() as u64, SeqCst);
}
