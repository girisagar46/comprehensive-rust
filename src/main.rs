#![allow(unused_variables, dead_code)]

mod arrays_and_for_loops;
mod implicit_conversions;
mod function_overloading;
mod methods;
mod functions;
mod string_and_str;
mod slices;
mod references;
mod arrays_and_tuples;
mod simple_code;

fn main() {
    simple_code::simple_code();

    arrays_and_tuples::arrays_and_tuple();

    references::references();

    // dangling_references(); // causes error

    slices::slices();

    string_and_str::string_and_str();

    functions::functions();

    methods::methods();

    function_overloading::function_overloading();

    implicit_conversions::implicit_conversions();

    arrays_and_for_loops::arrays_and_for_loops();
}

/*fn dangling_references() {
    println!("\n\n dangling_references");
    // Rust will statically forbid dangling references:
    let ref_x: &i32;
    {
        let x: i32= 10;  // x only lives inside this block
        ref_x = &x;  // ref_x borrows reference of x inside this short-lived block
    }
    println!("ref_x: {ref_x}")  // this causes error because it's a dangling reference and Rust
    // forbids this
    /*
    - A reference is said to “borrow” the value it refers to.
    - Rust is tracking the lifetimes of all references to ensure they live long enough.
     */
}*/








