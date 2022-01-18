use core::time::Duration;
use std::thread;

#[cfg(feature = "std")]
#[test]
fn base() {
    let last = clock_source::now();
    thread::sleep(Duration::from_millis(100));
    let now = clock_source::now();
    println!("{:?}", last.as_millis());
    println!("{:?}", now.as_millis());

    assert!(now - last > Duration::from_millis(100))
}

#[cfg(not(feature = "std"))]
#[test]
fn set() {
    static SOURCE: fn() -> Duration = || Duration::from_secs(1);
    clock_source::register_clock_source!(SOURCE);
    assert_eq!(clock_source::now(), Duration::from_secs(1));
    thread::sleep(Duration::from_millis(100));
    assert_eq!(clock_source::now(), Duration::from_secs(1));
}
