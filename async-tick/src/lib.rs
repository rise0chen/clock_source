#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

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

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub fn auto_tick(interval: Duration) {
    use std::thread;
    use std::time::Instant;

    let mut ticker = take_tick().unwrap();
    let mut last = Instant::now();
    thread::spawn(move || loop {
        let now = Instant::now();
        ticker.tick(now.saturating_duration_since(last));
        last = now;
        thread::sleep(interval);
    });
}
