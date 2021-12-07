use crate::solution::Solution;

pub struct Day7;

impl Solution for Day7 {
    type Input = Vec<u32>;
    type ReturnType = u32;

    fn parse_input(&self, mut lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines
            .next()
            .unwrap_or_default()
            .split(',')
            .flat_map(|n| n.parse())
            .collect()
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        let min: u32 = *input.iter().min().unwrap_or(&0);
        let max: u32 = *input.iter().max().unwrap_or(&0);

        (min..max)
            .map(|n| {
                input
                    .iter()
                    .map(|&v| i32::abs(n as i32 - v as i32) as u32)
                    .sum()
            })
            .min()
            .unwrap_or_default()
    }

    fn second_part(&self, input: &Self::Input) -> u32 {
        let min: u32 = *input.iter().min().unwrap_or(&0);
        let max: u32 = *input.iter().max().unwrap_or(&0);

        (min..max)
            .map(|n| {
                input
                    .iter()
                    .map(|&v| {
                        let dist = i32::abs(n as i32 - v as i32) as u32;
                        (dist * (dist + 1)) / 2
                    })
                    .sum()
            })
            .min()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;

    fn test_input_to_string_iter() -> Vec<String> {
        vec!["16,1,2,0,4,2,7,1,2,14"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn test_first_part() {
        let input = Day7.parse_input(test_input_to_string_iter().into_iter());
        assert_eq!(Day7.first_part(&input), 37)
    }

    #[test]
    fn test_second_part() {
        let input = Day7.parse_input(test_input_to_string_iter().into_iter());
        assert_eq!(Day7.second_part(&input), 168)
    }
}
