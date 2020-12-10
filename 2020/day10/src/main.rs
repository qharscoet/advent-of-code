use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let mut values: Vec<u32> = buf_reader
        .lines()
        .map(|line| line.unwrap().trim().parse().expect("Not a number"))
        .collect();

    values.sort();
    values.insert(0, 0);
    values.insert(values.len(), values[values.len() - 1] + 3);
    let result = values
        .windows(2)
        .map(|w| w[1] - w[0])
        .fold((0, 0), |(one, three), i| match i {
            1 => (one + 1, three),
            3 => (one, three + 1),
            _ => (one, three),
        });
    println!("Hello, world! Answer is : {:?}", result);
}
