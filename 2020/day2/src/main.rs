#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Data {
    low: usize,
    high: usize,
    letter: u8,
    password: String,
}

fn parse_line(s: &str) -> Option<Data> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    }

    match RE.captures(s) {
        None => return None,
        Some(caps) => {
            return Some(Data {
                low: caps[1].trim().parse().unwrap(),
                high: caps[2].trim().parse().unwrap(),
                letter: caps[3].as_bytes()[0],
                password: String::from(&caps[4]),
            })
        }
    }
}

fn is_valid_input(s: &str) -> bool {
    match parse_line(s) {
        None => return false,
        Some(data) => {
            let count = data
                .password
                .as_bytes()
                .iter()
                .filter(|&&c| c == data.letter)
                .count();
            return count >= data.low && count <= data.high;
        }
    }
}

fn is_valid_input_part2(s: &str) -> bool {
    match parse_line(s) {
        None => return false,
        Some(data) => {
            let is_pos1: bool = data.password.as_bytes()[data.low - 1] == data.letter;
            let is_pos2: bool = data.password.as_bytes()[data.high - 1] == data.letter;
            return is_pos1 ^ is_pos2;
        }
    }
}

//Main
fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let count = buf_reader
        .lines()
        .filter(|line| is_valid_input_part2(line.as_ref().unwrap()))
        .count();
    println!("Hello, world! Result is : \n{:?}", count);
}
