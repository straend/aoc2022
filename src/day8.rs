use itertools::Itertools;

use crate::helpers;
use std::cmp::Ordering;
use std::{fs::read, io};
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day8_test.txt");
        assert_eq!(run_part1(&lines), 21);

        let lines = helpers::read_file_to_vec::<String>("inputs/day8.txt");
        let part1 = run_part1(&lines);
        assert_ne!(part1, 732); // Too low
        assert_ne!(part1, 1524); // Too low

        assert_ne!(part1, 6122); // Too high
        assert_eq!(part1, 1533); // Perfect
    }
    #[test]
    fn test_part2() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day8_test.txt");
        assert_eq!(run_part2(&lines), 8);

        let lines = helpers::read_file_to_vec::<String>("inputs/day8.txt");
        let part2 = run_part2(&lines);
        assert_ne!(part2, 26460); // Too low
        assert!(part2 > 26460, "Not big enough");
    }
}
#[derive(Debug)]
struct Matrix<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}
#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Tree {
    height: u32,
    visible: bool,
    up: usize,
    down: usize,
    left: usize,
    right: usize,
    scenic: usize,
}
impl Ord for Tree {
    fn cmp(&self, other: &Self) -> Ordering {
        self.height.cmp(&other.height)
    }
}
/*
trait ScenicScore {
    fn scenic(self) -> u32;
}
impl ScenicScore for Tree {
    fn scenic(self) -> u32 {
        self.down * self.up * self.left * self.right
    }
}
*/
fn print_matrix(data: &Matrix<Tree>) {
    for row in 0..data.height {
        let iter = data.data.iter().skip(row * data.width).take(data.width);
        for (col, c) in iter.enumerate() {
            print!("[{},{}]", row, col);
            if c.visible {
                print!("({})", c.height);
            } else {
                print!(" {} ", c.height);
            }
        }
        println!("");
    }
}
fn read_input(input: &Vec<String>) -> Matrix<Tree> {
    let size = input.first().unwrap().len();
    let mut ret = Matrix {
        width: size,
        height: size,
        data: Vec::new(),
    };

    for l in input {
        ret.data.append(
            &mut l
                .chars()
                .map(|x| Tree {
                    height: u32::from(x) - 48,
                    visible: false,
                    left: 0,
                    right: 0,
                    up: 0,
                    down: 0,
                    scenic: 0,
                })
                .collect::<Vec<Tree>>(),
        );
    }

    ret
}
fn mark_borders_visible(data: &mut Matrix<Tree>) {
    data.data
        .iter_mut()
        .take(data.width)
        .for_each(|x| x.visible = true);
    data.data
        .iter_mut()
        .rev()
        .take(data.width)
        .for_each(|x| x.visible = true);
    data.data
        .iter_mut()
        .step_by(data.width)
        .take(data.width)
        .for_each(|x| x.visible = true);
    data.data
        .iter_mut()
        .rev()
        .step_by(data.width)
        .take(data.width)
        .for_each(|x| x.visible = true);
}

pub fn run_part1(input: &Vec<String>) -> usize {
    let mut data = read_input(input);
    let mut visible = 0;

    for row in 1..data.height - 1 {
        // Left ro right, ok
        let mut prev_max = data.data.get(row * data.width).unwrap().height;
        for col in 1..data.width - 1 {
            for x in data.data.iter_mut().skip(row * data.width + col).take(col) {
                if x.height > prev_max {
                    x.visible = true;
                }
                prev_max = if x.height > prev_max {
                    x.height
                } else {
                    prev_max
                };
            }
        }

        // From right to left, ok
        let mut prev_max = data.data.get((row + 1) * data.width - 1).unwrap().height;
        for col in 1..data.width - 1 {
            for x in data
                .data
                .iter_mut()
                .skip(row * data.width + (data.width - col) - 1)
                .take(col)
                .rev()
            {
                if x.height > prev_max {
                    x.visible = true;
                }
                prev_max = if x.height > prev_max {
                    x.height
                } else {
                    prev_max
                };
            }
        }
    }
    for col in 1..data.width - 1 {
        // Top to bottom, ok
        let mut prev_max = data.data.get(col).unwrap().height;
        for row in 1..data.height - 1 {
            for x in data
                .data
                .iter_mut()
                .skip(col + (row - 1) * data.width)
                .step_by(data.width)
                .take(row)
            {
                if x.height > prev_max {
                    x.visible = true;
                }
                prev_max = if x.height > prev_max {
                    x.height
                } else {
                    prev_max
                };
            }
        }

        // Bottom up, ok
        let mut prev_max = data
            .data
            .get(data.height * data.width - data.width + col)
            .unwrap()
            .height;
        for row in 1..data.height - 1 {
            let start = ((data.height - row - 1) * data.width) + col;
            for x in data
                .data
                .iter_mut()
                .skip(start)
                .step_by(data.width)
                .take(row)
                .rev()
            {
                if x.height > prev_max {
                    x.visible = true;
                }
                prev_max = if x.height > prev_max {
                    x.height
                } else {
                    prev_max
                };
            }
        }
    }

    mark_borders_visible(&mut data);

    let inside_visible = data.data.iter().filter(|t| t.visible == true).count();
    visible + inside_visible
}

fn get_visible_from(iter: &mut dyn Iterator<Item = &Tree>) -> usize {
    let xx = iter.next();
    if xx == None {
        return 1;
    }
    let me = xx.unwrap().height;
    let mut last = me;
    let mut can_see = 0;
    print!(" [{}]  ", me);

    for x in iter {
        print!(" {}  ", x.height);
        if x.height >= me {
            print!(" >");
            can_see += 1;
            break;
        } else if x.height == last && me > last {
            print!(" =");
            break;
        } else {
            can_see += 1;
        }
        last = x.height;
    }
    println!("  ({})", can_see);
    can_see
}

pub fn run_part2(input: &Vec<String>) -> usize {
    let mut data = read_input(input);
    print_matrix(&data);
    println!("---");
    let row = 3;
    let col = 2;
    for row in 1..data.height - 1 {
        for col in 1..data.width - 1 {
            // To right
            let start = row * data.width + col;
            let take = data.width - col;

            let mut iter = data.data.iter().skip(start).take(take);
            data.data.get_mut(start).unwrap().right = get_visible_from(&mut iter);

            // To left
            let start = data.data.len() - (row + 1) * data.width + col;
            let take = data.width - col;

            let mut iter = data.data.iter().rev().skip(start).take(take);
            data.data.get_mut(start).unwrap().left = get_visible_from(&mut iter);
        }
    }

    for col in 1..data.width - 1 {
        for row in 1..data.height - 1 {
            // To Down
            let start = row * data.width + col;
            let take = data.height - row;

            let mut iter = data.data.iter().skip(start).step_by(data.width).take(take);
            data.data.get_mut(start).unwrap().down = get_visible_from(&mut iter);

            // To up is wrong
            let start = data.data.len() - (row + 1) * data.width + col;
            let take = data.height - row + 1;

            let mut iter = data
                .data
                .iter()
                .rev()
                .skip(start)
                .step_by(data.width)
                .take(take);
            /*
            for c in iter {
                print!("{} ", c.height);
            }
            print!("\n");
            */
            data.data.get_mut(start).unwrap().up = get_visible_from(&mut iter);
        }
    }

    for row in 1..data.height - 1 {
        for (i, x) in data
            .data
            .iter_mut()
            .skip(row * data.width + 1)
            .take(3)
            .enumerate()
        {
            x.scenic = x.up * x.down * x.left * x.right;
            println!("{}\t{:?}", i + row * data.width, x);
        }
    }

    let xx = data
        .data
        .iter()
        .max_by(|&x, &y| x.scenic.cmp(&y.scenic))
        .unwrap();

    xx.scenic
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 8");

    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/day8_test.txt");
    //let mut data = read_input(&lines);
    //print_matrix(&data);

    // Part 1 goes here
    let p1 = run_part1(&lines);
    println!("Part1: {:?}", p1);

    // Part 2 Goes here
    let p2 = run_part2(&lines);
    println!("Part2: {:?}", p2);

    Ok(())
}
