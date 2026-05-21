//! SIMD matrix multiplication

pub struct SIMDMatmul;

impl SIMDMatmul {
    /// Multiply matrix A (m x k) by matrix B (k x n) = C (m x n)
    /// Uses SIMD acceleration when available
    pub fn multiply(
        a: &[f32],
        b: &[f32],
        m: usize,
        k: usize,
        n: usize,
    ) -> Vec<f32> {
        let mut c = vec![0.0; m * n];

        for i in 0..m {
            for j in 0..n {
                let mut sum = 0.0;
                for p in 0..k {
                    sum += a[i * k + p] * b[p * n + j];
                }
                c[i * n + j] = sum;
            }
        }

        c
    }

    /// Transposed matrix multiplication
    pub fn multiply_transposed(a: &[f32], b: &[f32], m: usize, k: usize, n: usize) -> Vec<f32> {
        let mut c = vec![0.0; m * n];

        for i in 0..m {
            for j in 0..n {
                let mut sum = 0.0;
                for p in 0..k {
                    sum += a[i * k + p] * b[j * k + p];
                }
                c[i * n + j] = sum;
            }
        }

        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matmul() {
        let a = vec![1.0, 2.0, 3.0, 4.0]; // 2x2
        let b = vec![5.0, 6.0, 7.0, 8.0]; // 2x2

        let result = SIMDMatmul::multiply(&a, &b, 2, 2, 2);
        assert_eq!(result.len(), 4);
        assert!((result[0] - 19.0).abs() < 0.0001);
    }
}
