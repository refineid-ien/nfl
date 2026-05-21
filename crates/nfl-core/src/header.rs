//! NFL file header and metadata parsing

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// NFL file header containing model metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub version: u32,
    pub model_name: String,
    pub model_size_bytes: u64,
    pub architecture: String,
    pub vocab_size: u32,
    pub hidden_size: u32,
    pub num_layers: u32,
    pub num_attention_heads: u32,
    pub max_sequence_length: u32,
    pub metadata: HashMap<String, String>,
}

impl Header {
    pub fn new(model_name: String, architecture: String) -> Self {
        Self {
            version: 1,
            model_name,
            model_size_bytes: 0,
            architecture,
            vocab_size: 32000,
            hidden_size: 4096,
            num_layers: 32,
            num_attention_heads: 32,
            max_sequence_length: 2048,
            metadata: HashMap::new(),
        }
    }

    pub fn validate(&self) -> bool {
        // Version check
        if self.version != 1 {
            return false;
        }

        // Basic field validation
        if self.model_name.is_empty() {
            return false;
        }

        if self.model_name.len() > 256 {
            return false; // Name too long
        }

        if self.vocab_size == 0 || self.vocab_size > 1_000_000 {
            return false; // Unrealistic vocab size
        }

        if self.hidden_size == 0 || self.hidden_size > 32768 {
            return false; // Unrealistic hidden size
        }

        if self.num_layers == 0 || self.num_layers > 1024 {
            return false; // Unrealistic num_layers
        }

        if self.num_attention_heads == 0 {
            return false;
        }

        // Check hidden size divisible by num_attention_heads
        if self.hidden_size % self.num_attention_heads != 0 {
            return false;
        }

        if self.max_sequence_length == 0 || self.max_sequence_length > 1_000_000 {
            return false; // Unrealistic sequence length
        }

        true
    }

    /// Get head dimension (hidden_size / num_attention_heads)
    pub fn head_dim(&self) -> u32 {
        self.hidden_size / self.num_attention_heads
    }

    /// Estimate intermediate size (typically 4x hidden_size)
    pub fn intermediate_size(&self) -> u32 {
        self.hidden_size * 4
    }

    /// Check if model configuration is reasonable
    pub fn is_reasonable(&self) -> bool {
        // Architecture-specific reasonable bounds
        match self.architecture.as_str() {
            "transformer" | "llama" | "mistral" | "gpt" => {
                // Typical bounds for transformer models
                self.hidden_size <= 16384
                    && self.num_layers <= 128
                    && self.vocab_size >= 1000
                    && self.max_sequence_length <= 131072
            }
            "moe" => {
                // Mixture of Experts might have different bounds
                self.hidden_size <= 32768
                    && self.num_layers <= 256
                    && self.vocab_size >= 1000
            }
            _ => true, // Unknown architecture, skip checks
        }
    }

    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get estimated memory usage in bytes (FP32)
    pub fn estimated_memory_fp32(&self) -> u64 {
        let params = self.estimate_parameters();
        params * 4 // 4 bytes per float32
    }

    /// Get estimated parameters count
    pub fn estimate_parameters(&self) -> u64 {
        let attention_params = (self.hidden_size as u64)
            * (self.hidden_size as u64)
            * 4
            * self.num_layers as u64;

        let mlp_params = (self.hidden_size as u64)
            * (self.intermediate_size() as u64)
            * 2
            * self.num_layers as u64;

        let embedding_params = (self.vocab_size as u64) * (self.hidden_size as u64);

        attention_params + mlp_params + embedding_params
    }

    /// Format header as readable string
    pub fn format_display(&self) -> String {
        let params = self.estimate_parameters();
        let memory_gb = self.estimated_memory_fp32() as f64 / (1024.0 * 1024.0 * 1024.0);

        format!(
            "Model: {}\n  Architecture: {}\n  Version: {}\n  Parameters: {} (≈{:.1}GB FP32)\n  Vocab: {}\n  Hidden: {}\n  Layers: {}\n  Heads: {} (dim={})\n  Max Seq: {}",
            self.model_name,
            self.architecture,
            self.version,
            params,
            memory_gb,
            self.vocab_size,
            self.hidden_size,
            self.num_layers,
            self.num_attention_heads,
            self.head_dim(),
            self.max_sequence_length
        )
    }
}

/// Tokenizer metadata embedded in NFL file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizerMetadata {
    pub vocab_size: u32,
    pub vocab: Vec<String>,
    pub special_tokens: HashMap<String, u32>,
}

impl TokenizerMetadata {
    pub fn new(vocab_size: u32) -> Self {
        Self {
            vocab_size,
            vocab: Vec::with_capacity(vocab_size as usize),
            special_tokens: HashMap::new(),
        }
    }

    /// Validate tokenizer metadata
    pub fn validate(&self) -> bool {
        if self.vocab_size == 0 {
            return false;
        }

        // Check if vocab size matches actual vocab
        if !self.vocab.is_empty() && self.vocab.len() as u32 != self.vocab_size {
            return false;
        }

        // Check special tokens are within vocab
        for &token_id in self.special_tokens.values() {
            if token_id >= self.vocab_size {
                return false;
            }
        }

        true
    }

    pub fn add_token(&mut self, token: String) {
        self.vocab.push(token);
    }

    pub fn add_special_token(&mut self, name: String, id: u32) {
        self.special_tokens.insert(name, id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_validation() {
        let header = Header::new("test".to_string(), "transformer".to_string());
        assert!(header.validate());
    }

    #[test]
    fn test_invalid_header_empty_name() {
        let mut header = Header::new("test".to_string(), "transformer".to_string());
        header.model_name.clear();
        assert!(!header.validate());
    }

    #[test]
    fn test_head_dim() {
        let header = Header::new("test".to_string(), "transformer".to_string());
        assert_eq!(header.head_dim(), 128); // 4096 / 32
    }

    #[test]
    fn test_reasonable_bounds() {
        let header = Header::new("test".to_string(), "transformer".to_string());
        assert!(header.is_reasonable());
    }

    #[test]
    fn test_estimated_memory() {
        let header = Header::new("test".to_string(), "transformer".to_string());
        let memory = header.estimated_memory_fp32();
        assert!(memory > 0);
    }

    #[test]
    fn test_format_display() {
        let header = Header::new("test-model".to_string(), "transformer".to_string());
        let display = header.format_display();
        assert!(display.contains("test-model"));
        assert!(display.contains("transformer"));
    }
}
