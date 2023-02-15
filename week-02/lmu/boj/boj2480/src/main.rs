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
    let (_a, _b, _c) = scan!(input, i32, i32, i32);
    let data = (_a.unwrap(), _b.unwrap(), _c.unwrap());

    /*
    if a == b && b == c {
        println!("{}", 10000 + a * 1000);
    } else if a == b && b != c {
        println!("{}", 1000 + a * 100);
    } else if a == c && a != b {
        println!("{}", 1000 + a * 100);
    } else if b == c && a != b {
        println!("{}", 1000 + b * 100);
    } else {
        if a >= b && a >= c {
            println!("{}", 100 * a);
        } else if b >= c && b >= c {
            println!("{}", 100 * b);
        } else {
            println!("{}", 100 * c);
        }
    }
    */

    let price = match data {
        (a, b, c) if a == b && b == c => a * 1000 + 10000,
        (a, b, c) if (a == b && b != c) || (a == c && a != b) => a * 100 + 1000,
        (a, b, c) if (b == c && a != b) => b * 100 + 1000,
        (a, b, c) if a >= b && a >= c => 100 * a,
        (a, b, c) if b >= a && b >= c => 100 * b,
        (_, _, c) => 100 * c,
    };
    println!("{}", price);
}
