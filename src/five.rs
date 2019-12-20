use super::int_code::*;
use std::fs::read_to_string;

pub fn main() {
    let input = read_to_string("src/five.txt").unwrap();
    let mut program = input.trim().parse::<Program>().unwrap();

    println!("Part one:");
    println!("{}", part_one(&mut (program.clone())));
    println!();

    println!("Part two:");
    println!("{}", part_two(&mut program));
    println!();
}

fn part_one(program: &mut Program) -> i64 {
    let output = program.run(vec![1]);

    output[output.len() - 1]
}

fn part_two(program: &mut Program) -> i64 {
    let output = program.run(vec![5]);

    output[0]
}
