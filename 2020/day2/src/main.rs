#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_valid_input(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    }

    match RE.captures(s) {
        None =>  return false,
        Some(caps) => {
            let low: usize = caps[1].trim().parse().unwrap();
            let high: usize = caps[2].trim().parse().unwrap();
            let letter: u8 = caps[3].as_bytes()[0];
            let password: &str = &caps[4];

            let count = password.as_bytes().iter().filter(|&&c| c == letter).count();

            return count >= low && count <= high;
        }
    }
}

//Main
fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let count = buf_reader
        .lines()
        .filter(|line| is_valid_input(line.as_ref().unwrap()))
        .count();
    println!("Hello, world! Result is : \n{:?}", count);
}