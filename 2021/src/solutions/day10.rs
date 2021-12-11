use crate::solution::Solution;

pub struct Day10;

fn get_closing(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '<' => '>',
        '{' => '}',
        _ => 0 as char,
    }
}

fn get_error_score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn get_autocomplete_score(c: char) -> u32 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

enum State {
    Complete(),
    Incomplete(String),
    Corrupt(char),
}

fn parse_line(s: &str) -> State {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '<' | '{' => stack.push(c),
            ')' | ']' | '>' | '}' => {
                if let Some(top) = stack.pop() {
                    if get_closing(top) != c {
                        return State::Corrupt(c);
                    }
                }
            }
            _ => {}
        }
    }

    if stack.is_empty() {
        State::Complete()
    } else {
        State::Incomplete(stack.iter().rev().map(|&c| get_closing(c)).collect())
    }
}

impl Solution for Day10 {
    type Input = Vec<String>;
    type ReturnType = u64;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.collect()
    }

    fn first_part(&self, input: &Self::Input) -> u64 {
        input
            .iter()
            .map(|line| match parse_line(line) {
                State::Complete() => 0,
                State::Incomplete(_s) => 0,
                State::Corrupt(c) => get_error_score(c) as u64,
            })
            .sum()
    }

    fn second_part(&self, input: &Self::Input) -> u64 {
        let mut scores : Vec<u64> = input
            .iter()
            .map(|line| match parse_line(line) {
                State::Complete() => 0,
                State::Incomplete(s) => s.chars().map(|c| get_autocomplete_score(c) as u64).fold(0, |acc, v| acc*5 +v),
                State::Corrupt(_c) => 0,
            })
            .filter(|score| score > &0)
            .collect();

        scores.sort();
        scores[scores.len()/2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;

    fn test_input_to_string_iter() -> Vec<String> {
        vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn test_first_part() {
        let input = Day10.parse_input(test_input_to_string_iter().into_iter());
        assert_eq!(Day10.first_part(&input), 26397)
    }

    #[test]
    fn test_second_part() {
        let input = Day10.parse_input(test_input_to_string_iter().into_iter());
        assert_eq!(Day10.second_part(&input), 288957)
    }
}
