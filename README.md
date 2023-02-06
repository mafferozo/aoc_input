# aoc_input
A simple program to download and cache your AoC puzzle inputs


## CLI usage example (requires `--feature="cli"`) 
`aoc-input $SESSION 2022 21` will return the puzzle input for day 21, year 2022. 
`$SESSION` should be your session token, found by inspecting the network request to adventofcode.com/2022/day/1 in any browser.


## Remarks
The program will try to cache the puzzle inputs in your home folder, in `~/.aoc_puzzles/`
