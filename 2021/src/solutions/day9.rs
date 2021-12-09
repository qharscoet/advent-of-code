use crate::solution::Solution;

use std::collections::HashSet;

pub struct Day9;

type Heightmap = Vec<Vec<u8>>;

fn get_neighbours(map: &Heightmap, pos: (usize, usize)) -> Vec<(usize,usize)> {
    let (i, j) = pos;
    let y_range = i.saturating_sub(1)..=std::cmp::min(i + 1, map.len() - 1);
    let x_range = j.saturating_sub(1)..=std::cmp::min(j + 1, map[0].len() - 1);
    (y_range).flat_map(|i| x_range.clone().map(move |j| (i,j))).filter(|(i,j)| *i == pos.0 || *j == pos.1).collect()

}

fn get_low_points(map: &Heightmap) -> Vec<(usize, usize)> {
    map.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(j, v)| *v == &get_neighbours(map, (i, *j)).iter().map(|(n_i,n_j)| map[*n_i][*n_j]).min().unwrap())
                .map(|(j, _v)| (i, j))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect()
}

fn get_basin(map: &Heightmap, pos: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut basin = get_neighbours(map, pos)
        .iter()
        .filter(|(i, j)| map[*i][*j] != 9 && map[*i][*j] > map[pos.0][pos.1])
        .map(|pos| *pos)
        .fold(HashSet::new(), |acc, pos| 
            acc.union(&get_basin(map, pos)).map(|&v| v).collect()
        );
    basin.insert(pos);
    basin
}

impl Solution for Day9 {
    type Input = Heightmap;
    type ReturnType = u32;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines
            .map(|line| {
                line.chars()
                    .map(|c| c as u8 - '0' as u8)
                    .collect::<Vec<u8>>()
            })
            .collect()
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        get_low_points(input)
            .iter()
            .map(|(i, j)| (input[*i][*j] + 1) as u32)
            .sum()
    }

    fn second_part(&self, input: &Self::Input) -> u32 {
        let mut sizes = get_low_points(input)
            .iter()
            .map(|pos| get_basin(input, *pos).len() as u32)
            .collect::<Vec<u32>>();
        sizes.sort_by(|a, b| b.cmp(a));
        sizes[0..3].iter().product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;

    fn test_input_to_string_iter() -> Vec<String> {
        vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn test_first_part() {
        let input = Day9.parse_input(test_input_to_string_iter().into_iter());
        assert_eq!(Day9.first_part(&input), 15)
    }

    #[test]
    fn test_second_part() {
        let input = Day9.parse_input(test_input_to_string_iter().into_iter());
        assert_eq!(Day9.second_part(&input), 1134)
    }
}
