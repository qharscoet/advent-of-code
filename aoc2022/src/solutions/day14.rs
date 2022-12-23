use std::{collections::HashMap, fmt::Display};

use crate::solution::Solution;

pub struct Day14;

type Coord = (usize,usize);
type Path = Vec<Coord>;

#[derive(Clone)]
enum Block {
    Air,
    Rock,
    Sand(bool)
}

#[derive(Clone)]
pub struct Grid {
    blocks : HashMap<Coord, Block>,
    y_limit : usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let xmax = self.blocks.keys().max_by_key(|(x,_y)| x).unwrap().0;
        let xmin = self.blocks.keys().min_by_key(|(x,_y)| x).unwrap().0;
    
        for y in 0..=self.y_limit +2 {
            write!(f,"{} : ", y)?;
            for x in xmin..=xmax {
                if let Some(block) = self.blocks.get(&(x,y)) {
                    write!(f, "{}", match block {
                        Block::Air => '.',
                        Block::Rock => '#',
                        Block::Sand(_) => '0',
                    })?;

                } else {
                    write!(f,"{}", if y == self.y_limit  +2 {'#'} else {'.'})?;
                }
            }
            write!(f,"\n")?;
        }
        write!(f,"\n")

    }
}


impl Grid {
    fn process(&mut self, is_p2:bool) -> bool {
        let active_sand = self.blocks.iter().map(|(p,b)| (*p,b.clone())).find(|(_,v)| matches!(v, Block::Sand(true)));

        if let Some(((x,y), _block)) = active_sand {
            let mut new_pos : Option<(usize,usize)>= None;
            if  is_p2 && y+1 == self.y_limit+2 {
                self.blocks.entry((x,y)).and_modify(|block| *block = Block::Sand(false) );
            } else {
                if self.blocks.contains_key(&(x, y+1)) {
                    if self.blocks.contains_key(&(x-1, y+1)) {
                        if self.blocks.contains_key(&(x +1, y+1)) {
                            self.blocks.entry((x,y)).and_modify(|block| *block = Block::Sand(false) );
                        } else {
                            new_pos = Some((x+1, y+1));
                        }
                    } else {
                        new_pos = Some((x-1, y+1));
                    }
                } else {
                    new_pos = Some((x, y+1));
                }
            }

            if let Some(p) = new_pos {
                self.blocks.remove(&(x,y));

                if p.1 < if is_p2 {self.y_limit +2 } else {self.y_limit} {
                    self.blocks.insert(p, Block::Sand(true));
                } else if !is_p2{
                    return false;
                }
            }

        } else {
            //Insert new sand
            self.blocks.insert((500,0), Block::Sand(true));
        }

        if !is_p2 {true} else { 
            if let Some(b) = self.blocks.get(&(500,0)){
                !matches!(b, Block::Sand(false))
            } else {true}
        }
    }
}

impl Solution for Day14 {
    type Input = Grid;
    type ReturnType = usize;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        let paths : Vec<Path> = lines.map(|l| {
            l.split("->").flat_map(|elem| elem.trim().split_once(',')).map(|(l,r)| (l.parse().unwrap_or_default(), r.parse().unwrap_or_default())).collect()
        }).collect();

        let mut grid = HashMap::new();
        let mut y_limit = 0;

        for p in paths.iter() {
            for w in p.windows(2) {
                let is_vertical = w[0].0 == w[1].0;
                let mut range = if is_vertical { w[0].1..=w[1].1} else {w[0].0..=w[1].0};
                if range.start() > range.end() {
                    range = *range.end()..=*range.start();
                }

                for v in range {
                    let key = if is_vertical { (w[0].0, v)} else { (v, w[0].1)};
                    grid.entry(key).or_insert(Block::Rock);
                    y_limit = y_limit.max(key.1);
                }
            }
        }
        
        Grid { blocks : grid,  y_limit:y_limit}
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        println!("{}", input);
        let mut input_copy = input.clone();
        while input_copy.process(false) {

        }
        println!("{}", input_copy);
        input_copy.blocks.values().filter(|b| matches!(b, Block::Sand(false))).count()
    }
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        println!("{}", input);
        let mut input_copy = input.clone();
        let mut counter = 10000;
        while input_copy.process(true) {
        }
        println!("{}", input_copy);
        input_copy.blocks.values().filter(|b| matches!(b, Block::Sand(false))).count()
    }
}

#[cfg(test)]
mod tests {
    use super::Day14;
    use crate::solution::Solution;

	static INPUT_TEST : &str =
"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day14.parse_input(lines);
		assert_eq!(Day14.first_part(&input),
        24)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day14.parse_input(lines);
		assert_eq!(Day14.second_part(&input),
        94)
    }
}