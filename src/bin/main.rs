#[allow(dead_code, unused_imports)]
use std::io;

use aoc2022::*;

fn main() -> io::Result<()> {
    let day = std::env::args().nth(1).expect("No day given");
    match day.parse::<i32>().unwrap() {
        1 => day1::run()?,
        2 => day2::run()?,
        3 => day3::run()?,
        4 => day4::run()?,
        5 => day5::run()?,
        6 => day6::run()?,
        7 => day7::run()?,
        8 => day8::run()?,
        9 => day9::run()?,
        10 => day10::run()?,
        11 => day11::run()?,
        // Day invocations
        _ => {}
    };

    Ok(())
}
