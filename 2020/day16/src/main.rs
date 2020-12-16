use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashSet, HashMap};
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

impl Rule {
    fn valid_value(&self, v:&u32) -> bool {
        self.ranges.0.contains(v) || self.ranges.1.contains(v)
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
    t.iter().filter(|val| rules.iter().all(|r| !r.valid_value(val))).cloned().collect()
}

fn error_rate(tickets: &[Ticket], rules: &[Rule] ) -> u32 {
    tickets.iter().flat_map(|t| ticket_error_values(t, rules)).sum()
}

fn value_can_be_rule(tickets: &[Ticket], i:usize, r : &Rule) -> bool {
    tickets.iter().map(|t| t[i]).all(|v| r.valid_value(&v))
}


fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines().flatten();
    let rules: Vec<Rule> = lines
        .by_ref()
        .take_while(|s| s != "your ticket:")
        .flat_map(|s| s.parse())
        .collect();

    let tickets: Vec<Ticket> = lines
        // .skip_while(|s| s != "nearby tickets:")
        .flat_map(|s| parse_ticket(&s))
        .collect();

    let result = error_rate(&tickets, &rules);
    println!("Hello, world! Part 1 is  {}", result);

     let valid_tickets: Vec<Ticket> = tickets
        .iter()
        .filter(|t| ticket_error_values(t, &rules).len() == 0)
        .cloned()
        .collect();

    let mut rule_index: HashMap<&String, usize> = HashMap::new();
    let mut remaining_indexes: HashSet<usize> = (0..rules.len()).collect();

    while rule_index.len() < rules.len() {
        let mut index_to_remove = 0;
        for &i in &remaining_indexes {
            let possible_rules: Vec<&String> = rules
                .iter()
                .filter(|&r|  !rule_index.contains_key(&r.name) && value_can_be_rule(&valid_tickets, i, r))
                .map(|r| &r.name)
                .collect();

            if possible_rules.len() == 1 {
                rule_index.insert(possible_rules[0], i);
                index_to_remove = i;
            }
        }
        remaining_indexes.remove(&index_to_remove);
    }

    let my_ticket = &tickets[0];
    let product = rule_index
        .iter()
        .filter(|(k, _v)| k.starts_with("departure"))
        .map(|(_k, &v)| my_ticket[v] as u64)
        .product::<u64>();

    dbg!(rule_index);
    println!("Hello, world! Part 2 is {}", product);
}
