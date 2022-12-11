use crate::utils::structs::{Answer, Solver};
use std::time::Instant;

pub struct Day;

pub struct CPU {
    current_cycle: i32,
    x: i32,
    result: i32,
    target_cycles: [i32; 6],
    current_target_cycle: usize,
}

impl CPU {
    pub fn new(target_cycles: [i32; 6]) -> Self {
        CPU {
            current_cycle: 1,
            x: 1,
            result: 0,
            target_cycles: target_cycles,
            current_target_cycle: 0,
        }
    }

    fn check_cycles(&mut self) -> i32 {
        if self.current_cycle == self.target_cycles[self.current_target_cycle] {
            self.result += self.current_cycle * self.x;
            self.current_target_cycle += 1;
            if self.current_target_cycle == self.target_cycles.len() {
                return -1;
            }
        }
        0
    }

    pub fn instruction(&mut self, ins: &str, num: i32, mut rep: i32) -> i32 {
        if ins == "noop" {
            let result = self.check_cycles();
            self.current_cycle += 1;
            return result;
        }

        if rep > 0 {
            rep -= 1;
            let result = self.check_cycles();
            if result == -1 {
                return -1;
            }
            self.current_cycle += 1;
            while rep > 0 {
                rep = self.instruction(ins, num, rep);
                if rep == -1 {
                    return -1;
                }
                self.x += num;
            }
        }
        rep
    }
}

pub struct CRT {
    current_cycle: usize,
    window: (i32, i32),
    result: [char; 240],
    offset: i32,
}

impl CRT {
    pub fn new() -> Self {
        CRT {
            current_cycle: 1,
            window: (1, 3),
            result: [' '; 240],
            offset: 0,
        }
    }

    fn draw(&mut self) {
        if self.window.0 <= (self.current_cycle as i32)
            && self.window.1 >= (self.current_cycle as i32)
        {
            self.result[self.current_cycle - 1] = '#';
        }
    }

    fn increase_cycle(&mut self) -> i32 {
        self.current_cycle += 1;
        if self.current_cycle >= 240 {
            return -1;
        }
        if self.current_cycle % 40 == 0 {
            self.window.0 += 40;
            self.window.1 += 40;
            self.offset += 40;
        }
        return 0;
    }

    pub fn instruction(&mut self, ins: &str, num: i32, mut rep: i32) -> i32 {
        if ins == "noop" {
            self.draw();
            return self.increase_cycle();
        }

        if rep > 0 {
            rep -= 1;
            self.draw();
            if self.increase_cycle() == -1 {
                return -1;
            }
            while rep > 0 {
                rep = self.instruction(ins, num, rep);
                self.window.0 += num;
                self.window.1 += num;
            }
        }
        rep
    }
    pub fn format_output(&self) -> String {
        "\n".to_string()
            + &self.result[..40].iter().collect::<String>()
            + "\n"
            + &self.result[40..80].iter().collect::<String>()
            + "\n"
            + &self.result[80..120].iter().collect::<String>()
            + "\n"
            + &self.result[120..160].iter().collect::<String>()
            + "\n"
            + &self.result[160..200].iter().collect::<String>()
            + "\n"
            + &self.result[200..240].iter().collect::<String>()
            + "\n"
    }
}

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let mut cpu = CPU::new([20, 60, 100, 140, 180, 220]);
        for l in vec {
            let ins: &str;
            let num: i32;
            let rep;
            match l.split_once(" ") {
                Some((i, n)) => {
                    ins = i;
                    num = n.parse::<i32>().unwrap();
                    rep = 2;
                }
                None => {
                    ins = "noop";
                    num = 0;
                    rep = 1;
                }
            }

            if cpu.instruction(ins, num, rep) == -1 {
                return Some(Answer::new((cpu.result).to_string(), time.elapsed()));
            }
        }
        return Some(Answer::new((cpu.result).to_string(), time.elapsed()));
    }

    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();
        let mut crt = CRT::new();

        for l in vec {
            let ins: &str;
            let num: i32;
            let rep;
            match l.split_once(" ") {
                Some((i, n)) => {
                    ins = i;
                    num = n.parse::<i32>().unwrap();
                    rep = 2;
                }
                None => {
                    ins = "noop";
                    num = 0;
                    rep = 1;
                }
            }

            if crt.instruction(ins, num, rep) == -1 {
                break;
            };
        }

        return Some(Answer::new(crt.format_output(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day10::*;
    use crate::utils::input;
    #[test]
    fn unit_test_part1() {
        let vec = input::read_file("inputs/day10_test.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "13140")
    }
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/day10.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "13060")
    }
}
