use std::collections::{HashMap, VecDeque};

use crate::solution::Solution;

pub struct Day12;

type Graph = Vec<Vec<char>>;

#[derive(Debug)]
pub struct Region {
    plant: char,
    positions : HashMap<(usize,usize), bool>,
    // area: usize,
    // perimeter:usize,
}

impl Region {
    fn area(&self) -> usize {
        self.positions.len()
    }

    fn min(&self) -> (usize,usize) {
        self.positions.iter().fold((usize::MAX,usize::MAX),|acc, p| (acc.0.min(p.0.0), acc.1.min(p.0.1)))
    }

    fn max(&self) -> (usize,usize) {
        self.positions.iter().fold((0,0),|acc, p| (acc.0.max(p.0.0), acc.1.max(p.0.1)))
    }

    fn perimeter_one_side(&self, side:bool) -> usize {
        let min = self.min();
        let max = self.max();

        let outer_range = if side {min.1..=max.1} else {min.0..=max.0};
        let inner_range = if side {min.0..=max.0} else {min.1..=max.1};

        outer_range.fold(0, |acc, x| {
            let (l, inside) = inner_range.clone().fold((0, 0), |(acc, inside), y| {
                let pos = if side {&(y,x)} else {&(x,y)};
                let state = (self.positions.contains_key(pos) as u32) + (*self.positions.get(pos).unwrap_or(&false) as u32);
                let changed = state != inside;
                let connect = changed && (inside > 0 && state > 0); 
                (acc + (changed as u32) + (connect as u32), state)
            });

            acc + l + ((inside != 0) as u32)
        }) as usize
    }

    fn perimeter(&self) -> usize {
        /*
            For each dimension, we check the number of line the state change, 
            For example, for the lien
            OOXXOOXXOO
            We go in and out of the "O slice" 6 times
        */

        let x = self.perimeter_one_side(true);
        let y = self.perimeter_one_side(false);
        
        (x + y) as usize
    }

    fn sides_count_1d(&self, side:bool) -> usize {
        let min = self.min();
        let max = self.max();

        let outer_range = if side {min.1..=max.1} else {min.0..=max.0};
        let inner_range = if side {min.0..=max.0} else {min.1..=max.1};

        let walls = outer_range.fold(HashMap::new(), |mut acc, x| {
            let (mut l, inside) = inner_range.clone().fold((HashMap::new(), false), |(mut acc, inside), y| {
                let pos = if side {(y,x)} else {(x,y)};
                let changed = *self.positions.get(&pos).unwrap_or(&false) != inside;
                if changed { acc.insert(pos, inside);}

                (acc , inside ^ changed)
            });
            if inside {
                let pos = if side {(max.0 +1, x)} else {(x,max.1 +1)};
                l.insert(pos, true);
            }

            l.iter().for_each(|v| {acc.insert(*v.0, *v.1);});
            acc
        });

        let r = Region{plant:'Y', positions: walls};
        r.perimeter_one_side(!side) /2
    }

    fn sides_count(&self) -> usize {
        /*
            Same base algorithm as the perimeter, but instead of counting the number of changes,
            we store those positiosn, and count the number of unique position on the other dimension
        */ 

        self.sides_count_1d(false) + self.sides_count_1d(true)
    }

    fn draw(&self) {
        let min = self.min();
        let max = self.max();

        for i in min.0..=max.0 {
            for j in min.1..=max.1{
                print!("{}", if self.positions.contains_key(&(i,j)) { if self.positions[&(i,j)] {self.plant} else {'O'}} else {'.'});
            }
            print!("\n");
        }
    }

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

fn explore(g:&Graph, pos : (usize,usize), visited : &mut Vec<bool>) -> Region {

    let c = g[pos.0][pos.1];

    let mut set = HashMap::new();
    let mut next = VecDeque::new();
    next.push_back(pos);

    while let Some(p) = next.pop_front() {

        let idx =  get_1d_idx(g, p);
        if !visited[idx] {
            for n in get_neighbours(g, p).iter().filter(|p| g[p.0][p.1] == c) {
                next.push_back(*n);
            }
            
            set.insert(p,true);
            visited[idx] = true;
        }

    }

    Region {plant: c, positions:set}
}


impl Solution for Day12 {
    type Input = Vec<Region>;
    type ReturnType = u32;
    const DAY: u32 = 12;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        let grid : Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();

        let mut visited = vec![false;grid.len() * grid[0].len()];
        let mut regions : Vec<Region> = vec![];

        while let Some(idx) = visited.iter().position(|v| *v == false) {
            regions.push(explore(&grid, get_2d_idx(&grid, idx), &mut visited));
        }

        regions
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        input.iter().map(|r| r.perimeter() * r.area()).sum::<usize>() as u32
    }

    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        input.iter().map(|r| r.sides_count() * r.area()).sum::<usize>() as u32
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

    static INPUT_TEST_2 : &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    static INPUT_TEST_3 : &str ="RRRRIICCFF
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
        assert_eq!(Day12.first_part(&input), 140);

        println!("test 2");
        let lines = INPUT_TEST_2.split('\n').map(|s| s.to_string());
        let input = Day12.parse_input(lines);
        assert_eq!(Day12.first_part(&input), 772);
        
        println!("test 3");
        let lines = INPUT_TEST_3.split('\n').map(|s| s.to_string());
        let input = Day12.parse_input(lines);
        assert_eq!(Day12.first_part(&input), 1930);
    }

    static E_SHAPED : &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    static OTHER_TEST : &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day12.parse_input(lines);
        assert_eq!(Day12.second_part(&input), 80);

        println!("test 2");
        let lines = INPUT_TEST_2.split('\n').map(|s| s.to_string());
        let input = Day12.parse_input(lines);
        assert_eq!(Day12.second_part(&input), 436);
        
        println!("test 3");
        let lines = INPUT_TEST_3.split('\n').map(|s| s.to_string());
        let input = Day12.parse_input(lines);
        assert_eq!(Day12.second_part(&input), 1206);

        println!("E shaped");
        let lines = E_SHAPED.split('\n').map(|s| s.to_string());
        let input = Day12.parse_input(lines);
        assert_eq!(Day12.second_part(&input), 236);

        println!("other");
        let lines = OTHER_TEST.split('\n').map(|s| s.to_string());
        let input = Day12.parse_input(lines);
        assert_eq!(Day12.second_part(&input), 368);
    }
}
