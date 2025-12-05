
use crate::solution::Solution;

pub struct Day5;

impl Solution for Day5 {
    type Input = u32;
    type ReturnType = u32;
    const DAY : u32 = 5;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        0
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
       0
    }
    
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Day5;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "";


    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day5.parse_input(lines);
        assert_eq!(Day5.first_part(&input), 13)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day5.parse_input(lines);
        assert_eq!(Day5.second_part(&input), 43);
    }
}
