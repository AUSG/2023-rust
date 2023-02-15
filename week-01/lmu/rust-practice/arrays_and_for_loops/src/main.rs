fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut retval = matrix;
    for i in 0..3 {
        for j in 0..3 {
            retval[i][j] = matrix[j][i];
        }
    }
    return retval;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for i in 0..3 {
        for j in 0..3 {
            print!("{} ", matrix[i][j]);
        }
        println!();
    }
}

fn main() {
    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];
    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("tranposed:");
    pretty_print(&transposed);
}
