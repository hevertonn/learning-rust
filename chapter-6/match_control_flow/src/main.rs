#[derive(Debug)]
enum UsStates {
    Florida,
    California,
}

enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates),
}

fn main() {
    // let coin = Coins::Quarter(UsStates::Florida);
    //
    // println!("{}", value_in_cents(coin));
    //
    // let eigth = 8;
    // //
    // let nine = dbg!(plus_one(Some(eigth)));
    // //
    // let none = dbg!(plus_one(None));

    match_a_number(6);
    match_a_number_or_do_nothing(5);
}

fn value_in_cents(coins: Coins) -> u8 {
    match coins {
        Coins::Penny => {
            println!("Lucky Penny");
            1
        }
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter(state) => {
            println!("The state of the Quarter is: {state:?}");
            25
        }
    }
}
//
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }
//
fn match_a_number(x: u32) {
    match x {
        5 => println!("Your number is correct"),
        other => println!("{other}"),
    }
}

fn match_a_number_or_do_nothing(x: u32) {
    match x {
        5 => println!("Your number is correct"),
        9 => println!("Your number is too big"),
        _ => (),
    }
}
