use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Update(u64, u64), //pos, mask
    Write(u64, u64),  //address, value
}

impl std::str::FromStr for Instruction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_WRITE: Regex =
                Regex::new(r"mem\[([[:digit:]]+)\] = ([[:digit:]]+)").unwrap();
        }

        if s.starts_with("mask") {
            let mask_str = s.split('=').skip(1).next().unwrap_or_default().trim();
            let positions = u64::from_str_radix(&mask_str.replace('0', "1").replace('X', "0"), 2).unwrap_or_default();
            let mask_value = u64::from_str_radix(&mask_str.replace('X', "0"), 2).unwrap_or_default();

            Ok(Instruction::Update(positions, mask_value))
        } else {
            if let Some(caps) = RE_WRITE.captures(s) {
                let address: u64 = caps[1].trim().parse().unwrap_or_default();
                let value: u64 = caps[2].trim().parse().unwrap_or_default();
                // dbg!(caps, address, value);

                Ok(Instruction::Write(address, value))
            } else {
                Err("not a valid input line")
            }
        }
    }
}

struct Memory{
    mask : (u64,u64),
    mem : HashMap<u64,u64>,
}

impl Memory {
    fn new() -> Memory {
        Memory{mask: (0,0), mem : HashMap::new()}
    }

    fn process_instruction(&mut self, i: &Instruction) {
        match *i {
            Instruction::Update(pos, mask) => self.mask = (pos, mask}
            Instruction::Write(address, value) => *self.mem.entry(address).or_insert(0) = (!self.mask.0 & value) | self.mask.1,
        }
    }

    fn get_sum(&self) ->u64 {
        self.mem.values().sum()
    }
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let instr: Vec<Instruction> = buf_reader
        .lines()
        .flatten()
        .map(|s| s.parse())
        .flatten()
        .collect();

    let mut mem = Memory::new();
    instr.iter().for_each(|i| mem.process_instruction(i));


    println!("Hello, world! Result is {:?}", mem.get_sum());
}
