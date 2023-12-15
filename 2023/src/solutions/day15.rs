use crate::solution::Solution;

pub struct Day15;

fn hash(s: &str) -> u8 {
    s.chars().fold(0, |acc, c| {
        let code = c as u8;
        let mut val = acc as u32;
        val += code as u32;
        val *= 17;
        val % 256
    }) as u8
}

impl Solution for Day15 {
    type Input = Vec<String>;
    type ReturnType = u32;
    const DAY: u32 = 15;

    fn parse_input(&self, mut lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.to_string())
            .collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        input.iter().map(|s| hash(s) as u32).sum()
    }
    fn second_part(&self, _input: &Self::Input) -> Self::ReturnType {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Day15;
    use crate::{solution::Solution, solutions::day15::hash};

    static INPUT_TEST: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        assert_eq!(hash("rn=1"), 30);
    }
    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day15.parse_input(lines);
        assert_eq!(Day15.first_part(&input), 1320)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day15.parse_input(lines);
        assert_eq!(Day15.second_part(&input), u32::MAX)
    }
}
