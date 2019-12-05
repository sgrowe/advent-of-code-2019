use std::fs::File;
use std::io::prelude::*;

fn get_fuel_needed_for_mass(mass: i64) -> i64 {
  (mass / 3) - 2
}

fn read_file(file_name: &str) -> String {
  let mut file = File::open(file_name).unwrap();
  let mut contents = String::new();

  file.read_to_string(&mut contents).unwrap();

  contents
}

fn parse_int(string: &str) -> i64 {
  string.parse::<i64>().unwrap()
}

fn parse_module_masses<'a>(input: &'a str) -> impl Iterator<Item = i64> + 'a {
  input.trim().split_ascii_whitespace().map(&parse_int)
}

fn part_one(input: &str) -> i64 {
  parse_module_masses(input)
    .map(|mass| get_fuel_needed_for_mass(mass))
    .sum()
}

//
// --- PART TWO ---
//

fn get_actual_fuel_needed_for_mass(mass: i64) -> i64 {
  let mut fuel = get_fuel_needed_for_mass(mass);

  let mut additional_fuel = get_fuel_needed_for_mass(fuel);

  while additional_fuel > 0 {
    fuel += additional_fuel;
    additional_fuel = get_fuel_needed_for_mass(additional_fuel);
  }

  fuel
}

fn part_two(input: &str) -> i64 {
  parse_module_masses(input)
    .map(|mass| get_actual_fuel_needed_for_mass(mass))
    .sum()
}

fn main() {
  let contents = read_file("src/one.txt");

  let mut total_fuel = part_one(&contents);

  println!("Part one:");
  println!("Total fuel needed: {}", total_fuel);

  total_fuel = part_two(&contents);

  println!();
  println!("Part two:");
  println!("Total fuel needed: {}", total_fuel);
}

#[cfg(test)]
mod part_one_tests {
  use super::*;

  #[test]
  fn test_with_mass_of_12() {
    assert_eq!(get_fuel_needed_for_mass(12), 2)
  }

  #[test]
  fn test_with_mass_of_14() {
    assert_eq!(get_fuel_needed_for_mass(14), 2)
  }

  #[test]
  fn test_with_mass_of_1969() {
    assert_eq!(get_fuel_needed_for_mass(1969), 654)
  }

  #[test]
  fn test_with_mass_of_100756() {
    assert_eq!(get_fuel_needed_for_mass(100756), 33583)
  }
}

#[cfg(test)]
mod part_two_tests {
  use super::*;

  #[test]
  fn test_with_mass_of_14() {
    assert_eq!(get_actual_fuel_needed_for_mass(14), 2)
  }

  #[test]
  fn test_with_mass_of_1969() {
    assert_eq!(get_actual_fuel_needed_for_mass(1969), 966)
  }

  #[test]
  fn test_with_mass_of_100756() {
    assert_eq!(get_actual_fuel_needed_for_mass(100756), 50346)
  }
}
