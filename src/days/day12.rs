use crate::utils::structs::{Answer, Solver};
use rand::seq::SliceRandom;
use std::{collections::HashMap, time::Instant};

pub struct Day;

pub fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    indices
}

pub struct Map {
    pub map: Vec<Vec<Node>>,
    pub init_pos: (usize, usize),
    pub final_pos: (usize, usize),
}

impl Map {
    fn new(map: Vec<Vec<Node>>) -> Self {
        Self {
            map: map,
            init_pos: (0, 0),
            final_pos: (0, 0),
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
    fn get_final_pos(&mut self, final_val: i8) -> (usize, usize) {
        for i in 0..self.map.len() {
            for j in 0..self.map[0].len() {
                if self.map[i][j].value == final_val {
                    self.final_pos = (i, j);
                    break;
                }
            }
            if self.final_pos != (0, 0) {
                break;
            };
        }

        // Compute distance from each point to end
        for (i, r) in self.map.iter_mut().enumerate() {
            for (j, c) in r.iter_mut().enumerate() {
                c.distance_to_end = (10000.0
                    * (f32::powf((i as f32) - (self.final_pos.0 as f32), 2.0)
                        + f32::powf((j as f32) - (self.final_pos.1 as f32), 2.0)))
                    as usize;
            }
        }

        self.final_pos
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

    fn get_next_step(&mut self, path: &Vec<(usize, usize)>) -> Option<(usize, usize)> {
        let last_node = path.last().unwrap();

        if self.map[last_node.0][last_node.1].accesible.is_empty() {
            return None;
        }

        let mut candidates = vec![];
        let mut distances = vec![];

        for v in self.map[last_node.0][last_node.1].accesible.iter() {
            if path.contains(v) {
                continue;
            }
            let new_n = &self.map[v.0][v.1];
            if new_n.is_blocked {
                continue;
            }
            candidates.push(*v);
            distances.push(new_n.distance_to_end);
        }

        if candidates.is_empty() {
            return None;
        }

        // Bias candidates towards the closest to the end
        let mut final_candidates: Vec<(usize, usize)> = vec![];
        let sort_order = argsort(&distances);
        for (n_rep, idx) in sort_order.iter().rev().enumerate() {
            for _ in 0..usize::pow(n_rep + 1, 2) {
                final_candidates.push(candidates[*idx].clone());
            }
        }

        return Some(*final_candidates.choose(&mut rand::thread_rng()).unwrap());
    }

    fn search(&mut self) -> u16 {
        let mut shortest_path = std::u16::MAX;
        for itr in 0..1000000 {
            let mut stop = false;
            let mut current_path = vec![self.init_pos];
            while !stop {
                if (current_path.len() as u16) - 1 >= shortest_path {
                    break;
                }
                match self.get_next_step(&current_path) {
                    Some(p) => {
                        current_path.push(p);
                        if p == self.final_pos {
                            let final_len = (current_path.len() - 1) as u16;
                            if final_len < shortest_path {
                                shortest_path = final_len;
                            }
                            for n in current_path.iter() {
                                if self.map[n.0][n.1].shortest_path < final_len {
                                    self.map[n.0][n.1].shortest_path = final_len;
                                }
                            }

                            stop = true;
                        } else {
                            self.map[p.0][p.1].last_explored = itr;
                        }
                    }
                    None => {
                        let n = current_path.last().unwrap();
                        if self.map[n.0][n.1].accesible.is_empty() {
                            self.map[n.0][n.1].is_blocked = true;
                        }

                        if self.map[n.0][n.1]
                            .accesible
                            .iter()
                            .all(|x| self.map[x.0][x.1].is_blocked)
                        {
                            self.map[n.0][n.1].is_blocked = true;
                        }

                        stop = true;
                    }
                }
            }
        }
        shortest_path
    }
}

#[derive(Clone)]
pub struct Node {
    pub coord: (usize, usize),
    pub shortest_path: u16,
    pub is_blocked: bool,
    pub last_explored: usize,
    pub value: i8,
    pub accesible: Vec<(usize, usize)>, // (y, x)
    pub distance_to_end: usize, // this will be squared, but it's indifferent for what we're doing
}

impl Node {
    fn new(
        coord: (usize, usize),
        shortest_path: u16,
        is_blocked: bool,
        last_explored: usize,
        value: i8,
    ) -> Self {
        Self {
            coord,
            shortest_path,
            is_blocked,
            last_explored,
            value,
            accesible: vec![], // (y, x)
            distance_to_end: 0,
        }
    }
}

fn get_h_map(vec: &Vec<String>) -> (Vec<Vec<Node>>, i8) {
    let mut score_map = ("abcdefghijklmnopqrstuvwxyz".chars().enumerate())
        .map(|(i, c)| (c, i as i8))
        .collect::<HashMap<char, i8>>();
    score_map.insert('E', score_map.len() as i8);
    score_map.insert('S', -1);

    let mut h_map: Vec<Vec<Node>> =
        vec![vec![Node::new((0, 0), std::u16::MAX, false, 0, -1); vec[0].len()]; vec.len()];

    for (i, r) in vec.iter().enumerate() {
        for (j, v) in r.chars().enumerate() {
            let value = score_map[&v];
            h_map[i][j] = Node::new((i, j), 0, false, 0, value);
        }
    }

    (h_map, (score_map.len() - 2) as i8)
}

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();
        let (m, final_val) = get_h_map(&vec);
        let mut h_map = Map::new(m);

        h_map.check_accesible();
        h_map.get_init_pos();
        h_map.get_final_pos(final_val);

        let ans = h_map.search();
        if ans == std::u16::MAX {
            for r in h_map.map {
                for c in r {
                    print!("{}", (c.last_explored > 0) as u8);
                }
                println!("")
            }
            return None;
        }

        Some(Answer::new(ans.to_string(), time.elapsed()))
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