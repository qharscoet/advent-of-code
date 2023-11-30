use std::{vec, cmp, collections::VecDeque, fmt::Display};

use crate::solution::Solution;

pub struct Day13;

#[derive(Clone,Debug, PartialEq, Eq)]
pub enum Value {
	Integer(u32),
	List(VecDeque<Value>)
}


impl std::str::FromStr for Value {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
		
		let ret = match s.chars().next() {
			Some('[') => {
				let mut count = 0;
				Ok(Value::List(s[1..(s.len() -1)].split(|c| {
					match c {
						'[' => {count += 1; false},
						']' => {count -= 1; false},
						',' => if count == 0 { true} else {false},
						_ => { false }
					}
				}).flat_map(|val| val.parse()).collect()))
			},
			Some(_) => Ok(Value::Integer(s.parse().unwrap_or_default())),
			None => Err("Error while parshing the value"),
		};	
		ret
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
		//println!("{} and {}", self, other);
        match (self, other) {
			(Value::Integer(li), Value::Integer(ri)) => { li.cmp(ri)},
			(Value::Integer(li), Value::List(_)) => { Value::List(vec![Value::Integer(*li)].into()).cmp(other) },
			(Value::List(_), Value::Integer(ri)) => { self.cmp(&Value::List(vec![Value::Integer(*ri)].into()))},
			(Value::List(ll), Value::List(rl)) => {
				let mut lcopy = ll.clone();
				let mut rcopy = rl.clone();
				
				loop {
					match (lcopy.pop_front(), rcopy.pop_front()) {
						(None, None) => break cmp::Ordering::Equal,
						(None, Some(_)) => break cmp::Ordering::Less,
						(Some(_), None) => break cmp::Ordering::Greater,
						(Some(lv), Some(rv)) => {
							match lv.cmp(&rv) {
								cmp::Ordering::Equal => {},
								val => break val,
							}
						} ,
					}
				}
			},
		}
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for Value {
    fn default() -> Self { Value::Integer(0) }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Value::Integer(v) => v.fmt(f),
			Value::List(l) => {
				write!(f, "[")?;
				for elem in l {
					elem.fmt(f)?;
					write!(f,",")?;
				}
				write!(f, "]")
			},
		}
    }
}

impl Solution for Day13 {
    type Input = Vec<[Value;2]>;
    type ReturnType = u32;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
       let collected : Vec<_> = lines.collect();
		collected.chunks(3).map(|w| [w[0].parse().unwrap_or_default(), w[1].parse().unwrap_or_default()]).collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
		input.iter().enumerate().filter(|(_idx,[l,r])| l < r ).map(|(idx, _val)| (idx +1) as u32).sum()
	}
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
		let mut values = input.iter().flatten().collect::<Vec<_>>();
		let dividers : Vec<Value> = vec!["[[2]]".parse().unwrap(), "[[6]]".parse().unwrap()];
		values.extend(dividers.iter());
		values.sort();
		values.iter().enumerate().filter(|&(_dir,&v)| dividers.contains(v)).map(|(idx, _val)| (idx +1) as u32).product()
	}
}

#[cfg(test)]
mod tests {
    use super::Day13;
    use crate::solution::Solution;

		static INPUT_TEST : &str =
"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day13.parse_input(lines);
		assert_eq!(Day13.first_part(&input),
        13)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day13.parse_input(lines);
		assert_eq!(Day13.second_part(&input),
            140)
    }
}