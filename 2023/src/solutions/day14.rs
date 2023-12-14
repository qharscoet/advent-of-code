use std::collections::HashMap;

use crate::solution::Solution;

pub struct Day14;

// #[derive(Clone, Copy)]
// #[derive(PartialEq, Eq)]
// pub enum Rock {
//     Round,
//     Block,
//     Empty
// }

enum Direction {
    North,
    East,
    South,
    West,
}

type Rock = char;

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|row| row[i]).collect())
        .collect()
}

fn slide_rocks(rocks: &Vec<Vec<Rock>>, dir: Direction) -> Vec<Vec<Rock>> {
    let needs_trans = matches!(dir, Direction::North | Direction::South);
    let left = matches!(dir, Direction::North | Direction::West);

    let dish = if needs_trans {
        transpose(&rocks)
    } else {
        rocks.clone()
    };

    let slided: Vec<_> = dish
        .iter()
        .map(|line| {
            line.split(|&r| r == '#')
                .map(|g| {
                    let total = g.len();
                    let count = g.iter().filter(|&&r| r == 'O').count();
                    let rocks = vec!['O'; count];
                    let remain = vec!['.'; total - count];
                    // if left { rocks.append(&mut remain)} else { remain.append(&mut rocks)}.
                    if left {
                        rocks.iter().chain(remain.iter())
                    } else {
                        remain.iter().chain(rocks.iter())
                    }
                    .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("#")
                .chars()
                .collect::<Vec<_>>()
        })
        .collect();

    if needs_trans {
        transpose(&slided)
    } else {
        slided
    }
}

fn cycle(rocks : &Vec<Vec<Rock>>) -> Vec<Vec<Rock>> {
    let mut copy = rocks.clone();
    copy = slide_rocks(&copy, Direction::North);
    copy = slide_rocks(&copy, Direction::West);
    copy = slide_rocks(&copy, Direction::South);
    copy = slide_rocks(&copy, Direction::East);
    copy
}

fn to_str(rocks : &Vec<Vec<Rock>>) -> String {
    rocks.iter().map(|line| line.iter().collect::<String>()).collect()
}

impl Solution for Day14 {
    type Input = Vec<Vec<char>>;
    type ReturnType = u32;
    const DAY: u32 = 14;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.map(|line| line.chars().collect()).collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        let slided = slide_rocks(&input, Direction::North);
        slided
            .iter()
            .enumerate()
            .map(|(i, line)| (slided.len() - i) * line.iter().filter(|&&c| c == 'O').count())
            .sum::<usize>() as u32
    }
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        let mut slided = input.clone();
        let mut cache : HashMap<String, u32> = HashMap::from([(to_str(&slided), 0)]);
        let max_cycle = 1_000_000_000;
        let mut i = 0;
        while i < max_cycle {
            slided = cycle(&slided);
            let slided_str = to_str(&slided);
            match cache.insert(slided_str, i) {
                Some(start) => {
                    i += (i - start) * ((max_cycle -i)/(i - start));
                } ,
                None => (),
            }
            i += 1;
        }
        
        slided
            .iter()
            .enumerate()
            .map(|(i, line)| (slided.len() - i) * line.iter().filter(|&&c| c == 'O').count())
            .sum::<usize>() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::Day14;
    use crate::{
        solution::Solution,
        solutions::day14::{slide_rocks, cycle,  Direction},
    };

    static INPUT_TEST: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    static SLIDED: &str = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

    static CYCLE_ONE: &str = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";

    #[test]
    fn test_slide() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day14.parse_input(lines);
        let slided = slide_rocks(&input, Direction::North);
        let slided_lines = SLIDED.split('\n').map(|s| s.to_string());
        let slided_ref = Day14.parse_input(slided_lines);
        assert_eq!(slided, slided_ref);
    }
    #[test]
    fn test_cycle() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day14.parse_input(lines);
        let slided = cycle(&input);
        let slided_lines = CYCLE_ONE.split('\n').map(|s| s.to_string());
        let slided_ref = Day14.parse_input(slided_lines);
        assert_eq!(slided, slided_ref);
    }

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day14.parse_input(lines);
        assert_eq!(Day14.first_part(&input), 136)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day14.parse_input(lines);
        assert_eq!(Day14.second_part(&input), 64)
    }
}
