# NFL Getting Started Guide

## Installation

### Prerequisites
- Rust 1.70+ (stable)
- Cargo

### Build from Source

```bash
git clone https://github.com/refineid-ien/nfl
cd nfl
cargo build --release
```

### Build Binary

```bash
cargo build --release --bin nfl
```

The binary will be available at `target/release/nfl`.

## Quick Start

### 1. Convert a Model

Convert a HuggingFace model to NFL format:

```bash
nfl convert path/to/model.bin --output model.nfl --quantization 4
```

Options:
- `--quantization 4`: INT4 quantization (default)
- `--quantization 2`: INT2 quantization (aggressive)
- `--quantization 16`: BF16 (no compression)

### 2. Inspect Model

View model metadata:

```bash
nfl inspect model.nfl
```

Output:
```
Model Name: llama2-7b
Architecture: transformer
Vocab Size: 32000
Hidden Size: 4096
Num Layers: 32
Total Size: 2.5 GB
```

### 3. Run Inference

Generate text from a prompt:

```bash
nfl run model.nfl --prompt "Hello, world" --max-tokens 100
```

### 4. Benchmark Performance

Measure inference speed:

```bash
nfl benchmark model.nfl --iterations 1000
```

## Usage Examples

### Basic Generation

```bash
nfl run model.nfl --prompt "The future of AI is"
```

### Batch Processing

```bash
nfl run model.nfl \
  --prompt "Question: What is 2+2?" \
  --max-tokens 50
```

### Verbose Output

```bash
nfl --verbose run model.nfl --prompt "test" -max-tokens 10
```

## Integration in Rust Code

### Basic Usage

```rust
use nfl_core::loader::NflLoader;
use nfl_engine::InferenceEngine;

fn main() -> anyhow::Result<()> {
    // Load model
    let nfl_file = NflLoader::load("model.nfl")?;
    
    // Create engine
    let engine = InferenceEngine::new(nfl_file.header);
    
    // Generate
    let output = engine.generate("Hello", 50);
    println!("{}", output);
    
    Ok(())
}
```

### With Custom Tokenizer

```rust
use nfl_tokenizer::Tokenizer;

let mut tokenizer = Tokenizer::new(32000);
tokenizer.add_token("hello".to_string(), 1);
tokenizer.add_special_token("[PAD]".to_string(), 0);

let ids = tokenizer.encode("hello world");
let text = tokenizer.decode(&ids);
```

### Memory-Mapped Access

```rust
use nfl_core::mmap::MmapLoader;

let loader = MmapLoader::new("model.nfl")?;
let slice = loader.get_slice(offset, length)?;
```

## Environment Variables

```bash
# Enable debug logging
export RUST_LOG=debug
nfl run model.nfl --prompt "test"

# Specific module logging
export RUST_LOG=nfl_engine=debug,nfl_core=info
```

## Troubleshooting

### Model Loading Fails

```
Error: Invalid NFL magic bytes
```

**Solution:** Ensure the file is a valid NFL model.

```bash
# Check file signature
xxd -l 16 model.nfl
# Should show: 4e 46 4c 31 (NFL1)
```

### Out of Memory

The model is trying to load entirely into RAM. NFL should stream from disk using mmap.

**Solution:** Use memory-mapped access:

```rust
let loader = MmapLoader::new("model.nfl")?;
// Does not load entire model into RAM
```

### Slow Inference

Check if SIMD acceleration is being used:

```bash
export RUST_LOG=nfl_simd=debug
nfl run model.nfl --prompt "test"
```

### Accuracy Issues

Verify quantization settings match model requirements:

```bash
nfl inspect model.nfl | grep quantization
```

## Performance Tips

1. **Use INT4 quantization** for 4x compression with minimal accuracy loss
2. **Enable memory mapping** to reduce RAM usage
3. **Leverage SIMD** - compile with CPU-specific features:
   ```bash
   RUSTFLAGS="-C target-cpu=native" cargo build --release
   ```
4. **Use smaller models** for embedded systems
5. **Enable KV cache** to reduce redundant computation

## Next Steps

- Read [FILE_FORMAT.md](../spec/FILE_FORMAT.md) for format specification
- Review [DESIGN.md](../architecture/DESIGN.md) for architecture details
- Check [examples/](../../examples/) for sample code
- Run tests: `cargo test --all`
