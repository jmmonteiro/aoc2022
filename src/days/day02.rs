use crate::utils::structs::Answer;
use std::time::{Duration, Instant};
pub fn solve() -> Duration {
    let vec = include_str!("../../inputs/day02.txt")
        .lines()
        .collect::<Vec<&str>>();
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

fn part1<'a>(vec: &'a Vec<&'a str>) -> Option<Answer<'a>> {
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
    return Some(Answer::new(
        "2.1",
        (total_score).to_string(),
        time.elapsed(),
    ));
}

fn part2<'a>(vec: &'a Vec<&'a str>) -> Option<Answer<'a>> {
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
    return Some(Answer::new(
        "2.2",
        (total_score).to_string(),
        time.elapsed(),
    ));
}
