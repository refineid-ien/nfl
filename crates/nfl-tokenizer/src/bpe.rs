//! Byte-pair encoding (BPE) support

use std::collections::HashMap;

/// Byte-pair encoding processor
pub struct BPEProcessor {
    merge_rules: Vec<(String, String)>,
    vocab_freq: HashMap<String, u32>,
}

impl BPEProcessor {
    pub fn new() -> Self {
        Self {
            merge_rules: Vec::new(),
            vocab_freq: HashMap::new(),
        }
    }

    /// Add merge rule for BPE
    pub fn add_merge_rule(&mut self, pair1: String, pair2: String) {
        self.merge_rules.push((pair1, pair2));
    }

    /// Process text with BPE
    pub fn encode(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|word| word.to_string())
            .collect()
    }

    /// Get vocabulary frequency
    pub fn vocab_frequency(&self) -> &HashMap<String, u32> {
        &self.vocab_freq
    }

    /// Update vocabulary frequency
    pub fn update_frequency(&mut self, token: String) {
        *self.vocab_freq.entry(token).or_insert(0) += 1;
    }
}

impl Default for BPEProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bpe_processor() {
        let mut bpe = BPEProcessor::new();
        bpe.add_merge_rule("he".to_string(), "llo".to_string());
        assert!(!bpe.merge_rules.is_empty());
    }
}
