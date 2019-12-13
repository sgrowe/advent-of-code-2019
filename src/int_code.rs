pub type Program = Vec<i64>;

pub fn parse_program(input: &str) -> Program {
    input
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Program>()
}

pub fn run_program(program: &mut Program) {
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

fn deref(program: &Program, i: usize) -> i64 {
    let pointer = program[i] as usize;

    program[pointer]
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