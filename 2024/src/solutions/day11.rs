use std::collections::HashMap;

use crate::solution::Solution;

pub struct Day11;

fn split(n: u64) -> (u64, u64) {
    let count = n.ilog10() + 1;
    let power10 = 10u64.pow(count / 2);
    let left = n / power10;

    (left, n - (power10 * left))
}

fn blink_stone(stone: u64) -> Vec<u64> {
    if stone == 0 {
        vec![1]
    } else {
        let count = stone.ilog10() + 1;
        if count & 1 == 0 {
            let new_stones = split(stone);
            vec![new_stones.0, new_stones.1]
        } else {
            vec![stone * 2024]
        }
    }
}

fn blink_stones(stones : &Vec<u64>) -> Vec<u64> {
    stones.iter().flat_map(|s| blink_stone(*s)).collect()
}

fn blink_stones_map(stones:&HashMap<u64,u64>) -> HashMap<u64,u64> {

    let mut res : HashMap<u64, u64> = HashMap::new();

    for (stone, n) in stones {
        blink_stone(*stone).iter().for_each(|s| 
            {res.entry(*s).and_modify(|count| *count += n).or_insert(*n);}
        );
    }
    res
}


fn blink_stones_n_times(stones:&Vec<u64>, n:u64) -> Vec<u64> {
    
    let mut stones_copy = stones.clone();
    
    for _ in 0..n {
        stones_copy = blink_stones(&stones_copy);
    }
    
    stones_copy
}

fn blink_stones_n_times_map(stones:&Vec<u64>, n:u64) -> HashMap<u64,u64>  {
    
    let mut res : HashMap<u64,u64> = stones.iter().fold(HashMap::new(), |mut acc, s| {
        acc.entry(*s).and_modify(|count| *count += 1).or_insert(1);
        acc
    });
    
    for _ in 0..n {
        res = blink_stones_map(&res);
    }
    
    res
}


impl Solution for Day11 {
    type Input = Vec<u64>;
    type ReturnType = u64;
    const DAY: u32 = 11;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines
            .flat_map(|line| {
                line.split_ascii_whitespace()
                    .flat_map(|s| s.parse())
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        blink_stones_n_times(input, 25).len() as u64
    }

    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        blink_stones_n_times_map(input, 75).values().sum::<u64>()
    }
}

#[cfg(test)]
mod tests {
    use super::Day11;
    use crate::{
        solution::Solution,
        solutions::day11::{blink_stone, blink_stones, split},
    };

    static INPUT_TEST: &str = "125 17";

    #[test]
    fn test_split() {
        assert_eq!(split(1342), (13, 42));
        assert_eq!(split(666666), (666, 666));
        assert_eq!(split(12345678), (1234, 5678));
    }

    #[test]
    fn test_blink() {
        assert_eq!(blink_stone(0), vec![1]);
        assert_eq!(blink_stone(1), vec![2024]);
        assert_eq!(blink_stone(2), vec![4048]);
        assert_eq!(blink_stone(12), vec![1, 2]);
        assert_eq!(blink_stone(1100), vec![11, 00]);
    }

    #[test]
    fn test_blink_stones(){
        assert_eq!(blink_stones(&vec![0,1,10,99,999]), vec![1,2024,1,0,9,9,2021976] );
        assert_eq!(blink_stones(&vec![125,17]), vec![253000,1,7] );
    }

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day11.parse_input(lines);
        assert_eq!(Day11.first_part(&input), 55312)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day11.parse_input(lines);
        assert_eq!(Day11.second_part(&input), 0);
    }
}
