mod days;
mod utils;
use std::time::Duration;
fn main() {
    let mut total_time = Duration::new(0, 0);

    println!("--- Day 01 ---");
    total_time += days::day01::solve();
    println!("--- Day 02 ---");
    total_time += days::day02::solve();

    println!(
        "Total solve time: {} s",
        (total_time.as_nanos() as f32) / (i32::pow(10, 9) as f32)
    )
}
