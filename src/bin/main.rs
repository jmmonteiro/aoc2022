use aoc2022::days;
use std::time::{Duration, Instant};

use aoc2022::utils::structs::Solver;
fn main() {
    let time = Instant::now();
    let mut total_algo_time = Duration::new(0, 0);

    println!("");
    total_algo_time += days::day01::Day.solve("inputs/day01.txt");
    total_algo_time += days::day02::Day.solve("inputs/day02.txt");
    total_algo_time += days::day03::Day.solve("inputs/day03.txt");
    total_algo_time += days::day04::Day.solve("inputs/day04.txt");
    total_algo_time += days::day05::Day.solve("inputs/day05.txt");
    total_algo_time += days::day06::Day.solve("inputs/day06.txt");
    total_algo_time += days::day07::Day.solve("inputs/day07.txt");
    total_algo_time += days::day08::Day.solve("inputs/day08.txt");

    println!(
        "Total algorithm solve time (excludes data reading time): {} s",
        (total_algo_time.as_nanos() as f32) / (i32::pow(10, 9) as f32)
    );
    println!(
        "Total runtime: {} s\n",
        (time.elapsed().as_nanos() as f32) / (i32::pow(10, 9) as f32)
    );
}
