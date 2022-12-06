use crate::helpers;
use std::{io, collections::{HashSet, HashMap}};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        //let lines = helpers::read_file_to_vec::<String>("inputs/day6_test.txt");

        assert_eq!(run_part1(&String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")), 5);
        assert_eq!(run_part1(&String::from("nppdvjthqldpwncqszvftbrmjlhg")), 6);
        assert_eq!(run_part1(&String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 10);
        assert_eq!(run_part1(&String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 11);
    }
    #[test]
    fn test_part2() {
        assert_eq!(run_part2(&String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 19);
        assert_eq!(run_part2(&String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")), 23);
        assert_eq!(run_part2(&String::from("nppdvjthqldpwncqszvftbrmjlhg")), 23);
        assert_eq!(run_part2(&String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 29);
        assert_eq!(run_part2(&String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 26);
    }
}

pub fn run_part1(input: &String) -> usize {
    if input.len() < 4 { return 0; }
    let mut iter = input.chars();
    let mut a = iter.next().unwrap();
    let mut b = iter.next().unwrap();
    let mut c = iter.next().unwrap();
    let mut d = iter.next().unwrap();
    
    for i in 4..input.len()-4 {
        if a != b && a != c && a != d && b!=c && b!=d && c!=d {
            return i;
        }
        a=b;
        b=c;
        c=d;
        d=iter.next().unwrap();
    }
    0
    
}

pub fn run_part2(input: &String) -> usize {
    const UNIQUE_CHARS:usize = 14;
    if input.len() < UNIQUE_CHARS {
        return  0;
    }
    let mut map:HashMap<char, u32> = HashMap::with_capacity(UNIQUE_CHARS);
    let mut iter = input.chars();
    let mut iter_remove = input.chars();

    for _ in 0..UNIQUE_CHARS {
        let n = iter.next().unwrap();
        if map.contains_key(&n) {
            let x = map.get_mut(&n).unwrap();
            *x += 1;
        } else {
            map.insert(n, 1);
        }
    }
    
    for i in UNIQUE_CHARS+1..input.len() {
    
        let r = map.get_mut(&iter_remove.next().unwrap()).unwrap();
        *r -= 1;
        
        let n = iter.next().unwrap();
        if map.contains_key(&n) {
            let x = map.get_mut(&n).unwrap();
            *x += 1;
        } else {
            map.insert(n, 1);
        }
        if map.iter().filter(| (_,v) | **v == 1).count() == UNIQUE_CHARS {
            return i;
        }
    }
    
    0
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 6");

    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/day6.txt");

    // Part 1 goes here
    let p1 = run_part1(&lines.first().unwrap());
    println!("Part1: {:?}", p1);

    // Part 2 Goes here
    let p2 = run_part2(&lines.first().unwrap());
    println!("Part2: {:?}", p2);

    Ok(())
}
