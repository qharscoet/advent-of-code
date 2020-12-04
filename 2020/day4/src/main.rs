use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


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

    let count = passports.iter().filter(|s| is_valid_passport(s)).count();
    println!("Hello, world! Result is : \n{:?}", count);
}
