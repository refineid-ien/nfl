# NFL System Architecture Diagram

## Complete Data Flow Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        User Applications                        │
│  (Python bindings, Go, Java, Web, Embedded, etc.)              │
└────────────────────────┬────────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────────┐
│                      NFL CLI Layer                              │
│  ┌──────────────┬──────────────┬──────────────┬──────────────┐ │
│  │   convert    │     run      │   inspect    │  benchmark   │ │
│  └──────────────┴──────────────┴──────────────┴──────────────┘ │
└────────────────────────┬────────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────────┐
│                 Inference Engine Layer                          │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ InferenceEngine                                         │   │
│  │  ├─ AttentionLayer      (Scaled dot-product)           │   │
│  │  ├─ MLPLayer            (Feed-forward networks)        │   │
│  │  ├─ Sampler             (Greedy/Temp/TopK/TopP)       │   │
│  │  └─ KVCache             (LRU eviction policy)          │   │
│  └─────────────────────────────────────────────────────────┘   │
└────────────────────────┬────────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────────┐
│              SIMD Acceleration Layer                            │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ SIMDMatmul          SIMDActivation      SIMDVectorOps   │  │
│  │ • Matrix multiply   • GELU activation   • Dot product   │  │
│  │ • Transposed mult   • ReLU              • Add/subtract  │  │
│  │                     • Softmax           • Scale/Norm    │  │
│  └──────────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Architecture Detection (Architecture-specific paths)     │  │
│  │ ├─ AVX2    (256-bit SIMD for x86_64)                   │  │
│  │ ├─ NEON    (128-bit SIMD for ARM64)                    │  │
│  │ ├─ WASM    (128-bit SIMD for WebAssembly)              │  │
│  │ └─ Generic (Portable scalar implementation)            │  │
│  └──────────────────────────────────────────────────────────┘  │
└────────────────────────┬────────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────────┐
│              Quantization Layer                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Int4Quantizer       Int2Quantizer      Bf16Quantizer    │  │
│  │ • Quantize float32  • Aggressive       • 16-bit float   │  │
│  │ • Dequantize to f32 • Compression      • Batch convert  │  │
│  │ • 8x compression    • 16x compression  • Batch ops      │  │
│  │                     • Experimental     • 2x compression  │  │
│  └──────────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ MixedPrecisionRouter                                     │  │
│  │ • Layer type → precision routing                        │  │
│  │ • Compression ratio calculation                         │  │
│  │ • Precision selection per layer                         │  │
│  └──────────────────────────────────────────────────────────┘  │
└────────────────────────┬────────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────────┐
│              Core Format Layer                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ NflLoader                                                │  │
│  │ • Magic byte validation (NFL1)                          │  │
│  │ • File parsing                                          │  │
│  │ • Header deserialization (CBOR)                         │  │
│  │ • Format validation                                     │  │
│  └──────────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ MmapLoader                                               │  │
│  │ • Memory-mapped file access                             │  │
│  │ • Zero-copy tensor streaming                            │  │
│  │ • Boundary checking                                     │  │
│  │ • Alignment verification                                │  │
│  └──────────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Tokenizer & Metadata                                     │  │
│  │ • Token encoding/decoding                               │  │
│  │ • Vocabulary management                                 │  │
│  │ • Special tokens ([PAD], [EOS], [BOS])                 │  │
│  │ • CBOR serialization                                    │  │
│  └──────────────────────────────────────────────────────────┘  │
└────────────────────────┬────────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────────┐
│                    Persistent Storage                           │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                  model.nfl (Single File)                │  │
│  │  [NFL1 magic] [header len] [CBOR header] [tokenizer]   │  │
│  │  [weight index] [quantized weights]                    │  │
│  │                                                          │  │
│  │  Size: 1-3 GB (INT4 compressed)                        │  │
│  │  RAM required: 1-2 GB (streaming via mmap)             │  │
│  │  Startup time: < 2 seconds                             │  │
│  └──────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────┘
```

## Inference Pipeline

```
Input Prompt
    │
    ▼
┌─────────────────────┐
│   Tokenizer.encode  │  "Hello world" → [1, 2, 3]
└──────────┬──────────┘
           │
           ▼
┌─────────────────────────────────────┐
│   InferenceEngine.forward()         │
│                                     │
│  For each token position:           │
│  ├─ Load embeddings (mmap)          │
│  │                                  │
│  ├─ AttentionLayer.forward()        │
│  │  ├─ SIMDMatmul (Q, K, V)        │
│  │  ├─ SIMDActivation (softmax)    │
│  │  └─ SIMDMatmul (attention, V)   │
│  │                                  │
│  ├─ MLPLayer.forward()              │
│  │  ├─ SIMDMatmul (linear)         │
│  │  ├─ SIMDActivation (GELU)       │
│  │  └─ SIMDMatmul (projection)     │
│  │                                  │
│  ├─ KVCache.add()                   │
│  │  (Store context for reuse)       │
│  │                                  │
│  └─ Output logits [vocab_size]      │
│                                     │
└──────────────┬──────────────────────┘
               │
               ▼
┌──────────────────────────┐
│  Sampler.sample()        │
│  ├─ Greedy: argmax       │
│  ├─ Temperature: scaled   │
│  ├─ Top-K: filtered      │
│  └─ Top-P: nucleus       │
│                          │
│  Output: next token ID   │
└──────────────┬───────────┘
               │
               ▼
┌──────────────────────────┐
│  Tokenizer.decode()      │  [4] → "There"
└──────────────┬───────────┘
               │
               ▼
        Output Text
```

## Memory Mapping Strategy

```
File: model.nfl
────────────────────────────────────────────────────────┐
│ [NFL1] [header_len: 8B] [header: CBOR]               │
│ [tokenizer: CBOR] [weights index] [weight blocks]    │
│ Total: 1-3 GB (highly compressible)                  │
└────────────────────────────────────────────────────────┘
                       │
                       │ MmapLoader::new()
                       │ Creates OS-level memory mapping
                       │ (no actual RAM allocation yet)
                       ▼
┌────────────────────────────────────────────────────────┐
│ Virtual Address Space (Page-Aligned Access)           │
│                                                        │
│  Requested page 0    Requested page 1   Requested...  │
│       │                    │                   │       │
│   (fault)                (fault)          (fault)      │
│       │                    │                   │       │
│       ▼                    ▼                   ▼       │
│  Load from disk      Load from disk     Load from...   │
│  into RAM (page)     into RAM (page)     RAM (page)    │
│       │                    │                   │       │
│       ▼                    ▼                   ▼       │
│  Use tensor data    Use tensor data    Use tensor...   │
└────────────────────────────────────────────────────────┘

Benefit: Only active pages in RAM
- 2 GB model, 512 MB context window → ~512-600 MB RAM
- No full model loading
- Automatic page eviction
- Cache-friendly access patterns
```

## Quantization Pipeline

```
Original FP32 Weights (100%)
         │
         ├─ Attention Layers (30%) ─────┐
         │                               │
         ├─ MLP Layers (50%) ────────┐   │
         │                           │   │
         ├─ Compression (10%) ───┐   │   │
         │                       │   │   │
         └─ Norms/Output (10%) ──┼───┼──┐
                                 │   │  │
                  INT4 (8x)      │   │  │
                  INT2 (16x)     │   │  │
                  BF16 (2x)  ────┘   │  │
                                     │  │
                                     ▼  ▼
                          Quantized Weights (5-6x)
                                     │
                                     │ Stored in .nfl
                                     │
                                     ▼
                          Memory-Mapped File
                          (Instant streaming access)
```

## Component Dependencies

```
nfl-cli
  ├─ nfl-core (file format)
  ├─ nfl-engine (inference)
  │  ├─ nfl-core
  │  ├─ nfl-simd
  │  └─ nfl-tokenizer
  ├─ nfl-writer (conversion)
  │  ├─ nfl-core
  │  ├─ nfl-quant
  │  └─ nfl-tokenizer
  └─ nfl-quant
     ├─ nfl-core
     └─ nfl-simd

nfl-simd (no dependencies - pure SIMD ops)

nfl-core (no internal dependencies - lowest level)

nfl-tokenizer (serde, unicode support)

nfl-quant
  ├─ nfl-core
  └─ nfl-simd
```

## Performance Optimization Layers

```
Layer 1: Format Level
├─ Aligned tensor storage (64-byte boundaries)
├─ CBOR metadata (not JSON in hot path)
└─ Memory-mapped access (zero full-load)

Layer 2: Quantization Level
├─ INT4 (8x compression, minimal accuracy loss)
├─ Mixed precision (higher precision for critical layers)
└─ Batch operations (amortize quantization overhead)

Layer 3: SIMD Level
├─ AVX2 (x86_64): 256-bit vector operations
├─ NEON (ARM64): 128-bit vector operations
├─ Generic fallback: Scalar implementation
└─ CPU auto-detection: Right path selected

Layer 4: Algorithm Level
├─ KV cache (reuse attention computations)
├─ Selective eviction (smart cache management)
└─ Streaming inference (low latency)

Layer 5: System Level
├─ Memory mapping (efficient file access)
├─ Page prefetching (anticipate access patterns)
└─ Cache alignment (minimize cache misses)

Result: 5-6x compression + 4-8x SIMD speedup = ~25-50x total improvement
```

---

**Architecture Version**: 1.0
**Last Updated**: 2026-05-21
