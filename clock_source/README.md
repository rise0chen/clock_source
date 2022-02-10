# clock_source

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
clock_source = "*"
```

```rust
let now: u64 = clock_source::now();
```

### custom clock source

```toml
[dependencies]
clock_source = { version = "*", features = ["custom"] }
```

```rust
static SOURCE: fn() -> u64 = || 1;
clock_source::register_clock_source!(SOURCE);
assert_eq!(clock_source::now(), 1);
```
