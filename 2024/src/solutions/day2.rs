use std::cmp::Ordering;

use crate::solution::Solution;

pub struct Day2;

type Report = Vec<u32>;

fn is_report_safe(report : &Report) -> bool {

    let all_gt = report.windows(2).all(|w| w[0].cmp(&w[1]) == Ordering::Greater);
    let all_lt = report.windows(2).all(|w| w[0].cmp(&w[1]) == Ordering::Less);
    let rule2 = report.windows(2).all(|w| (1..=3).contains(&(w[0].abs_diff(w[1]))));
    (all_gt || all_lt) && rule2
}

impl Solution for Day2 {
    type Input = Vec<Report>;
    type ReturnType = u32;
    const DAY : u32 = 2;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.map(|line| 
            line.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect()
        ).collect()
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        input.iter().filter(|report| is_report_safe(report)).count() as u32
    }
    fn second_part(&self, input: &Self::Input) -> u32 {
        input.iter().filter(|report| {
            //Try all potential reports by removing every level one by one..
            (0..report.len()).any(|i| {
                let new_report = report.iter().enumerate().filter_map(|(idx, val)| if idx == i { None } else {Some(*val)} ).collect();
                is_report_safe(&new_report)
            })
        }).count() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::Day2;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";


    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day2.parse_input(lines);
        assert_eq!(Day2.first_part(&input), 2)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day2.parse_input(lines);
        assert_eq!(Day2.second_part(&input), 4);
    }
}
