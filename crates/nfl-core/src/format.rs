//! NFL binary format specification and parsing

use serde::{Deserialize, Serialize};

/// NFL file structure
///
/// [NFL MAGIC BYTES "NFL1"]          (4 bytes)
/// [HEADER LENGTH]                   (8 bytes, u64 BE)
/// [CBOR HEADER BLOCK]               (variable)
/// [CBOR TOKENIZER BLOCK]            (variable)
/// [WEIGHTS INDEX TABLE]             (variable)
/// [QUANTIZED WEIGHT BLOCKS]         (variable)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NflFormat {
    pub magic: [u8; 4],
    pub header_len: u64,
    pub header_offset: u64,
    pub tokenizer_offset: u64,
    pub weights_index_offset: u64,
    pub weights_data_offset: u64,
}

impl NflFormat {
    pub fn new() -> Self {
        Self {
            magic: *b"NFL1",
            header_len: 0,
            header_offset: 12,
            tokenizer_offset: 0,
            weights_index_offset: 0,
            weights_data_offset: 0,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.magic == *b"NFL1"
    }
}

impl Default for NflFormat {
    fn default() -> Self {
        Self::new()
    }
}

/// Quantization precision specification
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Precision {
    BF16,
    INT4,
    INT2,
    FP32,
}

/// Layer-specific quantization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantizationConfig {
    pub attention_precision: Precision,
    pub mlp_precision: Precision,
    pub compression_precision: Precision,
    pub output_precision: Precision,
}

impl QuantizationConfig {
    pub fn default_mixed_precision() -> Self {
        Self {
            attention_precision: Precision::BF16,
            mlp_precision: Precision::INT4,
            compression_precision: Precision::INT2,
            output_precision: Precision::BF16,
        }
    }
}

/// Memory alignment requirement (in bytes)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MemoryAlignment {
    Bytes16 = 16,
    Bytes64 = 64,
}
