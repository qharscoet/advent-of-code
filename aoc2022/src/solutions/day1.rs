use std::vec;

use crate::solution::Solution;

pub struct Day1;

impl Solution for Day1 {
    type Input = Vec<i32>;
    type ReturnType = u32;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        vec![1,2,3]
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        0
    }
    fn second_part(&self, input: &Self::Input) -> u32 {
        0
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
            0
        );
    }

    #[test]
    fn test_second_part() {
        assert_eq!(
            Day1.second_part(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            0
        );
    }
}
