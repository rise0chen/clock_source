use core::time::Duration;

#[test]
fn base() {
    assert_eq!(async_tick::now(), 0);
    let mut tick = async_tick::take_tick().unwrap();
    tick.tick(Duration::from_nanos(100));
    assert_eq!(async_tick::now(), 100);
}
