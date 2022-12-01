use std::time::Instant;
pub struct Answer<'a> {
    pub prob_number: &'a str,
    pub answer: String,
    pub time: String,
}

impl<'a> Answer<'a> {
    pub fn new(prob_number: &'a str, answer: String, time: Instant) -> Answer {
        Answer {
            prob_number,
            answer,
            time: time.elapsed().as_millis().to_string(),
        }
    }
    pub fn display(&self) {
        println!(
            "(runtime: {} ms)\t Problem {}:\t {}",
            self.time, self.prob_number, self.answer,
        )
    }
}
