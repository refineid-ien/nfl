//! NFL file loading and validation

use crate::error::{NflError, NflResult};
use crate::format::NflFormat;
use crate::header::Header;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const MIN_FILE_SIZE: usize = 12; // Magic (4) + Header length (8)
const MAX_HEADER_SIZE: usize = 10 * 1024 * 1024; // 10 MB max header

/// NFL file loader - validates and parses NFL files
pub struct NflLoader;

impl NflLoader {
    /// Load and validate an NFL file
    pub fn load<P: AsRef<Path>>(path: P) -> NflResult<NflFile> {
        let path_ref = path.as_ref();
        
        // Check file exists
        if !path_ref.exists() {
            return Err(NflError::Custom(
                format!("File not found: {:?}", path_ref),
            ));
        }

        let mut file = File::open(path_ref)?;
        let mut buffer = vec![];
        
        // Read file with size limit check
        const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024 * 1024; // 100 GB
        let metadata = file.metadata()?;
        
        if metadata.len() > MAX_FILE_SIZE {
            return Err(NflError::Custom(
                format!("File too large: {} bytes", metadata.len()),
            ));
        }
        
        file.read_to_end(&mut buffer)?;

        Self::from_bytes(&buffer)
    }

    /// Parse NFL from bytes with comprehensive validation
    pub fn from_bytes(bytes: &[u8]) -> NflResult<NflFile> {
        // Validate minimum size
        if bytes.len() < MIN_FILE_SIZE {
            return Err(NflError::FileTooSmall {
                expected: MIN_FILE_SIZE,
                actual: bytes.len(),
            });
        }

        // Check magic bytes
        if &bytes[0..4] != b"NFL1" {
            return Err(NflError::InvalidMagic {
                expected: b"NFL1".to_vec(),
                actual: bytes[0..4].to_vec(),
            });
        }

        // Parse header length (big-endian)
        let header_len = u64::from_be_bytes([
            bytes[4], bytes[5], bytes[6], bytes[7], bytes[8], bytes[9], bytes[10], bytes[11],
        ]);

        // Validate header length
        if header_len == 0 {
            return Err(NflError::InvalidHeaderLength(
                "Header length cannot be zero".to_string(),
            ));
        }

        if header_len as usize > MAX_HEADER_SIZE {
            return Err(NflError::InvalidHeaderLength(
                format!("Header too large: {} bytes (max: {})", header_len, MAX_HEADER_SIZE),
            ));
        }

        // Check if file is large enough for header
        let header_end = 12 + header_len as usize;
        if header_end > bytes.len() {
            return Err(NflError::FileTooSmall {
                expected: header_end,
                actual: bytes.len(),
            });
        }

        // Parse CBOR header with detailed error context
        let header: Header = serde_cbor::from_slice(&bytes[12..header_end])
            .map_err(|e| NflError::CborError(e))?;

        // Validate header
        if !header.validate() {
            return Err(NflError::Custom(
                format!(
                    "Invalid header metadata: version={}, name={}, vocab_size={}",
                    header.version, header.model_name, header.vocab_size
                ),
            ));
        }

        // Additional validation for critical fields
        if header.hidden_size == 0 {
            return Err(NflError::Custom(
                "Hidden size must be greater than zero".to_string(),
            ));
        }

        if header.num_layers == 0 {
            return Err(NflError::Custom(
                "Number of layers must be greater than zero".to_string(),
            ));
        }

        if header.num_attention_heads == 0 || header.hidden_size % header.num_attention_heads != 0 {
            return Err(NflError::Custom(
                "Invalid attention head configuration".to_string(),
            ));
        }

        Ok(NflFile {
            format: NflFormat::new(),
            header,
            raw_bytes: bytes.to_vec(),
            header_offset: 12,
            header_len: header_len as usize,
        })
    }

    /// Validate file without loading full content
    pub fn validate<P: AsRef<Path>>(path: P) -> NflResult<()> {
        let path_ref = path.as_ref();
        let file = File::open(path_ref)?;
        let metadata = file.metadata()?;

        if metadata.len() < MIN_FILE_SIZE as u64 {
            return Err(NflError::FileTooSmall {
                expected: MIN_FILE_SIZE,
                actual: metadata.len() as usize,
            });
        }

        // Read header only
        let mut file = File::open(path_ref)?;
        let mut header_buf = vec![0u8; MIN_FILE_SIZE + MAX_HEADER_SIZE];
        let read_size = file.read(&mut header_buf)?;

        if read_size < MIN_FILE_SIZE {
            return Err(NflError::FileTooSmall {
                expected: MIN_FILE_SIZE,
                actual: read_size,
            });
        }

        // Check magic bytes
        if &header_buf[0..4] != b"NFL1" {
            return Err(NflError::InvalidMagic {
                expected: b"NFL1".to_vec(),
                actual: header_buf[0..4].to_vec(),
            });
        }

        Ok(())
    }
}

/// Loaded NFL file structure
#[derive(Debug)]
pub struct NflFile {
    pub format: NflFormat,
    pub header: Header,
    pub raw_bytes: Vec<u8>,
    pub header_offset: usize,
    pub header_len: usize,
}

impl NflFile {
    pub fn is_valid(&self) -> bool {
        self.format.is_valid() && self.header.validate()
    }

    pub fn total_size(&self) -> u64 {
        self.raw_bytes.len() as u64
    }

    /// Get file information as formatted string
    pub fn info(&self) -> String {
        format!(
            "Model: {}\nArchitecture: {}\nVersion: {}\nVocab Size: {}\nHidden Size: {}\nNum Layers: {}\nAttention Heads: {}\nMax Seq Length: {}\nTotal Size: {} bytes",
            self.header.model_name,
            self.header.architecture,
            self.header.version,
            self.header.vocab_size,
            self.header.hidden_size,
            self.header.num_layers,
            self.header.num_attention_heads,
            self.header.max_sequence_length,
            self.total_size()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_minimal_nfl() -> Vec<u8> {
        let mut data = vec![];
        data.extend_from_slice(b"NFL1"); // Magic
        
        let header = crate::header::Header::new(
            "test".to_string(),
            "transformer".to_string(),
        );
        let header_bytes = serde_cbor::to_vec(&header).unwrap();
        data.extend_from_slice(&(header_bytes.len() as u64).to_be_bytes());
        data.extend_from_slice(&header_bytes);
        data
    }

    #[test]
    fn test_valid_file() {
        let data = create_minimal_nfl();
        let result = NflLoader::from_bytes(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_magic() {
        let mut data = create_minimal_nfl();
        data[0] = b'X';
        let result = NflLoader::from_bytes(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_file_too_small() {
        let data = vec![0u8; 5];
        let result = NflLoader::from_bytes(&data);
        assert!(matches!(result, Err(NflError::FileTooSmall { .. })));
    }

    #[test]
    fn test_header_length_zero() {
        let mut data = vec![];
        data.extend_from_slice(b"NFL1");
        data.extend_from_slice(&0u64.to_be_bytes()); // Zero header length
        let result = NflLoader::from_bytes(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_file_validation() {
        let mut file = NamedTempFile::new().unwrap();
        let data = create_minimal_nfl();
        file.write_all(&data).unwrap();
        file.flush().unwrap();

        let result = NflLoader::validate(file.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_file_info() {
        let data = create_minimal_nfl();
        let nfl = NflLoader::from_bytes(&data).unwrap();
        let info = nfl.info();
        assert!(info.contains("test"));
        assert!(info.contains("transformer"));
    }
}
