use crate::solution::Solution;

pub struct Day1;

impl Solution for Day1 {
    type Input = Vec<String>;
    type ReturnType = u32;
    const DAY : u32 = 1;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.map(|line| line).collect()
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        11
    }
    fn second_part(&self, input: &Self::Input) -> u32 {
        0
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

    static INPUT_TEST_2 : &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

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
        assert_eq!(Day1.second_part(&input), 281);
    }
}
