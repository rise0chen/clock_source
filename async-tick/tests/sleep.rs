use core::time::Duration;
use futures_executor::block_on;
use std::time::Instant;

#[test]
fn base() {
    let interval = Duration::from_millis(1);
    async_tick::auto_tick(interval);

    block_on(async {
        let ten_millis = Duration::from_millis(10);
        let now = Instant::now();
        async_tick::sleep(ten_millis).await;
        println!("{:?}", now.elapsed());
        assert!(now.elapsed() >= ten_millis);
    });
}
