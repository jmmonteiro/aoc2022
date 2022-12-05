use crate::utils::structs::{Answer, Solver};
use std::{collections::HashMap, time::Instant};

use regex::Regex;

pub struct Day;

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();
        let re_pos = Regex::new(r"\s{2,}(\d+)\s{2,}").unwrap();
        let re_moves = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
        let mut idx_pos: usize = 0;
        let mut idx_moves: usize = 0;

        // Get positions of different sections of the input (to make this solution more general)
        for (i, l) in vec.iter().enumerate() {
            if re_pos.is_match(l) {
                idx_pos = i;
            } else if re_moves.is_match(l) {
                idx_moves = i;
                break;
            }
        }

        let crates = &vec[0..idx_pos];
        let moves = &vec[idx_moves..vec.len()];

        let mut map_positions: HashMap<usize, usize> = HashMap::new(); // <position in string, column number>
        let caps = Regex::new(r"(\d+)").unwrap();
        let caps = caps.find_iter(&vec[idx_pos]);
        for (i, c) in caps.enumerate() {
            map_positions.insert(c.start(), i + 1);
        }

        // Read in initial statelet tmp =
        let mut crates_hashmap: HashMap<usize, Vec<char>> = HashMap::new(); // column number , vector of crates
        for i in 1..(map_positions.len() + 1) {
            crates_hashmap.insert(i, Vec::new());
        }
        for l in crates.iter().rev() {
            for (pos, col) in map_positions.iter() {
                let tmp = l.chars().nth(*pos).unwrap();
                if tmp != ' ' {
                    crates_hashmap.get_mut(col).unwrap().push(tmp);
                }
            }
        }

        // Start moving crates
        for l in moves {
            let caps = re_moves.captures(&l).unwrap();
            for _m in 0..caps[1].parse::<usize>().unwrap() {
                let transfer = crates_hashmap
                    .get_mut(&caps[2].parse::<usize>().unwrap())
                    .unwrap()
                    .pop()
                    .unwrap();
                crates_hashmap
                    .get_mut(&caps[3].parse::<usize>().unwrap())
                    .unwrap()
                    .push(transfer);
            }
        }

        // Format result
        let mut output = String::new();
        for idx in 1..(map_positions.len() + 1) {
            output.push(
                crates_hashmap
                    .get_mut(&(idx as usize))
                    .unwrap()
                    .pop()
                    .unwrap(),
            )
        }

        return Some(Answer::new(output, time.elapsed()));
    }

    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();
        let re_pos = Regex::new(r"\s{2,}(\d+)\s{2,}").unwrap();
        let re_moves = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
        let mut idx_pos: usize = 0;
        let mut idx_moves: usize = 0;

        // Get positions of different sections of the input (to make this solution more general)
        for (i, l) in vec.iter().enumerate() {
            if re_pos.is_match(l) {
                idx_pos = i;
            } else if re_moves.is_match(l) {
                idx_moves = i;
                break;
            }
        }

        let crates = &vec[0..idx_pos];
        let moves = &vec[idx_moves..vec.len()];

        let mut map_positions: HashMap<usize, usize> = HashMap::new(); // <position in string, column number>
        let caps = Regex::new(r"(\d+)").unwrap();
        let caps = caps.find_iter(&vec[idx_pos]);
        for (i, c) in caps.enumerate() {
            map_positions.insert(c.start(), i + 1);
        }

        // Read in initial statelet tmp =
        let mut crates_hashmap: HashMap<usize, Vec<char>> = HashMap::new(); // column number , vector of crates
        for i in 1..(map_positions.len() + 1) {
            crates_hashmap.insert(i, Vec::new());
        }
        for l in crates.iter().rev() {
            for (pos, col) in map_positions.iter() {
                let tmp = l.chars().nth(*pos).unwrap();
                if tmp != ' ' {
                    crates_hashmap.get_mut(col).unwrap().push(tmp);
                }
            }
        }

        // Start moving crates
        for l in moves {
            let caps = re_moves.captures(&l).unwrap();
            let from_col = crates_hashmap
                .get_mut(&caps[2].parse::<usize>().unwrap())
                .unwrap();

            let mut transfer: Vec<char> = Vec::new();

            for _m in 0..caps[1].parse::<usize>().unwrap() {
                transfer.push(from_col.pop().unwrap());
            }
            transfer.reverse();

            crates_hashmap
                .get_mut(&caps[3].parse::<usize>().unwrap())
                .unwrap()
                .extend(transfer);
        }

        // Format result
        let mut output = String::new();
        for idx in 1..(map_positions.len() + 1) {
            output.push(
                crates_hashmap
                    .get_mut(&(idx as usize))
                    .unwrap()
                    .pop()
                    .unwrap(),
            )
        }

        return Some(Answer::new(output, time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day05::*;
    use crate::utils::input;
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/day05.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "BSDMQFLSP")
    }
    #[test]
    fn part2() {
        let vec = input::read_file("inputs/day05.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "PGSQBFLDP")
    }
}
