//! Main tokenizer implementation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// NFL Tokenizer with comprehensive token handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tokenizer {
    pub vocab_size: u32,
    pub vocab: HashMap<String, u32>,
    pub inv_vocab: HashMap<u32, String>,
    pub special_tokens: HashMap<String, u32>,
    pub unk_token: Option<u32>,
    pub pad_token: Option<u32>,
    pub eos_token: Option<u32>,
    pub bos_token: Option<u32>,
}

impl Tokenizer {
    pub fn new(vocab_size: u32) -> Self {
        Self {
            vocab_size,
            vocab: HashMap::new(),
            inv_vocab: HashMap::new(),
            special_tokens: HashMap::new(),
            unk_token: None,
            pad_token: None,
            eos_token: None,
            bos_token: None,
        }
    }

    /// Add token to vocabulary
    pub fn add_token(&mut self, token: String, id: u32) -> Result<(), String> {
        if id >= self.vocab_size {
            return Err(format!("Token ID {} exceeds vocab size {}", id, self.vocab_size));
        }

        self.vocab.insert(token.clone(), id);
        self.inv_vocab.insert(id, token);
        Ok(())
    }

    /// Add special token
    pub fn add_special_token(&mut self, token: String, id: u32) -> Result<(), String> {
        if id >= self.vocab_size {
            return Err(format!("Token ID {} exceeds vocab size {}", id, self.vocab_size));
        }

        self.special_tokens.insert(token.clone(), id);
        self.vocab.insert(token, id);
        Ok(())
    }

    /// Set unknown token (UNK)
    pub fn set_unk_token(&mut self, id: u32) {
        self.unk_token = Some(id);
        if let Some(unk) = self.inv_vocab.get(&id) {
            let _ = self.add_special_token(unk.clone(), id);
        }
    }

    /// Set padding token (PAD)
    pub fn set_pad_token(&mut self, id: u32) {
        self.pad_token = Some(id);
        if let Some(pad) = self.inv_vocab.get(&id) {
            let _ = self.add_special_token(pad.clone(), id);
        }
    }

    /// Set end-of-sequence token (EOS)
    pub fn set_eos_token(&mut self, id: u32) {
        self.eos_token = Some(id);
        if let Some(eos) = self.inv_vocab.get(&id) {
            let _ = self.add_special_token(eos.clone(), id);
        }
    }

    /// Set beginning-of-sequence token (BOS)
    pub fn set_bos_token(&mut self, id: u32) {
        self.bos_token = Some(id);
        if let Some(bos) = self.inv_vocab.get(&id) {
            let _ = self.add_special_token(bos.clone(), id);
        }
    }

    /// Encode text to token IDs
    pub fn encode(&self, text: &str) -> Vec<u32> {
        text.split_whitespace()
            .filter_map(|word| {
                self.vocab.get(word)
                    .copied()
                    .or(self.unk_token)
                    .or(Some(0)) // Fallback to token 0 if no UNK defined
            })
            .collect()
    }

    /// Encode with special prefix/suffix tokens
    pub fn encode_with_special_tokens(&self, text: &str) -> Vec<u32> {
        let mut tokens = Vec::new();

        // Add BOS if defined
        if let Some(bos) = self.bos_token {
            tokens.push(bos);
        }

        // Add regular tokens
        tokens.extend(self.encode(text));

        // Add EOS if defined
        if let Some(eos) = self.eos_token {
            tokens.push(eos);
        }

        tokens
    }

    /// Decode token IDs to text
    pub fn decode(&self, token_ids: &[u32]) -> String {
        token_ids
            .iter()
            .filter_map(|id| self.inv_vocab.get(id).cloned())
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Decode without spaces between special tokens
    pub fn decode_clean(&self, token_ids: &[u32]) -> String {
        let mut result = String::new();

        for id in token_ids {
            if let Some(token) = self.inv_vocab.get(id) {
                // Check if it's a special token
                if self.special_tokens.contains_key(token) {
                    result.push_str(token);
                } else {
                    if !result.is_empty() && !result.ends_with(' ') {
                        result.push(' ');
                    }
                    result.push_str(token);
                }
            }
        }

        result
    }

    /// Get token ID for string
    pub fn get_token_id(&self, token: &str) -> Option<u32> {
        self.vocab.get(token).copied()
    }

    /// Get string for token ID
    pub fn get_token_string(&self, id: u32) -> Option<String> {
        self.inv_vocab.get(&id).cloned()
    }

    /// Check if token exists
    pub fn has_token(&self, token: &str) -> bool {
        self.vocab.contains_key(token)
    }

    /// Get vocabulary size
    pub fn vocab_size(&self) -> usize {
        self.vocab.len()
    }

    /// Get number of special tokens
    pub fn num_special_tokens(&self) -> usize {
        self.special_tokens.len()
    }

    /// Convert token IDs to token strings
    pub fn ids_to_tokens(&self, ids: &[u32]) -> Vec<Option<String>> {
        ids.iter()
            .map(|id| self.inv_vocab.get(id).cloned())
            .collect()
    }

    /// Convert token strings to IDs
    pub fn tokens_to_ids(&self, tokens: &[&str]) -> Vec<Option<u32>> {
        tokens
            .iter()
            .map(|token| self.vocab.get(*token).copied())
            .collect()
    }

    /// Validate tokenizer configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.vocab.is_empty() {
            return Err("Vocabulary is empty".to_string());
        }

        // Check for ID conflicts
        let mut seen_ids = std::collections::HashSet::new();
        for &id in self.vocab.values() {
            if !seen_ids.insert(id) {
                return Err(format!("Duplicate token ID: {}", id));
            }
            if id >= self.vocab_size {
                return Err(format!("Token ID {} exceeds vocab size {}", id, self.vocab_size));
            }
        }

        Ok(())
    }
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new(32000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_basic() {
        let mut tok = Tokenizer::new(1000);
        let _ = tok.add_token("hello".to_string(), 1);
        let _ = tok.add_token("world".to_string(), 2);

        assert_eq!(tok.get_token_id("hello"), Some(1));
        assert_eq!(tok.decode(&[1, 2]), "hello world");
    }

    #[test]
    fn test_special_tokens() {
        let mut tok = Tokenizer::new(1000);
        let _ = tok.add_special_token("[PAD]".to_string(), 0);
        let _ = tok.add_special_token("[EOS]".to_string(), 1);

        tok.set_pad_token(0);
        tok.set_eos_token(1);

        assert_eq!(tok.pad_token, Some(0));
        assert_eq!(tok.eos_token, Some(1));
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let mut tok = Tokenizer::new(1000);
        let _ = tok.add_token("hello".to_string(), 1);
        let _ = tok.add_token("world".to_string(), 2);

        let text = "hello world";
        let encoded = tok.encode(text);
        let decoded = tok.decode(&encoded);

        assert_eq!(decoded, "hello world");
    }

    #[test]
    fn test_special_tokens_encoding() {
        let mut tok = Tokenizer::new(1000);
        let _ = tok.add_token("test".to_string(), 1);
        let _ = tok.add_special_token("[BOS]".to_string(), 100);
        let _ = tok.add_special_token("[EOS]".to_string(), 101);

        tok.set_bos_token(100);
        tok.set_eos_token(101);

        let encoded = tok.encode_with_special_tokens("test");
        assert_eq!(encoded[0], 100); // BOS
        assert_eq!(encoded[encoded.len() - 1], 101); // EOS
    }

    #[test]
    fn test_validation() {
        let mut tok = Tokenizer::new(10);
        let result = tok.validate();
        assert!(result.is_err()); // Empty vocab

        let _ = tok.add_token("test".to_string(), 0);
        let result = tok.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_has_token() {
        let mut tok = Tokenizer::new(1000);
        let _ = tok.add_token("hello".to_string(), 1);

        assert!(tok.has_token("hello"));
        assert!(!tok.has_token("goodbye"));
    }
}
