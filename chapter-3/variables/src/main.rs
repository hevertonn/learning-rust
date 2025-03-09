fn main() {
    println!("Hello, World!");

    simple_variable_declaration();
    constants();
    mutable_variable();
    scope();
    shadowing();
}

fn simple_variable_declaration() {
    let some_variable = 4; // n is imutable

    println!("{some_variable}");

    // some_variable = 5; // this throws an error
}

fn constants() {
    const SOME_NUMBER: i32 = 8; // constant declaration, the type must be annotated and is often named in
                                // upper case

    println!("{SOME_NUMBER}");
}

fn mutable_variable() {
    let mut some_variable = 5; // this is valid but the data type will not change

    println!("{some_variable}");

    some_variable = 6;

    println!("{some_variable}");
}

fn scope() {
    {
        // some_variable is declared here
        let some_variable = 9;
        println!("{some_variable}");
    }
    // and is no long valid here

    // println!("{}", some_variable); // this throws an error
}

fn shadowing() {
    let some_variable = "some work";

    let some_variable = some_variable.len(); // this is valid too, but it's like creating a new variable and the past is
                                             // deleted

    println!("{some_variable}");
}
