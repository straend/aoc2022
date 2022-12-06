use criterion::{black_box, Criterion, BenchmarkId};

use aoc2022::*;

pub fn bench_day6_p1(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/day6.txt");
    c.bench_function("Day6 Part 1", |b| {
        b.iter(|| day6::run_part1(black_box(&lines.first().unwrap())))
    });
}
pub fn bench_day6_p2(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/day6.txt");

    let mut group = c.benchmark_group("Day6 Part 2");
    let i = lines.first().unwrap();
    group.bench_with_input(BenchmarkId::new("Homemade", "HashMaps"), i, 
            |b, i| b.iter(|| day6::run_part2(i)));
    group.bench_with_input(BenchmarkId::new("Itertools", "Unique"), i, 
            |b, i| b.iter(|| day6::run_part2_itertools(i)));
            

    group.finish();
}
