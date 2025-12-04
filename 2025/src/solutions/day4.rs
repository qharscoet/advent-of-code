use std::vec;

use crate::solution::Solution;

pub struct Day4;

type Grid = Vec<Vec<bool>>;


fn get_neighbours(grid: &Grid, pos: (usize, usize)) -> Vec<(usize,usize)> {
	let (i, j) = pos;
	let y_range = i.saturating_sub(1)..=std::cmp::min(i + 1, grid.len() - 1);
	let x_range = j.saturating_sub(1)..=std::cmp::min(j + 1, grid[0].len() - 1);
	(y_range).flat_map(|i| x_range.clone().map(move |j| (i,j))).filter(|n| *n != pos).collect()
}


impl Solution for Day4 {
    type Input = Grid;
    type ReturnType = u32;
    const DAY : u32 = 4;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines
			.map(|line| {
				line.chars()
					.map(|c| c == '@')
					.collect()
			})
			.collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        let pos_iter = (0..input.len()).flat_map(|i| (0..input[0].len()).map(move |j| (i,j)));
        pos_iter.filter(|&pos| input[pos.0][pos.1] && get_neighbours(input, pos).iter().filter(|&&pos2| input[pos2.0][pos2.1]).count() < 4).count() as u32
    }
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        let mut grid = input.clone();
        let positions = (0..grid.len()).flat_map(|i| (0..grid[0].len()).map(move |j| (i,j))).collect::<Vec<_>>();
        
        let mut removed = 0;

        loop {
            let removable : Vec<_> = positions.iter().filter(|&&pos| grid[pos.0][pos.1] && get_neighbours(&grid, pos).iter().filter(|&&pos2| grid[pos2.0][pos2.1]).count() < 4).collect();
            let to_remove_count = removable.len();
            for pos in removable {
                grid[pos.0][pos.1] = false;
            }

            removed += to_remove_count as u32;
            if to_remove_count == 0 {
                break;
            }
        }

        removed
    }
}

#[cfg(test)]
mod tests {
    use super::Day4;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";



    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day4.parse_input(lines);
        assert_eq!(Day4.first_part(&input), 13)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day4.parse_input(lines);
        assert_eq!(Day4.second_part(&input), 43);
    }
}
