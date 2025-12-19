use itertools::Itertools;

fn parse_content_into_puzzle_input(content: String) -> Vec<Vec<u32>> {
    const RADIX: u32 = 10;
    let lines  = content.lines();
    lines.map(|line| {
        let digits = line.chars();
        digits.map(|x| {x.to_digit(RADIX).unwrap()}).collect::<Vec<u32>>()
    }).collect()
}

fn calculate_largest_joltage(battery: &Vec<u32>) -> u32 {
    let length_battery = battery.len();
    let range_to_find_fst_digit = &battery[..length_battery-1];

    // TODO: fix error: If several elements are equally maximum, the position of the last of them is returned.
    let first_digit_index = range_to_find_fst_digit.iter().position_max().unwrap();

    let range_to_find_snd_digit = &battery[first_digit_index+1..length_battery];

    let second_digit_index = first_digit_index + 1 + range_to_find_snd_digit.iter().position_max().unwrap();

    10 * battery[first_digit_index] + battery[second_digit_index]
}

pub fn solve_fst(content: String) -> String {
    let batteries = parse_content_into_puzzle_input(content);

    let joltages: Vec<u32> = batteries.iter().map(calculate_largest_joltage).collect();
    dbg!(&joltages);
    joltages.iter().sum::<u32>().to_string()
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
    fn correctly_calculates_joltage_first_example(){
        let battery = vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1];
        let result = calculate_largest_joltage(&battery);
        assert_eq!(result, 98)
    }

    #[test]
    fn correctly_calculates_joltage_second_example(){
        let battery = vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9];
        let result = calculate_largest_joltage(&battery);
        assert_eq!(result, 89)
    }

    #[test]
    fn correctly_calculates_joltage_third_example(){
        let battery = vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8];
        let result = calculate_largest_joltage(&battery);
        assert_eq!(result, 78)
    }

    #[test]
    fn correctly_calculates_joltage_fourth_example(){
        let battery = vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1];
        let result = calculate_largest_joltage(&battery);
        assert_eq!(result, 92)
    }

    
}
