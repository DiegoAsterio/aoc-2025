use std::cmp;

use crate::{extract::input::PuzzleInput, load::output::PuzzleOutput};

#[derive(PartialEq, Debug)]
enum Cell {
    Empty,
    Paper
}

fn parse_content_into_grid(content: String) -> Vec<Vec<Cell>> {
    let lines = content.lines();
    lines.map(|line| {
        let cells = line.chars();
        cells.map(|x| {
            match x {
                '@' => Cell::Paper,
                '.' => Cell::Empty,
                _ => panic!("Incorrect character.")
            }
        }).collect()
    }).collect()
}

fn surrounding_papers_count(content: &Vec<Vec<Cell>>, row_index: usize, col_index: usize) -> u32 {
    let mut count = 0;

    let num_rows = content.len();
    let num_columns = content.first().unwrap().len();

    for i in row_index.checked_sub(1).unwrap_or(0)..cmp::min(num_rows, row_index + 2) {
        for j in col_index.checked_sub(1).unwrap_or(0)..cmp::min(num_columns, col_index + 2) {
            if i != row_index || j != col_index {
                if content[i][j] == Cell::Paper {
                    count += 1
                }
             }
        }
    }
    count
}

fn get_papers_that_can_be_moved(plan: &Vec<Vec<Cell>>) -> Vec<(usize, usize)> {
    let mut list_of_positions: Vec<(usize, usize)> = vec![];

    for (i, row) in  plan.iter().enumerate() {
        for (j, cell) in row.iter().enumerate(){
            if let Cell::Paper = cell && surrounding_papers_count(&plan, i, j) < 4{
                list_of_positions.push((i,j));
            }
        }
    }

    list_of_positions
}

fn remove_paper(plan: &mut Vec<Vec<Cell>>, positions: &Vec<(usize,usize)>) -> () {
    for (i, j) in positions {
        plan[*i][*j] = Cell::Empty;
    }
}

fn solve_fst(content: String) -> String {
    let plan = parse_content_into_grid(content);

    format!("{}", get_papers_that_can_be_moved(&plan).len())
}

fn solve_snd(content: String) -> String {
    let mut total_papers_moved = 0;
    let mut plan = parse_content_into_grid(content);
    
    loop {
        let changes: Vec<(usize, usize)> = get_papers_that_can_be_moved(&plan);

        remove_paper(&mut plan, &changes);

        if changes.len() == 0 {
            break;
        }
        else {
            total_papers_moved += changes.len();
        }
    }

    format!("{}", total_papers_moved)
}

pub fn solve(input: &PuzzleInput) -> Result<PuzzleOutput, String> {
    match input {
        PuzzleInput{
            iteration: 1,
            text,
            ..
        } => Ok(PuzzleOutput{result: solve_fst(text.to_string())}),
        PuzzleInput{
            iteration: 2,
            text,
            ..
        } => Ok(PuzzleOutput{result: solve_snd(text.to_string())}),
        _ => Err("Incorrect Puzzle Input".to_string())
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fst_passes_input_example(){
        let result = solve_fst("..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.".to_string());
        assert_eq!(result, "13");
    }

    #[test]
    fn snd_passes_input_example(){
        let result = solve_snd("..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.".to_string());
        assert_eq!(result, "43");
    }
}
