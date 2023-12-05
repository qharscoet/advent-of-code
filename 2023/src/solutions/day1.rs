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
        "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => s.parse().unwrap_or_default(),
        _ => 0
    }
}

impl Solution for Day1 {
    type Input = Vec<String>;
    type ReturnType = u32;
    const DAY : u32 = 1;

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
            //Ugly, having a map and merging key and values may be cleaner
            let digit_strings = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1" , "2" , "3" , "4" , "5" , "6" , "7" , "8" , "9" ];

            let first_digit = digit_strings.iter().map(|digit_str| (digit_str_to_u32(&digit_str),line.find(digit_str).unwrap_or(usize::MAX))).min_by_key(|(_,idx)| *idx).unwrap_or((0, usize::MAX)).0;
            let last_digit = digit_strings.iter().map(|digit_str| (digit_str_to_u32(&digit_str), line.rfind(digit_str).unwrap_or(usize::MIN))).max_by_key(|(_,idx)| *idx).unwrap_or((0, usize::MIN)).0;

            first_digit * 10 + last_digit
        }).sum()
    }
}

#[allow(dead_code)]
impl Day1 {

    //Rust implem from https://github.com/jonathanpaulson 's solution from the leaderboard, did not think of it at first...
    fn second_part_opti(&self, input : &<Day1 as Solution>::Input) -> u32 {
        input.iter().map(|line| {
            let digit_strings = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

            let digits = line.chars().enumerate().fold(Vec::new(), |mut acc, (idx, c)| {
                if c.is_ascii_digit() {
                    acc.push(c.to_digit(10).unwrap_or_default());
                } 

                for (d, str) in digit_strings.iter().enumerate() {
                        if line[idx..].starts_with(str){
                            acc.push((d +1 ) as u32);
                        }
                }

                acc 
            });

            digits.first().unwrap() * 10 + digits.last().unwrap()
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
        assert_eq!(Day1.second_part(&input), 281);
        assert_eq!(Day1.second_part_opti(&input), 281);
    }
}
