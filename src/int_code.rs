use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Mode {
    Position,
    Immediate,
}

impl Mode {
    fn from_i64(int: i64) -> Mode {
        if int == 0 {
            Mode::Position
        } else {
            Mode::Immediate
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
    Add([Mode; 2]),
    Multiply([Mode; 2]),
    ReadInput(Mode),
    WriteOutput(Mode),
}

impl Instruction {
    fn from_i64(op_code: i64) -> Instruction {
        let code = op_code % 100;
        let mode_1 = Mode::from_i64((op_code / 100) % 10);
        let mode_2 = Mode::from_i64((op_code / 1000) % 10);
        let mode_3 = Mode::from_i64((op_code / 10000) % 10);

        match code {
            1 => Instruction::Add([mode_1, mode_2]),
            2 => Instruction::Multiply([mode_1, mode_2]),
            3 => Instruction::ReadInput(mode_1),
            4 => Instruction::WriteOutput(mode_1),
            _ => panic!("Unexpected opcode: {}", op_code),
        }
    }

    fn width(&self) -> usize {
        match self {
            Instruction::Add(_) => 4,
            Instruction::Multiply(_) => 4,
            Instruction::ReadInput(_) => 2,
            Instruction::WriteOutput(_) => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub inputs: VecDeque<i64>,
    pub code: Vec<i64>,
}

impl Program {
    pub fn run(&mut self) -> Vec<i64> {
        let mut outputs = Vec::new();

        let mut i = 0;

        while self.code[i] != 99 {
            let instruction = Instruction::from_i64(self.code[i]);

            match instruction {
                Instruction::Add([mode_1, mode_2]) => {
                    let x = self.read(i + 1, mode_1);
                    let y = self.read(i + 2, mode_2);

                    self.write(i + 3, Mode::Position, x + y);
                }
                Instruction::Multiply([mode_1, mode_2]) => {
                    let x = self.read(i + 1, mode_1);
                    let y = self.read(i + 2, mode_2);

                    self.write(i + 3, Mode::Position, x * y);
                }
                Instruction::ReadInput(mode) => {
                    let input = self.inputs.pop_front().expect("No input given");

                    self.write(i + 1, mode, input);
                }
                Instruction::WriteOutput(mode) => {
                    let output = self.read(i + 1, mode);

                    outputs.push(output);
                }
            }

            i += instruction.width();
        }

        outputs
    }

    fn read(&self, addr: usize, mode: Mode) -> i64 {
        let val = self.code[addr];

        match mode {
            Mode::Position => self.code[val as usize],
            Mode::Immediate => val,
        }
    }

    fn write(&mut self, addr: usize, mode: Mode, value: i64) {
        let write_addr = match mode {
            Mode::Position => self.code[addr] as usize,
            Mode::Immediate => addr,
        };

        self.code[write_addr] = value;
    }
}

impl FromStr for Program {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Program, Self::Err> {
        input
            .split(',')
            .map(|s| s.parse::<i64>())
            .collect::<Result<Vec<i64>, _>>()
            .map(|code| Program {
                inputs: VecDeque::new(),
                code,
            })
    }
}

#[cfg(test)]
mod day_two_tests {
    use super::*;

    fn assert_program_output_is(input: &str, expected_code: Vec<i64>) {
        let mut program = input.parse::<Program>().unwrap();

        program.run();

        let expected = Program {
            inputs: VecDeque::new(),
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

#[cfg(test)]
mod day_five_part_one_tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn allows_different_modes_for_operations() {
        let mut program = "1002,4,3,4,33".parse::<Program>().unwrap();

        program.run();

        assert_eq!(program.code, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn supports_negative_values_in_program_code() {
        let mut program = "1101,100,-1,4,0".parse::<Program>().unwrap();

        program.run();

        assert_eq!(program.code, vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn supports_op_codes_3_and_4() {
        let mut program = "3,0,4,0,99".parse::<Program>().unwrap();

        let mut inputs = VecDeque::new();
        inputs.push_back(5);

        program.inputs = inputs;

        let output = program.run();

        assert_eq!(output, vec!(5));
    }

    #[test]
    fn runs_diagnostic_program_correctly() {
        let mut program = read_to_string("src/five.txt")
            .unwrap()
            .trim()
            .parse::<Program>()
            .unwrap();

        let mut inputs = VecDeque::new();
        inputs.push_back(1);

        program.inputs = inputs;

        let output = program.run();

        let test_codes = &output[0..output.len() - 1];

        assert!(test_codes.iter().all(|&x| x == 0));
    }

}
