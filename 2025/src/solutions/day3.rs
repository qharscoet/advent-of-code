use std::vec;

use crate::solution::Solution;

pub struct Day3;

type Bank = Vec<u32>;


fn get_bank_joltage(bank : &Bank, nb : usize) -> u64 {
    
    let mut maxes = vec![0; nb];

    for (idx,&v) in bank[0..bank.len()].iter().enumerate() {
        let start = idx.saturating_sub(bank.len() - nb);
        for i in start..nb {
            if v > maxes[i] {
                for j in i+1..nb {
                    maxes[j] = 0;
                }
                maxes[i] = v;
                break;
            }
        }
    }

    maxes.iter().rev().enumerate().fold(0, |acc, (i, &v)| acc + v as u64 * 10u64.pow(i as u32))
}

impl Solution for Day3 {
    type Input = Vec<Bank>;
    type ReturnType = u64;
    const DAY : u32 = 3;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.map(|line| {
            line.chars().flat_map(|c| c.to_digit(10)).collect()
        }).collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        input.iter().map(|bank| get_bank_joltage(bank,2)).sum()
    }
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        input.iter().map(|bank| get_bank_joltage(bank, 12)).sum()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::Day3;
    use crate::{solution::Solution, solutions::day3::{get_bank_joltage}};

    static INPUT_TEST: &str = "987654321111111
811111111111119
234234234234278
818181911112111";


    #[test]
    fn test_joltage()
    {
        let v = vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1];
        let v2 = vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9];

        assert_eq!(get_bank_joltage(&v,2), 98);
        assert_eq!(get_bank_joltage(&v2,2), 89);
    }

    #[test]
    fn test_joltage2()
    {
        let v = vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1];
        let v2 = vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9];
        let v3 = vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8];
        let v4 = vec![8,1,1,1,1,1,1,1,1,1,1,7,1,1,9];

        assert_eq!(get_bank_joltage(&v, 12), 987654321111);
        assert_eq!(get_bank_joltage(&v2, 12), 811111111119);
        assert_eq!(get_bank_joltage(&v3, 12), 434234234278);
        assert_eq!(get_bank_joltage(&v4, 4), 8719);
    }

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day3.parse_input(lines);
        assert_eq!(Day3.first_part(&input), 357)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day3.parse_input(lines);
        assert_eq!(Day3.second_part(&input), 3121910778619);
    }
}
