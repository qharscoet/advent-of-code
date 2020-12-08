use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Instruction {
    Nop(isize),
    Jmp(isize),
    Acc(i32),
}

impl std::str::FromStr for Instruction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.split_whitespace().collect();

        if fields.len() < 2 {
            return Err("Not enough args ");
        }

        let arg: i32;
        match fields[1].trim().parse() {
            Err(_e) => return Err("arg is not an int"),
            Ok(data) => arg = data,
        }

        Ok(match fields[0] {
            "nop" => Instruction::Nop(arg as isize),
            "jmp" => Instruction::Jmp(arg as isize),
            "acc" => Instruction::Acc(arg),
            _ => Instruction::Nop(0),
        })
    }
}

impl Instruction {
    fn is_swappable(&self) -> bool {
        match self {
            Self::Acc(_) => false,
            _ => true,
        }
    }
}

fn swap_instr(i: &Instruction) -> Instruction {
    match i {
        Instruction::Nop(val) => Instruction::Jmp(*val),
        Instruction::Jmp(val) => Instruction::Nop(*val),
        Instruction::Acc(val) => Instruction::Acc(*val),
    }
}

struct Console {
    acc: i32,
    pc: usize,
    code: Box<Vec<Instruction>>,
    visited_instr: HashSet<usize>,
}

enum State {
    Loop,
    Running,
    Terminated,
}

impl Console {
    fn new(instr: Box<Vec<Instruction>>) -> Console {
        Console {
            acc: 0,
            pc: 0,
            code: instr,
            visited_instr: HashSet::new(),
        }
    }

    fn reset(&mut self) {
        self.acc = 0;
        self.pc = 0;
        self.visited_instr = HashSet::new();
    }

    fn step(&mut self) -> State {
        if !self.visited_instr.insert(self.pc) {
            return State::Loop;
        };

        match self.code[self.pc] {
            Instruction::Nop(_val) => self.pc += 1,
            Instruction::Jmp(offset) => self.pc = ((self.pc as isize) + offset) as usize,
            Instruction::Acc(val) => {
                self.acc += val;
                self.pc += 1;
            }
        }
        return if self.pc == self.code.len() {
            State::Terminated
        } else {
            State::Running
        };
    }

    fn swap_instr(&mut self, n: usize) {
        self.code[n] = swap_instr(&self.code[n]);
    }
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let instr: Box<Vec<Instruction>> = Box::new(
        buf_reader
            .lines()
            .flatten()
            .map(|s| s.parse())
            .flatten()
            .collect(),
    );

    let swappable_indexes: Vec<usize> = instr
        .iter()
        .enumerate()
        .filter(|(_idx, i)| i.is_swappable())
        .map(|(idx, _i)| idx)
        .collect();

    let mut console = Console::new(instr);
    for n in swappable_indexes {
        console.swap_instr(n);
        console.reset();
        let state = loop {
            match console.step() {
                State::Running => {}
                s => break s,
            }
        };
        if let State::Terminated = state {
            break;
        }

        console.swap_instr(n);
    }

    println!(
        "Hello, world! Instr has been reached again, acc is {}",
        console.acc
    );
}
