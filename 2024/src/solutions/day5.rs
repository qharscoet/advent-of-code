use std::collections::HashMap;

use crate::solution::Solution;

pub struct Day5;

type OrderingRules = HashMap<u32, Vec<u32>>;

#[derive(Debug)]
pub struct Updates {
    ordering_rules : OrderingRules,
    update_list : Vec<Vec<u32>>
}

fn sort_fn(a : &u32, b:&u32, rules:&OrderingRules) -> bool {
    if rules.contains_key(a) {
        rules[a].contains(b)
    } else {
        false
    }
}

impl Solution for Day5 {
    type Input = Updates;
    type ReturnType = u32;
    const DAY : u32 = 5;

    fn parse_input(&self, mut lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        let rules : OrderingRules = lines.by_ref().take_while(|l| !l.is_empty()).fold(HashMap::new(),|mut acc,l| {
            let vals = l.split_once('|').unwrap_or_default();
            let (a,b) = (vals.0.parse::<u32>().unwrap_or_default(), vals.1.parse::<u32>().unwrap_or_default());
            acc.entry(a).and_modify(|list| list.push(b)).or_insert(vec![b]);
            acc
        });

        let updates_list : Vec<Vec<u32>> = lines.map(|l| l.split(',').map(|n| n.parse::<u32>().unwrap_or_default()).collect()).collect();

        Updates{ordering_rules:rules, update_list:updates_list}
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        input
            .update_list
            .iter()
            .filter_map(|u| {
                if u.is_sorted_by(|a, b| sort_fn(a, b, &input.ordering_rules)) {
                    Some(u[u.len() / 2])
                } else {
                    None
                }
            })
            .sum()
    }


    fn second_part(&self, input: &Self::Input) -> u32 {
        let mut updates = input.update_list.clone();
        updates
            .iter_mut()
            .filter(|u| !u.is_sorted_by(|a, b| sort_fn(a, b, &input.ordering_rules)))
            .map(|u| {
                u.sort_by(|a, b| match sort_fn(a, b, &input.ordering_rules) {
                    true => std::cmp::Ordering::Less,
                    false => std::cmp::Ordering::Greater,
                });
                u[u.len() / 2]
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Day5;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day5.parse_input(lines);
        assert_eq!(Day5.first_part(&input), 143)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day5.parse_input(lines);
        assert_eq!(Day5.second_part(&input), 123);
    }
}
