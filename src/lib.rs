pub mod day01;

use std::fs;

pub fn parse_input(day: u8) -> Vec<String> {
    fs::read_to_string(format!("./challenge-inputs/day{:02}.txt", day))
        .unwrap()
        .lines()
        .map(|x| x.into())
        .collect()
}
