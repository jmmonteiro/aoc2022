use aoc2022::days::*;
use aoc2022::{utils::input::read_file, utils::structs::Solver};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn all_days_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("All Days");

    let vec = read_file("inputs/day01.txt");
    group.bench_function("Day 1 : Part 1", |b| {
        b.iter(|| day01::Day.part1(black_box(&vec)))
    });
    group.bench_function("Day 1 : Part 2", |b| {
        b.iter(|| day01::Day.part2(black_box(&vec)))
    });

    let vec = read_file("inputs/day02.txt");
    group.bench_function("Day 2 : Part 1", |b| {
        b.iter(|| day02::Day.part1(black_box(&vec)))
    });
    group.bench_function("Day 2 : Part 2", |b| {
        b.iter(|| day02::Day.part2(black_box(&vec)))
    });

    let vec = read_file("inputs/day03.txt");
    group.bench_function("Day 3 : Part 1", |b| {
        b.iter(|| day03::Day.part1(black_box(&vec)))
    });
    group.bench_function("Day 3 : Part 2", |b| {
        b.iter(|| day03::Day.part2(black_box(&vec)))
    });

    let vec = read_file("inputs/day04.txt");
    group.bench_function("Day 4 : Part 1", |b| {
        b.iter(|| day04::Day.part1(black_box(&vec)))
    });
    group.bench_function("Day 4 : Part 2", |b| {
        b.iter(|| day04::Day.part2(black_box(&vec)))
    });

    let vec = read_file("inputs/day05.txt");
    group.bench_function("Day 5 : Part 1", |b| {
        b.iter(|| day05::Day.part1(black_box(&vec)))
    });
    group.bench_function("Day 5 : Part 2", |b| {
        b.iter(|| day05::Day.part2(black_box(&vec)))
    });

    let vec = read_file("inputs/day06.txt");
    group.bench_function("Day 6 : Part 1", |b| {
        b.iter(|| day06::Day.part1(black_box(&vec)))
    });
    group.bench_function("Day 6 : Part 2", |b| {
        b.iter(|| day06::Day.part2(black_box(&vec)))
    });

    let vec = read_file("inputs/day07.txt");
    group.bench_function("Day 7 : Part 1", |b| {
        b.iter(|| day07::Day.part1(black_box(&vec)))
    });
    group.bench_function("Day 7 : Part 2", |b| {
        b.iter(|| day07::Day.part2(black_box(&vec)))
    });

    let vec = read_file("inputs/day08.txt");
    group.bench_function("Day 8 : Part 1", |b| {
        b.iter(|| day08::Day.part1(black_box(&vec)))
    });
    group.bench_function("Day 8 : Part 2", |b| {
        b.iter(|| day08::Day.part2(black_box(&vec)))
    });

    let vec = read_file("inputs/day09.txt");
    group.bench_function("Day 9 : Part 1", |b| {
        b.iter(|| day09::Day.part1(black_box(&vec)))
    });
    group.bench_function("Day 9 : Part 2", |b| {
        b.iter(|| day09::Day.part2(black_box(&vec)))
    });

    let vec = read_file("inputs/day10.txt");
    group.bench_function("Day 10 : Part 1", |b| {
        b.iter(|| day10::Day.part1(black_box(&vec)))
    });
    group.bench_function("Day 10 : Part 2", |b| {
        b.iter(|| day10::Day.part2(black_box(&vec)))
    });

    group.finish();
}
criterion_group! {
    name = all_days;
    config = Criterion::default().significance_level(0.01).sample_size(500);
    targets = all_days_benchmark
}

// --- Alternative implementations
pub fn day6_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 6 - Comparissons");

    let vec = read_file("inputs/day06.txt");
    group.bench_function("Part 1", |b| b.iter(|| day06::Day.part1(black_box(&vec))));
    group.bench_function("Part 2", |b| b.iter(|| day01::Day.part2(black_box(&vec))));

    group.bench_function("Part 1 (using collect)", |b| {
        b.iter(|| day06::old_implementation_collect::part1(black_box(&vec)))
    });
    group.bench_function("Part 2 (using collect)", |b| {
        b.iter(|| day06::old_implementation_collect::part2(black_box(&vec)))
    });

    group.finish();
}

criterion_group! {
    name = day6;
    config = Criterion::default().significance_level(0.01).sample_size(500);
    targets = day6_benchmark
}

criterion_main!(all_days, day6);
