use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Chars;

fn compute_chars(chars: &mut Chars) -> u64 {
    let mut res = 0;
    let mut last_op: char = ' ';

    fn apply_op(res: u64, op: char, val: u64) -> u64 {
        match op {
            '+' => res + val,
            '*' => res * val,
            ' ' => val,
            _ => 0
        }
    }

    loop {
        match chars.next() {
            Some(c) => match c {
                '+' | '*' => last_op = c,
                '(' => res = apply_op(res, last_op, compute_chars(chars)),
                ')' => return res,
                '0'..='9' => res = apply_op(res, last_op, c.to_digit(10).unwrap_or_default() as u64),
                _ => {}
            },
            None => break,
        }
    }
    res
}

fn compute_line(s: &str) -> u64 {
    let mut chars = s.chars();
    compute_chars(&mut chars)
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let sum: u64 = buf_reader
        .lines()
        .flatten()
        .map(|line| compute_line(&line))
        .sum();

    println!("Hello, world! Answer is {} ", sum);
}
