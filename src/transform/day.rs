use crate::{extract::input::PuzzleInput, load::output::PuzzleOutput};

mod one;
mod two;
mod three;
mod four;
mod five;

pub fn solve_puzzle(input: &PuzzleInput) -> Result<PuzzleOutput, String> {
    match input.day {
        1 => one::solve(input),
        2 => two::solve(input),
        3 => three::solve(input),
        4 => four::solve(input),
        5 => five::solve(input),
        n => Err(format!("No solution available for day {number}", number=n).to_string())
    }
}
