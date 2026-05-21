//! Example of SIMD operations

use nfl_simd::{SIMDMatmul, SIMDActivation, SIMDVectorOps, detect_simd};

fn main() {
    println!("=== NFL SIMD Example ===\n");

    // Detect SIMD capabilities
    let simd_target = detect_simd();
    println!("Detected SIMD target: {}", simd_target.name());
    println!("SIMD bit width: {} bits\n", simd_target.bits());

    // Vector operations
    println!("=== Vector Operations ===");
    let a = vec![1.0, 2.0, 3.0];
    let b = vec![4.0, 5.0, 6.0];

    let dot = SIMDVectorOps::dot_product(&a, &b);
    println!("Dot product: {}", dot);

    let sum = SIMDVectorOps::add(&a, &b);
    println!("Add result: {:?}", sum);

    let scaled = SIMDVectorOps::scale(&a, 2.0);
    println!("Scale by 2.0: {:?}", scaled);

    let norm = SIMDVectorOps::norm(&a);
    println!("L2 norm: {}", norm);

    // Activation functions
    println!("\n=== Activation Functions ===");
    let logits = vec![0.5, 1.0, 2.0, 0.0];

    let relu = SIMDActivation::relu(&logits);
    println!("ReLU output: {:?}", relu);

    let softmax = SIMDActivation::softmax(&logits);
    println!("Softmax output: {:?}", softmax);
    println!("Softmax sum: {}", softmax.iter().sum::<f32>());

    // Matrix multiplication
    println!("\n=== Matrix Multiplication ===");
    let a_mat = vec![1.0, 2.0, 3.0, 4.0]; // 2x2 matrix
    let b_mat = vec![5.0, 6.0, 7.0, 8.0]; // 2x2 matrix

    let result = SIMDMatmul::multiply(&a_mat, &b_mat, 2, 2, 2);
    println!("2x2 @ 2x2 = {:?}", result);
}
