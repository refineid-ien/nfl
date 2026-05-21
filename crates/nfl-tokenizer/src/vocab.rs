//! Vocabulary management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Vocabulary container for token mappings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vocabulary {
    pub tokens: HashMap<String, u32>,
    pub token_to_id: HashMap<String, u32>,
    pub id_to_token: HashMap<u32, String>,
    pub size: u32,
}

impl Vocabulary {
    pub fn new(size: u32) -> Self {
        Self {
            tokens: HashMap::new(),
            token_to_id: HashMap::new(),
            id_to_token: HashMap::new(),
            size,
        }
    }

    /// Load vocabulary from CBOR bytes
    pub fn from_cbor(bytes: &[u8]) -> Result<Self, serde_cbor::Error> {
        serde_cbor::from_slice(bytes)
    }

    /// Save vocabulary to CBOR bytes
    pub fn to_cbor(&self) -> Result<Vec<u8>, serde_cbor::Error> {
        serde_cbor::to_vec(self)
    }

    /// Add token to vocabulary
    pub fn add(&mut self, token: String, id: u32) {
        self.tokens.insert(token.clone(), id);
        self.token_to_id.insert(token.clone(), id);
        self.id_to_token.insert(id, token);
    }

    /// Look up token ID
    pub fn get_id(&self, token: &str) -> Option<u32> {
        self.token_to_id.get(token).copied()
    }

    /// Look up token string
    pub fn get_token(&self, id: u32) -> Option<String> {
        self.id_to_token.get(&id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vocabulary() {
        let mut vocab = Vocabulary::new(1000);
        vocab.add("test".to_string(), 42);
        assert_eq!(vocab.get_id("test"), Some(42));
        assert_eq!(vocab.get_token(42), Some("test".to_string()));
    }
}
