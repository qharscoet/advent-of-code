use std::collections::HashSet;

use crate::solution::Solution;

pub struct Day11;

type Coord = (usize,usize);
impl Solution for Day11 {
    type Input = HashSet<Coord>;
    type ReturnType = u32;
    const DAY : u32 = 11;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.enumerate()
            .flat_map(|(idx_l, line)| {
                line
                    .trim()
                    .chars()
                    .enumerate()
                    .filter_map(|(idx_c, c)| {
                        if c == '#' {
                            Some((idx_l, idx_c))
                        } else {
                            None
                        }
                    })
                    .collect::<HashSet<Coord>>()
            }).collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        println!("{:?}", input);
        let prod : HashSet<_> = input.iter().flat_map(|p1| input.iter().map(move |p2| if p1 < p2 {(p1,p2)} else {(p2,p1)})).filter(|(p1,p2)| p1 != p2).collect();
        prod.iter().map(|(p1,p2)| {
            let diff_x : u32= (std::cmp::min(p1.0, p2.0)..std::cmp::max(p1.0,p2.0)).map(|x| if input.iter().any(|p| p.0 == x) {1u32} else {2u32}).sum();
            let diff_y : u32= (std::cmp::min(p1.1, p2.1)..std::cmp::max(p1.1,p2.1)).map(|y| if input.iter().any(|p| p.1 == y) {1u32} else {2u32}).sum();
            //println!("diff between {:?} and {:?} is {} + {} = {}", p1, p2, diff_x, diff_y, diff_x + diff_y );

            diff_x + diff_y
        }).sum()
    }
    fn second_part(&self, _input: &Self::Input) -> Self::ReturnType {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Day11;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    static INPUT_TEST_2 : &str = "";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day11.parse_input(lines);
        assert_eq!(Day11.first_part(&input), 374)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST_2.split('\n').map(|s| s.to_string());
        let input = Day11.parse_input(lines);
        assert_eq!(Day11.second_part(&input), u32::MAX)
    }
}
