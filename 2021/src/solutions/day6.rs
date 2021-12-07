use crate::solution::Solution;

// use ndarray::{arr2, Array2};
use nalgebra::SMatrix;

pub struct Day6;

type Matrix9x9 = SMatrix<u64, 9, 9>;
type Matrix1x9 = SMatrix<u64, 1, 9>;

fn run_for_days(input: &[u64; 9], n: u32) -> u64 {
	let matrix : Matrix9x9 =  Matrix9x9::from_row_slice(&[ 0, 0, 0, 0, 0, 0, 1, 0, 1,
															1, 0, 0, 0, 0, 0, 0, 0, 0,
															0, 1, 0, 0, 0, 0, 0, 0, 0,
															0, 0, 1, 0, 0, 0, 0, 0, 0,
															0, 0, 0, 1, 0, 0, 0, 0, 0,
															0, 0, 0, 0, 1, 0, 0, 0, 0,
															0, 0, 0, 0, 0, 1, 0, 0, 0,
															0, 0, 0, 0, 0, 0, 1, 0, 0,
															0, 0, 0, 0, 0, 0, 0, 1, 0]);

	let mut fishes = Matrix1x9::from_row_slice(input);

	for _ in 0..n {
		fishes *= matrix;
	}

	fishes.sum()
}

impl Solution for Day6 {
	type Input = [u64; 9];
	type ReturnType = u64;

	fn parse_input(&self, mut lines: impl Iterator<Item = std::string::String>) -> Self::Input {
		let mut fish_count = [0u64; 9];
		for n in lines.next().unwrap_or_default().split(',') {
			fish_count[n.parse::<usize>().unwrap_or_default()] += 1;
		}

		println!("{:?}", fish_count);

		fish_count
	}

	fn first_part(&self, input: &Self::Input) -> u64 {
		run_for_days(&input, 80)
	}

	fn second_part(&self, input: &Self::Input) -> u64 {
		run_for_days(&input, 256)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::solution::Solution;

	fn test_input_to_string_iter() -> Vec<String> {
		vec!["3,4,3,1,2"].iter().map(|s| s.to_string()).collect()
	}

	#[test]
	fn test_first_part() {
		let input = Day6.parse_input(test_input_to_string_iter().into_iter());
		assert_eq!(Day6.first_part(&input), 5934)
	}

	#[test]
	fn test_second_part() {
		let input = Day6.parse_input(test_input_to_string_iter().into_iter());
		assert_eq!(Day6.second_part(&input) as u64, 26984457539u64)
	}
}
