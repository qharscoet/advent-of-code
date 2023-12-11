use std::collections::HashSet;

use crate::solution::Solution;

pub struct Day11;

type Coord = (usize,usize);

fn expand(universe: &HashSet<Coord>, expansion : u32) -> HashSet<Coord> {
    let rows : HashSet<usize> = universe.iter().map(|p| p.0).collect();
    let cols : HashSet<usize> = universe.iter().map(|p| p.1).collect();

    universe.iter().map(|p| {
        let count_x = rows.iter().filter(|&&r| r < p.0).count();
        let count_y = cols.iter().filter(|&&c| c < p.1).count();
        ((count_x + (p.0 - count_x) * (expansion as usize)),
        (count_y+ (p.1 - count_y) * (expansion as usize)))
    }).collect()
}

fn get_result(universe: &HashSet<Coord>) -> <Day11 as Solution>::ReturnType {
    let prod : HashSet<_> = universe.iter().flat_map(|p1| universe.iter().map(move |p2| if p1 < p2 {(p1,p2)} else {(p2,p1)})).filter(|(p1,p2)| p1 != p2).collect();
    prod.iter().map(|(p1,p2)| {
        let diff_x = if p2.0 > p1.0 { p2.0 - p1.0} else { p1.0 - p2.0};
        let diff_y = if p2.1 > p1.1 { p2.1 - p1.1} else { p1.1 - p2.1};

        diff_x + diff_y
    }).sum::<usize>() as u64
}

impl Solution for Day11 {
    type Input = HashSet<Coord>;
    type ReturnType = u64;
    const DAY : u32 = 11;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.enumerate()
            .flat_map(|(idx_l, line)| {
                line
                    .trim()
                    .chars()
                    .enumerate()
                    .filter_map(|(idx_c, c)| {
                        if c == '#' {
                            Some((idx_l, idx_c))
                        } else {
                            None
                        }
                    })
                    .collect::<HashSet<Coord>>()
            }).collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        let expanded = expand(&input, 2);
        get_result(&expanded)
    }
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        let expanded = expand(&input, 1000000);
        get_result(&expanded)
    }
}

#[cfg(test)]
mod tests {
    use super::Day11;
    use crate::{solution::Solution, solutions::day11::{expand, get_result}};

    static INPUT_TEST: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day11.parse_input(lines);
        assert_eq!(Day11.first_part(&input), 374)
    }

    #[test]
    fn test_second_part_1() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day11.parse_input(lines);
        let expanded = expand(&input, 10);
        assert_eq!(get_result(&expanded), 1030)
    }

    #[test]
    fn test_second_part_2() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day11.parse_input(lines);
        let expanded = expand(&input, 100);
        assert_eq!(get_result(&expanded), 8410)
    }
}
