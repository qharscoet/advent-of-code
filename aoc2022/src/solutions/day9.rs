use std::{collections::HashSet};

use crate::solution::Solution;

pub struct Day9;


pub enum Move {
	Right,
	Left,
	Up,
	Down,
}


pub struct Motion {
    m : Move,
    val : u32,
}

impl std::str::FromStr for Motion {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair = s.split_once(" ").unwrap_or_default();
        let value = pair.1.parse::<u32>().expect("Not a number");
        
        match pair.0 {
            "R" =>  Ok(Motion{m:Move::Right, val: value}),
            "L" =>  Ok(Motion{m:Move::Left, val: value}),
            "D" =>  Ok(Motion{m:Move::Down, val: value}),
            "U" =>  Ok(Motion{m:Move::Up, val: value}),
            _ => Err("Not a valid move")
        }
    }
}

impl Solution for Day9 {
    type Input = Vec<Motion>;
    type ReturnType = usize;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
       lines.flat_map(|l| l.parse()).collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        let mut h : (i32,i32) = (0,0);
        let mut t : (i32,i32) = (0,0);
        let mut visited : HashSet<(i32,i32)> = HashSet::new();
        
        for motion in input {
            for _ in 0..motion.val {
                match motion.m {
                    Move::Right=> h.0 += 1,
                    Move::Left=> h.0 -= 1,
                    Move::Up=> h.1 +=1,
                    Move::Down=> h.1 -= 1,
                };
                
                if  h.1.abs_diff(t.1) >= 2 || h.0.abs_diff(t.0) >= 2 {
                    t.0 += (h.0-t.0).signum();           
                    t.1 += (h.1-t.1).signum(); 
                } 
                
                visited.insert(t);
            }
        }
		visited.len()
	}
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        let mut rope : [(i32,i32);10] =[(0,0);10];
        let mut visited : HashSet<(i32,i32)> = HashSet::new();
        
        for motion in input {
            for _ in 0..motion.val {
                match motion.m {
                    Move::Right=> rope[0].0 += 1,
                    Move::Left=> rope[0].0 -= 1,
                    Move::Up=> rope[0].1 +=1,
                    Move::Down=> rope[0].1 -= 1,
                };
                
                for i in 1..10 {
                    if  rope[i-1].1.abs_diff(rope[i].1) >= 2 || rope[i-1].0.abs_diff(rope[i].0) >= 2 {
                        rope[i].0 += (rope[i-1].0-rope[i].0).signum();           
                        rope[i].1 += (rope[i-1].1-rope[i].1).signum(); 
                    } 

                }
                
                visited.insert(rope[9]);
            }
        }
		visited.len()
	}
}

#[cfg(test)]
mod tests {
    use super::Day9;
    use crate::solution::Solution;

		static INPUT_TEST : &str =
"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    static INPUT_TEST_2 : &str = 
    "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day9.parse_input(lines);
		assert_eq!(Day9.first_part(&input),
        13)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day9.parse_input(lines);
		assert_eq!(Day9.second_part(&input),
            1)
    }
    #[test]
    fn test_second_part_two() {
        let lines = INPUT_TEST_2.split('\n').map(|s| s.to_string());
        let input = Day9.parse_input(lines);
		assert_eq!(Day9.second_part(&input),
            36)
    }
}