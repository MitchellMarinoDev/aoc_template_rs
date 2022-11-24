use crate::CURRENT_DAY;
use clap::{Parser, ValueEnum};
use colored::control::{set_override, unset_override};
use colored::Colorize;
use std::path::{Path, PathBuf};

/// The options for the --color flag.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, ValueEnum)]
pub enum ColorOptions {
    #[default]
    Auto,
    Always,
    Never,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Parser)]
#[command(author, version, about)]
pub struct Args {
    /// The day to run. Defaults to the current day.
    #[arg(short, long)]
    #[arg(group = "day_spec")]
    #[arg(value_parser = valid_day)]
    #[arg(default_value_t = CURRENT_DAY)]
    pub day: usize,

    /// A flag that causes all days up to the current day to be ran.
    #[arg(short, long)]
    #[arg(group = "day_spec")]
    pub all: bool,

    /// How to output colors on the terminal.
    #[arg(short, long)]
    #[arg(value_enum)]
    #[arg(default_value_t = ColorOptions::Auto)]
    pub color: ColorOptions,

    /// The path to look for the puzzle input in.
    #[arg(short, long)]
    input_path: Option<PathBuf>,

    /// Don't print a header before solving.
    #[arg(short, long)]
    pub no_header: bool,
}

impl Args {
    /// Builds a string for a header of what days will be solved.
    pub fn header(&self) -> String {
        if self.no_header {
            return String::new();
        }

        if CURRENT_DAY == 0 {
            format!(
                "No days implemented yet. Change {} in {} to get started.",
                "CURRENT_DAY".bold(),
                "main.rs".bold()
            )
        } else if CURRENT_DAY == 1 || !self.all {
            format!(
                "{}",
                format!("Solving day {}", self.day.to_string().green()).bold()
            )
        } else {
            format!(
                "{}",
                format!(
                    "Solving days {}-{}",
                    1.to_string().bold().green(),
                    self.day.to_string().bold().green()
                )
                .bold()
            )
        }
    }

    /// Gets the input path, using a default value if unset.
    pub fn input_path(&self) -> &Path {
        match &self.input_path {
            Some(ip) => ip,
            None => Path::new("./input/"),
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

    if day > CURRENT_DAY {
        return Err(format!("day `{}` isn't implemented yet", day));
    }

    Ok(day)
}
