use crate::solution::Solution;

pub struct Day1;

impl Solution for Day1 {
    type Input = Vec<Vec<char>>;
    type ReturnType = u32;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.map(|line| line.trim().chars().collect()).collect()
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        let result : u32 = input.iter().map(|line| {
            let first_digit = line.iter().find(|&&c| c.is_ascii_digit()).expect("No digit found").to_digit(10).expect("that's no number!");
            let last_digit = line.iter().rev().find(|&&c| c.is_ascii_digit()).expect("No digit found").to_digit(10).expect("that's no number!");
            first_digit * 10 + last_digit
        }
        ).sum();

        result
    }
    fn second_part(&self, input: &Self::Input) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Day1;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day1.parse_input(lines);
        assert_eq!(Day1.first_part(&input), 142)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day1.parse_input(lines);
        assert_eq!(Day1.second_part(&input), 0)
    }
}
