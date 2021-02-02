use aoc_2020::{day01, parse_input};

fn main() {
    let mut day: u8 = 1;
    println!("Results of Day {:02}:", day);
    println!("  - [A]: {}", day01::a(parse_input(day)));
    println!("  - [B]: {}\n", day01::b(parse_input(day)));
    // day += 1;

    // println!("Results of Day {:02}:", day);
    // println!("  - [A]: {}", day02::a(parse_input(day, 1)));
    // println!("  - [B]: {}\n", day02::b(parse_input(day, 2)));
    // day += 1;
}
