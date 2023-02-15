use std::io;
macro_rules! scan {
    ( $string:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split(char::is_whitespace);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

fn main() {
    let mut input = String::new();
    let mut input2 = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).expect("Fail");
    stdin.read_line(&mut input2).expect("Fail");
    let a = scan!(input, i32).0.unwrap();
    let b = scan!(input2, i32).0.unwrap();

    if a > 0 && b > 0 {
        println!("1");
    } else if a < 0 && b > 0 {
        println!("2");
    } else if a < 0 && b < 0 {
        println!("3");
    } else {
        println!("4");
    }
}
