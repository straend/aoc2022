use crate::helpers;
use std::io;
use std::time::Instant;

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

pub fn run(bench: bool) -> io::Result<(u128, u128, u128)> {
    if !bench {
        println!("\n\nDay DAY_NUMBER");
    }
    let start = Instant::now();

    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/dayDAY_NUMBER.txt");

    let t_input = start.elapsed().as_micros();
    let start = Instant::now();

    // Part 1 goes here
    let p1 = run_part1(&lines);

    let t_part1 = start.elapsed().as_micros();
    if !bench {
        println!("Part1: {:?}", p1);
    }
    let start = Instant::now();

    // Part 2 Goes here
    let p2 = run_part2(&lines);

    let t_part2 = start.elapsed().as_micros();

    if !bench {
        println!("Part2: {:?}", p2);
    }

    Ok((t_input, t_part1, t_part2))
}
