use crate::helpers;
use std::io;
use std::time::Instant;

pub fn run(bench: bool) -> io::Result<(u128, u128, u128)> {
    if !bench {
        println!("\n\nDay DAY_NUMBER");
    }
    let start = Instant::now();

    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/dayDAY_NUMBER_test.txt");
    

    let t_input = start.elapsed().as_micros();
    let start = Instant::now();
    
    // Part 1 goes here
    
    let t_part1 = start.elapsed().as_micros();
    if !bench {
        println!("Part1: {:?}", 1);
    }
    let start = Instant::now();
    
    // Part 2 Goes here


    let t_part2 = start.elapsed().as_micros();
    if !bench {
        println!("Part2: {:?}", 2);
    }

    Ok((t_input, t_part1, t_part2))


}
