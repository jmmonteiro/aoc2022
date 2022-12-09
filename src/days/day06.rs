use crate::utils::structs::{Answer, Solver};
use itertools::Itertools;
use std::time::Instant;

fn slide_window(vec: &Vec<String>, size: usize, time: Instant) -> Option<Answer> {
    let mut signal = vec[0].split("").collect::<Vec<&str>>();
    signal.remove(0);

    for i in (size - 1)..signal.len() {
        let tmp = &signal[(i - (size - 1))..(i + 1)];
        if tmp.iter().unique().count() == size {
            return Some(Answer::new((i + 1).to_string(), time.elapsed()));
        }
    }
    None
}
pub struct Day;

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();
        slide_window(vec, 4, time)
    }

    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();
        slide_window(vec, 14, time)
    }
}

pub mod old_implementation_collect {
    use std::time::Instant;

    use itertools::Itertools;

    use crate::utils::structs::Answer;

    fn slide_window(vec: &Vec<String>, size: usize, time: Instant) -> Option<Answer> {
        let mut signal = vec[0].split("").collect::<Vec<&str>>();
        signal.remove(0);

        for i in (size - 1)..signal.len() {
            let tmp = &signal[(i - (size - 1))..(i + 1)];
            if tmp.iter().unique().collect::<Vec<&&str>>().len() == size {
                return Some(Answer::new((i + 1).to_string(), time.elapsed()));
            }
        }
        None
    }

    pub fn part1(vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();
        slide_window(vec, 4, time)
    }

    pub fn part2(vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();
        slide_window(vec, 14, time)
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
