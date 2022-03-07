use std::sync::Once;
use std::time::Instant;

pub const CLOCKS_PER_NANO: u64 = 1;
pub const CLOCKS_PER_MICRO: u64 = 1_000 * CLOCKS_PER_NANO;
pub const CLOCKS_PER_MILLI: u64 = 1_000 * CLOCKS_PER_MICRO;
pub const CLOCKS_PER_SEC: u64 = 1_000 * CLOCKS_PER_MILLI;

static mut TIME: Option<Instant> = None;
static INIT: Once = Once::new();

pub fn clock() -> u64 {
    unsafe {
        INIT.call_once(|| {
            TIME = Some(Instant::now());
        });
        TIME.unwrap().elapsed().as_nanos() as u64
    }
}
