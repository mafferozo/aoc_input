mod error;
use std::{io::Write, path::PathBuf};

use error::Error;

type AocResult<T> = Result<T, Error>;

pub fn get_puzzle_input(year: u32, day: u32, session: Option<String>) -> AocResult<String> {
    match from_cache(year, day) {
        Some(input) => Ok(input),
        None => session
            .ok_or(Error::SessionRequired)
            .and_then(|session| from_web(year, day, session, true)),
    }
}

pub fn from_web(year: u32, day: u32, session: String, should_cache: bool) -> AocResult<String> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let cookie = format!("session={}", session);

    let puzzle_input = reqwest::blocking::Client::new()
        .get(url)
        .header("Cookie", cookie)
        .send()?
        .text()?;

    if should_cache {
        set_cache(year, day, puzzle_input.clone());
    }
    Ok(puzzle_input)
}

// directory structure:
// ~/.aoc_puzzles/{year}/day-{day}.txt
fn get_path(year: u32, day: u32) -> Option<PathBuf> {
    home::home_dir().map(|mut h| {
        h.push(format!(".aoc_puzzles/{year}/day-{day}.txt"));
        h
    })
}

/// Try to get the puzzle from the cache.
fn from_cache(year: u32, day: u32) -> Option<String> {
    get_path(year, day).and_then(|p| std::fs::read_to_string(p).ok())
}

/// Try to cache the puzzle input under the home directory.
/// Discard any errors if caching fails
fn set_cache(year: u32, day: u32, puzzle_input: String) {
    if let Some(p) = get_path(year, day) {
        // should not panic
        let prefix = p.parent().unwrap();

        _ = std::fs::create_dir_all(prefix);
        _ = std::fs::File::create(p).and_then(|mut f| f.write_all(puzzle_input.as_bytes()));
    };
}
