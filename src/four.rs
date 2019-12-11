use std::ops::Range;

pub fn main() {
    println!("Part one:");
    println!(
        "Number of possible passwords: {}",
        count_possible_pass_codes_in_range(109165..576724, &is_possible_pass_code)
    );

    println!();
    println!("Part two:");
    println!(
        "Number of possible passwords: {}",
        count_possible_pass_codes_in_range(109165..576724, &is_possible_pass_code_v2)
    );
}

fn count_possible_pass_codes_in_range(
    range: Range<u32>,
    predicate: impl Fn(&u32) -> bool,
) -> usize {
    range.filter(&predicate).count()
}

fn is_possible_pass_code(code: &u32) -> bool {
    let digits = parse_digits(code);

    is_six_digit(&digits) && has_adjacent_digits(&digits) && digits_only_increase(&digits)
}

fn is_possible_pass_code_v2(code: &u32) -> bool {
    let digits = parse_digits(code);

    // TODO:
    is_six_digit(&digits) && has_adjacent_digits(&digits) && digits_only_increase(&digits)
}

fn parse_digits(code: &u32) -> Vec<u32> {
    let string = code.to_string();

    string
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
}

fn is_six_digit(digits: &Vec<u32>) -> bool {
    digits.len() == 6
}

fn has_adjacent_digits(digits: &Vec<u32>) -> bool {
    let mut digits_iter = digits.iter();

    let mut prev_digit = match digits_iter.next() {
        Some(digit) => digit,
        None => return false,
    };

    for digit in digits_iter {
        if (digit) == prev_digit {
            return true;
        };

        prev_digit = digit;
    }

    false
}

fn digits_only_increase(digits: &Vec<u32>) -> bool {
    let mut digits_iter = digits.iter();

    let mut prev_digit = digits_iter.next().unwrap();

    for digit in digits {
        if digit < prev_digit {
            return false;
        }

        prev_digit = digit;
    }

    true
}

#[cfg(test)]
mod part_one_tests {
    use super::*;

    #[test]
    fn example_case_1() {
        assert!(is_possible_pass_code(&111111));
    }

    #[test]
    fn example_case_2() {
        assert!(!is_possible_pass_code(&223450));
    }

    #[test]
    fn example_case_3() {
        assert!(!is_possible_pass_code(&123789));
    }
}
