use std::collections::{HashMap, HashSet, VecDeque};

use crate::solution::Solution;

pub struct Day12;

type Graph = Vec<Vec<char>>;

#[derive(Debug)]
pub struct Area {
    plant: char,
    positions : HashSet<(usize,usize)>,
    // area: usize,
    // perimeter:usize,
}

fn get_neighbours(g:&Graph, idx :(usize,usize)) -> Vec<(usize,usize)> {
    let mut n = vec![];
        
    if idx.0 > 0  { n.push((idx.0 -1, idx.1))}
    if idx.1 > 0   { n.push((idx.0, idx.1 -1))}
    if idx.0 < g.len() -1   { n.push((idx.0 +1, idx.1))}
    if idx.1 < g[0].len() - 1  { n.push((idx.0, idx.1 +1))}
    
    n
}

fn get_1d_idx(g : &Graph, idx :(usize,usize)) -> usize {
    idx.0 * g[0].len() + idx.1
}

fn get_2d_idx(g : &Graph, idx : usize) -> (usize,usize) {
    (idx / g[0].len(), idx % g[0].len())
}

fn explore(g:&Graph, pos : (usize,usize), visited : &mut Vec<bool>) -> Area {

    let c = g[pos.0][pos.1];

    let mut set = HashSet::new();
    let mut next = VecDeque::new();
    next.push_back(pos);

    while let Some(p) = next.pop_front() {
        for n in get_neighbours(g, p).iter().filter(|p| g[p.0][p.1] == c) {
            let idx = get_1d_idx(g, *n);
            if !visited[idx] {
                visited[idx] = true;
                set.insert(*n);
                next.push_back(*n);
            }
        }

    }

    Area {plant: c, positions:set}
}


impl Solution for Day12 {
    type Input = Vec<Vec<char>>;
    type ReturnType = u64;
    const DAY: u32 = 11;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.map(|l| l.chars().collect()).collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {

        let mut visited = vec![false;input.len() * input[0].len()];
        let mut areas : Vec<Area> = vec![];

        while let Some(idx) = visited.iter().position(|v| *v == false) {
            let p = get_2d_idx(input, idx);
            println!("exploring {} at {},{}", idx, p.0, p.1);
            areas.push(explore(input, get_2d_idx(input, idx), &mut visited));
        }

        println!("{:?}", areas);
        0
    }

    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Day12;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "AAAA
BBCD
BBCC
EEEC";

    static INPUT_TEST_2 : &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";


    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day12.parse_input(lines);
        assert_eq!(Day12.first_part(&input), 772);

        let lines = INPUT_TEST_2.split('\n').map(|s| s.to_string());
        let input = Day12.parse_input(lines);
        assert_eq!(Day12.first_part(&input), 1930);


    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day12.parse_input(lines);
        assert_eq!(Day12.second_part(&input), 0);
    }
}
