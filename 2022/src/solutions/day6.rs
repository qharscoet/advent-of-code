use std::collections::HashSet;

use crate::solution::Solution;

pub struct Day6;

fn find_packet(input : &String, win_size : usize) -> usize{
    input.as_bytes()
			.windows(win_size)
			.position(|w| w.iter().collect::<HashSet<&u8>>().len() == win_size) //Creating a set may not be optimal
			.unwrap_or_default() + win_size // Adding win_size as the expected answer is the pos of the last char
}

impl Solution for Day6 {
    type Input = String;
    type ReturnType = usize;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
       lines.collect::<Vec<String>>()[0].clone()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        find_packet(input, 4) 
    }
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        find_packet(input, 14)
    }
}

#[cfg(test)]
mod tests {
    use super::Day6;
    use crate::solution::Solution;

	static INPUT_TEST : [(&str, usize, usize); 5] =
[("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26)];

    #[test]
    fn test_first_part() {
        for (s, v,_) in INPUT_TEST {
			assert_eq!(Day6.first_part(&s.to_string()),v)
		}
    }

    #[test]
    fn test_second_part() {
        for (s, _, v) in INPUT_TEST {
			assert_eq!(Day6.second_part(&s.to_string()),v)
		}
    }
}