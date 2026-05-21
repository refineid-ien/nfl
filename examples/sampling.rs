//! Example of inference engine with sampling

use nfl_engine::{InferenceEngine, Sampler, SamplingStrategy};
use nfl_core::Header;

fn main() {
    println!("=== NFL Inference & Sampling Example ===\n");

    // Create engine
    let header = Header::new("example-model".to_string(), "transformer".to_string());
    let engine = InferenceEngine::new(header);

    println!("Engine initialized with model: {}\n", engine.get_model_info().model_name);

    // Example logits (would come from model forward pass)
    let logits = vec![0.5, 2.0, 1.5, 0.8, 3.0, 0.2];

    // Greedy sampling
    println!("=== Greedy Sampling ===");
    let sampler = Sampler::new(SamplingStrategy::Greedy);
    let token = sampler.sample(&logits);
    println!("Selected token: {}", token);

    // Temperature sampling
    println!("\n=== Temperature Sampling (temperature=1.0) ===");
    let sampler = Sampler::new(SamplingStrategy::Temperature(1.0));
    for i in 0..3 {
        let token = sampler.sample(&logits);
        println!("Sample {}: token {}", i + 1, token);
    }

    // Top-K sampling
    println!("\n=== Top-K Sampling (k=3, temperature=0.7) ===");
    let sampler = Sampler::new(SamplingStrategy::TopK {
        k: 3,
        temperature: 0.7,
    });
    for i in 0..3 {
        let token = sampler.sample(&logits);
        println!("Sample {}: token {}", i + 1, token);
    }

    // Top-P (nucleus) sampling
    println!("\n=== Top-P Sampling (p=0.9, temperature=0.8) ===");
    let sampler = Sampler::new(SamplingStrategy::TopP {
        p: 0.9,
        temperature: 0.8,
    });
    for i in 0..3 {
        let token = sampler.sample(&logits);
        println!("Sample {}: token {}", i + 1, token);
    }

    println!("\nNote: Different sampling strategies balance diversity vs quality");
}
