use std::ops::RangeInclusive;

use crate::solution::Solution;

pub struct Day4;

type ElfPair = (RangeInclusive<u32>, RangeInclusive<u32>);


impl Solution for Day4 {
    type Input = Vec<ElfPair>;
    type ReturnType = u32;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.map(|l| {
			//Could have used regex 🤔
			let (p1,p2) = l.split_once(',').unwrap_or_default();
			let r1 = p1.split_once('-').unwrap_or_default();
			let r2 = p2.split_once('-').unwrap_or_default();

			(r1.0.parse().unwrap_or_default()..=r1.1.parse().unwrap_or_default(),
			 r2.0.parse().unwrap_or_default()..=r2.1.parse().unwrap_or_default())

		})
		.collect()
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        input.iter().filter(|(r1, r2)|
			(r1.start() <= r2.start() && r1.end() >= r2.end()) || (r2.start() <= r1.start() && r2.end() >= r1.end()))
			.count().try_into().unwrap_or_default()
    }
    fn second_part(&self, input: &Self::Input) -> u32 {
        input.iter().filter(|(r1, r2)|
			(r2.start() <= r1.end() && r2.end() >= r1.start()))
			.count().try_into().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::Day4;
    use crate::solution::Solution;

	static INPUT_TEST : &str =
"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day4.parse_input(lines);
		assert_eq!(Day4.first_part(&input),
        2)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day4.parse_input(lines);
		assert_eq!(Day4.second_part(&input),
            4)
    }
}