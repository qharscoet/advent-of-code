use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn match_rule(rules: &HashMap<u32, Rule>, idx:u32) -> bool {
    false
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines().flatten();
    let rules: HashMap<u32, Rule> = lines
        .take_while(|s| s != "")
        .map(|s| {
            let rule: Rule = s.parse().unwrap();
            (rule.idx, rule)
        })
        .collect();

    println!("Hello, world!");
}
