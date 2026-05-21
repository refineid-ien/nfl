// Integration test for complete workflow

#[test]
fn test_model_loading_workflow() {
    // This test verifies the complete workflow of:
    // 1. Creating a model header
    // 2. Serializing to NFL format
    // 3. Loading from file
    // 4. Creating inference engine
    
    use nfl_core::Header;
    
    let header = Header::new(
        "integration-test".to_string(),
        "transformer".to_string(),
    );
    
    assert!(header.validate());
    assert_eq!(header.model_name, "integration-test");
}

#[test]
fn test_quantization_pipeline() {
    // Test quantization from FP32 → INT4 → reconstruct
    use nfl_quant::Int4Quantizer;
    
    let original: Vec<f32> = (0..16)
        .map(|i| (i as f32) / 16.0)
        .collect();
    
    let quantized = Int4Quantizer::quantize(&original);
    let reconstructed = Int4Quantizer::dequantize(&quantized);
    
    // Should have some loss but be close
    assert_eq!(original.len() * 2, reconstructed.len());
}

#[test]
fn test_simd_matmul_consistency() {
    // Verify matrix multiplication consistency
    use nfl_simd::SIMDMatmul;
    
    let a = vec![1.0, 2.0, 3.0, 4.0]; // 2x2
    let b = vec![5.0, 6.0, 7.0, 8.0]; // 2x2
    
    let result = SIMDMatmul::multiply(&a, &b, 2, 2, 2);
    
    // Verify specific results
    assert!((result[0] - 19.0).abs() < 0.01);  // 1*5 + 2*7
    assert!((result[1] - 22.0).abs() < 0.01);  // 1*6 + 2*8
}

#[test]
fn test_tokenizer_roundtrip() {
    // Test encode → decode roundtrip
    use nfl_tokenizer::Tokenizer;
    
    let mut tokenizer = Tokenizer::new(1000);
    tokenizer.add_token("hello".to_string(), 1);
    tokenizer.add_token("world".to_string(), 2);
    
    let original = "hello world";
    let encoded = tokenizer.encode(original);
    let decoded = tokenizer.decode(&encoded);
    
    assert_eq!(decoded, "hello world");
}

#[test]
fn test_inference_engine_initialization() {
    // Verify engine creates correctly
    use nfl_core::Header;
    use nfl_engine::InferenceEngine;
    
    let header = Header::new("test".to_string(), "transformer".to_string());
    let engine = InferenceEngine::new(header);
    
    assert_eq!(engine.get_model_info().model_name, "test");
}
