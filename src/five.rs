use super::int_code::*;
use std::collections::VecDeque;
use std::fs::read_to_string;

pub fn main() {
    let input = read_to_string("src/five.txt").unwrap();
    let mut program = input.trim().parse::<Program>().unwrap();

    println!("Part one:");
    println!("{}\n", part_one(&mut (program.clone())));
}

fn part_one(program: &mut Program) -> i64 {
    let mut inputs = VecDeque::new();
    inputs.push_back(1);

    program.inputs = inputs;

    let output = program.run();

    output[output.len() - 1]
}
