# NFL Quick Reference

## Building

```bash
# Release build (optimized)
cargo build --release

# Debug build
cargo build

# Check compilation
cargo check --all

# With native CPU optimizations
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

## Testing

```bash
# Run all tests
cargo test --all

# Run specific crate tests
cargo test -p nfl-core
cargo test -p nfl-engine
cargo test -p nfl-quant
cargo test -p nfl-simd
cargo test -p nfl-tokenizer

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench --all
```

## Documentation

```bash
# Build and open docs
cargo doc --no-deps --open

# View specific crate docs
cargo doc -p nfl-core --open
```

## Code Quality

```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check

# Lint with Clippy
cargo clippy --all

# Strict lint (warnings as errors)
cargo clippy --all -- -D warnings
```

## Running Examples

```bash
cargo run --example basic_inference
cargo run --example quantization
cargo run --example simd_operations
cargo run --example tokenizer
cargo run --example sampling
```

## Using Make

```bash
make build          # Release build
make test           # Run all tests
make test-core      # nfl-core tests
make lint           # Run clippy
make fmt            # Format code
make docs           # Build documentation
make clean          # Remove build artifacts
make example        # Run all examples
make native         # CPU-native build
```

## Project Structure

```
nfl/
├── crates/                  # 7 Rust crates
│   ├── nfl-core/           # Binary format
│   ├── nfl-engine/         # Inference
│   ├── nfl-quant/          # Quantization
│   ├── nfl-simd/           # SIMD ops
│   ├── nfl-tokenizer/      # Tokenization
│   ├── nfl-writer/         # Conversion
│   └── nfl-cli/            # CLI tool
├── docs/                    # Documentation
├── examples/                # Code samples
├── tests/                   # Integration tests
└── benchmarks/              # Performance tests
```

## Common Tasks

### Add a New Dependency

Edit workspace `Cargo.toml`:

```toml
[workspace.dependencies]
my-crate = "1.0"
```

Use in crate's `Cargo.toml`:

```toml
[dependencies]
my-crate.workspace = true
```

### Add a New Module

```rust
// In lib.rs
pub mod my_module;

// Create src/my_module.rs or src/my_module/mod.rs
pub struct MyType;
```

### Add a Test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        assert_eq!(my_function(), expected);
    }
}
```

### Create a Benchmark

```rust
#[cfg(test)]
mod benches {
    use criterion::{criterion_group, criterion_main, Criterion};

    fn my_benchmark(c: &mut Criterion) {
        c.bench_function("my_test", |b| {
            b.iter(|| my_function())
        });
    }

    criterion_group!(benches, my_benchmark);
    criterion_main!(benches);
}
```

## Performance Profiling

### Linux with perf

```bash
cargo build --release
perf record -g target/release/nfl run model.nfl
perf report
```

### Flamegraph

```bash
cargo install flamegraph
cargo flamegraph --release -- run model.nfl
```

## Documentation Comments

```rust
/// Brief description
///
/// Longer explanation of what this does
///
/// # Arguments
/// * `arg1` - Description
///
/// # Returns
/// Description of return value
///
/// # Examples
/// ```
/// let result = my_function(42);
/// assert_eq!(result, 42);
/// ```
///
/// # Panics
/// When it panics (if applicable)
///
/// # Errors
/// What errors it returns (if applicable)
pub fn my_function(arg1: i32) -> i32 {
    arg1
}
```

## Error Handling

### Creating Custom Errors

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type MyResult<T> = Result<T, MyError>;
```

### Using Results

```rust
fn operation() -> MyResult<String> {
    let file = std::fs::read_to_string("config.txt")?;
    Ok(file)
}
```

## Environment Variables

```bash
# Enable debug logging
export RUST_LOG=debug

# Module-specific logging
export RUST_LOG=nfl_core=debug,nfl_engine=info

# Run with logging
export RUST_LOG=debug cargo run --example basic_inference
```

## Dependency Management

### Update Dependencies

```bash
cargo update              # Update to latest patch versions
cargo outdated           # Check for outdated dependencies
cargo tree               # View dependency tree
```

### Remove Unused Dependencies

```bash
cargo tree --duplicates  # Find duplicate crates
cargo clean             # Clean build cache
```

## Cargo Workspace Commands

```bash
# Build all crates
cargo build --all

# Test all crates
cargo test --all

# Document all crates
cargo doc --all --no-deps

# Format all crates
cargo fmt --all

# Lint all crates
cargo clippy --all
```

## Debugging Tips

### Print Values During Testing

```rust
#[test]
fn my_test() {
    let value = 42;
    println!("Value: {}", value);  // Won't show unless test fails
    println!("Debug: {:?}", value); // Pretty print
    dbg!(value);                     // Macro that prints and returns
}
```

Run with:
```bash
cargo test -- --nocapture
```

### Conditional Compilation

```rust
#[cfg(test)]
mod tests { }

#[cfg(debug_assertions)]
eprintln!("Debug mode");

#[cfg(target_os = "linux")]
unsafe { /* platform-specific code */ }
```

## Performance Tips

1. Use `--release` for production builds
2. Enable LTO in `Cargo.toml`: `lto = true`
3. Use `criterion` for accurate benchmarks
4. Profile with `perf` before optimizing
5. Leverage SIMD operations
6. Use `rayon` for parallelization

## Common Errors

### "Cannot find crate"
```bash
cargo update
cargo clean
cargo build
```

### "Conflicting versions"
Edit `Cargo.lock` or use workspace dependencies

### "Import not found"
- Add to `Cargo.toml`
- Add `use` statement
- Make item `pub`

---

For more information, see:
- [README.md](README.md)
- [CONTRIBUTING.md](CONTRIBUTING.md)
- [docs/](docs/)
