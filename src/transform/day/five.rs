use std::str::FromStr;

use crate::{extract::input::PuzzleInput, load::output::PuzzleOutput};

struct FreshnessChecker {
    fresh_ranges: Vec<i64> 
}

/// Helper class to check if an ingredient is fresh or not. We rely on binary
/// search to perform freshness checks in a more peformant way.
impl FreshnessChecker {
    pub fn new(fresh_ingredients_ranges: &Vec<(i64, i64)>) -> FreshnessChecker {
        let mut ret = FreshnessChecker { fresh_ranges: vec![] };
        ret.initialize(fresh_ingredients_ranges);
        ret
    }

    /// The ranges of fresh ingredients are added ot the internal fresh_range
    /// vector. If any two intervals have empty intersection we perform a merge.
    ///
    /// The resulting fresh_ranges vector is a vector of even length. Where every
    /// two values theres the beggining and the end of two given intervals.
    fn initialize(&mut self, fresh_ingredients_ranges: &Vec<(i64, i64)>) {
        for (l, r) in fresh_ingredients_ranges.iter() {
            let mut l_idx = self.fresh_ranges.partition_point(|&x| { x < *l });
            let mut r_idx = self.fresh_ranges.partition_point(|&x| { x <= *r });
            let is_left_merge_required = l_idx%2 == 1;
            let is_right_merge_required = r_idx%2 == 1;

            if is_left_merge_required {
                l_idx -= 1;
            }
            else {
                self.fresh_ranges.insert(l_idx, *l);
                r_idx += 1;
            }

            if !is_right_merge_required {
                self.fresh_ranges.insert(r_idx, *r);
            }

            // merging section
            while l_idx + 1 < r_idx {
                self.fresh_ranges.remove(l_idx + 1);
                r_idx -= 1;
            }
        }
    }

    /// Checks if an ingredient is fresh. If the ingredient belongs to the fresh_ranges
    /// vector it's either the beggining or the end of some interval. In other case the
    /// ingredient belongs to an interval (iff it's at an odd index i.e. ingredient \in
    /// [fresh_ranges[i-1], fresh_ranges[i]]) or it does not belong to an interval
    /// (iff it's even i.e. fresh_ranges[i-1] < ingredient < fresh_ranges[i] where
    /// it's between two intervals [fresh_ranges[i-2], fresh_ranges[i-1]] and
    /// [fresh_ranges[i], fresh_ranges[i+1]])
    fn is_fresh(&self, ingredient: i64) -> bool{
        let ingredient_idx = self.fresh_ranges.binary_search(&ingredient);
        match ingredient_idx {
            Ok(_) => true,
            Err(idx) => idx % 2 == 1,
        }
    }
}

fn decode_content_into_puzzle_input(content: String) -> (Vec<(i64,i64)>, Vec<i64>){
    let mut splitted_content = content.trim().split("\n\n");
    let ranges = splitted_content.next().unwrap_or_default();
    let ingredients = splitted_content.next().unwrap_or_default();
    
    let fresh_ingredients_ranges = ranges.split('\n').map(|x| {
        let mut beg_end = x.split('-');
        let beg = beg_end.next().expect("The range definition is empty").parse::<i64>().unwrap();
        let end = beg_end.next().expect("The range definition is incorrect").parse::<i64>().unwrap();
        (beg, end)
    }).collect();

    let available_ingredients = ingredients.split('\n').map(i64::from_str).map(Result::unwrap).collect();
    (fresh_ingredients_ranges, available_ingredients)
}
    

fn solve_fst(content: String) -> String {
    let (fresh_ingredients_ranges, available_ingredients) = decode_content_into_puzzle_input(content);

    let checker = FreshnessChecker::new(&fresh_ingredients_ranges);

    let count = available_ingredients.into_iter().filter(|&ingredient| {checker.is_fresh(ingredient)}).count();

    count.to_string()
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
    fn fst_passes_input_example(){
        let result = solve_fst("3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32".to_string()).to_string();
        assert_eq!("3", result);
    }

    #[test]
    fn decode_works_correctly(){
        let (ranges, ingredients) = decode_content_into_puzzle_input("3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32".to_string());
        assert_eq!(vec![(3,5), (10,14), (16,20), (12, 18)], ranges);
        assert_eq!(vec![1,5,8,11,17,32], ingredients);
    }

    #[test]
    fn fresh_ranges_constructor_initializes_correctly(){
        let ranges = vec![(3,5), (10,14), (16,20), (12, 18)];
        let ret = FreshnessChecker::new(&ranges);
        assert_eq!(vec![3,5,10,20], ret.fresh_ranges);

        let ranges = vec![(3,5), (10,14), (16,20), (6,7), (12, 18)];
        let ret = FreshnessChecker::new(&ranges);
        assert_eq!(vec![3,5,6,7,10,20], ret.fresh_ranges);

        let ranges = vec![(3,5), (10,14), (16,20), (5,7), (12, 18)];
        let ret = FreshnessChecker::new(&ranges);
        assert_eq!(vec![3,7,10,20], ret.fresh_ranges);

        let ranges = vec![(3,5), (10,14), (16,20), (9,10), (12, 18)];
        let ret = FreshnessChecker::new(&ranges);
        assert_eq!(vec![3,5,9,20], ret.fresh_ranges);
    }

    #[test]
    fn is_fresh_correctly_works_on_input(){
        let ranges = vec![(3,5), (10,14), (16,20), (12, 18)];
        let checker = FreshnessChecker::new(&ranges);

        assert!(!checker.is_fresh(1));
        assert!(checker.is_fresh(5));
        assert!(!checker.is_fresh(8));
        assert!(checker.is_fresh(11));
        assert!(checker.is_fresh(17));
        assert!(!checker.is_fresh(32));
    }
}
