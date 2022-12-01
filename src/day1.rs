use crate::helpers;
use std::io;

pub fn run() -> io::Result<()> {
    println!("\n\nDay 1");

    // Reads one line as str
    let lines = helpers::read_file_to_vec::<String>("inputs/day1.txt");
    let cc:Vec<i32> = lines.into_iter().map(|x| {
        match x.parse::<i32>() {
            Ok(n) =>  n,
            _ => 0,
        }
        
    }).collect();

    let elfs: Vec<Vec<i32>> = cc.into_iter().fold(Vec::new(), |mut acc, x| {
        if x == 0 || acc.is_empty() {
            acc.push(Vec::new());
        }
        if x > 0 {
            acc.last_mut().unwrap().push(x);
        }
        acc
    });
    let mut sums:Vec<i32> = elfs.iter().map(|x| x.iter().fold(0, |acc, x| acc+x)).collect();
    println!("Part1: {:?}", sums.iter().max());


    sums.sort_by(|a,b| b.cmp(a) );
    let top3:i32 = sums.iter().take(3).sum();
    println!("Part2: {:?}", top3);

    Ok(())
}
