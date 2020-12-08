use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

enum Instruction {
    Nop,
    Jmp(isize),
    Acc(i32)
}

impl std::str::FromStr for Instruction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields:Vec<&str> = s.split_whitespace().collect();

        if fields.len() < 2{
            return Err("Not enough args ");
        }

        let arg :i32;
        match fields[1].trim().parse() {
            Err(_e) => return Err("arg is not an int"),
            Ok(data) => arg = data
        }

        Ok(match fields[0]{
            "nop" => Instruction::Nop,
            "jmp" => Instruction::Jmp(arg as isize),
            "acc" => Instruction::Acc(arg),
            _ => Instruction::Nop
        })
    }
}

struct Console {
    acc:i32,
    pc: usize,
    code: Box<Vec<Instruction>>,
    visited_instr : HashSet<usize>
}

impl Console {
    fn new(instr: Box<Vec<Instruction>>) -> Console {
        Console{acc:0, pc:0, code:instr, visited_instr: HashSet::new()}
    }

    fn step(&mut self) -> bool{
        if !self.visited_instr.insert(self.pc) {
            return false;
        };

        match self.code[self.pc] {
            Instruction::Nop => { self.pc += 1;},
            Instruction::Jmp(offset) => self.pc = ((self.pc as isize) + offset) as usize ,
            Instruction::Acc(val) => { self.acc += val; self.pc += 1;},
        }

        return true;
    }
}

fn main() {
     let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let instr: Box<Vec<Instruction>> = Box::new(buf_reader.lines().flatten().map(|s| s.parse()).flatten().collect::<Vec<Instruction>>());

    let mut console = Console::new(instr);

    while console.step() {}

    println!("Hello, world! Instr has been reached again, acc is {}", console.acc);
}
