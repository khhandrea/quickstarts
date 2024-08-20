enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {println!("You quit");},
            Message::Move {x, y} => {println!("Move to ({}, {})", x, y);},
            Message::Write(msg) => {println!("Write message: {}", msg);},
            Message::ChangeColor(r, g, b) => {println!("Change color to ({}, {}, {})", r, g, b);},
        }
    }
}


fn main() {
    let x = 10i8;
    let y: Option<i8> = Some(5);
    let z: i8 = if let Some(y) = y {
        x + y 
    } else {
        0i8
    };
    println!("x + y = {}", z);

    let mut msg = Message::Quit;
    msg.call();
    msg = Message::Move {x: 24, y: 32};
    msg.call();
    msg = Message::Write("Hello, world!".to_string());
    msg.call();
    msg = Message::ChangeColor(255, 0, 255);
    msg.call();

    if let Message::ChangeColor(r, g, b) = msg {
        println!("({}, {}, {}) from if let phrase", r, g, b);
    } else {
        println!("It's not color in if let phrase");
    }
}
