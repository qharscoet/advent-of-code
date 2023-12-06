use std::collections::HashSet;

use crate::solution::Solution;

pub struct Day4;

#[derive(Debug)]
pub struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    have_numbers: HashSet<u32>,
}
impl std::str::FromStr for Card {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((card, numbers)) = s.split_once(':') else {
            return Err("String not in the right format !");
        };

        //Card id, may be unnecessary but hey
        let Some((_, str_id)) = card.split_once(' ') else {
            return Err("String not in the right format !");
        };
        let Ok(id) = str_id.trim().parse::<u32>() else {
            return Err("Id not a number");
        };

        let Some((win, have)) = numbers.split_once('|') else {
            return Err("String not in the right format !");
        };

        Ok(Card {
            id: id,
            winning_numbers: win.trim().split(' ').flat_map(|n| n.parse()).collect(),
            have_numbers: have.trim().split(' ').flat_map(|n| n.parse()).collect(),
        })
    }
}

impl Solution for Day4 {
    type Input = Vec<Card>;
    type ReturnType = u32;
    const DAY: u32 = 4;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.map(|line| line.parse().unwrap()).collect()
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        input
            .iter()
            .map(|card| {
                let count = card.winning_numbers.intersection(&card.have_numbers).count() as u32;
                if count > 0 {2u32.pow(count -1)} else {0}
            })
            .sum()
    }
    fn second_part(&self, input: &Self::Input) -> u32 {
        let mut counts = vec![1u32; input.len()];

        for card in input {
            let won = card.winning_numbers.intersection(&card.have_numbers).count() as u32;
            for i in card.id..card.id+won {
                counts[i as usize] += counts[card.id as usize - 1]; 
            }
        }
        
        counts.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Day4;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day4.parse_input(lines);
        assert_eq!(Day4.first_part(&input), 13)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day4.parse_input(lines);
        assert_eq!(Day4.second_part(&input), 30)
    }
}
