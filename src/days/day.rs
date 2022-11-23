use std::path::Path;
use std::time::{Duration, Instant};

pub struct Day {
    /// The day number.
    day: usize,
    /// The function that is used to get the solution.
    solver: fn(String) -> (String, String),
}

impl Day {
    pub fn solve(&self, input_path: &String) -> Solution {
        let file_name = Path::new(input_path).join(format!("d{}.txt", self.day));
        let input = std::fs::read_to_string(file_name)
            .expect(&*format!("failed to read input file for day {}", self.day));
        let start = Instant::now();
        let r = (self.solver)(input);
        let dur = Instant::now() - start;
        Solution {
            p1: r.0,
            p2: r.1,
            duration: dur,
        }
    }

    pub const fn new(day: usize, solver: fn(String) -> (String, String)) -> Self {
        Day { day, solver }
    }
}

pub struct Solution {
    /// The solution to part1.
    p1: String,
    /// The solution to part2.
    p2: String,
    /// The time that it took to solve.
    duration: Duration,
}
