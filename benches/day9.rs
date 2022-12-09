use criterion::{black_box, Criterion};

use aoc2022::*;

pub fn bench_day9_p1(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/day9.txt");
    c.bench_function("Day9 Part 1", |b| {
        b.iter(|| day9::run_part1(black_box(&lines)))
    });
}
pub fn bench_day9_p2(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/day9.txt");
    c.bench_function("Day9 Part 2", |b| {
        b.iter(|| day9::run_part2(black_box(&lines)))
    });
}
