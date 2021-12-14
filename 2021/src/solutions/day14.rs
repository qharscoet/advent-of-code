use crate::solution::Solution;

use std::collections::HashMap;
pub struct Day14;

#[derive(Debug)]
pub struct Polymer {
    template: String,
    rules: HashMap<String, char>,
    map: HashMap<String, (String, String)>,
}

fn step(template: &String, rules: &HashMap<String, char>) -> String {
    let values = template
        .as_bytes()
        .windows(2)
        .map(|s| {
            let mut as_str = String::from_utf8(s.to_vec()).unwrap_or_default();
            if rules.contains_key(&as_str) {
                as_str.insert(1, rules[&as_str]);
            }

            as_str
        })
        .collect::<Vec<String>>();

    let mut s0 = values[0].clone();
    for s in values.iter().skip(1) {
        s0.push_str(&s[1..])
    }

    s0
}

impl Solution for Day14 {
    type Input = Polymer;
    type ReturnType = u64;

    fn parse_input(&self, mut lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        let template = lines.next().unwrap_or_default();

        let rules: HashMap<String, char> = //HashMap::new();
        lines
            .flat_map(|line| match line.split_once(" -> ") {
                Some((left, right)) => {
                    Some((left.to_string(), right.chars().next().unwrap_or_default()))
                }
                None => None,
            })
            .collect();

        let map: HashMap<String, (String, String)> = rules
            .iter()
            .map(|(s, c)| {
                let mut left = s[0..1].to_string();
                left.push(*c);
                (s.clone(), (left, c.to_string() + &s[1..2]))
            })
            .collect();

        Polymer {
            template: template,
            rules: rules,
            map: map,
        }
    }

    fn first_part(&self, input: &Self::Input) -> u64 {
        let mut template = input.template.clone();

        for _ in 0..10 {
            template = step(&template, &input.rules);
        }

        let mut occurences = [0u32; 255];
        for c in template.chars() {
            occurences[(c as u8) as usize] += 1;
        }

        let max = occurences.into_iter().filter(|&v| v != 0).max().unwrap_or_default();
        let min = occurences.into_iter().filter(|&v| v != 0).min().unwrap_or_default();
        (max - min) as u64
    }

    fn second_part(&self, input: &Self::Input) -> u64 {
        let template = input.template.clone();
        let mut pair_occurences: HashMap<String, u64> = HashMap::new();

        /*Initial count of the pairs we have*/
        for s in template.as_bytes().windows(2) {
            let as_str = String::from_utf8(s.to_vec()).unwrap_or_default();
            pair_occurences.entry(as_str).and_modify(|v| *v += 1).or_insert(1);
        }

        /*Each pair AB -> C is pushed as one AC and one CB*/
        for _ in 0..40 {
            let new_map: HashMap<String, u64> =
                pair_occurences.iter().fold(HashMap::new(), |mut acc, (s, &i)| {
                    if input.map.contains_key(s) {
                        let (new_a, new_b) = &input.map[s];
                        acc.entry(new_a.clone()).and_modify(|v| *v += i).or_insert(i);
                        acc.entry(new_b.clone()).and_modify(|v| *v += i).or_insert(i);
                    } else {
                        acc.entry(s.clone()).and_modify(|v| *v += i).or_insert(i);
                    }

                    acc
                });

            pair_occurences = new_map;
        }

        /*Now counting individual characters, each one except the two ends are
            counted twice as pairs overlaps*/
        let mut char_occurences = [0u64; 255];
        for (s, i) in pair_occurences {
            let chars = s.as_bytes();
            char_occurences[chars[0] as usize] += i;
            char_occurences[chars[1] as usize] += i;
        }

        for i in &mut char_occurences {
            *i /= 2;
        }

        char_occurences[template.chars().next().unwrap() as usize] += 1;
        char_occurences[template.chars().rev().next().unwrap() as usize] += 1;

        let max = char_occurences.into_iter().filter(|&v| v != 0).max().unwrap_or_default();
        let min = char_occurences.into_iter().filter(|&v| v != 0).min().unwrap_or_default();
        max - min
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;

    fn test_input_to_string_iter() -> Vec<String> {
        vec![
            "NNCB",
            "",
            "CH -> B",
            "HH -> N",
            "CB -> H",
            "NH -> C",
            "HB -> C",
            "HC -> B",
            "HN -> C",
            "NN -> C",
            "BH -> H",
            "NC -> B",
            "NB -> B",
            "BN -> B",
            "BB -> N",
            "BC -> B",
            "CC -> N",
            "CN -> C",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn test_first_part() {
        let input = Day14.parse_input(test_input_to_string_iter().into_iter());
        assert_eq!(Day14.first_part(&input), 1588)
    }

    #[test]
    fn test_second_part() {
        let input = Day14.parse_input(test_input_to_string_iter().into_iter());
        assert_eq!(Day14.second_part(&input), 2188189693529)
    }
}
