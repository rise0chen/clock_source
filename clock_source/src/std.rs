extern crate std;
use std::time::Instant;

static mut TIME: Option<Instant> = None;

pub const CLOCK: fn() -> u64 = || unsafe {
    if TIME.is_none() {
        TIME = Some(Instant::now())
    }
    TIME.unwrap().elapsed().as_nanos() as u64
};
