use crate::solution::Solution;

pub struct Day13;

type Pattern = Vec<Vec<bool>>;

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|row| row[i]).collect())
        .collect()
}

fn get_line(pat: &Pattern, filter: Option<usize>) -> Option<usize> {
    let len = pat[0].len();
    (1..pat[0].len())
        .filter(|&i| match filter {
            Some(v) => i != v,
            None => true,
        })
        .find(|&i| {
            let range = if i <= len / 2 { i } else { len - i };

            pat.iter().all(|line| {
                let left: Vec<_> = line[(i - range)..i].iter().collect();
                let right: Vec<_> = line[i..(i + range)].iter().rev().collect();

                left == right
            })
        })
}

fn get_answer(pat: &Pattern, filter: Option<usize>) -> u32 {
    if let Some(i) = get_line(&pat, filter) {
        i as u32
    } else {
        100 * get_line(&transpose(&pat), filter.and_then(|f| Some(f / 100))).unwrap_or_default()
            as u32
    }
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
                    .map(|line| {
                        line.chars()
                            .flat_map(|c| match c {
                                '.' => Some(false),
                                '#' => Some(true),
                                _ => None,
                            })
                            .collect()
                    })
                    .collect(),
            );
        }

        patterns
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        input.iter().map(|pat| get_answer(&pat, None)).sum::<u32>()
    }
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        input
            .iter()
            .map(|pat| {
                let old_line = get_answer(&pat, None);

                let mut vals = (0..pat.len()).flat_map(|r| (0..pat[0].len()).map(move |c| (r, c)));
                vals.find_map(|(r, c)| {
                    let mut pat_copy = pat.clone();
                    pat_copy[r][c] = !pat[r][c];

                    let line = get_answer(&pat_copy, Some(old_line as usize));

                    if line != old_line && line != 0 {
                        Some(line)
                    } else {
                        None
                    }
                })
                .unwrap() as u32
            })
            .sum::<u32>()
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

    static INPUT_ERROR: &str = "##.#.##
##.#.##
..#.#..
.######
#...#.#
#.#####
#.###.#";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day13.parse_input(lines);
        assert_eq!(Day13.first_part(&input), 405)
    }

    #[test]
    fn test_smudge() {
        let lines = INPUT_ERROR.split('\n').map(|s| s.to_string());
        let input = Day13.parse_input(lines);
        assert_ne!(Day13.second_part(&input), 0)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day13.parse_input(lines);
        assert_eq!(Day13.second_part(&input), 400)
    }
}
