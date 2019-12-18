use super::rolling_pairs::*;
use itertools::Itertools;
use std::ops::Range;

const CODE_LENGTH: usize = 6;

type Digits = [u32; CODE_LENGTH];

pub fn main() {
    let range = 109165..576724;

    println!("Part one:");
    println!(
        "Number of possible passwords: {}",
        count_possible_pass_codes_in_range(range.clone(), is_possible_pass_code)
    );

    println!();
    println!("Part two:");
    println!(
        "Number of possible passwords: {}",
        count_possible_pass_codes_in_range(range, is_possible_pass_code_v2)
    );
}

fn count_possible_pass_codes_in_range(range: Range<u32>, predicate: impl Fn(u32) -> bool) -> usize {
    range.filter(|x| predicate(*x)).count()
}

fn is_possible_pass_code(code: u32) -> bool {
    let digits = match parse_digits(code) {
        Some(d) => d,
        None => return false,
    };

    digits_only_increase(&digits) && has_adjacent_digits(&digits)
}

fn is_possible_pass_code_v2(code: u32) -> bool {
    let digits = match parse_digits(code) {
        Some(d) => d,
        None => return false,
    };

    digits_only_increase(&digits) && contains_an_adjacent_pair_of_digits(&digits)
}

fn parse_digits(mut code: u32) -> Option<Digits> {
    let mut digits = [0; CODE_LENGTH];

    for i in (0..CODE_LENGTH).rev() {
        digits[i] = code % 10;

        code /= 10;
    }

    if code > 0 {
        return None;
    }

    Some(digits)
}

fn has_adjacent_digits(digits: &Digits) -> bool {
    digits
        .iter()
        .rolling_pairs()
        .any(|(prev, current)| prev == current)
}

fn contains_an_adjacent_pair_of_digits(digits: &Digits) -> bool {
    digits
        .iter()
        .group_by(|digit| *digit)
        .into_iter()
        .any(|(_digit, group)| group.count() == 2)
}

fn digits_only_increase(digits: &Digits) -> bool {
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
        assert!(is_possible_pass_code(111111));
    }

    #[test]
    fn example_case_2() {
        assert!(!is_possible_pass_code(223450));
    }

    #[test]
    fn example_case_3() {
        assert!(!is_possible_pass_code(123789));
    }
}

#[cfg(test)]
mod part_two_tests {
    use super::*;

    #[test]
    fn example_case_1() {
        assert!(is_possible_pass_code_v2(112233));
    }

    #[test]
    fn example_case_2() {
        assert!(!is_possible_pass_code_v2(123444));
    }

    #[test]
    fn example_case_3() {
        assert!(is_possible_pass_code_v2(111122));
    }
}
