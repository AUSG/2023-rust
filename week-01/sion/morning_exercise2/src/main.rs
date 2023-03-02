// TODO: 구현이 완료되면 아래 줄은 삭제합니다.
#![allow(unused_variables, dead_code)]

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut matrix = matrix;
    for n in 0..3 {
        for m in 0..3 {
            if n < m {
                let temp = matrix[n][m];
                matrix[n][m] = matrix[m][n];
                matrix[m][n] = temp;
            }
        }
    }
    matrix
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        for col in row {
            print!("{:>4}", col);
        }
        println!();
    }
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
