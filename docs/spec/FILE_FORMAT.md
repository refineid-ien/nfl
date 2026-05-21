# NFL File Format Specification

## Version 1.0

This document describes the complete binary specification for `.nfl` (Neo-Flexible Light Format) files.

## Binary Structure

```
[NFL MAGIC BYTES]
[HEADER LENGTH]
[CBOR HEADER BLOCK]
[CBOR TOKENIZER BLOCK]
[WEIGHTS INDEX TABLE]
[QUANTIZED WEIGHT BLOCKS]
```

### Magic Bytes
- **Offset:** 0
- **Length:** 4 bytes
- **Value:** `0x4E46 4C31` (ASCII: "NFL1")
- **Purpose:** File identification and validation

### Header Length
- **Offset:** 4
- **Length:** 8 bytes (unsigned 64-bit big-endian)
- **Purpose:** Indicates the size of the CBOR header block

### CBOR Header Block
- **Offset:** 12
- **Length:** Variable (specified by Header Length)
- **Format:** CBOR (Concise Binary Object Representation)
- **Content:** Model metadata and configuration

#### Header Contents
```json
{
  "version": 1,
  "model_name": "string",
  "model_size_bytes": "u64",
  "architecture": "string",
  "vocab_size": "u32",
  "hidden_size": "u32",
  "num_layers": "u32",
  "num_attention_heads": "u32",
  "max_sequence_length": "u32",
  "metadata": {}
}
```

### Tokenizer Block
- **Offset:** 12 + header_len
- **Format:** CBOR encoded vocabulary
- **Content:**
  - Token to ID mappings
  - Special tokens ([PAD], [EOS], [BOS], etc.)

### Weights Index Table
- **Format:** Binary index structure
- **Purpose:** Maps tensor names to offsets/sizes in weight blocks
- **Entry Format:**
  ```
  [tensor_name_len: u32]
  [tensor_name: bytes]
  [offset: u64]
  [size: u64]
  [precision: u8] (0=BF16, 1=INT4, 2=INT2, 3=FP32)
  ```

### Weight Blocks
- **Format:** Quantized tensor data
- **Layout:** Memory-aligned (16/64-byte boundaries for SIMD)
- **Encoding:**
  - **INT4:** 4-bit integers, 2 per byte
  - **INT2:** 2-bit integers, 4 per byte
  - **BF16:** 16-bit floating point
  - **FP32:** 32-bit floating point

## Memory Alignment

All weight blocks are aligned to 64-byte boundaries for optimal SIMD performance.

```
Weight Block Alignment:
[padding to 64B]
[weight_data: multiples of 64 bytes]
[padding to 64B]
```

## Quantization Precision Table

| Layer Type | Default Precision | Bits | Use Case |
|------------|------------------|------|----------|
| Attention  | BF16             | 16   | High precision needed |
| MLP        | INT4             | 4    | Balanced compression |
| Compression| INT2             | 2    | Aggressive compression |
| LayerNorm  | BF16             | 16   | Numerical stability |
| Output     | BF16             | 16   | Final layer |

## Backward Compatibility

- All future versions MUST preserve the magic bytes and header structure
- New fields can only be added to the metadata section
- Version number changes trigger incompatibility checks
- Format extensions use the metadata dictionary for optional features

## Forward Compatibility

Readers MUST ignore unknown fields in metadata dictionaries to support future extensions.

## Example: Minimal Valid NFL File

```
4E 46 4C 31        # "NFL1" magic
00 00 00 00 00 00 00 50  # Header length = 80 bytes
[80 bytes of CBOR header data...]
```
