use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

#[derive(Eq, Debug)]
struct Bag {
    color: String,
    contents: HashMap<String, u32>,
}

impl PartialEq for Bag {
    fn eq(&self, other: &Bag) -> bool {
        self.color == other.color
    }
}

impl Borrow<String> for Bag {
    fn borrow(&self) -> &String {
        &self.color
    }
}

impl Borrow<str> for Bag {
    fn borrow(&self) -> &str {
        &self.color
    }
}

impl Hash for Bag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.color.hash(state);
    }
}

fn can_contain(bag: &Bag, color: &str, rules: &HashSet<Bag>) -> bool {
    match bag.contents.get(color) {
        None => {
            // println!("Bag {} can't contains shiny gold, contents {:?}\n", bag.color, bag.contents);
            bag.contents
                .keys()
                .any(|c| can_contain(rules.get(c).unwrap(), color, rules))
        }
        Some(count) => true,
    }
}

fn count_contained_bags(bag: &Bag, rules: &HashSet<Bag>) -> u32 {
    bag.contents.iter().fold(0, |acc, (color, count)| {
        acc + count + count * count_contained_bags(rules.get(color).unwrap(), rules)
    })
}

fn add_bag<'a>(bagset: &'a mut HashSet<Bag>, s: &str) -> Result<&'a HashSet<Bag>, &'static str> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"([[:alpha:]]+ [[:alpha:]]+) bags contain (.*)\.").unwrap();
        static ref RE_SUB: Regex =
            Regex::new(r"(?P<number>\d) (?P<color>[[:alpha:]]+ [[:alpha:]]+) bag").unwrap();
    }

    match RE.captures(s) {
        None => Err("Error matching"),
        Some(caps) => {
            let mut bag = Bag {
                color: caps[1].to_string(),
                contents: HashMap::new(),
            };

            for content in RE_SUB.captures_iter(&caps[2]) {
                bag.contents.insert(
                    content["color"].to_string(),
                    content["number"].trim().parse().unwrap_or_default(),
                );
            }

            bagset.insert(bag);
            Ok(bagset)
        }
    }
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let rules: Vec<String> = buf_reader.lines().flatten().collect();
    let mut bagset: HashSet<Bag> = HashSet::new();
    for r in rules {
        add_bag(&mut bagset, &r).unwrap();
    }

    // println!("Bags are :\n {:?}", bagset);
    println!(
        "Bags that can contain shiny gold : {:?}",
        bagset
            .iter()
            .map(|b| can_contain(b, "shiny gold", &bagset))
            .filter(|b| *b)
            .count()
    );
    println!(
        "A shiny gold bag can contain: {:?}",
        count_contained_bags(bagset.get("shiny gold").unwrap(), &bagset)
    );
}
