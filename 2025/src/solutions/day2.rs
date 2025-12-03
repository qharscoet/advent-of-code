use crate::solution::Solution;

pub struct Day2;

#[derive(Debug)]
pub struct Range {
    start: u64,
    end: u64,
}

impl std::str::FromStr for Range {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').map(|(s,e)| (s.trim().parse().unwrap_or_default(), e.trim().parse().unwrap_or_default())).unwrap_or_default();
        Ok(Range{start, end})
    }
}

fn split_int_in_half(value: u64) -> (u64,u64) {
    let digit_count = (value as f32).log10().ceil() as u32;
    let base = 10u64.pow(digit_count / 2);

    (value / base, value % base)
}


fn is_int_invalid(value:u64) -> bool {
    let (start, end) = split_int_in_half(value);
    
    start == end
}

fn split_int_by_digits(value: u64, digits: u32) -> Option<impl Iterator<Item = u64>>  {
    let digit_count = ((value+1) as f32).log10().ceil() as u32; //+1 is to handle exact powers of ten correctly

    //We want only equal parts, else numebrs like 20202 would give [2,2,2]
    if digit_count%digits != 0 {
        return None;
    }
    /* Some arithmetic magic to chunk numbers.
        For example, for 123456 and digits=2
            value / 10^0 % 10^2 = value % 100 = 56
            value / 10^2 % 10^2 = 1234,56 % 100 = 34
            value / 10^4 % 10^2 = 12,3456 % 100 = 12
    */
    Some((0..digit_count).step_by(digits as usize).map(move |exp| (value / 10u64.pow(exp) % 10u64.pow(digits)) as u64 ))
}

fn is_int_invalid_2(value:u64) -> bool {
    let digit_count = (value as f32).log10().ceil() as u32;
    //We try to cut the number into parts of i digits, and if possible,  see if all parts are equal
    (1..=(digit_count/2)).any(|i| 
        if let Some(mut parts) = split_int_by_digits(value, i as u32) {
            let first = parts.next();
            parts.all(|p| Some(p) == first)
        }
        else { false })
}

impl Solution for Day2 {
    type Input = Vec<Range>;
    type ReturnType = u64;
    const DAY : u32 = 2;

    fn parse_input(&self, mut lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.next().unwrap()
            .split(',')
            .flat_map(|part| part.parse::<Range>())
            .collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        input.iter().flat_map(|range| {
            range.start..=range.end
        }).filter(|v| is_int_invalid(*v)).sum()
    }
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        input.iter().flat_map(|range| {
            range.start..=range.end
        }).filter(|v| is_int_invalid_2(*v)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Day2;
    use crate::{solution::Solution, solutions::day2::{is_int_invalid_2, split_int_in_half}};

    static INPUT_TEST: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";


    #[test]
    fn test_split_int_in_half() {
        assert_eq!(split_int_in_half(1234), (12,34));
        assert_eq!(split_int_in_half(12345), (123,45));
        assert_eq!(split_int_in_half(1), (1,0)); // expected behavior, should we make it panic instead?
        assert_eq!(split_int_in_half(12), (1,2));
        assert_eq!(split_int_in_half(123), (12,3));
    }
    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day2.parse_input(lines);
        assert_eq!(Day2.first_part(&input), 1227775554)
    }

    #[test]
    fn test_is_invalid_2() {
        assert_eq!(is_int_invalid_2(123456789), false);
        assert_eq!(is_int_invalid_2(123123), true);
        assert_eq!(is_int_invalid_2(100), false);
        assert_eq!(is_int_invalid_2(1000), false);
        assert_eq!(is_int_invalid_2(20202), false);
    }
    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day2.parse_input(lines);
        assert_eq!(Day2.second_part(&input), 4174379265);
    }
}
