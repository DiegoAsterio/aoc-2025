use std::str::FromStr;
use std::string::ToString;
use std::iter;

use crate::extract::input::PuzzleInput;
use crate::load::output::PuzzleOutput;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn decode_content_into_puzzle_input(content: String) -> Vec<(String,Vec<i64>)> {
    let mut iter = content.trim().rsplit("\n");

    let operations = iter.next().expect("Input should have at least one row")
        .split_whitespace().map(ToString::to_string);

    let values = iter    
                  .map( |x| {
                      x.trim().split_whitespace()
                              .map(|y| {
                                  i64::from_str(y)
                                      .expect("Error casting L1-(N-1) to integers") 
                              }).collect()
                  }).collect();
    let t_values = transpose(values);

    iter::zip(operations, t_values).collect()
}

fn solve_problem(operation: &str, values: Vec<i64>) -> i64{
    match operation {
        "+" => values.into_iter().sum(),
        "*" => values.into_iter().product(),
        _   => panic!("This operation is not supported")
    }
}

fn solve_fst(content: String) -> String {
    let problems = decode_content_into_puzzle_input(content);
    problems.into_iter().map(|(operation, values)| {solve_problem(&operation, values)}).sum::<i64>().to_string()
}

pub fn solve(input: &PuzzleInput) -> Result<PuzzleOutput, String> {
    match input {
        PuzzleInput{
            iteration: 1,
            text,
            ..
        } => Ok(PuzzleOutput{result: solve_fst(text.to_string())}),
        _ => Err("Incorrect Puzzle Input".to_string())
    }
} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_test_input_correctly(){
        let problems = decode_content_into_puzzle_input("123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ".to_string());
        assert_eq!(("*".to_string(), vec![6, 45, 123]), problems[0]);
        assert_eq!(("+".to_string(), vec![98, 64, 328]), problems[1]);
        assert_eq!(("*".to_string(), vec![215, 387, 51]), problems[2]);
        assert_eq!(("+".to_string(), vec![314, 23, 64]), problems[3]);
    }

    #[test]
    fn solves_example_problems(){
        assert_eq!(33210, solve_problem("*", vec![6, 45, 123]));
        assert_eq!(490, solve_problem("+", vec![98, 64, 328]));
        assert_eq!(4243455, solve_problem("*", vec![215, 387, 51]));
        assert_eq!(401, solve_problem("+", vec![314, 23, 64]));
    }

    #[test]
    fn solves_example_for_fst(){
        let result = solve_fst("123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ".to_string());
        assert_eq!("4277556", result);
    }
}
