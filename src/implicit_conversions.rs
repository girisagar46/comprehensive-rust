pub fn implicit_conversions() {
    println!("\n\nimplicit_conversions");
    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x.into(), y));  // x.into() converts x into i16 type
}

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}
