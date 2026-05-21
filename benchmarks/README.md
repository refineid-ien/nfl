# Benchmarking Guide

## Running Benchmarks

### All Benchmarks

```bash
cargo bench --all
```

### Specific Crate

```bash
cargo bench -p nfl-simd
```

### Specific Benchmark

```bash
cargo bench matrix_multiply
```

## Writing Benchmarks

Use the built-in `criterion` benchmarking framework:

```rust
#[cfg(test)]
mod benches {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    fn bench_matrix_multiply(c: &mut Criterion) {
        let a = vec![1.0; 1024];
        let b = vec![1.0; 1024];
        
        c.bench_function("4x4_matmul", |bench| {
            bench.iter(|| {
                SIMDMatmul::multiply(
                    black_box(&a),
                    black_box(&b),
                    32,
                    32,
                    32
                )
            })
        });
    }

    criterion_group!(benches, bench_matrix_multiply);
    criterion_main!(benches);
}
```

## Performance Targets

| Operation | Target | Current |
|-----------|--------|---------|
| Model load | < 2s | - |
| Token gen | 4-10 tok/s | - |
| Attention | 100x vs baseline | - |
| Matrix mult | 8x vs naive | - |

## Profiling

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

## CI/CD

Benchmarks run automatically on:
- Every commit
- Pull requests
- Release builds

Results are tracked to detect regressions.
