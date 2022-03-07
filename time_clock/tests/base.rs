use core::time::Duration;
use std::thread;

#[test]
fn base() {
    let start = time_clock::clock();
    thread::sleep(Duration::from_millis(100));
    let end = time_clock::clock();
    println!("start: {:?}", start);
    println!("end: {:?}", end);

    assert!((end - start) / time_clock::CLOCKS_PER_MILLI > 100)
}
