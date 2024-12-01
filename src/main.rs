use std::fmt::Display;

use anyhow::Result;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The day of the challenge
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,

    /// The challenge part
    #[arg(value_enum)]
    part: Part,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Part {
    One,
    Two,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::One => write!(f, "One")?,
            Part::Two => write!(f, "Two")?,
        }
        Ok(())
    }
}

mod day;

fn main() -> Result<()> {
    let args = Args::parse();
    let days = day::Days::new();

    days.run(args.day, args.part)?;
    Ok(())
}
