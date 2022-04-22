#![no_std]

use core::sync::atomic::{AtomicBool, Ordering::SeqCst};
use core::time::Duration;

static USED: AtomicBool = AtomicBool::new(false);
static mut TIME: u64 = 0;

/// Nanosecond since `app start time` or `os start time`
pub fn now() -> u64 {
    unsafe { TIME }
}
clock_source::register_clock_source!(now);

pub struct Tick {}
impl Tick {
    /// returns nanosecond
    pub fn tick(&mut self, interval: Duration) -> u64 {
        unsafe {
            TIME += interval.as_nanos() as u64;
            TIME
        }
    }
}
pub fn take_tick() -> Option<Tick> {
    if USED.compare_exchange(false, true, SeqCst, SeqCst).is_ok() {
        Some(Tick {})
    } else {
        None
    }
}
