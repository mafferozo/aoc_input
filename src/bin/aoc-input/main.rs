mod args;

use std::{process};

use clap::{Parser};

use aoc_input_lib::get_puzzle_input;

use args::Args;

fn main()  {
    let cli = Args::parse();

    let input = get_puzzle_input(cli.year,cli.day,Some(cli.session));
    match input {
        Ok(input) => print!("{}",input),
        Err(error) => {eprintln!("{}",error); process::exit(1)},
    }
}