use std::collections::{HashSet, VecDeque};

use crate::solution::Solution;

pub struct Day16;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

type Beam = (usize, usize, Direction);

fn get_next((r, c, dir): Beam, (max_x, max_y): (usize, usize)) -> Option<Beam> {
    match dir {
        Direction::Up => {
            if r > 0 {
                Some((r - 1, c, dir))
            } else {
                None
            }
        }
        Direction::Right => {
            if c < max_x - 1 {
                Some((r, c + 1, dir))
            } else {
                None
            }
        }
        Direction::Down => {
            if r < max_y - 1 {
                Some((r + 1, c, dir))
            } else {
                None
            }
        }
        Direction::Left => {
            if c > 0 {
                Some((r, c - 1, dir))
            } else {
                None
            }
        }
    }
}

impl Solution for Day16 {
    type Input = Vec<Vec<char>>;
    type ReturnType = u32;
    const DAY: u32 = 16;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.map(|line| line.chars().collect()).collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        let mut queue = VecDeque::from([(0, 0, Direction::Right)]);
        let mut set = HashSet::new();

        let max = (input[0].len(), input.len());

        while let Some(beam) = queue.pop_front() {
            let mut push_next = |beam: Beam| {
                if let Some(next) = get_next(beam, max) {
                    if !set.contains(&next) {
                        queue.push_back(next);
                    }
                };
            };

            let (r, c, dir) = beam.clone();
            match input[r][c] {
                '.' => push_next(beam),
                '/' => match dir {
                    Direction::Up => push_next((r, c, Direction::Right)),
                    Direction::Right => push_next((r, c, Direction::Up)),
                    Direction::Down => push_next((r, c, Direction::Left)),
                    Direction::Left => push_next((r, c, Direction::Down)),
                },
                '\\' => match dir {
                    Direction::Up => push_next((r, c, Direction::Left)),
                    Direction::Right => push_next((r, c, Direction::Down)),
                    Direction::Down => push_next((r, c, Direction::Right)),
                    Direction::Left => push_next((r, c, Direction::Up)),
                },
                '|' => match dir {
                    Direction::Up | Direction::Down => push_next(beam),
                    Direction::Left | Direction::Right => {
                        push_next((r, c, Direction::Up));
                        push_next((r, c, Direction::Down));
                    }
                },
                '-' => match dir {
                    Direction::Left | Direction::Right => push_next(beam),
                    Direction::Up | Direction::Down => {
                        push_next((r, c, Direction::Left));
                        push_next((r, c, Direction::Right));
                    }
                },
                _ => (),
            }

            set.insert((r, c,dir));
        }

        set.iter().map(|t| (t.0, t.1)).collect::<HashSet<(usize,usize)>>().len() as u32
    }
    fn second_part(&self, _input: &Self::Input) -> Self::ReturnType {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Day16;
    use crate::solution::Solution;

    static INPUT_TEST: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day16.parse_input(lines);
        assert_eq!(Day16.first_part(&input), 46)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day16.parse_input(lines);
        assert_eq!(Day16.second_part(&input), 51)
    }
}
