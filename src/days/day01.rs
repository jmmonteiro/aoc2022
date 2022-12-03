use crate::utils::structs::{Answer, Solver};
use std::collections::BTreeSet;
use std::time::Instant;

pub struct Day;

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
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

        return Some(Answer::new((max_calories).to_string(), time.elapsed()));
    }

    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
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
            (max_calories.iter().sum::<i32>()).to_string(),
            time.elapsed(),
        ));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day01::*;
    use crate::utils::input;
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/day01.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "72511")
    }
    #[test]
    fn part2() {
        let vec = input::read_file("inputs/day01.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "212117")
    }
}
