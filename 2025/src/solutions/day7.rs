
use std::{collections::{HashMap, HashSet, VecDeque}, hash::Hash, mem::ManuallyDrop, vec};

use crate::solution::Solution;

pub struct Day7;

#[derive(Debug)]
pub struct Manifold {
    splitters : Vec<Vec<usize>>,
    start: (usize,usize)
}

impl Manifold {

    fn send_ray(&self, pos: (usize,usize)) -> u32{

        let mut rays = HashSet::new();
        let mut splitted = HashSet::new();
        rays.insert(pos);

        while !rays.is_empty() {
            let mut next_rays = HashSet::new();
            for r in &rays {
                let j = r.1;
                if let Some(&i) = self.splitters[j].iter().find(|&&i| i > r.0) {
                    if !splitted.contains(&(i,j)) {
                        splitted.insert((i,j));
                        next_rays.insert((i,j-1));
                        next_rays.insert((i,j+1));
                    }
                } 
            }
            rays = next_rays;
        }

        splitted.len() as u32
           
    }

    fn send_ray2(&self, pos: (usize,usize)) -> u64{

        let mut timeline_map = HashMap::new();

        fn inner_send_ray(manifold : &Manifold, pos : (usize, usize), map : &mut HashMap<(usize,usize), u64>) -> u64 {

            if let Some(&c) = map.get(&pos) {
                c
            } else {
                let j = pos.1;
                let count = if let Some(&i) = manifold.splitters[j].iter().find(|&&i| i > pos.0) {
                    inner_send_ray(manifold, (i,j-1), map) + inner_send_ray(manifold, (i, j+1), map) +1
                }
                else {
                    0
                };

                map.insert(pos, count);
                count
            }
        }

        inner_send_ray(&self, pos, &mut timeline_map)           
    }
}

impl Solution for Day7 {
    type Input = Manifold;
    type ReturnType = u64;
    const DAY : u32 = 7;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        let mut manifold = Manifold { splitters : vec![], start : (0,0)};
        for (i,l) in lines.enumerate() {
            for (j,c) in l.chars().enumerate() {
                if manifold.splitters.get(j).is_none() {
                    manifold.splitters.push(vec![]);
                }

                match c {
                    'S' => manifold.start = (i,j),
                    '^' => manifold.splitters[j].push(i),
                    _ => {}
                }
            }
        }
        manifold
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
       input.send_ray(input.start) as u64
    }
    
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        input.send_ray2(input.start)
    }
}

#[cfg(test)]
mod tests {
    use super::Day7;
    use crate::solution::Solution;

    static INPUT_TEST: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";


    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day7.parse_input(lines);
        assert_eq!(Day7.first_part(&input), 21)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day7.parse_input(lines);
        assert_eq!(Day7.second_part(&input), 40);
    }
}
