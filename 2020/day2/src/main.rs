use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Data {
    low: usize,
    high: usize,
    letter: u8,
    password: String,
}

impl std::str::FromStr for Data {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
        }

        match RE.captures(s) {
            None => return Err("COUCOU"),
            Some(caps) => {
                return Ok(Data {
                    low: caps[1].trim().parse().map_err(|_| "error parsing low")?,
                    high: caps[2].trim().parse().map_err(|_| "error parsing high")?,
                    letter: caps[3].as_bytes()[0],
                    password: String::from(&caps[4]),
                })
            }
        }
    }
}

fn is_valid_input(s: &str) -> bool {
    match s.parse::<Data>() {
        Err(_) => return false,
        Ok(data) => {
            let count = data
                .password
                .as_bytes()
                .iter()
                .filter(|&&c| c == data.letter)
                .count();

            return (data.low..=data.high).contains(&count);
            // return count >= data.low && count <= data.high;
        }
    }
}

fn is_valid_input_part2(s: &str) -> bool {
    match s.parse::<Data>() {
        Err(_) => return false,
        Ok(data) => {
            let byte_slice : &[u8] = data.password.as_bytes();
            let is_pos1: bool = byte_slice[data.low - 1] == data.letter;
            let is_pos2: bool = byte_slice[data.high - 1] == data.letter;
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
        .filter(|line| is_valid_input(line.as_ref().unwrap()))
        .count();
    println!("Hello, world! Result is : \n{:?}", count);
}
