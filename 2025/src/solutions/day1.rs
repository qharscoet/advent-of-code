use crate::solution::Solution;

pub struct Day1;

impl Solution for Day1 {
    type Input = u32;
    type ReturnType = u32;
    const DAY : u32 = 1;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        todo!()
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        todo!()
    }
    fn second_part(&self, input: &Self::Input) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day1;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "";

    static INPUT_TEST_2 : &str = "";

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
        assert_eq!(Day1.second_part(&input), 31);
    }
}
