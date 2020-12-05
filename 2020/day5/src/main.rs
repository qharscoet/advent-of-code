use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_seat_id(s: &str) -> u32 {
    // u32::from_str_radix(&s.chars().map(|c| if c == 'F' || c == 'L' {'0'} else {'1'}).collect::<String>(),2).unwrap_or_default()
    s.chars().rev().enumerate().fold(0, |val, (i, c)| val +  ((c == 'B' || c == 'R') as u32) * (1 << i))
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let values: Vec<String> = buf_reader.lines().flatten().collect();
    let max = values.iter().map(|s| get_seat_id(s)).max();
    println!("Hello, world! Contents is : \n{:?}", max);
}
