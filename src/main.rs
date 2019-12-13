mod file_utils;
mod four;
mod one;
mod rolling_pairs;
mod three;
mod int_code;
mod two;

fn main() {
    print_day_heading("ONE");
    one::main();

    print_day_heading("TWO");
    two::main();

    print_day_heading("THREE");
    three::main();

    print_day_heading("FOUR");
    four::main();
}

fn print_day_heading(day: &str) {
    println!();
    println!(">>>");
    println!(">>> DAY {}", day);
    println!(">>>");
    println!();
}
