#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect2: Rectangle) -> bool {
        self.height > rect2.height && self.width > rect2.width
    }

    fn is_square(&self) -> bool {
        self.height == self.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 32,
        height: 22,
    };

    let rect2 = Rectangle {
        height: 20,
        width: 20,
    };

    let sq1 = Rectangle::square(32);

    println!("{rect1:#?}");
    println!("The area is {}", rect1.area());
    println!("rect1 can hold rect2: {}", rect1.can_hold(rect2));
    println!("is sq1 a square: {}", sq1.is_square());
}
