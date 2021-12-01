use std::fs::File;
use std::io::BufReader;

use crate::solution::Solution;

pub struct Day1;

impl Solution for Day1 {
    type Input = i32;

    fn parse_input(&self, _r: BufReader<File>) -> Self::Input {
       42
    }

    fn first_part(&self, _input: &Self::Input) -> i32{
        42
    }
    fn second_part(&self, _input: &Self::Input) -> i32{
        42
    }
}


#[cfg(test)]
mod tests {
    use super::Day1;
    use crate::solution::Solution;

    #[test]
    fn test_first_part() {
        assert_eq!(Day1{}.first_part(&0), 42);
    }

    #[test]
    fn test_second_part() {
        assert_eq!(Day1{}.second_part(&0), 42);
    }
}