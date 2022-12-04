use regex::Regex;

use crate::utils::structs::{Answer, Solver};
use std::time::Instant;

pub struct Day;

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
        let mut count: u32 = 0;
        for l in vec {
            let cap = re.captures(l).unwrap();

            let (min_elf1, max_elf1) = (
                &cap[1].parse::<u32>().unwrap(),
                &cap[2].parse::<u32>().unwrap(),
            );
            let (min_elf2, max_elf2) = (
                &cap[3].parse::<u32>().unwrap(),
                &cap[4].parse::<u32>().unwrap(),
            );

            if ((min_elf1 <= min_elf2) && (max_elf1 >= max_elf2))
                || ((min_elf2 <= min_elf1) && (max_elf2 >= max_elf1))
            {
                count += 1;
            }
        }

        return Some(Answer::new((count).to_string(), time.elapsed()));
    }

    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
        let mut count: u32 = 0;
        for l in vec {
            let cap = re.captures(l).unwrap();

            let (min_elf1, max_elf1) = (
                &cap[1].parse::<u32>().unwrap(),
                &cap[2].parse::<u32>().unwrap(),
            );
            let (min_elf2, max_elf2) = (
                &cap[3].parse::<u32>().unwrap(),
                &cap[4].parse::<u32>().unwrap(),
            );

            if ((min_elf1 <= min_elf2) && (max_elf1 >= min_elf2))
                || ((min_elf2 <= min_elf1) && (max_elf2 >= min_elf1))
            {
                count += 1;
            }
        }

        return Some(Answer::new((count).to_string(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day04::*;
    use crate::utils::input;
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/day04.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "547")
    }
    #[test]
    fn part2() {
        let vec = input::read_file("inputs/day04.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "843")
    }
}
