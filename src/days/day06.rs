use crate::utils::structs::{Answer, Solver};
use itertools::Itertools;
use std::time::Instant;

pub struct Day;

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let mut signal = vec[0].split("").collect::<Vec<&str>>();
        signal.remove(0);

        for i in 3..signal.len() {
            let tmp = &signal[(i - 3)..(i + 1)];
            if tmp.iter().unique().collect::<Vec<&&str>>().len() == 4 {
                return Some(Answer::new((i + 1).to_string(), time.elapsed()));
            }
        }
        None
    }
    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let mut signal = vec[0].split("").collect::<Vec<&str>>();
        signal.remove(0);

        for i in 13..signal.len() {
            let tmp = &signal[(i - 13)..(i + 1)];
            if tmp.iter().unique().collect::<Vec<&&str>>().len() == 14 {
                return Some(Answer::new((i + 1).to_string(), time.elapsed()));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day06::*;
    use crate::utils::input;
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/day06.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "1134")
    }
    #[test]
    fn part2() {
        let vec = input::read_file("inputs/day06.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "2263")
    }
}
