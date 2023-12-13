use crate::solution::Solution;

pub struct Day13;

type Pattern = Vec<Vec<char>>;

fn transpose(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|row| row[i]).collect())
        .collect()
}

fn get_line(pat: &Pattern) -> Option<usize> {
    let len = pat[0].len();
    (1..pat[0].len()).find(|&i| {
        let range = if i <= len / 2 { i } else { len - i };

        pat.iter().all(|line| {
            let left: String = line[(i - range)..i].iter().collect();
            let right: String = line[i..(i + range)].iter().rev().collect();

            left == right
        })
    })
}
impl Solution for Day13 {
    type Input = Vec<Pattern>;
    type ReturnType = u32;
    const DAY: u32 = 13;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        let mut peekable = lines.peekable();

        let mut patterns = Vec::new();
        while peekable.peek().is_some() {
            patterns.push(
                peekable
                    .by_ref()
                    .take_while(|line| line != "")
                    .map(|line| line.chars().collect())
                    .collect(),
            );
        }

        patterns
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        input
            .iter()
            .map(|pat| {
                if let Some(i) = get_line(&pat) {
                    i as u32
                } else {
                    100 * get_line(&transpose(&pat)).unwrap_or_default() as u32
                }
            })
            .sum::<u32>()
    }
    fn second_part(&self, _input: &Self::Input) -> Self::ReturnType {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Day13;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    static INPUT_TEST_2: &str = "";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day13.parse_input(lines);
        assert_eq!(Day13.first_part(&input), 405)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST_2.split('\n').map(|s| s.to_string());
        let input = Day13.parse_input(lines);
        assert_eq!(Day13.second_part(&input), u32::MAX)
    }
}
