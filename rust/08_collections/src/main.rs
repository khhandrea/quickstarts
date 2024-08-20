fn main() {
    let mut v: Vec<i32> = Vec::new();
    v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    v.push(6);

    // Vector indexing
    let item1: &i32 = &v[2];
    println!("v[2] is {} by indexing", item1);

    let item2: Option<&i32> = v.get(2);
    match item2 {
        Some(third) => println!("v[2] is {} by get function", third),
        None => println!("There is no third element"),
    }

    // Vector iteration
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}")
    }

}
