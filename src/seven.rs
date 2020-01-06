use super::int_code::*;
use super::permutations::Permutations;
use std::fs::read_to_string;

pub fn main() {
    let input = read_to_string("src/seven.txt").unwrap();
    let program = input.trim().parse::<Program>().unwrap();

    println!("Part one:");
    println!(
        "Max thruster signal: {}",
        max_thruster_signal(&program, [0, 1, 2, 3, 4], run_amplifier_controller)
    );
    println!();

    println!("Part two:");
    println!(
        "Max thruster signal: {}",
        max_thruster_signal(&program, [5, 6, 7, 8, 9], run_amplifier_feedback_loop)
    );
    println!();
}

fn max_thruster_signal(
    program: &Program,
    phases: [i64; 5],
    run_amplifier: impl Fn(&Program, [i64; 5]) -> i64,
) -> i64 {
    Permutations::of(phases)
        .map(|phases| run_amplifier(&program, phases))
        .max()
        .unwrap()
}

fn run_amplifier_controller(program: &Program, [i, j, k, l, m]: [i64; 5]) -> i64 {
    let mut input = 0;

    input = first_output_of(program, [i, input]);
    input = first_output_of(program, [j, input]);
    input = first_output_of(program, [k, input]);
    input = first_output_of(program, [l, input]);
    input = first_output_of(program, [m, input]);

    input
}

fn first_output_of(program: &Program, inputs: [i64; 2]) -> i64 {
    let input = inputs.iter().copied();

    *program.clone().run(input).get(0).unwrap()
}

fn run_amplifier_feedback_loop(program: &Program, [i, j, k, l, m]: [i64; 5]) -> i64 {
    let mut a = FeedbackAmplifierProgram::new(program.clone(), vec![i, 0]);
    let mut b = FeedbackAmplifierProgram::new(program.clone(), vec![j]);
    let mut c = FeedbackAmplifierProgram::new(program.clone(), vec![k]);
    let mut d = FeedbackAmplifierProgram::new(program.clone(), vec![l]);
    let mut e = FeedbackAmplifierProgram::new(program.clone(), vec![m]);

    let mut output = 0;

    loop {
        output = match a
            .run_until_next_output(&mut b)
            .and_then(|_| b.run_until_next_output(&mut c))
            .and_then(|_| c.run_until_next_output(&mut d))
            .and_then(|_| d.run_until_next_output(&mut e))
            .and_then(|_| e.run_until_next_output(&mut a))
        {
            Some(x) => x,
            None => return output,
        };
    }
}

struct FeedbackAmplifierProgram {
    program: Program,
    inputs: Vec<i64>,
}

impl FeedbackAmplifierProgram {
    fn new(program: Program, inputs: Vec<i64>) -> FeedbackAmplifierProgram {
        FeedbackAmplifierProgram { program, inputs }
    }

    fn push_input(&mut self, input: i64) {
        self.inputs.push(input);
    }

    fn run_until_next_output(&mut self, output_to: &mut FeedbackAmplifierProgram) -> Option<i64> {
        let mut inputs_iter = self.inputs.iter().copied();
        let out = self.program.run_until_next_output(&mut inputs_iter);
        self.inputs = inputs_iter.collect();

        for x in out {
            output_to.push_input(x);
        }

        out
    }
}

#[cfg(test)]
mod part_one_tests {
    use super::*;

    #[test]
    fn example_case_1() {
        let program = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
            .parse::<Program>()
            .unwrap();

        assert_eq!(
            max_thruster_signal(&program, [0, 1, 2, 3, 4], run_amplifier_controller),
            43210
        );
    }

    #[test]
    fn example_case_2() {
        let program = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
            .parse::<Program>()
            .unwrap();

        assert_eq!(
            max_thruster_signal(&program, [0, 1, 2, 3, 4], run_amplifier_controller),
            54321
        );
    }

    #[test]
    fn example_case_3() {
        let program = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
            .parse::<Program>()
            .unwrap();

        assert_eq!(
            max_thruster_signal(&program, [0, 1, 2, 3, 4], run_amplifier_controller),
            65210
        );
    }
}

#[cfg(test)]
mod part_two_tests {
    use super::*;

    #[test]
    fn example_case_1() {
        let program =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
                .parse::<Program>()
                .unwrap();

        assert_eq!(
            max_thruster_signal(&program, [5, 6, 7, 8, 9], run_amplifier_feedback_loop),
            139629729
        );
    }

    #[test]
    fn example_case_2() {
        let program =
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
                .parse::<Program>()
                .unwrap();

        assert_eq!(
            max_thruster_signal(&program, [5, 6, 7, 8, 9], run_amplifier_feedback_loop),
            18216
        );
    }
}
