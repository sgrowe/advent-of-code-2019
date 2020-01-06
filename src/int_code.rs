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
    Add([Mode; 3]),
    Multiply([Mode; 3]),
    ReadInput(Mode),
    WriteOutput(Mode),
    JumpIfTrue([Mode; 2]),
    JumpIfFalse([Mode; 2]),
    LessThan([Mode; 3]),
    Equals([Mode; 3]),
}

impl Instruction {
    fn from_i64(op_code: i64) -> Instruction {
        let code = op_code % 100;
        let mode_1 = Mode::from_i64((op_code / 100) % 10);
        let mode_2 = Mode::from_i64((op_code / 1000) % 10);
        let mode_3 = Mode::from_i64((op_code / 10000) % 10);

        match code {
            1 => Instruction::Add([mode_1, mode_2, mode_3]),
            2 => Instruction::Multiply([mode_1, mode_2, mode_3]),
            3 => Instruction::ReadInput(mode_1),
            4 => Instruction::WriteOutput(mode_1),
            5 => Instruction::JumpIfTrue([mode_1, mode_2]),
            6 => Instruction::JumpIfFalse([mode_1, mode_2]),
            7 => Instruction::LessThan([mode_1, mode_2, mode_3]),
            8 => Instruction::Equals([mode_1, mode_2, mode_3]),
            _ => panic!("Unexpected opcode: {}", op_code),
        }
    }

    fn width(&self) -> usize {
        match self {
            Instruction::Add(_) => 4,
            Instruction::Multiply(_) => 4,
            Instruction::ReadInput(_) => 2,
            Instruction::WriteOutput(_) => 2,
            Instruction::JumpIfTrue(_) => 3,
            Instruction::JumpIfFalse(_) => 3,
            Instruction::LessThan(_) => 4,
            Instruction::Equals(_) => 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub code: Vec<i64>,
    i: usize,
}

impl Program {
    pub fn new(code: Vec<i64>) -> Program {
        Program { code, i: 0 }
    }

    pub fn run<I>(&mut self, inputs: I) -> Vec<i64>
    where
        I: IntoIterator<Item = i64>,
    {
        let mut inputs_iter = inputs.into_iter();
        let mut outputs = Vec::new();

        while let Some(output) = self.run_until_next_output(&mut inputs_iter) {
            outputs.push(output);
        }

        outputs
    }

    pub fn run_until_next_output<I>(&mut self, inputs: &mut I) -> Option<i64>
    where
        I: Iterator<Item = i64>,
    {
        while self.code[self.i] != 99 {
            let instruction = Instruction::from_i64(self.code[self.i]);

            match instruction {
                Instruction::Add([mode_1, mode_2, mode_3]) => {
                    let x = self.read(1, mode_1);
                    let y = self.read(2, mode_2);

                    self.write(3, mode_3, x + y);
                }

                Instruction::Multiply([mode_1, mode_2, mode_3]) => {
                    let x = self.read(1, mode_1);
                    let y = self.read(2, mode_2);

                    self.write(3, mode_3, x * y);
                }

                Instruction::ReadInput(mode) => {
                    let input = inputs.next().expect("No input given");

                    self.write(1, mode, input);
                }

                Instruction::WriteOutput(mode) => {
                    let output = self.read(1, mode);

                    self.i += instruction.width();
                    return Some(output);
                }

                Instruction::JumpIfTrue([mode_1, mode_2]) => {
                    if self.read(1, mode_1) != 0 {
                        self.i = self.read(2, mode_2) as usize;
                        continue;
                    }
                }

                Instruction::JumpIfFalse([mode_1, mode_2]) => {
                    if self.read(1, mode_1) == 0 {
                        self.i = self.read(2, mode_2) as usize;
                        continue;
                    }
                }

                Instruction::LessThan([mode_1, mode_2, mode_3]) => {
                    let x = self.read(1, mode_1);
                    let y = self.read(2, mode_2);

                    let out = if x < y { 1 } else { 0 };

                    self.write(3, mode_3, out);
                }

                Instruction::Equals([mode_1, mode_2, mode_3]) => {
                    let x = self.read(1, mode_1);
                    let y = self.read(2, mode_2);

                    let out = if x == y { 1 } else { 0 };

                    self.write(3, mode_3, out);
                }
            }

            self.i += instruction.width();
        }

        None
    }

    fn read(&self, offset: usize, mode: Mode) -> i64 {
        let val = self.code[self.i + offset];

        match mode {
            Mode::Position => self.code[val as usize],
            Mode::Immediate => val,
        }
    }

    fn write(&mut self, offset: usize, mode: Mode, value: i64) {
        let write_addr = match mode {
            Mode::Position => self.code[self.i + offset] as usize,
            Mode::Immediate => self.i + offset,
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
            .map(Program::new)
    }
}

#[cfg(test)]
mod day_two_tests {
    use super::*;

    fn assert_program_output_is(input: &str, expected_code: Vec<i64>) {
        let mut program = input.parse::<Program>().unwrap();

        program.run(vec![]);

        assert_eq!(program.code, expected_code)
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

        program.run(vec![]);

        assert_eq!(program.code, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn supports_negative_values_in_program_code() {
        let mut program = "1101,100,-1,4,0".parse::<Program>().unwrap();

        program.run(vec![]);

        assert_eq!(program.code, vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn supports_op_codes_3_and_4() {
        let mut program = "3,0,4,0,99".parse::<Program>().unwrap();

        let output = program.run(vec![5]);

        assert_eq!(output, vec!(5));
    }

    #[test]
    fn runs_diagnostic_program_correctly() {
        let mut program = read_to_string("src/five.txt")
            .unwrap()
            .trim()
            .parse::<Program>()
            .unwrap();

        let output = program.run(vec![1]);

        let test_codes = &output[0..output.len() - 1];

        assert!(test_codes.iter().all(|&x| x == 0));
    }
}

#[cfg(test)]
mod day_five_part_two_tests {
    use super::*;

    #[test]
    fn example_case_1() {
        let program = "3,9,8,9,10,9,4,9,99,-1,8".parse::<Program>().unwrap();

        assert_eq!(program.clone().run(vec!(7)), vec!(0));
        assert_eq!(program.clone().run(vec!(8)), vec!(1));
        assert_eq!(program.clone().run(vec!(9)), vec!(0));
    }

    #[test]
    fn example_jump_case_1() {
        let program = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"
            .parse::<Program>()
            .unwrap();

        assert_eq!(program.clone().run(vec!(0)), vec!(0));
        assert_eq!(program.clone().run(vec!(1)), vec!(1));
        assert_eq!(program.clone().run(vec!(-5)), vec!(1));
    }

    #[test]
    fn example_jump_case_2() {
        let program = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1"
            .parse::<Program>()
            .unwrap();

        assert_eq!(program.clone().run(vec!(0)), vec!(0));
        assert_eq!(program.clone().run(vec!(1)), vec!(1));
        assert_eq!(program.clone().run(vec!(-5)), vec!(1));
    }

    #[test]
    fn larger_example() {
        let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"
            .parse::<Program>()
            .unwrap();

        assert_eq!(program.clone().run(vec!(7)), vec!(999));
        assert_eq!(program.clone().run(vec!(8)), vec!(1000));
        assert_eq!(program.clone().run(vec!(9)), vec!(1001));
    }
}
