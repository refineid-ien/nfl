# 🚀 NFL - Neo-Flexible Light Format

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)
![Rust](https://img.shields.io/badge/rust-1.70+-orange)

> A universal executable format for AI models that behaves like a binary program, not a dataset.

**NFL** is a next-generation AI model format and inference runtime written in Rust, designed to replace fragmented deployment stacks like `.safetensors`, `.gguf`, and PyTorch checkpoints.

## 🚀 Quick Overview

### What is NFL?

NFL is a **unified binary format and runtime** that contains:
- ✅ Model weights (quantized)
- ✅ Tokenizer (embedded)
- ✅ Architecture metadata
- ✅ Memory mapping index
- ✅ Runtime execution contract

### One File = Everything

```bash
# Convert a model once
nfl convert llama2-7b.bin --output llama2-7b.nfl --quantization 4

# Run anywhere - no Python, no CUDA required
nfl run llama2-7b.nfl --prompt "Hello, world" --max-tokens 100
```

## 🎯 Core Design Principles

| Principle | Benefit |
|-----------|---------|
| **Single Artifact** | One .nfl file = complete executable model |
| **Zero External Dependencies** | No Python, CUDA, or complex runtimes |
| **Extreme Low-End Optimization** | Works on 2-4 GB RAM systems, embedded devices |
| **Memory-Mapped Execution** | Page-based access, zero-copy inference streams |
| **Rust-Native Safety** | Memory safety without GC, SIMD-first design |

## 📊 Performance Profile

**Target: Low-End CPU (i3 / ARM A53)**

| Metric | Target |
|--------|--------|
| Startup time | < 2 sec |
| RAM usage | 1–2 GB |
| Throughput | 4–10 tok/s |
| Model size | 1–3 GB (INT4) |

## 🧱 Architecture

```
┌──────────────────────────────┐
│ CLI / User Interface         │
├──────────────────────────────┤
│ Inference Engine             │
├──────────────────────────────┤
│ SIMD Acceleration Layer      │
├──────────────────────────────┤
│ Quantization Layer           │
├──────────────────────────────┤
│ Core Format (Binary + CBOR)  │
└──────────────────────────────┘
```

## 📦 Core Components

| Crate | Purpose |
|-------|---------|
| **nfl-core** | File parsing, validation, memory mapping |
| **nfl-engine** | Inference, attention, MLP, sampling |
| **nfl-quant** | INT4/INT2/BF16 quantization |
| **nfl-simd** | AVX2, NEON, WASM SIMD acceleration |
| **nfl-tokenizer** | Token vocabulary, byte-pair encoding |
| **nfl-writer** | Convert models to .nfl format |
| **nfl-cli** | Command-line interface |

## ⚡ Quantization Strategy

**Mixed-precision by layer type:**

| Layer | Precision | Bits | Compression |
|-------|-----------|------|-------------|
| Attention | BF16 | 16 | 2x |
| MLP | INT4 | 4 | 8x |
| Compression | INT2 | 2 | 16x |
| Output | BF16 | 16 | 2x |

**Result: ~5-6x compression with minimal accuracy loss**

## 🚀 Getting Started

### Build from Source

```bash
git clone https://github.com/refineid-ien/nfl
cd nfl
cargo build --release
```

### Convert a Model

```bash
nfl convert path/to/model.bin \
  --output model.nfl \
  --quantization 4
```

### Run Inference

```bash
nfl run model.nfl \
  --prompt "The future of AI is" \
  --max-tokens 100
```

### Inspect Model

```bash
nfl inspect model.nfl
```

## 💻 Integration Examples

### Basic Rust Usage

```rust
use nfl_core::loader::NflLoader;
use nfl_engine::InferenceEngine;

fn main() -> anyhow::Result<()> {
    // Load model
    let nfl_file = NflLoader::load("model.nfl")?;
    
    // Create engine
    let engine = InferenceEngine::new(nfl_file.header);
    
    // Generate text
    let output = engine.generate("Hello", 50);
    println!("{}", output);
    
    Ok(())
}
```

### Memory-Mapped Access

```rust
use nfl_core::mmap::MmapLoader;

// Zero-copy file access
let loader = MmapLoader::new("model.nfl")?;
let slice = loader.get_slice(offset, length)?;
```

## 📁 Repository Structure

```
nfl/
├── crates/                    # Rust workspace crates
│   ├── nfl-core/             # Binary format
│   ├── nfl-engine/           # Inference engine
│   ├── nfl-quant/            # Quantization
│   ├── nfl-simd/             # SIMD acceleration
│   ├── nfl-tokenizer/        # Tokenization
│   ├── nfl-writer/           # Model conversion
│   └── nfl-cli/              # CLI tool
├── docs/                      # Documentation
│   ├── spec/                 # Format specification
│   ├── architecture/         # Design documents
│   └── GETTING_STARTED.md    # Tutorial
├── examples/                  # Usage examples
├── tests/                     # Integration tests
├── benchmarks/               # Performance benchmarks
└── Cargo.toml                # Workspace manifest
```

## 🧪 Testing

### Run All Tests

```bash
cargo test --all
```

### Run Specific Crate Tests

```bash
cargo test -p nfl-core
cargo test -p nfl-engine
```

### Run Examples

```bash
cargo run --example basic_inference
cargo run --example quantization
cargo run --example simd_operations
cargo run --example tokenizer
cargo run --example sampling
```

## 🔧 Build Options

### Release Build (Optimized)

```bash
cargo build --release
```

### Native CPU Optimizations

```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

### With Debug Symbols

```bash
cargo build --release --debug-assertions
```

## 🌐 Compatibility

| Platform | Architecture | Status |
|----------|--------------|--------|
| Linux | x86_64 | ✅ Stable |
| Linux | ARM64 | ✅ Stable |
| macOS | x86_64 | ✅ Stable |
| macOS | ARM64 | ✅ Stable |
| Windows | x86_64 | ✅ Stable |
| WebAssembly | WASM | ✅ Experimental |

## 📚 Documentation

- [Getting Started](docs/GETTING_STARTED.md) - Tutorial and quick start
- [File Format Specification](docs/spec/FILE_FORMAT.md) - Binary format details
- [Architecture Guide](docs/architecture/DESIGN.md) - System design
- [Examples](examples/) - Code samples

## 🎯 Design Constraints (IMPORTANT)

All contributions must respect these constraints:

1. ✅ **No full-model RAM loading** by default
2. ✅ **No Python dependency** in runtime
3. ✅ **No JSON-based runtime parsing** in hot path
4. ✅ **All tensor operations** must be SIMD-aware
5. ✅ **File format** must remain forward-compatible
6. ✅ **Zero-copy principle** wherever possible

## 🚫 Non-Goals

NFL is **NOT** designed to:
- Replace GPU training frameworks
- Improve model intelligence
- Act as a training format
- Compete with distributed cloud inference

## 📝 License

Dual-licensed under **MIT** or **Apache 2.0**

## 🤝 Contributing

### RFC Process

Design changes to the format require an RFC (Request for Comments):

1. Open an RFC issue with detailed proposal
2. Community discussion and feedback
3. Implementation and testing
4. Merge to main branch

### Development

```bash
# Create feature branch
git checkout -b feature/my-feature

# Make changes and test
cargo test --all

# Submit PR
git push origin feature/my-feature
```

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/refineid-ien/nfl/issues)
- **Discussions**: [GitHub Discussions](https://github.com/refineid-ien/nfl/discussions)
- **Documentation**: [docs/](docs/)

## 🧭 Vision

NFL aims to become:

> **A universal executable format for AI models that behaves like a binary program, not a dataset.**

**Compile once. Run anywhere. Even on weak hardware.**

---

**Built with ❤️ for edge AI and inference efficiency**