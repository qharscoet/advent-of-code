use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_subarray(values: &[i64], target: i64) -> Option<&[i64]> {
    for i in 0..values.len() {
        let mut sum = values[i];
        for j in i + 1..values.len() {
            sum += values[j];

            if sum == target {
                return Some(&values[i..j]);
            }
        }
    }
    None
}

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

    let subarray = find_subarray(&values, result).unwrap_or_default();
    let (min, max) = (
        subarray.iter().min().unwrap_or(&0),
        subarray.iter().max().unwrap_or(&0),
    );
    println!("Hello, world! Answer is : {:?}", result);
    println!("Hello, world! Answer 2 is : {:?}", min + max);
}
