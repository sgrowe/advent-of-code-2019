mod file_utils;
mod one;
mod three;
mod two;

fn main() {
    print_day_heading("ONE");
    one::main();

    print_day_heading("TWO");
    two::main();

    print_day_heading("THREE");
    three::main();
}

fn print_day_heading(day: &str) {
    println!();
    println!(">>>");
    println!(">>> DAY {}", day);
    println!(">>>");
    println!();
}
