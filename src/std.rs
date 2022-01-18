extern crate std;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub const CLOCK: fn() -> Duration = || {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH).expect("Time went backwards")
};
