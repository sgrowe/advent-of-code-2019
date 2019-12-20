use super::int_code::*;
use std::fs::read_to_string;

pub fn main() {
    let contents = read_to_string("src/two.txt").unwrap();
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
    let mut program = input.parse::<Program>().unwrap();

    run_program_with_inputs(12, 2, &mut program);

    program.code[0]
}

fn part_two(input: &str) -> (i64, i64) {
    let target = 19690720;

    let initial_program = input.parse::<Program>().unwrap();

    for i in 0..99 {
        for j in 0..99 {
            let mut program = initial_program.clone();

            run_program_with_inputs(i, j, &mut program);

            if program.code[0] == target {
                return (i, j);
            }
        }
    }

    panic!("Could not find a pair of inputs resulting in {}", target);
}

fn run_program_with_inputs(a: i64, b: i64, program: &mut Program) {
    program.code[1] = a;
    program.code[2] = b;

    program.run(vec![]);
}
