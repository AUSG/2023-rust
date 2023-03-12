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
    stdin.read_line(&mut input).expect("Fail");

    let (_n, _x) = scan!(input, i32, i32);
    let (_, x) = (_n.unwrap(), _x.unwrap());

    let mut data_input = String::new();
    stdin.read_line(&mut data_input).expect("Fail");
    data_input
        .split(char::is_whitespace)
        .into_iter()
        .map(|s| s.parse::<i32>())
        .map(|t| t.to_owned().ok())
        .filter(|t| match t {
            Some(v) => x > *v,
            _ => false,
        })
        .for_each(|t| match t {
            Some(v) => print!("{} ", v),
            _ => (),
        })
}
