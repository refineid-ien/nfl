//! Example of tokenizer usage

use nfl_tokenizer::Tokenizer;

fn main() {
    println!("=== NFL Tokenizer Example ===\n");

    let mut tokenizer = Tokenizer::new(50000);

    // Add some common tokens
    let tokens = vec![
        ("hello", 1u32),
        ("world", 2u32),
        ("how", 3u32),
        ("are", 4u32),
        ("you", 5u32),
    ];

    for (token, id) in tokens {
        tokenizer.add_token(token.to_string(), id);
    }

    // Add special tokens
    tokenizer.add_special_token("[PAD]".to_string(), 0);
    tokenizer.add_special_token("[EOS]".to_string(), 50000);
    tokenizer.add_special_token("[BOS]".to_string(), 50001);

    println!("Tokenizer initialized with {} vocab size\n", tokenizer.vocab_size);

    // Encode text
    let text = "hello world";
    let token_ids = tokenizer.encode(text);
    println!("Text: '{}'", text);
    println!("Encoded: {:?}\n", token_ids);

    // Decode back
    let decoded = tokenizer.decode(&token_ids);
    println!("Decoded: '{}'", decoded);

    // Token lookup
    println!("\n=== Token Lookup ===");
    if let Some(id) = tokenizer.get_token_id("hello") {
        println!("'hello' -> ID: {}", id);
    }

    if let Some(token) = tokenizer.get_token_string(2) {
        println!("ID 2 -> '{}'", token);
    }

    // Special tokens
    println!("\n=== Special Tokens ===");
    if let Some(pad_id) = tokenizer.get_token_id("[PAD]") {
        println!("[PAD] token ID: {}", pad_id);
    }
}
