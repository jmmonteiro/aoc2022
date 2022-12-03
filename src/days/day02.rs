use crate::utils::structs::{Answer, Solver};
use std::time::Instant;

pub struct Day;

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();
        fn map(mine: char) -> char {
            match mine {
                'X' => return 'A', //rock
                'Y' => return 'B', //paper
                'Z' => return 'C', //scissors
                _ => panic!("{} is not a valid move for the second column.", mine),
            }
        }
        let mut total_score: u32 = 0;
        for l in vec {
            let opponent = l.as_bytes()[0] as char;
            let mine = map(l.as_bytes()[2] as char);

            match mine {
                'A' => total_score += 1,
                'B' => total_score += 2,
                'C' => total_score += 3,
                _ => {}
            }

            if opponent == mine {
                total_score += 3;
                continue;
            }

            match opponent {
                'A' => {
                    if mine == 'B' {
                        total_score += 6;
                    }
                }
                'B' => {
                    if mine == 'C' {
                        total_score += 6;
                    }
                }
                'C' => {
                    if mine == 'A' {
                        total_score += 6;
                    }
                }
                _ => panic!("{} is not a valid move for the first column.", mine),
            }
        }
        return Some(Answer::new((total_score).to_string(), time.elapsed()));
    }

    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();
        let mut total_score: u32 = 0;
        for l in vec {
            let opponent = l.as_bytes()[0] as char;
            let mut mine = l.as_bytes()[2] as char;

            match mine {
                // draw
                'Y' => {
                    mine = opponent;
                    total_score += 3
                }
                //win
                'Z' => match opponent {
                    'A' => {
                        mine = 'B';
                        total_score += 6
                    }
                    'B' => {
                        mine = 'C';
                        total_score += 6
                    }
                    'C' => {
                        mine = 'A';
                        total_score += 6
                    }
                    _ => panic!("{} is not a valid move for the first column.", opponent),
                },
                //lose
                'X' => match opponent {
                    'A' => mine = 'C',
                    'B' => mine = 'A',
                    'C' => mine = 'B',
                    _ => panic!("{} is not a valid move for the first column.", opponent),
                },

                _ => panic!("{} is not a valid move for the second column.", mine),
            }

            match mine {
                'A' => total_score += 1,
                'B' => total_score += 2,
                'C' => total_score += 3,
                _ => {}
            }
        }
        return Some(Answer::new((total_score).to_string(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day02::*;
    use crate::utils::input;
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/day02.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "15572")
    }
    #[test]
    fn part2() {
        let vec = input::read_file("inputs/day02.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "16098")
    }
}
