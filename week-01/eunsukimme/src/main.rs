fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut new_matrix: [[i32; 3]; 3] = [[0; 3]; 3];
    for x in 0..3 {
        for y in 0..3 {
            new_matrix[x][y] = matrix[y][x];
        }
    }
    return new_matrix;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for i in 0..3 {
        println!("{:?}", matrix[i]);
    }
    println!();
}

fn main() {
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