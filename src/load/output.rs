pub struct PuzzleOutput {
    pub result: String
}

impl PuzzleOutput {
    pub fn show(&self) {
        println!("The code for the elves is: {}", self.result)
    }
}
