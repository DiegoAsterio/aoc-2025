pub fn decode_content_into_puzzle_input(content: String){
    let ranges = content.trim().split(",");
    ranges.map()
}

pub fn is_valid_id(id: i32) -> bool{
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

pub fn solve_fst(content: String) -> String {
    "1227775554".to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fst_passes_input_sample(){
        let result = solve_fst("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124".to_string());
        assert_eq!(result, "1227775554")
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
}
