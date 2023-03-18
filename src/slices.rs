pub fn slices() {
    println!("\n\nslices");
    let a: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];
    println!("a sliced: {s:?}");

    // a[3] = 10; // can't modify already borrowed stuffs
    // println!("a sliced: {s:?}");
}
