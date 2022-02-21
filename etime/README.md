# Etime

## Usage

### Base

```rust
use core::time::Duration;
use etime::Etime;
use std::thread;

let etime = Etime::new();
etime.tic();
thread::sleep(Duration::from_secs(1));
let elapsed = etime.toc();
println!("{:?}", elapsed);
```

### Set Clock Source
