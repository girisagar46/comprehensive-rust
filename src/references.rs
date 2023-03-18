pub fn references() {
    println!("\n\nreferences");
    let mut x: i32 = 10;
    println!("x (before): {x}");
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x (after): {x}");
    /*
    - We must dereference ref_x when assigning to it, similar to C and C++ pointers.
    - Rust will auto-dereference in some cases, in particular when invoking methods (try ref_x
    .count_ones()).
    - References that are declared as mut can be bound to different values over their lifetime.
     */
}
