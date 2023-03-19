// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let converted = cc_number
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10));

    if converted.clone().count() < 2 {
        println!("count < 2");
        return false;
    }

    match converted
        .rev()
        .enumerate()
        .map(|(i, x)| match x {
            Some(v) if i % 2 == 1 => Some(2 * v),
            x => x,
        })
        .map(|x| match x {
            Some(v) if v >= 10 => Some(v / 10 + v % 10),
            x => x,
        })
        .fold(Some(0), |acc, x| match (acc, x) {
            (Some(acc_v), Some(v)) => Some(acc_v + v),
            _ => None,
        }) {
        Some(v) if v % 10 == 0 => true,
        _ => false,
    }
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}
