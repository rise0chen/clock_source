use core::time::Duration;

#[test]
fn base() {
    assert_eq!(tick_clock::now(), 0);
    tick_clock::tick(Duration::from_nanos(100));
    assert_eq!(tick_clock::now(), 100);
}
