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

const LETTER_A: u8 = 'A' as u8;
const LETTER_Z: u8 = 'Z' as u8;
const LETTER_S_A: u8 = 'a' as u8;
const LETTER_S_Z: u8 = 'z' as u8;

pub fn run_part1(input: &Vec<String>) -> i128 {
    let p: Vec<(Vec<u8>, Vec<u8>)> = input
        .iter()
        .map(|x| {
            let sp = x.split_at(x.len() / 2);
            let v1: Vec<u8> =
                sp.0.bytes()
                    .map(|b| match b {
                        LETTER_A..=LETTER_Z => b - 38,
                        LETTER_S_A..=LETTER_S_Z => b - 96,
                        _ => 0,
                    })
                    .collect();
            let v2: Vec<u8> =
                sp.1.bytes()
                    .map(|b| match b {
                        LETTER_A..=LETTER_Z => b - 38,
                        LETTER_S_A..=LETTER_S_Z => b - 96,
                        _ => 0,
                    })
                    .collect();
            (v1, v2)
        })
        .collect();
    let mut intersect = Vec::new();
    for l in &p {
        for x in l.0.iter().filter(|u| l.1.contains(u)) {
            intersect.push(*x);
            // Only one item "extra" per backpack
            break;
        }
    }
    intersect.iter().fold(0, |acc, x| acc + *x as i128)
}

pub fn run_part2(input: &Vec<String>) -> i128 {
    let p: Vec<Vec<u8>> = input
        .iter()
        .map(|x| {
            let v1: Vec<u8> = x
                .bytes()
                .map(|b| match b {
                    LETTER_A..=LETTER_Z => b - 38,
                    LETTER_S_A..=LETTER_S_Z => b - 96,
                    _ => 0,
                })
                .collect();

            v1
        })
        .collect();
    let mut iter = p.iter();
    let mut codes: Vec<u8> = Vec::new();
    for _ in 0..iter.len() / 3 {
        let v1 = iter.next().unwrap();
        let v2 = iter.next().unwrap();
        let v3 = iter.next().unwrap();
        for c in v1 {
            if v2.contains(c) && v3.contains(c) {
                codes.push(*c);
                break;
            }
        }
    }
    codes.iter().fold(0, |acc, x| acc + *x as i128)
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
