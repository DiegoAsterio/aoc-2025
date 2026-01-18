use crate::extract::input::PuzzleInput;

mod output;
mod one;
mod two;
mod three;

pub fn solve_puzzle(input: &PuzzleInput) -> Result<output::PuzzleOutput, String> {
    match input.day {
        1 => one::solve(input),
        2 => two::solve(input),
        3 => three::solve(input),
        n => Err(format!("No solution available for day {number}", number=n).to_string())
    }
}
