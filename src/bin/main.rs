#[allow(dead_code, unused_imports)]
use std::io;

use aoc2022::*;

fn main() -> io::Result<()> {
    let day = std::env::args().nth(1).expect("No day given");
    match day.parse::<i32>().unwrap() {
        1 => day1::run()?,
        2 => day2::run()?,
        3 => day3::run()?,
        // Day invocations
        _ => {}
    };

    Ok(())
}