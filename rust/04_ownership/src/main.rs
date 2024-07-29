fn get_length(s: &String) -> usize {
    s.len()
}

fn put_world(s: &mut String) {
    s.push_str(" world");
}

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn main() {
    // Ownership
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1: {s1}"); // Error, because s1 is moved to s2
    println!("s2: {s2}"); // No error

    // Borrowing
    let immut_str = String::from("Hello world");
    println!("length of {immut_str} is {}", get_length(&immut_str));

    let mut mut_str = String::from("Hello");
    put_world(&mut mut_str);
    println!("value of mut_str is {mut_str}");

    // Slice type
    let first_word = get_first_word(&mut_str);
    println!("First word of mut_str is {first_word}");
    // put_world(&mut mut_str); // Error, because the word is already borrowed to immutable function
}
