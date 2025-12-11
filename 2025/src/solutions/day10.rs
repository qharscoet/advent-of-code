
use std::{cmp::Ordering, collections::BinaryHeap};

use crate::solution::Solution;

pub struct Day10;

type Button = Vec<u32>;

#[derive(Debug)]
pub struct Machine {
    goal : (u32, usize),
    buttons : Vec<Button>,
    joltages : Vec<u32>,
}

impl std::str::FromStr for Machine {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut components = s.split_whitespace();

        let goal_str = components.next().unwrap_or_default();
        let goal = goal_str[1..goal_str.len()].chars().enumerate().fold(0, |acc, (i, c)| 
            match c {
                '#' => acc | 1 << i,
                _ => acc
            } );

        let mut buttons = vec![];
        let mut joltage = vec![];
        for c in components {
            if c.starts_with('(') {
                buttons.push(c[1..c.len()-1].split(',').flat_map(|p| p.parse()).collect());
            } else if c.starts_with('{') {
                joltage = c[1..c.len()-1].split(',').flat_map(|p| p.parse()).collect()
            }
        }

        
        Ok(Machine { goal: (goal, goal_str.len() -2), buttons: buttons, joltages: joltage})
    }
}


#[derive(Debug)]
pub struct Graph {
    nodes : Vec<u32>,
    edges : Vec<Vec<u32>>,
}


//Stolen from doc
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn press_button(v : u32, button : &Button) -> u32 {
    button.iter().fold(v, |acc, input| acc ^ (1 << input))
}
impl Graph {

    fn new(machine : &Machine) -> Graph {
       
        let nodes_count = 2u32.pow(machine.goal.1 as u32) as usize;
        let nodes = (0..nodes_count).map(|v| v as u32).collect();
        
        let mut edges = vec![vec![];nodes_count];
        for state in 0..nodes_count {
            for b in &machine.buttons {
                edges[state].push(press_button(state as u32, b));
            }
        }
       Graph { nodes: nodes, edges: edges }
    }

    fn shortest_path(&self, start:usize, goal:usize) -> Option<usize> {

        let mut dist = vec![usize::MAX; self.nodes.len()];
        let mut heap = BinaryHeap::new();

        // We're at `start`, with a zero cost
        dist[start] = 0;
        heap.push(State { cost: 0, position: start });

        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(State { cost, position }) = heap.pop() {
            // Alternatively we could have continued to find all shortest paths
            if position == goal { return Some(cost); }

            // Important as we may have already found a better way
            if cost > dist[position] { continue; }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            for edge in &self.edges[position] {
                let next = State { cost: cost + 1, position: *edge as usize};

                // If so, add it to the frontier and continue
                if next.cost < dist[next.position] {
                    heap.push(next);
                    // Relaxation, we have now found a better way
                    dist[next.position] = next.cost;
                }
            }
        }

        // Goal not reachable
        None
    }
}

impl Solution for Day10 {
    type Input = Vec<Machine>;
    type ReturnType = u32;
    const DAY : u32 = 10;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.flat_map(|l| l.parse()).collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        input.iter().map(|m| {
            let g = Graph::new(&m);
            g.shortest_path(0, m.goal.0 as usize).unwrap_or_default()
        }).sum::<usize>() as u32
    }
    
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Day10;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";


    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day10.parse_input(lines);
        assert_eq!(Day10.first_part(&input), 7)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day10.parse_input(lines);
        assert_eq!(Day10.second_part(&input), 43);
    }
}
