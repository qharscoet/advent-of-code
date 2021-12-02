use crate::solution::Solution;

pub struct Day2;

#[derive(Debug)]
pub enum Action {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl std::str::FromStr for Action {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, value) = s.split_once(" ").unwrap_or_default();

        let v: u32;
        match value.trim().parse() {
            Err(_e) => return Err("v is not an int"),
            Ok(data) => v = data,
        }

        match direction {
            "forward" => Ok(Action::Forward(v)),
            "up" => Ok(Action::Up(v)),
            "down" => Ok(Action::Down(v)),
            _ => Err("Action not valid"),
        }
    }
}

impl Solution for Day2 {
    type Input = Vec<Action>;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.flat_map(|line| line.parse()).collect()
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        let (pos, depth) = input.iter().fold((0, 0), |(pos, depth), a| match a {
            Action::Forward(v) => (pos + v, depth),
            Action::Up(v) => (pos, depth - v),
            Action::Down(v) => (pos, depth + v),
        });

        pos * depth
    }
    fn second_part(&self, _input: &Self::Input) -> u32 {
        42
    }
}

#[cfg(test)]
mod tests {
    use super::Day2;
    use crate::solution::Solution;

    #[test]
    fn test_first_part() {
        let strings: Vec<String> = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let input = Day2.parse_input(strings.into_iter());
        assert_eq!(Day2.first_part(&input), 150);
    }

    #[test]
    fn test_second_part() {
        panic!()
    }
}
