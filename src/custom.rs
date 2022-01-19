#[macro_export]
macro_rules! register_clock_source {
    ($path:path) => {
        #[no_mangle]
        extern "C" fn __RUST_CRATE_CLOCK_SOURCE() -> u64 {
            $path()
        }
    };
}
extern "C" {
    fn __RUST_CRATE_CLOCK_SOURCE() -> u64;
}

pub const CLOCK: fn() -> u64 = || unsafe { __RUST_CRATE_CLOCK_SOURCE() };
