use crate::solution::Solution;

pub struct Day3;

fn transpose(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|row| row[i]).collect())
        .collect()
}

fn select_by_criteria(values: &Vec<Vec<char>>,  f: fn(usize, usize) -> bool ) -> u32 {
    fn select_for_column(values: &Vec<Vec<char>>, i : usize, f:fn(usize, usize) -> bool) -> u32 {
        match values.len() {
            1 => u32::from_str_radix(&values[0].iter().collect::<String>(), 2).unwrap_or_default(),
            _ => {
                let one = f(values.iter().map(|row| row[i]).filter(|&c| c == '1').count(), (values.len() +1)/ 2);
                let new_list: Vec<Vec<char>> = values
                .iter()
                .filter(|row| row[i] == if one { '1' } else { '0' })
                .cloned()
                .collect();
                select_for_column(&new_list, i+1, f)
            }
        }
    }

    select_for_column(values, 0, f)
}

impl Solution for Day3 {
    type Input = Vec<Vec<char>>;
    type ReturnType = u32;

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

    fn second_part(&self, input: &Self::Input) -> u32 {
        select_by_criteria(input, |a,b| a >= b) * select_by_criteria(input, |a,b| a < b)
    }
}

#[cfg(test)]
mod tests {
    use super::Day3;
    use crate::solution::Solution;

    fn test_input_to_string_iter() -> Vec<String> {
        vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn test_first_part() {
        let input = Day3.parse_input(test_input_to_string_iter().into_iter());
        assert_eq!(Day3.first_part(&input), 198)
    }

    #[test]
    fn test_second_part() {
        let input = Day3.parse_input(test_input_to_string_iter().into_iter());
        assert_eq!(Day3.second_part(&input), 230)
    }
}
