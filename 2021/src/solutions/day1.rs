use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::solution::Solution;

pub struct Day1;

impl Solution for Day1 {
    type Input = Vec<i32>;

    fn parse_input(&self, buf_reader: BufReader<File>) -> Self::Input {
        buf_reader
            .lines()
            .map(|line| line.unwrap().trim().parse().expect("Not a number"))
            .collect()
    }

    fn first_part(&self, input: &Self::Input) -> i32 {
        input.windows(2).map(|w| (w[1] > w[0]) as i32).sum()
    }
    fn second_part(&self, input: &Self::Input) -> i32 {
        input
            .windows(3)
            .map(|w| w.iter().sum())
            .collect::<Vec<i32>>()
            .windows(2)
            .map(|w| (w[1] > w[0]) as i32)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Day1;
    use crate::solution::Solution;

    #[test]
    fn test_first_part() {
        assert_eq!(
            Day1.first_part(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            7
        );
    }

    #[test]
    fn test_second_part() {
        assert_eq!(
            Day1.second_part(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            5
        );
    }
}