use std::{collections::VecDeque};

use crate::solution::Solution;

pub struct Day11;

#[derive(Debug, Clone)]
enum Operation {
	Add(u32),
	Multiply(u32),
}

#[derive(Debug, Clone)]
pub struct Monkey {
	items : VecDeque<u64>,
	op : Option<Operation>,
	test : u64, //The value tested for divisible
	if_true : usize,
	if_false : usize, //both are monkeys index;
	total_inspects : u64,
}

fn monkey_round(monkeys:&mut Vec<Monkey>, p2 : bool) {
	for idx in 0..monkeys.len() {
		while let Some(item) = monkeys[idx].items.pop_front() {
			monkeys[idx].total_inspects += 1;
			let mut worry = item;
			if let Some(op) = &monkeys[idx].op {
				match op {
					Operation::Add(v) => worry += if *v == 0 { worry} else {*v as u64},
					Operation::Multiply(v) => worry *= if *v == 0 { worry} else {*v as u64},
				}
			}

			if p2 {
				let product_prime :u64 = monkeys.iter().map(|m| m.test).product();
				worry %= product_prime;
			} else {
				worry /= 3;
			}

			if worry%monkeys[idx].test == 0 {
				
				let true_idx = monkeys[idx].if_true;
				monkeys[true_idx].items.push_back(worry);
			} else {	
				let false_idx = monkeys[idx].if_false;
				monkeys[false_idx].items.push_back(worry);
			}
		}
	}
}


impl Solution for Day11 {
    type Input = Vec<Monkey>;
    type ReturnType = u64;
	
	/*
	Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3 */

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
		let collected : Vec<_> = lines.collect();
		collected.split(|l| l.is_empty()).map(|s| {
			Monkey {
				items : s[1].split(":").last().unwrap_or_default().split(',').flat_map(|val| val.trim().parse()).collect(),
				op : if let Some(p) =  s[2].split("= old").last().unwrap_or_default().trim().split_once(" ") {
					match p.0 {
						"*" => Some(Operation::Multiply(p.1.parse().unwrap_or_default())),
						"+" => Some(Operation::Add(p.1.parse().unwrap_or_default())),
						_ => None
					} 
				} else { None },
				test : s[3].split_ascii_whitespace().last().unwrap_or_default().parse().unwrap_or_default(),
				if_true : s[4].split_ascii_whitespace().last().unwrap_or_default().parse().unwrap_or_default(),
				if_false : s[5].split_ascii_whitespace().last().unwrap_or_default().parse().unwrap_or_default(),
				total_inspects : 0,
			}
		}).collect::<Vec<Monkey>>()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
		let mut monkeys_copy = input.clone();
		for _ in 0..20 {
			monkey_round(&mut monkeys_copy, false);
		}
		let mut inspects : Vec<_> = monkeys_copy.iter().map(|m| m.total_inspects).collect();
		inspects.sort();
		inspects.iter().rev().take(2).product()
	}
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
		let mut monkeys_copy = input.clone();
		for _ in 0..10000 {
			monkey_round(&mut monkeys_copy, true);
		}
		let mut inspects : Vec<_> = monkeys_copy.iter().map(|m| m.total_inspects).collect();
		inspects.sort();
		inspects.iter().rev().take(2).product()
	}
}

#[cfg(test)]
mod tests {
    use super::Day11;
    use crate::solution::Solution;

		static INPUT_TEST : &str =
"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day11.parse_input(lines);
		assert_eq!(Day11.first_part(&input),
        10605)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day11.parse_input(lines);
		assert_eq!(Day11.second_part(&input),
		2713310158)
    }
}