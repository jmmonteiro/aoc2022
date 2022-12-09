use crate::utils::structs::{Answer, Solver};
use std::time::Instant;

pub struct Day;

fn convert_vec(vec: &Vec<String>) -> Vec<Vec<u8>> {
    // TODO: Re-write this part
    let mut matrix: Vec<Vec<u8>> = vec![];
    for r in vec.iter() {
        matrix.push(
            r.chars()
                .into_iter()
                .map(|i| i.to_string().parse::<u8>().unwrap())
                .collect(),
        );
    }
    matrix
}

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        // Convert
        let matrix = convert_vec(&vec);

        let mut total: u32 = (matrix.len() * 2 + (matrix[0].len() - 2) * 2) as u32;

        for j in 1..(matrix.len() - 1) {
            for i in 1..(matrix[0].len() - 1) {
                let target_value = matrix[j][i];

                let mut clear = true;
                for ii in 0..i {
                    if matrix[j][ii] >= target_value {
                        clear = false;
                        break;
                    }
                }
                if clear {
                    total += 1;
                    continue;
                }
                clear = true;

                for ii in i + 1..matrix[0].len() {
                    if matrix[j][ii] >= target_value {
                        clear = false;
                        break;
                    }
                }
                if clear {
                    total += 1;
                    continue;
                }
                clear = true;

                for jj in 0..j {
                    if matrix[jj][i] >= target_value {
                        clear = false;
                        break;
                    }
                }
                if clear {
                    total += 1;
                    continue;
                }
                clear = true;

                for jj in j + 1..matrix.len() {
                    if matrix[jj][i] >= target_value {
                        clear = false;
                        break;
                    }
                }
                if clear {
                    total += 1;
                }
            }
        }

        return Some(Answer::new(total.to_string(), time.elapsed()));
    }

    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        // Convert
        let matrix = convert_vec(&vec);

        let mut max_scenic: u128 = 0;

        for j in 1..(matrix.len() - 1) {
            for i in 1..(matrix[0].len() - 1) {
                let target_value = matrix[j][i];
                let mut current_scenic: u128 = 1;

                let mut current_trees: u128 = 0;
                for ii in (0..i).rev() {
                    current_trees += 1;
                    if matrix[j][ii] >= target_value {
                        break;
                    }
                }
                current_scenic *= current_trees;
                current_trees = 0;
                for ii in i + 1..matrix[0].len() {
                    current_trees += 1;
                    if matrix[j][ii] >= target_value {
                        break;
                    }
                }

                current_scenic *= current_trees;
                current_trees = 0;
                for jj in (0..j).rev() {
                    current_trees += 1;
                    if matrix[jj][i] >= target_value {
                        break;
                    }
                }
                current_scenic *= current_trees;
                current_trees = 0;
                for jj in j + 1..matrix.len() {
                    current_trees += 1;
                    if matrix[jj][i] >= target_value {
                        break;
                    }
                }

                current_scenic *= current_trees;
                if current_scenic > max_scenic {
                    max_scenic = current_scenic;
                }
            }
        }

        return Some(Answer::new(max_scenic.to_string(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day08::*;
    use crate::utils::input;
    #[test]
    fn unit_test_part1() {
        let vec = input::read_file("inputs/day08_test.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "21")
    }
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/day08.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "1794")
    }
    #[test]
    fn unit_test_part2() {
        let vec = input::read_file("inputs/day08_test.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "8")
    }
    #[test]
    fn part2() {
        let vec = input::read_file("inputs/day08.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "199272")
    }
}
