use crate::utils::input;
use std::time::Duration;
pub struct Answer<'a> {
    pub prob_number: &'a str,
    pub answer: String,
    pub time: Duration,
}

impl<'a> Answer<'a> {
    pub fn new(prob_number: &'a str, answer: String, time: Duration) -> Answer {
        Answer {
            prob_number,
            answer,
            time,
        }
    }
    pub fn display(&self) {
        fn p(time: String, unit: &str, prob_number: &str, answer: &str) {
            println!(
                "(runtime: {} {})\t Problem {}:\t {}",
                time, unit, prob_number, answer,
            );
        }
        if self.time.as_secs() > 0 {
            p(
                self.time.as_secs().to_string(),
                "s",
                self.prob_number,
                &self.answer,
            );
        } else if self.time.as_millis() > 0 {
            p(
                self.time.as_millis().to_string(),
                "ms",
                self.prob_number,
                &self.answer,
            );
        } else if self.time.as_micros() > 0 {
            p(
                self.time.as_micros().to_string(),
                "μs",
                self.prob_number,
                &self.answer,
            );
        } else {
            p(
                self.time.as_nanos().to_string(),
                "ns",
                self.prob_number,
                &self.answer,
            );
        }
    }
}

pub trait Solver {
    fn solve(&self, filepath: &str) -> Duration {
        let vec = input::read_file(filepath);
        let mut total_time = Duration::new(0, 0);
        match self.part1(&vec) {
            Some(i) => {
                total_time += i.time;
                i.display();
            }
            None => println!("No answer found for part 1."),
        };
        match self.part2(&vec) {
            Some(i) => {
                total_time += i.time;
                i.display();
            }
            None => println!("No answer found for part 2."),
        };
        total_time
    }

    fn part1(&self, vec: &Vec<String>) -> Option<Answer>;
    fn part2(&self, vec: &Vec<String>) -> Option<Answer>;
}
