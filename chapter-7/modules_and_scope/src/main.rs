mod garden;
use crate::garden::vegetables::Aspargus;

fn main() {
    let plant = Aspargus {};

    println!("I'm growing {plant:?}!")
}
