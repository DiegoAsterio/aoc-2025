use std::{env, process};

use aoc_2025::{extract::{config::Config, input::PuzzleInput}, transform::day};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Extract
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Could not parse arguments into valid input: {err}");
        process::exit(1);
    });

    let input = PuzzleInput::new(&config).unwrap_or_else(|err| {
        println!("Unable to correctly read input: {err}");
        process::exit(1);
    });

    // Transform
    let solution = day::solve_puzzle(&input).unwrap_or_else(|err| {
        println!("Unable to correctly process the input: {err}");
        process::exit(1);
    });

    // Load (TODO: maybe update the value to AOC directly)
    solution.show();
}
