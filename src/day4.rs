use crate::helpers;
use std::io;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day4_test.txt");

        assert_eq!(run_part1(&lines), 2);
    }
    #[test]
    fn test_part2() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day4_test.txt");

        assert_eq!(run_part2(&lines), 4_i128);
    }
}

fn get_parsed_input(input: &Vec<String>) -> Vec<((u32, u32), (u32, u32))> {
    let mut pairs = Vec::new();
    for l in input {
        let (p1, p2) = l.split_once(',').unwrap();

        let (p1_1, p1_2) = p1.split_once('-').unwrap();
        let (p2_1, p2_2) = p2.split_once('-').unwrap();

        pairs.push((
            (
                u32::from_str_radix(p1_1, 10).unwrap(),
                u32::from_str_radix(p1_2, 10).unwrap(),
            ),
            (
                u32::from_str_radix(p2_1, 10).unwrap(),
                u32::from_str_radix(p2_2, 10).unwrap(),
            ),
        ));
    }
    pairs
}

pub fn run_part1(input: &Vec<String>) -> i128 {
    let pairs = get_parsed_input(input);

    let mut doubles = 0;

    // Check if one is included on the other
    for (p1, p2) in pairs {
        if p1.0 >= p2.0 && p1.1 <= p2.1 {
            doubles += 1;
            continue;
        }
        if p2.0 >= p1.0 && p2.1 <= p1.1 {
            doubles += 1;
            continue;
        }
    }

    doubles
}

pub fn run_part2(input: &Vec<String>) -> i128 {
    let mut doubles = 0;

    let pairs = get_parsed_input(input);

    // Check if one is included on the other
    for (p1, p2) in pairs {
        if (p1.0 >= p2.0 && p1.0 <= p2.1)
            || (p1.1 <= p2.1 && p1.1 >= p2.0)
            || (p2.0 >= p1.0 && p2.0 <= p1.1)
            || (p2.1 <= p1.1 && p2.1 >= p1.0)
        {
            doubles += 1;
            continue;
        }
    }
    doubles
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 4");

    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/day4.txt");

    // Part 1 goes here
    let p1 = run_part1(&lines);
    println!("Part1: {:?}", p1);

    // Part 2 Goes here
    let p2 = run_part2(&lines);
    println!("Part2: {:?}", p2);

    Ok(())
}
