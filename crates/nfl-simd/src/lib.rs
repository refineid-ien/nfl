//! SIMD Acceleration Layer - AVX2, NEON, and WASM SIMD support

pub mod arch;
pub mod matmul;
pub mod activation;
pub mod vector_ops;

pub use arch::{SimdTarget, detect_simd};
pub use matmul::SIMDMatmul;
pub use activation::SIMDActivation;
pub use vector_ops::SIMDVectorOps;
