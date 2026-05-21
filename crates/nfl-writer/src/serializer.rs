//! NFL file serialization

use anyhow::Result;
use bytes::BytesMut;
use nfl_core::{format::NflFormat, header::Header};
use std::io::Write;

/// NFL file serializer
pub struct NflSerializer;

impl NflSerializer {
    /// Serialize model to NFL format
    pub fn serialize(header: &Header, output_path: &str) -> Result<()> {
        let mut file = std::fs::File::create(output_path)?;

        // Write magic bytes
        file.write_all(b"NFL1")?;

        // Serialize header to CBOR
        let header_bytes = serde_cbor::to_vec(header)?;
        let header_len = header_bytes.len() as u64;

        // Write header length
        file.write_all(&header_len.to_be_bytes())?;

        // Write header
        file.write_all(&header_bytes)?;

        Ok(())
    }

    /// Create NFL format structure
    pub fn create_format(header_len: u64) -> NflFormat {
        let mut format = NflFormat::new();
        format.header_len = header_len;
        format.header_offset = 12; // 4 bytes magic + 8 bytes length
        format
    }

    /// Pack multiple weight blocks
    pub fn pack_weights(
        weights: &[Vec<f32>],
        quantization_bits: u32,
    ) -> BytesMut {
        let mut buf = BytesMut::new();
        for weight_block in weights {
            buf.extend_from_slice(&(weight_block.len() as u32).to_le_bytes());
            for &w in weight_block {
                buf.extend_from_slice(&w.to_le_bytes());
            }
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_serializer_basic() {
        let header = Header::new("test".to_string(), "transformer".to_string());
        let temp_file = NamedTempFile::new().unwrap();
        let result = NflSerializer::serialize(&header, temp_file.path().to_str().unwrap());
        assert!(result.is_ok());
    }
}
