struct Color(u32, u32, u32);

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(Color)
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit!"),
            Message::Move {x, y} => println!("Move to ({x}, {y})"),
            Message::Write(message) => println!("Message: {message}"),
            Message::ChangeColor(Color(r, g, b)) => println!("Color changed to ({r}, {g}, {b})"),
        }
    }
}
fn main() {
    let m = Message::Write("Hello, world!".to_string());
    m.call();
}
