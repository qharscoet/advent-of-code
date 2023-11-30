use std::vec;

use crate::solution::Solution;

pub struct Day1;

impl Solution for Day1 {
    type Input = Vec<Vec<u32>>;
    type ReturnType = u32;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        let v : Vec<Vec<u32>> = lines.fold(vec![vec![]], |mut acc, s| { match &s[..] {
            "" => acc.push(vec![]),
            _ => acc.last_mut().unwrap().push(s.parse().expect("Not a number !"))
        } 
        acc} );
        v
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        input.iter().map(|group| group.iter().sum()).max().unwrap_or_default()
    }
    fn second_part(&self, input: &Self::Input) -> u32 {
        let mut sums : Vec<u32> = input.iter().map(|group| group.iter().sum()).collect();
        sums.sort();
        sums.iter().rev().take(3).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Day1;
    use crate::solution::Solution;

    #[test]
    fn test_first_part() {
        assert_eq!(
            Day1.first_part(&vec![
                vec![1000,2000,3000],
                vec![4000],
                vec![5000,6000],
                vec![7000,8000,9000],
                vec![10000]
            ]),
            24000
        );
    }

    #[test]
    fn test_second_part() {
        assert_eq!(
            Day1.second_part(&vec![
                vec![1000,2000,3000],
                vec![4000],
                vec![5000,6000],
                vec![7000,8000,9000],
                vec![10000]
            ]),
            45000
        );
    }
}
