mod args;

use std::error::Error;

use clap::{Parser};

use aoc_input_lib::get_puzzle_input;

use args::Args;

type AppResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

fn main() -> AppResult<()> {
    let cli = Args::parse();

    let input = get_puzzle_input(cli.year,cli.day,cli.session)?;
    print!("{}", input);

    Ok(())
}