use core::time::Duration;

#[macro_export]
macro_rules! register_clock_source {
    ($path:path) => {
        #[no_mangle]
        static __RUST_CLOCK_SOURCE__: fn() -> Duration = $path;
    };
}
extern "Rust" {
    static __RUST_CLOCK_SOURCE__: fn() -> Duration;
}

pub const CLOCK: fn() -> Duration = || unsafe { __RUST_CLOCK_SOURCE__() };
