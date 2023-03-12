//use io::Write;
use std::io;
macro_rules! scan {
    ( $string:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split(char::is_whitespace);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

fn main() {
    let stdin = io::stdin();
    let mut data = Vec::new();
    for _ in 0..9 {
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Fail"); // skip, no need n
        let a = scan!(input, i32).0.unwrap();
        data.push(a);
    }
    let (idx, val) = data.into_iter().enumerate().max_by_key(|t| t.1).unwrap();
    println!("{}\n{}", val, idx + 1);
}
