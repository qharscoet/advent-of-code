use std::{vec, collections::BinaryHeap};

use crate::solution::Solution;

pub struct Day12;

#[derive(Clone,Debug)]
pub struct Graph
{
	nodes : Vec<Vec<u8>>,
	start : (usize, usize),
	end : (usize,usize),
}

#[derive(Eq, PartialEq)]
struct NodeDist {
	node_idx : (usize,usize),
	dist : usize,
}

impl PartialOrd for NodeDist {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.dist.partial_cmp(&self.dist)
    }
}

impl Ord for NodeDist {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl Graph {
	fn get_neigbhours(&self, idx :(usize,usize)) -> Vec<(usize,usize)> {
		let mut n = vec![];
		let val = self.nodes[idx.0][idx.1] +1 ;
		
		if idx.0 > 0 && self.nodes[idx.0 -1 ][idx.1] <= val  { n.push((idx.0 -1, idx.1))}
		if idx.1 > 0 && self.nodes[idx.0][idx.1 -1] <= val  { n.push((idx.0, idx.1 -1))}
		if idx.0 < self.nodes.len() -1 && self.nodes[idx.0 + 1][idx.1] <= val   { n.push((idx.0 +1, idx.1))}
		if idx.1 < self.nodes[0].len() - 1  && self.nodes[idx.0][idx.1 +1] <= val  { n.push((idx.0, idx.1 +1))}
		
		n
	}
	
	fn get_1d_idx(&self, idx :(usize,usize)) -> usize {
		idx.0 * self.nodes[0].len() + idx.1
	}
}

fn shortest_path(g : &Graph) -> u32 {
	
	let mut distances : Vec<usize> = g.nodes.iter().flat_map(|v| vec![usize::MAX; v.len()]).collect();
	let mut heap = BinaryHeap::new();
	
	let start_pos = g.get_1d_idx(g.start);
	
	distances[start_pos] = 0;
	heap.push(NodeDist{node_idx : g.start, dist:0});
	
	while let Some(nd) = heap.pop() {
		let node_1d_pos = g.get_1d_idx(nd.node_idx);
			
		for neighbour in g.get_neigbhours(nd.node_idx) {
			let idx_pos = g.get_1d_idx(neighbour);
			if distances[node_1d_pos] + 1 < distances[idx_pos] {
				distances[idx_pos] = distances[node_1d_pos] +1;
				heap.push(NodeDist{node_idx : neighbour, dist:distances[idx_pos]});
			}
		}
	}
	
	distances[g.get_1d_idx(g.end)] as u32
}

impl Solution for Day12 {
    type Input = Graph;
    type ReturnType = u32;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
       let mut nodes : Vec<Vec<u8>>= lines.map(|l| l.chars().map(|c| (c as u8) ).collect()).collect();
	   let mut start : (usize,usize) = (0,0);
	   let mut end : (usize,usize) = (0,0);
	   
	   for i in 0..nodes.len() {
		for j in 0..nodes[0].len() {
			match nodes[i][j] {
				b'S' => {nodes[i][j] = 0; start = (i,j)},
				b'E' => {nodes[i][j] = 25; end = (i,j)},
				_ => {nodes[i][j] -= b'a';}
			}
		}
	   }
	   
	   Graph { nodes:nodes, start: start,end:end}
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
		shortest_path(&input)
	}
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
		let mut input_copy = input.clone();
		
		let starting_pos : Vec<(usize,usize)> = input_copy.nodes.iter().enumerate().map(|(i, row)| row.iter().enumerate().filter(|(_,&c)| c == 0).map(|(j,_)|(i,j)).collect::<Vec<_>>()).flatten().collect();
		starting_pos.iter().map(|&pos| {
			input_copy.start = pos;
			shortest_path(&input_copy)
		}).min().unwrap_or_default()
	}
}

#[cfg(test)]
mod tests {
    use super::Day12;
    use crate::solution::Solution;

		static INPUT_TEST : &str =
"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day12.parse_input(lines);
		assert_eq!(Day12.first_part(&input),
        31)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day12.parse_input(lines);
		assert_eq!(Day12.second_part(&input),
            29)
    }
}