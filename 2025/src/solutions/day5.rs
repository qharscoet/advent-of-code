
use std::ops::RangeInclusive;

use crate::solution::Solution;

pub struct Day5;

#[derive(Debug)]
pub struct Ingredients {
    ranges : Vec<RangeInclusive<u64>>,
    fresh_list : Vec<u64>,
}

impl Solution for Day5 {
    type Input = Ingredients;
    type ReturnType = usize;
    const DAY : u32 = 5;

    fn parse_input(&self, mut lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        let ranges : Vec<_> = lines.by_ref().take_while(|line| line != "")
            .map(|line| {
                let (x, y) = line.split_once("-").unwrap_or_default();
                
                    x.parse::<u64>().unwrap_or_default()..=
                    y.parse::<u64>().unwrap_or_default()
                
            })
            .collect();

        let list : Vec<_> = lines.flat_map(|line| line.parse::<u64>().ok()).collect();
        
        Ingredients{ranges : ranges, fresh_list : list}
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        input.fresh_list.iter().filter(|&item| {
            input.ranges.iter().any(|range| range.contains(item))
        }).count()
    }
    
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Day5;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";


    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day5.parse_input(lines);
        assert_eq!(Day5.first_part(&input), 3)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day5.parse_input(lines);
        assert_eq!(Day5.second_part(&input), 43);
    }
}
