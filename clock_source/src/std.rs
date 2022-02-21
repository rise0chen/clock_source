extern crate std;
use std::sync::Once;
use std::time::Instant;

static mut TIME: Option<Instant> = None;
static INIT: Once = Once::new();

pub const CLOCK: fn() -> u64 = || unsafe {
    INIT.call_once(|| {
        TIME = Some(Instant::now());
    });
    TIME.unwrap().elapsed().as_nanos() as u64
};
