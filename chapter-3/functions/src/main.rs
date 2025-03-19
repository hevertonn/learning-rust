use std::io;

fn main() {
    let name = get_name();
    another_function(name);
}

// function with return value
fn get_name() -> String {
    let mut name = String::new(); // statement

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line!");

    name // expression
}

// simple function with parameters
fn another_function(name: String) {
    println!("Hello, {name}!");
}
