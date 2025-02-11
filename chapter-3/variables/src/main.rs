fn main() {
    let n = 4; // n is imutable

    println!("{n}");

    // n = 5; this thows an error

    let mut a = 5; // this is valid

    println!("{a}");

    a = 6;

    println!("{a}");

    // scope

    {
        // n is declared here
        let n = 9;
        println!("{n}");
    }
    // and is no long valid here

    // shadowing

    let n = 5; // this is valid too, but it's like creating a new variable and the past is
               // deleted

    println!("{n}");

    // Constants

    const SOME_NUMBER: i32 = 8; // constant declaration, the type must be annotated and is often named in
                                // upper case

    println!("{SOME_NUMBER}");
}
