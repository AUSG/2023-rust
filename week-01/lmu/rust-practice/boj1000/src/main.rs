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
    let (_a, _b, _c) = scan!(input, i32, i32, i32);
    let (a, b, c) = (_a.unwrap(), _b.unwrap(), _c.unwrap());
    println!(
        "{}\n{}\n{}\n{}",
        (a + b) % c,
        (a + b) % c,
        (a * b) % c,
        (a * b) % c
    );
}
