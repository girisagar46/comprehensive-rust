pub fn arrays_and_tuple() {
    println!("\n\nArrays and tuples");
    // Arrays	[T; N]	-> [20, 30, 40], [0; 3]
    let mut a_array: [i8; 5] = [42; 5]; //[42, 42, 42, 0, 42]
    a_array[3] = 0;
    println!("a_array: {:?}", a_array);

    // Tuples	(), (T,), (T1, T2), …	-> (), ('x',), ('x', 1.2), …
    let tuple: (i8, bool) = (7, true);
    println!("tuple: {:?}", tuple);
    println!("1st index: {}", tuple.0);
    println!("2nd index: {}", tuple.1);
}
