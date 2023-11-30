use crate::solution::Solution;

use std::collections::HashSet;

pub struct Day3;



// pub struct Rucksack {
//     compartments: [Vec<char>;2],
// }

type Rucksack = Vec<char>;

fn get_priority(c:char) -> u8 {
    match c {
        'a'..='z' => (c as u8) - b'a' +1,
        'A'..='Z' => (c as u8) - b'A' + 27,
        _ => 0
    }
}


impl Solution for Day3 {
    type Input = Vec<Rucksack>;
    type ReturnType = u32;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.map(|l| l.chars().collect()).collect()
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        input.iter().map(|rucksack| {
            let first_set = rucksack[..rucksack.len()/2].iter().collect::<HashSet<&char>>();
            let second_set = rucksack[rucksack.len()/2..].iter().collect::<HashSet<&char>>();
            get_priority(**first_set.intersection(&second_set).next().unwrap_or(&&' ')) as u32
        } ).sum()
    }
    fn second_part(&self, input: &Self::Input) -> u32 {
        input.chunks(3).map(|w| {
            let mut first = w[0].iter().collect::<HashSet<&char>>();
            first.retain(|c| w[1..].iter().all(|l| l.contains(c)));
            get_priority(**first.iter().next().unwrap_or(&&' ' )) as u32
        }).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Day3;
    use crate::solution::Solution;

	static INPUT_TEST : &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day3.parse_input(lines);
		assert_eq!(Day3.first_part(&input),
        157)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day3.parse_input(lines);
		assert_eq!(Day3.second_part(&input),
            70)
    }
}
