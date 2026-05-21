//! BF16 (Brain Float 16) support for critical layers

/// BF16 quantizer - 16-bit float for attention and output layers
pub struct Bf16Quantizer;

impl Bf16Quantizer {
    /// Convert float32 to BF16
    pub fn quantize(value: f32) -> u16 {
        let bits = value.to_bits();
        ((bits + 0x7FFF + ((bits >> 16) & 1)) >> 16) as u16
    }

    /// Convert BF16 back to float32
    pub fn dequantize(bf16: u16) -> f32 {
        let bits = (bf16 as u32) << 16;
        f32::from_bits(bits)
    }

    /// Batch convert float32 slice to BF16
    pub fn quantize_batch(values: &[f32]) -> Vec<u16> {
        values.iter().map(|&v| Self::quantize(v)).collect()
    }

    /// Batch convert BF16 slice to float32
    pub fn dequantize_batch(values: &[u16]) -> Vec<f32> {
        values.iter().map(|&v| Self::dequantize(v)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bf16_conversion() {
        let original = 1.5f32;
        let quantized = Bf16Quantizer::quantize(original);
        let reconstructed = Bf16Quantizer::dequantize(quantized);
        assert!((original - reconstructed).abs() < 0.01);
    }
}
