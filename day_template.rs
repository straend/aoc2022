use crate::helpers;
use std::io;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/dayDAY_NUMBER_test.txt");

        assert_eq!(run_part1(&lines), 100_i128);
    }
    #[test]
    fn test_part2() {
        let lines = helpers::read_file_to_vec::<String>("inputs/dayDAY_NUMBER_test.txt");

        assert_eq!(run_part2(&lines), 70_i128);
    }
}

pub fn run_part1(input: &Vec<String>) -> i128 {
    100
}

pub fn run_part2(input: &Vec<String>) -> i128 {
    200
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay DAY_NUMBER");
    
    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/dayDAY_NUMBER.txt");

    // Part 1 goes here
    let p1 = run_part1(&lines);
    println!("Part1: {:?}", p1);
    
    // Part 2 Goes here
    let p2 = run_part2(&lines);
    println!("Part2: {:?}", p2);
    
    Ok(())
}
