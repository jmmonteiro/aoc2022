mod days;
mod utils;
use std::time::Duration;

use crate::utils::structs::Solver;
fn main() {
    let mut total_time = Duration::new(0, 0);

    println!("\n----------------- Day 01 -----------------------");
    total_time += days::day01::Day.solve("inputs/day01.txt");
    println!("----------------- Day 02 -----------------------");
    total_time += days::day02::Day.solve("inputs/day02.txt");
    println!("----------------- Day 03 -----------------------");
    total_time += days::day03::Day.solve("inputs/day03.txt");

    println!(
        "\nTotal algorithm solve time (excludes data reading time): {} s\n",
        (total_time.as_nanos() as f32) / (i32::pow(10, 9) as f32)
    );
}
