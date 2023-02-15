use std::io;
macro_rules! scan {
    ( $string:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split(char::is_whitespace);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fail");
    let a = scan!(input, i32, i32, i32).0.unwrap();
    if a % 400 == 0 {
        println!("1");
    } else if a % 100 == 0 {
        println!("0");
    } else if a % 4 == 0 {
        println!("1");
    } else {
        println!("0");
    }
}
