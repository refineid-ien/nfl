.DEFAULT_GOAL := help

.PHONY: help
help:
	@echo "NFL - Neo-Flexible Light Format"
	@echo ""
	@echo "Available commands:"
	@echo "  make build      - Build release binary"
	@echo "  make test       - Run all tests"
	@echo "  make test-core  - Run nfl-core tests"
	@echo "  make test-engine- Run nfl-engine tests"
	@echo "  make docs       - Build documentation"
	@echo "  make fmt        - Format code"
	@echo "  make lint       - Run clippy linter"
	@echo "  make check      - Run cargo check"
	@echo "  make clean      - Clean build artifacts"
	@echo "  make example    - Run example code"

.PHONY: build
build:
	cargo build --release

.PHONY: test
test:
	cargo test --all

.PHONY: test-core
test-core:
	cargo test -p nfl-core

.PHONY: test-engine
test-engine:
	cargo test -p nfl-engine

.PHONY: test-quant
test-quant:
	cargo test -p nfl-quant

.PHONY: test-simd
test-simd:
	cargo test -p nfl-simd

.PHONY: test-tokenizer
test-tokenizer:
	cargo test -p nfl-tokenizer

.PHONY: docs
docs:
	cargo doc --no-deps --open

.PHONY: fmt
fmt:
	cargo fmt --all

.PHONY: lint
lint:
	cargo clippy --all --all-targets -- -D warnings

.PHONY: check
check:
	cargo check --all

.PHONY: clean
clean:
	cargo clean

.PHONY: example
example:
	@echo "Running basic_inference example..."
	cargo run --example basic_inference
	@echo ""
	@echo "Running quantization example..."
	cargo run --example quantization
	@echo ""
	@echo "Running simd_operations example..."
	cargo run --example simd_operations
	@echo ""
	@echo "Running tokenizer example..."
	cargo run --example tokenizer
	@echo ""
	@echo "Running sampling example..."
	cargo run --example sampling

.PHONY: bench
bench:
	cargo bench --all

.PHONY: native
native:
	RUSTFLAGS="-C target-cpu=native" cargo build --release

.PHONY: profile
profile:
	cargo build --release
	perf record -g target/release/nfl run test.nfl
	perf report
