#![no_std]

#[cfg(not(feature = "std"))]
mod custom;
#[cfg(feature = "std")]
mod std;

use core::time::Duration;
#[cfg(not(feature = "std"))]
use custom::CLOCK;
#[cfg(feature = "std")]
use std::CLOCK;

static CLOCK_SOURCE: fn() -> Duration = CLOCK;

pub fn now() -> Duration {
    CLOCK_SOURCE()
}
