
pub fn learn_ownership() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s1 is no longer valid. s2: {}", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);

    let s1 = String::from("hello");
    let s2 = takes_ownership(s1);
    println!("s1 is no longer valid. s2: {}", s2);

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s1 = String::from("hello");
    change(&mut s1);
    println!("After change: {}", s1);

    let s = String::from("hello, world");
    let hello = &s[0..5];
    let world = &s[7..12];
    let hello_world = &s[..];
    println!("hello: {}, world: {}, hello_world: {}", hello, world, hello_world);
}

fn takes_ownership(some_string: String) -> String {
    println!("Taking ownership of: {}", some_string);
    some_string
}

fn calculate_length(s: &String) -> usize {
    println!("This is a reference to: {}", s);
    s.len()
}

fn change(some_string: &mut String) {
    println!("This is a mutable reference to: {}", some_string);
    some_string.push_str(", world");
}