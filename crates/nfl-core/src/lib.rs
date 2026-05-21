//! NFL Core - File format parsing, header validation, and memory mapping
//!
//! This module provides the fundamental components for working with NFL files:
//! - Binary format parsing and validation
//! - Header and metadata decoding via CBOR
//! - Memory-mapped file access for zero-copy inference
//! - Index-based tensor access without full model loading

pub mod error;
pub mod format;
pub mod header;
pub mod loader;
pub mod mmap;

pub use error::{NflError, NflResult};
pub use format::NflFormat;
pub use header::Header;
pub use loader::NflLoader;
pub use mmap::MmapLoader;

pub const NFL_MAGIC: &[u8] = b"NFL1";
pub const HEADER_VERSION: u32 = 1;
