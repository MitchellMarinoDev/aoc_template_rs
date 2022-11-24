use crate::args::Args;
use crate::days::Solution;
use clap::Parser;
use std::process::exit;

mod args;
mod days;
mod solutions;

/// The puzzle day that you are currently on.
///
/// You should increment this constant as you go, in order for this to work properly.
const CURRENT_DAY: usize = 1;

fn main() {
    let args = Args::parse();
    args.apply_color_option();
    println!("{}", args.header());
    check_input_dir(&args);

    if args.all {
        let solutions: Vec<_> = days::DAYS
            .iter()
            .take(CURRENT_DAY)
            .map(|d| d.solve(&args.input_path()))
            .collect();

        solutions.iter().for_each(Solution::print);
    } else {
        if args.day > CURRENT_DAY {
            exit(1);
        }
        let solution = days::DAYS[args.day - 1].solve(&args.input_path());
        solution.print();
    }
}

/// Checks if the input files that will be needed exist.
fn check_input_dir(args: &Args) {
    if !args.input_path().exists() {
        panic!(
            "input path {} does not exist. \
            Create it or specify a different path with the -i flag",
            args.input_path().display()
        );
    }

    if args.all {
        for day in days::DAYS.iter().take(CURRENT_DAY) {
            if !day.input_file(args.input_path()).exists() {
                panic!(
                    "input file for day {} does not exist. \
                    Create it or specify a different path with the -i flag",
                    day.day
                );
            }
        }
    } else {
        if !days::DAYS[args.day - 1]
            .input_file(args.input_path())
            .exists()
        {
            panic!(
                "input file for day {} does not exist. \
                Create it or specify a different path with the -i flag",
                args.day
            );
        }
    }
}
