use clap::Parser;

mod args;
mod days;

const CURRENT_DAY: usize = 0;

fn main() {
    let args = args::Args::parse();
    println!("{}", args.header());

    match args.day {
        None => {
            let results: Vec<_> = days::DAYS.iter().take(CURRENT_DAY).map(|d| d()).collect();
        },
        Some(day) => {
            let result = days::DAYS[day]();
        },
    }
}
