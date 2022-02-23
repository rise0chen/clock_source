use core::time::Duration;

#[test]
fn base() {
    assert_eq!(tick_clock::now(), 0);
    let mut tick = tick_clock::take_tick().unwrap();
    tick.tick(Duration::from_nanos(100));
    assert_eq!(tick_clock::now(), 100);
}
