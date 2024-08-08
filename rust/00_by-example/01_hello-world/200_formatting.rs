fn main() {
    let x = 75;
    let y = 10;

    let msg: String = format!("{} + {} = {}", x, y, x + y);
    println!("{}", msg);
    print!("This will not make newline ");
    eprintln!("This is an error!"); // Output to io::stderr

    println!("{} days", 31);
    println!("{0} + {1} = {1} + {0}", x, y);
    println!("{first} + {second} = {second} + {first}",
             first=x,
             second=y);
    println!();

    println!("Base 10: {}", x);
    println!("Base 2: {:b}", x);
    println!("Base 8: {:o}", x);
    println!("Base 16: {:x}", x);
    println!();

    println!("right: {:>10}", x);
    println!("right with pad: {:0>10}", x);
    println!("right with variable pad: {:0>pad$}", x, pad=y);
}
