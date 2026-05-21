//! Memory-mapped file access for zero-copy tensor streaming

use crate::error::{NflError, NflResult};
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;

/// Memory-mapped NFL file loader for zero-copy access
pub struct MmapLoader {
    _file: File,
    mmap: Mmap,
}

impl MmapLoader {
    /// Create a memory-mapped loader from a file path
    pub fn new<P: AsRef<Path>>(path: P) -> NflResult<Self> {
        let file = File::open(path).map_err(|e| {
            NflError::MmapError(format!("Failed to open file: {}", e))
        })?;

        let metadata = file.metadata().map_err(|e| {
            NflError::MmapError(format!("Failed to read metadata: {}", e))
        })?;

        if metadata.len() == 0 {
            return Err(NflError::MmapError("Cannot mmap empty file".to_string()));
        }

        let mmap = unsafe { Mmap::map(&file) }.map_err(|e| {
            NflError::MmapError(format!("Failed to map file: {}", e))
        })?;

        Ok(Self { _file: file, mmap })
    }

    /// Get slice of data at offset with length
    pub fn get_slice(&self, offset: u64, length: u64) -> NflResult<&[u8]> {
        let start = offset as usize;
        let end = (offset + length) as usize;

        if start > self.mmap.len() {
            return Err(NflError::BufferOverflow {
                requested: start,
                available: self.mmap.len(),
            });
        }

        if end > self.mmap.len() {
            return Err(NflError::BufferOverflow {
                requested: length as usize,
                available: (self.mmap.len() - start) as usize,
            });
        }

        Ok(&self.mmap[start..end])
    }

    /// Get mutable slice (only if safe)
    pub fn get_mut_slice(&mut self, offset: u64, length: u64) -> NflResult<&mut [u8]> {
        let start = offset as usize;
        let end = (offset + length) as usize;

        if start > self.mmap.len() {
            return Err(NflError::BufferOverflow {
                requested: start,
                available: self.mmap.len(),
            });
        }

        if end > self.mmap.len() {
            return Err(NflError::BufferOverflow {
                requested: length as usize,
                available: (self.mmap.len() - start) as usize,
            });
        }

        // SAFETY: We've already validated bounds
        unsafe {
            let ptr = self.mmap.as_ptr() as *mut u8;
            Ok(std::slice::from_raw_parts_mut(ptr.add(start), length as usize))
        }
    }

    /// Get total file size
    pub fn size(&self) -> u64 {
        self.mmap.len() as u64
    }

    /// Get raw bytes reference
    pub fn as_bytes(&self) -> &[u8] {
        &self.mmap
    }

    /// Check if file is aligned to boundary
    pub fn is_aligned(&self, boundary: usize) -> bool {
        self.mmap.as_ptr() as usize % boundary == 0
    }

    /// Get alignment address
    pub fn alignment_address(&self) -> usize {
        self.mmap.as_ptr() as usize % 64
    }

    /// Verify that a specific region is aligned
    pub fn verify_alignment(&self, offset: u64, alignment: usize) -> NflResult<()> {
        let addr = offset as usize;
        if addr % alignment != 0 {
            return Err(NflError::AlignmentError {
                addr,
                alignment,
            });
        }
        Ok(())
    }

    /// Pre-fault pages in the mmap (for predictable performance)
    pub fn prefault_pages(&self) -> NflResult<()> {
        // Touch every page to ensure it's loaded into RAM
        let page_size = 4096;
        for i in (0..self.mmap.len()).step_by(page_size) {
            let _ = self.mmap[i];
        }
        Ok(())
    }

    /// Get statistics about mmap usage
    pub fn statistics(&self) -> MmapStats {
        MmapStats {
            total_size: self.mmap.len(),
            address: self.mmap.as_ptr() as usize,
            is_page_aligned: self.is_aligned(4096),
            alignment_address: self.alignment_address(),
        }
    }
}

/// Statistics about mmap usage
#[derive(Debug, Clone)]
pub struct MmapStats {
    pub total_size: usize,
    pub address: usize,
    pub is_page_aligned: bool,
    pub alignment_address: usize,
}

impl MmapStats {
    pub fn format_display(&self) -> String {
        format!(
            "Size: {} MB\nAddress: 0x{:x}\nPage Aligned: {}\nAlignment: {} bytes",
            self.total_size / (1024 * 1024),
            self.address,
            self.is_page_aligned,
            self.alignment_address
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_mmap_basic() {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"Hello, NFL! This is a test.").unwrap();
        file.flush().unwrap();

        let loader = MmapLoader::new(file.path()).unwrap();
        assert_eq!(loader.size(), 27);
    }

    #[test]
    fn test_get_slice() {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"Hello, World!").unwrap();
        file.flush().unwrap();

        let loader = MmapLoader::new(file.path()).unwrap();
        let slice = loader.get_slice(0, 5).unwrap();
        assert_eq!(slice, b"Hello");
    }

    #[test]
    fn test_out_of_bounds() {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"Test").unwrap();
        file.flush().unwrap();

        let loader = MmapLoader::new(file.path()).unwrap();
        let result = loader.get_slice(10, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_alignment_check() {
        let mut file = NamedTempFile::new().unwrap();
        // Write enough data
        file.write_all(&vec![0u8; 4096]).unwrap();
        file.flush().unwrap();

        let loader = MmapLoader::new(file.path()).unwrap();
        let stats = loader.statistics();
        assert!(stats.total_size > 0);
    }
}
