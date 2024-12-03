use crate::solution::Solution;

use regex::Regex;
pub struct Day3;


impl Solution for Day3 {
    type Input = Vec<String>;
    type ReturnType = u32;
    const DAY : u32 = 3;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.collect()
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        let total_memory = input.join("\n");
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        re.captures_iter(&total_memory).map(|c| {
            let (_, [op1, op2]) = c.extract();
            op1.parse::<u32>().unwrap_or_default() * op2.parse::<u32>().unwrap_or_default()
        }).sum()
    }
    fn second_part(&self, input: &Self::Input) -> u32 {
        let total_memory = input.join("\n");
        let re = Regex::new(r"(mul)\((\d{1,3}),(\d{1,3})\)|(do)\(()()\)|(don't)\(()()\)").unwrap();
    
        let mut enabled = true;
        re.captures_iter(&total_memory).map(|c| {
            let (_, [instr,op1, op2]) = c.extract();

            match instr {
                "mul" => if enabled {op1.parse::<u32>().unwrap_or_default() * op2.parse::<u32>().unwrap_or_default() } else {0},
                "do" => { enabled = true; 0},
                "don't" => { enabled = false; 0},
                _ => 0
            }
        }).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Day3;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    static INPUT_TEST_2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";


    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day3.parse_input(lines);
        assert_eq!(Day3.first_part(&input), 161)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST_2.split('\n').map(|s| s.to_string());
        let input = Day3.parse_input(lines);
        assert_eq!(Day3.second_part(&input), 48);
    }
}
