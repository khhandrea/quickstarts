#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 64,
        height: 48
    };

    let rect2 = Rectangle {
        width: 32,
        height: 16
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("rect2 is {rect2:?}");
    println!("can rect1 hold rect2?: {}", rect1.can_hold(&rect2));
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width) && (self.height > other.height)
    }
}