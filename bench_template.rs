use criterion::{black_box, Criterion};

use aoc2022::*;

pub fn bench_dayDAY_NUMBER_p1(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/dayDAY_NUMBER.txt");
    c.bench_function("DayDAY_NUMBER Part 1", |b| b.iter(|| dayDAY_NUMBER::run_part1(black_box(&lines))));
}
pub fn bench_dayDAY_NUMBER_p2(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/dayDAY_NUMBER.txt");
    c.bench_function("DayDAY_NUMBER Part 2", |b| b.iter(|| dayDAY_NUMBER::run_part2(black_box(&lines))));
}
