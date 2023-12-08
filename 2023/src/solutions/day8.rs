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

fn euclid(a: u64, b: u64) -> u64 {
    match b {
        0 => a,
        _ => euclid(b, a % b),
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / euclid(a, b)
}

fn find_path_count<F>(input: &Map, start_node: Node, f: F) -> u32
where
    F: Fn(Node) -> bool,
{
    input
        .instr
        .iter()
        .cycle()
        .scan(start_node, |node, i| {
            if f(node.clone()) {
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

impl Solution for Day8 {
    type Input = Map;
    type ReturnType = u64;
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
        find_path_count(&input, "AAA".to_string(), |node| node == "ZZZ") as u64
    }
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        let nodes: Vec<&Node> = input
            .network
            .keys()
            .filter(|k| k.chars().last().unwrap() == 'A')
            .collect();

        nodes
            .iter()
            .map(|node| {
                find_path_count(&input, node.to_string(), |n| {
                    n.chars().last().unwrap() == 'Z'
                }) as u64
            })
            .reduce(|acc, v| lcm(acc, v))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Day8;
    use crate::{solution::Solution, solutions::day8::lcm};

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

    static INPUT_TEST_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

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
        let lines = INPUT_TEST_3.split('\n').map(|s| s.to_string());
        let input = Day8.parse_input(lines);
        assert_eq!(Day8.second_part(&input), 6)
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(18, 12), 36)
    }
}
