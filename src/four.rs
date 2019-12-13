use super::rolling_pairs::*;
use std::ops::Range;

pub fn main() {
    let range = 109165..576724;

    println!("Part one:");
    println!(
        "Number of possible passwords: {}",
        count_possible_pass_codes_in_range(range.clone(), &is_possible_pass_code)
    );

    println!();
    println!("Part two:");
    println!(
        "Number of possible passwords: {}",
        count_possible_pass_codes_in_range(range, &is_possible_pass_code_v2)
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

    is_six_digit(&digits)
        && contains_an_adjacent_pair_of_digits(&digits)
        && digits_only_increase(&digits)
}

fn parse_digits(code: &u32) -> Vec<u32> {
    code.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
}

fn is_six_digit(digits: &Vec<u32>) -> bool {
    digits.len() == 6
}

fn has_adjacent_digits(digits: &Vec<u32>) -> bool {
    digits
        .iter()
        .rolling_pairs()
        .any(|(prev, current)| prev == current)
}

fn contains_an_adjacent_pair_of_digits(digits: &Vec<u32>) -> bool {
    let mut digits_iter = digits.iter();

    let mut prev_digit = match digits_iter.next() {
        Some(digit) => digit,
        None => return false,
    };

    let mut adjacent_count = 1;

    for digit in digits_iter {
        if digit == prev_digit {
            adjacent_count += 1;
        } else {
            if adjacent_count == 2 {
                return true;
            }

            adjacent_count = 1;
        }

        prev_digit = digit;
    }

    adjacent_count == 2
}

fn digits_only_increase(digits: &Vec<u32>) -> bool {
    digits
        .iter()
        .rolling_pairs()
        .all(|(prev_digit, digit)| digit >= prev_digit)
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

#[cfg(test)]
mod part_two_tests {
    use super::*;

    #[test]
    fn example_case_1() {
        assert!(is_possible_pass_code_v2(&112233));
    }

    #[test]
    fn example_case_2() {
        assert!(!is_possible_pass_code_v2(&123444));
    }

    #[test]
    fn example_case_3() {
        assert!(is_possible_pass_code_v2(&111122));
    }
}
