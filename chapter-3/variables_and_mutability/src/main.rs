fn main() {
    println!("Hello, World!");

    variable_declaration();
    mutable_variable();
    constants();
    shadowing();
    scope();
}

fn variable_declaration() {
    let some_variable = 4; // n is imutable

    println!("{some_variable}");

    // some_variable = 5; // this throws an error
}

fn mutable_variable() {
    let mut some_variable = 5;

    println!("{some_variable}");

    some_variable = 6;

    println!("{some_variable}");

    // some_variable = "Hello World!" // this throws an error because only the value of the
    // "some_variable" can be changed and the type must be the same
}

fn constants() {
    const MINUTES_IN_A_WEEK: i32 = 60 * 24 * 7; // in a constant declaration the type must be
                                                // noted

    println!("{MINUTES_IN_A_WEEK}");
}

fn shadowing() {
    let some_variable = "some word";

    let some_variable = some_variable.len(); // this is like declaring a new variable and
                                             // deleting the old one

    println!("{some_variable}");
}

fn scope() {
    let some_number = 1; // declaration here

    {
        let some_number = some_number + 9; // shadowing here and the value of the variable now
                                           // is 10
        println!("{some_number}");
    }
    // here the value 10 goes out of the scope and the value returns to 1
}
