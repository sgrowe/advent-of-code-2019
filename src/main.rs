mod five;
mod four;
mod int_code;
mod one;
mod rolling_pairs;
mod three;
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

    print_day_heading("FIVE");
    five::main();

    println!();
}

fn print_day_heading(day: &str) {
    println!();
    println!(">>>");
    println!(">>> DAY {}", day);
    println!(">>>");
    println!();
}
