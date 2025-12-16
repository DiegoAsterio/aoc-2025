use std::env;
use std::fs;

mod day_one;
mod day_two;
mod day_three;

fn get_content_from_file(file_path: String) -> String{
    println!("Reading file path: {file_path}");
    fs::read_to_string(file_path)
        .expect("Error reading file.")
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    let puzzle = &args[2];

    let content = get_content_from_file(format!("./data/day/{day}/input"));

    let response = match (day.as_str(), puzzle.as_str()) {
        ("1", "1") => day_one::solve_fst(content),
        ("1", "2") => day_one::solve_snd(content),
        ("2", "1") => day_two::solve_fst(content),
        ("2", "2") => day_two::solve_snd(content),
        ("3", "1") => day_three::solve_fst(content),
        _ => "No solution for day={day} and puzzle={puzzle}".to_string(),
    };

    println!("The code for the elves is: {response}")
}
