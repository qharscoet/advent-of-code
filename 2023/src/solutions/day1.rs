use crate::solution::Solution;

pub struct Day1;

fn digit_str_to_u32(s:&str) -> u32
{
    match s 
    {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0
    }
}

impl Solution for Day1 {
    type Input = Vec<String>;
    type ReturnType = u32;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.map(|line| line).collect()
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        input.iter().map(|line| {
            let first_digit = line.chars().find(char::is_ascii_digit).expect("No digit found").to_digit(10).expect("that's no number!");
            let last_digit = line.chars().rev().find(char::is_ascii_digit).expect("No digit found").to_digit(10).expect("that's no number!");
            first_digit * 10 + last_digit
        }
        ).sum()
    }
    fn second_part(&self, input: &Self::Input) -> u32 {
        input.iter().map(|line| {
            let digit_strings = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

            let first_str_digit = digit_strings.iter().map(|digit_str| (digit_str_to_u32(&digit_str),line.find(digit_str).unwrap_or(usize::MAX))).min_by_key(|(_,idx)| *idx).unwrap_or((0, usize::MAX));
            let first_real_digit = line.char_indices().find(|(_, c)| c.is_ascii_digit()).map(|(idx,c)| (c.to_digit(10).expect("Not a number"), idx)).unwrap_or((0, usize::MAX));
            
            let last_str_digit = digit_strings.iter().map(|digit_str| (digit_str_to_u32(&digit_str), line.rfind(digit_str).unwrap_or(usize::MIN))).max_by_key(|(_,idx)| *idx).unwrap_or((0, usize::MIN));
            let last_real_digit = line.char_indices().rfind(|(_, c)| c.is_ascii_digit()).map(|(idx,c)| (c.to_digit(10).expect("Not a number"), idx)).unwrap_or((0, usize::MIN));

            let first_digit = std::cmp::min_by_key(first_str_digit, first_real_digit, |(_,idx)| *idx).0;
            let last_digit = std::cmp::max_by_key(last_str_digit, last_real_digit, |(_,idx)| *idx).0;

            first_digit * 10 + last_digit
        }).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Day1;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    static INPUT_TEST_2 : &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day1.parse_input(lines);
        assert_eq!(Day1.first_part(&input), 142)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST_2.split('\n').map(|s| s.to_string());
        let input = Day1.parse_input(lines);
        assert_eq!(Day1.second_part(&input), 281)
    }
}
