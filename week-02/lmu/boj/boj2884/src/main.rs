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
    let (_a, _b) = scan!(input, i32, i32);
    let (a, b) = (_a.unwrap(), _b.unwrap());
    let t = a * 60 + b;
    let t = (t + 24 * 60 - 45) % (24 * 60);
    println!("{} {}", t / 60, t % 60);
}
