use crate::solution::Solution;

pub struct Day9;

fn get_hist(line: &Vec<<Day9 as Solution>::ReturnType>) -> <Day9 as Solution>::Input {
    let mut hist: <Day9 as Solution>::Input = Vec::new();
    hist.push(line.clone());
    // let mut seq = hist.last
    while !hist.last().unwrap().iter().all(|v| v == &0) {
        hist.push(
            hist.last()
                .unwrap()
                .windows(2)
                .map(|v| v[1] - v[0])
                .collect(),
        );
    }

    hist
}
impl Solution for Day9 {
    type Input = Vec<Vec<i64>>;
    type ReturnType = i64;
    const DAY: u32 = 9;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|v| v.parse().expect("not a i32"))
                    .collect()
            })
            .collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        input
            .iter()
            .map(|line| {
                let hist = get_hist(line);
                hist.iter()
                    .rev()
                    .fold(0, |acc, vec| acc + vec.last().unwrap())
            })
            .sum()
    }
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        input
            .iter()
            .map(|line| {
                let hist = get_hist(line);
                hist.iter()
                    .rev()
                    .fold(0, |acc, vec| vec.first().unwrap() - acc)
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Day9;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day9.parse_input(lines);
        assert_eq!(Day9.first_part(&input), 114)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day9.parse_input(lines);
        assert_eq!(Day9.second_part(&input), 2)
    }
}
