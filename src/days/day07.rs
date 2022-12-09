use crate::utils::structs::{Answer, Solver};
use regex::Regex;
use std::time::Instant;

use self::filetree::FileSystem;

pub struct Day;

pub mod filetree {
    pub struct Node {
        pub filesize: u32,
        pub is_dir: bool,
        pub parent: Option<usize>,
        pub children: Option<Vec<(usize, String)>>, // idx, name
    }

    pub struct FileSystem {
        pub tree: Vec<Node>,
    }
    impl FileSystem {
        pub fn new(filesize: u32) -> Self {
            Self {
                tree: vec![Node::new(filesize, true)],
            }
        }
    }

    impl Node {
        fn new(filesize: u32, is_dir: bool) -> Self {
            Self {
                filesize,
                is_dir,
                parent: None,
                children: None,
            }
        }
        pub fn contains(&self, node_name: &str) -> bool {
            match self.children.as_ref() {
                Some(v) => {
                    for c in v {
                        if c.1 == node_name {
                            return true;
                        }
                    }
                }
                None => return false,
            }
            false
        }
    }

    impl FileSystem {
        pub fn node(&mut self, filesize: u32, type_is_dir: bool) -> usize {
            let idx = self.tree.len();
            self.tree.push(Node::new(filesize, type_is_dir));
            idx
        }

        pub fn increment_size(&mut self, origin_node_idx: usize) {
            let quantity = self.tree[origin_node_idx].filesize;
            let mut current_idx = origin_node_idx;
            let mut stop = false;
            while !stop {
                match self.tree[current_idx].parent {
                    Some(i) => {
                        self.tree[i].filesize += quantity;
                        current_idx = i;
                    }
                    None => stop = true,
                }
            }
        }
    }
}

fn build_filesystem(vec: &Vec<String>) -> FileSystem {
    let mut current_idx: usize = 0;
    let mut system: FileSystem = filetree::FileSystem::new(0);
    let re_file = Regex::new(r"^(\d+) (\S+)").unwrap();
    let re_directory = Regex::new(r"^dir (\S+)").unwrap();
    let re_cd_up = Regex::new(r"^\$ cd \.\.").unwrap();
    let re_cd = Regex::new(r"^\$ cd (\S+)").unwrap();

    for l in vec {
        match l.as_str() {
            "$ cd /" => {
                current_idx = 0;
                continue;
            }
            "$ ls" => {
                continue;
            }
            _ => {
                if re_directory.is_match(l) {
                    let dir = &re_directory.captures(l).unwrap()[1];
                    if !system.tree[current_idx as usize].contains(dir) {
                        let idx = system.node(0, true);
                        match &mut system.tree[current_idx].children {
                            Some(c) => c.push((idx, dir.to_string())),
                            None => {
                                system.tree[current_idx].children =
                                    Some(vec![(idx, dir.to_string())])
                            }
                        }
                        system.tree[idx].parent = Some(current_idx);
                    }
                } else if re_file.is_match(l) {
                    let size = re_file.captures(l).unwrap()[1].parse::<u32>().unwrap();
                    let name = &re_file.captures(l).unwrap()[2];
                    if !system.tree[current_idx as usize].contains(name) {
                        let idx = system.node(size, false);
                        match &mut system.tree[current_idx].children {
                            Some(c) => c.push((idx, name.to_string())),
                            None => {
                                system.tree[current_idx].children =
                                    Some(vec![(idx, name.to_string())])
                            }
                        }
                        system.tree[idx].parent = Some(current_idx);
                        match system.tree[idx].parent {
                            Some(_) => {
                                system.increment_size(idx);
                            }
                            None => {
                                panic!("This should have a parent.")
                            }
                        }
                    }
                } else if re_cd_up.is_match(l) {
                    match system.tree[current_idx].parent {
                        Some(i) => current_idx = i,
                        None => panic!("Trying to cd .. from a directory without a parent."),
                    }
                } else if re_cd.is_match(l) {
                    let target_dir = &re_cd.captures(l).unwrap()[1];
                    match &system.tree[current_idx].children {
                        Some(children) => {
                            for c in children {
                                if c.1 == target_dir {
                                    current_idx = c.0;
                                }
                            }
                        }
                        None => panic!("Trying to cd to a non-existing directory."),
                    }
                } else {
                    panic!("{} is not a recognised command", l);
                }
            }
        }
    }

    system
}

impl Solver for Day {
    /// Solution partly inspired by this blogpost from 2019
    /// <https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6>

    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();
        // Add up values
        let answer: u32 = build_filesystem(vec)
            .tree
            .iter()
            .filter(|f| f.is_dir && (f.filesize <= 100000))
            .map(|f| f.filesize)
            .sum();

        return Some(Answer::new(answer.to_string(), time.elapsed()));
    }

    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();
        let system = build_filesystem(vec);
        let du = system.tree[0].filesize as i32;
        let needed = 70000000 - du - 30000000;
        assert!(needed < 0);
        let needed = (-needed) as u32;
        // Add up values
        let answer = system
            .tree
            .iter()
            .filter(|f| f.is_dir && f.filesize >= needed)
            .map(|f| f.filesize)
            .min()
            .unwrap();

        return Some(Answer::new(answer.to_string(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day07::*;
    use crate::utils::input;
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/day07.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "1513699")
    }
    #[test]
    fn part2() {
        let vec = input::read_file("inputs/day07.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "7991939")
    }
}
