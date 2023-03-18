pub fn functions() {
    println!("\n\nfunctions");
    fizzbuzz_to(20);
}

fn fizzbuzz_to(n: u32) {  // `-> ()` is normally omitted
    for i in 1..=n {
        fizzbuzz(i);
    }
}

fn fizzbuzz(n: u32) -> () {  // No return value means returning the unit type `()`
    match (is_divisible_by(n, 3), is_divisible_by(n, 5)) {
        (true, true) => print!("fizzbuzz "),
        (true, false) => print!("fizz "),
        (false, true) => print!("buzz "),
        (false, false) => print!("{n} "),
    }
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;  // Corner case, early return
    }
    lhs % rhs == 0     // The last expression in a block is the return value
}
