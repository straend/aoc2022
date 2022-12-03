use criterion::{black_box, Criterion};

use aoc2022::*;

pub fn bench_day3_p1(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/day3.txt");
    c.bench_function("Day3 Part 1", |b| {
        b.iter(|| day3::run_part1(black_box(&lines)))
    });
}
pub fn bench_day3_p2(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/day3.txt");
    c.bench_function("Day3 Part 2", |b| {
        b.iter(|| day3::run_part2(black_box(&lines)))
    });
}
