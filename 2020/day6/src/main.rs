use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn get_group_count(group: &[String]) -> u32 {
    let set :HashSet<char> = group.iter().map(|s| s.chars()).flatten().collect();
    set.len() as u32
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let groups: Vec<Vec<String>> = buf_reader.lines().flatten().fold(vec![], |mut acc, s| {
        if s.is_empty() {
            acc.push(vec![]);
        } else {
            match acc.last_mut() {
                None => acc.push(vec![s]),
                Some(group) => group.push(s)
            }
        }
        return acc;
    });

    let sum:u32 = groups.iter().map(|g| get_group_count(g)).sum();
    println!("Hello, world! Result is : \n{:?}", sum);
}
