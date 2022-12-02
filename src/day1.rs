use crate::helpers;
use std::io;
use std::time::Instant;

pub fn run(bench: bool) -> io::Result<(u128, u128, u128)> {
    if !bench {
        println!("\n\nDay 1");
    }
    let start = Instant::now();

    // Reads one line as str
    let lines = helpers::read_file_to_vec::<String>("inputs/day1.txt");
    let cc: Vec<i32> = lines
        .into_iter()
        .map(|x| match x.parse::<i32>() {
            Ok(n) => n,
            _ => 0,
        })
        .collect();

    let t_input = start.elapsed().as_micros();
    let start = Instant::now();
    
    let elfs: Vec<Vec<i32>> = cc.into_iter().fold(Vec::new(), |mut acc, x| {
        if x == 0 || acc.is_empty() {
            acc.push(Vec::new());
        }
        if x > 0 {
            acc.last_mut().unwrap().push(x);
        }
        acc
    });
    let mut sums: Vec<i32> = elfs
        .iter()
        .map(|x| x.iter().fold(0, |acc, x| acc + x))
        .collect();
    let p1 = sums.iter().max();
    
    let t_part1 = start.elapsed().as_micros();
    if !bench {
        println!("Part1: {:?}", p1);
    }
    let start = Instant::now();
    
    sums.sort_by(|a, b| b.cmp(a));
    let top3: i32 = sums.iter().take(3).sum();

    let t_part2 = start.elapsed().as_micros();
    if !bench {
        println!("Part2: {}", top3);
    }

    Ok((t_input, t_part1, t_part2))


}
