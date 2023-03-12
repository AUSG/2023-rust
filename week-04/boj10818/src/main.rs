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
    let max_value = data_input
        .split(char::is_whitespace)
        .into_iter()
        .map(|s| s.parse::<i32>())
        .map(|t| t.to_owned().ok())
        .map(|t| match t {
            Some(v) => v,
            _ => -1_000_000,
        })
        .max()
        .unwrap();
    let min_value = data_input
        .split(char::is_whitespace)
        .into_iter()
        .map(|s| s.parse::<i32>())
        .map(|t| t.to_owned().ok())
        .map(|t| match t {
            Some(v) => v,
            _ => 1_000_000,
        })
        .min()
        .unwrap();
    println!("{min_value} {max_value}");
}
