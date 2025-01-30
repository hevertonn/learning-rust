#[derive(Debug)]
struct User {
    active: bool,
    name: String,
    age: u8,
}

fn main() {
    let mut user1 = User {
        active: true,
        name: String::from("David"),
        age: dbg!(80 - 50),
    };

    let user2 = User {
        active: false,
        ..user1
    };

    user1.name = String::from("Elric");

    let user3 = create_user(String::from("Alfonso"), 50);

    println!("{}", user3.name);

    println!("{}", user2.name);

    println!("{}", user1.name);

    println!("{:#?}", user1);
}

fn create_user(name: String, age: u8) -> User {
    User {
        active: true,
        name,
        age,
    }
}
