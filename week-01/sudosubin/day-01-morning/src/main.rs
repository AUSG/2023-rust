fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result: [[i32; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];

    let mut row_index: usize = 0;
    for row in matrix {
        let mut col_index: usize = 0;
        for number in row {
            result[col_index][row_index] = number;
            col_index += 1;
        }
        row_index += 1;
    }

    return result;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    let mut row_index: i32 = 0;

    for row in matrix {
        let mut col_index: i32 = 0;

        match row_index {
            0 => print!("⎡"),
            1 => print!("⎢"),
            2 => print!("⎣"),
            _ => print!(""),
        }

        for number in row {
            if col_index != 0 {
                print!(" ");
            }
            print!("{number}");
            col_index += 1;
        }

        match row_index {
            0 => print!("⎤"),
            1 => print!("⎥"),
            2 => print!("⎦"),
            _ => print!(""),
        }

        row_index += 1;
        println!();
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
