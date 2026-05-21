#!/bin/bash

# NFL Build and Test Script
# Usage: ./build.sh [release|debug|test|bench]

set -e

COMMAND=${1:-release}
VERBOSE=${VERBOSE:-0}

if [ "$VERBOSE" = "1" ]; then
    set -x
fi

echo "=== NFL Build Script ==="
echo "Command: $COMMAND"
echo ""

case "$COMMAND" in
    release)
        echo "Building release binary..."
        cargo build --release
        echo "✅ Release build complete"
        echo "Binary: target/release/nfl"
        ;;
    debug)
        echo "Building debug binary..."
        cargo build
        echo "✅ Debug build complete"
        echo "Binary: target/debug/nfl"
        ;;
    test)
        echo "Running tests..."
        cargo test --all
        echo "✅ All tests passed"
        ;;
    bench)
        echo "Running benchmarks..."
        cargo bench --all
        echo "✅ Benchmarks complete"
        ;;
    check)
        echo "Running cargo check..."
        cargo check --all
        cargo clippy --all -- -D warnings
        cargo fmt --all -- --check
        echo "✅ All checks passed"
        ;;
    clean)
        echo "Cleaning build artifacts..."
        cargo clean
        echo "✅ Cleanup complete"
        ;;
    native)
        echo "Building with native CPU optimizations..."
        RUSTFLAGS="-C target-cpu=native" cargo build --release
        echo "✅ Native build complete"
        echo "Binary: target/release/nfl"
        ;;
    *)
        echo "Usage: $0 [release|debug|test|bench|check|clean|native]"
        exit 1
        ;;
esac

echo ""
echo "Done!"
