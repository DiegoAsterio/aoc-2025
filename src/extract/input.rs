use std::fs;
use std::error::Error;

use crate::extract::config::Config;

pub struct PuzzleInput {
    pub day: u8,
    pub iteration: u8,
    pub text: String
}

impl PuzzleInput {
    pub fn new(config: &Config) -> Result<PuzzleInput, Box<dyn Error>> {
        let file_path = format!("./data/day/{}/input", config.day);

        let day = config.day.parse::<u8>()?;

        let iteration = config.iteration.parse::<u8>()?;

        let text = fs::read_to_string(file_path)?;

        Ok(PuzzleInput {day, iteration, text})
    }

}
