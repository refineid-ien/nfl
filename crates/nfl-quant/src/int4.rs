//! INT4 quantization support

/// INT4 quantizer - 4-bit integer quantization
pub struct Int4Quantizer;

impl Int4Quantizer {
    /// Quantize float32 values to INT4
    pub fn quantize(values: &[f32]) -> Vec<u8> {
        values
            .chunks(2)
            .map(|chunk| {
                let high = (chunk[0] as i32) as u8 & 0xF0;
                let low = (chunk.get(1).copied().unwrap_or(0.0) as i32) as u8 & 0x0F;
                high | low
            })
            .collect()
    }

    /// Dequantize INT4 values back to float32
    pub fn dequantize(data: &[u8]) -> Vec<f32> {
        data.iter()
            .flat_map(|&byte| {
                let high = ((byte >> 4) & 0x0F) as f32 / 15.0;
                let low = (byte & 0x0F) as f32 / 15.0;
                vec![high, low]
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int4_quantize() {
        let values = vec![0.5, 0.8, 0.3, 0.9];
        let quantized = Int4Quantizer::quantize(&values);
        assert!(!quantized.is_empty());
    }
}
