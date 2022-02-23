#![no_std]

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering::SeqCst};
use core::time::Duration;

static USED: AtomicBool = AtomicBool::new(false);
static TIME: AtomicU64 = AtomicU64::new(0);

/// Nanosecond since `app start time` or `os start time`
pub fn now() -> u64 {
    TIME.load(SeqCst)
}
clock_source::register_clock_source!(now);

pub struct Tick {}
impl Tick {
    /// returns nanosecond
    pub fn tick(&mut self, interval: Duration) -> u64 {
        let old = TIME.fetch_add(interval.as_nanos() as u64, SeqCst);
        old + interval.as_nanos() as u64
    }
}
pub fn take_tick() -> Option<Tick> {
    if USED.compare_exchange(false, true, SeqCst, SeqCst).is_ok() {
        Some(Tick {})
    } else {
        None
    }
}
