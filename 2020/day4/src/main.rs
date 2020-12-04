use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

type Rule =  fn(&str) -> bool;
fn is_valid_passport_part2(s: &str) -> bool {

    lazy_static! {
        static ref RULES: HashMap<&'static str, Rule> = {
            let mut hash_rules : HashMap<&str, Rule> = HashMap::new();
            hash_rules.insert("byr",|s:&str| (1920..=2002).contains(&s.parse::<u32>().unwrap_or_default()));
            hash_rules.insert("iyr",|s:&str| (2010..=2020).contains(&s.parse::<u32>().unwrap_or_default()));
            hash_rules.insert("eyr",|s:&str| (2020..=2030).contains(&s.parse::<u32>().unwrap_or_default()));
            hash_rules.insert("hgt",|s:&str| Regex::new(r"((^1([5-8]\d|9[0-3]))cm$|^(59|6\d|7[0-6])in$)").unwrap().is_match(s));
            hash_rules.insert("hcl",|s:&str| Regex::new(r"^#[[:xdigit:]]{6}$").unwrap().is_match(s));
            hash_rules.insert("ecl",|s:&str| vec!["amb","blu","brn","gry","grn","hzl","oth"].contains(&s));
            hash_rules.insert("pid",|s:&str| s.len() == 9 && s.chars().all(char::is_numeric));
            hash_rules
        };
    }

    let map = s.split_whitespace().fold(HashMap::new(), |mut acc, s|{
        let split_s :Vec<&str> = s.split(":").collect();
        acc.insert(split_s[0], split_s[1]);
        return acc;
    });

    return RULES.iter().all(|(key, rule)| map.contains_key(key) && rule(map[key]));

}



fn is_valid_passport(s: &str) -> bool {
    let map = s.split_whitespace().fold(HashMap::new(), |mut acc, s|{
        let split_s :Vec<&str> = s.split(":").collect();
        acc.insert(split_s[0], split_s[1]);
        return acc;
    });

    return vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().all(|key| map.contains_key(key));
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let passports: Vec<String> = buf_reader.lines().flatten().fold(vec![], |mut acc, s| {
        if s.is_empty() {
            acc.push(s);
        } else {
            let str = acc.pop().unwrap_or(String::new()).trim().to_string();
            acc.push(str + " " + &s);
        }
        return acc;
    });

    let count = passports.iter().filter(|s| is_valid_passport_part2(s)).count();
    println!("Hello, world! Result is : \n{:?}", count);
}
