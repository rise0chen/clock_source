#![no_std]

cfg_if::cfg_if! {
    if #[cfg(any(feature = "custom", not(feature = "std")))] {
        mod custom;
        use custom::CLOCK;
    } else {
        mod std;
        use std::CLOCK;
    }
}

static CLOCK_SOURCE: fn() -> u64 = CLOCK;

/// Nanosecond since `app start time` or `os start time`
pub fn now() -> u64 {
    CLOCK_SOURCE()
}
