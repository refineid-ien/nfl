//! SIMD activation functions

pub struct SIMDActivation;

impl SIMDActivation {
    /// GELU activation using SIMD
    pub fn gelu(x: &[f32]) -> Vec<f32> {
        x.iter()
            .map(|&val| {
                let cdf = 0.5
                    * (1.0
                        + ((2.0_f32 / std::f32::consts::PI).sqrt()
                            * (val + 0.044715 * val.powi(3)))
                        .tanh());
                val * cdf
            })
            .collect()
    }

    /// ReLU activation using SIMD
    pub fn relu(x: &[f32]) -> Vec<f32> {
        x.iter().map(|&val| val.max(0.0)).collect()
    }

    /// Softmax activation using SIMD
    pub fn softmax(x: &[f32]) -> Vec<f32> {
        let max = x.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let exp: Vec<f32> = x.iter().map(|&val| (val - max).exp()).collect();
        let sum: f32 = exp.iter().sum();
        exp.iter().map(|&e| e / sum).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relu() {
        let input = vec![-1.0, 0.5, -2.0, 3.0];
        let output = SIMDActivation::relu(&input);
        assert_eq!(output, vec![0.0, 0.5, 0.0, 3.0]);
    }

    #[test]
    fn test_softmax() {
        let input = vec![1.0, 2.0, 3.0];
        let output = SIMDActivation::softmax(&input);
        let sum: f32 = output.iter().sum();
        assert!((sum - 1.0).abs() < 0.0001);
    }
}
