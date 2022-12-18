use crate::utils::structs::{Answer, Solver};
use std::{collections::HashMap, time::Instant};

pub struct Day;

pub struct Map {
    pub map: Vec<Vec<Node>>,
    pub init_pos: (usize, usize),
}

impl Map {
    fn new(map: Vec<Vec<Node>>) -> Self {
        Self {
            map: map,
            init_pos: (0, 0),
        }
    }

    fn get_init_pos(&mut self) -> (usize, usize) {
        for i in 0..self.map.len() {
            for j in 0..self.map[0].len() {
                if self.map[i][j].value == -1 {
                    self.init_pos = (i, j);
                    return self.init_pos;
                }
            }
        }
        self.init_pos
    }

    fn check_accesible(&mut self) {
        for i in 0..self.map.len() {
            for j in 0..self.map[0].len() {
                if self.map[i][j].coord.0 > 0 {
                    if self.map[i - 1][j].value <= self.map[i][j].value + 1 {
                        self.map[i][j].accesible.push((i - 1, j));
                    }
                }
                if self.map[i][j].coord.0 < self.map.len() - 1 {
                    if self.map[i + 1][j].value <= self.map[i][j].value + 1 {
                        self.map[i][j].accesible.push((i + 1, j));
                    }
                }

                if self.map[i][j].coord.1 > 0 {
                    if self.map[i][j - 1].value <= self.map[i][j].value + 1 {
                        self.map[i][j].accesible.push((i, j - 1));
                    }
                }
                if self.map[i][j].coord.1 < self.map[0].len() - 1 {
                    if self.map[i][j + 1].value <= self.map[i][j].value + 1 {
                        self.map[i][j].accesible.push((i, j + 1));
                    }
                }
            }
        }
    }

    fn get_next_step(&self, coord: (usize, usize)) -> Result<(usize, usize)> {
        for v in self.map[coord.0][coord.1].accesible.iter() {
            if self.map[v.0][v.1].shortest_path == self.map[coord.0][coord.1] {
                return Some(*v);
            }
        }
        None
    }

    fn search(&mut self) -> u16 {
        let stop = false;
        while !stop {
            return 0;
        }
        0
    }
}

#[derive(Clone)]
pub struct Node {
    pub coord: (usize, usize),
    pub shortest_path: u16,
    pub is_blocked: bool,
    pub value: i8,
    pub accesible: Vec<(usize, usize)>, // (y, x)
}

impl Node {
    fn new(coord: (usize, usize), shortest_path: u16, is_blocked: bool, value: i8) -> Self {
        Self {
            coord,
            shortest_path,
            is_blocked,
            value,
            accesible: vec![], // (y, x)
        }
    }
}

fn get_h_map(vec: &Vec<String>) -> Vec<Vec<Node>> {
    let mut score_map = ("abcdefghijklmnopqrstuvwxyz".chars().enumerate())
        .map(|(i, c)| (c, i as i8))
        .collect::<HashMap<char, i8>>();
    score_map.insert('E', score_map.len() as i8);
    score_map.insert('S', -1);

    let mut h_map: Vec<Vec<Node>> =
        vec![vec![Node::new((0, 0), 0, false, -1); vec[0].len()]; vec.len()];

    for (i, r) in vec.iter().enumerate() {
        for (j, v) in r.chars().enumerate() {
            let value = score_map[&v];
            h_map[i][j] = Node::new((i, j), 0, false, value);
        }
    }

    h_map
}

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();
        let mut h_map = Map::new(get_h_map(&vec));

        h_map.check_accesible();
        h_map.get_init_pos();
        h_map.search();

        return Some(Answer::new(("test").to_string(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day12::*;
    use crate::utils::input;
    #[test]
    fn unit_test_part1() {
        let vec = input::read_file("inputs/day12_test.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "31")
    }
}
