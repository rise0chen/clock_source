# time-clock

Returns the processor time consumed by the program.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
time-clock = "*"
```

```rust
use core::time::Duration;
use std::thread;
let start = time_clock::clock();
thread::sleep(Duration::from_millis(100));
let end = time_clock::clock();
println!("start: {:?}", start);
println!("end: {:?}", end);

assert!((end - start) / time_clock::CLOCKS_PER_MILLI > 100)
```
