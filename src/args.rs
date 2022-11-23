use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// The day to run.
    #[arg(short, long)]
    #[arg(value_parser = valid_day)]
    day: Option<usize>,
}

fn valid_day(s: &str) -> Result<usize, String> {
    let err = || format!("`{}` isn't a valid day", s);

    let day: usize = s
        .parse()
        .map_err(|_| err())?;

    match day {
        0..=25 => Ok(day),
        _ => Err(err()),
    }
}