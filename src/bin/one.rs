use std::fs::File;
use std::io::prelude::*;

fn get_fuel_for_module(mass: i64) -> i64 {
  (mass / 3) - 2
}

fn read_file(file_name: &str) -> String {
  let mut file = File::open(file_name).unwrap();
  let mut contents = String::new();

  file.read_to_string(&mut contents).unwrap();

  contents
}

fn main() {
  let contents = read_file("src/one.txt");

  let fuel_requirements = contents.trim().split_ascii_whitespace().map(|string| {
    let mass = string.parse::<i64>().unwrap();

    get_fuel_for_module(mass)
  });

  let total_fuel = fuel_requirements.fold(0, |x, y| x + y);

  println!("Total fuel needed: {}", total_fuel)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_with_mass_of_12() {
    assert_eq!(get_fuel_for_module(12), 2)
  }

  #[test]
  fn test_with_mass_of_14() {
    assert_eq!(get_fuel_for_module(14), 2)
  }

  #[test]
  fn test_with_mass_of_1969() {
    assert_eq!(get_fuel_for_module(1969), 654)
  }

  #[test]
  fn test_with_mass_of_100756() {
    assert_eq!(get_fuel_for_module(100756), 33583)
  }
}
