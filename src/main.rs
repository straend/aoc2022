#[allow(dead_code, unused_imports)]
use std::io;
mod helpers;

mod day1;
mod day2;
// Day modules

fn main() -> io::Result<()> {
    let day = std::env::args().nth(1).expect("No day given");
    let bench = std::env::args().len() > 2;
    let res = match day.parse::<i32>().unwrap() {
        1 => day1::run(bench)?,
        2 => day2::run(bench)?,
        // Day invocations
        _ => (0, 0, 0),
    };

    if bench {
        println!("{}\t{}\t{}", res.0, res.1, res.2);
    }

    Ok(())
}
