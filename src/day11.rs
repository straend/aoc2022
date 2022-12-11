use itertools::Itertools;

use crate::helpers;
use num::integer::lcm;
use std::{
    collections::{HashMap, VecDeque},
    io,
    str::FromStr,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day11_test.txt");

        assert_eq!(run_part1(&lines), 10605);
    }
    #[test]
    fn test_part2() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day11_test.txt");

        assert_eq!(run_part2(&lines), 2713310158);
    }
}

#[derive(Debug)]
pub enum Instruction {
    Addition,
    Subtraction,
    Division,
    Multiplication,
}
#[derive(Debug)]
pub struct Operation {
    instruction: Instruction,
    param: usize,
    old: bool,
}
#[derive(Debug)]
pub struct TestOper {
    instruction: Instruction,
    param: usize,
    iftrue: usize,
    iffalse: usize,
}
#[derive(Debug)]
pub struct Monkey {
    id: usize,
    operation: Operation,
    test: TestOper,
    items: VecDeque<usize>,
    inspections: usize,
}
impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Instruction::Multiplication),
            "/" => Ok(Instruction::Division),
            "+" => Ok(Instruction::Addition),
            "-" => Ok(Instruction::Subtraction),
            "divisable" => Ok(Instruction::Division),
            "divisible" => Ok(Instruction::Division),

            _ => return Err(()),
        }
    }
}
impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, o) = s.split_once("old ").unwrap();
        let (oper, val) = o.split_once(" ").unwrap();
        let inst = Instruction::from_str(oper).unwrap();

        match val {
            "old" => Ok(Operation {
                instruction: inst,
                param: 0,
                old: true,
            }),
            x => match usize::from_str(x) {
                Ok(val) => Ok(Operation {
                    instruction: inst,
                    param: val,
                    old: false,
                }),
                _ => Err(()),
            }
        }
    }
}

impl<'a> FromIterator<&'a String> for TestOper {
    fn from_iter<I: IntoIterator<Item = &'a String>>(iter: I) -> Self {
        let mut op = TestOper {
            instruction: Instruction::Addition,
            iffalse: 0,
            iftrue: 0,
            param: 0,
        };
        for x in iter {
            //println!("{:?}", x);

            let xx = x.trim_start();
            if xx.starts_with("Test") {
                let (_, s) = xx.split_once(": ").unwrap();
                let (oper, param) = s.split_once(" by ").unwrap();
                let val = usize::from_str(param);
                let o = Instruction::from_str(oper);

                match (o, val) {
                    (Ok(ins), Ok(v)) => {
                        op.instruction = ins;
                        op.param = v;
                    }
                    (_, _) => {
                        println!("FAIL");
                    }
                };
            } else if xx.starts_with("If true") {
                let t = xx.split_ascii_whitespace().last().unwrap();
                let target = usize::from_str(t).unwrap();
                op.iftrue = target;
            } else if xx.starts_with("If false") {
                let t = xx.split_ascii_whitespace().last().unwrap();
                let target = usize::from_str(t).unwrap();
                op.iffalse = target;
            }
        }
        op
    }
}
impl<'a> FromIterator<&'a String> for Monkey {
    fn from_iter<I: IntoIterator<Item = &'a String>>(iter: I) -> Self {
        let mut m = Monkey {
            id: 0,
            operation: Operation {
                instruction: Instruction::Addition,
                param: 0,
                old: false,
            },
            test: TestOper {
                instruction: Instruction::Addition,
                iffalse: 0,
                iftrue: 0,
                param: 0,
            },
            items: VecDeque::new(),
            inspections: 0,
        };
        for x2 in iter {
            let x = x2.trim();
            if x.starts_with("Monkey") {
                let xx = x.rsplit_once(" ").unwrap();
                let id = usize::from_str(xx.1.strip_suffix(":").unwrap()).unwrap();
                m.id = id;
            } else if x.starts_with("Starting items") {
                let (_, its) = x.rsplit_once(": ").unwrap();
                let c: VecDeque<usize> = its
                    .split(",")
                    .map(|x| usize::from_str(x.trim()).unwrap())
                    .collect();
                m.items = c;
            } else if x.starts_with("Operation:") {
                let op = Operation::from_str(x).unwrap();
                m.operation = op;
            } else if x.starts_with("Test:") {
                let op = TestOper::from_str(x).unwrap();
                m.test = op;
            } else if x.starts_with("If true:") {
                let t = x.split_ascii_whitespace().last().unwrap();
                let target = usize::from_str(t).unwrap();
                m.test.iftrue = target;
            } else if x.starts_with("If false") {
                let t = x.split_ascii_whitespace().last().unwrap();
                let target = usize::from_str(t).unwrap();
                m.test.iffalse = target;
            }
        }
        m
    }
}
impl FromStr for TestOper {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s.split_once(": ").unwrap();
        let (oper, param) = s.split_once(" by ").unwrap();
        let val = usize::from_str(param);
        let o = Instruction::from_str(oper);
        match (o, val) {
            (Ok(op), Ok(v)) => Ok(TestOper {
                instruction: op,
                param: v,
                iffalse: 0,
                iftrue: 0,
            }),
            _ => Err(()),
        }
    }
}
trait MonkeyMap {
    fn one_round(&mut self, modulor: usize);
    fn monkey_business(self) -> usize;
    fn get_modulor(&self) -> usize;
}

impl MonkeyMap for HashMap<usize, Monkey> {
    fn one_round(&mut self, modulor: usize) {
        for i in 0..self.len() {
            let m = self.get_mut(&i).unwrap();
            let items = m.items.len();
            m.inspections += items;
            for item in m.items.iter_mut() {
                *item = match (&m.operation.instruction, &m.operation.old) {
                    (Instruction::Addition, false) => *item + m.operation.param,
                    (Instruction::Addition, true) => *item + *item,
                    (Instruction::Subtraction, false) => *item - m.operation.param,
                    (Instruction::Subtraction, true) => *item - *item,
                    (Instruction::Multiplication, false) => *item * m.operation.param,
                    (Instruction::Multiplication, true) => *item * *item,
                    (Instruction::Division, false) => *item / m.operation.param,
                    (Instruction::Division, true) => *item / *item,
                };

                // Monkey inspected item, and item is still whole, reduce worryingfactor
                if modulor == 0 {
                    *item = *item / 3;
                } else {
                    *item = *item % modulor;
                }
            }
            let throw = (m.test.iftrue, m.test.iffalse);
            let divby = m.test.param;
            
            for _ in 0..items {
                // check item
                let it = self.get_mut(&i).unwrap().items.pop_front().unwrap();
                let throw_to = if it % divby == 0 { throw.0 } else { throw.1 };
                self.get_mut(&throw_to).unwrap().items.push_back(it);
            }
        }
    }

    fn monkey_business(self) -> usize {
        let mut iters: Vec<usize> = self.iter().map(|(_, v)| v.inspections).collect();
        iters.sort();
        iters.reverse();

        if iters.len() > 1 {
            return iters[0] * iters[1];
        }
        0
    }
    fn get_modulor(&self) -> usize {
        let divis: Vec<usize> = self.iter().map(|(_, v)| v.test.param).collect();
        let mut cdiv = 1;
        for d in divis.iter() {
            cdiv = lcm(cdiv, *d);
        }
        cdiv
    }
}

fn read_input(input: &Vec<String>) -> HashMap<usize, Monkey> {
    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();

    for m_iter in input.iter().chunks(7).into_iter() {
        let tot = m_iter.take(6);
        let m = Monkey::from_iter(tot);
        monkeys.insert(m.id, m);
    }

    monkeys
}

pub fn run_part1(input: &Vec<String>) -> usize {
    let mut monkeys = read_input(input);
    for _ in 0..20 {
        monkeys.one_round(0);
    }

    monkeys.monkey_business()
}

pub fn run_part2(input: &Vec<String>) -> usize {
    let mut monkeys = read_input(input);

    let modulor = monkeys.get_modulor();

    for _ in 0..10000 {
        monkeys.one_round(modulor);
    }
    monkeys.monkey_business()
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 11");

    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/day11.txt");

    // Part 1 goes here
    let p1 = run_part1(&lines);
    println!("Part1: {:?}", p1);

    // Part 2 Goes here
    let p2 = run_part2(&lines);
    println!("Part2: {:?}", p2);

    Ok(())
}
