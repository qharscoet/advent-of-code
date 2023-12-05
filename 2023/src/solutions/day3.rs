use std::collections::HashSet;

use crate::solution::Solution;

pub struct Day3;

type Engine = Vec<Vec<char>>;

fn get_symbols(map: &Engine) -> Vec<(usize, usize)> {
    let mut symbols: Vec<(usize, usize)> = Vec::new();
    for (line_idx, line) in map.iter().enumerate() {
        for (col_idx, &c) in line.iter().enumerate() {
            if !c.is_ascii_digit() && c != '.' {
                symbols.push((line_idx, col_idx));
            }
        }
    }

    symbols
}

fn get_neighbours(map: &Engine, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let (i, j) = pos;
    let y_range = i.saturating_sub(1)..=std::cmp::min(i + 1, map.len() - 1);
    let x_range = j.saturating_sub(1)..=std::cmp::min(j + 1, map[0].len() - 1);
    (y_range)
        .flat_map(|i| x_range.clone().map(move |j| (i, j)))
        .filter(|n| *n != pos)
        .collect()
}

fn get_parts_from_symbol(map: &Engine, symbol: (usize, usize)) -> Vec<u32> {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut parts = Vec::new();
    let (i, j) = symbol;
    for (i2, j2) in get_neighbours(map, (i, j)) {
        //println!("{}, {}", i2,j2);
        let r = map[i2][j2];
        let len = map[i2].len();
        if r.is_ascii_digit() {
            /* We go from a neighbour position and scan left and write to recompose the number, maintaining a set in order to not have duplicates */
            let number_left: String = map[i2]
                .iter()
                .rev()
                .skip(len - j2)
                .enumerate()
                .take_while(|(idx, c)| c.is_ascii_digit() && seen.insert((i2, j2 - idx - 1)))
                .map(|(_, c)| c)
                .collect();
            let number_right: String = map[i2]
                .iter()
                .skip(j2)
                .enumerate()
                .take_while(|(idx, c)| c.is_ascii_digit() && seen.insert((i2, j2 + idx)))
                .map(|(_, c)| c)
                .collect();
            let number_str = number_left.chars().rev().collect::<String>() + &number_right;
            if number_str.len() > 0 {
                let n: u32 = number_str.parse().unwrap();
                parts.push(n);
            }
        }
    }

    parts
}

impl Solution for Day3 {
    type Input = Engine;
    type ReturnType = u32;
    const DAY: u32 = 3;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.map(|line| line.chars().collect()).collect()
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        let symbols = get_symbols(input);

        symbols
            .iter()
            .flat_map(|s| get_parts_from_symbol(input, *s))
            .sum()
    }
    fn second_part(&self, input: &Self::Input) -> u32 {
        let symbols = get_symbols(input);

        symbols
            .iter()
            .map(|s| get_parts_from_symbol(input, *s))
            .filter(|parts| parts.len() == 2)
            .map(|parts| parts.iter().product::<u32>())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Day3;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    static INPUT_TEST_2: &str = "";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day3.parse_input(lines);
        assert_eq!(Day3.first_part(&input), 4361)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day3.parse_input(lines);
        assert_eq!(Day3.second_part(&input), 467835)
    }
}
