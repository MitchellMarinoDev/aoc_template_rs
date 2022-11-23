use clap::{Parser, ValueEnum};
use colored::Colorize;
use colored::control::{set_override, unset_override};
use crate::CURRENT_DAY;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, ValueEnum)]
pub enum ColorOptions {
    #[default]
    Auto,
    Always,
    Never,
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// The day to run. Defaults to the current day.
    #[arg(short, long)]
    #[arg(group = "day_spec")]
    #[arg(value_parser = valid_day)]
    pub day: Option<usize>,

    /// A flag that causes all days up to the current day to be ran.
    #[arg(short, long)]
    #[arg(group = "day_spec")]
    pub all: bool,

    #[arg(short, long)]
    #[arg(value_enum)]
    #[arg(default_value_t = ColorOptions::Auto)]
    pub color: ColorOptions,

    /// Don't print a header before solving.
    #[arg(short, long)]
    pub no_header: bool,
}

impl Args {
    /// Builds a string for a header of what days will be solved.
    pub fn header(&self) -> String {
        if self.no_header { return String::new(); }

        let day = self.day.unwrap_or(CURRENT_DAY);

        if CURRENT_DAY == 0 {
            format!("No days implemented yet. Change {} in {} to get started.", "CURRENT_DAY".bold(), "main.rs".bold())
        }
        else if CURRENT_DAY == 1 || !self.all {
            format!("Solving day {}", day.to_string().bold().green())
        }
        else {
            format!("Solving days {}-{}", 1.to_string().bold().green(), day.to_string().bold().green())
        }
    }

    /// Applies the terminal colorizing settings from the `color` field.
    pub fn apply_color_option(&self) {
        match self.color {
            ColorOptions::Auto => unset_override(),
            ColorOptions::Always => set_override(true),
            ColorOptions::Never => set_override(false),
        }
    }
}

/// A parsing/validation function for getting the day from the cli args.
fn valid_day(s: &str) -> Result<usize, String> {
    let day: usize = s
        .parse()
        .map_err(|_| format!("`{}` isn't a valid day", s))?;

    if !(1..=25).contains(&day) {
        return Err(format!("`{}` isn't a valid day", day));
    }

    if day <= CURRENT_DAY {
        return Err(format!("day `{}` isn't implemented yet", day))
    }

    Ok(day)
}