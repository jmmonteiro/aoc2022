use crate::utils::{input, structs::Answer};
use std::collections::BTreeSet;
use std::time::{Duration, Instant};
pub fn solve() -> Duration {
    let vec = input::read_file("inputs/day01.txt");
    let mut total_time = Duration::new(0, 0);
    match part1(&vec) {
        Some(i) => {
            total_time += i.time;
            i.display();
        }
        None => println!("No answer found for part 1."),
    };
    match part2(&vec) {
        Some(i) => {
            total_time += i.time;
            i.display();
        }
        None => println!("No answer found for part 2."),
    };
    total_time
}

fn part1(vec: &Vec<String>) -> Option<Answer> {
    let time = Instant::now();

    let mut current_calories = 0;
    let mut max_calories = -1;

    for l in vec {
        match l.parse::<i32>() {
            Ok(ok) => current_calories += ok,
            Err(_) => {
                if current_calories > max_calories {
                    max_calories = current_calories;
                }
                current_calories = 0;
            }
        }
    }

    return Some(Answer::new(
        "1.1",
        (max_calories).to_string(),
        time.elapsed(),
    ));
}

fn part2(vec: &Vec<String>) -> Option<Answer> {
    let time = Instant::now();

    let mut current_calories: i32 = 0;
    let mut max_calories: BTreeSet<i32> = BTreeSet::from([-1, -2, -3]);

    for l in vec {
        match l.parse::<i32>() {
            Ok(ok) => current_calories += ok,
            Err(_) => {
                // max_calories is an ordered set
                let min = max_calories.iter().next().unwrap().clone();
                if min < current_calories {
                    max_calories.remove(&min);
                    max_calories.insert(current_calories);
                }
                current_calories = 0;
            }
        }
    }

    return Some(Answer::new(
        "1.2",
        (max_calories.iter().sum::<i32>()).to_string(),
        time.elapsed(),
    ));
}
