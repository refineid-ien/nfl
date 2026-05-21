//! Example of quantization workflow

use nfl_quant::{Int4Quantizer, Int2Quantizer, Bf16Quantizer, MixedPrecisionRouter};
use nfl_core::format::Precision;

fn main() {
    println!("=== NFL Quantization Example ===\n");

    // Example float values
    let values = vec![0.5, 0.8, 0.3, 0.9, 0.1, 0.7];

    println!("Original values (FP32): {:?}", values);

    // INT4 Quantization
    let quantized_int4 = Int4Quantizer::quantize(&values);
    println!("INT4 quantized ({} bytes): {:?}", quantized_int4.len(), quantized_int4);

    // INT2 Quantization
    let quantized_int2 = Int2Quantizer::quantize(&values[0..4]);
    println!("INT2 quantized ({} bytes): {:?}", quantized_int2.len(), quantized_int2);

    // BF16 Quantization
    let bf16_values = Bf16Quantizer::quantize_batch(&values);
    println!("BF16 quantized ({} bytes): {:?}", bf16_values.len() * 2, &bf16_values[..3]);

    // Compression Ratios
    println!("\n=== Compression Analysis ===");
    println!("FP32 (32-bit): 100%");
    println!(
        "INT4 (4-bit): {:.1}%",
        MixedPrecisionRouter::compression_ratio(32, Precision::INT4) * 100.0 / 32.0
    );
    println!(
        "INT2 (2-bit): {:.1}%",
        MixedPrecisionRouter::compression_ratio(32, Precision::INT2) * 100.0 / 32.0
    );
    println!(
        "BF16 (16-bit): {:.1}%",
        MixedPrecisionRouter::compression_ratio(32, Precision::BF16) * 100.0 / 32.0
    );

    // Layer precision routing
    println!("\n=== Layer Precision Routing ===");
    let layers = vec!["attention", "mlp", "compression", "norm"];
    for layer in layers {
        let precision = MixedPrecisionRouter::route(layer);
        println!("{}: {:?}", layer, precision);
    }
}
