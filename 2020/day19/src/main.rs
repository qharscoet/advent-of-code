use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Chars;

#[derive(Debug)]
enum RuleArg {
    Letter(char),
    SubRule(Vec<Vec<u32>>), // indexes of subrule to validate
}

#[derive(Debug)]
struct Rule {
    idx: u32,
    args: RuleArg,
}

impl std::str::FromStr for Rule {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new("\"([[:alpha:]])\"").unwrap();
        }

        let mut r: Rule = Rule {
            idx: 0,
            args: RuleArg::Letter(' '),
        };

        let mut fields = s.split(":");
        r.idx = fields.next().unwrap().trim().parse().unwrap_or_default();
        let content = fields.next().unwrap().trim();

        match RE.captures(content) {
            Some(caps) => r.args = RuleArg::Letter(caps[1].chars().next().unwrap()),
            None => {
                let subrules: Vec<Vec<u32>> = content
                    .split('|')
                    .map(|s| {
                        s.split_whitespace()
                            .map(|subrule| subrule.parse::<u32>().unwrap())
                            .collect::<Vec<u32>>()
                    })
                    .collect();

                r.args = RuleArg::SubRule(subrules);
            }
        }

        Ok(r)
    }
}

fn match_rule(chars:&mut Chars, rules: &HashMap<u32, Rule>, idx:u32) -> bool {
    match &rules[&idx].args {
        RuleArg::Letter(c) => chars.next().unwrap_or_default() == *c,
        RuleArg::SubRule(subrules) => subrules.iter().any(|subrule| {
            let mut my_chars = chars.clone();
            if subrule.iter().all(|&r| match_rule(&mut my_chars, rules, r)) {
                //if it's true we propagate the remaining chars to the upper call
                *chars = my_chars.clone();
                true
            } else {
                false
            }
        })
    }
}

fn match_rule_start(s:&str, rules: &HashMap<u32, Rule>, idx:u32 ) -> bool {
    let mut chars = s.chars();
    let result = match_rule(&mut chars, rules, idx);
    match chars.next() {
        Some(_) => false,
        None => result
    }

}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines().flatten();
    let rules: HashMap<u32, Rule> = lines
        .by_ref()
        .take_while(|s| s != "")
        .map(|s| {
            let rule: Rule = s.parse().unwrap();
            (rule.idx, rule)
        })
        .collect();

    let count = lines.filter(|s| match_rule_start(s, &rules, 0)).count();
    println!("Hello, world! Part 1 is {}", count);
}
