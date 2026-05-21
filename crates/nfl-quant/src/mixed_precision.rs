//! Mixed precision routing based on layer type

use nfl_core::format::Precision;

/// Router for selecting quantization precision per layer
pub struct MixedPrecisionRouter;

impl MixedPrecisionRouter {
    /// Route layer to appropriate precision based on type and importance
    pub fn route(layer_type: &str) -> Precision {
        match layer_type {
            "attention" => Precision::BF16,
            "mlp" => Precision::INT4,
            "compression" => Precision::INT2,
            "norm" | "output" => Precision::BF16,
            _ => Precision::INT4,
        }
    }

    /// Get quantization bits for precision
    pub fn bits_for_precision(precision: Precision) -> u32 {
        match precision {
            Precision::BF16 => 16,
            Precision::INT4 => 4,
            Precision::INT2 => 2,
            Precision::FP32 => 32,
        }
    }

    /// Estimate compression ratio
    pub fn compression_ratio(original_bits: u32, target_precision: Precision) -> f32 {
        let target_bits = Self::bits_for_precision(target_precision);
        original_bits as f32 / target_bits as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_routing() {
        assert_eq!(MixedPrecisionRouter::route("attention"), Precision::BF16);
        assert_eq!(MixedPrecisionRouter::route("mlp"), Precision::INT4);
    }

    #[test]
    fn test_compression_ratio() {
        let ratio = MixedPrecisionRouter::compression_ratio(32, Precision::INT4);
        assert_eq!(ratio, 8.0);
    }
}
