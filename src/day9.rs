use crate::helpers;
use std::{
    collections::HashSet,
    hash::Hash,
    io,
    ops::{Add, Sub},
    str::FromStr,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day9_test.txt");

        assert_eq!(run_part1(&lines), 13);

        let lines = helpers::read_file_to_vec::<String>("inputs/day9.txt");
        let part1 = run_part1(&lines);
        assert_ne!(part1, 67);
        assert!(part1 > 200, "Too low");
        assert_eq!(part1, 6236);
    }
    #[test]
    fn test_part2() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day9_test.txt");
        assert_eq!(run_part2(&lines), 1);

        let lines = helpers::read_file_to_vec::<String>("inputs/day9_test2.txt");
        assert_eq!(run_part2(&lines), 36);

        let lines = helpers::read_file_to_vec::<String>("inputs/day9.txt");
        assert_eq!(run_part2(&lines), 2449);
    }
}
#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Rigth,
    Up,
    Down,
}
#[derive(Debug)]
struct Command {
    direction: Direction,
    steps: i32,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split_ascii_whitespace().collect();

        match (split.get(0), split.get(1)) {
            (None, _) => return Err(()),
            (_, None) => return Err(()),
            (Some(d), Some(s)) => Ok(Command {
                direction: match d {
                    &"L" => Direction::Left,
                    &"R" => Direction::Rigth,
                    &"D" => Direction::Down,
                    &"U" => Direction::Up,
                    _ => return Err(()),
                },
                steps: i32::from_str(s).unwrap(),
            }),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn move_dir(&mut self, dir: &Direction) {
        match dir {
            Direction::Left => self.x -= 1,
            Direction::Rigth => self.x += 1,
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
        };
    }

    fn move_towards(&mut self, x: i32, y: i32) -> bool {
        let diff_x = x - self.x;
        let diff_y = y - self.y;
        match (diff_x, diff_y) {
            (2, 0) => {
                self.x += 1;
                true
            }
            (-2, 0) => {
                self.x -= 1;
                true
            }
            (0, 2) => {
                self.y += 1;
                true
            }
            (0, -2) => {
                self.y -= 1;
                true
            }

            (2, 1) => {
                self.x += 1;
                self.y += 1;
                true
            }
            (2, -1) => {
                self.x += 1;
                self.y -= 1;
                true
            }
            (-2, 1) => {
                self.x -= 1;
                self.y += 1;
                true
            }
            (-2, -1) => {
                self.x -= 1;
                self.y -= 1;
                true
            }

            (1, 2) => {
                self.x += 1;
                self.y += 1;
                true
            }
            (-1, 2) => {
                self.x -= 1;
                self.y += 1;
                true
            }
            (1, -2) => {
                self.x += 1;
                self.y -= 1;
                true
            }
            (-1, -2) => {
                self.x -= 1;
                self.y -= 1;
                true
            }
            (2, 2) => {
                self.x += 1;
                self.y += 1;
                true
            }
            (2, -2) => {
                self.x += 1;
                self.y -= 1;
                true
            }
            (-2, -2) => {
                self.x -= 1;
                self.y -= 1;
                true
            }
            (-2, 2) => {
                self.x -= 1;
                self.y += 1;
                true
            }
            // Do nothing places
            (1, _) => false,
            (_, 1) => false,
            (-1, _) => false,
            (_, -1) => false,
            (0, 0) => false,
            (x, y) => {
                println!("\tToo far off diff:{},{}", x, y);
                false
            }
        }
    }
}
impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
struct Rope {
    knots: Vec<Point>,
    tail_visist: HashSet<Point>,
}

impl Rope {
    fn new(length: usize) -> Self {
        Rope {
            knots: vec![Point { x: 0, y: 0 }; length],
            tail_visist: HashSet::new(),
        }
    }
    fn tail_visit_count(self) -> usize {
        // Tail always visists 0,0
        self.tail_visist.len() + 1
    }
    fn run_command(&mut self, cmd: &Command) {
        for _ in 0..cmd.steps {
            for i in 0..self.knots.len() - 1 {
                let head = self.knots.get_mut(i).unwrap();

                // only first head moves all the time
                if i == 0 {
                    head.move_dir(&cmd.direction);
                }
                let head_x = head.x;
                let head_y = head.y;

                let tail = self.knots.get_mut(i + 1).unwrap();

                // move tail towards head
                let moved = tail.move_towards(head_x, head_y);

                if !moved {
                    // No need to check rest if we did not move
                    break;
                }
                // If we are the tail, add to visits
                if i == self.knots.len() - 2 {
                    let tail = self.knots.get(i + 1).unwrap();
                    self.tail_visist.insert(*tail);
                }
            }
        }
    }
}

pub fn run_part1(input: &Vec<String>) -> usize {
    let commands: Vec<Command> = input
        .iter()
        .map(|x| Command::from_str(x).unwrap())
        .collect();

    let mut rope = Rope::new(2);

    for c in commands.iter() {
        rope.run_command(&c);
    }

    rope.tail_visit_count()
}

pub fn run_part2(input: &Vec<String>) -> usize {
    let commands: Vec<Command> = input
        .iter()
        .map(|x| Command::from_str(x).unwrap())
        .collect();

    let mut rope = Rope::new(10);

    for c in commands.iter() {
        rope.run_command(&c);
    }

    rope.tail_visit_count()
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 9");

    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/day9.txt");

    // Part 1 goes here
    let p1 = run_part1(&lines);
    println!("Part1: {:?}", p1);

    // Part 2 Goes here
    let p2 = run_part2(&lines);
    println!("Part2: {:?}", p2);

    Ok(())
}
