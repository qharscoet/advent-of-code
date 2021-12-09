use crate::solution::Solution;

use std::collections::HashSet;

pub struct Day8;

type Digit = HashSet<char>;
pub struct Entry {
	digits: Vec<Digit>,
	output: Vec<Digit>,
}

fn get_digit_map() -> [HashSet<char>; 10] {
	[
		"abcefg".chars().collect(),
		"cf".chars().collect(),
		"acdeg".chars().collect(),
		"acdfg".chars().collect(),
		"bcdf".chars().collect(),
		"abdfg".chars().collect(),
		"abdefg".chars().collect(),
		"acd".chars().collect(),
		"abcdefg".chars().collect(),
		"abcdfg".chars().collect(),
	]
}

impl Solution for Day8 {
	type Input = Vec<Entry>;
	type ReturnType = u32;

	fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
		lines.map(|line| {
			let (digits, output) = line.split_once(" | ").unwrap_or_default();
			let digits = digits.split(" ").map(|d| d.chars().collect::<Digit>()).collect();
			let output = output.split(" ").map(|d| d.chars().collect::<Digit>()).collect();

			Entry { digits:digits, output:output}
		}).collect()
	}

	fn first_part(&self, input: &Self::Input) -> u32 {
		let digit_map = get_digit_map();
		input
			.iter()
			.map(|entry| {
				entry
					.output
					.iter()
					.filter(|digit|
						digit_map
							.iter()
							.filter(|set| set.len() == digit.len())
							.count() == 1
					)
					.count() as u32
			})
			.sum::<u32>() as u32
	}

	fn second_part(&self, input: &Self::Input) -> u32 {

		input.iter().map(|entry| {
			let one = entry.digits.iter().find(|d| d.len() == 2).unwrap();
			let seven = entry.digits.iter().find(|d| d.len() == 3).unwrap();
			let four = entry.digits.iter().find(|d| d.len() == 4).unwrap();
			let eight = entry.digits.iter().find(|d| d.len() == 7).unwrap();

			let seven_plus_four = seven.union(&four).map(|&c| c).collect::<Digit>();
			let nine = entry.digits.iter().find(|d| d.len() == 6 &&  d.difference(&seven_plus_four).count() == 1).unwrap();

			let zero_six : Vec<Digit> = entry.digits.iter().filter(|d| d.len() == 6 && eight.difference(d).count() == 1 && *d != nine).map(|d| d.clone()).collect();

			let zero = zero_six.iter().find(|d| eight.difference(d).map(|&c| c).collect::<Digit>().difference(one).count() == 1).unwrap();
			let six = zero_six.iter().find(|&d| d != zero).unwrap();
			let three = entry.digits.iter().find(|d| d.len() == 5 && d.is_superset(one)).unwrap();
			let two = entry.digits.iter().find(|d| d.len() == 5 && d.difference(nine).count() == 1).unwrap();
			let five = entry.digits.iter().find(|d| d.len() == 5 && d.difference(six).count() == 0).unwrap();

			let digits = [ zero, one, two, three, four, five, six, seven, eight, nine];

			entry.output.iter().map(|o| (digits.iter().position(|d| *d == o).unwrap() as u8 + 48u8) as char).collect::<String>().parse::<u32>().unwrap_or_default()

		}).sum()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::solution::Solution;

	fn test_input_to_string_iter() -> Vec<String> {
		vec![
"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
"edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
"fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
"fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
"aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
"fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
"dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
"bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
"egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
"gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
// "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
].iter().map(|s| s.to_string()).collect()
	}

	#[test]
	fn test_first_part() {
		let input = Day8.parse_input(test_input_to_string_iter().into_iter());
		assert_eq!(Day8.first_part(&input), 26)
	}

	#[test]
	fn test_second_part() {
		let input = Day8.parse_input(test_input_to_string_iter().into_iter());
		assert_eq!(Day8.second_part(&input), 61229)
	}
}
