use std::{collections::HashSet, hash::Hash};

use crate::solution::Solution;

pub struct Day10;

type Heightmap = Vec<Vec<u8>>;

fn get_neighbours(map:&Heightmap, idx :(usize,usize)) -> Vec<(usize,usize)> {
    let mut n = vec![];
        
    if idx.0 > 0  { n.push((idx.0 -1, idx.1))}
    if idx.1 > 0   { n.push((idx.0, idx.1 -1))}
    if idx.0 < map.len() -1   { n.push((idx.0 +1, idx.1))}
    if idx.1 < map[0].len() - 1  { n.push((idx.0, idx.1 +1))}
    
    n
}

fn trailhead_score(map : &Heightmap, pos : (usize,usize)) -> u32 {

    fn explore(map:&Heightmap, pos:(usize,usize), set:&mut HashSet<(usize,usize)>) {
        if map[pos.0][pos.1] == 9 { 
            set.insert(pos);
        } else {
            get_neighbours(map, pos).iter().filter(|n| map[n.0][n.1] == map[pos.0][pos.1] +1).for_each(|p| explore(map,*p,set))
        }
    }

    let mut reached = HashSet::new();
    explore(map, pos,&mut reached);
    reached.len() as u32
}

fn trailhead_rating(map : &Heightmap, pos : (usize,usize)) -> u32 {

    fn explore(map:&Heightmap, pos:(usize,usize)) -> u32  {
        if map[pos.0][pos.1] == 9 { 
            1
        } else {
            get_neighbours(map, pos).iter().filter(|n| map[n.0][n.1] == map[pos.0][pos.1] +1).map(|p| explore(map,*p)).sum()
        }
    }

    explore(map, pos)
}

impl Solution for Day10 {
    type Input = Heightmap;
    type ReturnType = u32;
    const DAY: u32 = 10;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines
            .map(|line| {
                line.chars()
                    .map(|c| if c == '.' {255} else {c as u8 - '0' as u8})
                    .collect::<Vec<u8>>()
            })
            .collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        let trailheads : Vec<_> = input.iter().enumerate().flat_map(|(i,v)| v.iter().enumerate().filter_map(|(j, height)| if *height == 0 {Some((i,j))} else {None}).collect::<Vec<_>>()).collect();
        trailheads.iter().map(|t| trailhead_score(&input, *t)).sum()
        
    }

    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        let trailheads : Vec<_> = input.iter().enumerate().flat_map(|(i,v)| v.iter().enumerate().filter_map(|(j, height)| if *height == 0 {Some((i,j))} else {None}).collect::<Vec<_>>()).collect();
        trailheads.iter().map(|t| trailhead_rating(&input, *t)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Day10;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    static INPUT_TEST_BETA : &str="...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";

    #[test]
    fn test_first_part_beta() {
        let lines = INPUT_TEST_BETA.split('\n').map(|s| s.to_string());
        let input = Day10.parse_input(lines);
        assert_eq!(Day10.first_part(&input), 2)
    }

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day10.parse_input(lines);
        assert_eq!(Day10.first_part(&input), 36)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day10.parse_input(lines);
        assert_eq!(Day10.second_part(&input), 81);
    }
}
