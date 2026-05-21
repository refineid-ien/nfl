//! INT2 quantization support (experimental)

/// INT2 quantizer - 2-bit integer quantization (experimental)
pub struct Int2Quantizer;

impl Int2Quantizer {
    /// Quantize float32 values to INT2 (aggressive compression)
    pub fn quantize(values: &[f32]) -> Vec<u8> {
        values
            .chunks(4)
            .map(|chunk| {
                let mut byte = 0u8;
                for (i, &val) in chunk.iter().enumerate() {
                    let quantized = ((val.max(0.0).min(1.0) * 3.0) as u8) & 0x03;
                    byte |= quantized << (i * 2);
                }
                byte
            })
            .collect()
    }

    /// Dequantize INT2 values back to float32
    pub fn dequantize(data: &[u8]) -> Vec<f32> {
        data.iter()
            .flat_map(|&byte| {
                (0..4)
                    .map(|i| {
                        let val = ((byte >> (i * 2)) & 0x03) as f32;
                        val / 3.0
                    })
                    .collect::<Vec<f32>>()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int2_quantize() {
        let values = vec![0.0, 0.33, 0.66, 1.0];
        let quantized = Int2Quantizer::quantize(&values);
        assert!(!quantized.is_empty());
    }
}
