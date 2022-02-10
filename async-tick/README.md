# async-tick

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
async-tick = "*"
```

```rust
use core::time::Duration;
async_tick::tick();
let now: u64 = async_tick::now();

async_tick::sleep(Duration::from_secs(1)).await;

let interval = async_tick::interval(Duration::from_secs(1));
while let Some(now) = interval.await{
    // task
}
```
