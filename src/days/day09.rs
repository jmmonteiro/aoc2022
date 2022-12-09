use crate::utils::structs::{Answer, Solver};
use std::{collections::HashSet, time::Instant};

struct Rope {
    segments: Vec<(i32, i32)>,
    visited_pos: HashSet<(i32, i32)>,
}

impl Rope {
    pub fn new(length: usize) -> Self {
        Rope {
            segments: vec![(0, 0); length],
            visited_pos: HashSet::from([(0, 0)]),
        }
    }
    pub fn move_head(&mut self, direction: &str) {
        match direction {
            "R" => self.segments[0].0 += 1,
            "L" => self.segments[0].0 -= 1,
            "U" => self.segments[0].1 += 1,
            "D" => self.segments[0].1 -= 1,
            _ => {
                panic!("{} is not a valid direction.", direction)
            }
        }
        self.move_tail();
    }
    fn move_tail(&mut self) {
        for i in 1..self.segments.len() {
            let dx = self.segments[i - 1].0 - self.segments[i].0;
            let dy = self.segments[i - 1].1 - self.segments[i].1;

            if dx.abs() > 1 || dy.abs() > 1 {
                if dx != 0 {
                    self.segments[i].0 += dx / dx.abs();
                }
                if dy != 0 {
                    self.segments[i].1 += dy / dy.abs();
                }
                if i == self.segments.len() - 1 {
                    self.visited_pos.insert(self.segments[i]);
                }
            } else {
                break; // no point in testing the rest of the rope, no movement was made
            }
        }
    }
}

pub struct Day;

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let mut rope = Rope::new(2);
        for l in vec {
            let split: Vec<&str> = l.split(" ").collect();
            let direction = split[0];
            let rep = split[1].parse::<usize>().unwrap();
            for _ in 0..rep {
                rope.move_head(direction)
            }
        }

        return Some(Answer::new(
            (rope.visited_pos.len()).to_string(),
            time.elapsed(),
        ));
    }

    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let mut rope = Rope::new(10);
        for l in vec {
            let split: Vec<&str> = l.split(" ").collect();
            let direction = split[0];
            let rep = split[1].parse::<usize>().unwrap();
            for _ in 0..rep {
                rope.move_head(direction)
            }
        }

        return Some(Answer::new(
            (rope.visited_pos.len()).to_string(),
            time.elapsed(),
        ));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day09::*;
    use crate::utils::input;
    #[test]
    fn unit_test_part1() {
        let vec = input::read_file("inputs/day09_test.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "13")
    }
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/day09.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "6190")
    }
    #[test]
    fn part2() {
        let vec = input::read_file("inputs/day09.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "2516")
    }
}
