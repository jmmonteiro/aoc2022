use crate::utils::structs::{Answer, Solver};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub struct Day;

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let score_map = ("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .enumerate())
        .map(|(i, c)| (c, i + 1))
        .collect::<HashMap<char, usize>>();

        let mut total_score: u32 = 0;
        for l in vec {
            let size = l.len() / 2;

            let mut first: HashSet<char> = HashSet::new();
            for i in 0..size {
                first.insert(l.as_bytes()[i] as char);
            }
            for i in size..l.len() {
                let letter = l.as_bytes()[i] as char;
                if first.contains(&letter) {
                    total_score += score_map[&letter] as u32;
                    break;
                }
            }
        }

        return Some(Answer::new((total_score).to_string(), time.elapsed()));
    }

    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let score_map = ("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .enumerate())
        .map(|(i, c)| (c, i + 1))
        .collect::<HashMap<char, usize>>();

        let mut total_score: u32 = 0;

        for group in 0..vec.len() / 3 {
            let elf1: HashSet<char> = vec[3 * group].chars().collect();
            let elf2: HashSet<char> = vec[3 * group + 1].chars().collect();
            let elf3: HashSet<char> = vec[3 * group + 2].chars().collect();

            let common_item = elf1
                .intersection(&elf2)
                .map(|i| *i)
                .collect::<HashSet<char>>()
                .intersection(&elf3)
                .map(|i| *i)
                .collect::<Vec<char>>();
            assert!(common_item.len() == 1);
            total_score += score_map[&common_item[0]] as u32;
        }

        return Some(Answer::new((total_score).to_string(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day03::*;
    use crate::utils::input;
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/day03.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "8394")
    }
    #[test]
    fn part2() {
        let vec = input::read_file("inputs/day03.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "2413")
    }
}
