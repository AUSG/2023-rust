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
    let (_a, _b) = scan!(input, i32, i32);
    let (a, b) = (_a.unwrap(), _b.unwrap());
    let c = scan!(input2, i32).0.unwrap();

    let t = (a * 60 + b + c) % (24 * 60);
    println!("{} {}", t / 60, t % 60);
}
