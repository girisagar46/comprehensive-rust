pub fn arrays_and_for_loops() {
    println!("\n\narrays_and_for_loops");

    print!("Iterating over array:");
    let array = [10, 20, 30];
    for n in array {
        print!(" {n}");
    }
    println!();

    print!("Iterating over range:");
    for i in 0..3 {
        print!(" {}", array[i]);
    }
    println!();

    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];
    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed_matrix: [[i32; 3]; 3] = [[0; 3]; 3];
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            transposed_matrix[i][j] = matrix[j][i];
        }
    }
    return transposed_matrix;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            print!("{} ", matrix[i][j]);
        }
        println!();
    }
}

#[test]
fn test_transpose() {
    let matrix = [
        [1, 2, 3], //
        [10, 20, 30],
        [100, 200, 300],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [1, 10, 100], //
            [2, 20, 200],
            [3, 30, 300],
        ]
    );
}
