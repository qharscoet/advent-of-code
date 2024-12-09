use std::collections::{HashMap, HashSet};

use crate::solution::Solution;

pub struct Day8;
pub struct Map {
    size: (usize, usize),
    antennas: HashMap<char, Vec<(usize, usize)>>, //<Antenna>
}

impl Solution for Day8 {
    type Input = Map;
    type ReturnType = u32;
    const DAY: u32 = 8;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        let lines_vec: Vec<_> = lines.collect();
        //let antennas = lines_vec.iter().enumerate().flat_map(|(i, l)| l.chars().enumerate().filter_map(|(j, c)| if c != '.' {Some(Antenna{frequency:c, pos:(i,j)})} else {None}).collect::<Vec<_>>()).collect();
        let antennas = lines_vec
            .iter()
            .enumerate()
            .flat_map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .filter_map(|(j, c)| if c != '.' { Some((c, (i, j))) } else { None })
                    .collect::<Vec<_>>()
            })
            .fold(HashMap::new(), |mut acc, (c, pos)| {
                acc.entry(c).and_modify(|v : &mut Vec<_>| v.push(pos)).or_insert(vec![pos]);
                acc
            });

        Map {
            antennas: antennas,
            size: (lines_vec.len(), lines_vec[0].len()),
        }
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        input.antennas.values().fold(HashSet::new(), |acc,  antennas| {
            let v = antennas.iter().flat_map(|p1| antennas.iter().map(|p2| (p1,p2)).collect::<Vec<_>>()).filter_map(|(p1,p2)| {
                if p1 == p2 { return None; }
                
                let (p1i, p2i) = ((p1.0 as isize, p1.1 as isize), (p2.0 as isize, p2.1 as isize));
                let antinode = (p1i.0 - (p2i.0 - p1i.0), p1i.1 - (p2i.1 - p1i.1));
                
                if antinode.0 >= 0 && antinode.0 < input.size.0 as isize && antinode.1 >= 0 && antinode.1 < input.size.1 as isize {
                    Some(antinode)
                } else {None}
            }).collect::<HashSet<_>>();

            acc.union(&v).map(|pos | *pos).collect()
        }).len() as u32
    }

    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        input.antennas.values().fold(HashSet::new(), |acc,  antennas| {
            let v = antennas.iter().flat_map(|p1| antennas.iter().map(|p2| (p1,p2)).collect::<Vec<_>>()).flat_map(|(p1,p2)| {
                let (p1i, p2i) = ((p1.0 as isize, p1.1 as isize), (p2.0 as isize, p2.1 as isize));
                if p1 == p2 { return vec![p1i]};

                let mut antinodes = Vec::new();

                let mut antinode  = p1i;
                
                while antinode.0 >= 0 && antinode.0 < input.size.0 as isize && antinode.1 >= 0 && antinode.1 < input.size.1 as isize {
                    antinodes.push(antinode);
                    antinode.0 += p2i.0 - p1i.0;
                    antinode.1 += p2i.1 - p1i.1;
                } 

                antinodes
            }).collect::<HashSet<_>>();

            acc.union(&v).map(|pos | *pos).collect()
        }).len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::Day8;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day8.parse_input(lines);
        assert_eq!(Day8.first_part(&input), 14)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day8.parse_input(lines);
        assert_eq!(Day8.second_part(&input), 34);
    }
}
