mod days;
mod utils;
use std::time::Duration;

use crate::utils::structs::Solver;
fn main() {
    let mut total_time = Duration::new(0, 0);

    println!("");
    total_time += days::day01::Day.solve("inputs/day01.txt");
    total_time += days::day02::Day.solve("inputs/day02.txt");
    total_time += days::day03::Day.solve("inputs/day03.txt");
    total_time += days::day04::Day.solve("inputs/day04.txt");

    println!(
        "Total algorithm solve time (excludes data reading time): {} s\n",
        (total_time.as_nanos() as f32) / (i32::pow(10, 9) as f32)
    );
}
