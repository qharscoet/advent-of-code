
use std::ops::RangeInclusive;

use crate::solution::Solution;

pub struct Day5;

#[derive(Debug)]
pub struct Ingredients {
    ranges : Vec<RangeInclusive<u64>>,
    fresh_list : Vec<u64>,
}

impl Solution for Day5 {
    type Input = Ingredients;
    type ReturnType = u64;
    const DAY : u32 = 5;

    fn parse_input(&self, mut lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        let ranges : Vec<_> = lines.by_ref().take_while(|line| line != "")
            .map(|line| {
                let (x, y) = line.split_once("-").unwrap_or_default();
                
                    x.parse::<u64>().unwrap_or_default()..=
                    y.parse::<u64>().unwrap_or_default()
                
            })
            .collect();

        let list : Vec<_> = lines.flat_map(|line| line.parse::<u64>().ok()).collect();
        
        Ingredients{ranges : ranges, fresh_list : list}
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        input.fresh_list.iter().filter(|&item| {
            input.ranges.iter().any(|range| range.contains(item))
        }).count() as u64
    }
    
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        let mut ranges = input.ranges.clone();
        ranges.sort_by(|a,b| {
            if a.start() != b.start() {
                a.start().cmp(b.start())
            } else {
                a.end().cmp(b.end())
            }
        });

        let mut count = 0u64;
        
        let mut start = *ranges[0].start();
        let mut end = *ranges[0].end();
        for r in &ranges[1..] {
            if end < *r.start() { //new disjointed range
                count += end - start + 1; // +1 because inclusive
                start = *r.start();
                end = *r.end();
            } else if end < *r.end() { //Overlap, only save the new end
                end = *r.end();
            }
        }
        
        count += end - start +1; // last one
        count
        
        /* Below does not work on input, don't know why, some range might be counted 2 times */
        // let mut cursor = 0u64;
        // for r in ranges {
        //     if cursor < *r.start() { // new disjointed range, add its count
        //         count += r.end() - r.start() +1; //+1 because range is inclusive
        //     } else if cursor < *r.end() { // overlap, add the diff between the end of the previous and this one
        //         count += r.end() - cursor; // No +1 as already counted the cursor pos
        //     } else {
        //         println!("noop");
        //     }
        //     cursor = *r.end();
        //     println!("{:?} cursor {} count {}", r,  cursor, count);
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::Day5;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";


    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day5.parse_input(lines);
        assert_eq!(Day5.first_part(&input), 3)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day5.parse_input(lines);
        assert_eq!(Day5.second_part(&input), 14);
    }
}
