use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::solution::Solution;

pub struct Day2;

impl Solution for Day2 {
    type Input = Vec<i32>;

    fn parse_input(&self, buf_reader: BufReader<File>) -> Self::Input {
        buf_reader
            .lines()
            .map(|line| line.unwrap().trim().parse().expect("Not a number"))
            .collect()
    }

    fn first_part(&self, _input: &Self::Input) -> i32 {
        42
    }
    fn second_part(&self, _input: &Self::Input) -> i32 {
        42
    }
}

#[cfg(test)]
mod tests {
    use super::Day2;
    use crate::solution::Solution;

    #[test]
    fn test_first_part() {
        assert_eq!(
            Day2.first_part(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            42
        );
    }

    #[test]
    fn test_second_part() {
        assert_eq!(
            Day2.second_part(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            42
        );
    }
}