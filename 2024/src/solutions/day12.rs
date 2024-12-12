use std::collections::{HashMap, HashSet, VecDeque};

use crate::solution::Solution;

pub struct Day12;

type Graph = Vec<Vec<char>>;

#[derive(Debug)]
pub struct Region {
    plant: char,
    positions : HashSet<(usize,usize)>,
    // area: usize,
    // perimeter:usize,
}

impl Region {
    fn area(&self) -> usize {
        self.positions.len()
    }

    fn min(&self) -> (usize,usize) {
        self.positions.iter().fold((usize::MAX,usize::MAX),|acc, p| (acc.0.min(p.0), acc.1.min(p.1)))
    }

    fn max(&self) -> (usize,usize) {
        self.positions.iter().fold((0,0),|acc, p| (acc.0.max(p.0), acc.1.max(p.1)))
    }

    fn perimeter_x(&self) -> usize {
        let min = self.min();
        let max = self.max();

        (min.1..=max.1).fold(0, |acc, x| {
            let (l, inside) = (min.0..=max.0).fold((0, false), |(acc, inside), y| {
                let changed = self.positions.contains(&(y, x)) != inside;
                (acc + (changed as u32), inside ^ changed)
            });

            acc + l + (inside as u32)
        }) as usize
    }

    fn perimeter_y(&self) -> usize {
        let min = self.min();
        let max = self.max();

        (min.0..=max.0).fold(0, |acc, x| {
            let (l, inside) = (min.1..=max.1).fold((0, false), |(acc, inside), y| {
                let changed = self.positions.contains(&(x, y)) != inside;
                (acc + (changed as u32), inside ^ changed)
            });

            acc + l + (inside as u32)
        }) as usize
    }

    fn perimeter(&self) -> usize {
        /*
            For each dimension, we check the number of line the state change, 
            For example, for the lien
            OOXXOOXXOO
            We go in and out of the "O slice" 6 times
        */

        let x = self.perimeter_x();
        let y = self.perimeter_y();
        
        (x + y) as usize
    }

    fn sides_count(&self) -> usize {
        /*
            Same base algorithm as the perimeter, but instead of counting the number of changes,
            we store those positiosn, and count the number of unique position on the other dimension
        */ 
        let min = self.min();
        let max = self.max();

        let y = (min.0..=max.0).fold(HashSet::new(), |acc, x| {
            let (mut l, inside) = (min.1..=max.1).fold((HashSet::new(), false), |(mut acc, inside), y| {
                let changed = self.positions.contains(&(x, y)) != inside;
                if changed { acc.insert((x,y));}

                (acc , inside ^ changed)
            });
            if inside {l.insert((x, max.1 +1));}

            acc.union(&l).map(|v| *v).collect()
        });

        let r = Region{plant:'Y', positions: y};
        r.draw();
        let sides_y = r.perimeter_x() /2;

        let x = (min.1..=max.1).fold(HashSet::new(), |acc, y| {
            let (mut l, inside) = (min.0..=max.0).fold((HashSet::new(), false), |(mut acc, inside), x| {
                let changed = self.positions.contains(&(x, y)) != inside;
                if changed { acc.insert((x, y));}

                (acc , inside ^ changed)
            });
            if inside {l.insert((max.0 +1, y));}

            acc.union(&l).map(|v| *v).collect()
        });

        let r = Region{plant:'X', positions: x};
        r.draw();
        let sides_x = r.perimeter_y() /2;

        println!("Refion {}, sides {} + {} = {}", self.plant, sides_x, sides_y, sides_x + sides_y);
        sides_x + sides_y
    }

    fn draw(&self) {
        let min = self.min();
        let max = self.max();

        for i in min.0..=max.0 {
            for j in min.1..=max.1{
                print!("{}", if self.positions.contains(&(i,j)) { self.plant} else {'.'});
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

    let mut set = HashSet::new();
    let mut next = VecDeque::new();
    next.push_back(pos);

    while let Some(p) = next.pop_front() {

        let idx =  get_1d_idx(g, p);
        if !visited[idx] {
            for n in get_neighbours(g, p).iter().filter(|p| g[p.0][p.1] == c) {
                next.push_back(*n);
            }
            
            set.insert(p);
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
