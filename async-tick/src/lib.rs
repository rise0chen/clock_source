#![no_std]

mod interval;
mod sleep;
mod timeout;

use async_ach_waker::WakerPool;
use core::time::Duration;
pub use interval::*;
pub use sleep::*;
pub use timeout::*;

const WAITS_NUM: usize = 64;
static WAITS: WakerPool<u64, WAITS_NUM> = WakerPool::new();

/// Nanosecond since `app start time` or `os start time`
pub fn now() -> u64 {
    tick_clock::now()
}

pub struct Tick {
    tick: tick_clock::Tick,
}
impl Tick {
    /// returns nanosecond
    pub fn tick(&mut self, interval: Duration) -> u64 {
        let now = self.tick.tick(interval);
        WAITS.retain(|entity| {
            if entity.val <= now {
                entity.waker.wake_by_ref();
                false
            } else {
                true
            }
        });
        now
    }
}
pub fn take_tick() -> Option<Tick> {
    if let Some(tick) = tick_clock::take_tick() {
        Some(Tick { tick })
    } else {
        None
    }
}
