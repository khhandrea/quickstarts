struct UnPrintable(i32);

#[derive(Debug)]
struct Printable(i32);

#[derive(Debug)]
struct Deep(Printable);

fn main() {
    // Error
    // println!("Hi {}", UnPrintable(10));
    // println!("Hi {:?}", UnPrintable(10));
    // println!("Hi {}", Printable(10));
    
    println!("Hi {:?}", Printable(10));
    println!("Deep {:?}", Deep(Printable(10)));
    println!("Pretty {:#?}", Deep(Printable(10)));
}
