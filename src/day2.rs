use crate::helpers;
use std::io;


pub fn run() -> io::Result<()> {
    println!("\n\nDay 2");
    let lines = helpers::read_file_to_vec::<String>("inputs/day2.txt");
    // You  Win      6 points
    //      Loose    0
    //      Draw     3
    // Choose
    //      Rock     1
    //      Paper    2
    //      Scissors 3
    
    // Not so elegant, bore of a brute force mapping, but it works
    let sum:i32 = lines.iter().map(|x| {
        match x.split_once(" ").unwrap() {
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
    println!("Part1: {:?}", sum);

    let sum:i32 = lines.iter().map(|x| {
        match x.split_once(" ").unwrap() {
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
    println!("Part2: {}", sum);

    Ok(())
}
