# NFL Architecture Guide

## System Layers

```
┌──────────────────────────────┐
│  CLI / User Interface Layer  │  nfl-cli
├──────────────────────────────┤
│  Inference Engine            │  nfl-engine
│  (attention, MLP, sampling)  │
├──────────────────────────────┤
│  SIMD Acceleration Layer     │  nfl-simd
│  (AVX2, NEON, WASM SIMD)     │
├──────────────────────────────┤
│  Quantization Layer          │  nfl-quant
│  (INT4, INT2, BF16)          │
├──────────────────────────────┤
│  Core Format                 │  nfl-core
│  (parsing, validation, mmap) │
└──────────────────────────────┘
```

## Component Responsibilities

### nfl-core
**File format parsing and memory management**

- Binary format validation
- CBOR header/tokenizer deserialization
- Memory-mapped file access
- Tensor index resolution

**Key Types:**
- `NflFormat`: Format specification
- `Header`: Model metadata
- `NflLoader`: File parser
- `MmapLoader`: Zero-copy access

### nfl-engine
**Inference orchestration and computation**

- Attention mechanism (scaled dot-product)
- MLP forward pass
- Token sampling (greedy, temperature, top-k, top-p)
- KV cache management
- Generation loop

**Key Types:**
- `InferenceEngine`: Main orchestrator
- `AttentionLayer`: Attention computation
- `MLPLayer`: Feed-forward network
- `Sampler`: Token generation
- `KVCache`: Context caching

### nfl-simd
**Hardware-accelerated operations**

- Architecture detection (AVX2, NEON, WASM)
- SIMD matrix multiplication
- Vectorized activations (GELU, ReLU, Softmax)
- Vector operations (dot product, normalization)

**Key Types:**
- `SimdTarget`: Architecture enum
- `SIMDMatmul`: Matrix operations
- `SIMDActivation`: Activation functions
- `SIMDVectorOps`: Vector operations

### nfl-quant
**Mixed-precision quantization**

- INT4 quantization (MLP default)
- INT2 quantization (compression layers)
- BF16 float (attention/output)
- Layer-specific precision routing

**Key Types:**
- `Int4Quantizer`: 4-bit compression
- `Int2Quantizer`: 2-bit compression
- `Bf16Quantizer`: 16-bit float
- `MixedPrecisionRouter`: Precision selection

### nfl-tokenizer
**Vocabulary and tokenization**

- Token-to-ID mappings
- CBOR vocabulary storage
- Byte-pair encoding support
- Special token handling

**Key Types:**
- `Tokenizer`: Main tokenizer
- `Vocabulary`: Token mappings
- `BPEProcessor`: BPE merging

### nfl-writer
**Model conversion and serialization**

- HuggingFace safetensors → NFL
- GGUF format → NFL
- PyTorch checkpoint → NFL
- Weight quantization and packing

**Key Types:**
- `ModelConverter`: Format conversion
- `NflSerializer`: Binary serialization

### nfl-cli
**Command-line interface**

- `convert`: Model conversion
- `run`: Inference execution
- `inspect`: File metadata inspection
- `benchmark`: Performance testing

## Data Flow: Model Loading

```
File: model.nfl
    ↓
[NflLoader::load()]
    ↓ validates magic bytes
    ↓
[NflLoader::from_bytes()]
    ↓ parses CBOR header
    ↓
[NflFile] → [MmapLoader]
    ↓
[InferenceEngine::new()]
    ↓
Ready for inference
```

## Data Flow: Inference

```
Prompt
  ↓
[Tokenizer::encode()]
  ↓
[Token IDs]
  ↓
[InferenceEngine::forward()]
  ├→ [AttentionLayer::forward()]
  │  ├→ [SIMDMatmul::multiply()]
  │  └→ [SIMDActivation::softmax()]
  │
  ├→ [MLPLayer::forward()]
  │  ├→ [SIMDMatmul::multiply()]
  │  └→ [SIMDActivation::gelu()]
  │
  └→ [KVCache::add()]
  ↓
[Output Logits]
  ↓
[Sampler::sample()]
  ↓
[Next Token ID]
  ↓
[Tokenizer::decode()]
  ↓
Output Text
```

## Quantization Strategy

### Mixed Precision Distribution

```
Model Size: 100%
├─ Attention Layers (30%): BF16 (high precision)
├─ MLP Layers (50%): INT4 (4x compression)
├─ Compression Layers (10%): INT2 (16x compression)
└─ Norms/Output (10%): BF16 (numerical stability)

Net compression: ~5-6x vs FP32
```

### Precision Selection

```
Layer Type          Importance  Precision  Bits  Impact
─────────────────────────────────────────────────────────
Query/Key/Value     Critical    BF16       16    5%
Attention scores    High        BF16       16    8%
Feed-forward up     Medium      INT4        4   25%
Feed-forward down   Medium      INT4        4   25%
Projection          High        BF16       16    5%
LayerNorm           Critical    BF16       16    7%
Embeddings          Medium      INT4        4   15%
```

## Performance Optimization

### Memory Bandwidth
- Zero-copy mmap access
- Aligned tensor layout (64-byte boundaries)
- Streaming weight access (no full model loading)

### CPU Utilization
- SIMD vectorization (8-16x speedup)
- Rayon parallelization for attention
- Architecture-specific code paths (AVX2/NEON)

### Cache Efficiency
- Page-aligned access patterns
- KV cache with LRU eviction
- Quantization reduces memory pressure

## Compatibility Matrix

| Target | Architecture | SIMD Support | Status |
|--------|--------------|--------------|--------|
| x86_64 | Generic      | Generic      | ✓ Stable |
| x86_64 | AVX2         | AVX2         | ✓ Optimized |
| x86_64 | AVX-512      | AVX2 fallback| ✓ Supported |
| ARM64  | Generic      | Generic      | ✓ Stable |
| ARM64  | Cortex-A72+  | NEON         | ✓ Optimized |
| WASM   | Generic      | Generic      | ✓ Supported |
| WASM   | SIMD         | WASM SIMD    | ✓ Optimized |
| Embedded| ARM Cortex  | NEON         | ✓ Tested |
