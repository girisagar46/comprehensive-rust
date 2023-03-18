pub fn string_and_str() {
    println!("\n\nstring_and_str");
    // &str is an immutable reference to a string slice.
    // String is a mutable string buffer.
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2 after push: {s2}");

    let s3: &str = &s2[6..];
    println!("s3: {s3}");
}
