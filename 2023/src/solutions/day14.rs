use crate::solution::Solution;

pub struct Day14;

// #[derive(Clone, Copy)]
// #[derive(PartialEq, Eq)]
// pub enum Rock {
//     Round,
//     Block,
//     Empty
// }

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

fn slide_rocks(rocks: &Vec<Vec<Rock>>) -> Vec<Vec<Rock>> {
    let trans = transpose(rocks);

    let slided: Vec<_> = trans
        .iter()
        .map(|line| {
            line.split(|&r| r == '#')
                .map(|g| {
                    let total = g.len();
                    let count = g.iter().filter(|&&r| r == 'O').count();
                    let mut rocks = vec!['O'; count];
                    rocks.extend(vec!['.'; total - count].iter());
                    rocks.iter().collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("#")
                .chars()
                .collect::<Vec<_>>()
        })
        .collect();

    transpose(&slided)
}

fn dispay(rocks: &Vec<Vec<Rock>>) {
    for line in rocks {
        println!("{}", line.iter().collect::<String>());
    }
}

impl Solution for Day14 {
    type Input = Vec<Vec<char>>;
    type ReturnType = u32;
    const DAY: u32 = 14;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.map(|line| line.chars().collect()).collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        let slided = slide_rocks(&input);
        slided.iter().enumerate().map(|(i, line)| (slided.len() - i) * line.iter().filter(|&&c| c == 'O').count()).sum::<usize>() as u32
    }
    fn second_part(&self, _input: &Self::Input) -> Self::ReturnType {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Day14;
    use crate::{solution::Solution, solutions::day14::slide_rocks};

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

    #[test]
    fn test_slide() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day14.parse_input(lines);
        let slided = slide_rocks(&input);
        let slided_lines = SLIDED.split('\n').map(|s| s.to_string());
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
        assert_eq!(Day14.second_part(&input), u32::MAX)
    }
}
