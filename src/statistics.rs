use std::time::Instant;

pub struct Statistics {
    pub current_text: String,
    pub wrong_text: String,
    pub n_correct: i32,
    pub n_wrong: i32,
    pub start_time: Instant,
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            current_text: String::new(),
            wrong_text: String::new(),
            n_correct: 0,
            n_wrong: 0,
            start_time: Instant::now(),
        }
    }

    pub fn reset(&mut self) {
        self.current_text.clear();
        self.wrong_text.clear();
        self.n_correct = 0;
        self.n_wrong = 0;
        self.start_time = Instant::now();
    }

    pub fn add_char(&mut self, original_text: &String, new_char: char) {
        if original_text.chars().nth(self.current_text.len()).unwrap() == new_char
            && self.wrong_text.len() == 0
        {
            self.current_text.push(new_char);
            self.n_correct += 1;
        } else {
            self.wrong_text.push(new_char);
            self.n_wrong += 1;
        }
    }

    pub fn delete_error(&mut self) {
        if self.wrong_text.len() > 0 {
            self.wrong_text.pop();
        }
    }

    pub fn get_accuracy(&self) -> f64 {
        self.n_correct as f64 / (self.n_correct + self.n_wrong) as f64 * 100.0
    }

    pub fn get_wpm(&self) -> f64 {
        let elapsed_time = self.start_time.elapsed();
        let n_words = self.current_text.split_whitespace().count();
        n_words as f64 / elapsed_time.as_secs_f64() * 60.0
    }
}
