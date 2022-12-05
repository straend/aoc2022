use crate::helpers;
use std::io;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day5_test.txt");

        assert_eq!(run_part1(&lines), "CMZ");
    }
    #[test]
    fn test_part2() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day5_test.txt");

        assert_eq!(run_part2(&lines), "MCD");
    }
}

fn get_input(input: &Vec<String>) -> (Vec<Vec<char>>, usize) {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut command_start = 0;
    for _ in 0..input.first().unwrap().len() / 4 + 1 {
        stacks.push(Vec::new());
    }
    for (i, l) in input.iter().enumerate() {
        let chars = l.chars().collect::<Vec<char>>();
        for (i, c) in chars.iter().enumerate() {
            if c == &'[' {
                stacks[i / 4].push(chars[i + 1]);
            }
        }

        if l.len() < 2 {
            command_start = i + 1;
            break;
        }
    }
    for s in stacks.iter_mut() {
        s.reverse();
    }

    (stacks, command_start)
}
#[inline]
fn get_command(command: &String) -> (usize, usize, usize) {
    let splits: Vec<u32> = command
        .split_whitespace()
        .map(|x| u32::from_str_radix(x, 10).unwrap_or(0))
        .collect();
    let to_move = splits[1] as usize;
    let from = splits[3] as usize - 1;
    let to = splits[5] as usize - 1;

    (to_move, from, to)
}
pub fn run_part1(input: &Vec<String>) -> String {
    let (mut stacks, command_start) = get_input(input);

    for command in input.iter().skip(command_start) {
        let (to_move, from, to) = get_command(command);

        for _ in 0..to_move {
            let movethis = stacks[from].pop().unwrap();
            stacks[to].push(movethis);
        }
    }
    let res: String = stacks
        .iter_mut()
        .map(|s| s.pop().unwrap())
        .collect::<String>();
    res
}

pub fn run_part2(input: &Vec<String>) -> String {
    let (mut stacks, command_start) = get_input(input);

    for command in input.iter().skip(command_start) {
        let (to_move, from, to) = get_command(command);

        let mut move_this = Vec::new();
        for _ in 0..to_move {
            let movethis = stacks[from].pop().unwrap();
            move_this.push(movethis);
        }
        for c in move_this.iter().rev() {
            stacks[to].push(*c);
        }
    }
    let res: String = stacks
        .iter_mut()
        .map(|s| s.pop().unwrap())
        .collect::<String>();
    res
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 5");

    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/day5.txt");

    // Part 1 goes here
    let p1 = run_part1(&lines);
    println!("Part1: {:?}", p1);

    // Part 2 Goes here
    let p2 = run_part2(&lines);
    println!("Part2: {:?}", p2);

    Ok(())
}
