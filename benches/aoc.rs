use criterion::{criterion_group, criterion_main};

mod day2;
mod day3;
// Add days here

criterion_group!(day3, day3::bench_day3_p1, day3::bench_day3_p2);
criterion_group!(day2, day2::bench_day2_p1, day2::bench_day2_p2);
// Add day group here

// Do not remove from last line
criterion_main!(day2, day3);
