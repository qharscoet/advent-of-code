
use std::collections::HashMap;

use crate::solution::Solution;

pub struct Day15;

#[derive(Debug, Clone, Copy)]
pub enum Move {
    Up,Down,Left,Right
}

#[derive(Debug, Clone)]
pub enum Item {
    Wall,Box,
}

#[derive(Debug, Clone)]
pub struct Map {
    items:HashMap<(usize,usize), Item>,
    robot:(usize,usize),
}

#[derive(Debug, Clone)]
pub struct Input {
    map:Map,
    moves:Vec<Move>
}

fn add_pos(pos:(usize,usize), off : (i32,i32)) -> (usize,usize) {
    (((pos.0 as i32) + off.0) as usize, ((pos.1 as i32) + off.1) as usize)
}
impl Map {
    pub fn new() -> Self {
        Self { items:HashMap::new(), robot:(0,0)}
    }

    fn move_item(&mut self, pos : (usize,usize), off : (i32,i32)) -> (i32,i32) {
       
        let new_pos = add_pos(pos, off);
        match self.items.get(&new_pos) {
            Some(item) => match item {
                Item::Wall => (0,0),
                Item::Box => {
                    let o = self.move_item(new_pos, off);
                    self.items.remove(&new_pos);
                    self.items.insert(add_pos(new_pos, o), Item::Box);
                    o
                },
            },
            None => off,
        }
        

    }
    fn step(&mut self, m : Move) {

        let offset = match m {
            Move::Up => (-1, 0),
            Move::Down => (1,0),
            Move::Left => (0,-1),
            Move::Right => (0,1),
        };

        self.robot = add_pos(self.robot, self.move_item(self.robot, offset));
    }

    fn min(&self) -> (usize,usize) {
        self.items.iter().fold((usize::MAX,usize::MAX),|acc, p| (acc.0.min(p.0.0), acc.1.min(p.0.1)))
    }

    fn max(&self) -> (usize,usize) {
        self.items.iter().fold((0,0),|acc, p| (acc.0.max(p.0.0), acc.1.max(p.0.1)))
    }


    #[allow(dead_code)]
    fn draw(&self) {
        let min = self.min();
        let max = self.max();

        for i in min.0..=max.0 {
            for j in min.1..=max.1{
                print!("{}",match self.items.get(&(i,j)) {
                    Some(item) => { match item {
                            Item::Wall => '#',
                            Item::Box => 'O',
                        }},
                    None => { if self.robot == (i,j) {'@'} else {'.'} }
                });
            }
            print!("\n");
        }
    }

}

impl Solution for Day15 {
    type Input = Input;
    type ReturnType = u32;
    const DAY: u32 = 15;

    fn parse_input(&self, mut lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        let map = lines.by_ref().take_while(|l| !l.is_empty()).enumerate().fold(Map::new(),|mut acc,(i,l)| {
            l.chars().enumerate().for_each(|(j,c)| {
                match c {
                    '#' => {acc.items.insert((i,j), Item::Wall);},
                    'O' => {acc.items.insert((i,j), Item::Box);},
                    '@' => {acc.robot = (i,j);},
                    _ => {}
                };
            });
            acc
        });

        let moves = lines.fold(Vec::new(), |mut acc, l| {
            acc.extend(l.chars().flat_map(|c|
                match c {
                    '^' => Some(Move::Up),
                    '>' => Some(Move::Right),
                    '<' => Some(Move::Left),
                    'v' => Some(Move::Down),
                    _ => None,
                }));
            acc 
        });

        Input {map:map, moves: moves}   
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        let mut state = input.clone();
        let map = &mut state.map;

        for m in state.moves {
            map.step(m);
        }

        map.items.iter().filter_map(|(p,item)| if matches!(item, Item::Box) { Some(p.0 * 100 + p.1)} else { None }).sum::<usize>() as u32
        
    }

    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Day15;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

static INPUT_TEST_SMALLER : &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";


    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST_SMALLER.split('\n').map(|s| s.to_string());
        let input = Day15.parse_input(lines);
        assert_eq!(Day15.first_part(&input), 2028);

        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day15.parse_input(lines);
        assert_eq!(Day15.first_part(&input), 10092);
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day15.parse_input(lines);
        assert_eq!(Day15.second_part(&input), 10);
    }
}
