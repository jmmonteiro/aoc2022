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
