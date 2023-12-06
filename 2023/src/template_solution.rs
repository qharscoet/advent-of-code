use crate::solution::Solution;

pub struct Day0;

impl Solution for Day0 {
    type Input = Vec<String>;
    type ReturnType = u32;
    const DAY : u32 = 0;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.collect()
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
    use super::Day0;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "";

    static INPUT_TEST_2 : &str = "";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day0.parse_input(lines);
        assert_eq!(Day0.first_part(&input), u32::MAX)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST_2.split('\n').map(|s| s.to_string());
        let input = Day0.parse_input(lines);
        assert_eq!(Day0.second_part(&input), u32::MAX)
    }
}
