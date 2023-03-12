//use io::Write;
use std::io;
macro_rules! scan {
    ( $string:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split(char::is_whitespace);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).expect("Fail"); // skip, no need n
    let mut data_input = String::new();
    stdin.read_line(&mut data_input).expect("Fail");
    let mut wanted = String::new();
    stdin.read_line(&mut wanted).expect("Fail");
    let x = scan!(wanted, i32).0.unwrap();
    println!(
        "{}",
        data_input
            .split(char::is_whitespace)
            .into_iter()
            .map(|s| s.parse::<i32>())
            .filter(|t| match t.to_owned().ok() {
                Some(v) => x == v,
                _ => false,
            })
            .count()
    );
}
