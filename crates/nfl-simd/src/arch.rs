//! Architecture detection and target selection

use std::sync::OnceLock;

static SIMD_TARGET: OnceLock<SimdTarget> = OnceLock::new();

/// Available SIMD targets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimdTarget {
    Generic,
    Avx2,
    Neon,
    WasmSimd,
    Sse2,
}

impl SimdTarget {
    pub fn name(&self) -> &'static str {
        match self {
            SimdTarget::Generic => "generic",
            SimdTarget::Avx2 => "avx2",
            SimdTarget::Neon => "neon",
            SimdTarget::WasmSimd => "wasm_simd",
            SimdTarget::Sse2 => "sse2",
        }
    }

    pub fn bits(&self) -> usize {
        match self {
            SimdTarget::Generic => 32,
            SimdTarget::Avx2 => 256,
            SimdTarget::Neon => 128,
            SimdTarget::WasmSimd => 128,
            SimdTarget::Sse2 => 128,
        }
    }
}

/// Detect available SIMD capabilities
pub fn detect_simd() -> SimdTarget {
    *SIMD_TARGET.get_or_init(|| {
        #[cfg(target_arch = "x86_64")]
        if is_x86_feature_detected!("avx2") {
            return SimdTarget::Avx2;
        }

        #[cfg(target_arch = "x86_64")]
        if is_x86_feature_detected!("sse2") {
            return SimdTarget::Sse2;
        }

        #[cfg(target_arch = "aarch64")]
        if cfg!(target_feature = "neon") {
            return SimdTarget::Neon;
        }

        #[cfg(target_arch = "wasm32")]
        if cfg!(target_feature = "simd128") {
            return SimdTarget::WasmSimd;
        }

        SimdTarget::Generic
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_simd() {
        let target = detect_simd();
        assert_ne!(target, SimdTarget::Generic);
    }
}
