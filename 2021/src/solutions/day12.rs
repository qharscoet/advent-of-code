use crate::solution::Solution;

use std::collections::{HashMap, HashSet};

pub struct Day12;

type Caves = HashMap<String, HashSet<String>>;


impl Solution for Day12 {
	type Input = Caves;
	type ReturnType = u32;

	fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
		let mut caves : Caves = HashMap::new();

		for line in lines {
			let (s,e) = line.split_once("-").unwrap_or_default();
			caves.entry(s.to_string()).and_modify(|v| {v.insert(e.to_string());}).or_insert(HashSet::from([e.to_string()]));
			caves.entry(e.to_string()).and_modify(|v| {v.insert(s.to_string());}).or_insert(HashSet::from([s.to_string()]));
		}
		caves
	}

	fn first_part(&self, input: &Self::Input) -> u32 {

		fn explore(n : String,  caves: &Caves, visited: &mut HashSet<String>, curr_path: &mut Vec<String>, paths: &mut Vec<String>) {
			if n == "end" {
				curr_path.push("end".to_string());
				paths.push(curr_path.join(","));
				curr_path.pop();
			} else {
				if n.chars().all(|c| c.is_uppercase()) || !visited.contains(&n) {
					visited.insert(n.clone());
					curr_path.push(n.clone());

					for cave in &caves[&n] {
						explore(cave.clone(), caves, visited, curr_path, paths)
					}

					visited.remove(&n);
					curr_path.pop();
				}
			}

		}

		let mut visited : HashSet<String> = HashSet::new();
		let mut path = vec![];
		let mut paths = vec![];

		explore("start".to_string(), &input, &mut visited, &mut path, &mut paths);

		paths.len() as u32

	}

	fn second_part(&self, input: &Self::Input) -> u32 {
		0
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::solution::Solution;

	fn test_input_to_string_iter() -> Vec<String> {
		vec![
			"start-A",
			"start-b",
			"A-c",
			"A-b",
			"b-d",
			"A-end",
			"b-end",
		]
		.iter()
		.map(|s| s.to_string())
		.collect()
	}

	fn test_input2_to_string_iter() -> Vec<String> {
		vec![
			"dc-end",
			"HN-start",
			"start-kj",
			"dc-start",
			"dc-HN",
			"LN-dc",
			"HN-end",
			"kj-sa",
			"kj-HN",
			"kj-dc",
		]
		.iter()
		.map(|s| s.to_string())
		.collect()
	}

	fn test_input3_to_string_iter() -> Vec<String> {
		vec![
			"fs-end",
			"he-DX",
			"fs-he",
			"start-DX",
			"pj-DX",
			"end-zg",
			"zg-sl",
			"zg-pj",
			"pj-he",
			"RW-he",
			"fs-DX",
			"pj-RW",
			"zg-RW",
			"start-pj",
			"he-WI",
			"zg-he",
			"pj-fs",
			"start-RW",
		]
		.iter()
		.map(|s| s.to_string())
		.collect()
	}

	#[test]
	fn test_first_part() {
		let input = Day12.parse_input(test_input_to_string_iter().into_iter());
		assert_eq!(Day12.first_part(&input), 10)
	}

	#[test]
	fn test_first_part2() {
		let input = Day12.parse_input(test_input2_to_string_iter().into_iter());
		assert_eq!(Day12.first_part(&input), 19)
	}

	#[test]
	fn test_first_part3() {
		let input = Day12.parse_input(test_input3_to_string_iter().into_iter());
		assert_eq!(Day12.first_part(&input), 226)
	}

	#[test]
	fn test_second_part() {
		let input = Day12.parse_input(test_input_to_string_iter().into_iter());
		assert_eq!(Day12.second_part(&input), 36)
	}

	#[test]
	fn test_second_part2() {
		let input = Day12.parse_input(test_input2_to_string_iter().into_iter());
		assert_eq!(Day12.second_part(&input), 103)
	}

	#[test]
	fn test_second_part3() {
		let input = Day12.parse_input(test_input3_to_string_iter().into_iter());
		assert_eq!(Day12.second_part(&input), 3509)
	}
}