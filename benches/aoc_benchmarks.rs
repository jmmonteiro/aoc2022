use aoc2022::days::*;
use aoc2022::{utils::input::read_file, utils::structs::Solver};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day01_benchmark(c: &mut Criterion) {
    let vec = read_file("inputs/day01.txt");
    c.bench_function("day01_part1", |b| {
        b.iter(|| day01::Day.part1(black_box(&vec)))
    });
    c.bench_function("day01_part2", |b| {
        b.iter(|| day01::Day.part2(black_box(&vec)))
    });
}
criterion_group!(benches, day01_benchmark);
criterion_main!(benches);
