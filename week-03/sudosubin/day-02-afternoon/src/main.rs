// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    // Condition 1: Ignore all spaces. Reject number with less than two digits.
    let number = clean_cc_number(cc_number);
    if number.len() < 2 {
        return false;
    }

    // Condition 2: Moving from right to left, double every second digit: for the number 1234, we double 3 and 1.
    // Condition 3: After doubling a digit, sum the digits. So doubling 7 becomes 14 which becomes 5.
    // Condition 4: Sum all the undoubled and doubled digits.
    let result: u32 = number
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| calculate_cc_digit(i, c).unwrap())
        .sum();

    // Condition 5: The credit card number is valid if the sum ends with 0.
    result % 10 == 0
}

fn clean_cc_number(cc_number: &str) -> String {
    return cc_number
        .replace(" ", "")
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if let Ok(unwrapped) = String::from(c).parse::<u8>() {
                return Some(unwrapped.to_string());
            }
            return None;
        })
        .filter_map(|c| c)
        .collect();
}

fn calculate_cc_digit(cc_index: usize, cc_digit: char) -> Option<u32> {
    println!("cc_index: {}, cc_digit: {}", cc_index, cc_digit);
    if cc_index % 2 == 1 {
        let digit = match cc_digit.to_digit(10) {
            Some(digit) => digit * 2,
            _ => return None,
        };

        if digit >= 10 {
            return Some(digit / 10 + digit % 10);
        } else {
            return Some(digit);
        }
    }

    return cc_digit.to_digit(10);
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
