use std::collections::HashMap;

use crate::solution::Solution;

pub struct Day8;

#[derive(Debug)]
enum Direction {
    R,
    L,
}

type Node = String;
type Network = HashMap<Node, (Node, Node)>;

pub struct Map {
    instr: Vec<Direction>,
    network: Network,
}

impl Solution for Day8 {
    type Input = Map;
    type ReturnType = u32;
    const DAY: u32 = 8;

    fn parse_input(&self, mut lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        let instr_str: String = lines.nth(0).unwrap();
        let instr: Vec<Direction> = instr_str
            .chars()
            .map(|c| {
                match c {
                    'R' => Some(Direction::R),
                    'L' => Some(Direction::L),
                    _ => None,
                }
                .unwrap()
            })
            .collect();

        let network: Network = lines
            .skip(1)
            .map(|l| {
                let (node_name, dirs) = l.split_once('=').expect("Invalid format");
                let (dir_l, dir_r) = dirs.split_once(',').expect("Error parsing dirs");

                (
                    node_name.trim().to_string(),
                    (
                        dir_l.trim()[1..].to_string(),
                        dir_r.trim().trim_end_matches(')').to_string(),
                    ),
                )
            })
            .collect();

        Map {
            instr: instr,
            network: network,
        }
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        input
            .instr
            .iter()
            .cycle()
            .scan("AAA".to_string(), |node, i| {
                if node == "ZZZ" {
                    return None;
                }

                let (l, r) = input.network.get(node).expect("Unknown node");
                let next_node = match i {
                    Direction::L => l,
                    Direction::R => r,
                };

                // println!("curr node : {}, dir {:?}, next {}", node, i , next_node);
                *node = next_node.clone();
                Some(next_node)
            })
            .count() as u32
    }
    fn second_part(&self, _input: &Self::Input) -> Self::ReturnType {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Day8;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    static INPUT_TEST_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day8.parse_input(lines);
        assert_eq!(Day8.first_part(&input), 2)
    }

    #[test]
    fn test_first_part_2() {
        let lines = INPUT_TEST_2.split('\n').map(|s| s.to_string());
        let input = Day8.parse_input(lines);
        assert_eq!(Day8.first_part(&input), 6)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST_2.split('\n').map(|s| s.to_string());
        let input = Day8.parse_input(lines);
        assert_eq!(Day8.second_part(&input), u32::MAX)
    }
}
