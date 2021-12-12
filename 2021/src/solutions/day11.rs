use crate::solution::Solution;

pub struct Day11;

type Grid = Vec<Vec<u32>>;

struct OctopusGrid {
	octopuses : Grid,
	flashes: u32,
}


fn get_neighbours(grid: &Grid, pos: (usize, usize)) -> Vec<(usize,usize)> {
	let (i, j) = pos;
	let y_range = i.saturating_sub(1)..=std::cmp::min(i + 1, grid.len() - 1);
	let x_range = j.saturating_sub(1)..=std::cmp::min(j + 1, grid[0].len() - 1);
	(y_range).flat_map(|i| x_range.clone().map(move |j| (i,j))).filter(|n| *n != pos).collect()
}

impl OctopusGrid {
	fn flash(&mut self, pos:(usize, usize)) {
		let octo = &mut self.octopuses[pos.0][pos.1];
		*octo += 1;
		if  *octo == 10 {
			self.flashes += 1;
			for n in get_neighbours(&self.octopuses, pos) {
				self.flash(n);
			}
		}
	}

	fn step(&mut self) {
		for i in 0..self.octopuses.len() {
			for j in 0..self.octopuses[0].len() {
				self.flash((i,j));
			}
		}

		for octo in self.octopuses.iter_mut().flatten() {
			if *octo > 9 {
				*octo = 0;
			}
		}
	}
}


impl Solution for Day11 {
	type Input = Grid;
	type ReturnType = u32;

	fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
		lines
			.map(|line| {
				line.chars()
					.map(|c| (c as u8 - '0' as u8) as u32)
					.collect::<Vec<u32>>()
			})
			.collect()
	}

	fn first_part(&self, input: &Self::Input) -> u32 {
		let mut grid = OctopusGrid{octopuses : input.clone(), flashes:0};
		for _ in 0..100 {
			grid.step();
		}

		grid.flashes
	}

	fn second_part(&self, input: &Self::Input) -> u32 {
		let mut grid = OctopusGrid{octopuses : input.clone(), flashes:0};

		let mut step = 0;
		loop {
			step += 1;
			grid.step();
			if grid.octopuses.iter().flatten().all(|o| *o == 0) {
				break step;
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::solution::Solution;

	fn test_input_to_string_iter() -> Vec<String> {
		vec![
			"5483143223",
			"2745854711",
			"5264556173",
			"6141336146",
			"6357385478",
			"4167524645",
			"2176841721",
			"6882881134",
			"4846848554",
			"5283751526",
		]
		.iter()
		.map(|s| s.to_string())
		.collect()
	}

	#[test]
	fn test_first_part() {
		let input = Day11.parse_input(test_input_to_string_iter().into_iter());
		assert_eq!(Day11.first_part(&input), 1656)
	}

	#[test]
	fn test_second_part() {
		let input = Day11.parse_input(test_input_to_string_iter().into_iter());
		assert_eq!(Day11.second_part(&input), 195)
	}
}
