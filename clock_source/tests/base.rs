use core::time::Duration;
use std::thread;

#[cfg(not(target_os = "none"))]
#[test]
fn base() {
    let last = clock_source::now();
    thread::sleep(Duration::from_millis(100));
    let now = clock_source::now();
    println!("{:?}", last);
    println!("{:?}", now);

    assert!(now - last > Duration::from_millis(100).as_nanos() as u64)
}

#[cfg(feature = "custom")]
#[test]
fn set() {
    static SOURCE: fn() -> u64 = || 1;
    clock_source::register_clock_source!(SOURCE);
    assert_eq!(clock_source::now(), 1);
    thread::sleep(Duration::from_millis(100));
    assert_eq!(clock_source::now(), 1);
}
