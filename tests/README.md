# Integration Tests

Integration tests verify that components work correctly together.

## Running Tests

```bash
# Run all tests
cargo test --all

# Run specific test
cargo test nfl_core::tests --lib

# Run with output
cargo test -- --nocapture
```

## Test Organization

Tests are organized by crate and module:

- `nfl-core` - Format parsing, validation, mmap
- `nfl-engine` - Inference, sampling
- `nfl-quant` - Quantization operations
- `nfl-simd` - SIMD operations
- `nfl-tokenizer` - Tokenization
- `nfl-writer` - Serialization

## Test Coverage

Target minimum coverage:
- Core format: 90%
- Engine: 80%
- Quantization: 85%
- SIMD: 75%
- Tokenizer: 80%

## Fuzzing

Fuzzing tests binary format robustness:

```bash
cargo fuzz
```

## Performance Tests

Benchmark-driven tests:

```bash
cargo bench
```
