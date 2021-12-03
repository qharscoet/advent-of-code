use crate::solution::Solution;

pub struct Day3;

fn transpose(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|row| row[i]).collect())
        .collect()
}

impl Solution for Day3 {
    type Input = Vec<Vec<char>>;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect()
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        let transposed = transpose(input);
        let gamma = transposed
            .iter()
            .map(|col| {
                // u32::from_str_radix(&col.iter().collect::<String>(), 2)
                //     .unwrap_or_default()
                //     .count_ones()
                col.iter().filter(|&&c| c == '1').count() > input.len() / 2
            })
            .rev()
            .enumerate()
            .fold(0, |acc, (i, b)| acc | ((b as u32) << i));

        let mask = (1 << transposed.len()) - 1;
        let epsilon = !gamma & mask;
        gamma * epsilon
    }
    
    fn second_part(&self, _input: &Self::Input) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Day3;
    use crate::solution::Solution;

    #[test]
    fn test_first_part() {
        let strings: Vec<String> = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let input = Day3.parse_input(strings.into_iter());
        assert_eq!(Day3.first_part(&input), 198)
    }

    #[test]
    fn test_second_part() {
        panic!()
    }
}
