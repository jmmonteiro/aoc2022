use aoc2022::days::*;
use aoc2022::{utils::input::read_file, utils::structs::Solver};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn benchmarks(c: &mut Criterion) {
    let vec = read_file("inputs/day01.txt");
    c.bench_function("day01_part1", |b| {
        b.iter(|| day01::Day.part1(black_box(&vec)))
    });
    c.bench_function("day01_part2", |b| {
        b.iter(|| day01::Day.part2(black_box(&vec)))
    });

    let vec = read_file("inputs/day02.txt");
    c.bench_function("day02_part1", |b| {
        b.iter(|| day02::Day.part1(black_box(&vec)))
    });
    c.bench_function("day02_part2", |b| {
        b.iter(|| day02::Day.part2(black_box(&vec)))
    });

    let vec = read_file("inputs/day03.txt");
    c.bench_function("day03_part1", |b| {
        b.iter(|| day03::Day.part1(black_box(&vec)))
    });
    c.bench_function("day03_part2", |b| {
        b.iter(|| day03::Day.part2(black_box(&vec)))
    });

    let vec = read_file("inputs/day04.txt");
    c.bench_function("day04_part1", |b| {
        b.iter(|| day04::Day.part1(black_box(&vec)))
    });
    c.bench_function("day04_part2", |b| {
        b.iter(|| day04::Day.part2(black_box(&vec)))
    });

    let vec = read_file("inputs/day05.txt");
    c.bench_function("day05_part1", |b| {
        b.iter(|| day05::Day.part1(black_box(&vec)))
    });
    c.bench_function("day05_part2", |b| {
        b.iter(|| day05::Day.part2(black_box(&vec)))
    });

    let vec = read_file("inputs/day06.txt");
    c.bench_function("day06_part1", |b| {
        b.iter(|| day06::Day.part1(black_box(&vec)))
    });
    c.bench_function("day06_part2", |b| {
        b.iter(|| day06::Day.part2(black_box(&vec)))
    });

    let vec = read_file("inputs/day07.txt");
    c.bench_function("day07_part1", |b| {
        b.iter(|| day07::Day.part1(black_box(&vec)))
    });
    c.bench_function("day07_part2", |b| {
        b.iter(|| day07::Day.part2(black_box(&vec)))
    });
}
criterion_group!(benches, benchmarks);
criterion_main!(benches);
