use std::io;
use std::collections::HashMap;

pub fn run_word_counter() {
    println!("\n--- Word Counter ---");
    println!("Enter a sentence:");

    let mut sentence = String::new();
    io::stdin().read_line(&mut sentence)
        .expect("Failed to read line");

    let words: Vec<String> = sentence
        .to_lowercase() // Convert to lowercase for case-insensitive counting
        .split_whitespace() // Split by whitespace
        .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string()) // Remove punctuation and convert to String
        .filter(|s| !s.is_empty()) // Filter out empty strings that might result from punctuation removal
        .collect();

    println!("Words in sentence: {:?}", words);

    println!("\nEnter the word to count:");
    let mut word_to_count = String::new();
    io::stdin().read_line(&mut word_to_count)
        .expect("Failed to read line");
    let word_to_count = word_to_count.trim().to_lowercase();

    let count = count_word_occurrences(&words, &word_to_count);

    println!("The word {} appears {} times.", word_to_count, count);
}

fn count_word_occurrences(words: &Vec<String>, target_word: &str) -> usize {
    let mut word_counts = HashMap::new();

    for word in words {
        *word_counts.entry(word.clone()).or_insert(0) += 1;
    }

    *word_counts.get(target_word).unwrap_or(&0)
}

