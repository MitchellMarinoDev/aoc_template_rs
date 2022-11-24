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
    day: usize,
    /// The solution to part1.
    p1: String,
    /// The solution to part2.
    p2: String,
    /// The time that it took to solve.
    duration: Duration,
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
        println!("    {} {}", "p1:".bold().blue(), self.p1.bold().green());
        println!("    {} {}", "p2:".bold().blue(), self.p2.bold().green());
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
