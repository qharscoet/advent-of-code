use std::collections::{BTreeSet, HashMap, HashSet};

use crate::solution::Solution;

pub struct Day8;

type Box = (u32, u32, u32);
fn dist(a: Box, b: Box) -> u64 {
    let ai = (a.0 as i64, a.1 as i64, a.2 as i64);
    let bi = (b.0 as i64, b.1 as i64, b.2 as i64);
    ((ai.0 - bi.0) * (ai.0 - bi.0) + (ai.1 - bi.1) * (ai.1 - bi.1) + (ai.2 - bi.2) * (ai.2 - bi.2))
        as u64
}

fn compute_dist(boxes: &Vec<Box>) -> BTreeSet<(u64, Box, Box)>{
    boxes
        .iter()
        .flat_map(|&v| {
            boxes.iter().flat_map(move |&v2|{
                if dist(v, v2) > 0 {
                    Some((dist(v, v2), v.min(v2), v.max(v2)))
                } else {
                    None
                }
            })
        })
        .collect()
}

impl Solution for Day8 {
    type Input = Vec<Box>;
    type ReturnType = u64;
    const DAY: u32 = 8;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines
            .map(|l| {
                let mut it = l.split(',').flat_map(|s| s.trim().parse::<u32>());
                (
                    it.next().unwrap_or_default(),
                    it.next().unwrap_or_default(),
                    it.next().unwrap_or_default(),
                )
            })
            .collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        //println!("{:?}", input);
        let distances = compute_dist(&input);

        //Adjacency list
        let mut edges : HashMap<Box, HashSet<Box>> = HashMap::new();

        for (_, a, b) in distances.iter().take(1000) {
            edges.entry(*a).and_modify(|set| {set.insert(*b);}).or_insert(HashSet::from([*b]));
            edges.entry(*b).and_modify(|set| {set.insert(*a);}).or_insert(HashSet::from([*a]));
        }

        let mut visited : HashSet<Box> = HashSet::new();
        let mut groups : Vec<HashSet<Box>> = vec![];

        // Put b and all it's neighbours in group g
        fn register_box(b : &Box, edges: &HashMap<Box, HashSet<Box>>,  visited : &mut HashSet<Box>, g : &mut HashSet<Box>) {
            if visited.contains(b) {
                return;
            }
            visited.insert(*b);
            g.insert(*b);

            if let Some(n) = edges.get(b) {
                for neighbour in n {
                    register_box(neighbour, edges, visited, g);
                }
            }
        }

        for b in input {
            if visited.contains(b) {
                continue;
            }
            let mut g = HashSet::new();
            register_box(b, &edges, &mut visited, &mut g);
            groups.push(g);
        }

        groups.sort_by_key(|g| g.len());

        groups.iter().rev().take(3).map(|g| g.len()).product::<usize>() as u64
    }

    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        //println!("{:?}", input);
        let distances = compute_dist(&input);

        //Adjacency list
        let mut edges : HashMap<Box, HashSet<Box>> = HashMap::new();

        let mut b1 : Box = (0,0,0);
        let mut b2 : Box = (0,0,0);

        for (_, a, b) in distances.iter() {
            edges.entry(*a).and_modify(|set| {set.insert(*b);}).or_insert(HashSet::from([*b]));
            edges.entry(*b).and_modify(|set| {set.insert(*a);}).or_insert(HashSet::from([*a]));

            // Put b and all it's neighbours in group g
            fn register_box(b : &Box, edges: &HashMap<Box, HashSet<Box>>,  visited : &mut HashSet<Box>, g : &mut HashSet<Box>) {
                if visited.contains(b) {
                    return;
                }
                visited.insert(*b);
                g.insert(*b);

                if let Some(n) = edges.get(b) {
                    for neighbour in n {
                        register_box(neighbour, edges, visited, g);
                    }
                }
            }

            let mut visited : HashSet<Box> = HashSet::new();
            let mut g = HashSet::new();
            if let Some(b) = input.first() {
                if visited.contains(b) {
                    continue;
                }
                register_box(b, &edges, &mut visited, &mut g);
            }

            if g.len() == input.len() {
                b1 = *a;
                b2 = *b;
                break;
            }
        }


        b1.0 as u64 * b2.0 as u64
    }
}

#[cfg(test)]
mod tests {
    use super::Day8;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day8.parse_input(lines);
        assert_eq!(Day8.first_part(&input), 40)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day8.parse_input(lines);
        assert_eq!(Day8.second_part(&input), 25272);
    }
}
