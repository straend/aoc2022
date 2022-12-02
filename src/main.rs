#[allow(dead_code, unused_imports)]
use std::io;
mod helpers;

mod day1;
mod day2;
// Day modules

fn main() -> io::Result<()> {
    let day = std::env::args().nth(1).expect("No day given");
    match day.parse::<i32>().unwrap() {
        1 => day1::run()?,
        2 => day2::run()?,
        // Day invocations
        _ => println!("Not implemented"),
    }

    Ok(())
}
