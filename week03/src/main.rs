pub fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let mut i = 0;

    for char in cc_number.chars().rev() {
        if char.is_whitespace() {
            continue;
        }

        let number = char.to_digit(10);
        match number {
            Some(a) if i % 2 == 0 => sum += a,
            Some(a) => {
                let mut tmp = a * 2;
                if tmp > 9 {
                    tmp = tmp - 9;
                }
                sum += tmp;
            },
            None => return false
        }
        i += 1;
    }

    if i < 2 {
        return false;
    }

    if sum % 10 == 0 {
        return true;
    }

    false
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
fn main() {
    luhn(" 0 0 ");
}