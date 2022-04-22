#![no_std]

cfg_if::cfg_if! {
    if #[cfg(feature = "custom")] {
        mod custom;
        use custom::CLOCK;
    } else {
        use time_clock::clock as CLOCK;
    }
}

pub static CLOCK_SOURCE: fn() -> u64 = CLOCK;

/// Nanosecond since `app start time` or `os start time`
pub fn now() -> u64 {
    CLOCK_SOURCE()
}
