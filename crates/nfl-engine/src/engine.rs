//! Main inference engine orchestration

use nfl_core::header::Header;
use nfl_tokenizer::Tokenizer;

/// Main NFL inference engine
pub struct InferenceEngine {
    header: Header,
    tokenizer: Option<Tokenizer>,
}

impl InferenceEngine {
    pub fn new(header: Header) -> Self {
        Self {
            header,
            tokenizer: None,
        }
    }

    pub fn with_tokenizer(mut self, tokenizer: Tokenizer) -> Self {
        self.tokenizer = Some(tokenizer);
        self
    }

    /// Generate text given a prompt
    pub fn generate(&self, prompt: &str, max_tokens: usize) -> String {
        // Placeholder for inference implementation
        format!("Generated output from prompt: {}", prompt)
    }

    pub fn get_model_info(&self) -> &Header {
        &self.header
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let header = Header::new("test-model".to_string(), "transformer".to_string());
        let engine = InferenceEngine::new(header);
        assert_eq!(engine.header.model_name, "test-model");
    }
}
