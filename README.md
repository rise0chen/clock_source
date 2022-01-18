# clock_source

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
clock_source = "*"
```

```rust
let timestamp = clock_source::now().as_millis()
```

### custom clock source

```toml
[dependencies]
clock_source = { version = "*", default-features = false }
```

```rust
static SOURCE: fn() -> Duration = || Duration::from_secs(1);
clock_source::register_clock_source!(SOURCE);
assert_eq!(clock_source::now(), Duration::from_secs(1));
```
