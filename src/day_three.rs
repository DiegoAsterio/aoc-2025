use itertools::Itertools;

fn parse_content_into_puzzle_input(content: String) -> Vec<Vec<u32>> {
    const RADIX: u32 = 10;
    let lines  = content.lines();
    lines.map(|line| {
        let digits = line.chars();
        digits.map(|x| {x.to_digit(RADIX).unwrap()}).collect::<Vec<u32>>()
    }).collect()
}

fn index_to_leftmost_biggest_digit(xs: &[u32]) -> usize {
    let range_size = xs.len();

    let steps_back_from_the_end_to_the_fst_biggest_digit = xs.iter().rev().position_max().unwrap();
    let last_index = range_size - 1;

    last_index - steps_back_from_the_end_to_the_fst_biggest_digit
}

fn calculate_largest_joltage_with_n_batteries(bank: &Vec<u32>, n: usize) -> u64 {
    let length_bank = bank.len();

    let mut left = 0;
    let mut joltage: u64 = 0;

    for i in 0..n {
        let range_containing_ith_index = &bank[left..length_bank - n + 1 + i];
        let ith_index = left + index_to_leftmost_biggest_digit(
            range_containing_ith_index
        );
        joltage = 10*joltage + u64::from(bank[ith_index]);
        left = ith_index + 1;
    }
    joltage
}

pub fn solve_for_n_batteries_turned_on(content: String, n: usize) -> String {
    let batteries = parse_content_into_puzzle_input(content);

    let joltages: Vec<u64> = batteries.iter().map(|x| calculate_largest_joltage_with_n_batteries(x, n)).collect();
    joltages.iter().sum::<u64>().to_string()
}

pub fn solve_fst(content: String) -> String {
    solve_for_n_batteries_turned_on(content, 2)
}

pub fn solve_snd(content: String) -> String {
    solve_for_n_batteries_turned_on(content, 12)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fst_passes_input_example(){
        let result = solve_fst("987654321111111\n811111111111119\n234234234234278\n818181911112111".to_string());
        assert_eq!(result, "357".to_string());
    }

    #[test]
    fn fst_correctly_calculates_joltage_first_example(){
        let bank = vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1];
        let result = calculate_largest_joltage_with_n_batteries(&bank, 2);
        assert_eq!(result, 98)
    }

    #[test]
    fn fst_correctly_calculates_joltage_second_example(){
        let bank = vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9];
        let result = calculate_largest_joltage_with_n_batteries(&bank, 2);
        assert_eq!(result, 89)
    }

    #[test]
    fn fst_correctly_calculates_joltage_third_example(){
        let bank = vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8];
        let result = calculate_largest_joltage_with_n_batteries(&bank, 2);
        assert_eq!(result, 78)
    }

    #[test]
    fn fst_correctly_calculates_joltage_fourth_example(){
        let bank = vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1];
        let result = calculate_largest_joltage_with_n_batteries(&bank, 2);
        assert_eq!(result, 92)
    }

    #[test]
    fn snd_passes_input_example(){
        let result = solve_snd("987654321111111\n811111111111119\n234234234234278\n818181911112111".to_string());
        assert_eq!(result, "3121910778619".to_string());
    }

    #[test]
    fn snd_correctly_calculates_joltage_first_example(){
        let bank = vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1];
        let result = calculate_largest_joltage_with_n_batteries(&bank, 12);
        assert_eq!(result, 987654321111)
    }

    #[test]
    fn snd_correctly_calculates_joltage_second_example(){
        let bank = vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9];
        let result = calculate_largest_joltage_with_n_batteries(&bank, 12);
        assert_eq!(result, 811111111119)
    }

    #[test]
    fn snd_correctly_calculates_joltage_third_example(){
        let bank = vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8];
        let result = calculate_largest_joltage_with_n_batteries(&bank, 12);
        assert_eq!(result, 434234234278)
    }

    #[test]
    fn snd_correctly_calculates_joltage_fourth_example(){
        let bank = vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1];
        let result = calculate_largest_joltage_with_n_batteries(&bank, 12);
        assert_eq!(result, 888911112111)
    }
    
}
