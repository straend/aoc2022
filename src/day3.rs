use crate::helpers;
use std::io;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day3_test.txt");

        assert_eq!(run_part1(&lines), 157_i128);
    }
    #[test]
    fn test_part2() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day3_test.txt");

        assert_eq!(run_part2(&lines), 70_i128);
    }
}

pub fn run_part1(input: &Vec<String>) -> i128 {
    let mut sum:i32=0;
    for l in input {
        let (s1,s2) = l.split_at(l.len()/2);
        for x in s1.chars() {
            if s2.contains(x) {
                sum += if x.is_ascii_uppercase() {x as i32 - 38} else {x as i32 - 96} as i32;
                break;
            }
        }

    }

    sum as i128
}

pub fn run_part2(input: &Vec<String>) -> i128 {
    let mut iter = input.iter();
    let mut sum = 0;
    for _ in 0..iter.len() / 3 {
        let v1 = iter.next().unwrap();
        let v2 = iter.next().unwrap();
        let v3 = iter.next().unwrap();
        for x in v1.chars() {
            if v2.contains(x) && v3.contains(x) {
                sum += if x.is_ascii_uppercase() {x as i32 - 38} else {x as i32 - 96} as i32;
                
                break;
            }
        }
    }
    sum as i128
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 3");

    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/day3.txt");

    // Part 1 goes here
    let p1 = run_part1(&lines);
    println!("Part1: {:?}", p1);

    // Part 2 Goes here
    let p2 = run_part2(&lines);
    println!("Part2: {:?}", p2);

    Ok(())
}
