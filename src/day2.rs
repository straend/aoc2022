use crate::helpers;
use std::io;
use std::time::Instant;


pub fn run(bench: bool) -> io::Result<(u128, u128, u128)> {
    if !bench {
        println!("\n\nDay 2");
    }
    let start = Instant::now();
    
    let lines = helpers::read_file_to_vec::<String>("inputs/day2.txt");
    let lines: Vec<(&str, &str)> = lines.iter().map(|x| { x.split_once(" ").unwrap() }).collect();
    
    let t_input = start.elapsed().as_micros();
    
    let start = Instant::now();
    // You  Win      6 points
    //      Loose    0
    //      Draw     3
    // Choose
    //      Rock     1
    //      Paper    2
    //      Scissors 3
    
    // Not so elegant, bore of a brute force mapping, but it works
    let sum:i32 = lines.iter().map(|x| {
        match x {
            // A X Rock
            // B Y Paper
            // C Z Scissor
            // Rock
            ("A", "X") => 1+3,
            ("A", "Y") => 2+6,
            ("A", "Z") => 3+0,

            // Paper
            ("B", "X") => 1+0,
            ("B", "Y") => 2+3,
            ("B", "Z") => 3+6,

            // Scisssor
            ("C", "X") => 1+6,
            ("C", "Y") => 2+0,
            ("C", "Z") => 3+3,
            (_, _) => 0,
        }
    }).fold(0, |acc, x| acc+ x);
    
    let t_part1 = start.elapsed().as_micros();
    if !bench {
        println!("Part1: {:?}", sum);
    }
    let start = Instant::now();

    let sum:i32 = lines.iter().map(|x| {
        match x {
            // A X Rock
            // B Y Paper
            // C Z Scissor
            // Second column 
            //  X Loose, Y Draw, Z Win
            // Rock
            ("A", "X") => 3+0,
            ("A", "Y") => 1+3,
            ("A", "Z") => 2+6,

            // Paper
            ("B", "X") => 1+0,
            ("B", "Y") => 2+3,
            ("B", "Z") => 3+6,

            // Scisssor
            ("C", "X") => 2+0,
            ("C", "Y") => 3+3,
            ("C", "Z") => 1+6,
            (_, _) => 0,
        }
    }).fold(0, |acc, x| acc+ x);

    let t_part2 = start.elapsed().as_micros();
    if !bench {
        println!("Part2: {}", sum);
    }

    Ok((t_input, t_part1, t_part2))
}
