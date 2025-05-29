use ring::digest::{Context, SHA256};
use std::fmt::Write;

const DIFFICULTY: usize = 4;

fn meets_difficulty(hash_str: &str, difficulty: usize) -> bool {
    hash_str.starts_with(&"0".repeat(difficulty))
}

fn main() {
    // Data to be hashed
    let data = "Hello, World!";

    let mut nonce = 0u64;

    loop {
        let mut context = Context::new(&SHA256); // Create a new context in each iteration

        // Update the context with the data and nonce
        context.update(data.as_bytes());
        context.update(&nonce.to_be_bytes());

        // Finish the hashing process and get the digest
        let digest = context.finish();
        let mut actual_hex = String::new();

        // Use the write! macro instead of map and format for hex conversion
        for &byte in digest.as_ref() {
            write!(&mut actual_hex, "{:02x}", byte).expect("Failed to write hex");
        }

        // Print the current hash result and nonce for demonstration
        println!("Nonce: {}, Hash: {}", nonce, actual_hex);

        // Check if the hash meets the difficulty target
        if meets_difficulty(&actual_hex, DIFFICULTY) {
            println!("Hash meets the difficulty target.");
            break;
        }

        // Increment the nonce for the next iteration
        nonce += 1;
    }
}
