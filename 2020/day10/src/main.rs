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



    // let count = values.iter().fold(
    //     vec![0u64; (values.last().unwrap_or(&0) + 1) as usize],
    //     |mut acc, &n| {
    //         let idx = n as usize;
    //         acc[idx] = if idx == 0 {1} else {acc[idx.saturating_sub(3)..idx].iter().sum()};
    //         acc
    //     },
    // );

    let mut count = vec![0u64; (values.last().unwrap_or(&0)+ 1) as usize];
    count[0] = 1;
    for n in &values[1..] {
        let idx = *n as usize;
        count[idx] = count[idx.saturating_sub(3)..idx].iter().sum();
    }

    println!("Hello, world! Answer is : {:?}", result);
    println!("Hello, world! Answer 2 is : {:?}", count.last());
}
