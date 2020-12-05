use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_seat_id(s: &str) -> u16 {
    // u32::from_str_radix(&s.chars().map(|c| if c == 'F' || c == 'L' {'0'} else {'1'}).collect::<String>(),2).unwrap_or_default()
    s.chars().rev().enumerate().fold(0, |val, (i, c)| val +  ((c == 'B' || c == 'R') as u16) * (1 << i))
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let values: Vec<String> = buf_reader.lines().flatten().collect();
    let seats : Vec<u16> = values.iter().map(|s| get_seat_id(s)).collect();

    let max = seats.iter().max();
    let my_seat = (1..1024).into_iter().filter(|i| !seats.contains(i) && seats.contains(&(i-1)) && seats.contains(&(i+1))).nth(0);
    println!("Hello, world! Max is : \n{:?}", max);
    println!("Hello, world! My seat is : \n{:?}", my_seat);
}
