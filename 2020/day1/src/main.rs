use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_values(values: &Vec<i32>, target_value: i32) -> Option<i32> {
    for v in values {
        for v2 in values {
            if v + v2 == target_value {
                return Some(v * v2);
            }
        }
    }
    None
}

fn find_3_values(values: &Vec<i32>, target_value: i32) -> Option<i32> {
    for v in values {
        if let Some(ret) = find_values(values, target_value - v) {
            return Some(v * ret);
        }
    }
    None
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let values: Vec<i32> = buf_reader
        .lines()
        .map(|line| line.unwrap().trim().parse().expect("Not a number"))
        .collect();
    println!("Hello, world! Contents is : \n{:?}", values);
    println!(
        "Result : {}",
        find_values(&values, 2020).unwrap_or_default()
    );
    println!(
        "Result : {}",
        find_3_values(&values, 2020).unwrap_or_default()
    );
}
