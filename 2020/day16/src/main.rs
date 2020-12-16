use lazy_static::lazy_static;
use regex::Regex;
// use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: (RangeInclusive<u32>, RangeInclusive<u32>),
}

impl std::str::FromStr for Rule {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<name>.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        }

        match RE.captures(s) {
            Some(caps) => {
                let range1: RangeInclusive<u32> = caps[2].parse().unwrap_or_default()..=caps[3].parse().unwrap_or_default();
                let range2: RangeInclusive<u32> = caps[4].parse().unwrap_or_default()..=caps[5].parse().unwrap_or_default();
                Ok(Rule {
                    name: caps["name"].to_string(),
                    ranges: (range1, range2),
                })
            }
            None => Err("Not a rule"),
        }
    }
}

type Ticket = Vec<u32>;


fn parse_ticket(s: &str) -> Result<Ticket, &'static str> {
    s.split(',').try_fold(Vec::new(), |mut acc, n| {
        acc.push(n.parse::<u32>().map_err(|_| "error parsing")?);
        Ok(acc)
    })
}

fn ticket_error_values(t: &[u32], rules : &[Rule] ) -> Vec<u32> {
    t.iter().filter(|val| rules.iter().all(|r| !r.ranges.0.contains(val) && !r.ranges.1.contains(val))).cloned().collect()
}

fn error_rate(tickets: &[Ticket], rules: &[Rule] ) -> u32 {
    tickets.iter().flat_map(|t| ticket_error_values(t, rules)).sum()
}


fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines().flatten();
    let rules: Vec<Rule> = lines
        .by_ref()
        .take_while(|s| s != "your ticket:")
        .map(|s| s.parse())
        .flatten()
        .collect();

    let tickets: Vec<Ticket> = lines
        .skip_while(|s| s != "nearby tickets:")
        .flat_map(|s| parse_ticket(&s))
        .collect();

    let result = error_rate(&tickets, &rules);

    println!("Hello, world! {:?}", result);
}
