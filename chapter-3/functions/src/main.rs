use std::io;

fn main() {
    let mut name = String::new();

    println!("What's your name?");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to parse variable");

    another_function(name);
}

// Simple function with parameters
fn another_function(name: String) {
    println!("Hello, {name}!");
}
