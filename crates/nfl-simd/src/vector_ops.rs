//! SIMD vector operations

pub struct SIMDVectorOps;

impl SIMDVectorOps {
    /// Vector dot product
    pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
        a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
    }

    /// Vector addition
    pub fn add(a: &[f32], b: &[f32]) -> Vec<f32> {
        a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
    }

    /// Vector subtraction
    pub fn sub(a: &[f32], b: &[f32]) -> Vec<f32> {
        a.iter().zip(b.iter()).map(|(x, y)| x - y).collect()
    }

    /// Vector scalar multiplication
    pub fn scale(v: &[f32], scalar: f32) -> Vec<f32> {
        v.iter().map(|x| x * scalar).collect()
    }

    /// Vector L2 norm
    pub fn norm(v: &[f32]) -> f32 {
        v.iter().map(|x| x * x).sum::<f32>().sqrt()
    }

    /// Vector normalization
    pub fn normalize(v: &[f32]) -> Vec<f32> {
        let norm = Self::norm(v);
        if norm == 0.0 {
            v.to_vec()
        } else {
            v.iter().map(|x| x / norm).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        assert_eq!(SIMDVectorOps::dot_product(&a, &b), 32.0);
    }

    #[test]
    fn test_norm() {
        let v = vec![3.0, 4.0];
        assert_eq!(SIMDVectorOps::norm(&v), 5.0);
    }
}
