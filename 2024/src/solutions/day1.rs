use std::collections::HashMap;

use crate::solution::Solution;

pub struct Day1;

impl Solution for Day1 {
    type Input = Vec<(u32, u32)>;
    type ReturnType = u32;
    const DAY : u32 = 1;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.map(|line| {
            let  mut split = line.split_whitespace().map(|s| s.parse::<u32>().unwrap());
            (split.next().unwrap(), split.next().unwrap())
        }).collect()
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        let mut left_list : Vec<u32> = input.iter().map(|pair| pair.0).collect();
        let mut right_list : Vec<u32> = input.iter().map(|pair| pair.1).collect();

        left_list.sort();
        right_list.sort();

        std::iter::zip(left_list,right_list).fold(0, |acc, (a,b)| acc + a.abs_diff(b))
    }
    fn second_part(&self, input: &Self::Input) -> u32 {
        let left_list : Vec<u32> = input.iter().map(|pair| pair.0).collect();
        let right_list : Vec<u32> = input.iter().map(|pair| pair.1).collect();

        let occurences = right_list.iter().fold(HashMap::new(), |mut acc, v| {
            acc.entry(v).and_modify(|v| *v += 1).or_insert(1);
            acc
        });

        left_list.iter().map(|v| v * occurences.get(v).unwrap_or(&0)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Day1;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    static INPUT_TEST_2 : &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day1.parse_input(lines);
        assert_eq!(Day1.first_part(&input), 11)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST_2.split('\n').map(|s| s.to_string());
        let input = Day1.parse_input(lines);
        assert_eq!(Day1.second_part(&input), 31);
    }
}
