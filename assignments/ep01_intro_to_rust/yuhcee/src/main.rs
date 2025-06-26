use std::io;

fn main() {
    println!("Hello, Rusteaceans!");
    greeter();
}

fn greeter() {
    println!("What is your name?");
    let mut name = String::new();
    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read input");

    println!("Welcome to Rust, {}!", name.trim());
}