// https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html
fn main() {
    // Variables
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // Constants
    const CONSTANT_VALUE: u32 = 32;
    println!("The value of CONSTANT_VALUE is {CONSTANT_VALUE}");

    // Shadowing
    let x = 5;
    println!("The value of x is {x}");
    {
        let x = "Hi guys";
        println!("The value of x is {x}");
    }
    println!("The value of x is {x}");

    // Data types
    let integer: i32 = 1_000;
    println!("The value of integer is {integer}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of tuple is ({x}, {y}, {z})");

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of tuple is [{}, {}, {}, {}, {}]", arr[0], arr[1], arr[2], arr[3], arr[4]);

    // Condition
    let condition = true;
    let nuumber = if condition { 5 } else { 6 };

    let mut counter = 0;
    let result = loop {
        if counter == 10 {
            break counter * 2;
        }
        counter += 1;
    };
    println!("The result is {result}");

    for element in arr {
        println!("The value is {element}");
    }
}