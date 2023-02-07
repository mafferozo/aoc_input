use clap::{Parser, command};

/// Simple program to download puzzle input of Advent Of Code
/// 
/// To reduce network traffic on adventofcode.com, 
/// the program will try to cache the puzzles in ~/.aoc_puzzles
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Args {
    /// The year of the puzzle
    #[arg(value_parser = year_in_range)]
    pub year: u32,

    /// The day of the puzzle
    #[arg(value_parser = clap::value_parser!(u32).range(1..26))]
    pub day: u32,

    /// Your session token
    pub session: Option<String>,
}

/// TODO: get current year for `end` bound. 
fn year_in_range(s: &str) -> Result<u32, String> {
    let year_range = 2015..2024;

    let year: u32 = s.parse().map_err(|_| format!("`{}` isn't a year", s))?;

    if year_range.contains(&year) {
        Ok(year)
    } else {
        Err(format!(
            "year not in range {}-{}",
            year_range.start, year_range.end
        ))
    }
}