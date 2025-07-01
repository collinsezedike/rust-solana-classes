use std::io;

fn main() {
    println!("Hello, world!");
    greeter();
}

fn greeter() {
    println!("What's your name: "); 
    let mut user_name = String::new();

    io::stdin().read_line(&mut user_name).expect("Failed to read name"); 
    println!("Hello, {}!", user_name.trim());
}