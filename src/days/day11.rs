use crate::utils::structs::{Answer, Solver};
use std::time::Instant;

pub struct Day;

pub struct Monkey {
    items: Vec<u128>,
    update_fun: Box<dyn FnMut(u128) -> u128>,
    div_num: u128,
    true_monkey_idx: usize,
    false_monkey_idx: usize,
    num_inspect: u128,
}

impl Monkey {
    pub fn new(
        items: Vec<u128>,
        update_fun: Box<dyn FnMut(u128) -> u128>,
        div_num: u128,
        true_monkey_idx: usize,
        false_monkey_idx: usize,
    ) -> Monkey {
        Monkey {
            items,
            update_fun,
            div_num,
            true_monkey_idx,
            false_monkey_idx,
            num_inspect: 0,
        }
    }

    pub fn inspect_items(&mut self) -> Vec<(usize, u128)> {
        let mut throw: Vec<(usize, u128)> = vec![];
        for item in self.items.iter() {
            self.num_inspect += 1;
            let new_val = (((self.update_fun)(*item) as f64) / (3 as f64)).floor() as u128;
            if new_val % self.div_num == 0 {
                throw.push((self.true_monkey_idx, new_val));
            } else {
                throw.push((self.false_monkey_idx, new_val));
            }
        }
        self.items = vec![];
        throw
    }
}

pub struct MonkeyGang {
    monkeys: [Monkey; 8],
}

impl MonkeyGang {
    pub fn new(monkeys: [Monkey; 8]) -> Self {
        MonkeyGang { monkeys }
    }
    pub fn round(&mut self) {
        for i in 0..self.monkeys.len() {
            for (j, val) in self.monkeys[i].inspect_items() {
                self.monkeys[j].items.push(val);
            }
        }
    }
}

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let mut monkeys = MonkeyGang::new([
            Monkey::new(
                vec![89, 95, 92, 64, 87, 68],
                Box::new(|i: u128| -> u128 { i * 11 }),
                2,
                7,
                4,
            ),
            Monkey::new(
                vec![87, 67],
                Box::new(|i: u128| -> u128 { i + 1 }),
                13,
                3,
                6,
            ),
            Monkey::new(
                vec![95, 79, 92, 82, 60],
                Box::new(|i: u128| -> u128 { i + 6 }),
                3,
                1,
                6,
            ),
            Monkey::new(
                vec![67, 97, 56],
                Box::new(|i: u128| -> u128 { i * i }),
                17,
                7,
                0,
            ),
            Monkey::new(
                vec![80, 68, 87, 94, 61, 59, 50, 68],
                Box::new(|i: u128| -> u128 { i * 7 }),
                19,
                5,
                2,
            ),
            Monkey::new(
                vec![73, 51, 76, 59],
                Box::new(|i: u128| -> u128 { i + 8 }),
                7,
                2,
                1,
            ),
            Monkey::new(vec![92], Box::new(|i: u128| -> u128 { i + 5 }), 1, 3, 0),
            Monkey::new(
                vec![99, 76, 78, 76, 79, 90, 89],
                Box::new(|i: u128| -> u128 { i + 7 }),
                5,
                4,
                5,
            ),
        ]);

        for _ in 0..20 {
            monkeys.round();
        }

        //return Some(Answer::new(("test").to_string(), time.elapsed()));
        None
    }
}

//#[cfg(test)]
//mod tests {
//    use crate::days::day11::*;
//    use crate::utils::input;
//    #[test]
//    fn part1() {
//        let vec = input::read_file("inputs/day11.txt");
//        assert_eq!(Day.part1(&vec).unwrap().answer, "15572")
//    }
//}
