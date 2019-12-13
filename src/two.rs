use super::file_utils;
use super::int_code::*;

pub fn main() {
  let contents = file_utils::read("src/two.txt");
  let input = contents.trim();

  println!("Part one:");
  println!("Final value at position 0: {}", part_one(&input));

  println!();
  println!("Part two:");
  let (noun, verb) = part_two(&input);
  println!("Noun: {} - Verb: {}", noun, verb);
  println!("100 * noun + verb: {}", (100 * noun) + verb);
}

fn part_one(input: &str) -> i64 {
  let mut program = parse_program(&input);

  run_program_with_inputs(12, 2, &mut program);

  program[0]
}

fn part_two(input: &str) -> (i64, i64) {
  let target = 19690720;

  let initial_program = parse_program(&input);

  for i in 0..99 {
    for j in 0..99 {
      let mut program = initial_program.clone();

      run_program_with_inputs(i, j, &mut program);

      if program[0] == target {
        return (i, j);
      }
    }
  }

  panic!("Could not find a pair of inputs resulting in {}", target);
}

fn run_program_with_inputs(a: i64, b: i64, program: &mut Vec<i64>) {
  program[1] = a;
  program[2] = b;

  run_program(program)
}
