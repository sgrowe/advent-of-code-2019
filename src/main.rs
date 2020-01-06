mod five;
mod four;
mod int_code;
mod one;
mod permutations;
mod rolling_pairs;
mod seven;
mod six;
mod three;
mod two;

fn main() {
    run_day("ONE", one::main);

    run_day("TWO", two::main);

    run_day("THREE", three::main);

    run_day("FOUR", four::main);

    run_day("FIVE", five::main);

    run_day("SIX", six::main);

    run_day("SEVEN", seven::main);
}

fn run_day(name: &str, print_solutions: impl FnOnce()) {
    print_day_heading(name);
    print_solutions();
    println!();
}

fn print_day_heading(day: &str) {
    println!();
    println!(">>>");
    println!(">>> DAY {}", day);
    println!(">>>");
    println!();
}
