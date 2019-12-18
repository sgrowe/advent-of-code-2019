use std::num::ParseIntError;
use std::str::FromStr;

enum Mode {
    Position,
    Immediate,
}

enum Instruction {
    Add(Mode, Mode),
    Multiply(Mode, Mode),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub code: Vec<i64>,
}

pub trait RunProgram {
    fn run(&mut self);
}

impl RunProgram for Program {
    fn run(&mut self) {
        let mut i = 0;

        while self.code[i] != 99 {
            let op = get_opp(self.code[i]);
            let x = deref(&self.code, i + 1);
            let y = deref(&self.code, i + 2);
            let target_pos = self.code[i + 3] as usize;

            let res = op(x, y);

            self.code[target_pos] = res;

            i += 4;
        }
    }
}

impl FromStr for Program {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Program, Self::Err> {
        input
            .split(',')
            .map(|s| s.parse::<i64>())
            .collect::<Result<Vec<i64>, _>>()
            .map(|code| Program { code })
    }
}

fn deref(program: &Vec<i64>, i: usize) -> i64 {
    let pointer = program[i] as usize;

    program[pointer]
}

fn parse_mode(int: i64) -> Mode {
    if int == 0 {
        Mode::Position
    } else {
        Mode::Immediate
    }
}

fn parse_instruction(op_code: i64) -> Instruction {
    let code = op_code % 100;
    let mode_1 = parse_mode((op_code / 100) % 10);
    let mode_2 = parse_mode((op_code / 1000) % 10);

    match code {
        1 => Instruction::Add(mode_1, mode_2),
        2 => Instruction::Multiply(mode_1, mode_2),
        _ => panic!("Unexpected opcode: {}", op_code),
    }
}

fn get_opp(op_code: i64) -> impl Fn(i64, i64) -> i64 {
    match op_code {
        1 => add,
        2 => multiply,
        _ => panic!("Unexpected opcode: {}", op_code),
    }
}

fn add(x: i64, y: i64) -> i64 {
    x + y
}

fn multiply(x: i64, y: i64) -> i64 {
    x * y
}

#[cfg(test)]
mod day_two_tests {
    use super::*;

    fn assert_program_output_is(input: &str, expected_code: Vec<i64>) {
        let mut program = input.parse::<Program>().unwrap();

        program.run();

        let expected = Program {
            code: expected_code,
        };

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
