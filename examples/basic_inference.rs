#!/usr/bin/env rust-script

//! Basic example of loading and using an NFL model
//!
//! cargo run --example basic_inference

use anyhow::Result;
use nfl_core::loader::NflLoader;
use nfl_engine::InferenceEngine;

fn main() -> Result<()> {
    // In a real scenario, you would load an actual .nfl file
    // For this example, we'll show the structure

    println!("=== NFL Basic Inference Example ===\n");

    // Create a model header as if it was loaded from a file
    let header = nfl_core::Header::new(
        "example-model".to_string(),
        "transformer".to_string(),
    );

    println!("Model Name: {}", header.model_name);
    println!("Architecture: {}", header.architecture);
    println!("Vocab Size: {}", header.vocab_size);
    println!("Hidden Size: {}", header.hidden_size);
    println!("Num Layers: {}", header.num_layers);
    println!("Max Sequence Length: {}", header.max_sequence_length);

    // Create inference engine
    let engine = InferenceEngine::new(header);

    // Generate some text
    let prompt = "Hello, world!";
    let output = engine.generate(prompt, 50);

    println!("\nInput: {}", prompt);
    println!("Output: {}", output);

    Ok(())
}
