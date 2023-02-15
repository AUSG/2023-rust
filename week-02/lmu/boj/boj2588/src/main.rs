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
    let _a = scan!(input, i32).0;
    let a = _a.unwrap();

    let mut input2 = String::new();
    stdin.read_line(&mut input2).expect("Fail");
    let _b = scan!(input2, i32).0;
    let b = _b.unwrap();

    let mut t = b;
    while t > 0 {
        println!("{}", (t % 10) * a);
        t /= 10;
    }
    println!("{}", a * b);
}
