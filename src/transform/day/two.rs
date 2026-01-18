use crate::{extract::input::PuzzleInput, load::output::PuzzleOutput};

fn decode_content_into_puzzle_input(content: String) -> Vec<(i64, i64)>{
    let ranges = content.trim().split(",");
    ranges.map(|x| {
        let mut ys = x.trim().split("-");
        let y0 = ys.next().expect("No first item found").parse::<i64>().expect("Failed to parse into integer.");
        let y1 = ys.next().expect("No second item found").parse::<i64>().expect("Failed to parse into integer.");
        (y0, y1)
    }).collect()
}

fn is_valid_id(id: i64) ->  bool{
    let id = id.to_string();
    let length = id.len();

    if !length.is_multiple_of(2) {
        true
    } else {
        let chars: Vec<char> = id.chars().collect();
        for i in 0..(length/2) {
            if chars[i] != chars[i+length/2] {
                return true
            }
        }
        false
    }
}

fn solve_fst(content: String) -> PuzzleOutput {
    let mut ret = 0;
    for (y0, y1) in decode_content_into_puzzle_input(content) {
        for y in y0..y1+1 {
            if !is_valid_id(y){
                ret += y;
            }
        }
    }
    PuzzleOutput{
        result: ret.to_string()
    }
}

fn can_build_id_by_concatenating_seq(id: &str, seq: &str) -> bool{
    let id_chars: Vec<char> = id.chars().collect();
    let seq_chars: Vec<char> = seq.chars().collect();

    let len_id = id.len();
    let len_chars = seq.len();
        
    if !len_id.is_multiple_of(len_chars){
        false
    } else {
        for i in 0..len_id {
            if id_chars[i] != seq_chars[i%len_chars] {
                return false
            }
        }
        true
    }
}

fn contains_repeated_sequence(id: i64) -> bool{
    let id = id.to_string();
    let length = id.len();

    for r in 1..(length/2)+1 {
        let seq = &id[..r];
        if can_build_id_by_concatenating_seq(&id, seq) {
            return true
        }
        
    }    
    false
}

fn solve_snd(content: String) -> PuzzleOutput {
    let mut ret = 0;
    for (y0, y1) in decode_content_into_puzzle_input(content) {
        for y in y0..y1+1 {
            if contains_repeated_sequence(y){
                ret += y;
            }
        }
    }
    PuzzleOutput {
        result: ret.to_string()
    }
}

pub fn solve(input: &PuzzleInput) -> Result<PuzzleOutput, String> {
    match input {
        PuzzleInput {
            day: 2,
            iteration: 1,
            text
        } => Ok(solve_fst(text.to_string())),
        PuzzleInput {
            day: 2,
            iteration: 2,
            text
        } => Ok(solve_snd(text.to_string())),
        _ => Err("Incorrect Puzzle Input".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fst_passes_input_sample(){
        let result = solve_fst("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124".to_string());
        assert_eq!(result.result, "1227775554")
    }

    #[test]
    fn valid_id_odd_digits_test(){
        let result = is_valid_id(123);
        assert!(result);
    }

    #[test]
    fn valid_id_even_digits_test(){
        let result = is_valid_id(1234);
        assert!(result);
    }

    #[test]
    fn invalid_id_two_fives_test(){
        let result = is_valid_id(55);
        assert!(!result);
    }

    #[test]
    fn snd_passess_input_sample(){
        let result = solve_snd("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124".to_string());
        assert_eq!(result.result, "4174379265")
    }

    #[test]
    fn valid_id_for_snd_exercise(){
        let result = contains_repeated_sequence(12);
        assert!(!result);
    }

    #[test]
    fn invalid_id_repeats_two_sequences(){
        let result = contains_repeated_sequence(11);
        assert!(result)
    }

    #[test]
    fn invalid_id_repeats_three_sequences(){
        let result = contains_repeated_sequence(121212);
        assert!(result)
    }

}
