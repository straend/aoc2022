use crate::helpers;
use std::io;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_equals() {
        let res = 10;
        assert_eq!(res, 10);
    }
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay DAY_NUMBER");

    // Reads one line as i64
    let lines = helpers::read_file_to_vec::<i64>("inputs/dayDAY_NUMBER.txt");

    println!("Part1: {}", 1);

    println!("Part2: {}", 2);

    Ok(())
}
