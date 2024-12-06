use std::{collections::HashSet, hash::Hash};

use crate::solution::Solution;

pub struct Day6;


#[derive(Debug, Clone)]
pub struct Map {
    obstacles : HashSet<(usize, usize)>,
    pos : (usize,usize),
    size : (usize,usize)
}


#[derive(Hash, Eq, PartialEq, Clone,Copy)]
enum Direction {
    Left,Right,Up,Down
}


//Returns every pos in the path if there is no loop, or None
fn get_path(map:&Map) -> Option<HashSet<(usize,usize)>> {
    let mut curr_dir = Direction::Up;
    let mut curr_pos = map.pos;
    let mut total_pos = HashSet::new();

    let mut visited_states = HashSet::new();

    loop {
        total_pos.insert(curr_pos);

        if visited_states.contains(&(curr_pos, curr_dir)) {
            break None;
        }

        visited_states.insert((curr_pos,curr_dir));

        let facing = match curr_dir {
            Direction::Left => if curr_pos.1 > 0 {Some((curr_pos.0, curr_pos.1 -1)) } else {None},
            Direction::Right => if curr_pos.1 < map.size.1 {Some((curr_pos.0, curr_pos.1 +1))} else {None},
            Direction::Up => if curr_pos.0 > 0 { Some((curr_pos.0 -1, curr_pos.1)) } else {None},
            Direction::Down => if curr_pos.0 < map.size.0 { Some((curr_pos.0 +1, curr_pos.1)) } else {None}
        };

        if let Some(p) = facing {
            if map.obstacles.contains(&p) {
                curr_dir = match curr_dir {
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                };
            } else {
                curr_pos = p;
            }
        } else {
            break Some(total_pos);
        }
    }

}

impl Solution for Day6 {
    type Input = Map;
    type ReturnType = u32;
    const DAY : u32 = 6;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        let mut pos : (usize,usize) = (0,0);
        let obstacles :  HashSet<(usize, usize)> = lines.enumerate().flat_map(|(i, l)| l.chars().enumerate().filter_map(|(j, c)| match c {
            '^' => {pos = (i,j); None},
            '#' => Some((i,j)),
            _ => None
        }).collect::<Vec<_>>()).collect();

        let size_x = obstacles.iter().max_by_key(|pos| pos.0).unwrap().0;
        let size_y = obstacles.iter().max_by_key(|pos| pos.1).unwrap().1;

        Map{ obstacles : obstacles, pos:pos, size : (size_x, size_y)}
    }

    fn first_part(&self, input: &Self::Input) -> u32 {

        if let Some(path) = get_path(&input) {
            path.len() as u32
        } else { 0 }
    }


    fn second_part(&self, input: &Self::Input) -> u32 {

        let initial_path = get_path(&input).unwrap();

        initial_path.iter().filter(|pos| {
            let mut input_copy = input.clone();
            input_copy.obstacles.insert(**pos);
            match get_path(&input_copy) {
                Some(_) => false,
                None => true,
            }
        }).count() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::Day6;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day6.parse_input(lines);
        assert_eq!(Day6.first_part(&input), 41)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day6.parse_input(lines);
        assert_eq!(Day6.second_part(&input), 6);
    }
}
