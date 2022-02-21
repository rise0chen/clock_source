use core::time::Duration;
use etime::{expect_time, Etime};
use std::thread;

#[test]
fn base() {
    let mut etime = Etime::new();
    etime.tic();
    thread::sleep(Duration::from_secs(1));
    let elapsed = etime.toc();
    expect_time(
        elapsed,
        Duration::ZERO..Duration::from_secs(2),
        |_| unreachable!(),
    );
    expect_time(elapsed, Duration::ZERO..Duration::from_secs(1), |e| {
        println!("failed: {:?}", e);
    });
}
