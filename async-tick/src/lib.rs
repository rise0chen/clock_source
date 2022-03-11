#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod interval;
mod sleep;
mod timeout;
mod waiter;

use core::time::Duration;
use core::u64::MAX;
pub use interval::*;
pub use sleep::*;
pub use timeout::*;
use waiter::Waiter;

const WAITS_NUM: usize = 64;
static WAITS: Waiter<WAITS_NUM> = Waiter::new();

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
        if let Some(next) = WAITS.take_next() {
            if now < next {
                WAITS.set_next(next);
                return now;
            }
        } else {
            return now;
        }
        let mut min = MAX;
        WAITS.retain(|entity| {
            if entity.val <= now {
                entity.waker.wake_by_ref();
                false
            } else {
                min = min.min(entity.val);
                true
            }
        });
        WAITS.set_next(min);
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
    thread::Builder::new()
        .name("async-tick".into())
        .spawn(move || loop {
            let now = Instant::now();
            ticker.tick(now.saturating_duration_since(last));
            last = now;
            thread::sleep(interval);
        })
        .unwrap();
}
