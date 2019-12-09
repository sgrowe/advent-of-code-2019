mod file;

fn main() {
  let input = file::read("src/two.txt");

  let mut program = parse_program(&input.trim());

  // Restore program to the state it had just before the last computer caught fire
  program[1] = 12;
  program[2] = 2;

  run_program(&mut program);

  println!("Part one:");
  println!("Final value at position 0: {}", program[0]);
}

fn parse_program(input: &str) -> Vec<i64> {
  input
    .split(',')
    .map(|s| s.parse::<i64>().unwrap())
    .collect::<Vec<i64>>()
}

fn run_program(program: &mut Vec<i64>) {
  let mut i = 0;

  while program[i] != 99 {
    let op = get_opp(program[i]);
    let x = deref(program, i + 1);
    let y = deref(program, i + 2);
    let target_pos = program[i + 3] as usize;

    let res = op(x, y);

    program[target_pos] = res;

    i += 4;
  }
}

fn deref(program: &Vec<i64>, i: usize) -> i64 {
  let pointer = program[i] as usize;

  program[pointer]
}

fn get_opp(op_code: i64) -> Box<Fn(i64, i64) -> i64> {
  match op_code {
    1 => Box::new(|x, y| x + y),
    2 => Box::new(|x, y| x * y),
    _ => panic!("Unexpected opcode: {}", op_code),
  }
}

#[cfg(test)]
mod part_one_tests {
  use super::*;

  fn assert_program_output_is(input: &str, expected: Vec<i64>) {
    let mut program = parse_program(input);
    run_program(&mut program);
    assert_eq!(program, expected)
  }

  #[test]
  fn example_input() {
    assert_program_output_is(
      "1,9,10,3,2,3,11,0,99,30,40,50",
      vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
    )
  }

  #[test]
  fn example_test_case_1() {
    assert_program_output_is("1,0,0,0,99", vec![2, 0, 0, 0, 99])
  }

  #[test]
  fn example_test_case_2() {
    assert_program_output_is("2,3,0,3,99", vec![2, 3, 0, 6, 99])
  }

  #[test]
  fn example_test_case_3() {
    assert_program_output_is("2,4,4,5,99,0", vec![2, 4, 4, 5, 99, 9801])
  }

  #[test]
  fn example_test_case_4() {
    assert_program_output_is("1,1,1,4,99,5,6,0,99", vec![30, 1, 1, 4, 2, 5, 6, 0, 99])
  }
}
