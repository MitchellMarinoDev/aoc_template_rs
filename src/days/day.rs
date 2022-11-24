use crate::solutions::SOLUTIONS;
use colored::Colorize;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub struct Day {
    /// The day number.
    pub day: usize,
    /// The function that is used to get the solution.
    solver: fn(String) -> (String, String),
}

impl Day {
    pub fn solve(&self, input_path: &Path) -> Solution {
        let input = std::fs::read_to_string(self.input_file(input_path))
            .expect(&*format!("failed to read input file for day {}", self.day));

        let start = Instant::now();
        let r = (self.solver)(input);
        let dur = Instant::now() - start;

        Solution {
            day: self.day,
            p1: r.0,
            p2: r.1,
            duration: dur,
        }
    }

    pub fn input_file(&self, input_path: &Path) -> PathBuf {
        input_path.join(format!("d{}.txt", self.day))
    }

    pub const fn new(day: usize, solver: fn(String) -> (String, String)) -> Self {
        Day { day, solver }
    }
}

pub struct Solution {
    /// The day that this is the solution for.
    pub day: usize,
    /// The solution to part1.
    pub p1: String,
    /// The solution to part2.
    pub p2: String,
    /// The time that it took to solve.
    pub duration: Duration,
}

impl Solution {
    pub fn print(&self) {
        println!(
            "{}",
            format!(
                "d{} in {}:",
                self.day.to_string().green(),
                format_duration(self.duration)
            )
            .bold()
        );
        let mut p1 = format!("    p1: {}", self.p1).bold();
        let mut p2 = format!("    p2: {}", self.p2).bold();

        // color the outputs: blue if no known solution, green if correct, red if wrong
        if SOLUTIONS[self.day - 1].0.is_empty() {
            p1 = p1.blue();
        } else if SOLUTIONS[self.day - 1].0 == self.p1 {
            p1 = p1.green();
        } else {
            p1 = p1.red();
        }

        if SOLUTIONS[self.day - 1].1.is_empty() {
            p2 = p2.blue();
        } else if SOLUTIONS[self.day - 1].1 == self.p2 {
            p2 = p2.green();
        } else {
            p2 = p2.red();
        }

        println!("{}", p1);
        println!("{}", p2);
    }
}

fn format_duration(d: Duration) -> String {
    let nanos = d.as_nanos();
    if nanos < 10_000 {
        return format!("{}ns", nanos.to_string().green());
    }

    let micros = d.as_micros();
    if micros < 10_000 {
        return format!("{}us", micros.to_string().green());
    }

    let millis = d.as_millis();
    if millis < 10_000 {
        return format!("{}ms", millis.to_string().green());
    }

    return format!("{}s", d.as_secs().to_string().green());
}
