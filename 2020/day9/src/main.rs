use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_sum(values: &[i64], target: i64) -> bool {
    let as_set: HashSet<&i64> = values.iter().collect();
    as_set.iter().any(|&&i| as_set.contains(&(target - i)))
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let values: Vec<i64> = buf_reader
        .lines()
        .map(|line| line.unwrap().trim().parse().expect("Not a number"))
        .collect();

    const PREAMBLE_SIZE: usize = 25;
    let result = values
        .windows(PREAMBLE_SIZE + 1)
        .find(|w| !is_sum(&w[..PREAMBLE_SIZE], w[PREAMBLE_SIZE]))
        .unwrap()[PREAMBLE_SIZE];
    println!("Hello, world! Answer is : {:?}", result);
}
