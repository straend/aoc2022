use crate::helpers;
use std::{io, str::FromStr};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day10_test.txt");

        assert_eq!(run_part1(&lines), 13140);
    }
    #[test]
    fn test_part2() {}
}

struct CPU {
    pc: usize,
    reg_x: i32,
    addx_delay: [i32; 3],
    program: Option<Vec<Instruction>>,
    id_x: usize,
    program_len: usize,
    signal_strengths: Vec<usize>,
}

#[derive(Debug)]
enum InstructionE {
    Noop,
    Addx,
}
#[derive(Debug)]
struct Instruction {
    cmd: InstructionE,
    param: i32,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();
        let opcode = split.first().unwrap();
        let mut p1 = 0;
        if split.len() > 1 {
            p1 = i32::from_str_radix(split.get(1).unwrap(), 10).unwrap();
        }
        match opcode {
            &"noop" => Ok(Instruction {
                cmd: InstructionE::Noop,
                param: 0,
            }),
            &"addx" => Ok(Instruction {
                cmd: InstructionE::Addx,
                param: p1,
            }),
            _ => Err(()),
        }
    }
}

impl CPU {
    fn new() -> Self {
        CPU {
            pc: 0,
            reg_x: 1,
            addx_delay: [0; 3],
            program: None,
            id_x: 0,
            program_len: 0,
            signal_strengths: Vec::new(),
        }
    }

    fn load_program(&mut self, program: Vec<Instruction>) {
        self.program_len = program.iter().count();
        self.id_x = 0;
        self.pc = 0;
        self.reg_x = 1;

        self.program = Some(program);
    }
    fn trace(&self, pc: usize) {
        if pc % 40 == 0 {
            println!("");
        }
        let c = self.reg_x - (pc % 40) as i32;
        let p = match c {
            -1..=1 => '#',
            _ => ' ',
        };
        print!("{}", p);
    }
    fn run(&mut self, breakpoints: &Vec<usize>, has_gpu: bool) {
        let p: &Vec<Instruction>;

        match &self.program {
            None => {
                return;
            }
            Some(x) => p = x,
        }
        let mut pc = 0;

        for _ in 0..self.program_len + 1 {
            if breakpoints.contains(&pc) {
                self.signal_strengths.push(pc * self.reg_x as usize);
            }
            self.reg_x += self.addx_delay[0];
            self.addx_delay[0] = self.addx_delay[1];
            self.addx_delay[1] = self.addx_delay[2];
            self.addx_delay[2] = 0;
            if has_gpu {
                self.trace(pc);
            }

            // During cycle
            match p.get(self.id_x) {
                Some(cmd) => {
                    self.id_x += 1;
                    match cmd.cmd {
                        InstructionE::Noop => {}
                        InstructionE::Addx => {
                            self.addx_delay[2] = cmd.param;
                            pc += 1;
                            self.reg_x += self.addx_delay[0];
                            self.addx_delay[0] = self.addx_delay[1];
                            self.addx_delay[1] = self.addx_delay[2];
                            self.addx_delay[2] = 0;

                            if breakpoints.contains(&pc) {
                                self.signal_strengths.push(pc * self.reg_x as usize);
                            }

                            if has_gpu {
                                self.trace(pc);
                            }
                        }
                    };
                }
                _ => {
                    // No instruction just keep going
                }
            }
            pc += 1;
            self.reg_x += self.addx_delay[0];
            self.addx_delay[0] = self.addx_delay[1];
            self.addx_delay[1] = self.addx_delay[2];
            self.addx_delay[2] = 0;
        }
    }
}

fn load_rogram_from(input: &Vec<String>) -> Vec<Instruction> {
    input
        .iter()
        .map(|l| Instruction::from_str(l).unwrap())
        .collect()
}
pub fn run_part1(input: &Vec<String>) -> usize {
    let mut cpu = CPU::new();
    let program = load_rogram_from(input);

    cpu.load_program(program);
    let bkps: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    cpu.run(&bkps, false);
    cpu.signal_strengths.iter().sum()
}

pub fn run_part2(input: &Vec<String>) -> i128 {
    let mut cpu = CPU::new();
    let program = load_rogram_from(input);

    cpu.load_program(program);
    let bkps: Vec<usize> = Vec::new();

    cpu.run(&bkps, true);

    200
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 10");

    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/day10.txt");

    // Part 1 goes here
    let p1 = run_part1(&lines);
    println!("Part1: {:?}", p1);

    // Part 2 Goes here
    println!("Part2");
    run_part2(&lines);

    Ok(())
}
